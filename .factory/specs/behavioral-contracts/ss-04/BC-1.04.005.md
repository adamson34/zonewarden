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

# BC-1.04.005: Bidirectional conduit permits initiation from either zone

## Description

A `Bidirectional` conduit (direction=bidirectional) permits flows where either side initiates. A flow from `from_zone` to `to_zone` AND a flow from `to_zone` to `from_zone` are both `Allowed` if they match the conduit's proto and port. This models symmetric communication patterns common in OT (e.g., SCADA polling + device unsolicited reporting on the same port).

## Preconditions

1. A flow's zone-pair matches a Bidirectional conduit (src in `from_zone` and dst in `to_zone`, OR src in `to_zone` and dst in `from_zone`).
2. The flow's proto and port match the conduit's proto and port.

## Postconditions

1. Verdict = `Allowed(conduit_id)`.
2. No WrongDirection verdict occurs for the reverse-initiated case.
3. `allowed` tally incremented by 1.

## Invariants

1. A Bidirectional conduit effectively acts as two Forward conduits (A→B and B→A) for matching purposes.
2. If only a Bidirectional conduit covers a zone-pair, both directions are Allowed.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Conduit `A↔B Bidirectional TCP 502`; flow A→B | Allowed |
| EC-002 | Conduit `A↔B Bidirectional TCP 502`; flow B→A | Allowed (bidirectional) |
| EC-003 | Conduit `A↔B Bidirectional TCP 502`; flow A→B on UDP 502 | NoMatchingConduit (proto mismatch) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Conduit `field↔control Bidirectional TCP [502]`; flow `control → field TCP 502` | Verdict `Allowed`; no violation | happy-path |
| Conduit `field↔control Bidirectional TCP [502]`; flow `field → control TCP 502` | Verdict `Allowed`; no violation | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.005-a | Bidirectional conduit permits both initiation directions | proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-006 ("Match conduits: bidirectional if declared (DI-007)") per capabilities.md §CAP-006 |
| L2 Domain Invariants | DI-007 ("A Bidirectional conduit permits initiation from either side") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-006 ("Match conduits") per capabilities.md §CAP-006 — bidirectional conduit semantics are part of DI-007 and an explicit component of CAP-006 matching |

## Related BCs

- BC-1.04.004 — forward conduit directionality (the non-bidirectional case)

## Architecture Anchors

- `architecture/SS-04-classification.md#directionality` — bidirectional matching

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.005-a — bidirectional permits both directions
