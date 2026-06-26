---
document_type: pipeline-state
level: ops
version: "2.0"
status: in-progress
producer: state-manager
timestamp: 2026-06-26T00:00:00
phase: 3
inputs: []
input-hash: "[live-state]"
traces_to: ""
project: "zonewarden"
mode: "greenfield"
current_step: "phase-3 TDD — Wave 4 in progress (S-3.01/3.02/4.02 done); S-4.03 classifier core READY"
current_cycle: "zonewarden-greenfield"
dtu_required: false
---

<!--
  STATE.md SIZE BUDGET: Keep this file under 200 lines.
  Historical content belongs in cycle files, NOT here.
  Run /vsdd-factory:compact-state if this file grows past 200 lines.
-->

# Pipeline State: zonewarden

## Project Metadata

| Field | Value |
|-------|-------|
| **Product** | zonewarden — OT Segmentation-as-Code validator |
| **Repository** | /Users/lukeadamson/1898/otposture/zonewarden |
| **Mode** | greenfield |
| **Language** | Rust |
| **Target Workspace** | /Users/lukeadamson/1898/otposture/zonewarden |
| **Started** | 2026-06-17 |
| **Last Updated** | 2026-06-26 |
| **Current Phase** | 3 |
| **Current Step** | Phase 3 TDD — Wave 4 in progress (10/17): S-3.01/3.02/4.02 done; S-4.03 classifier READY |

## Phase Progress

| Phase | Status | Started | Completed | Gate | Finding Progression |
|-------|--------|---------|-----------|------|---------------------|
| 0: Codebase Ingestion | n/a (greenfield) | | | | |
| 1: Spec Crystallization | COMPLETE | 2026-06-17 | 2026-06-17 | passed | 8 adversarial passes: 14→16→11→15→9→5→9→15; ~93 findings fixed; 0 CRIT ×6, all HIGH-to-date fixed. L2 FROZEN at v1.8 (D-009). Loop not converging to 0-HIGH (novelty 1.0) — accepted sound + proceeded to PRD. Pass-8 MED/LOW = backlog. |
| 2: Story Decomposition | COMPLETE | 2026-06-17 | 2026-06-17 | passed | 6 epics, 17 stories, 5 waves, 10 holdout scenarios, 44/44 BC coverage, acyclic |
| 3: TDD Implementation | in-progress | 2026-06-17 | | | Waves 1-3 COMPLETE + gated; Wave 4 in progress (3/5): + S-3.01, S-3.02, S-4.02 DONE. 10/17 stories; 119 tests green (release) incl proptests + Kani VP-1.03.001-a & VP-1.04.007-a verified; clippy -D + fmt clean. S-4.02 = IDMZ truth table (DI-006, b2f81b4). Wave 4 remaining: S-4.03 classifier core (8pts, now READY), S-4.04. |
| 4: Holdout Evaluation | not-started | | | | |
| 5: Adversarial Refinement | not-started | | | | |
| 6: Formal Hardening | not-started | | | | |
| 7: Convergence | not-started | | | | |

