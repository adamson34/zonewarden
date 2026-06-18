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
subsystem: "SS-06"
capability: "CAP-010"
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

# BC-1.06.005: Warnings emitted to stderr in deterministic order; never change exit code

## Description

Warnings produced during a run (e.g., zero-member zone, skipped flow records, broad prefix) are emitted to stderr, not stdout. They are also collected in `ConformanceResult.warnings: Vec<String>` for inclusion in the JSON report. Warnings are ordered deterministically (per DI-009 ordering for the flow-related warnings; policy warnings in validation order). Warnings NEVER change the exit code — only `--fail-on-skipped` causes a skipped-flow warning to affect the exit code. A run with warnings is still a valid completed run.

## Preconditions

1. One or more warning conditions have been triggered during the run (malformed flow lines, zero-member zones, broad prefix declarations, etc.).

## Postconditions

1. Each warning is written to stderr as a single line starting with `WARNING:`.
2. Warnings are also collected in `ConformanceResult.warnings`.
3. Warnings appear in the JSON report's `warnings` field.
4. The exit code is not changed by warnings (unless `--fail-on-skipped` is set and the warning is about skipped flows).
5. Warning order is deterministic (same run → same warning order).

## Invariants

1. Warnings go to stderr; the main output (text/JSON/Mermaid) goes to stdout or `--output`.
2. Warnings are idempotent per category (the same warning is not emitted multiple times for the same cause — e.g., "N flow records skipped" is one warning, not N).
3. Warning content is deterministic.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | 5 malformed flow lines | One warning: "5 flow records skipped (malformed/unparseable)" |
| EC-002 | Zero-member zone warning + malformed flow warning | Two warnings; policy warning first (load-time), flow warning after |
| EC-003 | No warning conditions triggered | `ConformanceResult.warnings = []`; no stderr output |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Run with 3 malformed flows; no violations | Exit 0; stderr has 1 warning; `warnings: ["3 flow records skipped..."]` in JSON | edge-case |
| Run with no warnings | Exit as per violations; stderr empty | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.06.005-a | Warnings alone never change exit code | unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-010 ("Render outputs: ordered warnings to stderr") per capabilities.md §CAP-010 |
| L2 Domain Invariants | DI-019 ("Warnings emitted to stderr; collected in ConformanceResult.warnings; stable deterministic order; never change the exit code"), DI-013 (skipped flows → mandatory warning) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-010 ("Render outputs") per capabilities.md §CAP-010 — ordered warnings to stderr is an explicitly listed output behavior of CAP-010 per DI-019 |

## Related BCs

- BC-1.06.006 — --fail-on-skipped (the one flag that turns a warning into an exit-code change)
- BC-1.01.008 — zero-member zone warning (one of the warning sources)

## Architecture Anchors

- `architecture/SS-06-reporting.md#warnings` — warning collection and emission

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.06.005-a — warnings alone → no exit code change
