---
document_type: vp-index
level: L4
version: "1.0"
status: draft
producer: architect
timestamp: 2026-06-17T00:00:00
phase: 1b
traces_to: specs/architecture/ARCH-INDEX.md
---

# Verification Property Index: zonewarden

> **Source of truth** for VP catalog. Changes to VP count, module assignments, tools,
> or phase must propagate to `verification-architecture.md` and `verification-coverage-matrix.md`.
>
> Total VPs: 10

## VP Catalog

| VP ID | Title | Module | Invariant | Proof Method | Phase | Status |
|-------|-------|--------|-----------|-------------|-------|--------|
| VP-001 | Total Endpoint Resolution | `resolver` | DI-003 | kani | P0 | verified |
| VP-002 | Longest-Prefix Totality and Uniqueness | `resolver` | DI-004 | kani | P0 | verified |
| VP-003 | IDMZ No-Bypass Rule | `idmz` | DI-006 | kani | P0 | verified |
| VP-004 | Verdict Totality | `classifier` | DI-015 | kani | P0 | verified |
| VP-005 | Accounting Identity and Bounds | `aggregator` | DI-015 | proptest | P0 | draft |
| VP-006 | Tally Overflow Detection | `aggregator` | FM-009 | kani | P0 | verified |
| VP-007 | Digest Stability | `digest` | DI-018 | proptest | P0 | draft |
| VP-008 | PortSet Canonical Form | `portset` | DI-020 | kani + proptest | P0 | partial (proptest verified; Kani harness unwinds) |
| VP-009 | Offline Purity — No Network I/O | `zonewarden-core` | DI-012 | manual (cargo deny + strace) | P0 | draft |
| VP-010 | Multicast/Broadcast Exemption Detection | `multicast` | DI-016 | kani | P0 | verified |

## Tool Summary

| Tool | VP Count | VPs |
|------|---------|-----|
| Kani | 7 | VP-001, VP-002, VP-003, VP-004, VP-006, VP-008 (partial), VP-010 |
| proptest | 3 | VP-005, VP-007, VP-008 (partial) |
| manual/structural | 1 | VP-009 |

> Note: VP-008 uses both Kani (bounded correctness of `canonicalize`) and proptest
> (larger input sequences). It is counted once in the total.

## DI-to-VP Traceability

| Domain Invariant | VP(s) |
|-----------------|-------|
| DI-003 Total endpoint resolution | VP-001 |
| DI-004 Deterministic overlap resolution | VP-002 |
| DI-006 IDMZ no-bypass | VP-003 |
| DI-009 Determinism | (via VP-004 verdict totality + VP-007 digest stability; DI-009 is also tested by aggregator sort order — integration test) |
| DI-012 Offline purity | VP-009 |
| DI-015 Verdict totality and accounting | VP-004, VP-005 |
| DI-016 Multicast exemption | VP-010 |
| DI-018 Stable policy digest | VP-007 |
| DI-020 PortSet canonical form | VP-008 |
| FM-009 Tally overflow | VP-006 |

## Verification-Infeasibility Flags

The following invariants/BCs are NOT Kani-provable and use alternative validation:

| BC/Invariant | Reason Not Kani-Provable | Alternative |
|-------------|------------------------|-------------|
| DI-009 (full output determinism) | Output includes string formatting + SHA-256; too wide for Kani | Integration test: run same input twice, `diff` outputs |
| DI-018 (digest correctness) | SHA-256 state is 256 bits wide | proptest (VP-007) + golden fixtures |
| BC-1.06.002..004 (JSON/text/Mermaid format) | String generation with many branches | Golden output integration tests |
| BC-1.02.001..006 (Zeek parser) | Effectful; file I/O in hot path | cargo-fuzz (`fuzz_flow_parse`) |
| BC-1.01.001..003 (YAML parser) | Effectful; third-party parser | cargo-fuzz (`fuzz_policy_parse`) |
