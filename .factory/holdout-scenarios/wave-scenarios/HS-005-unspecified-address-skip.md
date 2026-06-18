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
traces_to: "BC-1.02.003"
id: "HS-005"
category: "security-probes"
must_pass: "true"
priority: "must-pass"
epic_id: "E-2"
behavioral_contracts: [BC-1.02.003, BC-1.06.005]
lifecycle_status: active
introduced: zonewarden-greenfield
last_evaluated: null
staleness_check: null
stale_reason: null
retired: null
---

# Holdout Scenario: Unspecified Address Flows Are Skipped With Warning

## Scenario

1. A valid policy and a flow log containing three records:
   - Flow A: valid TCP flow (src=10.0.1.5, dst=10.0.2.10)
   - Flow B: TCP flow with `0.0.0.0` as the source address
   - Flow C: TCP flow with `::` (IPv6 unspecified) as the destination address
2. zonewarden is run.
3. Flow A is processed normally.
4. Flows B and C are skipped and NOT processed as violations.
5. `skipped` tally is 2.
6. `total_flows` is 1 (only Flow A counted).
7. Warnings about skipped flows appear on stderr (not stdout).
8. The DI-015 identity holds: `total_flows (1) == sum of five VerdictKind tallies`.

## Behavioral Contract Linkage

| BC ID | Clause Tested | Scenario Aspect |
|-------|--------------|-----------------|
| BC-1.02.003 | Skip unspecified-address flows + warn | Flows B and C skipped with warning |
| BC-1.06.005 | Warnings to stderr in deterministic order | Two warnings on stderr |

## Verification Approach

1. Policy: two zones, one conduit permitting Flow A.
2. Flow log: three lines; Flow B has `0.0.0.0` src; Flow C has `::` dst.
3. Run: `zonewarden --policy policy.yaml --flows flows.log --format json 2>stderr.txt`
4. Assert: exit code == 0 (Flow A is allowed, no violations)
5. Parse JSON: `skipped` == 2; `total_flows` == 1; `allowed` == 1
6. Read stderr.txt: contains 2 warning lines mentioning the skipped flows
7. Verify DI-015: `1 == 0 + 1 + 0 + 0 + 0`

## Evaluation Rubric

- **Functional correctness** (weight: 0.4): Is `skipped` == 2 and `total_flows` == 1?
- **Edge case handling** (weight: 0.3): Are warnings on stderr (not stdout)?
- **Error quality** (weight: 0.2): Do warning messages identify the unspecified address?
- **Data integrity** (weight: 0.1): DI-015 identity holds?

## Edge Conditions

- `0.0.0.0` as source must be skipped (not EXTERNAL-resolved and classified).
- `::` as destination must be skipped.
- These flows must NOT be counted in `total_flows` — only in `skipped`.

## Failure Guidance

"HOLDOUT LOW: HS-005 (satisfaction: 0.XX) — unspecified-address flows were not correctly skipped: they appeared in total_flows, or warnings were not emitted to stderr, or the skipped counter was wrong."
