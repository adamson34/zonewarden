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

# BC-1.05.001: DI-015 accounting identity holds for every ConformanceResult

## Description

The ConformanceResult's tally counters must satisfy a strict accounting identity: `total_flows == intra_zone + allowed + no_matching_conduit + wrong_direction + multicast_exempt`. This is the DI-015 invariant — it is a completeness check that ensures no flow was silently lost or double-counted. Additionally: `idmz_bypasses ≤ total_flows`, `distinct_violating_flows ≤ total_flows`, and `skipped` is excluded from `total_flows`. All counters are `u64` (canonical). Note: `external_endpoints` is a diagnostic counter and NOT part of the identity (DEC-026).

## Preconditions

1. All flows have been classified (ST-6 complete).
2. Aggregation (ST-7) assembles the ConformanceResult.

## Postconditions

1. `total_flows == intra_zone + allowed + no_matching_conduit + wrong_direction + multicast_exempt` (exact equality; u64 arithmetic; overflow is caught by BC-1.05.004).
2. `idmz_bypasses ≤ total_flows`.
3. `distinct_violating_flows ≤ total_flows`.
4. `skipped` is a separate counter; it does NOT contribute to `total_flows`.
5. `external_endpoints` is a separate diagnostic counter; NOT in the identity.
6. All tallies are `u64`; no platform `usize` used.

## Invariants

1. The accounting identity holds for any valid input, including empty flow input (all tallies zero).
2. `idmz_bypasses` may count flows also in `allowed` (additive) — the identity does not include `idmz_bypasses` because it would break the five-way partition.
3. Checked arithmetic (BC-1.05.004) ensures overflow is caught before a wrong total is produced.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Zero flows | `total_flows=0, intra_zone=0, ..., multicast_exempt=0`; identity holds (0==0+0+0+0+0) |
| EC-002 | All flows are intra-zone | `total_flows==intra_zone`; all other counts 0 |
| EC-003 | Mix of all five VerdictKinds | Identity holds; each flow counted exactly once |
| EC-004 | Flow allowed AND idmz_bypass=true | Counted in `allowed`; also in `idmz_bypasses`; `distinct_violating_flows` also incremented for flow_index; identity still holds |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 10 intra_zone + 5 allowed + 3 NoMatchingConduit + 1 WrongDirection + 1 MulticastExempt | `total_flows=20`; identity: `20 == 10+5+3+1+1` | happy-path |
| 0 flows | All-zero ConformanceResult; `0 == 0+0+0+0+0` | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.05.001-a | DI-015 identity: `total_flows == sum of five tallies` for all valid inputs | kani |
| VP-1.05.001-b | `idmz_bypasses ≤ total_flows` | kani |
| VP-1.05.001-c | `distinct_violating_flows ≤ total_flows` | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 ("Aggregate result: Verdict accounting (DI-015)") per capabilities.md §CAP-009 |
| L2 Domain Invariants | DI-015 ("Verdict totality & accounting: total_flows == intra_zone + allowed + no_matching_conduit + wrong_direction + multicast_exempt. idmz_bypasses tallied separately. distinct_violating_flows ≤ total_flows. idmz_bypasses ≤ total_flows. ⊢") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-009 ("Aggregate result: Verdict accounting (DI-015)") per capabilities.md §CAP-009 — the DI-015 accounting identity is the primary requirement of the aggregation capability |

## Related BCs

- BC-1.04.010 — verdict totality (enables this accounting)
- BC-1.05.004 — u64 overflow guard (enables safe counting)
- BC-1.05.005 — empty flow input (zero case of this identity)

## Architecture Anchors

- `architecture/SS-05-aggregation.md#accounting` — ConformanceResult assembly and DI-015 check

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.05.001-a — DI-015 identity
- VP-1.05.001-b — idmz_bypasses ≤ total_flows
- VP-1.05.001-c — distinct_violating_flows ≤ total_flows
