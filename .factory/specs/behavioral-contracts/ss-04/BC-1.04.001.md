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

# BC-1.04.001: Intra-zone flows are allowed without conduit evaluation

## Description

A flow whose source and destination endpoints resolve to the same zone (including both-EXTERNAL per BC-1.03.005) receives the `IntraZone` verdict and is not evaluated against conduits. Intra-zone traffic is implicitly allowed under the IEC 62443 zone model — conduits govern inter-zone traffic only. This implements DI-002. `IntraZone` is never a violation; these flows are counted in the `intra_zone` tally of ConformanceResult.

## Preconditions

1. Both src and dst endpoints have been resolved to zones (BC-1.03.001 or BC-1.03.002).
2. The src zone ID equals the dst zone ID.
3. The flow is not MulticastExempt (MulticastExempt takes precedence and is handled before intra-zone check per DI-016 — BC-1.04.011).

## Postconditions

1. Verdict = `IntraZone`.
2. No conduit lookup is performed.
3. `idmz_bypass` is computed independently but: if both endpoints are in the same zone, one cannot be ≤L3 and the other ≥L4 simultaneously (they are in the same zone at the same level), so `idmz_bypass = false` by definition.
4. Flow counted in `intra_zone` tally.

## Invariants

1. `IntraZone` is never a violation.
2. Conduit evaluation is skipped entirely for intra-zone flows.
3. IntraZone can occur for both-EXTERNAL flows (a legal case per DI-002 and BC-1.03.005).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `10.0.1.5 → 10.0.1.10`; both in zone `field` | IntraZone; allowed; not a violation (DEC-004) |
| EC-002 | `8.8.8.8 → 1.1.1.1`; both EXTERNAL | IntraZone per DI-002 (DEC-032) |
| EC-003 | Intra-zone flow with multicast dst | MulticastExempt wins (BC-1.04.011); not IntraZone (DI-016 precedence, DEC-027) |
| EC-004 | Policy with zero conduits; all intra-zone flows | All intra-zone flows remain allowed; cross-zone flows all NoMatchingConduit |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Flow `10.0.1.5 → 10.0.1.10`; both in zone `field` | Verdict `IntraZone`; `intra_zone` tally +1; no violation | happy-path |
| Flow `8.8.8.8 → 1.1.1.1`; no declared zones | Verdict `IntraZone` (both-EXTERNAL); no violation | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.001-a | Same-zone src/dst (non-multicast) → IntraZone verdict; conduit evaluation skipped | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Classify (deny-by-default): intra-zone (DI-002)") per capabilities.md §CAP-007 |
| L2 Domain Invariants | DI-002 ("A flow whose two endpoints resolve to the same zone is IntraZone — conduits do not govern it") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-007 ("Classify (deny-by-default)") per capabilities.md §CAP-007 — intra-zone classification is an explicitly listed component of CAP-007 per DI-002 |

## Related BCs

- BC-1.03.005 — both-EXTERNAL as IntraZone (feeds into this BC)
- BC-1.04.011 — MulticastExempt takes precedence before IntraZone

## Architecture Anchors

- `architecture/SS-04-classification.md#intrazone` — intra-zone classification logic

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.001-a — same-zone → IntraZone, no conduit lookup
