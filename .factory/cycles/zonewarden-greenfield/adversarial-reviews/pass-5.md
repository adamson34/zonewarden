---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-06-17T13:30:00
phase: 1d
inputs: ["product-brief.md", "domain-spec/ (v1.4)"]
input-hash: "n/a"
traces_to: domain-spec/L2-INDEX.md
pass: 5
previous_review: pass-4.md
scope: "specs --scope=full (product-brief.md + domain-spec/ v1.4)"
---

# Adversarial Review: ZoneWarden (Pass 5)

**Scope:** specs --scope=full (product-brief.md + domain-spec/ v1.4). Comprehensive.
**Convergence rule (D-008, human-decided):** converge on 2 consecutive passes with 0 CRIT and 0 HIGH; MED/LOW tracked as backlog, not blockers.
**Key convergence question this pass:** Are there ANY CRIT or HIGH findings? **YES — 1 CRITICAL, 1 HIGH. NOT CONVERGED.**

## Part A — Fix Verification

| ID | Status | Notes |
|----|--------|-------|
| 1. DI-018 enum sort tokens + byte-wise UTF-8 + Zone.name/sl_t digested | RESOLVED | invariants.md:39 — all four sub-claims present (Proto/Direction/PortSet tokens, byte-wise UTF-8 collation, Zone.name & sl_t digested). |
| 2. DI-016 directed broadcast resolve-then-override; family-wide needs no zone | RESOLVED | invariants.md:37 — (1) family-wide test needs no zone; (2) otherwise resolve longest-prefix zone then override Explicit→MulticastBroadcast on all-ones host. |
| 3. DI count is 20, L2-INDEX authoritative | RESOLVED | L2-INDEX.md:69 "DI-NNN | 20"; line 74 authoritative. invariants.md enumerated = 20 rows. |
| 4. All tallies and flow_index use u64 | RESOLVED | entities.md:30 — tallies and flow_index are u64 (canonical, not usize); ConformanceResult fields u64; Flow.flow_index: u64. |
| 5. DI-010 tie-break citation | RESOLVED | invariants.md:31 — tie defined (same family, equal prefix length); disjoint same-length CIDRs not a tie (DEC-022). |
| 6. FM-007 re-anchor + FM-008 overflow | RESOLVED (in failure-modes.md) | FM-007 = OOM/streaming (failure-modes.md:29); FM-008 = overflow (failure-modes.md:30). Correctly anchored in failure-modes.md. BUT entities.md:30 still cites wrong ID (FM-007) for overflow — see CRIT-001. |
| 7. Overflow → abort (FM-008 checked arithmetic) | PARTIALLY_RESOLVED | failure-modes.md:30 says "abort" but mechanism stated as "Checked/saturating arithmetic" — self-contradictory; saturating clamps, does not abort. See HIGH-001. |
| 8. DEC-012 IPv4-mapped → IPv4 | RESOLVED | edge-cases.md:30 — canonicalized to IPv4 before resolution. |
| 9. DI-020 PortSet canonical form | RESOLVED | invariants.md:41 — sorted non-overlapping non-adjacent [lo,hi]; adjacents coalesce; Any distinct sentinel never folded with 0-65535. |
| 10. DEC-028 duplicate YAML keys → error | RESOLVED | edge-cases.md:46 — load-time error (no silent serde last-wins). |
| 11. DEC-025 reordered | RESOLVED | edge-cases.md:43 DEC-025 = multicast/broadcast destination; DEC contiguous; sits in multicast cluster. |
| 12. RSTO fix | RESOLVED | entities.md:41 — RSTO/RSTR bucketed under Established; fixes previously-wrong bucket. |
| 13. DEC-029 /0 member → error | RESOLVED | edge-cases.md:47 — 0.0.0.0/0 or ::/0 member = load-time error (DI-010). |
| 14. CAP-007 citations | RESOLVED | capabilities.md:28 — cites DI-015, DI-002, DI-001, DI-016, DI-017. |
| 15. ID counts DI=20, DEC=29 (contiguous), FM=8, ASM=9, CAP=14, ST=8, R=6 | RESOLVED | Enumerated: DI-001..020 (20), DEC-001..029 (29, contiguous), FM-001..008 (8), ASM-001..009 (9), CAP-001..014 (14), ST-1..8 (8), R-001..006 (6). All counts correct and contiguous. (FM mis-citation tracked separately as CRIT-001, not a count error.) |

