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

# BC-1.01.008: Warn (not error) on declared zone with zero members

## Description

A zone declared with an empty `members` list is structurally valid and semantically legal — it is a "dead zone" that no IP address will ever resolve to. This situation is more likely a configuration oversight than an intentional design, so the tool emits a load-time warning to stderr. The warning does not prevent policy validation from succeeding, and flow processing proceeds normally with the empty zone in place (it will simply never match anything).

## Preconditions

1. Structural parse succeeded.
2. At least one zone in the parsed policy has `members: []` (empty list).

## Postconditions

1. A warning is emitted to stderr identifying the zone by id: e.g., `"WARNING: zone 'staging' has no members and will never match any endpoint"`.
2. Policy validation continues; if all other checks pass, the policy is accepted.
3. The zero-member zone is present in the validated policy model and is included in the Mermaid diagram.

## Invariants

1. A zero-member zone warning does NOT change the exit code.
2. The warning is emitted exactly once per zero-member zone.
3. Warnings from this check are included in `ConformanceResult.warnings` for downstream rendering (DI-019).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Policy with 3 zones, 2 of which have zero members | Two warnings emitted, one per zone; policy valid |
| EC-002 | Zero-member zone that is referenced by a conduit | Warning for zone; conduit is still valid (conduit endpoints only require the zone to exist) |
| EC-003 | Zero-member zone that is the ONLY zone in the policy | Warning emitted; policy valid with zero-member zone |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Policy with `field` zone having `members: []` | Stderr: `WARNING: zone 'field' has no members...`; pipeline continues normally | edge-case |
| Policy where all zones have at least one member | No warning emitted | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.01.008-a | Zero-member zone never produces exit 2 on its own | unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 ("Validate policy") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-010 (policy validity; zero-member zone is explicitly legal — "Legal but inert; emit a load-time warning, not an error" per DEC-018) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-002 ("Validate policy") per capabilities.md §CAP-002 — the handling of zero-member zones is part of policy semantic validation as defined by DEC-018 |

## Related BCs

- BC-1.01.004 — companion validation
- BC-1.06.005 — deterministic warning order

## Architecture Anchors

- `architecture/SS-01-policy.md#validate-members` — member validation

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.01.008-a — zero-member zone never exits 2
