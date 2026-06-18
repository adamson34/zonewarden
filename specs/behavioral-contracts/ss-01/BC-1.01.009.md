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

# BC-1.01.009: PortSet canonical form: sorted non-overlapping non-adjacent ranges

## Description

All `PortSet` values in the internal model must be maintained in a canonical form: a sorted list of non-overlapping, non-adjacent inclusive `[lo, hi]` ranges. Adjacent ranges (e.g., `500-502` and `503-505`) are coalesced into `500-505`. Singleton ports are stored as `[p, p]`. The `Any` sentinel is a distinct value and is never folded into or from `0-65535`. Canonical form is required for both matching (DI-014) and the stable policy digest (DI-018), and it is the Kani proof target for DI-020.

## Preconditions

1. A `PortSet` expression has been parsed from the YAML policy.
2. The `PortSet` is structurally valid (not malformed; malformed is caught by BC-1.01.007).

## Postconditions

1. The resulting `PortSet` is in canonical form: sorted, non-overlapping, non-adjacent inclusive ranges.
2. `PortSet::Any` is preserved as `Any` and is never converted to or from `[0, 65535]`.
3. Two `PortSet` values that are equivalent as sets of port numbers are equal as canonical forms (structural equality after normalization).
4. Port ranges with `lo > hi` are rejected as malformed (BC-1.01.007).

## Invariants

1. For any two ports `a < b` in the same canonical range `[lo, hi]`: `lo <= a <= b <= hi`.
2. No two ranges in the canonical form overlap: for adjacent ranges `[lo1, hi1]` and `[lo2, hi2]`, `lo2 > hi1 + 1` (non-adjacent).
3. `Any` is a singleton value distinct from any range representation.
4. The canonical form is unique: there is exactly one canonical representation for each port set value.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `[500, 501, 502]` (three singletons) | Coalesced to `[500-502]` |
| EC-002 | `[500-502, 503-505]` (adjacent ranges) | Coalesced to `[500-505]` |
| EC-003 | `[0-65535]` explicit range | Stored as `[0-65535]`; distinct from `Any` (DEC-021) |
| EC-004 | `any` | Stored as `PortSet::Any` |
| EC-005 | `[100-200, 150-250]` (overlapping) | Coalesced to `[100-250]` |
| EC-006 | `[502, 500-502]` (singleton within range) | Coalesced to `[500-502]` |
| EC-007 | `[502]` (single port) | Stored as `[502-502]` |
| EC-008 | `any` vs `[0-65535]`: matching a portless ICMP flow | `Any` matches portless; `[0-65535]` does not match portless (DEC-021) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `[80, 443, 8080]` | `[80-80, 443-443, 8080-8080]` | happy-path |
| `[500-502, 503]` | `[500-503]` (coalesced adjacent) | edge-case |
| `[100-200, 150-250, 300]` | `[100-250, 300-300]` (coalesced overlap + singleton) | edge-case |
| `any` | `PortSet::Any` | happy-path |
| `[0-65535]` | `[0-65535]` (NOT `Any`) | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.01.009-a | Canonical form is idempotent: `normalize(normalize(x)) == normalize(x)` | kani |
| VP-1.01.009-b | Canonical form is unique: equal port sets → equal canonical forms | kani |
| VP-1.01.009-c | `Any` is never equal to `[0-65535]` | kani |
| VP-1.01.009-d | No two ranges in canonical form are adjacent or overlapping | kani/proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-006 ("Match conduits: Determine whether any allowed conduit permits a flow's (zone-pair, proto, port, direction). PortSet canonical form (DI-020)") per capabilities.md §CAP-006 |
| L2 Domain Invariants | DI-020 ("PortSet canonical form: sorted non-overlapping non-adjacent inclusive [lo,hi] ranges; Any distinct sentinel") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-006 ("Match conduits") per capabilities.md §CAP-006 — PortSet canonical form is explicitly listed as a matching requirement of CAP-006 per DI-020 |

## Related BCs

- BC-1.01.007 — malformed PortSet rejection (precondition)
- BC-1.04.006 — portless protocol matching (relies on Any vs explicit-port distinction)
- BC-1.05.003 — stable policy digest (relies on canonical PortSet for deterministic serialization)

## Architecture Anchors

- `architecture/SS-01-policy.md#portset` — PortSet type and normalization

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.01.009-a — idempotent normalization
- VP-1.01.009-b — unique canonical form
- VP-1.01.009-c — Any ≠ [0-65535]
- VP-1.01.009-d — no adjacent/overlapping ranges in canonical form
