---
document_type: wave-gate-report
wave: wave-5
cycle: zonewarden-greenfield
status: passed
date: 2026-06-26
producer: wave-gate
---

# Wave Gate: wave-5 (FINAL — completes Phase 3)

Wave 5 stories: S-5.02 (aggregator), S-5.03 (deterministic sort), S-6.01 (reporter),
S-6.02 (atomic write + warnings), S-6.03 (CLI). This wave makes zonewarden a
runnable end-to-end MVP — so for the first time the demo (Gate 4) and holdout
(Gate 5) gates could execute against a real binary (deferred since Wave 2).

## Result

```
  Gate 1 — Test Suite:       ✅ PASS (187 tests, 0 failures; clippy -D + fmt clean; release binary builds)
  Gate 2 — DTU Validation:   ⏭️ SKIP (no DTU-covered modules in this wave)
  Gate 3 — Adversarial:      ✅ PASS (after fixes — 2 HIGH + 2 MED found & resolved, 0 CRITICAL)
  Gate 4 — Demo Evidence:    ✅ PASS (8 CLI scenarios captured from the binary)
  Gate 5 — Holdout Eval:     ✅ PASS (mean 0.97, min critical 0.85; HS-001..010 all ≥ 0.85)
  Gate 6 — State Update:     ✅ PASS

  GATE_CHECK: gate=1 name=test-suite status=pass note=187 tests, 0 failures
  GATE_CHECK: gate=2 name=dtu-validation status=skip note=no DTU-covered modules in wave
  GATE_CHECK: gate=3 name=adversarial-review status=pass note=novelty HIGH; 2 HIGH + 2 MED found and fixed, 0 critical
  GATE_CHECK: gate=4 name=demo-evidence status=pass note=8 CLI scenarios captured from running binary
  GATE_CHECK: gate=5 name=holdout-eval status=pass note=mean 0.97, min critical 0.85
  GATE_CHECK: gate=6 name=state-update status=pass note=sprint-state + STATE updated; Phase 3 complete

  WAVE GATE: ✅ PASSED — Phase 3 (TDD Implementation) COMPLETE
```

## Mutation testing
All 5 wave stories are `tdd_mode: strict` (zero facade / zero `mutation_testing_required`).
Per BC-8.30.002, mutation testing step skipped — no facade stories in wave.

## Gate 3 — adversarial findings (all resolved before pass)

The machine-readable spine (exit codes BC-1.06.001, JSON BC-1.06.002, aggregation
BC-1.05.001/002/004, atomic write BC-1.06.008, determinism, offline) was verified
clean. Defects clustered in the two human-facing reporters and were all fixed
(commit 8b183d5):

| ID | Sev | BC | Issue | Fix |
|----|-----|----|----|-----|
| WAVE5-001 | HIGH | BC-1.06.004 | Mermaid emitted zone ids as raw node identifiers → invalid Mermaid for a valid policy (id with space/quote) | Synthetic `z{i}` aliases; raw id only in quoted, `"`-escaped label; edges use aliases |
| WAVE5-002 | HIGH | BC-1.06.003 PC2 | Text violation lines omitted src_ip/dst_ip/service/explanation | Full violation line: zones, kind, severity, endpoints, service [heuristic], explanation |
| WAVE5-003 | MED | BC-1.06.003 PC3 | Text summary omitted intra_zone/allowed/multicast_exempt/distinct/policy_digest | Full summary block with all tallies + policy_digest |
| WAVE5-004 | MED | BC-1.06.005 inv1/2 | emit_text re-printed warnings to stdout (skipped notice 3×) | Warnings go to stderr only (via CLI); skipped count is a summary stat |

## Gate 5 — holdout findings

| Scenario | must_pass | sat | note |
|----------|-----------|-----|------|
| HS-001 deny-by-default | yes | 1.00 | |
| HS-002 idmz-bypass | yes | 1.00 | |
| HS-003 multicast-exemption | yes | 1.00 | |
| HS-004 determinism | yes | 1.00 | 3 runs byte-identical |
| HS-005 unspecified-skip | yes | 0.85 | non-blocking: aggregate skip warning doesn't name the 0.0.0.0/:: address |
| HS-006 portset-semantics | yes | 1.00 | `[0-65535]` ≠ portless; only `any` matches portless ICMP |
| HS-007 ingest-cap | yes | 1.00 | exit 2, no output file written |
| HS-008 accounting-identity | yes | 1.00 | identity exact; no regression from the WrongDirection fix |
| HS-009 offline-invariant | yes | 0.85 | non-blocking: only libSystem linked, zero network symbols; hard strace/cargo-deny proof deferred to Linux/Phase 6 |
| HS-010 wrong-direction | yes | 1.00 | **was 0.20** — fixed (commit a413cd2) |

**HS-010 (critical) initially scored 0.20** — the reverse 4-tuple of a permitted
flow was NoMatchingConduit instead of WrongDirection (conduit port matched only
against dst_port; in return traffic the service port is on src). Fixed: reverse
orientation now matches the conduit port against src OR dst port. Re-evaluation
mean 0.97, min critical 0.85.

## Backlog filed (non-blocking)

- **WAVE5-BL-001** (HS-005, LOW): per-flow skip warnings naming the unspecified address (currently one aggregate line). Polish.
- **WAVE5-BL-002** (HS-009, verification): run offline invariant under Linux `strace -e trace=network` + `unshare --net` and `cargo deny` — Phase 6 formal hardening.
- **BC-1.04.004 clarification** (spec): the "flow … on port 502" prose is ambiguous about which side the service port is on; the implementation now treats both dst-side (reach-in) and src-side (return traffic) as WrongDirection. Confirm/record in the BC.
- Carried from prior gates: WAVE4-001 (multicast DstKind vs MatchKind — PO/spec), WAVE3-001 (validator warnings-on-error — PO), WAVE3-005 (dup-id spans — spec).
