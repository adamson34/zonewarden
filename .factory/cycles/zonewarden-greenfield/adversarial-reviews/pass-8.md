---
document_type: adversarial-review
level: ops
version: "1.0"
producer: adversary
timestamp: 2026-06-17T00:00:00
phase: 1d
pass: 8
inputs: ["product-brief.md", "domain-spec/"]
traces_to: domain-spec/L2-INDEX.md
---

# Adversarial Review — Pass 8

**Scope:** FULL (product-brief.md + domain-spec/ v1.7). Convergence rule D-008: 2 consecutive passes with 0 CRIT and 0 HIGH. Result: 0 CRIT, 4 HIGH — convergence clock does NOT start; clean-pass streak remains 0. TRAJECTORY REGRESSION: Pass 7 total was 9; Pass 8 total is 15 (+6). This is the SECOND consecutive regression (5→9→15). Protocol requires investigating root cause before further convergence passes.

---

## Summary

| Severity | Count |
|----------|-------|
| CRIT | 0 |
| HIGH | 4 |
| MED | 7 |
| LOW | 4 |
| **Total** | **15** |

**Overall Assessment:** 0 CRIT, 4 HIGH. All four HIGH findings trace to a single under-specified seam: how the reserved EXTERNAL zone and the IPv4/IPv6 unspecified address (0.0.0.0/::) behave under zone resolution, IntraZone predicate evaluation, and directional verdicts. These have plausible interpretations (hence HIGH, not CRIT), but each plausible interpretation changes a security-relevant verdict, and they should block convergence until the EXTERNAL-sentinel identity rule and the unspecified-address rule (both src and dst) are written down explicitly.

**Convergence:** FINDINGS_REMAIN — iterate.

**Readiness:** requires revision.

---

## Part A — Fix Verification

All 9 Pass-7 fixes confirmed by this fresh pass:

