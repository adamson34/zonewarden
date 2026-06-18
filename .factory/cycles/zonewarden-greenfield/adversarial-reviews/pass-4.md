---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-06-17T13:00:00
phase: 1d
inputs: ["product-brief.md", "domain-spec/"]
input-hash: "[live-state]"
traces_to: domain-spec/L2-INDEX.md
pass: 4
previous_review: pass-3.md
scope: "specs --scope=full (product-brief.md + domain-spec/ v1.3)"
---

# Adversarial Review: zonewarden (Pass 4)

**Pass:** 4
**Scope:** `specs --scope=full`
**Target:** `product-brief.md` + `domain-spec/` at **v1.3**
**Reviewer mode:** Spec Review (Phase 1), fresh context
**Finding ID convention:** `ADV-ZWGF-P04-<SEV>-<SEQ>` (cycle `ZWGF`, pass `P04`, three-digit seq); `SEV in {CRIT, HIGH, MED, LOW}`

---

## Part A — Fix Verification

| ID | Verification Item | Status | Evidence |
|----|-------------------|--------|----------|
| ADV-ZWGF-P03-HIGH-001 | policy_digest canonicalization fully specified & unambiguous | **PARTIALLY_RESOLVED** | DI-018 (invariants.md:39) specifies: canonical JSON, UTF-8, object keys sorted lexicographically; zones sorted by `id`; conduits de-duplicated then sorted by `(from_zone, to_zone, proto, direction, normalized_ports)`; each `PortSet` normalized (ranges merged, overlaps collapsed, sorted; `Any` is a distinct sentinel); `None`/absent Option fields omitted; `SHA-256` (lowercase hex). Most criteria present. Remaining gaps (see P04-HIGH-001): (a) sort key uses `proto`/`direction`/PortSet enums whose canonical *string form* used as a sort key is never defined; (b) "sorted lexicographically" gives no collation (byte vs codepoint); (c) `Zone.name` inclusion in the digest unspecified; (d) number/float encoding for `sl_t` and FR vector unspecified. |
| ADV-ZWGF-P03-HIGH-002 | external_endpoints precisely defined; both-EXTERNAL once; diagnostic-only outside DI-015 | **RESOLVED** | entities.md:30 + DEC-026 (edge-cases.md:44) both define external_endpoints = count of flows with >=1 EXTERNAL endpoint, both-EXTERNAL once, diagnostic and excluded from the DI-015 accounting identity. Unambiguous and consistent. |
| ADV-ZWGF-P03-MED-001 | DEC-007 scoped to the "Unknown" result only | **RESOLVED** | DEC-007 (edge-cases.md:25): known OT service on a non-default port -> `service_source = Unknown` (never PortHeuristic), consistent with entities.md:47. (Item wording "Unknown verdict" is imprecise — DEC-007 governs `service_source`, not a VerdictKind; the fix is correct.) |
| ADV-ZWGF-P03-MED-002 | Multicast precedence: short-circuits before IntraZone; truth-table row; idmz_bypass=false honored | **RESOLVED** | (a) DI-016 (invariants.md:37): MulticastExempt short-circuits before zone-pair classification; ST-6 (events.md:27) lists MulticastExempt before IntraZone. (b) Truth-table row present (invariants.md:57). (c) DI-016 + DI-006 force `idmz_bypass = false` when either endpoint is MulticastBroadcast. DEC-027 covers intra-zone+multicast. All three sub-parts hold. |
| ADV-ZWGF-P03-MED-003 | Broadcast/multicast ranges complete & correct | **RESOLVED** | DI-016 (invariants.md:37): IPv4 multicast 224.0.0.0/4, limited broadcast 255.255.255.255, directed broadcast = all-ones host of longest-prefix-matched zone CIDR; IPv6 ff00::/8, no broadcast. Ranges correct. (See P04-HIGH-002 for a NEW ordering-circularity in the *directed broadcast* clause — ranges right, algorithm contradictory with ST-6.) |
| ADV-ZWGF-P03-MED-004 | Other(String) payloads verbatim, exact-byte | **RESOLVED** | DI-009 (invariants.md:30): Other(String)/Other(u8) payloads preserved verbatim (no Unicode/case normalization), compared by exact bytes; byte-stable iff input is. entities.md:40-41 reinforce. |
| ADV-ZWGF-P03-MED-005 | Tallies pinned 64-bit; no narrower types implied | **PARTIALLY_RESOLVED** | Brief (product-brief.md:131) pins "64-bit targets only (tally counters assume 64-bit `usize`)"; entities.md:30 "All tallies are 64-bit"; FM-007 references `u64::MAX`. Inconsistency (see P04-MED-001): spec mixes platform-dependent `usize` with abstract "64-bit" and `u64`; digest-relevant counts (`external_endpoints`, `flow_index`) declared `usize`, coupling stable identity to a platform-dependent width. |
| ADV-ZWGF-P03-LOW-001 | DEC renumbered to contiguous 001-027, no gaps/dups | **RESOLVED (caveat)** | All 27 DEC IDs present exactly once, gap/dup-free. Caveat (see P04-LOW-001): DEC-025 placed out of numeric order (between DEC-013 and DEC-014); the *set* is contiguous but table ordering is not monotonic. |
| ADV-ZWGF-P03-LOW-002 | Unknown-token policy error (DI-010 rejects unrecognized tokens) | **RESOLVED** | DI-010 (invariants.md:31): unrecognized direction/proto or malformed PortSet tokens are policy errors (no permissive defaulting). (Minor cross-ref drift in entities.md:21 — see P04-MED-002 — but the rule is present.) |
| ADV-ZWGF-P03-LOW-003 | flow_index dense / gap-free | **RESOLVED** | entities.md:25 + DI-013 (invariants.md:34): flow_index is dense, gap-free `0..n` among successfully-normalized flows; skipped records do not consume an index. Consistent. |
| ADV-ZWGF-P03-LOW-004 | Glossary "Flow" refreshed/consistent | **RESOLVED** | ubiquitous-language.md:30 defines Flow with the full current attribute set and points to entities.md as authoritative; flow_index has its own glossary entry. Matches entities.md:25. |

