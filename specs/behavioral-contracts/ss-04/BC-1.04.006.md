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

# BC-1.04.006: Portless protocol (ICMP/Other) matches only ports: Any conduit

## Description

Flows where the protocol is ICMP or `Proto::Other(u8)` (portless protocols) have `src_port = None` and `dst_port = None`. Such flows match a conduit **only** if the conduit's `PortSet` is `Any`. A conduit with an explicit port set (even `[0-65535]`) never matches a portless flow. This is the `Any` vs `[0-65535]` distinction from DI-020 and DEC-021. This means ICMP liveness checks across a zone boundary require a conduit with `ports: any`; a conduit specifying port 0 or a range does not cover ICMP.

## Preconditions

1. A flow has `proto = ICMP` or `proto = Other(u8)`, meaning `src_port = None` and `dst_port = None`.
2. One or more conduits exist for the flow's zone-pair.

## Postconditions

1. If any conduit for the zone-pair and direction has `ports: Any` AND matching `proto`: Verdict = `Allowed`.
2. If no conduit has `ports: Any` (all have explicit port ranges): Verdict = `NoMatchingConduit`.
3. A conduit with `ports: [0-65535]` does NOT match a portless flow.

## Invariants

1. `PortSet::Any` is the only value that matches portless flows.
2. An explicit port set (including `[0-65535]`) never matches a portless flow.
3. The proto must also match (a `ports: any, proto: tcp` conduit does NOT match an ICMP flow).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | ICMP flow; conduit `ports: any, proto: icmp` | Allowed |
| EC-002 | ICMP flow; conduit `ports: any, proto: tcp` | NoMatchingConduit (proto mismatch) |
| EC-003 | ICMP flow; conduit `ports: [0-65535], proto: icmp` | NoMatchingConduit (explicit range ≠ portless; DEC-021) |
| EC-004 | ICMP flow; conduit `ports: [0, 65535], proto: icmp` | NoMatchingConduit (same: explicit range) |
| EC-005 | TCP flow with explicit port; conduit `ports: any, proto: tcp` | Allowed (Any also matches ported flows) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| ICMP flow `field → control`; conduit `field→control ICMP ports:any` | Verdict `Allowed` | happy-path |
| ICMP flow `field → control`; conduit `field→control ICMP ports:[0-65535]` | Verdict `NoMatchingConduit` | edge-case |
| ICMP flow `field → control`; no conduit at all | Verdict `NoMatchingConduit` | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.006-a | Portless flow (ICMP/Other) with explicit-port conduit → NoMatchingConduit | kani |
| VP-1.04.006-b | PortSet::Any matches portless flows; PortSet::[0-65535] does not | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-006 ("Match conduits: PortSet canonical form (DI-020)") per capabilities.md §CAP-006 |
| L2 Domain Invariants | DI-020 ("Any is a distinct sentinel and is never folded with 0-65535 (they differ semantically: Any matches portless flows, 0-65535 does not — DEC-021)"), DEC-021 |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-006 ("Match conduits") per capabilities.md §CAP-006 — portless protocol matching semantics (Any vs explicit range) are part of the PortSet canonical form requirement of CAP-006 per DI-020 and DEC-021 |

## Related BCs

- BC-1.01.009 — PortSet canonical form (Any sentinel)
- BC-1.04.003 — any-match union (this BC is a specialization of the matching rule)

## Architecture Anchors

- `architecture/SS-04-classification.md#portless-match` — portless protocol matching

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.006-a — portless + explicit port → NoMatchingConduit
- VP-1.04.006-b — Any vs [0-65535] semantic distinction
