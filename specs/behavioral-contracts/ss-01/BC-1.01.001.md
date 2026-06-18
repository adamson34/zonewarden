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

# BC-1.01.001: Parse valid YAML policy into internal Policy model

## Description

A well-formed YAML policy file is read from disk and deserialized into the internal `Policy` model, populating all zones, conduits, and their attributes. This is the entry point for the entire pipeline: a successful parse is a precondition for every subsequent stage. The internal model is the sole representation used downstream — the raw YAML is not retained.

## Preconditions

1. A file exists at the path specified by `--policy <PATH>` and is readable by the process.
2. The file content is valid UTF-8.
3. The file contains YAML conforming to the zonewarden policy schema (zones array, conduits array, each with required fields).
4. No duplicate YAML mapping keys are present (enforced at the YAML layer before schema validation).

## Postconditions

1. A `Policy` value is produced with all zones and conduits populated.
2. Each `Zone` has: a unique `id`, a `name`, a valid `purdue_level` (PurdueLevel enum value), an optional `sl_t`, and a `members` list (may be empty — see BC-1.01.008).
3. Each `Conduit` has: `from_zone`, `to_zone` (each referencing an existing zone id or the reserved `EXTERNAL` zone), a `direction` (Forward or Bidirectional), a `proto` (Tcp/Udp/Icmp/Other(u8)), and a `ports: PortSet`.
4. `PortSet` values are in canonical form per DI-020 (sorted, non-overlapping, non-adjacent ranges; `Any` is distinct from `[0,65535]`).
5. The policy is not yet validated (validation happens in ST-2 / BC-1.01.004); this contract covers only successful structural parsing.
6. The raw YAML bytes are not retained after parsing.

## Invariants

1. The internal `Policy` model is immutable after construction.
2. Parsing is deterministic: identical YAML bytes always produce an identical `Policy` model.
3. No network I/O occurs during parsing (DI-012).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | YAML file with comments and extra whitespace | Parses successfully; whitespace/comments are ignored by YAML parser |
| EC-002 | Policy with zero conduits declared | Parses successfully; `conduits` is an empty vec; policy is strict deny-all (validated later) |
| EC-003 | Conduit with `direction: unidirectional` (alias for forward) | Accepted; normalized to `Direction::Forward` in the model |
| EC-004 | Zone with `sl_t` as scalar integer `3` | Accepted; `SlTarget { overall: 3, fr_vector: None }` |
| EC-005 | Zone with `sl_t` as a 7-element FR vector | Accepted; `SlTarget { overall: 3, fr_vector: Some([...]) }` |
| EC-006 | PortSet expressed as a list with adjacent entries `[500, 501, 502]` | Canonical form coalescese to `[500-502]` per DI-020 |
| EC-007 | Conduit with `ports: any` (lowercase) | Parsed as `PortSet::Any` |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Valid minimal policy: 2 zones (field/control), 1 conduit (TCP 502 forward), deny-by-default | `Policy { zones: [field, control], conduits: [tcp/502 forward] }` | happy-path |
| Policy with `direction: unidirectional` on a conduit | Conduit with `Direction::Forward`; no error | edge-case |
| Policy YAML file with correct syntax but missing required `purdue_level` field on a zone | Parse error returned; `Err(E-POL-002)` | error |
| Empty YAML file (zero bytes) | Parse error: schema validation failure; `Err(E-POL-001)` | error |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.01.001-a | Parsing is deterministic: `parse(bytes) == parse(bytes)` for all valid inputs | kani |
| VP-1.01.001-b | Parsed PortSets satisfy the DI-020 canonical form invariant | kani/proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 ("Load policy: Parse a declarative YAML segmentation policy into the internal Policy model") per capabilities.md §CAP-001 |
| L2 Domain Invariants | DI-010 (policy validity), DI-011 (no partial state) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-001 ("Load policy") per capabilities.md §CAP-001 — this BC implements the structural parsing step of loading a YAML policy file, which is exactly what CAP-001 defines |

## Related BCs

- BC-1.01.002 — the failure path for this BC (malformed YAML)
- BC-1.01.003 — duplicate key detection during parse
- BC-1.01.004 — semantic validation after successful parse (depends on this BC)
- BC-1.01.009 — PortSet canonical form (composes with)

## Architecture Anchors

- `architecture/SS-01-policy.md#load` — policy loading module

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.01.001-a — deterministic parse
- VP-1.01.001-b — PortSet canonical form post-parse
