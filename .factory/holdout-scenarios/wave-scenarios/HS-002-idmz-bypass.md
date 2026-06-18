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
traces_to: "BC-1.04.007"
id: "HS-002"
category: "security-probes"
must_pass: "true"
priority: "must-pass"
epic_id: "E-4"
behavioral_contracts: [BC-1.04.007, BC-1.04.008]
lifecycle_status: active
introduced: zonewarden-greenfield
last_evaluated: null
staleness_check: null
stale_reason: null
retired: null
---

# Holdout Scenario: IDMZ Bypass — Allowed Flow Still Flags Security Violation

## Scenario

1. A policy contains: an OT zone `field` (L2, `10.0.1.0/24`), an IT zone `historian` (L4, `10.0.4.0/24`), and an IDMZ zone `dmz` (L3.5, `10.0.3.0/24`). A conduit is declared from `field` to `historian` (TCP/any, forward) — the conduit explicitly permits this flow.
2. A flow log contains one TCP flow: src=`10.0.1.5`, dst=`10.0.4.10`, TCP/1234. The conduit covers it.
3. zonewarden is run.
4. The tool exits with code 1 (not 0), despite the flow being conduit-allowed.
5. The JSON output shows `allowed == 1` AND `idmz_bypasses == 1`.
6. The violations array contains an `IdmzBypass` entry for this flow.
7. The `distinct_violating_flows` is 1.

## Behavioral Contract Linkage

| BC ID | Clause Tested | Scenario Aspect |
|-------|--------------|-----------------|
| BC-1.04.007 | Postcondition 1 (idmz_bypass = true) | OT↔IT without IDMZ endpoint |
| BC-1.04.007 | Postcondition 4 (additive) | Allowed AND idmz_bypass both true |
| BC-1.04.008 | EXTERNAL exclusion (negative test) | No EXTERNAL endpoints involved |

## Verification Approach

1. Policy: `field` (L2, 10.0.1.0/24), `historian` (L4, 10.0.4.0/24), `dmz` (L3.5, 10.0.3.0/24). Conduit: field→historian, TCP, any, forward.
2. Flow: src=10.0.1.5:1234, dst=10.0.4.10:4840, tcp, SF.
3. Run: `zonewarden --policy policy.yaml --flows flows.log --format json`
4. Assert: exit code == 1 (IDMZ bypass overrides exit 0)
5. Parse JSON: `allowed` == 1 (conduit matches); `idmz_bypasses` == 1; `distinct_violating_flows` == 1; `violations[0].kind` == `"IdmzBypass"`

## Evaluation Rubric

- **Functional correctness** (weight: 0.6): Does the tool exit 1 AND report BOTH allowed AND idmz_bypass?
- **Edge case handling** (weight: 0.2): Is the conduit verdict `Allowed` (not NoMatchingConduit)?
- **Error quality** (weight: 0.1): Does the IdmzBypass violation have an explanation?
- **Data integrity** (weight: 0.1): Does `allowed + no_matching_conduit + wrong_direction + intra_zone + multicast_exempt == total_flows`?

## Edge Conditions

- The flow IS explicitly permitted by a conduit. The IDMZ bypass is an additive finding on top.
- This is the primary security control of IEC 62443 — failing this scenario is a critical defect.

## Failure Guidance

"HOLDOUT LOW: HS-002 (satisfaction: 0.XX) — IDMZ bypass detection failed: either exit code was 0 (no bypass detected), or the allowed tally was incorrect, or the IdmzBypass violation was missing from the output."
