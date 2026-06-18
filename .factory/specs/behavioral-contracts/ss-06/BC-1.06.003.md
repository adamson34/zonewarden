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

# BC-1.06.003: Emit human-readable text violations report

## Description

The default output format (`--format text` or when no format is specified) is a human-readable text report written to stdout. It summarizes the ConformanceResult: header with policy digest and run statistics, followed by a violations list with one violation per line showing src/dst zones, protocol, port, ViolationKind, Severity, and explanation. Heuristic service labels are visibly flagged with `[heuristic]`. Allowed flows are NOT listed (only violations and summary statistics). The report is deterministic (same inputs → same text).

## Preconditions

1. ConformanceResult is assembled.
2. `--format text` (default) is active.

## Postconditions

1. A human-readable text report is written to stdout (or `--output` file).
2. Each violation includes: flow_index, src_zone, dst_zone, src_ip:src_port, dst_ip:dst_port, proto, service (if any, flagged `[heuristic]` if PortHeuristic), ViolationKind, Severity, explanation.
3. Summary section shows: total_flows, intra_zone, allowed, no_matching_conduit, wrong_direction, multicast_exempt, idmz_bypasses, distinct_violating_flows, skipped, policy_digest.
4. Report is deterministic (violations in total-order sort; statistics deterministic).

## Invariants

1. Heuristic service names are always flagged `[heuristic]` in the text report.
2. The text report never shows internal implementation details.
3. The text is deterministic.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Zero violations | Report shows summary statistics; "No violations found"; exit 0 |
| EC-002 | Violation with `service=None` | Service field omitted from violation line |
| EC-003 | Violation with `service_source=PortHeuristic` | Service name shown with `[heuristic]` label |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 1 NoMatchingConduit violation; src_zone=field; dst_zone=dmz; TCP 502 | Line: `[0] field (10.0.1.5) → dmz (10.0.2.10) TCP:502 Modbus[heuristic] NoMatchingConduit ESTABLISHED: No conduit permits this flow` | happy-path |
| Zero violations | "Conformance check: PASS. No violations found." + statistics | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.06.003-a | Heuristic services always flagged in text output | unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-010 ("Render outputs: Emit a human-readable violations report") per capabilities.md §CAP-010 |
| L2 Domain Invariants | DI-008 (service provenance honesty: heuristics flagged), DI-009 (deterministic output) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-010 ("Render outputs") per capabilities.md §CAP-010 — human-readable report is an explicitly listed output of CAP-010 |

## Related BCs

- BC-1.06.002 — JSON report (parallel output format)
- BC-1.02.004 — service inference (produces service_source flag for display)

## Architecture Anchors

- `architecture/SS-06-reporting.md#text-report` — text report formatting

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.06.003-a — heuristic services flagged
