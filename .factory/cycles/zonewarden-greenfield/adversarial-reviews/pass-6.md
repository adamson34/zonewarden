---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-06-17T13:45:00
phase: 1d
inputs: ["product-brief.md", "domain-spec/"]
input-hash: "[live-state]"
traces_to: domain-spec/L2-INDEX.md
pass: 6
previous_review: pass-5.md
scope: "specs --scope=full (product-brief.md + domain-spec/ v1.5)"
convergence_rule: "D-008 — converge on 2 consecutive passes with 0 CRIT and 0 HIGH; MED/LOW tracked as backlog"
---

# Adversarial Review: zonewarden (Pass 6)

> Scope: FULL — `product-brief.md` + all `domain-spec/` section files at version 1.5.
> Fresh-context pass. Adversary saw only the target artifacts plus the Part-A claimed fixes; no prior pass findings were leaked.

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` (cycle = `zonewarden-greenfield`).

## Part A — Fix Verification

| Fix | Status | Evidence |
|-----|--------|----------|
| **1. P05-CRIT-001** (entities overflow cites FM-008; FM-008 exists & is overflow FM) | **RESOLVED** | entities.md:30 — `"overflow handled per FM-008 (checked arithmetic → abort, not silent wrap)"`. failure-modes.md:30 — `FM-008 | ST-7 (aggregate) | u64 tally counter overflow`. No stray FM-007 overflow reference remains; FM-007 is correctly the OOM/streaming FM. |
| **2. P05-HIGH-001** (FM-008 checked/abort, no "saturating") | **RESOLVED** | failure-modes.md:30 — `"Checked arithmetic (checked_add); on overflow abort with a distinct exit code + diagnostic (never saturate, never silent wrap)"`. Only `saturat` token is inside prohibition `"never saturate"`. No "saturating arithmetic" strategy language. |
| **3. MED DI-016** (directed-broadcast IPv4-only, step-2) | **RESOLVED** | invariants.md:37 — `"Step 2 is IPv4-only — IPv6 has no broadcast, so a v6 dst is only exempt via the step-1 ff00::/8 test and never via this override."` Mirrored at DEC-030. |
| **4. MED DI-006** (managed vs EXTERNAL by zone identity, not purdue_level) | **RESOLVED** | invariants.md:27 — `"\"Managed\" vs EXTERNAL is decided by zone identity (the reserved EXTERNAL zone id), not by purdue_level — a declared zone may legitimately be L5 and is still \"managed\"."` Truth-table row reinforces. |
| **5. MED DI-010** (direction + proto token enums) | **RESOLVED** | invariants.md:31 — `"legal direction tokens are exactly {forward, bidirectional, unidirectional(=forward alias)}; legal proto tokens {tcp, udp, icmp, other:<u8>}; any unrecognized → policy error, no permissive defaulting"`. Both enumerations present and complete. |
| **6. MED differentiators citation** (Verification-grade cites proof-set DI-015/016/018/020) | **RESOLVED** | differentiators.md:27 — `"CAP-006, CAP-007, DI-003, DI-004, DI-009, DI-015, DI-016, DI-018, DI-020 (the ⊢ proof-target invariants)"`. All cited DIs exist and are ⊢-flagged. Superset of required {015,016,018,020}. |
| **7. LOW version sync** (all section files at 1.5) | **RESOLVED** | grep `^version:` across domain-spec → all 11 files (L2-INDEX + 10 sections) read `version: "1.5"`. |
| **8. LOW FM-008 ingest-cap relationship** | **PARTIALLY_RESOLVED** | failure-modes.md:30 references `"a documented max-flow-count ingest cap (set well below u64::MAX)"`. The reference is present and coherent, BUT the cap is defined/owned by no DI, no stage, and not by FM-007 (the ingest/OOM FM where it would logically be enforced). See ADV-P06-MED-001. |
| **9. LOW DEC-030** (directed-broadcast override, IPv4 only) | **RESOLVED** | edge-cases.md:48 — `"DEC-030 | CAP-007 | Dst IPv4 == the directed-broadcast (all-ones host) of its longest-prefix-matched zone (e.g. 10.0.0.255 for 10.0.0.0/24) | Step-2 override (DI-016) ... (IPv4 only; an IPv6 dst never takes this branch)"`. Coherent; matches DI-016 step-2. (Spec models DEC as domain edge cases, not a separate decision registry — wording in the request is loose, but the item exists correctly.) |

**Index claim verification (L2-INDEX.md) — independently counted via grep:**

| ID | Claimed | Actual | Contiguous? |
|----|---------|--------|-------------|
| CAP-NNN | 14 | 14 | 001–014 ✓ |
| DI-NNN | 20 | 20 | 001–020 ✓ |
| ST-N | 8 | 8 | 1–8 ✓ |
| DEC-NNN | 30 | 30 | 001–030 ✓ |
| ASM-NNN | 9 | 9 | 001–009 ✓ |
| R-NNN | 6 | 6 | 001–006 ✓ |
| FM-NNN | 8 | 8 | 001–008 ✓ |

All index counts accurate and ranges contiguous. No spec-fidelity count mismatches.

## Part B — New Findings

### HIGH

#### ADV-P06-HIGH-001: Directed-broadcast override silently exempts /31 and /32 unicast destinations (silent-allow vector)
- **Severity:** HIGH
- **Category:** missing-edge-cases / verification-gaps / security-surface (silent-allow)
- **Location:** invariants.md:37 (DI-016 step 2); edge-cases.md:48 (DEC-030); entities.md:23 (AssetMatcher /32 note), :27 (ResolvedEndpoint)
- **Description:** DI-016 step 2 says: after resolving `dst` to its longest-prefix zone, "if `dst` equals that zone's directed-broadcast (all-ones host) address, override … → `MulticastBroadcast` → `MulticastExempt`." The "all-ones host" is undefined for prefix lengths /31 and /32. For a **/32 host matcher, the all-ones host of a /32 equals the host address itself.** Therefore a flow whose destination is a legitimate single-host /32 zone member computes `dst == directed-broadcast(/32) == dst` and is classified `MulticastExempt` — i.e. **never a violation**, regardless of conduit policy. entities.md:23 explicitly contemplates `/32` host matchers ("A single IP is a /32"); DEC-023 discusses a `/32` host coinciding with a broadcast address but does not carve out the /32-is-its-own-broadcast degeneracy. This produces the **silent-allow / mis-classification** outcome R-002 names as the headline security risk, defeating deny-by-default for any single-host zone.
- **Evidence:** invariants.md:37 `"resolve dst to its longest-prefix zone (DI-004), then if dst equals that zone's directed-broadcast (all-ones host) address, override the Explicit MatchKind to MulticastBroadcast"`; entities.md:23 `"A single IP is a /32 (v4) or /128 (v6) CIDR — unifies matching as longest-prefix."`
- **Proposed Fix:** In DI-016 step 2 (and DEC-030) restrict the directed-broadcast override to IPv4 prefixes **/30 and shorter** (`prefix_len ≤ 30`). State explicitly that /31 (RFC 3021 point-to-point, no broadcast) and /32 (host route, all-ones host == the host) are **excluded** and resolve as ordinary `Explicit` unicast.

### MEDIUM

#### ADV-P06-MED-001: Ingest cap referenced by FM-008 is defined and enforced by nothing
- **Severity:** MEDIUM
- **Category:** interface-gaps (dangling concept) / verification-gaps
- **Location:** failure-modes.md:30 (FM-008); cross-ref FM-007 (failure-modes.md:29), ST-3 (events.md:24), DI-013 (invariants.md:34)
- **Description:** FM-008's safety argument rests on "a documented max-flow-count ingest cap (set well below `u64::MAX`)" that makes the overflow abort "a defense-in-depth backstop that should be unreachable in practice." But the ingest cap is referenced only in FM-008 (grep confirmed). No invariant, no stage (ST-3 ingest), no entity field, and not FM-007 (the natural home for an ingest bound) defines the cap's value, enforcement point, or over-cap behavior (abort? skip-and-warn like DI-013?). The cap is the load-bearing premise of FM-008's "unreachable in practice" claim yet is itself unspecified — un-testable and un-anchored. Enforcement-point mismatch: the cap logically belongs at ST-3 ingest (FM-007) but is asserted inside FM-008/ST-7 aggregate.
- **Evidence:** failure-modes.md:30 `"A documented max-flow-count ingest cap (set well below u64::MAX) makes the abort a defense-in-depth backstop that should be unreachable in practice"`; FM-007 discusses "Very large flow input exhausts memory … bounded memory per flow; documented scale limits in PRD NFRs" — mentions no flow-count cap.
- **Proposed Fix:** Either (a) define the ingest cap as a first-class concept — add it to FM-007 (its enforcement point) and a stage note in ST-3, stating cap value source (PRD NFR), over-cap behavior (abort vs truncate-and-warn), and interaction with DI-013 `skipped` semantics; or (b) soften FM-008 to not depend on an undefined cap ("checked arithmetic aborts on overflow" stands alone).

### LOW

#### ADV-P06-LOW-001: Overflow-abort exit code not reconciled with the {0,1,2} exit-code model
- **Severity:** LOW
- **Category:** ambiguous-language / contradictions
- **Location:** failure-modes.md:30 (FM-008) vs events.md:40–43 (Exit semantics)
- **Description:** FM-008 mandates overflow "abort with a distinct exit code." events.md enumerates exactly three exit codes (`0` conformant, `1` violations, `2` policy/usage/I-O error) deferring numerics to PRD, but does not allocate or acknowledge a fourth "distinct" code for the overflow/internal-abort case, nor for `--fail-on-skipped` non-zero exit. A reader reconciling the two files cannot determine whether overflow reuses code 2 or introduces a new code.
- **Evidence:** failure-modes.md:30 `"abort with a distinct exit code + diagnostic"`; events.md:40–43 `"0 conformant …, 1 violations present, 2 policy/usage/I-O error. … Exact numeric codes fixed in PRD."`
- **Proposed Fix:** Add the internal-abort/overflow exit case to the events.md Exit-semantics note (e.g. "internal invariant failure / overflow abort → distinct non-zero code, fixed in PRD") so the model is exhaustive across files.

#### ADV-P06-LOW-002: DI-018, DI-019, DI-020 are not ID-cited by any capability (traceability looseness)
- **Severity:** LOW
- **Category:** spec-fidelity / coverage-gap
- **Location:** capabilities.md (CAP-009 line 30, CAP-010 line 31) vs invariants.md (DI-018/019/020)
- **Description:** Capabilities cite DI-001–009 and DI-013–017 by ID, but DI-018 (policy digest), DI-019 (deterministic warning model), and DI-020 (PortSet canonical form) are not referenced by any CAP-NNN. They are within CAP-009/CAP-010 scope and cited elsewhere (entities, differentiators), so not a content defect — but the capability→invariant trace is incomplete, weakening BC-seeding traceability the index advertises (L2-INDEX.md:56).
- **Evidence:** grep of `DI-0NN` in capabilities.md returns no DI-018/019/020; CAP-009 cites only DI-009; CAP-010 cites only DI-009.
- **Proposed Fix:** Add DI-018 to CAP-009's Business Rule and DI-019/DI-020 to CAP-009/CAP-010 as appropriate.

#### ADV-P06-LOW-003: Severity mapping for ConnState::Other(String) is implicit only
- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** entities.md:41 (ConnState), :42 (Severity); invariants.md:38 (DI-017)
- **Description:** `Severity` is two-value (`Established | Attempted`). `ConnState` has three variants including `Other(String)`. entities.md:41 says ConnState "Drives Severity," but DI-017's only explicit rule is "Attempted when conn_state = Attempted … Established otherwise." So `Other(String)` falls into "otherwise → Established" by inference. Defensible (conservative) but never stated for `Other`, leaving an implementer to infer an exotic conn_state is graded a confirmed breach.
- **Evidence:** entities.md:42 `"Severity — violation grading from conn_state (DI-017): Established … | Attempted …"`; invariants.md:38 `"Established otherwise (incl. when conn_state is absent — conservative)"` — does not name Other.
- **Proposed Fix:** Make DI-017 explicit: "…Established otherwise (including absent conn_state and any Other(_) state — conservative)."

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 1 |
| LOW | 3 |

**Fix verification:** 8 of 9 claimed fixes RESOLVED; 1 (FM-008 ingest-cap, fix #8) PARTIALLY_RESOLVED. All L2-INDEX counts accurate and contiguous; all frontmatter at 1.5; no "saturating" strategy language remains; entities cites FM-008 correctly.

**Overall Assessment:** pass-with-findings
**Convergence:** FINDINGS_REMAIN — 1 HIGH present, so D-008 convergence clock does NOT start. This is NOT one of the 2 consecutive clean passes.
**Readiness:** requires revision (resolve ADV-P06-HIGH-001 before directed-broadcast logic is implemented)

**KEY QUESTION — Does this pass have any CRITICAL or HIGH findings?** YES. CRITICAL = 0, HIGH = 1 (ADV-P06-HIGH-001).

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 6 |
| **New findings** | 5 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (5 / 5) |
| **Median severity** | 2.0 (LOW=1, MED=2, HIGH=3 → median of [3,2,1,1,1] = 1; weighting LOW-heavy) |
| **Trajectory** | 14→16→11→15→9→5 |
| **Verdict** | FINDINGS_REMAIN |

<!--
  Novelty remains high (1.0) — all 5 findings are genuinely new (NF-1 directed-broadcast /31-/32 degeneracy and NF-2 dangling ingest cap are substantive new gaps, not retreads). Count is down (9→5) and severity is lower (1 HIGH vs prior 1 CRIT + 1 HIGH), but the 1 HIGH blocks convergence under D-008. NOT converged.
-->
