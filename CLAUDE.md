# CLAUDE.md

Orientation for working in this repository. Keep it accurate — it describes
current behavior, not plans.

## What this is

`zonewarden` — segmentation-as-code for OT/ICS networks. It validates an IEC
62443 zone/conduit **policy** (YAML) against **observed flows** (Zeek `conn.log`)
and classifies every flow as allowed or violating, including the Purdue L3.5 IDMZ
no-bypass check. Fully offline; deterministic (byte-identical output for identical
input). See [README.md](README.md) for the product pitch and usage.

## Workspace layout

Two-crate Cargo workspace with a strict pure-core / effectful-shell split
(ADR-002):

- **`zonewarden-core/`** — pure library, the verification target. No I/O, no
  sockets, no serde; `#![forbid(unsafe_code)]`. Modules: `validator`, `resolver`
  (longest-prefix zone resolution), `classifier` (the verdict engine),
  `idmz` (no-bypass truth table), `multicast`, `portset`, `severity`,
  `aggregator`, `digest` (canonical-JSON SHA-256 policy digest), `types`,
  `errors`.
- **`zonewarden/`** — thin CLI binary (effectful shell): arg parsing (`clap`),
  the `zeek` adapter, YAML policy loading (`serde_norway`), `reporter` (JSON /
  text / Mermaid), atomic output, exit-code mapping.
- **`fuzz/`** — `cargo-fuzz` targets for the two parsers (`fuzz_zeek`,
  `fuzz_policy`).

Pipeline: `load → validate → resolve → classify → aggregate → report`.

## Commands

```sh
cargo build --release            # build (binary: target/release/zonewarden)
cargo test                       # full suite (workspace)
cargo fmt --all --check          # formatting gate
cargo clippy --all-targets -- -D warnings   # lint gate (warnings are errors)
```

Formal / quality tooling (Phase 6):

```sh
cargo kani                       # Kani proofs (kani harnesses live under #[cfg(kani)])
cargo mutants -p zonewarden-core --exclude-re kani_harness   # mutation testing
cargo llvm-cov --workspace --summary-only                    # coverage
cargo deny check                 # supply-chain / license
cargo +nightly fuzz run fuzz_zeek    # from fuzz/
```

## Conventions

- **Tests** are integration tests under `zonewarden-core/tests/` (one file per
  module) plus inline `#[cfg(test)]` where it reads better (e.g. `portset`).
  Test fn names embed the behavioral-contract id, e.g.
  `test_BC_1_04_004_…`; `#![allow(non_snake_case)]` is set per file for this.
- **Exit codes** (ST-8): `0` conformant · `1` violations · `2` usage/policy/limit
  error. Mapped in `main.rs` via `ZonewardenError::exit_code`.
- **Determinism** is a hard requirement: sort keys, the policy digest, and
  Mermaid emission are all driven by sorted structures. Don't introduce
  hash-order-dependent output.
- Core stays pure: if you reach for `std::net::TcpStream`, serde, or file I/O in
  `zonewarden-core`, it belongs in the `zonewarden` shell instead. (`std::net`
  *value* types like `IpAddr` are fine.)

## Spec & process artifacts

This project is built spec-first (VSDD). The full paper trail — domain spec, PRD
with 44 behavioral contracts, architecture + ADRs, verification properties,
stories, and adversarial-review history — lives on the `factory-artifacts`
branch, mounted as a git worktree at `.factory/` (gitignored). The pipeline
state is `.factory/STATE.md`.