## Current Phase Steps

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| Brainstorming | brainstorming | completed | planning/brainstorming-report.md |
| Domain research | research-agent | completed | specs/research/domain-ot-segmentation-validation-2026-06-17.md |
| Product brief | create-brief | completed | specs/product-brief.md |
| Domain spec (L2, sharded) | create-domain-spec | completed | specs/domain-spec/ (11 files) |
| Adversarial spec review (Pass 1) | adversary | completed | cycles/zonewarden-greenfield/adversarial-reviews/pass-1.md |
| Domain spec revision (Pass-1 fixes) | business-analyst | completed | specs/domain-spec/ v1.1 (all 14 findings) |
| Adversarial spec review (Pass 2) | adversary | completed | cycles/zonewarden-greenfield/adversarial-reviews/pass-2.md (16 findings; 1C/3H/6M/6L) |
| Domain spec revision (Pass-2 fixes) | business-analyst | completed | specs/domain-spec/ v1.2 (all 16 findings; verdict decoupled, conn_state grading, multicast-exempt, ICMP/portless) |
| Adversarial spec review (Pass 3) | adversary | completed | cycles/zonewarden-greenfield/adversarial-reviews/pass-3.md (11 findings; 0C/2H/5M/4L) |
| Domain spec revision (Pass-3 fixes) | business-analyst | completed | specs/domain-spec/ v1.3 (all 11; digest canonicalization, external_endpoints, multicast precedence, etc.) |
| Adversarial spec review (Pass 4) | adversary | completed | cycles/zonewarden-greenfield/adversarial-reviews/pass-4.md (15 findings; 0C/3H/7M/5L) |
| Domain spec revision (Pass-4 fixes) | business-analyst | completed | specs/domain-spec/ v1.4 (all 15: digest sort-key, broadcast/zone-resolution ordering, DI-020 PortSet, u64 tallies, IPv4-mapped, dup-keys, /0 reject, RSTO) |
| Adversarial spec review (Pass 5) | adversary | completed | cycles/zonewarden-greenfield/adversarial-reviews/pass-5.md (9 findings; 1C/1H/4M/3L) |
| Domain spec revision (Pass-5 fixes) | business-analyst | completed | specs/domain-spec/ v1.5 (all 9; FM citation, checked-not-saturating, IPv4-only directed bcast, token enum, versions synced) |
| Adversarial spec review (Pass 6) | adversary | completed | cycles/zonewarden-greenfield/adversarial-reviews/pass-6.md (5 findings; 0C/1H/1M/3L) |
| Domain spec revision (Pass-6 fixes) | business-analyst | completed | specs/domain-spec/ v1.6 (all 5; directed-bcast /31-/32 exclusion, ingest cap anchored, CAP cites) |
| Adversarial spec review (Pass 7) | adversary | completed | pass-7.md (9; 0C/2H/4M/3L) |
| Domain spec revision (Pass-7 fixes) | business-analyst | completed | specs/domain-spec/ v1.7 (usize→u64, DI-017 single-source, FM split, + cross-doc sweep) |
| Adversarial spec review (Pass 8) | adversary | completed | pass-8.md (15; 0C/4H/7M/4L) |
| Domain spec revision (Pass-8 fixes) | business-analyst | completed | specs/domain-spec/ v1.8 (all 4 HIGH: EXTERNAL single-zone, both-EXTERNAL=IntraZone, unspecified-addr skip+warn) |
| L2 domain spec — ACCEPTED/FROZEN at v1.8 | human | completed | D-009: sound (0 CRIT ×6, all HIGH-to-date fixed); proceed to PRD |
| PRD + behavioral contracts | product-owner | completed | specs/prd.md + 44 BCs (ss-01..06) + 4 supplements; D-009 backlog in PRD §9 |
| Architecture (L3/L4) | architect | completed | specs/architecture/ (4 sections + index + 7 ADRs) + 10 VPs; pure core/effectful shell at crate boundary |
| Story decomposition (Phase 2) | story-writer | completed | stories/ (17 stories, 6 epics, 5 waves) + 10 holdout scenarios; 44/44 BC coverage |
| TDD: S-1.01 workspace + PortSet | (inline TDD) | completed | zonewarden-core + zonewarden crates; 10 tests; commit f380859 |
| TDD: S-1.02 policy YAML load | (inline TDD) | completed | zonewarden lib+bin, policy::load; 10 tests; commit 6f3f01b |
| TDD: S-4.01 severity grading | (inline TDD) | completed | zonewarden-core::severity (DI-017 single source; OQ-001 13-state); 5 tests; commit 4e95c29 |
| TDD: S-2.01 Zeek parser | (inline TDD) | completed | zonewarden::adapters::zeek + RealitySource + FlowParseError; 19 tests + proptests; commit f9a7ee4 |
| Wave 2 integration gate | wave-gate | completed | PASSED 2026-06-26; 2 HIGH fixed (de2b4c9); report: cycles/zonewarden-greenfield/adversarial-reviews/wave-2-gate.md |
| TDD: S-1.03 policy validation | (inline TDD) | completed | zonewarden-core::validator (E-POL-005..008, ipnet prefix index); 17 tests; commit 9d8575d |
| TDD: S-2.02 service inference + cap | (inline TDD) | completed | infer_service (D-010), ingest cap (E-SYS-001/002), IngestError refactor; 14 tests; commit 2327a66 |
| TDD: S-5.01 policy digest | (inline TDD) | completed | zonewarden-core::digest (canonical JSON + SHA-256, ADR-004); 9 tests; commit 5fdd5fa |
| Wave 3 integration gate | wave-gate | completed | PASSED 2026-06-26; 0 CRIT/HIGH, 2 MED + 3 LOW backlogged; report: cycles/zonewarden-greenfield/adversarial-reviews/wave-3-gate.md |
| TDD: S-3.01 zone resolver | (inline TDD) | completed | resolver::resolve/resolve_pair (longest-prefix + EXTERNAL); Kani VP-1.03.001-a verified; WAVE3-002 fixed; 11 tests; commit 351a2e7 |
| TDD: S-3.02 multicast/bcast detection | (inline TDD) | completed | multicast::classify_dst (DI-016 Step-1/2); DstKind; 12 tests; commit b2440fd |
| TDD: S-4.02 IDMZ truth table | (inline TDD) | completed | idmz::check (DI-006 OT<->IT; additive; EXTERNAL/multicast exclusions); Kani VP-1.04.007-a verified; 8 tests; commit b2f81b4 |
| TDD: Wave 4 remaining | — | next | S-4.03 classifier core (READY — the verdict engine, 8pts); then S-4.04 multicast-exempt + totality |

