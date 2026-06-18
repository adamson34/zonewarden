---
document_type: architecture-section
level: L3
section: pipeline
version: "1.0"
status: draft
producer: architect
timestamp: 2026-06-17T00:00:00
phase: 1b
inputs: [domain-spec/L2-INDEX.md, specs/prd.md, specs/prd-supplements/module-criticality.md]
traces_to: ARCH-INDEX.md
---

# SS-01: Core Pipeline & Components

## Module Catalog

Each entry: module name, crate location, purity classification, criticality tier, single responsibility.

### Pure Core Modules (`zonewarden-core`)

| Module | Path | Purity | Criticality | Single Responsibility |
|--------|------|--------|-------------|----------------------|
| `portset` | `core/src/portset.rs` | **Pure** | CRITICAL | PortSet canonical form (DI-020) and port matching |
| `validator` | `core/src/validator.rs` | **Pure** | CRITICAL | Policy semantic validation: zone uniqueness, conduit endpoints, tie detection, /0 rejection |
| `resolver` | `core/src/resolver.rs` | **Pure** | CRITICAL | Build sorted-prefix index; longest-prefix match; EXTERNAL fallback |
| `multicast` | `core/src/multicast.rs` | **Pure** | HIGH | Step-1 (family-wide) and Step-2 (directed-broadcast) exemption detection (DI-016) |
| `classifier` | `core/src/classifier.rs` | **Pure** | CRITICAL | ST-6 verdict evaluation: IntraZone, conduit any-match union, WrongDirection, NoMatchingConduit; calls idmz and severity |
| `idmz` | `core/src/idmz.rs` | **Pure** | CRITICAL | DI-006 truth table: managed ≤L3 ↔ managed ≥L4, no IDMZ endpoint → `idmz_bypass = true` |
| `severity` | `core/src/severity.rs` | **Pure** | HIGH | ConnState → Severity bucket (DI-017); conservative default = Established |
| `aggregator` | `core/src/aggregator.rs` | **Pure** | HIGH | Assemble ConformanceResult; checked-add tallies (FM-009); sort violations by total-order key (DI-009) |
| `digest` | `core/src/digest.rs` | **Pure** | HIGH | Canonical JSON serialization + SHA-256 → `policy_digest` (DI-018) |

### Effectful Shell Modules (`zonewarden` binary crate)

| Module | Path | Purity | Criticality | Single Responsibility |
|--------|------|--------|-------------|----------------------|
| `policy` | `src/policy.rs` | **Effectful** | HIGH | YAML file read, serde deserialization, duplicate-key detection; emits typed `PolicyModel` |
| `zeek_adapter` | `src/adapters/zeek.rs` | **Effectful** | HIGH | Zeek conn.log TSV streaming parser; normalize to `Flow`; service inference; IPv4-mapped canonicalization |
| `reporter` | `src/reporter.rs` | **Effectful** | MEDIUM | text / JSON / Mermaid output formatting; atomic write (temp+rename) |
| `cli` | `src/main.rs` + `src/cli.rs` | **Effectful** | MEDIUM | Argument parsing (`clap`); flag validation; pipeline orchestration; exit code emission |

## Inter-Module Interfaces

### `portset` → `validator` and `classifier`
```rust
pub fn canonicalize(raw: &[PortRange]) -> PortSet
pub fn matches_port(ps: &PortSet, port: Option<u16>) -> bool
```

### `validator` → `resolver`
```rust
// returns the sorted prefix index; error if policy invalid
pub fn validate(policy: &PolicyModel) -> Result<ValidatedPolicy, PolicyError>
```

### `resolver` → `classifier` (via `ResolvedEndpoint`)
```rust
pub fn resolve(index: &PrefixIndex, ip: IpAddr) -> ResolvedEndpoint
```
`ResolvedEndpoint` carries `zone_id`, `match_kind: MatchKind`. Multicast detection runs
first (see `multicast::classify_dst`); if the destination is multicast/broadcast the
resolver short-circuits.

### `zeek_adapter` → `classifier`
`zeek_adapter` implements the `RealitySource` trait:
```rust
pub trait RealitySource {
    fn flows(&mut self) -> impl Iterator<Item = Result<Flow, FlowParseError>>;
}
```
The adapter is the **only** entity that touches file I/O in the ingest path. The engine
accepts a `&mut dyn RealitySource` so alternative adapters (NetFlow, P1) slot in.

### `classifier` → `aggregator`
Per-flow: `fn classify(ctx: &ClassifyCtx, flow: &Flow, resolved: ResolvedPair) -> Verdict`

`ClassifyCtx` holds the `ValidatedPolicy` (immutable) + `PrefixIndex` (immutable).

### `aggregator` → `reporter`
`fn aggregate(verdicts: impl Iterator<Item=Verdict>, policy: &ValidatedPolicy, skipped: u64) -> Result<ConformanceResult, SysError>`

## BC-to-Module Traceability

| BC ID | Implementing Module(s) |
|-------|----------------------|
| BC-1.01.001..003 | `policy` (YAML load + dup-key) |
| BC-1.01.004..007 | `validator` |
| BC-1.01.008 | `validator` (warning path) |
| BC-1.01.009 | `portset` |
| BC-1.02.001..005 | `zeek_adapter` |
| BC-1.02.006 | `zeek_adapter` (cap check) + `cli` (--max-flows) |
| BC-1.03.001..002 | `resolver` |
| BC-1.03.003..004 | `multicast` |
| BC-1.03.005 | `resolver` (both-EXTERNAL → IntraZone) |
| BC-1.04.001..006,009..011 | `classifier` |
| BC-1.04.007..008 | `idmz` |
| BC-1.05.001..005 | `aggregator` + `digest` |
| BC-1.06.001 | `cli` (exit code logic) |
| BC-1.06.002..004 | `reporter` |
| BC-1.06.005 | `aggregator` (warnings Vec) + `reporter` |
| BC-1.06.006 | `cli` (--fail-on-skipped) |
| BC-1.06.007 | **all modules** (no `std::net`; enforced structurally) |
| BC-1.06.008 | `reporter` (atomic write) |

## Purity Verification Summary

| Concern | Pure Modules | Verification Method |
|---------|-------------|---------------------|
| Zero I/O | `portset`, `validator`, `resolver`, `multicast`, `classifier`, `idmz`, `severity`, `aggregator`, `digest` | `#![no_std]` compat check; no `std::fs`/`std::net` imports |
| No global mutable state | All pure modules | No `static mut`; no interior mutability in hot path |
| Deterministic output | `aggregator` sort; `digest` canonical form | Kani proof VP-003, VP-007 |
| Checked arithmetic | `aggregator` | `checked_add`; VP-006 tests overflow detection |

## Verification-Infeasibility Notes

- `reporter` (Mermaid/text/JSON formatting): string-generation logic is testable but NOT a Kani target — too many string branches, no numeric invariant. Integration-tested via golden output.
- `zeek_adapter` (file I/O): effectful; Kani cannot model syscalls. Fuzz-targeted instead (`fuzz_flow_parse`).
- `policy` (YAML parsing): effectful; fuzz-targeted (`fuzz_policy_parse`).
