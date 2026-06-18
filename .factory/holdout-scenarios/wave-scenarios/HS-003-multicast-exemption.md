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
traces_to: "BC-1.03.003"
id: "HS-003"
category: "edge-case-combinations"
must_pass: "true"
priority: "must-pass"
epic_id: "E-3"
behavioral_contracts: [BC-1.03.003, BC-1.04.011]
lifecycle_status: active
introduced: zonewarden-greenfield
last_evaluated: null
staleness_check: null
stale_reason: null
retired: null
---

# Holdout Scenario: Multicast Traffic Is Exempt — No False Violations

## Scenario

1. A policy contains a single zone `field` (L2, `10.0.1.0/24`) with no conduits.
2. A flow log contains three flows:
   - Flow A: src=`10.0.1.5`, dst=`224.0.0.1` (IPv4 multicast), UDP
   - Flow B: src=`10.0.1.5`, dst=`255.255.255.255` (limited broadcast), UDP
   - Flow C: src=`10.0.1.5`, dst=`10.0.2.10` (non-existent zone, EXTERNAL), TCP/502
3. zonewarden is run.
4. The exit code is 1 (Flow C is a violation — no matching conduit for EXTERNAL dst).
5. The `multicast_exempt` tally is 2 (Flow A and B).
6. The `no_matching_conduit` tally is 1 (Flow C).
7. Flows A and B do NOT appear in the violations array.
8. The DI-015 accounting identity holds: `total_flows (3) == intra_zone (0) + allowed (0) + no_matching_conduit (1) + wrong_direction (0) + multicast_exempt (2)`.

## Behavioral Contract Linkage

| BC ID | Clause Tested | Scenario Aspect |
|-------|--------------|-----------------|
| BC-1.03.003 | Multicast short-circuits before zone resolution | Flows A and B dst = multicast/broadcast |
| BC-1.04.011 | MulticastExempt short-circuits before conduit eval | Flows A and B not in violations |

## Verification Approach

1. Policy: zone `field` (L2, 10.0.1.0/24), no conduits.
2. Flow log: three flows as described.
3. Run: `zonewarden --policy policy.yaml --flows flows.log --format json`
4. Assert: exit code == 1 (Flow C is violation)
5. Parse JSON: `multicast_exempt` == 2; `no_matching_conduit` == 1; `total_flows` == 3; `violations` has exactly one entry (Flow C)
6. Verify: `0 + 0 + 1 + 0 + 2 == 3` (DI-015)

## Evaluation Rubric

- **Functional correctness** (weight: 0.5): Are multicast flows exempt and the unicast flow correctly a violation?
- **Edge case handling** (weight: 0.3): Is the accounting identity satisfied exactly?
- **Data integrity** (weight: 0.2): Is `multicast_exempt` == 2 and `violations` has exactly 1 entry?

## Edge Conditions

- Multicast detection must precede zone resolution and conduit evaluation.
- The `255.255.255.255` limited broadcast is a distinct case from the CIDR directed-broadcast rule.

## Failure Guidance

"HOLDOUT LOW: HS-003 (satisfaction: 0.XX) — multicast exemption failed: multicast/broadcast flows were incorrectly reported as violations, or the accounting identity was violated."