## Decisions Log

| ID | Decision | Rationale | Phase | Date | Made By |
|----|----------|-----------|-------|------|---------|
| D-001 | Build zonewarden: OT Segmentation-as-Code validator | Confirmed market whitespace; leverages OT domain expertise; strong Rust+verification fit | 1 | 2026-06-17 | human |
| D-002 | Language = Rust | Showcase + native fit with factory verification (Kani, cargo-fuzz, cargo-mutants) | 1 | 2026-06-17 | human |
| D-003 | Pluggable RealitySource adapters; flows v1 (Zeek conn.log), firewall config v2 | Extensible design; flow-based = automated 62443 FR5 conformance | 1 | 2026-06-17 | human |
| D-004 | Policy format YAML-first; custom DSL phase-2 stretch | Get engine working fast; DSL becomes the Kani/fuzz parser showcase later | 1 | 2026-06-17 | human |
| D-005 | Diagram = Mermaid for MVP, native SVG later | Instant visual, renders in GitHub; SVG is polish | 1 | 2026-06-17 | human |
| D-006 | MVP bar = full core loop (validate + report + diagram) | Smallest resume-worthy demo of the value prop | 1 | 2026-06-17 | human |
| D-007 | Time budget = open-ended / learning-first | Optimize for full factory rigor + learning over speed | 1 | 2026-06-17 | human |
| D-008 | Spec convergence rule = 2 consecutive passes with 0 CRIT + 0 HIGH (not 3 fully-clean passes); MED/LOW residuals tracked as backlog | Adversary novelty stayed 1.0 with oscillating count (14→16→11→15); pure clean-pass goal unlikely to terminate; spec is sound (0 CRIT). Pragmatic, defensible stopping rule | 1 | 2026-06-17 | human |
| D-009 | Freeze L2 domain spec at v1.8 and proceed to PRD without meeting D-008 | After 8 passes the loop did not converge to 0-HIGH (oscillating 5→9→15, novelty 1.0); spec is sound (0 CRIT ×6, all HIGH-to-date fixed); remaining edge semantics are better pinned as PRD behavioral contracts + TDD tests; diminishing returns for the portfolio goal. Pass-8 MED/LOW carried as backlog | 1 | 2026-06-17 | human |
| D-010 | Service table: DNP3 & EtherNet/IP match TCP+UDP (not TCP-only); add IT services HTTP/HTTPS/DNS/NTP | Real-world DNP3/EtherNet/IP also run over UDP; labeling common IT traffic improves the report. Supersedes BC-1.02.004 v1.0 EC-004 (TCP-only / ASM-009). BC-1.02.004 bumped to v1.1 to keep spec↔code in sync (no drift) | 3 | 2026-06-26 | human |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| Codebase ingestion (Phase 0) | yes | Greenfield — no existing codebase |
| UX Spec | likely | CLI-first product, no GUI surfaces in MVP (confirm at PRD) |

