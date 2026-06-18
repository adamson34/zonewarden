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
traces_to: "BC-1.04.002"
id: "HS-001"
category: "behavioral-subtleties"
must_pass: "true"
priority: "must-pass"
epic_id: "E-4"
behavioral_contracts: [BC-1.04.002, BC-1.04.003]
lifecycle_status: active
introduced: zonewarden-greenfield
last_evaluated: null
staleness_check: null
stale_reason: null
retired: null
---

# Holdout Scenario: Deny-by-Default — No Conduit = Violation

## Scenario

1. A valid policy is loaded containing two zones (zone A and zone B) with no conduit declared between them.
2. A flow log is provided containing one TCP flow from a host in zone A to a host in zone B on port 8080.
3. zonewarden is run against this policy and flow log.
4. The tool exits with code 1.
5. The output (text or JSON) contains exactly one violation entry for this flow.
6. The violation kind identifies the absence of a permitted conduit (not a direction error).
7. The flow does NOT appear in any allow tally.

## Behavioral Contract Linkage

| BC ID | Clause Tested | Scenario Aspect |
|-------|--------------|-----------------|
| BC-1.04.002 | Deny-by-default postcondition | No matching conduit → violation verdict |
| BC-1.04.003 | Any-match union (negative case) | Zero conduits means zero matches |

## Verification Approach

1. Create a policy YAML with two zones: `field` (L2, `10.0.1.0/24`) and `control` (L3, `10.0.2.0/24`). No conduits.
2. Create a Zeek conn.log with one valid TCP flow: src=`10.0.1.5:1234`, dst=`10.0.2.10:8080`, proto=tcp, SF state.
3. Run: `zonewarden --policy policy.yaml --flows flows.log --format json`
4. Assert: exit code == 1
5. Parse JSON: `violations` array length == 1; `no_matching_conduit` == 1; `allowed` == 0; violation `kind` == `"NoMatchingConduit"`

## Evaluation Rubric

- **Functional correctness** (weight: 0.5): Does exit code == 1 and JSON contain exactly one NoMatchingConduit violation? 
- **Edge case handling** (weight: 0.2): Is `allowed` tally zero (not accidentally allowed)?
- **Error quality** (weight: 0.2): Is the explanation field non-empty and informative?
- **Data integrity** (weight: 0.1): Does `total_flows == no_matching_conduit` (accounting identity)?

## Edge Conditions

- The policy has zones but zero conduits — the most aggressive deny-all configuration.
- The flow is from an explicit zone member, not EXTERNAL.

## Failure Guidance

"HOLDOUT LOW: HS-001 (satisfaction: 0.XX) — zonewarden did not correctly apply deny-by-default: a flow with no matching conduit was not reported as a violation."