### Part A structural sanity check (v1.3 counts)

| Expected (target) | Actual in spec | Status |
|-----------------|----------------|--------|
| DI 1-18 (18 invariants) | **DI-001..DI-019 = 19 invariants**; L2-INDEX.md:67 also says 19. | **MISMATCH — see P04-HIGH-003.** Internally consistent (file + index agree on 19) but contradicts the stated v1.3 target of 18. Must be adjudicated. |
| DEC 1-27 (27 decisions) | DEC-001..DEC-027 = 27, all present, no gaps/dups; L2-INDEX.md:69 says 27. | CONFIRMED (ordering caveat in P04-LOW-001). |
| ASM 1-9 (9 assumptions) | ASM-001..ASM-009 = 9; L2-INDEX.md:70 says 9. | CONFIRMED. |
| CAP count | CAP-001..CAP-014 = 14; L2-INDEX.md:66 says 14. | CONFIRMED. |
| ST count | ST-1..ST-8 = 8; L2-INDEX.md:68 says 8. | CONFIRMED. |
| FM count | FM-001..FM-007 = 7; L2-INDEX.md:72 says 7. | CONFIRMED. |
| R count | R-001..R-006 = 6; L2-INDEX.md:71 says 6. | CONFIRMED. |

---

## Part B — New Findings

### HIGH

#### ADV-ZWGF-P04-HIGH-001: Canonical-serialization sort key under-specified (enum/PortSet form + collation)
- **Severity:** HIGH
- **Category:** verification-gaps / determinism
- **Location:** domain-spec/invariants.md:39 (DI-018)
- **Description:** The conduit sort key `(from_zone, to_zone, proto, direction, normalized_ports)` relies on a total order over `proto`, `direction`, and `normalized_ports`, but the canonical *serialized form* of those enum values used for sorting is never defined. `Proto::Other(u8)`, `Direction::{Forward, Bidirectional}`, and `PortSet` (with `Any` sentinel) have no defined string/byte rendering for comparison. Two implementations could order them differently and produce different digests for the same model — defeating DI-018 and DI-009 byte-stability.
- **Evidence:** DI-018 defines which fields are keys and how PortSet normalizes, but never how proto/direction/PortSet-as-key serialize. "object keys sorted lexicographically" gives no collation (byte vs codepoint), which matters for non-ASCII zone `id`/`Other(String)` under DI-009's verbatim rule.
- **Proposed Fix:** Define the exact JSON token each enum variant serializes to (e.g. `Proto::Tcp`->"tcp", `Proto::Other(6)`->"other:6", `Direction::Forward`->"forward"); the PortSet canonical JSON form and its sort position relative to `Any`; state byte-wise UTF-8 collation; decide whether `Zone.name` and `sl_t` are digested and how numbers encode.

