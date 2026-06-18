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
subsystem: "SS-01"
capability: "CAP-002"
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

# BC-1.01.007: Reject unrecognized direction/proto tokens (no permissive default)

## Description

The legal token sets for `direction` and `proto` fields in conduit declarations are fixed by the domain model. Any unrecognized token must produce a policy load error — there is no permissive defaulting, no silent ignore. This prevents misconfigured conduits from silently allowing or blocking traffic under an unintended interpretation.

**Legal `direction` tokens:** `forward`, `bidirectional`, `unidirectional` (alias for `forward`).
**Legal `proto` tokens:** `tcp`, `udp`, `icmp`, `other:<u8>` where `<u8>` is a decimal integer 0–255.

Additionally, a malformed `PortSet` value (e.g., overlapping ranges specified in the wrong format, or a non-integer port number) is rejected as a policy error.

## Preconditions

1. Structural YAML parsing succeeded.
2. A conduit declaration contains an unrecognized `direction` token, an unrecognized `proto` token, or a malformed `PortSet` expression.

## Postconditions

1. Exit 2; stderr identifies the conduit (by from/to zones), the offending field, and the invalid token value.
2. No flow processing occurs.

## Invariants

1. An unrecognized `direction` token is never silently treated as `forward` or `bidirectional`.
2. An unrecognized `proto` token is never silently treated as TCP/UDP or wildcard.
3. `other:<u8>` must be a valid decimal integer 0–255; `other:256` or `other:abc` are errors.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `direction: unidirectional` | Accepted as alias for `forward`; normalized in model |
| EC-002 | `direction: FORWARD` (uppercase) | Rejected; tokens are case-sensitive (or implementation may normalize — **chosen behavior: case-insensitive accept**: `FORWARD` → `Forward`) — **decision: case-sensitive reject** per DI-010 to minimize surprises |
| EC-003 | `proto: tcp` | Accepted |
| EC-004 | `proto: TCP` | Rejected (case-sensitive); exit 2 |
| EC-005 | `proto: other:17` | Accepted; `Proto::Other(17)` (UDP's protocol number) |
| EC-006 | `proto: other:256` | Exit 2; out-of-range u8 |
| EC-007 | `ports: "500-abc"` | Exit 2; malformed port range |
| EC-008 | `ports: "500-499"` (reversed range) | Exit 2; invalid range (lo > hi) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Conduit with `direction: forward` | Accepted; `Direction::Forward` | happy-path |
| Conduit with `direction: bidirectional` | Accepted; `Direction::Bidirectional` | happy-path |
| Conduit with `direction: both` (unrecognized) | Exit 2; "unrecognized direction token: both" | error |
| Conduit with `proto: tcp` | Accepted | happy-path |
| Conduit with `proto: sctp` (unrecognized) | Exit 2; "unrecognized proto token: sctp" | error |
| Conduit with `ports: "100-abc"` | Exit 2; malformed PortSet | error |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.01.007-a | Any unrecognized direction/proto token produces exit 2 | proptest/fuzz |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 ("Validate policy") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-010 ("legal direction tokens are exactly {forward, bidirectional, unidirectional(=forward alias)}; legal proto tokens {tcp, udp, icmp, other:<u8>}; any unrecognized direction/proto or malformed PortSet → policy error, no permissive defaulting") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-002 ("Validate policy") per capabilities.md §CAP-002 — token legality is an explicitly listed requirement of CAP-002 per DI-010 |

## Related BCs

- BC-1.01.004 — companion validation
- BC-1.01.009 — PortSet canonical form (related: malformed PortSet is caught here before normalization)

## Architecture Anchors

- `architecture/SS-01-policy.md#token-validation` — direction/proto token validation

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.01.007-a — unrecognized token → exit 2