## Blocking Issues

| ID | Issue | Severity | Blocking Phase | Owner | Resolution |
|----|-------|----------|---------------|-------|------------|
| BI-001 | Pass-1 adversarial spec review found 5 blocking foundation defects (SL-T contradiction; AmbiguousMembership; EXTERNAL level; IDMZ single-flow semantics; ingest resilience). | CRITICAL | PRD authoring | business-analyst / product-owner | **RESOLVED** — all fixed by v1.1 and verified RESOLVED in Pass-2 Part A. Superseded by the ongoing convergence loop (D-008 rule) |

## Session Resume Checkpoint

| Field | Value |
|-------|-------|
| **Date** | 2026-06-26 |
| **Position** | Phase 3 (TDD). Waves 1-2 COMPLETE on dev (4/17): S-1.01 workspace+PortSet; S-1.02 policy YAML load; S-4.01 severity (DI-017 single source, commit 4e95c29); S-2.01 Zeek conn.log parser — zonewarden::adapters::zeek + RealitySource trait + FlowParseError, conn_state reuses severity (commit f9a7ee4). ~60 tests green incl proptests, clippy -D warnings clean, fmt clean. Repo: 3-branch model (main/dev/factory-artifacts worktree); code on dev. Wave 2 GATE PASSED 2026-06-26 (de2b4c9 fixed 2 HIGH). Wave 3 COMPLETE (3/3): S-1.03 validator (9d8575d); S-2.02 service inference + ingest cap (2327a66; D-010; BC-1.02.004 v1.1; IngestError refactor); S-5.01 policy digest (5fdd5fa; canonical JSON + SHA-256, serde_json+sha2 in core per ADR-004). Wave 3 GATE PASSED 2026-06-26. Wave 4 in progress (3/5): S-3.01 resolver (351a2e7; Kani VP-1.03.001-a), S-3.02 multicast (b2440fd), S-4.02 IDMZ truth table (b2f81b4; Kani VP-1.04.007-a) DONE. NEXT: S-4.03 classifier core (8pts, READY — the verdict engine: ties resolver+multicast+IDMZ+severity into per-flow Verdicts; BC-1.04.001-006,009); then S-4.04 multicast-exempt + verdict totality. After Wave 4 the classification core is complete; Wave 5 = aggregation + report + CLI. Wave 3 gate backlog: WAVE3-002 CLOSED in S-3.01 (IPv4-mapped member canonicalization). Still open: WAVE3-001 (validator warnings on Err path — PO adjudicate BC-1.01.004 inv 4); WAVE3-004 (E-SYS-003 -> S-5.02); WAVE3-005 (dup-id "both occurrences" -> spec wording). Deferred/backlog: cargo-fuzz target + WAVE2-006 (brittle YAML err classification) -> Phase 6; WAVE2-005 (sl_t empty-mapping `{}` -> SlTarget{None,None}) is a LOAD-layer fix in policy.rs SlTargetYaml::into_core — small follow-up, still OPEN; demo+holdout gates -> Wave 5 (need CLI); SysError::TallyOverflow (E-SYS-003) defined, consumed by S-5.02 aggregator. |
| **Convergence counter** | spec loop closed by D-009 (not via D-008 streak) |

## Historical Content

| Content | Location |
|---------|----------|
| Burst history | `cycles/<cycle>/burst-log.md` |
| Convergence trajectory | `cycles/<cycle>/convergence-trajectory.md` |
| Session checkpoints | `cycles/<cycle>/session-checkpoints.md` |
| Lessons learned | `cycles/<cycle>/lessons.md` |
| Resolved blockers | `cycles/<cycle>/blocking-issues-resolved.md` |