#### ADV-ZWGF-P04-HIGH-002: Directed-broadcast detection vs zone-resolution ordering circularity
- **Severity:** HIGH
- **Category:** contradictions / verdict-precedence
- **Location:** domain-spec/invariants.md:37 (DI-016) vs events.md:26-27 (ST-5/ST-6), entities.md:27
- **Description:** DI-016 defines directed broadcast as "the all-ones host of the longest-prefix-matched zone's CIDR" — detection requires having resolved dst to a zone CIDR. But ST-5 zone resolution produces MatchKind that *includes* MulticastBroadcast (DI-003), and ST-6 says MulticastExempt short-circuits *before* zone-pair classification. DI-003 mandates exactly one outcome per endpoint, so a directed-broadcast dst both "resolves to a zone" (to compute the all-ones host) and "resolves to MulticastBroadcast." The spec never states which, or defines a sub-stage to reclassify.
- **Evidence:** DI-003 (invariants.md:24): every endpoint resolves to exactly one outcome. DI-016 (invariants.md:37): directed broadcast = all-ones host of longest-prefix-matched zone CIDR. ST-6 (events.md:27): MulticastExempt listed first. No defined sub-stage resolves dst zone then tests dst == zone broadcast and reclassifies.
- **Proposed Fix:** Specify the algorithm: ST-5 resolves dst to its longest-prefix zone; if dst equals that zone's directed-broadcast address (or in 224.0.0.0/4 / == 255.255.255.255 / in ff00::/8), override dst MatchKind to MulticastBroadcast (precedence over Explicit) -> ST-6 yields MulticastExempt. Clarify family-wide ranges (no zone needed) vs directed broadcast (zone needed) as distinct sub-steps, and that EXTERNAL dst (no matched CIDR) cannot have a directed broadcast.

#### ADV-ZWGF-P04-HIGH-003: DI invariant count is 19, contradicts declared v1.3 target of 18
- **Severity:** HIGH
- **Category:** spec-fidelity / count mismatch
- **Location:** domain-spec/invariants.md (DI-001..DI-019), L2-INDEX.md:67
- **Description:** The invariant count is 19 (DI-001..DI-019), and L2-INDEX agrees (19). The declared v1.3 structure expects 18. Either the target is stale, an invariant slipped in without updating the target, or two invariants that should have merged did not. Material because downstream BCs/PRD trace to specific DI-NNN IDs. Likely culprit: DI-019 (Deterministic warning model) which its own text says is "part of DI-009."
- **Evidence:** invariants.md:40 ends at "DI-019 | Deterministic warning model." L2-INDEX.md:67 says 19. Declared v1.3 structure: 18.
- **Proposed Fix:** Adjudicate intent. If 19 is correct, update target/changelog and confirm downstream consumers. If 18, merge DI-019 into DI-009 (its text says it is part of DI-009) and renumber. Blocks convergence until reconciled.

### MEDIUM

#### ADV-ZWGF-P04-MED-001: Tally width described three ways (usize / "64-bit" / u64)
- **Severity:** MEDIUM
- **Category:** spec-fidelity / determinism
- **Location:** product-brief.md:131, entities.md:25,30, failure-modes.md:29, invariants.md:36 (DI-015)
- **Description:** Same counters described as `usize` (brief + entity fields), abstract "64-bit" (entities/DI-015), and `u64` (FM-007 `u64::MAX`). `usize` is platform-width, only 64-bit by the build-target constraint; FM-007 overflow check assumes `u64`. Mixing invites a `u32` somewhere or mishandled overflow; a narrower flow_index would change the DI-009 tiebreaker domain.
- **Evidence:** product-brief.md:131 "64-bit `usize`"; entities.md:30 "All tallies are 64-bit"; failure-modes.md:29 "near `u64::MAX`".
- **Proposed Fix:** Pin one canonical width — recommend "all tallies and flow_index are `u64` (not usize)", or explicitly state usize==u64 on the pinned target and that u64::MAX is the canonical overflow bound. Align FM-007 and entity declarations.

