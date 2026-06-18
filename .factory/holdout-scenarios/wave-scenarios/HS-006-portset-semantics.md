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
traces_to: "BC-1.04.006"
id: "HS-006"
category: "edge-case-combinations"
must_pass: "true"
priority: "must-pass"
epic_id: "E-4"
behavioral_contracts: [BC-1.04.006, BC-1.01.009]
lifecycle_status: active
introduced: zonewarden-greenfield
last_evaluated: null
staleness_check: null
stale_reason: null
retired: null
---

# Holdout Scenario: Portless Protocol Matches Only "Any" PortSet

## Scenario

1. A policy contains: zone `field` (L2, 10.0.1.0/24) and zone `control` (L3, 10.0.2.0/24).
   Two conduits:
   - Conduit A: field→control, ICMP, ports=[0-65535] (explicit range)
   - Conduit B: field→control, ICMP, ports=any
2. A flow log contains two ICMP flows:
   - ICMP Flow 1: src=10.0.1.5, dst=10.0.2.10 (should match Conduit B only)
   - ICMP Flow 2: src=10.0.1.5, dst=10.0.2.10 (same — tests any-match with both conduits)
3. zonewarden is run.
4. Both ICMP flows are `Allowed` (Conduit B matches via `ports: any`).
5. Conduit A with `[0-65535]` does NOT match the ICMP flows.
6. Exit code is 0 (conformant).

## Behavioral Contract Linkage

| BC ID | Clause Tested | Scenario Aspect |
|-------|--------------|-----------------|
| BC-1.04.006 | Portless protocol matches only ports:Any | ICMP not matched by [0-65535] |
| BC-1.01.009 | Any is distinct from [0-65535] | Semantic distinction preserved at match time |

## Verification Approach

1. Policy: as described. Conduit A has `ports: [0-65535]`; Conduit B has `ports: any`.
2. Flow log: two ICMP flow lines (proto=icmp, no ports).
3. Run: `zonewarden --policy policy.yaml --flows flows.log --format json`
4. Assert: exit code == 0
5. Parse JSON: `allowed` == 2; `no_matching_conduit` == 0; `violations` == []
6. Inspect (if debugging): conduit_idx in allowed verdict points to Conduit B (index 1), not A (index 0)

## Evaluation Rubric

- **Functional correctness** (weight: 0.6): Are ICMP flows allowed (matched by `any` conduit)?
- **Edge case handling** (weight: 0.3): Is `[0-65535]` correctly NOT matching portless ICMP?
- **Data integrity** (weight: 0.1): Exit code 0; no violations.

## Edge Conditions

- `PortSet::Any` is the ONLY portset that matches portless protocols (ICMP, Other).
- `PortSet::Ranges([0-65535])` explicitly does NOT match portless — they are semantically distinct.

## Failure Guidance

"HOLDOUT LOW: HS-006 (satisfaction: 0.XX) — PortSet semantics incorrect: ICMP flows were not allowed (PortSet::Any not matching portless protocol) or were incorrectly allowed by [0-65535] range."
