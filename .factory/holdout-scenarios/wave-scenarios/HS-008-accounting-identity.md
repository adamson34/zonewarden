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
traces_to: "BC-1.05.001"
id: "HS-008"
category: "behavioral-subtleties"
must_pass: "true"
priority: "must-pass"
epic_id: "E-5"
behavioral_contracts: [BC-1.05.001, BC-1.04.010]
lifecycle_status: active
introduced: zonewarden-greenfield
last_evaluated: null
staleness_check: null
stale_reason: null
retired: null
---

# Holdout Scenario: Accounting Identity Holds Across All Verdict Kinds

## Scenario

1. A policy with three zones: `field` (L1, 10.0.1.0/24), `control` (L2, 10.0.2.0/24), `historian` (L4, 10.0.4.0/24).
   Conduits: field→control (TCP/502, forward); no conduit field↔historian.
   A multicast zone: 224.0.0.0/4 is NOT declared (handled implicitly).
2. A flow log contains 7 flows:
   - F1: field→field (intra-zone, 10.0.1.5→10.0.1.10, TCP)
   - F2: field→control (TCP/502, SF — allowed by conduit)
   - F3: control→field (TCP/502, SF — WrongDirection; conduit is forward only)
   - F4: field→historian (TCP/80, SF — NoMatchingConduit)
   - F5: field→224.0.0.1 (ICMP multicast — MulticastExempt)
   - F6: control→historian (TCP/80, SF — NoMatchingConduit AND IdmzBypass additive)
   - F7: field→10.0.5.1 (no zone — EXTERNAL; flows to EXTERNAL are allowed if conduit permits or denied)
3. zonewarden is run.
4. The accounting identity holds: `total_flows == intra_zone + allowed + no_matching_conduit + wrong_direction + multicast_exempt`.
5. `idmz_bypasses` is counted separately (F6 triggers it).
6. `distinct_violating_flows` ≤ `total_flows`.

## Behavioral Contract Linkage

| BC ID | Clause Tested | Scenario Aspect |
|-------|--------------|-----------------|
| BC-1.05.001 | DI-015 accounting identity | All five tallies sum to total_flows |
| BC-1.04.010 | Verdict totality | Every flow gets exactly one VerdictKind |

## Verification Approach

1. Construct policy and 7-flow log as described.
2. Run: `zonewarden --policy policy.yaml --flows flows.log --format json`
3. Parse JSON and verify: `total_flows == intra_zone + allowed + no_matching_conduit + wrong_direction + multicast_exempt`
4. Verify specific counts match expected (F1=intra, F2=allowed, F3=wrong_dir, F4=NMC, F5=multicast, F6=NMC+idmz_bypass, F7 depends on conduit coverage)
5. Verify: `idmz_bypasses` >= 1 (F6); `distinct_violating_flows` >= 2 (F3, F4, F6)

## Evaluation Rubric

- **Functional correctness** (weight: 0.4): Is the accounting identity exactly satisfied?
- **Edge case handling** (weight: 0.3): Does the mixed scenario (all 5 verdict kinds) produce correct tallies?
- **Data integrity** (weight: 0.3): Are `idmz_bypasses` and `distinct_violating_flows` consistent with the violations array?

## Edge Conditions

- F6 is simultaneously `NoMatchingConduit` AND has `idmz_bypass = true`. It must be counted in `no_matching_conduit` (not a separate bucket) while ALSO being in `idmz_bypasses`.
- `external_endpoints` is a diagnostic counter and NOT part of the identity.

## Failure Guidance

"HOLDOUT LOW: HS-008 (satisfaction: 0.XX) — accounting identity violation: `total_flows != intra_zone + allowed + no_matching_conduit + wrong_direction + multicast_exempt`, or idmz_bypasses tally was incorrect for flows with both conduit verdict and IDMZ bypass."