#### ADV-ZWGF-P04-MED-002: Zone-tie rule mis-anchored to DI-004 instead of DI-010
- **Severity:** MEDIUM
- **Category:** spec-fidelity / cross-reference drift
- **Location:** entities.md:22 (Zone), invariants.md:25 (DI-004) vs :31 (DI-010)
- **Description:** Zone entity row cites DI-004 for "must not produce equal-length prefix ties," but DI-004 is longest-prefix overlap *resolution* (says nothing about rejecting ties). Tie *rejection* is DI-010 (and DEC-002 attributes it to DI-010). Mis-anchor will mislead implementers into thinking DI-004 rejects ties.
- **Evidence:** entities.md:22 "...equal-length prefix ties...(DI-004)". DI-004 (invariants.md:25) = longest prefix wins. DI-010 (invariants.md:31) = no membership ties (defines tie). DEC-002 (edge-cases.md:20) attributes tie-rejection to DI-010.
- **Proposed Fix:** Change the tie-rejection citation in entities.md:22 from (DI-004) to (DI-010), or cite both with roles distinguished (ties rejected at load DI-010; non-tie overlaps resolved longest-prefix DI-004).

#### ADV-ZWGF-P04-MED-003: FM-007 anchored to ST-6 but describes ST-3/ST-7 concerns
- **Severity:** MEDIUM
- **Category:** spec-fidelity / subsystem mis-anchor
- **Location:** failure-modes.md:29 (FM-007)
- **Description:** FM-007 subsystem is ST-6 (Classify flow), but the failure mode is "large flow input exhausts memory or overflows tally counters" with recovery "stream flows" — these are ST-3 (Ingest) and ST-7 (Aggregate) concerns. ST-6 is per-flow, stateless. Mis-attribution routes the test/owner to the wrong stage.
- **Evidence:** FM-007 (failure-modes.md:29) subsystem ST-6 + "Stream flows...64-bit tallies." events.md:35-39: ST-3 streaming iterator, ST-7 aggregation/tally.
- **Proposed Fix:** Re-anchor FM-007 to ST-3/ST-7, or split into streaming-memory (ST-3) and tally-overflow (ST-7).

#### ADV-ZWGF-P04-MED-004: Tally-overflow behavior undefined (silent-wrap risk)
- **Severity:** MEDIUM
- **Category:** missing-edge-cases / silent failure
- **Location:** failure-modes.md:29 (FM-007), entities.md:30
- **Description:** FM-007 raises tally overflow and entities.md:30 says "inputs assumed below the overflow threshold," but no behavior is defined at/over the threshold (saturate? abort? wrap?). Release Rust wraps silently, producing a wrong-but-plausible ConformanceResult violating the DI-015 accounting identity with no signal. Astronomically unlikely (~1.8e19) hence MEDIUM, but the spec raised it then left it undefined.
- **Evidence:** FM-007 detection "counter near u64::MAX" but recovery only "documented max flow count." entities.md:30 "assumed below the overflow threshold." No invariant/FM defines behavior.
- **Proposed Fix:** Use checked/saturating arithmetic; on overflow abort with a distinct exit code + diagnostic, OR enforce a documented hard input cap at ingest. State which.

#### ADV-ZWGF-P04-MED-005: IPv4-mapped IPv6 canonicalization unspecified
- **Severity:** MEDIUM
- **Category:** missing-edge-cases / determinism
- **Location:** entities.md:23 (AssetMatcher), edge-cases.md:30 (DEC-012), invariants.md:31 (DI-010 tie = "same IP family")
- **Description:** Matching is "family-aware longest-prefix" and ties are scoped "same IP family," but canonicalization of IPv4-mapped IPv6 (::ffff:a.b.c.d) is never defined. A mapped-form src matching a plain-IPv4 zone CIDR would be treated as IPv6, match no IPv4 matcher, and resolve EXTERNAL (R-002 silent mis-resolve). Conversely, normalizing to IPv4 makes the "same IP family" tie boundary ambiguous.
- **Evidence:** DEC-012 (edge-cases.md:30): "family-aware longest-prefix." DI-010 (invariants.md:31): "same IP family." No mapped-address handling anywhere.
- **Proposed Fix:** Add/extend DEC-012 to specify canonicalization (e.g. mapped addresses canonicalized to IPv4 before resolution, or treated strictly as IPv6 with documented consequence). Add a fixture.

