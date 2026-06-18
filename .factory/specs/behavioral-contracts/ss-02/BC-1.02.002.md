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
subsystem: "SS-02"
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

# BC-1.02.002: Skip and count malformed flow lines; never abort the run

## Description

Individual malformed or unparseable flow records in the Zeek conn.log must never abort the run. They are skipped, the `skipped` counter is incremented, and processing continues with the next line. This implements the "fail loud on policy, degrade gracefully on flows" asymmetry (DI-013). After all flows are processed, if `skipped > 0`, a mandatory warning is emitted and included in the ConformanceResult (DI-013, DEC-024). The `--fail-on-skipped` flag changes the exit code when `skipped > 0` (see BC-1.06.006).

## Preconditions

1. The flow file is readable (otherwise FM-003 applies).
2. A specific line in the file cannot be parsed into a valid `Flow` (missing required fields, type errors, etc.).

## Postconditions

1. The malformed line is discarded.
2. `skipped` counter incremented by 1.
3. `flow_index` for subsequent successfully-parsed flows is NOT affected (gap-free indexing continues among successful flows only).
4. Processing continues with the next line.
5. After all lines: if `skipped > 0`, a warning "N flow records skipped (malformed/unparseable)" is added to `ConformanceResult.warnings`.

## Invariants

1. Malformed lines never abort the run.
2. Malformed lines never contribute to `total_flows` — only `skipped`.
3. `flow_index` is dense over successfully-normalized flows only.
4. The warning is always emitted when `skipped > 0` (even if no violations were found).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All lines in the file are malformed | `total_flows = 0`, `skipped = N`; exit 0 (or non-zero with `--fail-on-skipped`); ConformanceResult with all-zero tallies plus skipped count |
| EC-002 | File has 1000 lines, 5 malformed interspersed | `total_flows = 995`, `skipped = 5`; flow_index 0..994 (dense) |
| EC-003 | Malformed line followed immediately by a valid line | Both handled independently; only valid line counted in total_flows |
| EC-004 | Line with valid syntax but src_ip = `0.0.0.0` | Skipped by BC-1.02.003 rule (unspecified address); also increments `skipped` |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| conn.log with one valid line and one truncated line | `total_flows=1, skipped=1`; warning in ConformanceResult | edge-case |
| conn.log with all valid lines | `skipped=0`; no warning | happy-path |
| conn.log with all malformed lines | `total_flows=0, skipped=N`; warning; exit 0 (or exit 1 if --fail-on-skipped) | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.02.002-a | Malformed lines never cause a panic or process abort | fuzz |
| VP-1.02.002-b | `total_flows + skipped == count(non-header lines in file)` | proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003 ("Ingest flows: Unparseable flow lines are skipped + counted, never abort the run (DI-013)") per capabilities.md §CAP-003 |
| L2 Domain Invariants | DI-013 ("Ingest resilience asymmetry: malformed flow record is skipped and counted, never aborting the run") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-003 ("Ingest flows") per capabilities.md §CAP-003 — the skip-and-count behavior for malformed lines is the explicit resilience requirement of CAP-003 per DI-013 |

## Related BCs

- BC-1.02.001 — happy-path flow parse (companion)
- BC-1.02.003 — unspecified address skip (same counter, different rule)
- BC-1.06.006 — --fail-on-skipped flag (modifies exit code when skipped > 0)

## Architecture Anchors

- `architecture/SS-02-flow-ingest.md#skip-handling` — malformed record handling

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.02.002-a — fuzz: no panic on malformed input
- VP-1.02.002-b — total_flows + skipped accounting identity
