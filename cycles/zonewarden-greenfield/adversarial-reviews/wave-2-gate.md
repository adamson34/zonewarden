---
document_type: wave-gate-report
level: ops
wave: wave-2
status: passed
producer: wave-gate
timestamp: 2026-06-26T00:00:00
cycle: zonewarden-greenfield
stories: [S-1.02, S-4.01, S-2.01]
---

# Wave 2 Integration Gate — zonewarden

Stories: **S-1.02** (policy YAML load), **S-4.01** (severity grading), **S-2.01**
(Zeek conn.log parser). Delivery model: inline TDD on `dev` (no per-story
worktree/PR/develop branch). Gate run 2026-06-26.

## Gate Results

| Gate | Name | Status | Note |
|------|------|--------|------|
| 1 | Test suite (release) | ✅ PASS | 48 tests (10 core + 22 zeek + 11 policy + 5 severity), 0 failures; clippy `-D warnings` clean; `cargo +nightly fmt --check` clean |
| 2 | DTU validation | ⏭️ SKIP | `dtu_required: false`; no DTU clones; no DTU-covered modules in wave |
| — | Mutation testing | ⏭️ SKIP | All 3 wave stories are `tdd_mode: strict` (zero facade / Option-B) — no mutation obligation |
| 3 | Adversarial review of wave diff | ✅ PASS (after fixes) | 0 CRITICAL, 2 HIGH (both fixed), 1 MEDIUM + 3 LOW (dispositioned). See below |
| 4 | Demo evidence | ⏭️ SKIP | No runnable surface yet — CLI/pipeline is S-6.03 (Wave 5). Library behavior evidenced by the test suite; demos deferred to the wave that ships an executable |
| 5 | Holdout evaluation | ⏭️ SKIP | Holdout scenarios (HS-001..010) are end-to-end conformance runs requiring the full pipeline + CLI (Wave 5). Not executable until then |
| 6 | State update | ✅ PASS | sprint-state + STATE.md updated; this report committed |

```
GATE_CHECK: gate=1 name=test-suite status=pass note=48 tests, 0 failures, clippy+fmt clean
GATE_CHECK: gate=2 name=dtu-validation status=skip note=dtu_required=false, no DTU modules in wave
GATE_CHECK: gate=3 name=adversarial-review status=pass note=0 critical, 2 HIGH fixed, 3 LOW backlogged
GATE_CHECK: gate=4 name=demo-evidence status=skip note=no runnable surface until CLI (S-6.03, wave 5)
GATE_CHECK: gate=5 name=holdout-eval status=skip note=end-to-end scenarios need full pipeline (wave 5)
GATE_CHECK: gate=6 name=state-update status=pass note=sprint-state + STATE updated
```

## Gate 3 Findings & Dispositions

Reviewer: fresh-context adversary (read-only). Bootstrap exception (D-354): these
stories predate per-story adversary convergence, so Gate 3 held blocking authority
over within-story CRITICAL/HIGH.

| ID | Sev | Finding | Disposition |
|----|-----|---------|-------------|
| WAVE2-001 | HIGH | `parse_ts` unchecked `secs * 1e9` → panic (dev, aborts run) / silent wrap (release). Violates BC-1.02.002 / DI-013 | **FIXED** — checked_mul/checked_add → E-FLW-001; commit de2b4c9; regression test `test_BC_1_02_002_overflowing_timestamp_skipped_not_panic` |
| WAVE2-002 | HIGH | Non-UTF-8 byte → `read_line` Err treated as EOF → silently drops rest of file. Violates BC-1.02.002 / EC-003 / accounting identity | **FIXED** — `read_until` + `from_utf8_lossy`; commit de2b4c9; regression test `test_BC_1_02_002_non_utf8_line_skipped_rest_parsed` |
| WAVE2-003 | MED | Fuzz VPs (VP-1.02.001-c / 002-a) only had a `".*"` proptest (UTF-8 strings, no newlines → never reached parser). Root cause of 001/002 surviving | **PARTIAL** — added byte-domain panic-free proptest `test_parser_panic_free_on_arbitrary_bytes` (header + arbitrary bytes). Real `cargo-fuzz` target remains deferred to **Phase 6** (formal hardening) per S-2.01 task #11 |
| WAVE2-004 | LOW | `parse_ts` mis-handled signed/garbage fractional part (e.g. `1.-5`) | **FIXED** alongside 001 (rejects non-digit fraction → malformed) |
| WAVE2-005 | LOW | `sl_t: {}` (empty mapping) → `SlTarget{None,None}`, contradicting the types.rs "at least one field set" doc note | **BACKLOG → S-1.03** — policy semantic validation owns SlTarget completeness; tracked there |
| WAVE2-006 | LOW/obs | `classify_yaml_error` distinguishes E-POL-002/003/004 by substring match on serde_norway messages — brittle to dependency wording changes | **BACKLOG** — pin/pinning note for `serde_norway`; revisit in Phase 6 hardening |

**Verified clean (no defects):** severity/OQ-001 13-state table + DI-017 single-source
reuse by the parser; IPv4-mapped canonicalization + unspecified gating ordering;
flow_index density; PortSet canonical form.

## Verdict

**WAVE GATE: ✅ PASSED** — 0 CRITICAL; both HIGH blockers fixed and regression-tested;
LOW items backlogged (WAVE2-005 → S-1.03, WAVE2-006 → Phase 6). Demo/holdout gates
legitimately deferred to Wave 5 (no executable surface yet). Wave 3 unblocked.