#### ADV-ZWGF-P04-MED-006: PortSet normalization — adjacency merge and 0-65535-vs-Any undefined
- **Severity:** MEDIUM
- **Category:** missing-edge-cases / determinism
- **Location:** entities.md:39 (PortSet), invariants.md:39 (DI-018), edge-cases.md:27 (DEC-009)
- **Description:** DI-018 requires "ranges merged, overlaps collapsed" for the digest but never defines whether *adjacent* ranges merge (500-502 + 503-510), whether a singleton adjacent to a range merges, or how 0-65535 relates to `Any`. These make normalization non-deterministic across implementations. Critically, 0-65535 and `Any` are NOT semantically equal (0-65535 never matches a portless flow per DEC-021; `Any` does), so they must not be merged — but the spec never says so.
- **Evidence:** DI-018 (invariants.md:39): "ranges merged...Any is a distinct sentinel." entities.md:39: "Any matches...portless flows; an explicit port set never matches a portless flow."
- **Proposed Fix:** Specify: (a) whether adjacent ranges/ports coalesce (recommend yes for a unique form); (b) explicitly 0-65535 is NOT folded into Any (DEC-021); (c) canonical representation = sorted non-overlapping non-adjacent inclusive [lo,hi] ranges. Add fixtures.

#### ADV-ZWGF-P04-MED-007: Duplicate YAML mapping keys not addressed (review != execution risk)
- **Severity:** MEDIUM
- **Category:** missing-edge-cases / parser robustness
- **Location:** invariants.md:31 (DI-010), capabilities.md:22 (CAP-001), failure-modes.md:24 (FM-002)
- **Description:** DI-010 validates uniqueness/existence/purdue/ties/EXTERNAL/token-legality but says nothing about duplicate keys within a YAML mapping. serde_yaml default last-wins silently could produce a model the reviewer didn't intend yet a stable digest — contradicting CAP-001 ("never partially loaded") and the digest-trust model. FM-002 only covers syntax/schema-invalid.
- **Evidence:** CAP-001 (capabilities.md:22): "never partially loaded." DI-010 (invariants.md:31) omits duplicate-key handling. FM-002 (failure-modes.md:24): "YAML syntax or schema-invalid."
- **Proposed Fix:** Add to DI-010 (or a DEC): duplicate mapping keys are a load-time error (deny silent last-wins). Note deliberate divergence from serde_yaml defaults (deny_unknown_fields + duplicate-key detection).

### LOW

#### ADV-ZWGF-P04-LOW-001: DEC-025 placed out of numeric order
- **Severity:** LOW
- **Category:** ambiguous-language / readability
- **Location:** edge-cases.md:32 (DEC-025 placement)
- **Description:** DEC-025 sits between DEC-013 and DEC-014 while DEC-026/027 are appended at the end. The ID set is complete and gap-free, so cosmetic, but confuses readers/tooling assuming row order = ID order.
- **Evidence:** Table order: DEC-013, DEC-025, DEC-014.
- **Proposed Fix:** Move DEC-025 to its numeric position (after DEC-024). Non-blocking.

#### ADV-ZWGF-P04-LOW-002: SensorDpi superseded-name leak risk (research only; spec clean)
- **Severity:** LOW
- **Category:** spec-fidelity / consistency
- **Location:** ubiquitous-language.md:35,50 vs entities.md:40,37
- **Description:** Glossary service_source and Direction entries are consistent with entities. The research file still uses old `SensorDpi`; no under-review file leaks it (grep confirms). Forward note only.
- **Evidence:** entities.md:40 "DpiConfirmed is the normative name; supersedes the research draft's SensorDpi." No SensorDpi in any under-review file.
- **Proposed Fix:** None required. Optional: add a "research uses superseded SensorDpi" pointer to prevent copy-paste regressions.

#### ADV-ZWGF-P04-LOW-003: Zeek conn_state -> ConnState mapping incomplete; RSTO bucket likely wrong
- **Severity:** LOW
- **Category:** ambiguous-language / missing-edge-cases
- **Location:** entities.md:41 (ConnState), invariants.md:38 (DI-017)
- **Description:** ConnState buckets Zeek states into Established/Attempted/Other but gives only examples, not a total mapping of all 13 Zeek values. entities.md:41 buckets RSTO as Attempted ("handshake not completed"), but Zeek defines RSTO = "Established, originator aborted (sent RST)" — i.e. it WAS established; DI-017 severity grading depends on this. Nine states have no stated bucket.
- **Evidence:** entities.md:41: "Attempted (e.g. S0, REJ, RSTO — handshake not completed)." Research: "RSTO | Established, originator aborted." DI-017 only exemplifies S0/REJ.
- **Proposed Fix:** Provide a complete 13-row Zeek-state -> bucket table; reconcile RSTO against Zeek's "established then aborted" semantics. LOW (severity grading is a confidence/UX feature, not the verdict), but the RSTO example looks factually wrong.

