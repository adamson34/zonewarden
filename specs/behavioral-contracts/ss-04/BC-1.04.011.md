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

# BC-1.04.011: MulticastExempt short-circuits before IntraZone and conduit evaluation

## Description

The `MulticastExempt` verdict takes highest precedence in the classification pipeline (ST-6 order). A flow whose destination is `MulticastBroadcast` (set by BC-1.03.003 or BC-1.03.004) receives `MulticastExempt` immediately, without evaluating whether the endpoints are in the same zone or whether any conduit permits the flow. This avoids false-positive violations for dominant cyclic OT I/O patterns (EtherNet/IP implicit I/O, BACnet broadcasts). MulticastExempt is never a violation.

## Preconditions

1. The flow's dst endpoint has `MatchKind::MulticastBroadcast` (set by Step-1 or Step-2 detection).

## Postconditions

1. Verdict = `MulticastExempt`.
2. No conduit matching is performed.
3. `idmz_bypass = false` (forced; DI-006).
4. `multicast_exempt` tally incremented by 1.
5. The flow is NOT reported as a violation.

## Invariants

1. `MulticastExempt` is always the first verdict checked (highest precedence in ST-6 order).
2. An intra-zone flow with a multicast dst receives `MulticastExempt`, NOT `IntraZone` (DEC-027).
3. A multicast dst flow where a conduit would permit it still receives `MulticastExempt` (MulticastExempt short-circuits conduit evaluation).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | BACnet broadcast `field → 255.255.255.255`; no BACnet conduit | MulticastExempt; no violation (DEC-025) |
| EC-002 | EtherNet/IP implicit I/O `field → 239.192.x.x`; conduit exists | MulticastExempt (short-circuits conduit check) |
| EC-003 | Intra-zone flow; dst happens to be multicast | MulticastExempt (takes precedence over IntraZone; DEC-027) |
| EC-004 | `field (L2) → multicast dst`; would be IdmzBypass if dst were managed L4 | `idmz_bypass = false` (multicast exception; BC-1.04.008) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| BACnet broadcast `10.0.1.5 → 255.255.255.255` | Verdict `MulticastExempt`; not a violation | happy-path |
| Intra-zone flow `10.0.1.5 → 224.0.0.5` (same zone) | Verdict `MulticastExempt` (not IntraZone; DEC-027) | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.011-a | MulticastBroadcast dst → always MulticastExempt; never IntraZone or conduit verdict | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Classify: multicast exemption (DI-016)") per capabilities.md §CAP-007 |
| L2 Domain Invariants | DI-016 ("Verdict precedence is the ST-6 order: MulticastExempt short-circuits before zone-pair classification"), DI-015 (MulticastExempt is one of the five VerdictKinds) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-007 ("Classify (deny-by-default): multicast exemption (DI-016)") per capabilities.md §CAP-007 — MulticastExempt precedence is an explicitly listed component of the classification capability per DI-016 |

## Related BCs

- BC-1.03.003 — Step-1 multicast detection (sets MulticastBroadcast MatchKind)
- BC-1.03.004 — Step-2 directed-broadcast override (also sets MulticastBroadcast)
- BC-1.04.010 — verdict totality (MulticastExempt is one of the five mutually-exclusive verdicts)

## Architecture Anchors

- `architecture/SS-04-classification.md#multicast-exempt` — MulticastExempt precedence

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.011-a — MulticastBroadcast → MulticastExempt (always, regardless of zone/conduit)
