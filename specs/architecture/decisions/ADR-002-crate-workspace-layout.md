---
document_type: adr
level: L3
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-06-17T00:00:00
phase: 1b
traces_to: ARCH-INDEX.md
---

# ADR-002: Cargo Workspace Layout — Pure `core` Lib + Thin `cli` Binary

**Status:** accepted

## Context

The Iron Law requires the purity boundary to be enforced structurally, not by convention.
The choice of workspace layout determines whether the boundary can be violated accidentally
at compile time or only at runtime.

## Options Considered

1. **Single crate, all in `src/`** — Simplest layout. Purity boundary is a naming
   convention only; any module can import `std::fs`. Cannot use Kani's `--target-dir`
   cleanly because the binary entry point pulls in I/O deps.
2. **Cargo workspace: `core` lib crate + `zonewarden` binary crate (chosen)** —
   `zonewarden-core` is a lib with no I/O deps; `zonewarden` binary depends on `core`
   and adds I/O/CLI deps. Kani harnesses target `zonewarden-core` directly.
3. **Three-crate workspace: `types` + `core` + `cli`** — Separates type definitions from
   logic. Adds complexity without meaningful benefit at this scale.

## Decision

Cargo workspace with two crates:
- `zonewarden-core/` — lib crate; pure logic; `#![forbid(unsafe_code)]`; no `std::fs`,
  `std::net`, or `tokio`; Kani proof target.
- `zonewarden/` — binary crate; depends on `zonewarden-core`; adds `clap`, YAML parser,
  `zeek_adapter`, `reporter`; integration test target.

## Rationale

- The crate boundary is enforced by the Rust compiler: `zonewarden-core` literally cannot
  import `std::fs` without adding it explicitly; any accidental I/O import fails to compile.
- Kani runs `cargo kani` against the lib crate. This cleanly isolates proof harnesses from
  binary-crate I/O glue code.
- Binary size is unaffected (Cargo merges monolithically in release builds).
- This is the standard pattern for verification-grade Rust: pure logic in a lib, effectful
  shell in a binary.

## Consequences

- All domain types (`Flow`, `Zone`, `Conduit`, `Verdict`, `ConformanceResult`, etc.) live
  in `zonewarden-core`.
- The `RealitySource` trait lives in `zonewarden-core` (pure side); implementations live
  in `zonewarden/` (effectful side).
- Test utilities (e.g., `MockRealitySource`) live in `zonewarden-core/src/testing.rs`
  behind `#[cfg(test)]`.

## Verification Feasibility

This layout is the direct enabler of all Kani proofs. VP-001 through VP-009 all target
`zonewarden-core` functions. Without this boundary, Kani cannot prove properties over
functions that transitively import I/O types.
