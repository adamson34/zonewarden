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

# BC-1.02.006: Enforce max_flows ingest cap; abort with exit 2 on breach

## Description

To prevent unbounded memory use and overflow risk, the ingest stage enforces a configurable maximum flow count. When the number of successfully-normalized flows reaches `max_flows`, the run is aborted immediately with exit code 2 and a diagnostic message. The default cap is `1,000,000` flows (far below `u64::MAX`). No partial ConformanceResult or report is emitted on cap-abort — the run produces no output (clean abort). The cap breach is distinct from a violations exit (exit 1) and a policy error (also exit 2, but with a different diagnostic).

**Default cap:** `1_000_000` flows (configurable via `--max-flows <N>`).

## Preconditions

1. Flow ingest is in progress (ST-3).
2. The count of successfully-normalized flows has reached the `max_flows` threshold.

## Postconditions

1. Process exits with code `2`.
2. Stderr emits a diagnostic: `"Ingest cap exceeded: max_flows = <N>. Run aborted after <N> flows. Re-run with a higher --max-flows cap or split the input."`.
3. No ConformanceResult is assembled.
4. No output artifacts (report, JSON, Mermaid) are written.
5. Any partial in-memory state is discarded.

## Invariants

1. The cap check is applied to `successfully-normalized flows` only; skipped records do not consume cap quota.
2. `max_flows` must be `> 0`; a value of 0 is a usage error (exit 2, different diagnostic).
3. The cap applies globally to the entire input; there is no per-file sub-limit.
4. The default cap is `1_000_000`; this is a constant that may be increased in future versions.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `--max-flows 0` | Exit 2; usage error: "max_flows must be > 0" |
| EC-002 | Input has exactly `max_flows` flows | Run completes normally; no cap breach (cap is a strict `>` check: breach at `> max_flows`) |
| EC-003 | Input has `max_flows + 1` flows | Cap breach at the `max_flows + 1` th flow; exit 2 |
| EC-004 | Input has many malformed lines (high `skipped`) but fewer than `max_flows` valid flows | No cap breach; cap applies to valid flows only |
| EC-005 | `--max-flows` not specified | Default of `1_000_000` applies |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| conn.log with 500,000 flows; `--max-flows 1000000` | Run completes normally | happy-path |
| conn.log with 1,000,001 flows; `--max-flows 1000000` | Exit 2; cap breach diagnostic | error |
| `--max-flows 0` | Exit 2; usage error | error |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.02.006-a | `flow_count > max_flows` always produces exit 2 | proptest |
| VP-1.02.006-b | No output artifact is written on cap breach | unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003 ("Ingest flows") per capabilities.md §CAP-003 |
| L2 Domain Invariants | FM-008 ("A configured max_flows ingest cap enforced at ST-3; reaching it aborts with exit code 2 + diagnostic") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-003 ("Ingest flows") per capabilities.md §CAP-003 — the ingest cap is a required safety property of the flow ingestion stage per FM-008 |

## Related BCs

- BC-1.05.004 — u64 overflow guard (FM-009; defense-in-depth backstop behind this cap)
- BC-1.06.001 — exit code semantics

## Architecture Anchors

- `architecture/SS-02-flow-ingest.md#ingest-cap` — max_flows cap enforcement

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.02.006-a — cap breach → exit 2
- VP-1.02.006-b — no output on cap breach
