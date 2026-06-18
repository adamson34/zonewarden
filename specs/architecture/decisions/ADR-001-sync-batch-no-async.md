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

# ADR-001: Synchronous Batch Processing — No Async Runtime

**Status:** accepted

## Context

zonewarden processes two files (policy YAML + flow log) and exits. It has no concurrent
I/O requirements: policy is fully loaded before flow processing begins, and flows are
processed sequentially as a streaming iterator. The question is whether to adopt
`tokio` (async) for I/O or use the standard synchronous Rust I/O primitives.

## Options Considered

1. **Sync (`std::io::BufReader`, sync file I/O)** — No runtime overhead; simple stack;
   `BufReader` streaming is fast enough for sequential file reads at > 1 GB/s.
2. **Async (`tokio`, `async-std`)** — Needed for concurrent I/O or network. Adds
   compile-time complexity, binary size (~1–2 MB), and non-deterministic scheduling
   that complicates reasoning about output ordering.

## Decision

Use synchronous batch processing with `std::io::BufReader` for streaming.

## Rationale

- zonewarden is offline and file-only; there is no concurrent I/O to overlap.
- Async runtime adds no throughput benefit for sequential file reads.
- Sync code is simpler to reason about, simpler to Kani-proof (no `Future` poll state machines).
- Binary size stays minimal (NFR-012: ≤ 20 MB).
- No network I/O means no `tokio::net` imports, which makes the offline constraint
  (DI-012) easier to audit.

## Consequences

- All modules are sync; no `async fn` in `zonewarden-core`.
- Flow iterator is `impl Iterator<Item = Result<Flow, FlowParseError>>` — pull-based, no executor.
- If a future adapter (CAP-011 NetFlow) requires async I/O, the `RealitySource` trait
  can be extended to a sync iterator over pre-fetched data without breaking the core pipeline.

## Verification Feasibility

Sync functions with no executor are the ideal shape for Kani model checking. The absence
of async state machines means all Kani harnesses are straightforward `fn verify_*()` bodies
with no polling loop complications.
