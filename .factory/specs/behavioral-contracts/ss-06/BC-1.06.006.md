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
capability: "CAP-003"
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

# BC-1.06.006: --fail-on-skipped: skipped > 0 forces non-zero exit

## Description

The `--fail-on-skipped` flag changes the exit code semantics for runs with skipped flow records. Normally a run with `skipped > 0` exits 0 if there are no violations (the warning is informational). With `--fail-on-skipped`, if `skipped > 0`, the exit code is upgraded to 1 even if there are no violations. This supports strict CI pipelines that treat any data loss as a gate failure. The flag does NOT change the ConformanceResult contents or the warning messages — only the exit code.

## Preconditions

1. `--fail-on-skipped` flag is present on the command line.
2. `skipped > 0` (at least one flow record was skipped).

## Postconditions

1. Exit code = 1 (even if `distinct_violating_flows == 0` and `idmz_bypasses == 0`).
2. The mandatory warning for skipped flows is still emitted to stderr.
3. The ConformanceResult and report contents are unchanged.

## Invariants

1. `--fail-on-skipped` only affects exit code when `skipped > 0`; if `skipped == 0`, the flag has no effect.
2. If violations are also present (`distinct_violating_flows > 0` OR `idmz_bypasses > 0`), the exit is still 1 (overlapping — both conditions produce exit 1).
3. `--fail-on-skipped` never produces exit 2.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `--fail-on-skipped`; `skipped=5`; no violations | Exit 1 |
| EC-002 | `--fail-on-skipped`; `skipped=0`; no violations | Exit 0 (flag has no effect) |
| EC-003 | `--fail-on-skipped`; `skipped=5`; also has violations | Exit 1 (violations also trigger exit 1) |
| EC-004 | No `--fail-on-skipped`; `skipped=5`; no violations | Exit 0; warning only |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `--fail-on-skipped`; 1 skipped flow; 0 violations | Exit 1; warning in stderr | edge-case |
| No `--fail-on-skipped`; 1 skipped flow; 0 violations | Exit 0; warning in stderr | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.06.006-a | `--fail-on-skipped` with `skipped > 0` always produces exit 1 | unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003 ("Ingest flows: skipped > 0 exits 0 with a mandatory warning unless --fail-on-skipped is set (DEC-024)") per capabilities.md §CAP-003 |
| L2 Domain Invariants | DI-013 ("clean run with skipped > 0 exits 0 with a mandatory warning unless --fail-on-skipped is set (DEC-024)"), DEC-024 |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-003 ("Ingest flows") per capabilities.md §CAP-003 — the --fail-on-skipped flag is an explicitly listed behavior modifier of the ingest capability per DI-013 and DEC-024 |

## Related BCs

- BC-1.06.001 — exit code semantics
- BC-1.06.005 — warnings to stderr

## Architecture Anchors

- `architecture/SS-06-reporting.md#fail-on-skipped` — --fail-on-skipped flag handling

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.06.006-a — --fail-on-skipped with skipped > 0 → exit 1
