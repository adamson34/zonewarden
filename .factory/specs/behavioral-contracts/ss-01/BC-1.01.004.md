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

# BC-1.01.004: Validate zone IDs unique and conduit endpoints exist

## Description

After structural parsing succeeds (BC-1.01.001), the policy is semantically validated. This contract covers two core semantic checks: (1) all zone IDs must be globally unique, and (2) every conduit's `from_zone` and `to_zone` must reference either a declared zone ID or the reserved `EXTERNAL` zone. Additional validator checks (tie detection, token legality, /0 rejection) are in BC-1.01.005 through BC-1.01.007. Validation failure always aborts before any flow processing.

## Preconditions

1. Structural parse (BC-1.01.001) succeeded; a `Policy` model is in memory.
2. Validation is invoked as ST-2.

## Postconditions

1. **On success:** the policy is marked validated; the longest-prefix resolution index is built; flow processing may proceed.
2. **On duplicate zone ID:** exit 2; stderr names the duplicate id and both occurrences.
3. **On missing conduit endpoint:** exit 2; stderr names the conduit (by from/to values) and the unknown zone id.
4. The reserved `EXTERNAL` zone is always valid as a conduit endpoint; it need not appear in the `zones` array.
5. A zone with zero members emits a **warning** to stderr but does not cause a validation failure (DEC-018).

## Invariants

1. Zone IDs form a set (no duplicates) after successful validation.
2. The reserved id `EXTERNAL` is never redefined; if a declared zone uses `EXTERNAL` as its `id`, that is an error (DI-010).
3. Validation is all-or-nothing: if any check fails, the whole policy is rejected (DI-011).
4. A zero-member zone warning is emitted even when other validation errors are present (warnings are informational and do not gate the error path).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Two zones with id `"office"` and `"Office"` (different case) | Not a duplicate; zone IDs are case-sensitive. Both zones are valid |
| EC-002 | Conduit with `from_zone: EXTERNAL` | Valid; `EXTERNAL` is always a legal conduit endpoint (DEC-020) |
| EC-003 | Conduit with `to_zone: EXTERNAL` | Valid; models permitted inbound flows |
| EC-004 | Conduit with `from_zone: ghost_zone` where `ghost_zone` is not declared | Exit 2; "conduit references unknown zone: ghost_zone" |
| EC-005 | Zone with `id: EXTERNAL` declared | Exit 2; "EXTERNAL is a reserved zone id and cannot be redeclared" |
| EC-006 | Zone with zero members | Warning emitted to stderr; policy is otherwise valid |
| EC-007 | Policy with suspicious broad prefix (e.g., member `0.0.0.0/4`) | Warning emitted about unusually broad prefix (prefix_len < 8); not a hard error (OQ-004 resolution) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Policy with zones `[field, control, dmz]`, conduits referencing only these ids | Validation passes; pipeline proceeds | happy-path |
| Policy with two zones both having `id: field` | Exit 2; stderr: "duplicate zone id: field" | error |
| Policy with conduit `from_zone: ghost` where `ghost` is not declared | Exit 2; stderr: "unknown zone: ghost" | error |
| Policy with zone `id: EXTERNAL` | Exit 2; stderr: "EXTERNAL is reserved" | error |
| Policy with one zone having `members: []` | Warning "zone 'field' has no members"; validation succeeds | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.01.004-a | After successful validation, all conduit zone references are resolvable | kani |
| VP-1.01.004-b | Duplicate zone ID always produces exit 2 | proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 ("Validate policy: Statically validate the loaded policy") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-010 (zone ids unique; conduit endpoints exist; EXTERNAL not redefined), DI-011 (no partial state) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-002 ("Validate policy") per capabilities.md §CAP-002 — this BC implements the first two semantic validation checks: unique zone IDs and conduit endpoint existence |

## Related BCs

- BC-1.01.001 — precondition: parse succeeded
- BC-1.01.005 — tie detection (companion semantic check)
- BC-1.01.006 — /0 member rejection (companion semantic check)
- BC-1.01.007 — token legality (companion semantic check)

## Architecture Anchors

- `architecture/SS-01-policy.md#validate` — policy validator

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.01.004-a — resolvable conduit references post-validation
- VP-1.01.004-b — duplicate zone ID → exit 2
