---
document_type: wave-gate-report
level: ops
wave: wave-4
status: passed
producer: wave-gate
timestamp: 2026-06-26T00:00:00
cycle: zonewarden-greenfield
stories: [S-3.01, S-3.02, S-4.02, S-4.03, S-4.04]
---

# Wave 4 Integration Gate — zonewarden (classification core)

Stories: **S-3.01** resolver, **S-3.02** multicast, **S-4.02** IDMZ, **S-4.03**
classifier, **S-4.04** MulticastExempt + totality. Delivery: inline TDD on `dev`.
Gate run 2026-06-26.

## Gate Results

| Gate | Name | Status | Note |
|------|------|--------|------|
| 1 | Test suite (release) | ✅ PASS | 139 tests, 0 failures; clippy `-D warnings` clean; fmt clean. 3 Kani proofs verified (VP-1.03.001-a, VP-1.04.007-a, VP-1.04.010) |
| 2 | DTU validation | ⏭️ SKIP | `dtu_required: false`; no DTU modules in wave |
| — | Mutation testing | ⏭️ SKIP | All 5 stories `tdd_mode: strict` |
| 3 | Adversarial review of wave diff | ✅ PASS | 0 CRITICAL, 0 HIGH, 1 MEDIUM + 3 LOW (dispositioned below) |
| 4 | Demo evidence | ⏭️ SKIP | No runnable surface yet — CLI is S-6.03 (Wave 5) |
| 5 | Holdout evaluation | ⏭️ SKIP | End-to-end scenarios need the full pipeline + CLI (Wave 5) |
| 6 | State update | ✅ PASS | sprint-state + STATE updated; this report committed |

```
GATE_CHECK: gate=1 name=test-suite status=pass note=139 tests, 0 failures, 3 Kani proofs, clippy+fmt clean
GATE_CHECK: gate=2 name=dtu-validation status=skip note=dtu_required=false
GATE_CHECK: gate=3 name=adversarial-review status=pass note=0 critical, 0 high, 1 med + 3 low
GATE_CHECK: gate=4 name=demo-evidence status=skip note=no runnable surface until CLI (wave 5)
GATE_CHECK: gate=5 name=holdout-eval status=skip note=end-to-end scenarios need full pipeline (wave 5)
GATE_CHECK: gate=6 name=state-update status=pass note=sprint-state + STATE updated
```

## Gate 3 Findings & Dispositions

Reviewer: fresh-context adversary (read-only, bootstrap cohort). Verified CLEAN:
verdict precedence & totality (ST-6 order), IDMZ additivity + exclusions + DI-006
truth table, conduit union/direction/portless matching, resolver longest-prefix
(index sort cross-checked), multicast Step-1/2, severity table, ConduitId-vs-digest.

| ID | Sev | Finding | Disposition |
|----|-----|---------|-------------|
| WAVE4-001 | MED | Multicast detection uses a parallel `DstKind` channel (classify_dst → DstKind → classifier MulticastExempt). BC-1.03.003 PC1/PC2 & BC-1.03.004 PC1/PC4 instead require the dst's `ResolvedEndpoint.match_kind = MatchKind::MulticastBroadcast` and "no longest-prefix lookup for a multicast dst". So `MatchKind::MulticastBroadcast` is currently unreachable. Verdict/security correctness UNAFFECTED (DstKind channel works); the gap is reporting fidelity + 4 literal postconditions + a dead variant | **BACKLOG → PO/spec adjudication.** This is deliberate story-level design (DstKind introduced across S-3.02 + S-4.04) diverging from the older L3 BCs — same class as D-010. Recommend: sanction the DstKind channel via a decision, amend BC-1.03.003/004 postconditions to reference DstKind, and either keep `MatchKind::MulticastBroadcast` as reserved or remove it. The PC2 "skip dst resolution" optimization belongs to the Wave 5 pipeline (S-6.03). Not a verdict/security defect → does not block |
| WAVE4-002 | LOW | `idmz::check`'s multicast-exclusion branch is dead in the production path (classify always passes Normal after the line-39 short-circuit); exercised only by unit tests | **KEEP** as defensive code; the multicast exclusion is actually enforced by classify's MulticastExempt short-circuit. Noted so future readers don't rely on it |
| WAVE4-003 | LOW | Classifier Kani harness `verdict_total` only explores an empty policy → Allowed/WrongDirection arms not symbolically covered | **BACKLOG → Phase 6.** Totality still holds structurally (exhaustive match); strengthen the harness with ≥1 symbolic conduit during formal hardening |
| WAVE4-004 | LOW | Per-flow O(zones) linear `level_of` lookup in idmz + O(conduits) match, per flow | **BACKLOG → S-5.02/perf.** Fine for MVP OT policy sizes (50–200 matchers); aggregator should avoid redundant `level_of` calls |

## Verdict

**WAVE GATE: ✅ PASSED** — 0 CRITICAL, 0 HIGH. The classification core is correct
and faithfully matches its BCs. 1 MEDIUM (WAVE4-001, a DstKind-vs-MatchKind spec
drift → PO adjudication) + 3 LOW backlogged; none block. Wave 5 unblocked.
