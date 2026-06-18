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

# BC-1.01.006: Reject 0.0.0.0/0 or ::/0 catch-all member declarations

## Description

A zone member declared as the IPv4 catch-all `0.0.0.0/0` or the IPv6 catch-all `::/0` must be rejected at policy validation time. A `/0` prefix would match every address in that family, shadowing the implicit `EXTERNAL` zone entirely and suppressing IDMZ-EXTERNAL exclusions. This is a hard policy error, not a warning.

## Preconditions

1. A zone in the parsed policy contains an `AssetMatcher` with prefix length `0` for IPv4 or IPv6.

## Postconditions

1. Exit 2; stderr identifies the zone and the rejected CIDR.
2. No flow processing occurs.

## Invariants

1. Only `/0` prefix length (for either family) is rejected by this rule. `/1` through `/7` are legal (though a warning may be emitted for prefix < 8; see OQ-004 and BC-1.01.004 EC-007).
2. The `EXTERNAL` zone is not a declared zone and is never a candidate for this check.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Zone member `0.0.0.0/0` | Exit 2; error |
| EC-002 | Zone member `::/0` | Exit 2; error |
| EC-003 | Zone member `0.0.0.0/1` (covers half the IPv4 space) | Legal; warning emitted for broad prefix (< /8) but not an error |
| EC-004 | Zone member `0.0.0.0/8` | Legal; no warning |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Zone with `members: [0.0.0.0/0]` | Exit 2; "0.0.0.0/0 catch-all is not permitted in a zone member" | error |
| Zone with `members: [::/0]` | Exit 2; "::/0 catch-all is not permitted" | error |
| Zone with `members: [0.0.0.0/1]` | Validation proceeds (warning if prefix < /8); no hard error | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.01.006-a | Any zone member with prefix_len == 0 always produces exit 2 | unit test + proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 ("Validate policy") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-010 ("rejects a declared 0.0.0.0/0 or ::/0 member — a family catch-all would shadow the implicit EXTERNAL zone"), DI-005 (implicit EXTERNAL zone) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-002 ("Validate policy") per capabilities.md §CAP-002 — /0 rejection is an explicitly listed validation requirement per DI-010 |

## Related BCs

- BC-1.01.004 — companion validation (depends on)
- BC-1.03.002 — implicit EXTERNAL zone (this BC protects that mechanism)

## Architecture Anchors

- `architecture/SS-01-policy.md#validate-members` — member validation logic

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.01.006-a — /0 always exits 2
