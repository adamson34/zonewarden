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
capability: "CAP-006"
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

# BC-1.04.003: Any-match conduit union: flow allowed if ≥1 conduit permits it

## Description

Conduit matching uses union semantics: a flow is `Allowed` if at least one conduit permits its `(zone-pair, proto, port, direction)` tuple. Multiple conduits between the same zone-pair are legal (DI-014) — they may overlap, duplicate, or provide partial coverage. If any one of them permits the flow, the result is `Allowed`. The `Allowed(conduit_id)` variant records the first/any matching conduit for traceability. Overlapping conduits are never a load-time error.

## Preconditions

1. Both endpoints have been resolved to non-matching zones (cross-zone flow; not IntraZone).
2. Neither endpoint is MulticastBroadcast.
3. At least one conduit in the policy has `from_zone` == src zone AND `to_zone` == dst zone (forward) OR `from_zone` == dst zone AND `to_zone` == src zone with `Bidirectional` direction.

## Postconditions

1. Verdict = `Allowed(conduit_id)` where `conduit_id` identifies any one of the matching conduits.
2. The flow is not a violation (not in `violations` list).
3. `allowed` tally incremented by 1.
4. `idmz_bypass` is still evaluated independently and may be true even for an Allowed flow (DEC-005).

## Invariants

1. `Allowed` requires at least one fully-matching conduit (zone-pair, proto, port, direction all match).
2. An overlapping or duplicate conduit never overrides a correctly-matching result.
3. The matching is order-independent: conduit order in the policy YAML does not affect the result.
4. A port-range conduit `[500-510]` matches any flow with `dst_port ∈ [500, 510]` (inclusive bounds).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Two conduits `[502]` and `[500-510]` between same zone-pair; flow on port 505 | Allowed (second conduit matches); DEC-015 |
| EC-002 | Conduits `Forward A→B` and `Bidirectional A↔B` both exist; flow B→A | Allowed (Bidirectional permits B→A initiation) |
| EC-003 | Conduit `A→B TCP 502`; flow uses UDP 502 | NOT matched (proto mismatch); if no other conduit matches → NoMatchingConduit |
| EC-004 | Conduit with `ports: any`; ICMP flow | Matched (Any matches portless flows — DEC-021) |
| EC-005 | Conduit with `ports: [0-65535]`; ICMP flow | NOT matched (explicit range does not match portless — DEC-021) |
| EC-006 | Conduit `EXTERNAL → control`; flow from EXTERNAL to control | Matched per conduit semantics (DEC-020) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Flow `field → control TCP 502`; conduit `field→control TCP [502]` | Verdict `Allowed`; not a violation | happy-path |
| Flow `field → control TCP 505`; conduits `[502]` and `[500-510]` | Verdict `Allowed` (second conduit matches) | edge-case |
| Flow `field → control UDP 502`; only `TCP [502]` conduit | NoMatchingConduit (proto mismatch) | edge-case |
| ICMP flow `field → control`; conduit `ports: any` | Allowed (Any matches portless ICMP) | edge-case |
| ICMP flow `field → control`; conduit `ports: [0-65535]` | NoMatchingConduit (explicit range ≠ portless) | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.003-a | If ≥1 conduit matches, result is Allowed regardless of other conduits | proptest |
| VP-1.04.003-b | Conduit order does not affect Allowed/NoMatchingConduit result | proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-006 ("Match conduits: Determine whether any allowed conduit permits a flow's (zone-pair, proto, port, direction). Any-match union (DI-014)") per capabilities.md §CAP-006 |
| L2 Domain Invariants | DI-014 ("Conduit matching is any-match (union): a flow is Allowed if at least one conduit permits its tuple") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-006 ("Match conduits") per capabilities.md §CAP-006 — any-match union is explicitly listed as the matching semantics for CAP-006 per DI-014 |

## Related BCs

- BC-1.04.004 — directionality enforcement (part of what "matching" requires)
- BC-1.04.006 — portless protocol matching (related)

## Architecture Anchors

- `architecture/SS-04-classification.md#conduit-match` — conduit matching algorithm

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.003-a — any-match union semantics
- VP-1.04.003-b — order-independent matching