#### ADV-ZWGF-P04-LOW-004: Declared 0.0.0.0/0 (or ::/0) zone member unaddressed
- **Severity:** LOW
- **Category:** missing-edge-cases / security-surface
- **Location:** invariants.md:25 (DI-004), entities.md:23
- **Description:** DI-004 says EXTERNAL is not a 0.0.0.0/0 matcher, but the spec never says whether a user may *declare* a zone member of 0.0.0.0/0 or ::/0. If allowed, it shadows EXTERNAL entirely (every unmatched IP matches it as longest-prefix /0), silently changing semantics and suppressing IDMZ-EXTERNAL exclusions; if disallowed it should be a DI-010 validation.
- **Evidence:** DI-004 (invariants.md:25): "EXTERNAL is not a 0.0.0.0/0 matcher." entities.md:23 covers /32//128 but not /0.
- **Proposed Fix:** Add a DEC: a declared /0 is either explicitly permitted (documented as a family catch-all that suppresses EXTERNAL for that family) or rejected/warned at validation.

#### ADV-ZWGF-P04-LOW-005: CAP-007 under-cites DI-015 for "exactly one VerdictKind"
- **Severity:** LOW
- **Category:** spec-fidelity / traceability
- **Location:** capabilities.md:28 (CAP-007) vs entities.md:28 (Verdict)
- **Description:** CAP-007 verdict set matches entities.md (5 kinds), but its Business Rule cites DI-016/DI-017 and not DI-015 (verdict totality/exactly-one) or DI-002 — the invariants that make "exactly one VerdictKind" true. Traceability under-citation, not a contradiction.
- **Evidence:** capabilities.md:28: "Assign each flow exactly one VerdictKind...Deny-by-default (DI-001); multicast exemption (DI-016); severity grading (DI-017)." DI-015 uncited.
- **Proposed Fix:** Add DI-015 (and optionally DI-002) to CAP-007's Business Rule citations. Non-blocking.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 3 |
| MEDIUM | 7 |
| LOW | 5 |

Part A: 8 RESOLVED, 2 PARTIALLY_RESOLVED (HIGH-001, MED-005), plus structural DI-count MISMATCH (folded into P04-HIGH-003).

**Overall Assessment:** pass-with-findings -> BLOCK on three HIGH items (P04-HIGH-001 digest sort-key under-specification, P04-HIGH-002 directed-broadcast/zone-resolution circularity, P04-HIGH-003 DI count 19 vs target 18). The v1.3 revision resolved 8/11 prior items fully and all DEC/ASM counts confirmed.
**Convergence:** NOT CONVERGED — findings remain, iterate. Per the Semantic Anchoring Audit, the DI-004/DI-010 tie mis-anchor (P04-MED-002) and FM-007 subsystem mis-anchor (P04-MED-003) independently block convergence.
**Readiness:** Not ready to seed PRD/BCs. After P04-HIGH-001/002/003 and P04-MED-002/003 are fixed (ideally also MED-005/006/007), the spec should be one short pass from convergence.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 15 (3 HIGH, 7 MED, 5 LOW) |
| **Duplicate/variant findings** | ~0 direct duplicates; P04-HIGH-001 extends prior digest work (sort-key serialization gap) rather than re-treading it |
| **Novelty score** | ~1.0 (15 new / 15 total) |
| **Median severity** | MEDIUM |
| **Trajectory** | 14 -> 16 -> 11 -> 15 |
| **Verdict** | FINDINGS_REMAIN / NOT CONVERGED |

**Novelty narrative:** NOT a converging trajectory. Pass 4 surfaced 3 genuinely new HIGH findings prior passes missed: the directed-broadcast/zone-resolution ordering circularity (structural, index/field reviews miss it), the DI-count contradiction (only visible enumerating against the declared target), and the digest sort-key serialization gap (a deeper layer of the digest work). Count rose 11 -> 15 with three HIGHs and MEDIAN MEDIUM — the opposite of nitpick-decay. Minimum 3 clean passes NOT met (this pass is not clean).

### Process-gap note
- **[process-gap]** The DI-count contradiction (P04-HIGH-003) and DEC-025 out-of-order placement (P04-LOW-001) indicate there is no automated ID-registry contiguity/ordering check validating each domain-spec/*.md against the L2-INDEX ID Registry Summary counts and the declared target structure. Recommend a lint that enumerates IDs per file and asserts contiguous + monotonic + matching the index count.