Part A totals: 13 RESOLVED, 2 PARTIALLY_RESOLVED (#7, #6's propagation defect), 0 UNRESOLVED.

## Part B — New Findings

### CRITICAL

#### ADV-ZWGREEN-P05-CRIT-001: ConformanceResult cites FM-007 (OOM) for overflow behavior that lives in FM-008
- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** entities.md:30 (ConformanceResult row)
- **Description:** ConformanceResult states overflow is "handled per FM-007", but FM-007 is the OOM / large-input streaming failure mode. The overflow failure mode is FM-008. An implementer following this anchor would look up FM-007 (streaming via ST-3 iterator) and find nothing about checked-arithmetic overflow abort — the exact behavior the sentence intends. A cross-reference pointing an implementer at the wrong failure mode for the described behavior is a mis-anchor and blocks convergence.
- **Evidence:** entities.md:30: "overflow handled per FM-007 (checked arithmetic → abort, not silent wrap)"; failure-modes.md:29: "FM-007 ... Very large flow input exhausts memory ... Stream flows via the ST-3 iterator"; failure-modes.md:30: "FM-008 ... u64 tally counter overflow ... on overflow abort".
- **Proposed Fix:** In entities.md:30 change "overflow handled per FM-007" → "overflow handled per FM-008".

### HIGH

#### ADV-ZWGREEN-P05-HIGH-001: FM-008 recovery contract says "checked/saturating" — saturating contradicts abort
- **Severity:** HIGH
- **Category:** contradictions
- **Location:** failure-modes.md:30 (FM-008 Recovery column)
- **Description:** FM-008 specifies recovery as "Checked/saturating arithmetic; on overflow abort". Checked and saturating are mutually exclusive: checked returns None/errs on overflow (enabling abort); saturating clamps to u64::MAX and never signals overflow — it cannot trigger abort and would silently produce the wrong-but-plausible counts this FM exists to prevent. An implementer choosing the saturating arm satisfies the literal text while violating intent. Internal contradiction in a determinism/accounting-critical counter contract (DI-015 accounting identity depends on exact tallies).
- **Evidence:** failure-modes.md:30: "Checked/saturating arithmetic; on overflow abort with a distinct exit code + diagnostic (never silent wrap)".
- **Proposed Fix:** Drop "saturating". State "Checked arithmetic (checked_add); on overflow abort with a distinct exit code + diagnostic (never silent wrap or saturate)."

### MEDIUM

#### ADV-ZWGREEN-P05-MED-001: DI-016 step-2 directed-broadcast override undefined/dangerous for IPv6
- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** invariants.md:37 (DI-016 step 2)
- **Description:** Step-2 directed-broadcast override compares dst to "the resolved zone's all-ones host address". IPv6 has no broadcast, so "all-ones host" is meaningless for an IPv6 zone, yet step 2 has no family qualifier. An implementer could classify a legitimate IPv6 host (...:ffff:ffff:ffff:ffff) as MulticastBroadcast → MulticastExempt, silently suppressing a real violation (deny-by-default safety regression).
- **Evidence:** invariants.md:37 step 2 has no family qualifier, unlike step 1 which is family-tagged.
- **Proposed Fix:** Add: "Directed-broadcast override applies to IPv4 zones only; an IPv6 dst is never reclassified by step 2."

#### ADV-ZWGREEN-P05-MED-002: Verification-grade differentiator cites incomplete provable-invariant set
- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** differentiators.md:27
- **Description:** The Kani-proven-core differentiator cites CAP-006, CAP-007, DI-003, DI-004, DI-009 — but omits the headline ⊢-flagged invariants the claim actually rests on (DI-015 verdict-totality accounting, DI-016 multicast precedence). Mapping is incomplete/stale relative to the ⊢ set in invariants.md.
- **Evidence:** differentiators.md:27 supporting set vs invariants.md ⊢ flags (DI-003/004/006/009/010/012/015/016/018/020).
- **Proposed Fix:** Expand supporting set to include core provable invariants (at minimum DI-015, DI-016) or reword to "the ⊢-flagged invariant set (see invariants.md)".

#### ADV-ZWGREEN-P05-MED-003: Direction legal-token set split across documents (alias vs legality check)
- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** entities.md:37 / ubiquitous-language.md:50 vs invariants.md:31 (DI-010)
- **Description:** "unidirectional" is accepted as an alias for "forward" (entities.md:37), but DI-010's token-legality rule has no co-located enumeration of legal tokens. An implementer reading DI-010 alone could reject "unidirectional" as unrecognized, contradicting entities.md. No single source of truth for the legal-token set.
- **Evidence:** entities.md:37 alias note; invariants.md:31 "unrecognized direction/proto → policy error" with no enumerated list.
- **Proposed Fix:** Add the canonical legal-token enumeration (incl. unidirectional→forward alias) to DI-010, or have DI-010 explicitly reference entities.md:37.

#### ADV-ZWGREEN-P05-MED-004: DI-006 "managed L5" vs "EXTERNAL L5" not mechanically decidable from data model
- **Severity:** MEDIUM
- **Category:** contradictions
- **Location:** invariants.md:27, 50-52 (DI-006 truth table) vs entities.md:27
- **Description:** DI-006 distinguishes "managed L5" (idmz_bypass eligible) from "EXTERNAL L5 sentinel" (excluded), but both are L5 by purdue_level alone. DI-006 prose keys off "managed" + "level ≥L4" rather than MatchKind, so rows 51/52 are non-decidable from the data model unless the discriminator is the MatchKind (Explicit vs ImplicitExternal).
- **Evidence:** invariants.md:50-52 truth table rows; entities.md:27 ResolvedEndpoint MatchKind ∈ {Explicit{prefix_len}, ImplicitExternal, MulticastBroadcast}.
- **Proposed Fix:** State in DI-006 that "managed" = MatchKind::Explicit and EXTERNAL = MatchKind::ImplicitExternal, making the discriminator the MatchKind, not the level.

### LOW

#### ADV-ZWGREEN-P05-LOW-001: Version frontmatter skew — content edited for v1.4 but version stamp still 1.0
- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** failure-modes.md:6, edge-cases.md:6 (and other section files)
- **Description:** L2-INDEX, invariants, entities are version 1.4; all other sections remain 1.0. But edge-cases.md (DEC-028/029) and failure-modes.md (FM-008) received v1.4 content changes while keeping version 1.0 frontmatter.
- **Evidence:** failure-modes.md:6 version "1.0" despite FM-008 v1.4 fix; edge-cases.md:6 version "1.0" despite DEC-028/029 v1.4 additions; L2-INDEX.md:4 version "1.4".
- **Proposed Fix:** Bump version frontmatter of changed section files (at minimum edge-cases.md, failure-modes.md) to 1.4, or document the per-section versioning convention if intentional.

#### ADV-ZWGREEN-P05-LOW-002: FM-008 ingest cap relationship to u64 overflow unstated
- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** failure-modes.md:30 (FM-008)
- **Description:** FM-008 mandates a "documented max-flow-count cap at ingest" but no cap value is given, and the relationship between the cap and u64 overflow is unstated. If the cap < u64::MAX the overflow-abort path is unreachable (dead contract). Ambiguous whether the cap is for memory (FM-007) or overflow (FM-008).
- **Evidence:** failure-modes.md:30 "enforce a documented max-flow-count cap at ingest"; entities.md:30 tallies u64; no cap value in any section.
- **Proposed Fix:** Clarify whether the ingest cap is the primary overflow guard (abort = defense-in-depth backstop) or vice versa; note cap value is PRD-pinned; distinguish from FM-007's memory bound.

#### ADV-ZWGREEN-P05-LOW-003: No DEC edge case exercises DI-016 directed-broadcast override
- **Severity:** LOW
- **Category:** coverage-gap
- **Location:** edge-cases.md (DEC set) vs invariants.md:37 (DI-016 step 2)
- **Description:** DI-016 step-2 directed-broadcast override (dst == resolved zone's all-ones host) is the most error-prone branch but has no DEC edge case. DEC-023 covers a different scenario (host /32 vs another zone's broadcast). No test-anchor for the override path.
- **Evidence:** invariants.md:37 step 2; edge-cases.md DEC-023 covers host-vs-other-zone broadcast, not "dst == own resolved zone's directed broadcast → MulticastExempt".
- **Proposed Fix:** Add DEC-030 "Destination equals its own resolved zone's directed-broadcast (all-ones host)" → MatchKind overridden to MulticastBroadcast, verdict MulticastExempt, idmz_bypass=false (DI-016 step 2).

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 1 |
| HIGH | 1 |
| MEDIUM | 4 |
| LOW | 3 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate (1 CRIT + 1 HIGH; D-008 rule requires 0 CRIT and 0 HIGH)
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 9 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | MEDIUM |
| **Trajectory** | 14→16→11→15→9 |
| **Verdict** | FINDINGS_REMAIN |

Note: CRIT-001 survived prior passes because Part A pass-4 verified FM-007/FM-008 in failure-modes.md but did not check propagation to the citing site in entities.md (sibling-file propagation gap). HIGH-001 undermines the intent of the pass-4 overflow→abort fix.
