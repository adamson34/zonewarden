---
document_type: holdout-scenario
level: ops
version: "1.0"
status: draft
producer: story-writer
timestamp: 2026-06-17T00:00:00
phase: 2
inputs: [stories/, behavioral-contracts/, prd.md]
input-hash: "[md5]"
traces_to: "BC-1.02.006"
id: "HS-007"
category: "behavioral-subtleties"
must_pass: "true"
priority: "must-pass"
epic_id: "E-2"
behavioral_contracts: [BC-1.02.006]
lifecycle_status: active
introduced: zonewarden-greenfield
last_evaluated: null
staleness_check: null
stale_reason: null
retired: null
---

# Holdout Scenario: Ingest Cap Breach Aborts Cleanly — No Partial Output

## Scenario

1. A valid policy is loaded.
2. A flow log contains 10 valid flows.
3. zonewarden is run with `--max-flows 5`.
4. The tool processes 5 flows and then hits the cap.
5. The tool exits with code 2 (error condition).
6. No JSON/text/Mermaid report is written (no `--output` file created; stdout is empty or contains only a diagnostic).
7. Stderr contains a diagnostic message identifying the cap value (5) and that the cap was breached.
8. No partial ConformanceResult is emitted.

## Behavioral Contract Linkage

| BC ID | Clause Tested | Scenario Aspect |
|-------|--------------|-----------------|
| BC-1.02.006 | Abort with exit 2 on cap breach | Exit code is 2 |
| BC-1.02.006 | No partial output (OQ-006) | No report written |

## Verification Approach

1. Policy: standard 2-zone 1-conduit policy.
2. Flow log: 10 valid TCP flow lines.
3. Run: `zonewarden --policy policy.yaml --flows flows.log --max-flows 5 --format json`
4. Assert: exit code == 2
5. Assert: stdout is empty (no JSON emitted)
6. Assert: stderr contains "cap" or "max_flows" and "5"
7. If `--output out.json` used: assert `out.json` does not exist

## Evaluation Rubric

- **Functional correctness** (weight: 0.5): Exit code == 2; no JSON on stdout.
- **Error quality** (weight: 0.3): Does stderr identify the cap value and count?
- **Data integrity** (weight: 0.2): No partial output file exists.

## Edge Conditions

- Cap breach must abort before any report is emitted (DI-009: all-or-nothing output).
- The cap counts only successfully-normalized flows (malformed lines don't count toward cap).

## Failure Guidance

"HOLDOUT LOW: HS-007 (satisfaction: 0.XX) — ingest cap enforcement failed: either exit code was not 2, partial output was emitted, or stderr did not describe the cap breach."
