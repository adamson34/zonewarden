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

# ADR-006: Error Handling — `thiserror` + Exit-Code Mapping at CLI Boundary

**Status:** accepted

## Context

zonewarden has a detailed error taxonomy (E-POL-NNN, E-FLW-NNN, E-IO-NNN, E-SYS-NNN)
and three distinct exit codes (0, 1, 2). The architecture must ensure that:
1. Errors are typed — no stringly-typed `anyhow::Error` in hot paths.
2. Exit code mapping is centralized — no `std::process::exit()` scattered through modules.
3. Pure core modules return `Result<T, TypedError>` with no process exit.

## Options Considered

1. **`thiserror` for typed errors; `anyhow` for CLI boundary (chosen)**
   - `thiserror`: zero-cost typed error enums in `zonewarden-core`.
   - `anyhow` (or simply `Box<dyn Error>`) at the `main` boundary to collect and convert.
   - Clean separation: core errors are always typed; CLI decides the exit code.

2. **`anyhow` everywhere**
   - Ergonomic for applications; poor for libraries.
   - Loses type information needed for precise error code selection (E-POL vs E-SYS).
   - Rejected for `zonewarden-core`; acceptable only in the binary crate's `main`.

3. **Hand-rolled error enums (no `thiserror`)**
   - Maximum control; verbose `Display` and `Error` impls by hand.
   - `thiserror` is a macro that generates exactly this; no benefit in hand-rolling.

## Decision

- `zonewarden-core`: all functions return `Result<T, XxxError>` where `XxxError` derives
  `thiserror::Error`. No `unwrap`, no `expect`, no `process::exit`.
- `zonewarden` binary crate: a top-level `ZonewardenError` enum (also `thiserror`) wraps
  all error types from core and I/O. `main` matches on this enum to determine exit code.

Exit code mapping:
```rust
fn main() -> ! {
    match run() {
        Ok(ConformanceResult { distinct_violating_flows: 0, idmz_bypasses: 0, .. }) => exit(0),
        Ok(_) => exit(1),
        Err(ZonewardenError::Policy(_) | ZonewardenError::Io(_) | ZonewardenError::Sys(_)) => exit(2),
    }
}
```

## Rationale

- Typed errors enable the test-writer to assert specific error variants without string matching.
- Single-point exit code mapping eliminates the risk of a buried `process::exit(0)` that
  swallows a violation.
- `thiserror` is zero-cost at runtime (macro-generated `Display` + `From` impls only).

## Consequences

- CRITICAL and HIGH modules in `zonewarden-core` never call `process::exit`. Enforced by
  `#![deny(clippy::exit)]` on the `core` crate.
- E-FLW-NNN errors (degraded flows) are NOT returned as `Err`; they are yielded as
  `Ok(FlowResult::Skipped)` from the iterator, accumulated in `skipped`, and surfaced
  as warnings. This keeps the ingest iterator alive (DI-013 resilience asymmetry).

## Verification Feasibility

Typed error enums are directly modelable in Kani. Where VP harnesses need to exercise
error paths (e.g., VP-006 tally overflow), the harness calls the function with inputs
that trigger the error and asserts the `Err(SysError::TallyOverflow)` variant — not a
process exit.
