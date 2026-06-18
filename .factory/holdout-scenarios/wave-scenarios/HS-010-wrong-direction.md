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
traces_to: "BC-1.04.004"
id: "HS-010"
category: "behavioral-subtleties"
must_pass: "true"
priority: "must-pass"
epic_id: "E-4"
behavioral_contracts: [BC-1.04.004, BC-1.04.005]
lifecycle_status: active
introduced: zonewarden-greenfield
last_evaluated: null
staleness_check: null
stale_reason: null
retired: null
---

# Holdout Scenario: Directionality — WrongDirection vs NoMatchingConduit

## Scenario

1. Policy: zone `field` (L2, 10.0.1.0/24), zone `control` (L3, 10.0.2.0/24).
   One conduit: field→control, TCP/502, direction=forward.
2. Flow log: two flows:
   - Flow A: src=10.0.1.5:1234, dst=10.0.2.10:502, TCP/SF — correctly directed (field→control)
   - Flow B: src=10.0.2.10:502, dst=10.0.1.5:1234, TCP/SF — reverse-directed (control→field)
3. zonewarden is run.
4. Flow A: `Allowed` (conduit matches).
5. Flow B: `WrongDirection` (not `NoMatchingConduit` — the conduit exists but direction is wrong).
6. Exit code is 1 (violation: WrongDirection).
7. `wrong_direction` tally is 1; `no_matching_conduit` is 0.

## Behavioral Contract Linkage

| BC ID | Clause Tested | Scenario Aspect |
|-------|--------------|-----------------|
| BC-1.04.004 | Forward conduit rejects reverse flow | Flow B is WrongDirection |
| BC-1.04.005 | Bidirectional permits either direction (negative case) | This conduit is forward-only |

## Verification Approach

1. Policy: as described (field→control, TCP/502, forward).
2. Flow log: Flow A (field→control) and Flow B (control→field).
3. Run: `zonewarden --policy policy.yaml --flows flows.log --format json`
4. Assert: exit code == 1
5. Parse JSON: `allowed` == 1; `wrong_direction` == 1; `no_matching_conduit` == 0
6. In violations: `violations[0].kind` == `"WrongDirection"`

## Evaluation Rubric

- **Functional correctness** (weight: 0.6): Is WrongDirection reported (not NoMatchingConduit)?
- **Edge case handling** (weight: 0.3): Is `no_matching_conduit` == 0 (the conduit was found, just wrong direction)?
- **Data integrity** (weight: 0.1): `total_flows == allowed + wrong_direction` == 2.

## Edge Conditions

- The distinction between WrongDirection and NoMatchingConduit is critical for operator diagnosis.
- WrongDirection means "a conduit exists between these zones, but direction is wrong."
- NoMatchingConduit means "no conduit exists for these zones at all."

## Failure Guidance

"HOLDOUT LOW: HS-010 (satisfaction: 0.XX) — directionality check failed: reverse-direction flow was reported as NoMatchingConduit instead of WrongDirection, or was incorrectly allowed."
