---
document_type: architecture-section
level: L3
section: cli-adapter
version: "1.0"
status: draft
producer: architect
timestamp: 2026-06-17T00:00:00
phase: 1b
inputs: [specs/prd-supplements/interface-definitions.md, specs/prd-supplements/error-taxonomy.md, domain-spec/invariants.md]
traces_to: ARCH-INDEX.md
---

# SS-03: CLI & Adapter Boundary

## CLI Design (`clap`)

**Decision (ADR-001, ADR-006):** `clap` v4 with derive macros. Sync; no async runtime.

```
zonewarden --policy <PATH> --flows <PATH>
           [--format text|json|mermaid]
           [--output <PATH>]
           [--fail-on-skipped]
           [--max-flows <N>]
```

`cli` module (`src/main.rs` + `src/cli.rs`) responsibilities:
1. Parse args with `clap`; reject `--max-flows 0` (exit 2, E-SYS-002).
2. Open `--policy` file (exit 2 on E-IO-001/002).
3. Open `--flows` file (exit 2 on E-IO-003/004).
4. Call `policy::load` → `validator::validate` → on error emit diagnostic + exit 2.
5. Call `zeek_adapter::new` → pass to pipeline.
6. Collect `ConformanceResult`; map to exit code.
7. Call `reporter::emit`; on I/O error exit 2.

Exit code logic lives entirely in `cli`; no other module emits a process exit.

## YAML Parser Choice (ADR-005)

**Requirement:** Duplicate YAML mapping keys must be detected and rejected as E-POL-004
(DI-010). The default `serde_yaml` (v0.9) silently takes the last value and is also
deprecated as of 2024.

**Options evaluated:**

| Crate | Dup-key detection | Status | Verdict |
|-------|------------------|--------|---------|
| `serde_yaml` 0.9 | No (silent last-wins) | Deprecated | Rejected |
| `serde_norway` (libyaml-safer fork) | Yes — `Deserializer` returns error on dup key | Maintained | **Chosen** |
| `marked-yaml` | Partial (warn mode) | Niche | Rejected |
| Manual `yaml-rust2` + serde | Yes (if hand-rolled) | Complex | Fallback only |

**Decision:** Use `serde_norway` which wraps `libyaml` and exposes a `strict` mode that
errors on duplicate mapping keys. This satisfies DI-010 and BC-1.01.003 without a
hand-rolled parser. Record in ADR-005.

**Verification note:** YAML parsing is an effectful operation; it is a **fuzz target**
(`fuzz_policy_parse`) not a Kani target. The fuzz harness exercises the full
`policy::load` path with arbitrary byte sequences.

## RealitySource — Zeek Adapter (`zeek_adapter`)

`zeek_adapter` implements `RealitySource` for Zeek `conn.log` (TSV format).

**Parsing strategy:** Line-by-line via `BufReader<File>`; never loads the full file.
Each line is mapped to `Result<Flow, FlowParseError>`.

Key normalization steps:
1. Skip `#`-prefixed comment/header lines.
2. Split on `\t`; extract columns by index (Zeek header defines column order).
3. Parse `ts` as fractional seconds → nanosecond-precision `Timestamp`.
4. IPv4-mapped IPv6 (`::ffff:x.x.x.x`) → canonicalized IPv4 (BC-1.02.005).
5. Reject `0.0.0.0` / `::` as src or dst (BC-1.02.003; emit E-FLW-002 warning).
6. Service inference against the canonical port/proto table (DI-008, BC-1.02.004).
7. Ingest cap check: if `flow_index >= max_flows`, return `Err(SysError::CapExceeded)`.
8. `flow_index` assigned only to successfully-normalized flows (dense, gap-free, DI-013).

**Fuzz target:** `fuzz_flow_parse` exercises `zeek_adapter::parse_line` with arbitrary
byte sequences. Goal: zero panics in 10-minute run (NFR-009).

## Error Handling Strategy (ADR-006)

**Decision:** `thiserror` for typed errors; map to exit codes at the `cli` boundary only.

```rust
// In zonewarden-core:
#[derive(Debug, thiserror::Error)]
pub enum PolicyError { /* E-POL-NNN variants */ }

#[derive(Debug, thiserror::Error)]
pub enum SysError { CapExceeded { max: u64 }, TallyOverflow }

// In binary crate:
#[derive(Debug, thiserror::Error)]
pub enum ZonewardenError {
    #[error(transparent)] Policy(#[from] PolicyError),
    #[error(transparent)] Io(#[from] std::io::Error),
    #[error(transparent)] Sys(#[from] SysError),
}
```

Exit code mapping in `cli::main`:
- `PolicyError::*` or I/O error → exit 2
- `SysError::CapExceeded` → exit 2
- Violations found → exit 1
- Clean → exit 0

No `.unwrap()` or `.expect()` in production code paths (enforced by `#[deny(clippy::unwrap_used)]`
on the hot path modules).

## Output Formatters (`reporter`)

| Format | Implementation | Atomic write? |
|--------|---------------|--------------|
| `text` | `Display` impl over `ConformanceResult` | Yes (temp+rename) when `--output` |
| `json` | `serde_json::to_writer` with `ConformanceResult` that derives `Serialize` | Yes |
| `mermaid` | String generation — see below | Yes |

**Mermaid generation (ADR-007):** Pure string construction; no render dependencies.
Emits a `graph LR` with zones as nodes and conduits as edges. Violations annotated
with `:::violation` style. The Mermaid string is deterministic (nodes sorted by zone ID).
This is testable via golden string comparison; not a Kani target.

**Atomic write:** When `--output <PATH>` is given, `reporter` writes to a temp file in
the same directory, then `rename()` to the target path. Partial-file corruption on
error is impossible (BC-1.06.008, FM-006).

## Exit Code Semantics

| Code | Condition |
|------|-----------|
| 0 | Conformant: `distinct_violating_flows == 0 && idmz_bypasses == 0` (and `--fail-on-skipped` not triggered) |
| 1 | Violations found; OR `--fail-on-skipped` with `skipped > 0` |
| 2 | Policy error; I/O error; usage error; ingest cap breach; u64 overflow |

Warnings are always emitted to stderr; they never affect exit code.

## Offline Enforcement

`zonewarden-core` declares `#![forbid(unsafe_code)]` and has no dependency on any
network-touching crate. The `Cargo.toml` for the workspace is audited via `cargo deny`
in CI, with `deny = ["net"]` capability to block accidental network dep addition.
BC-1.06.007 and DI-012 are satisfied by construction.
