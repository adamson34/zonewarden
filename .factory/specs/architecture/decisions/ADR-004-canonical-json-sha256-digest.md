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

# ADR-004: Canonical JSON + SHA-256 for Policy Digest

**Status:** accepted

## Context

DI-018 requires a `policy_digest` that is implementation-deterministic: two model-identical
policies (differing only in YAML key order, whitespace, or duplicate conduits) must produce
the same digest. The digest proves the policy used in a run and supports the "edit-and-rerun"
demo determinism contract.

## Options Considered

1. **Canonical JSON + SHA-256 (chosen)** — Custom serializer that sorts keys; standard
   SHA-256 via `sha2` crate. Deterministic by construction.
2. **Canonical YAML → SHA-256** — No standard canonical YAML form; implementations diverge.
   Rejected: not stable across parser versions.
3. **Bincode / MessagePack + SHA-256** — Compact but format version-dependent; field order
   is layout-dependent. Not human-inspectable. Rejected.
4. **BLAKE3 instead of SHA-256** — Faster; equally suitable. SHA-256 chosen for wider
   tooling compatibility (openssl, sha256sum for cross-verification).

## Decision

Canonical JSON serialization via a custom `serde_json`-based serializer, then SHA-256
via the `sha2` crate. The canonical form is fully specified in DI-018.

## Rationale

- `serde_json` with sorted-key output is achievable via the `indexmap` feature or a
  custom `Serialize` impl that emits keys in sorted order.
- SHA-256 is universally verifiable with `sha256sum` — assessors can independently verify
  the digest without installing zonewarden.
- The canonical form is fully specified in DI-018 (zone sort, conduit dedup+sort, PortSet
  normalization, enum tokens, None-field omission). This is a deterministic algorithm,
  not a format version.

## Consequences

- `digest` module in `zonewarden-core` implements the canonical serializer.
- `sha2` crate is a pure-Rust dep (no C FFI); works in no-std contexts.
- The digest is computed after `aggregator` assembles `ConformanceResult`; it does not
  participate in the classification hot path.

## Verification Feasibility

SHA-256 itself is too wide for Kani (256-bit state). The proof strategy for VP-007 is:
- **proptest:** generate pairs of `Policy` models with structural equivalence (same zones
  and conduits modulo order/duplicates); assert `digest(a) == digest(b)`.
- **unit tests:** golden digest fixtures for hand-authored policies.

This is NOT a Kani target. Proptest is the correct tool for this property.
