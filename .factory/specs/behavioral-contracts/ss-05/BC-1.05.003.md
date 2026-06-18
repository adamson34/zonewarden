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
subsystem: "SS-05"
capability: "CAP-009"
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

# BC-1.05.003: Stable policy digest via canonical JSON serialization (SHA-256)

## Description

The ConformanceResult includes a `policy_digest` field: the SHA-256 hash (lowercase hex) of the canonical serialization of the loaded Policy model. The canonical serialization is defined precisely in DI-018: canonical JSON, UTF-8, object keys sorted lexicographically; zones sorted by `id`; conduits de-duplicated then sorted by `(from_zone, to_zone, proto, direction, normalized_ports)`; each PortSet normalized per DI-020; `None`/absent Option fields omitted; numbers as JSON integers with no leading zeros; enum tokens fixed. Two policy files that are model-equivalent (differing only in whitespace, comment, key order, or duplicate conduits) produce the same digest. This supports the "edit and rerun" demo determinism.

## Preconditions

1. A policy has been successfully loaded and validated.
2. All conduits have been de-duplicated (model-level deduplication, not error).
3. PortSets are in canonical form (BC-1.01.009).

## Postconditions

1. `policy_digest` is a 64-character lowercase hex string representing the SHA-256 hash of the canonical serialization.
2. Two model-identical policies (ignoring whitespace/comments/key order/duplicate conduits) produce the same `policy_digest`.
3. A single-character change to any semantic field of the policy changes the `policy_digest`.
4. The digest is computed from the in-memory model, not the raw YAML bytes.

## Invariants

1. The canonical serialization is deterministic: same model → same bytes → same digest.
2. Duplicate conduits are de-duplicated before digesting (duplicate conduits are legal per DI-014 but produce the same model digest as the de-duplicated version).
3. The `Zone.name` and `sl_t` fields ARE included in the digest (they are semantically meaningful).
4. `purdue_level` and `direction` use fixed string tokens for serialization (defined in DI-018).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Policy A and Policy B differ only in YAML whitespace | Same `policy_digest` |
| EC-002 | Policy A has conduit `X→Y TCP 502` twice; Policy B has it once | Same `policy_digest` (deduplication) |
| EC-003 | Policy A with `zones:` listed alphabetically; Policy B with zones in reverse order | Same `policy_digest` (canonical sort) |
| EC-004 | One byte change to a zone `name` field | Different `policy_digest` |
| EC-005 | Policy with `PortSet::Any` vs `PortSet::[0-65535]` | Different `policy_digest` (Any and [0-65535] serialize differently per DI-018) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Minimal policy (2 zones, 1 conduit) | Specific 64-char SHA-256 hex (value determined by first correct implementation; used as golden test) | happy-path |
| Same policy, YAML with different whitespace/comments | Same digest as above | edge-case |
| Policy with a duplicate conduit removed | Same digest as the version with the duplicate | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.05.003-a | Canonical serialization is deterministic: same model → same bytes | kani |
| VP-1.05.003-b | Duplicate conduit de-duplication does not change digest | proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 ("Aggregate result: stable policy digest (DI-018)") per capabilities.md §CAP-009 |
| L2 Domain Invariants | DI-018 ("Stable policy digest (canonicalization defined): SHA-256 (lowercase hex) over canonical JSON, UTF-8, keys sorted lexicographically... ⊢") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-009 ("Aggregate result: stable policy digest (DI-018)") per capabilities.md §CAP-009 — stable policy digest is an explicitly listed requirement of CAP-009 per DI-018 |

## Related BCs

- BC-1.01.009 — PortSet canonical form (used in digest serialization)
- BC-1.05.002 — determinism (digest is part of the deterministic output)

## Architecture Anchors

- `architecture/SS-05-aggregation.md#policy-digest` — canonical serialization and SHA-256 computation

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.05.003-a — deterministic canonical serialization
- VP-1.05.003-b — dedup does not change digest