| Pass-7 Fix | Description | Pass-8 Status |
|-----------|-------------|--------------|
| P07-HIGH-001 | brief now u64 not usize | VERIFIED (Part A #1) |
| P07-HIGH-002 | DI-017 defers to entities ConnState bucket as single source | VERIFIED (Part A #2) |
| FM-008 split: max_flows ingest cap @ST-3 | FM-008 cap | VERIFIED (Part A #3) |
| FM-009 overflow @ST-7 | FM-009 overflow | VERIFIED (Part A #3) |
| DI-015 distinct_violating_flows<=total_flows and idmz_bypasses<=total_flows | count invariants | VERIFIED (Part A #4) |
| DEC-031 multicast-in-declared-zone + 0.0.0.0/:: | multicast/unspecified | PARTIALLY VERIFIED — multicast/broadcast resolved; 0.0.0.0/:: dst hedged + src missing (see P08-HIGH-003/004) |
| DEC-030 re-anchored to CAP-005 | broadcast anchor | VERIFIED |
| DEC-031 differentiator anchored to CAP-005 | CAP anchor | VERIFIED (Part A #6) |
| glossary Violation note | Violation coherence | VERIFIED (Part A #7) |

Detailed Part A verdicts from the adversary's independent verification (verbatim):

1. **Numeric types (u64 not usize):** CONFIRMED. usize appears only in product-brief.md:131 and entities.md:30, both as the deliberate disclaimer "not platform usize." All tally counters and flow_index are u64. No stray usize typing anywhere. (Minor citation nit -> P08-LOW-004.)
2. **ConnState bucket single source of truth:** CONFIRMED. DI-017 (invariants.md:38) explicitly states "entities.md ConnState is the single source of bucket membership (no per-state list duplicated here)" and defers to entities.md:41. No competing bucket list. Correct and consistent.
3. **Failure-mode split (cap vs overflow):** CONFIRMED. FM-008 = ingest max_flows cap (ST-3); FM-009 = u64 overflow checked-arithmetic abort (ST-7). Distinct IDs, cross-referenced consistently ("the FM-008 ingest cap makes [FM-009] an unreachable defense-in-depth backstop"). No document references a merged/old single failure mode. (Brief citation cosmetics -> P08-LOW-004.)
4. **Count invariants (distinct_violating_flows <= total_flows, idmz_bypasses <= total_flows):** CONFIRMED. Both bounds are stated verbatim in DI-015 (invariants.md:36) and entities ConformanceResult (entities.md:30). Both are entailed: distinct_violating_flows de-dups by flow_index over resolved flows (<= total_flows); idmz_bypasses is per-flow additive (<= total_flows). Named quantities exist in entities + glossary. Sound.
5. **Multicast / unspecified-address handling:** FINDING. Multicast and broadcast (incl. directed-broadcast override) are well-specified (DI-016, DEC-025/030/031). However 0.0.0.0/:: handling is NOT unambiguous: dst is hedged "likely EXTERNAL" rather than a rule (-> P08-HIGH-003), and the source case is entirely unaddressed (-> P08-HIGH-004). Verdict: FINDING (P08-HIGH-003, P08-HIGH-004).
6. **Differentiator anchored to CAP-005:** CONFIRMED. differentiators.md:21 ("Validates policy against observed flows") and :22 ("62443/Purdue-native") both cite CAP-005. CAP-005 ("Resolve endpoints to zones") exists in capabilities.md:26 and is the correct, semantically-apt anchor for both. Anchor target exists and is correct.
7. **Glossary "Violation" coherence:** CONFIRMED. Glossary line 41 defines Violation as NoMatchingConduit | WrongDirection | IdmzBypass, "one flow may yield up to two entries… distinct_violating_flows de-dups by flow_index." This is consistent with entities Violation entity (:29), DI-015 accounting, DEC-005, and events ST-6. The Verdict-vs-idmz_bypass distinction (idmz_bypass is additive, NOT a VerdictKind) is maintained consistently across glossary/entities/DI-006. Coherent.

---

# Adversarial Review — zonewarden Spec Package (Phase 1, Pass 08)

I read all 12 in-scope documents in full and independently verified every Part A claim and every ID series from the raw text.

## CRITICAL Findings

None. (See final statement.)

## Important (HIGH) Findings

### P08-HIGH-001 — `external_endpoints` "both-EXTERNAL counts once" contradicts the EXTERNAL-EXTERNAL flow's verdict accounting
- Severity: HIGH
- Location: entities.md line 30 (ConformanceResult, `external_endpoints` note) + edge-cases.md DEC-026 + invariants.md DI-015
- Description: A flow with both endpoints in EXTERNAL is a legal input (DEC-026 explicitly contemplates it; DI-005 governs EXTERNAL flows "like any zone"). DI-015 requires every resolved flow to receive exactly one VerdictKind and that total_flows == intra_zone + allowed + no_matching_conduit + wrong_direction + multicast_exempt. But the spec never states which bucket an EXTERNAL->EXTERNAL flow lands in. It is not intra-zone (DI-002 requires the same zone; the spec is silent on whether two endpoints both resolving to the reserved EXTERNAL sentinel count as "the same zone"). If EXTERNAL-to-EXTERNAL is treated as IntraZone, then deny-by-default is silently bypassed for all Internet-to-Internet traffic that appears in a capture (a real R-002 "silent allow" vector). If it is not IntraZone, then it must be NoMatchingConduit unless a conduit EXTERNAL->EXTERNAL exists — but DEC-020 only documents EXTERNAL as `from`, never an EXTERNAL->EXTERNAL conduit, and it is unclear whether such a conduit is even legal.
- Why it matters: This is a deny-by-default classification gap on a documented, legal input class. The behavior determines whether the tool silently passes or flags Internet-transit traffic — a security-relevant verdict with no defined answer. An implementer will guess, and the two plausible guesses produce opposite security verdicts.
- Suggested fix: Add an invariant/edge-case stating explicitly how a both-EXTERNAL flow is classified. Recommended: treat the reserved EXTERNAL zone as a single zone so both-EXTERNAL is IntraZone (matching DI-002's literal "same zone"), OR explicitly classify as NoMatchingConduit. Either way, state it and add a DEC row mirroring DEC-026.

### P08-HIGH-002 — DI-002 "same zone" is undefined for the reserved EXTERNAL sentinel; IntraZone predicate is ambiguous
- Severity: HIGH
- Location: invariants.md DI-002, DI-005; ubiquitous-language.md line 40 ("Intra-zone flow")
- Description: DI-002 defines IntraZone as "two endpoints resolve to the same zone." DI-005 says unmatched IPs resolve to the reserved EXTERNAL zone. The glossary "Intra-zone flow" entry says "endpoints are in the same zone; allowed without a conduit." Nowhere is it stated whether two distinct unmanaged IPs (e.g., 8.8.8.8 and 1.1.1.1), both resolving to EXTERNAL, satisfy "same zone." This is the predicate root cause behind P08-HIGH-001 but is independently a HIGH ambiguity because it also affects IDMZ exclusion logic and the external_endpoints diagnostic.
- Why it matters: "Same zone" is the IntraZone gate (DI-002) which short-circuits conduit evaluation (deny-by-default). An ambiguous gate over the catch-all zone is security-relevant.
- Suggested fix: Add one sentence to DI-002 or DI-005: "Two endpoints both resolving to the reserved EXTERNAL zone {do / do not} satisfy the IntraZone predicate." Cross-reference from the glossary.

### P08-HIGH-003 — DI-016 / DEC-031: `0.0.0.0` and `::` (unspecified) as a flow destination are silently classified by ordinary zone resolution, which can route them to a managed zone and yield a spurious verdict
- Severity: HIGH (security-relevant ambiguity, but with a plausible interpretation)
- Location: edge-cases.md DEC-031; invariants.md DI-016 (step-1 / step-2 detection)
- Description: DEC-031 states 0.0.0.0/:: "are neither multicast nor broadcast -> resolve via longest-prefix (likely EXTERNAL)." But "likely EXTERNAL" is not guaranteed: a policy author could declare a zone with a matcher covering 0.0.0.0 (e.g., a non-/0 prefix such as 0.0.0.0/8, which DI-010/DEC-029 do NOT reject — only /0 is rejected). Then 0.0.0.0 as a destination resolves to that managed zone and produces a normal Allowed/NoMatchingConduit/IdmzBypass verdict for a non-routable "any" address that never appears as a legitimate destination in real traffic. The spec does not flag 0.0.0.0/::-as-destination as anomalous/skipped/warned.
- Why it matters: An unspecified address as a destination is malformed/meaningless traffic; classifying it as a real flow against a managed zone can produce false violations (noise) or, worse, contribute a spurious IdmzBypass finding. The spec's "likely EXTERNAL" hedge is not a rule.
- Suggested fix: Make 0.0.0.0/:: as a destination deterministic: either (a) skip+warn as a malformed flow, or (b) force-resolve to EXTERNAL regardless of declared matchers, or (c) explicitly state it is classified normally and accept the consequence. Replace "likely EXTERNAL" with a definite rule.

### P08-HIGH-004 — `0.0.0.0`/`::` as a flow source (initiator) is entirely unspecified
- Severity: HIGH
- Location: edge-cases.md DEC-031 (only addresses dst); entities.md Flow (`src_ip`)
- Description: DEC-031 only discusses 0.0.0.0/:: as a destination. The unspecified address as a src_ip (the connection initiator) is never addressed. A DHCP-discover or spoofed-source record in a capture can carry src=0.0.0.0. Per DI-007 directionality the src is the initiator and drives Forward/Bidirectional matching; resolving 0.0.0.0 source to a managed zone (same 0.0.0.0/8 declaration risk as P08-HIGH-003) produces a directional verdict from a meaningless initiator.
- Why it matters: Part A item 5 explicitly asks whether 0.0.0.0/:: handling is unambiguous. It is addressed for dst only; the src case is a genuine gap, and src drives the directional security verdict.
- Suggested fix: Extend DEC-031 (or add a new DEC) to cover 0.0.0.0/:: as src_ip, with the same deterministic rule chosen for the dst case.

## MEDIUM Findings

### P08-MED-001 — DI-006 IDMZ predicate is undefined when an endpoint is ImplicitExternal on the source side combined with multicast precedence ordering
- Severity: MEDIUM
- Location: invariants.md DI-016 (precedence), DI-006; edge-cases.md DEC-027
- Description: DI-016 establishes MulticastExempt short-circuits before zone-pair classification (ST-6 order), and the DI-006 truth table forces idmz_bypass=false when the dst is MulticastBroadcast. But idmz_bypass is described as "independent additive… regardless of the verdict." There is a latent precedence question: the multicast detection is a destination-only test (DI-016 step 1/2 both key on dst). The truth-table row "any | MulticastBroadcast (dst)" covers dst multicast. This is internally consistent — but the interaction of "idmz_bypass is independent of verdict" with "idmz_bypass forced false for multicast dst" is a special-cased exception to independence that is easy to implement wrong. It is correct as written but under-emphasized.
- Why it matters: Implementer divergence risk — someone implementing "idmz_bypass is independent of verdict" literally would not force it false for multicast dst.
- Suggested fix: Add an explicit note in DI-006 (not only the truth table) that multicast-dst is the single exception to verdict-independence of idmz_bypass.

### P08-MED-002 — Severity is defined only for violations, but `severity` field placement and the non-violation case are inconsistent across docs
- Severity: MEDIUM
- Location: entities.md line 29 (Violation has `severity`), line 42 (Severity); invariants.md DI-017; ubiquitous-language.md line 45
- Description: Severity is a property of a Violation (entities Violation entity carries severity). DI-017 says "A violation's severity is Attempted iff…". That is consistent. But the entities Severity value-object note says "Flows with no conn_state… default to Established" — phrased over flows, not violations. An Allowed (non-violating) flow has a conn_state but no Violation, hence no severity. The wording "flows… default to Established" implies severity is computed for all flows. Minor scope confusion: severity exists only on Violation entries.
- Why it matters: Could mislead an implementer into computing/storing severity on non-violating verdicts, bloating output and creating a determinism surface that DI-009 must then cover.
- Suggested fix: Reword the Severity value-object note to "Violations whose flow has no conn_state… default to Established," scoping it to violations.

### P08-MED-003 — `service_source` precedence (DpiConfirmed vs PortHeuristic disagreement) is an open question in the brief but the domain spec asserts a resolution without flagging it open
- Severity: MEDIUM
- Location: product-brief.md Open Questions #2 ("service_source precedence rules… still open"); entities.md ServiceSource / Canonical table; ubiquitous-language.md line 35
- Description: The brief lists "service_source precedence rules: when port-heuristic and any future DPI signal disagree, which wins" as still open. The MVP has no DPI source (Zeek conn.log only), so there is no disagreement to resolve in MVP — but the domain spec treats DpiConfirmed as "authoritative" (glossary line 35, entities line 40) without noting that no MVP adapter ever produces it. This is a latent dangling enum variant: DpiConfirmed is specified as a normative value with no MVP producer and no precedence rule.
- Why it matters: A spec'd enum variant with no producer and an explicitly-open precedence rule risks a half-implemented code path and untestable contract for MVP.
- Suggested fix: Add a note in entities ServiceSource that DpiConfirmed has no MVP adapter producer (MVP Zeek conn.log yields only PortHeuristic/Unknown); defer precedence to PRD per brief Open Question #2.

### P08-MED-004 — FM-007 (OOM, streaming) and FM-008 (`max_flows` cap) interact but the ordering/guarantee is unstated
- Severity: MEDIUM
- Location: failure-modes.md FM-007, FM-008
- Description: FM-007 mandates streaming so memory is bounded "per flow." But the engine must hold all Verdicts / Violations to produce a deterministic ordered ConformanceResult (DI-009 total ordering requires a sort over the full set). With max_flows = 1_000_000_000, materializing one Verdict per flow is up to ~1e9 structs in memory — which contradicts FM-007's "don't materialize all" intent. FM-007 addresses ingest, but the aggregation stage (ST-7) inherently materializes per-flow results for sorting; this is not reconciled.
- Why it matters: A claimed bounded-memory guarantee that the determinism requirement structurally violates at the documented cap. Either the cap is unrealistically high for the memory model, or DI-009 ordering needs an external/streaming sort. Implementer will hit OOM at scale despite "FM-007 handled."
- Suggested fix: Clarify that FM-007 bounds ingest/normalization memory, while ST-7 aggregation memory scales with the number of retained results (verdicts/violations), and either lower the default max_flows, or specify that only violations (not all verdicts) are retained, with counts streamed.

### P08-MED-005 — DI-010 rejects declared `/0` but allows other catch-all-ish prefixes (`/1`…`/7`) that partially shadow EXTERNAL without warning
- Severity: MEDIUM
- Location: invariants.md DI-010; edge-cases.md DEC-029
- Description: DEC-029/DI-010 reject 0.0.0.0/0 and ::/0 because a /0 catch-all shadows the implicit EXTERNAL zone. But a very short non-zero prefix (e.g., 0.0.0.0/1, 128.0.0.0/1) covers half the IPv4 space and similarly captures vast unmanaged ranges into a managed zone, defeating EXTERNAL's catch-all role for those IPs and altering IDMZ exclusion. The rule draws the line only at exactly /0.
- Why it matters: The stated rationale for DEC-029 ("would shadow the implicit EXTERNAL zone and suppress IDMZ-EXTERNAL exclusions") applies in degree to any over-broad prefix, not just /0. A user can re-introduce the exact failure DEC-029 guards against with /1. Security-relevant but easily mitigated and unlikely in practice.
- Suggested fix: Either justify why only /0 is special (it is the unique value that matches every address including the unspecified), or add a load-time warning for suspiciously broad prefixes (e.g., prefix < /8). At minimum document the deliberate /0-only boundary.

### P08-MED-006 — IPv4 broadcast detection (DI-016 step 2) ignores the all-zeros host "network address" as a destination
- Severity: MEDIUM
- Location: invariants.md DI-016 step 2; edge-cases.md DEC-030
- Description: DI-016 step 2 exempts the all-ones host (directed broadcast, e.g. 10.0.0.255 for /24). It does not address the all-zeros host (network address, e.g. 10.0.0.0/24's 10.0.0.0), which is also not a valid unicast destination. A dst of 10.0.0.0 resolves as a normal Explicit endpoint and gets a unicast verdict. This is asymmetric with the broadcast handling and can produce spurious verdicts for the network address.
- Why it matters: Same class as the directed-broadcast issue the spec deliberately handles (R-002 silent-allow / false-positive avoidance); the network-address case is left as a normal classification, an inconsistency in the multicast/broadcast exemption design.
- Suggested fix: Decide and document whether the all-zeros network address as a destination is (a) exempted like broadcast, (b) skipped/warned, or (c) classified normally — and state the rationale for the asymmetry if (c).

### P08-MED-007 — `max_flows` cap reaching behavior loses already-valid results without stating partial-output policy
- Severity: MEDIUM
- Location: failure-modes.md FM-008; invariants.md DI-011 ("no partial state")
- Description: FM-008 says reaching max_flows "aborts with exit code 2." It does not state whether any partial ConformanceResult/report is emitted or whether prior in-flight verdicts are discarded. DI-011 ("no partial state") is scoped to policy, not flow processing. For a CI tool, aborting at the cap with no diagnostic detail beyond exit-2 is acceptable, but the silent loss of an otherwise-valid partial analysis vs. an explicit "no output on cap-abort" is unspecified.
- Why it matters: Determines whether an over-cap run is fully wasted or yields partial evidence; affects CI ergonomics and the determinism contract.
- Suggested fix: State explicitly that an over-cap run emits no ConformanceResult (clean abort) — consistent with deterministic all-or-nothing — and that the diagnostic names the cap value.

## LOW / Observations

### P08-LOW-001 — [process-gap] L2-INDEX "ID Registry Summary" hardcodes counts that must be manually kept in sync
- Severity: LOW [process-gap]
- Location: L2-INDEX.md lines 64-74
- Description: The index hardcodes CAP=14, DI=20, ST=8, DEC=31, ASM=9, R=6, FM=9 and asserts "ID-count is authoritative as listed here." I independently counted and all match (see ID-count table). No defect today — but there is no automated assertion tying these numbers to the actual files; a future addition will drift silently. This is the VP-INDEX-style drift class flagged in the review protocol, applied to the L2 index.
- Suggested fix: Add a CI check (or a generated index) asserting the registry counts equal the grepped row counts.

### P08-LOW-002 — Glossary "Direction" lists `unidirectional` as an alias but the brief/PRD-pin note says "CAP-001 / PRD pin the full grammar" — grammar authority is split
- Severity: LOW
- Location: ubiquitous-language.md line 50; entities.md line 37; invariants.md DI-010
- Description: Three docs each state the direction token grammar (forward/bidirectional, unidirectional=alias). DI-010 makes token legality a load-time validation. This is consistent across all three (verified), but the canonical grammar is asserted in three places plus deferred to PRD ("CAP-001 / PRD pin the full grammar") — a single-source-of-truth smell. No current contradiction.
- Suggested fix: Nominate one doc (entities Value Objects) as the canonical token grammar and have the others reference it.

### P08-LOW-003 — RSTO bucket note references a "previously-wrong" mapping as if there were prior versions in this fresh artifact
- Severity: LOW (editorial)
- Location: entities.md line 41 ("the examples here fix the previously-wrong RSTO bucket")
- Description: The spec self-references a prior erroneous state ("previously-wrong"). In a delivered spec this is dangling provenance. Functionally, RSTO/RSTR are placed in Established ("connection was established then reset"), which is technically correct for Zeek conn_state semantics. No defect in the mapping; the meta-comment is editorial noise.
- Suggested fix: Drop "the examples here fix the previously-wrong" phrasing; keep the BACKLOG note that the full 13-state mapping is PRD-pinned.

### P08-LOW-004 — Brief Constraints cite "FM-008" for u64 rationale, but the u64/overflow rationale lives in FM-009; FM-008 is the cap
- Severity: LOW
- Location: product-brief.md line 131 ("see entities.md / FM-008")
- Description: The brief's u64-not-usize note points to FM-008. FM-008 is the max_flows cap; the u64 overflow handling rationale is FM-009. The cap (FM-008) is the relevant backstop, so the citation is defensible, but the overflow/checked-arithmetic story is FM-009. A reader chasing "why u64" lands on the cap, not the overflow handling.
- Suggested fix: Cite "FM-008/FM-009" in the brief constraint line.

---

## ID-Count Table

| Prefix | Count Found | Index Claims | Contiguous? | Match? |
|--------|-------------|--------------|-------------|--------|
| DI (invariants) | 20 (DI-001...020) | 20 | Yes, no gaps/dups | CONFIRMED |
| DEC (edge cases) | 31 (DEC-001...031) | 31 | Yes | CONFIRMED |
| FM (failure modes) | 9 (FM-001...009) | 9 | Yes | CONFIRMED |
| ASM (assumptions) | 9 (ASM-001...009) | 9 | Yes | CONFIRMED |
| CAP (capabilities) | 14 (CAP-001...014) | 14 | Yes | CONFIRMED |
| ST (stages) | 8 (ST-1...8) | 8 | Yes | CONFIRMED |
| R (risks) | 6 (R-001...006) | 6 | Yes | CONFIRMED |

Notes: the DEC series is the edge-cases catalog (31 IDs). The differentiators document carries no IDs (prose->CAP traceability table) by design per the L2-INDEX document map, not a numbering error. Priority distribution (P0=10, P1=2, P2=2) sums to 14 = CAP count: CONFIRMED.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 8 |
| New findings | 11 (the 4 HIGH + 7 MED are new region; the 4 LOW are refinements/restatements of prior LOW/MED themes) |
| Duplicate/variant findings | 4 |
| Novelty score | 0.73 |
| Median severity | 2.5 |
| Trajectory | 14→16→11→15→9→5→9→15 |
| Verdict | FINDINGS_REMAIN |

Novelty: MEDIUM. HIGH findings cluster on one genuinely under-specified region: the behavior of the reserved EXTERNAL sentinel and the unspecified address at the edges of zone resolution (HIGH-001/002 = EXTERNAL-as-zone identity; HIGH-003/004 = 0.0.0.0/:: as dst/src). The multicast/broadcast machinery is thorough, but the adjacent "unspecified address" and "both-EXTERNAL" cases were left as hedges ("likely EXTERNAL") rather than rules — a coherent blind spot.

## THE KEY QUESTION — Are there genuine CRIT or HIGH findings remaining?

TOTAL CRIT: 0.
TOTAL HIGH: 4 — genuine, not manufactured: P08-HIGH-001 (both-EXTERNAL flow classification undefined — deny-by-default gap), P08-HIGH-002 ("same zone" undefined for the EXTERNAL sentinel — ambiguous IntraZone gate), P08-HIGH-003 (0.0.0.0/:: as dst is "likely EXTERNAL", not a deterministic rule; reachable via a legal /1–/8 declaration), P08-HIGH-004 (0.0.0.0/:: as src/initiator entirely unspecified). All four trace to a single under-specified seam: how the reserved EXTERNAL zone and the IPv4/IPv6 unspecified address behave under zone resolution, IntraZone, and directional verdicts. They have plausible interpretations (hence HIGH, not CRIT), but each plausible interpretation changes a security verdict, so they should block convergence until the EXTERNAL-sentinel identity rule and the unspecified-address rule (both src and dst) are written down explicitly.
