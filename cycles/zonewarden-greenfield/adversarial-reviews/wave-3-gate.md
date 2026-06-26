---
document_type: wave-gate-report
level: ops
wave: wave-3
status: passed
producer: wave-gate
timestamp: 2026-06-26T00:00:00
cycle: zonewarden-greenfield
stories: [S-1.03, S-2.02, S-5.01]
---

# Wave 3 Integration Gate — zonewarden

Stories: **S-1.03** (policy semantic validation), **S-2.02** (service inference +
ingest cap), **S-5.01** (policy digest). Delivery: inline TDD on `dev`. Gate run
2026-06-26.

## Gate Results

| Gate | Name | Status | Note |
|------|------|--------|------|
| 1 | Test suite (release) | ✅ PASS | 89 tests, 0 failures; clippy `-D warnings` clean; `cargo +nightly fmt --check` clean |
| 2 | DTU validation | ⏭️ SKIP | `dtu_required: false`; no DTU-covered modules in wave |
| — | Mutation testing | ⏭️ SKIP | All 3 stories `tdd_mode: strict` — no facade/Option-B obligation |
| 3 | Adversarial review of wave diff | ✅ PASS | 0 CRITICAL, 0 HIGH, 2 MEDIUM, 3 LOW (dispositioned below) |
| 4 | Demo evidence | ⏭️ SKIP | No runnable surface yet — CLI is S-6.03 (Wave 5); behavior evidenced by the test suite |
| 5 | Holdout evaluation | ⏭️ SKIP | End-to-end conformance scenarios need the full pipeline + CLI (Wave 5) |
| 6 | State update | ✅ PASS | sprint-state + STATE.md updated; this report committed |

```
GATE_CHECK: gate=1 name=test-suite status=pass note=89 tests, 0 failures, clippy+fmt clean
GATE_CHECK: gate=2 name=dtu-validation status=skip note=dtu_required=false, no DTU modules in wave
GATE_CHECK: gate=3 name=adversarial-review status=pass note=0 critical, 0 high, 2 med + 3 low backlogged
GATE_CHECK: gate=4 name=demo-evidence status=skip note=no runnable surface until CLI (S-6.03, wave 5)
GATE_CHECK: gate=5 name=holdout-eval status=skip note=end-to-end scenarios need full pipeline (wave 5)
GATE_CHECK: gate=6 name=state-update status=pass note=sprint-state + STATE updated
```

## Gate 3 Findings & Dispositions

Reviewer: fresh-context adversary (read-only). No CRITICAL/HIGH → no blockers.

| ID | Sev | Finding | Disposition |
|----|-----|---------|-------------|
| WAVE3-001 | MED | Validator discards `warnings` on any `Err` path (BC-1.01.004 inv 4: warnings should emit even with errors). Structurally unimplementable with `Result<ValidatedPolicy, PolicyError>` signature | **BACKLOG → PO adjudication.** Likely a spec defect (inv 4 vs pure-Result shape). Either scope inv 4 to the success path, or change `validate` to return warnings alongside errors. Low impact (lost informational warning on an exit-2 run) |
| WAVE3-002 | MED | IPv4-mapped IPv6 zone members not canonicalized → BC-1.01.005 EC-005 tie undetected; mapped member is also a dead matcher (flow path canonicalizes, policy path doesn't) | **BACKLOG → fold into S-3.01 (zone resolver)** address-handling, where matching canonicalization lives. Apply `to_ipv4_mapped` + prefix-len adjustment (`/120`→`/24`) to members; add the EC-005 vector. Low real-world likelihood (`::ffff:` CIDRs in an OT policy) |
| WAVE3-003 | LOW | Digest hashes raw member string (host bits intact; `Ip(x)` ≠ `Cidr{x,/32}`) | **WORKING AS SPECIFIED** — BC-1.05.003 guarantees *model* equivalence; these are distinct model values. Optional nicety for "edit-and-rerun"; noted, not fixed |
| WAVE3-004 | LOW | `SysError::TallyOverflow` (E-SYS-003) defined but unreached here; `next_index += 1` unguarded | **BACKLOG → S-5.02.** E-SYS-003 is the aggregator overflow backstop (BC-1.05.004); the cap aborts long before `u64::MAX`. No reachable bug |
| WAVE3-005 | LOW | `DuplicateZoneId` names id only, not "both occurrences" (BC-1.01.004 postcond 2) | **BACKLOG → spec wording.** Unimplementable without source spans in the pure model; canonical test vector satisfied. Amend postcondition or carry spans in the loader |

**Verified clean (no defects):** service table exactly matches BC-1.02.004 **v1.1 / D-010**
(DNP3 & EtherNet/IP TCP+UDP; IT services; transport-mismatch/non-default/portless → Unknown;
`service_source` always set; no `DpiConfirmed`); ingest cap strict-`>` semantics + clean abort +
skips-don't-consume-quota; IngestError refactor preserves S-2.01 skip-inline behavior; IPv4 tie
detection (canonical-net equality, cross-zone-only, family isolation); error taxonomy + exit codes.

## Verdict

**WAVE GATE: ✅ PASSED** — 0 CRITICAL, 0 HIGH. 2 MEDIUM + 3 LOW dispositioned to
backlog (no merge blockers). Demo/holdout deferred to Wave 5 (no executable yet).
Wave 4 unblocked.
