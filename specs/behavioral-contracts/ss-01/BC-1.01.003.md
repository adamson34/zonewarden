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
capability: "CAP-001"
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

# BC-1.01.003: Reject duplicate YAML mapping keys at load time

## Description

If the YAML policy file contains duplicate mapping keys at any level (e.g., two `id:` entries in the same zone block, or the `zones` key appearing twice at the top level), the tool must reject the file with a load-time error. This is an explicit defense against silent serde last-wins behavior that would corrupt the digest-trust model. The rejection must happen before any schema validation or conduit matching.

## Preconditions

1. A policy file path is provided and the file is readable.
2. The YAML file contains at least one duplicate mapping key in any mapping node.

## Postconditions

1. Process exits with code `2`.
2. Stderr identifies the duplicate key and its location (key name; line number if determinable).
3. No Policy model is constructed from the file.
4. No flows are processed.

## Invariants

1. Duplicate key detection happens at the YAML deserialization layer, before schema validation.
2. Any duplicate key — at any nesting depth — triggers the error; there is no depth limit.
3. The first encountered duplicate key is reported; subsequent duplicates in the same file may or may not be reported (implementation may short-circuit after the first).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Two zones with the same `id:` value nested under different top-level array elements | This is a semantic duplicate (same zone id), caught by BC-1.01.004 semantic validation — NOT by this BC (which is YAML-level structural duplicates only) |
| EC-002 | Top-level YAML with `zones:` appearing twice | Exit 2; duplicate YAML key error |
| EC-003 | A zone block with `name:` appearing twice with different values | Exit 2; duplicate key within the zone mapping |
| EC-004 | Policy file with no duplicate keys | Not triggered; passes to parse (BC-1.01.001) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| YAML with `zones:\n  - id: z1\n    id: z1_dup` (duplicate key in a mapping) | Exit 2; stderr mentions "duplicate key: id" | error |
| YAML with `zones: [...]` appearing twice at root | Exit 2; stderr mentions "duplicate key: zones" | error |
| Valid YAML with no duplicate keys | No error from this check; passes to BC-1.01.001 | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.01.003-a | Duplicate YAML key always produces exit 2 | proptest/fuzz |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 ("Load policy: Parse a declarative YAML segmentation policy into the internal Policy model") per capabilities.md §CAP-001 |
| L2 Domain Invariants | DI-010 ("duplicate YAML mapping keys → load error — no silent last-wins; diverges from serde_yaml defaults") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-001 ("Load policy") per capabilities.md §CAP-001 — duplicate key detection is a required property of the policy load step per DI-010 |

## Related BCs

- BC-1.01.001 — happy-path parse (this BC gates entry to that path)
- BC-1.01.002 — general malformed YAML error handling
- BC-1.05.003 — policy digest (protects digest-trust model that this BC defends)

## Architecture Anchors

- `architecture/SS-01-policy.md#dup-key-detection` — YAML deserializer configuration

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.01.003-a — duplicate key always exits 2
