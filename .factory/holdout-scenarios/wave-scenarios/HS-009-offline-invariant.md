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
traces_to: "BC-1.06.007"
id: "HS-009"
category: "security-probes"
must_pass: "true"
priority: "must-pass"
epic_id: "E-6"
behavioral_contracts: [BC-1.06.007]
lifecycle_status: active
introduced: zonewarden-greenfield
last_evaluated: null
staleness_check: null
stale_reason: null
retired: null
---

# Holdout Scenario: zonewarden Never Opens a Network Socket

## Scenario

1. A valid policy and flow log are provided.
2. zonewarden is run in a network-restricted environment (e.g., with `unshare --net` on Linux, or firewall blocking all outbound) or verified via `strace -e trace=network`.
3. The tool completes successfully (exit 0 or 1 as appropriate).
4. No network syscalls (`socket`, `connect`, `bind`, `sendto`) were made during the run.
5. The tool produces correct output despite the network restriction.

## Behavioral Contract Linkage

| BC ID | Clause Tested | Scenario Aspect |
|-------|--------------|-----------------|
| BC-1.06.007 | No network socket opened | Zero network syscalls during full run |

## Verification Approach

**On Linux:**
```bash
strace -e trace=network zonewarden --policy policy.yaml --flows flows.log --format json 2>&1 | grep -E 'socket|connect|bind' | grep -v "ENOENT\|EACCES"
```
Expected: empty output (no network calls).

**Alternative:** Run inside `unshare --net` (network namespace with no interfaces):
```bash
unshare --net zonewarden --policy policy.yaml --flows flows.log
```
Expected: exits 0 or 1 (no network error).

**Structural test:** `cargo deny check` with `deny = ["net"]` capability should pass at build time.

## Evaluation Rubric

- **Functional correctness** (weight: 0.6): Does the tool run successfully with zero network syscalls?
- **Error quality** (weight: 0.2): Does the tool produce correct output in the isolated environment?
- **Data integrity** (weight: 0.2): Does `cargo deny check` pass (build-time enforcement)?

## Edge Conditions

- This is a structural guarantee — satisfied by construction (no `std::net` imports in `zonewarden-core`).
- The evaluator should verify both runtime behavior (strace/unshare) and static analysis (cargo deny).

## Failure Guidance

"HOLDOUT LOW: HS-009 (satisfaction: 0.XX) — offline invariant violated: zonewarden attempted a network syscall during a run with network access restricted."
