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
subsystem: "SS-04"
capability: "CAP-007"
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

# BC-1.04.010: Verdict totality: every resolved flow receives exactly one VerdictKind

## Description

Every successfully-normalized flow that passes the pre-classification filters (unspecified address skipped; flow index assigned) must receive exactly one `VerdictKind`. The five possible kinds — `MulticastExempt`, `IntraZone`, `Allowed`, `WrongDirection`, `NoMatchingConduit` — are mutually exclusive and together cover all cases. No flow can receive zero verdicts (silently dropped) or two verdicts (ambiguous). This is the verdict totality invariant (DI-015) and is a primary Kani proof target.

## Preconditions

1. A flow has been fully normalized and assigned a `flow_index`.
2. Both endpoints have been resolved (to a zone, to EXTERNAL, or to MulticastBroadcast).

## Postconditions

1. The flow receives exactly one `VerdictKind` ∈ {MulticastExempt, IntraZone, Allowed(conduit_id), WrongDirection, NoMatchingConduit}.
2. The verdict is stored in a `Verdict` struct keyed by `flow_index`.
3. The flow contributes to exactly one of the five VerdictKind tallies in ConformanceResult.

## Invariants

1. **Verdict totality:** every resolved flow gets exactly one VerdictKind.
2. **Precedence order (ST-6):** MulticastExempt → IntraZone → Allowed/WrongDirection/NoMatchingConduit (conduit matching). A flow satisfying an earlier condition short-circuits and never reaches later evaluation.
3. `idmz_bypass` is evaluated independently and does NOT add a VerdictKind; it produces an additive `Violation{IdmzBypass}` alongside the normal verdict.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Flow satisfies both multicast AND same-zone | MulticastExempt (precedence; DEC-027) |
| EC-002 | IntraZone flow also happens to be L1↔L4 (impossible by definition: same zone cannot span two purdue levels) | N/A — same zone has one purdue_level; idmz_bypass always false |
| EC-003 | Flow allowed by a conduit AND also an IDMZ bypass | VerdictKind = Allowed; PLUS additive Violation{IdmzBypass} (DEC-005) |
| EC-004 | All flows in a policy with zero conduits | Every non-intrazone flow is NoMatchingConduit |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 100 flows; mix of intra-zone, conduit-allowed, violations | Each flow has exactly one VerdictKind; `total_flows == sum of all five tallies` | happy-path |
| Flow matching both MulticastExempt and IntraZone criteria | Exactly one verdict: MulticastExempt | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.010-a | For all valid inputs: every normalized flow produces exactly one VerdictKind (DI-015 totality) | kani |
| VP-1.04.010-b | No flow appears in two VerdictKind tallies simultaneously | kani/proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Classify (deny-by-default): Verdict totality, exactly one kind (DI-015)") per capabilities.md §CAP-007 |
| L2 Domain Invariants | DI-015 ("Verdict totality & accounting: Every resolved flow receives exactly one VerdictKind. total_flows == intra_zone + allowed + no_matching_conduit + wrong_direction + multicast_exempt. ⊢") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-007 ("Classify (deny-by-default)") per capabilities.md §CAP-007 — verdict totality (DI-015) is an explicitly listed component of CAP-007 |

## Related BCs

- BC-1.05.001 — DI-015 accounting identity (depends on this BC's totality guarantee)
- BC-1.04.011 — MulticastExempt precedence (first in the precedence order)

## Architecture Anchors

- `architecture/SS-04-classification.md#verdict-totality` — ST-6 precedence ordering

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.010-a — verdict totality (DI-015)
- VP-1.04.010-b — no double-counting
