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

# BC-1.05.005: Empty flow input yields all-zero ConformanceResult; exits 0

## Description

If the flow input contains zero successfully-normalized flows (either because the file is empty, all lines are comments/headers, or all lines are malformed/skipped), the run is still valid. A ConformanceResult with all-zero tallies is produced, and the process exits with code 0 (no violations). This is not an error — it is a valid conformance check where no flows were observed (DEC-014). If `skipped > 0` (all lines malformed), the mandatory warning applies (BC-1.02.002) but does not change the exit code.

## Preconditions

1. The policy loaded and validated successfully.
2. The flow file is readable but produces zero successfully-normalized flows.

## Postconditions

1. `ConformanceResult { total_flows: 0, intra_zone: 0, allowed: 0, no_matching_conduit: 0, wrong_direction: 0, multicast_exempt: 0, idmz_bypasses: 0, distinct_violating_flows: 0, violations: [], external_endpoints: 0, skipped: N, warnings: [...], policy_digest: ... }`.
2. Exit code 0 (no violations; zero flows is conformant).
3. JSON report emits an all-zero ConformanceResult with the policy_digest.
4. Mermaid diagram shows the policy zones/conduits with no violations highlighted.

## Invariants

1. Zero flows is a valid result; never an error.
2. The accounting identity holds: `0 == 0+0+0+0+0`.
3. Policy digest is still computed and included.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Empty flow file (zero bytes) | All-zero ConformanceResult; exit 0 (DEC-014) |
| EC-002 | Flow file with only Zeek header lines | All-zero ConformanceResult; exit 0 |
| EC-003 | Flow file with 5 malformed lines, 0 valid | `total_flows=0, skipped=5`; warning; exit 0 (exit 1 if --fail-on-skipped) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Empty conn.log file | All-zero ConformanceResult; exit 0 | edge-case (DEC-014) |
| conn.log with only `#fields` header | All-zero ConformanceResult; exit 0 | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.05.005-a | Zero flows → exit 0 (not exit 1 or 2) | unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 ("Aggregate result: Verdict accounting (DI-015)") per capabilities.md §CAP-009 |
| L2 Domain Invariants | DI-015 (accounting identity holds; zero case), DEC-014 ("Empty flow input: valid run, all-zero tallies, exit 0; not an error") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-009 ("Aggregate result") per capabilities.md §CAP-009 — the empty-flow case is the degenerate instance of the aggregation capability and must be handled correctly per DEC-014 |

## Related BCs

- BC-1.05.001 — accounting identity (zero is the base case)
- BC-1.06.001 — exit code (zero flows → exit 0)

## Architecture Anchors

- `architecture/SS-05-aggregation.md#empty-input` — empty flow handling

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.05.005-a — zero flows → exit 0
