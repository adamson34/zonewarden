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

# BC-1.04.004: Forward conduit enforces directionality; reverse-initiated flow → WrongDirection

## Description

A `Forward` conduit (direction=forward or direction=unidirectional) permits flows where the **initiator (src)** is in the `from_zone` and the responder (dst) is in the `to_zone`. If a flow arrives with the initiator in `to_zone` and the responder in `from_zone` — the reverse of what the conduit declares — and no other conduit permits it, the result is `WrongDirection` (a violation). This implements DI-007. The `WrongDirection` verdict distinguishes a legitimate service communicating in the wrong direction from an unknown service (NoMatchingConduit).

## Preconditions

1. A flow's zone-pair matches a Forward conduit in the wrong direction (src is in `to_zone`, dst is in `from_zone`).
2. The proto and port match the conduit's proto/port.
3. No other conduit permits the flow in the correct direction (or Bidirectional).

## Postconditions

1. Verdict = `WrongDirection`.
2. A `Violation { kind: ViolationKind::WrongDirection, severity: ..., explanation: ... }` is produced.
3. `wrong_direction` tally incremented by 1.

## Invariants

1. `WrongDirection` only applies when the flow matches a conduit's zone-pair and proto/port, but with the wrong initiator direction.
2. If no conduit at all matches the zone-pair+proto+port, the verdict is `NoMatchingConduit`, not `WrongDirection`.
3. `WrongDirection` is a violation (always).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Conduit `A→B TCP 502`; flow initiated by B→A on port 502 | WrongDirection violation (DEC-006) |
| EC-002 | Conduit `A→B TCP 502` AND conduit `B→A TCP 502`; flow B→A | Allowed by the second conduit (any-match union; not WrongDirection) |
| EC-003 | Conduit `A↔B Bidirectional TCP 502`; flow B→A | Allowed (Bidirectional permits both directions; BC-1.04.005) |
| EC-004 | No conduit at all for zone-pair; flow from B→A | NoMatchingConduit (not WrongDirection — there's no conduit to be "wrong" for) |
| EC-005 | Conduit `A→B TCP any`; flow initiated by B on port 502 | WrongDirection (wrong direction, but proto/port match) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Conduit `field→control TCP [502]`; flow `control → field TCP 502` | Verdict `WrongDirection`; violation | happy-path |
| Conduit `field→control TCP [502]`; flow `field → control TCP 502` | Verdict `Allowed`; no violation | happy-path |
| No conduit; flow `control → field TCP 502` | Verdict `NoMatchingConduit` (not WrongDirection) | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.004-a | A flow matching conduit zone-pair+proto+port but with wrong initiator direction → WrongDirection | proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-006 ("Match conduits: directional by initiator, bidirectional if declared (DI-007)") per capabilities.md §CAP-006 |
| L2 Domain Invariants | DI-007 ("Conduit directionality: from→to permits only flows whose initiator (src) is in from and dst is in to. Reverse-initiated flow on Forward conduit → WrongDirection") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-006 ("Match conduits") per capabilities.md §CAP-006 — directional enforcement is an explicitly listed component of conduit matching per DI-007 |

## Related BCs

- BC-1.04.005 — bidirectional conduit (the alternative to this BC's WrongDirection case)
- BC-1.04.003 — any-match union (frames the "any conduit permits" check this BC is the failure case of)

## Architecture Anchors

- `architecture/SS-04-classification.md#directionality` — conduit direction enforcement

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.004-a — wrong initiator direction → WrongDirection
