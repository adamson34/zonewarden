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
traces_to: "BC-1.05.002"
id: "HS-004"
category: "behavioral-subtleties"
must_pass: "true"
priority: "must-pass"
epic_id: "E-5"
behavioral_contracts: [BC-1.05.002, BC-1.05.003]
lifecycle_status: active
introduced: zonewarden-greenfield
last_evaluated: null
staleness_check: null
stale_reason: null
retired: null
---

# Holdout Scenario: Deterministic Output — Identical Inputs Produce Identical Output

## Scenario

1. A valid policy and flow log are provided.
2. zonewarden is run twice consecutively on the same inputs with `--format json --output run1.json` and `--output run2.json`.
3. The two output files are byte-identical (`sha256sum run1.json == sha256sum run2.json`).
4. The `policy_digest` field in both JSONs is the same 64-character lowercase hex string.
5. The `violations` array in both JSONs is in the same order.

## Behavioral Contract Linkage

| BC ID | Clause Tested | Scenario Aspect |
|-------|--------------|-----------------|
| BC-1.05.002 | Total-order violation sort | Violations in same order across runs |
| BC-1.05.003 | Stable policy digest | `policy_digest` identical across runs |

## Verification Approach

1. Use the standard test policy (2 zones, 1 conduit, 2 violations, 1 IDMZ bypass) and a fixed flow log.
2. Run 1: `zonewarden --policy policy.yaml --flows flows.log --format json --output run1.json`; capture exit code.
3. Run 2: `zonewarden --policy policy.yaml --flows flows.log --format json --output run2.json`; capture exit code.
4. Assert: exit codes are equal.
5. Assert: `sha256sum run1.json` == `sha256sum run2.json` (byte-identical output).
6. Parse both JSONs: `policy_digest` fields are equal; `violations` arrays are equal (same order, same content).

## Evaluation Rubric

- **Functional correctness** (weight: 0.6): Are outputs byte-identical?
- **Data integrity** (weight: 0.4): Is `policy_digest` consistent and violations order identical?

## Edge Conditions

- Any source of non-determinism (hash map iteration order, system time, random sort) will cause this scenario to fail.
- The sort key must be the exact 7-tuple: `(ts, src_ip, src_port, dst_ip, dst_port, proto, flow_index)`.

## Failure Guidance

"HOLDOUT LOW: HS-004 (satisfaction: 0.XX) — output is not deterministic: two runs on identical inputs produced different output files or different violation ordering."
