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

# BC-1.01.002: Reject malformed YAML with precise diagnostic (fail-fast)

## Description

When the policy file cannot be parsed as valid YAML or does not conform to the policy schema, the tool must fail immediately with a precise, actionable error message referencing the specific line/field. No flow processing occurs. This is the "fail loud on policy" rule (DI-013): policy errors are never silently tolerated.

## Preconditions

1. `--policy <PATH>` is provided.
2. The file is readable (otherwise BC-1.06.001 / FM-001 applies).
3. The file content is either invalid YAML syntax OR valid YAML but missing required fields / containing type mismatches.

## Postconditions

1. Process exits with code `2` (E-POL-001 or E-POL-002).
2. A diagnostic message is emitted to stderr identifying: the file path, the failure kind (syntax error vs. schema mismatch), and the line/field if determinable.
3. No flows are ingested or processed.
4. No output artifacts are written.

## Invariants

1. A policy error always produces exit code 2 — never 0 or 1.
2. The error message is emitted to stderr (not stdout).
3. The partial Policy model (if any was built before the error) is discarded; the engine never runs on a partially-loaded policy (DI-011).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | YAML with a tab character used for indentation (YAML forbids tabs) | Exit 2; message cites line number and "YAML syntax error: tabs not allowed" |
| EC-002 | YAML with wrong type for `purdue_level` (e.g., a string like `"L99"`) | Exit 2; message cites field name and the invalid value |
| EC-003 | Policy file contains only comments and no data | Exit 2; message indicates empty/missing policy structure |
| EC-004 | Policy file with a valid zone block but no `id` field | Exit 2; message cites "missing required field: id" |
| EC-005 | Policy file where `purdue_level` is a valid token but for a non-existent level (e.g., `"L99"`) | Exit 2; valid parse but schema error |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| YAML file with a tab-indented block | Exit 2; stderr contains "YAML syntax error" and line number | error |
| YAML file where a zone is missing the required `id` field | Exit 2; stderr contains "missing field: id" | error |
| YAML file with `purdue_level: L99` (invalid enum value) | Exit 2; stderr contains the field path and invalid value | error |
| Valid YAML structure but `zones` is a scalar, not a sequence | Exit 2; schema error | error |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.01.002-a | Any parse error always produces exit code 2 (never 0 or 1) | proptest |
| VP-1.01.002-b | No flow processing occurs when policy parse fails | kani (property: no ST-3 invocation after ST-1 error) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 ("Load policy: Parse a declarative YAML segmentation policy into the internal Policy model") per capabilities.md §CAP-001 |
| L2 Domain Invariants | DI-010 (policy validity), DI-011 (no partial state) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-001 ("Load policy") per capabilities.md §CAP-001 — this BC covers the failure path of CAP-001: malformed policy always fails fast with a precise diagnostic, never partially loading |

## Related BCs

- BC-1.01.001 — the happy path (successful parse)
- BC-1.06.001 — exit code semantics (depends on this BC's exit code guarantee)

## Architecture Anchors

- `architecture/SS-01-policy.md#load-error` — policy parse error handling

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.01.002-a — exit code on parse error
- VP-1.01.002-b — no flow processing after policy error
