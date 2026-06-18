---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [domain-spec/L2-INDEX.md]
input-hash: "[live-state]"
traces_to: domain-spec/L2-INDEX.md
origin: greenfield
subsystem: "SS-05"
capability: "CAP-009"
lifecycle_status: active
introduced: v0.1.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.05.002: Output ordered by total-order key (ts, src_ip, src_port, dst_ip, dst_port, proto, flow_index)

## Description

All violations and the per-flow verdict list in the ConformanceResult are ordered by a total-order key: `(ts, src_ip, src_port, dst_ip, dst_port, proto, flow_index)`. The `flow_index` field is the final tiebreaker, guaranteeing a unique ordering over any flow multiset (including flows with identical timestamps and addresses). Timestamps are compared at full nanosecond precision (not truncated). `Other(String)` and `Other(u8)` payloads are compared by exact bytes — verbatim, no Unicode normalization or case folding. This determinism guarantee (DI-009) means identical inputs always produce byte-identical output.

## Preconditions

1. All flows have been classified and aggregated into a ConformanceResult.
2. Violations list and the ordered result list are being finalized for output.

## Postconditions

1. `violations` list is sorted by the total-order key on the associated flow's fields.
2. The sort is stable with respect to the key (no random tiebreaking).
3. IP addresses are sorted as their binary representation (IPv4 < IPv6 if mixed; within-family as numeric value).
4. `ts` is compared at full u64/i64 nanosecond precision.

## Invariants

1. `flow_index` as the final tiebreaker guarantees uniqueness: no two flows share a `flow_index` in the same run.
2. Sort is deterministic: no random number generators, no hash-based ordering.
3. `Other(String)` payloads compared byte-for-byte (DI-009: verbatim, no normalization).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Two violations with identical `ts`, `src_ip`, `src_port`, `dst_ip`, `dst_port`, `proto` | Ordered by `flow_index` (final tiebreaker) |
| EC-002 | Flow with `proto = Other(17)` vs `proto = Other(6)` | Ordered by byte value of the u8 (17 > 6) |
| EC-003 | Mixed IPv4 and IPv6 flows | Sorted by binary representation; consistent ordering |
| EC-004 | Empty violations list | Trivially sorted; output is empty |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Two violations: same ts, same src/dst, different flow_index | Ordered by flow_index (ascending) | edge-case |
| 5 violations with different timestamps | Ordered by ts ascending | happy-path |
| Same input run twice | Byte-identical output both times | happy-path (DI-009) |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.05.002-a | Identical inputs → identical sort output (DI-009 determinism) | kani |
| VP-1.05.002-b | flow_index breaks all ties uniquely | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 ("Aggregate result: deterministic, stable ordering (DI-009)") per capabilities.md §CAP-009 |
| L2 Domain Invariants | DI-009 ("Determinism: Output ordered by the total-order key (ts, src_ip, src_port, dst_ip, dst_port, proto) at full ts precision, with flow_index as the final stable tiebreaker. Other(String)/Other(u8) compared by exact bytes. ⊢") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-009 ("Aggregate result: deterministic, stable ordering (DI-009)") per capabilities.md §CAP-009 — deterministic output ordering is an explicitly listed requirement of CAP-009 per DI-009 |

## Related BCs

- BC-1.05.001 — accounting identity (depends on deterministic flow counting)
- BC-1.06.002 — JSON output (consumes this ordered result)

## Architecture Anchors

- `architecture/SS-05-aggregation.md#ordering` — total-order sort implementation

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.05.002-a — DI-009 determinism
- VP-1.05.002-b — flow_index uniqueness
