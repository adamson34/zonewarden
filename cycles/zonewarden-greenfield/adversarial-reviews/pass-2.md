---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-06-17T00:00:00
phase: "1d"
inputs: [product-brief.md, "domain-spec/"]
input-hash: "[live-state]"
traces_to: domain-spec/L2-INDEX.md
pass: 2
previous_review: pass-1.md
---

# Adversarial Review: zonewarden (Pass 2)

> Scope: specs --scope=full (product-brief.md + domain-spec/ v1.1). Fresh-context pass. Part A verifies Pass-1 fixes; Part B reports new findings.

## Part A — Fix Verification

The adversary was given only the Pass-1 finding IDs (not their text or fixes) and judged each area cold.

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-ZWGF-P01-CRIT-001 | CRITICAL | PARTIALLY_RESOLVED | SlTarget scalar/optional-vector contradiction resolved (entities.md:36 explicit "Resolves the prior 7-vector-vs-scalar contradiction"; ubiquitous-language.md:28). BUT a CRIT-level accounting/verdict-totality contradiction remains live (see ADV-ZWGF-P02-CRIT-001): DI-015 "exactly one VerdictKind per resolved flow" vs DI-006/DEC-005 "a flow may carry both a conduit verdict and an IdmzBypass." Net PARTIALLY_RESOLVED. |
| ADV-ZWGF-P01-HIGH-001 | HIGH | RESOLVED | DI-009 (invariants.md:30) now specifies total-order key (ts, src_ip, src_port, dst_ip, dst_port, proto) with original input index as final stable tiebreaker — unique ordering over any flow multiset. Testable. |
| ADV-ZWGF-P01-HIGH-002 | HIGH | RESOLVED | EXTERNAL consistently defined as reserved, membership-less, fixed L5, governed by conduits not IDMZ (ubiquitous-language.md:39, entities.md:35, DI-005 invariants.md:26), excluded from DI-006; DEC-017 (edge-cases.md:35) closes the <=L3->EXTERNAL question. |
| ADV-ZWGF-P01-HIGH-003 | HIGH | RESOLVED | DEC-016 (edge-cases.md:34): one connection record = exactly one Flow keyed on originator, responder traffic implicit, explicitly avoids false WrongDirection. Reinforced by ubiquitous-language.md:30 and CAP-003. |
| ADV-ZWGF-P01-HIGH-004 | HIGH | RESOLVED | DI-014 (invariants.md:35) specifies any-match union, overlaps legal, first-match ordering irrelevant. Reinforced by DEC-015 and CAP-006. Deterministic and testable. |
| ADV-ZWGF-P01-MED-001 | MEDIUM | PARTIALLY_RESOLVED | DEC-005 (edge-cases.md:23) + DI-006 specify IdmzBypass raised in addition to a conduit verdict, but this is the behavior that creates the DI-015 accounting contradiction (ADV-ZWGF-P02-CRIT-001). Specified-but-inconsistent. |
| ADV-ZWGF-P01-MED-002 | MEDIUM | RESOLVED | DEC-008 (edge-cases.md:26) + canonical table (entities.md:43-56) + DI-008 define transport-aware inference: default-port match -> PortHeuristic; mismatch/non-default -> Unknown. |
| ADV-ZWGF-P01-MED-003 | MEDIUM | RESOLVED | DEC-014 (edge-cases.md:32): zero-flow is a valid run, all-zero tallies, exit 0. |
| ADV-ZWGF-P01-MED-004 | MEDIUM | RESOLVED | DI-013 + DI-015 + entities.md:30 consistently track skipped separately and exclude from total_flows. |
| ADV-ZWGF-P01-MED-005 | MEDIUM | RESOLVED | DEC-018 (zero-member zone -> warning) and DEC-019 (zero-conduit -> strict deny-all) both specified. |
| ADV-ZWGF-P01-LOW-001 | LOW | RESOLVED | DEC-012 (edge-cases.md:30): family-aware longest-prefix; AssetMatcher /32 or /128. |
| ADV-ZWGF-P01-LOW-002 | LOW | PARTIALLY_RESOLVED | DEC-013 (edge-cases.md:31) addresses resolution, but multicast/broadcast destination zone-resolution remains problematic (see ADV-ZWGF-P02-MED-004). |
| ADV-ZWGF-P01-LOW-003 | LOW | RESOLVED | DEC-011 (edge-cases.md:29): missing/zero port/proto -> Unknown service, still resolve+classify. |
| ADV-ZWGF-P01-LOW-004 | LOW | PARTIALLY_RESOLVED | events.md:38 (ST-8) gives 0/1/2 exit codes and FM-001/003/006 use exit 2, but exact codes deferred to PRD and the clean-run-with-skipped-flows case is undefined (see ADV-ZWGF-P02-LOW-002). |

**Part A summary:** 8 RESOLVED, 4 PARTIALLY_RESOLVED, 0 UNRESOLVED. Most serious residual: the verdict-totality/accounting contradiction from the IDMZ-bypass-plus-conduit-allow rule.

## Part B — New Findings

### CRITICAL

#### ADV-ZWGF-P02-CRIT-001: DI-015 "exactly one VerdictKind per flow" contradicts DI-006's "IdmzBypass in addition to a conduit verdict"; accounting identity ambiguous/unsound
- **Severity:** CRITICAL
- **Category:** contradictions
- **Location:** invariants.md:36 (DI-015), invariants.md:27 (DI-006), entities.md:28 (Verdict row), entities.md:29 (Violation row), edge-cases.md:23 (DEC-005)
- **Description:** Three statements cannot all be true: (1) entities.md:28 "VerdictKind in {IntraZone, Allowed(conduit_id), Violation(Violation)} — exactly one per resolved flow (DI-015)"; (2) DI-015 "Every resolved flow receives exactly one VerdictKind. total_flows == allowed + intra_zone + (distinct violating flows)"; (3) DI-006/DEC-005/entities.md:29 a flow allowed by a direct <=L3<->>=L4 conduit is still flagged IdmzBypass — simultaneously Allowed and a Violation. For a both-conduit-allowed-and-IDMZ-bypass flow, which VerdictKind applies? If Allowed: the IdmzBypass is not in violations -> silent under-reporting of the headline check (violates the brief's "zero false silence"). If Violation: the allowed tally is wrong. The accounting identity has no defined treatment for a flow in both sets, making the crown-jewel IDMZ-bypass verdict (product-brief.md:56-57) non-deterministic in its tally and potentially silently dropped.
- **Evidence:** entities.md:28 "exactly one per resolved flow (DI-015)"; DI-006 "a flow nominally allowed by a direct <=L3<->>=L4 conduit is still flagged"; DEC-005 "Still flagged IdmzBypass (DI-006 overrides conduit allow)"; entities.md:29 "a flow may carry both a conduit-violation and an IdmzBypass finding."
- **Proposed Fix:** Decouple the IDMZ check from the conduit verdict. Make Verdict.kind the conduit/intra-zone classification (one of IntraZone/Allowed/NoMatchingConduit/WrongDirection); model IdmzBypass as an independent additive finding attached to the flow, not a member of the mutually-exclusive VerdictKind. Redefine the accounting identity over the conduit verdict only (total_flows == allowed + intra_zone + conduit_violations) and tally idmz_bypass_count separately (may overlap allowed). Update DI-015, the Verdict/Violation entity rows, and ConformanceResult fields (entities.md:30) to add idmz_bypasses distinct from violations; state explicitly a flow can appear in both allowed and idmz_bypasses.

### HIGH

#### ADV-ZWGF-P02-HIGH-001: ConformanceResult accounting cannot represent IDMZ-bypass flows; "distinct violating flows" is not computable from declared structures
- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** entities.md:30 (ConformanceResult), entities.md:29 (Violation), invariants.md:36 (DI-015)
- **Description:** Downstream of CRIT-001. ConformanceResult has allowed: usize, intra_zone: usize, violations: Vec<Violation> and asserts total_flows == allowed + intra_zone + (count of distinct violating flows). The Violation note (entities.md:29) says one flow can produce two Violation entries (e.g. NoMatchingConduit + IdmzBypass), so violations.len() != the count term in the identity. There is no field/rule to derive "distinct violating flows" from violations: Vec<Violation> without de-duplicating by flow identity — and Flow has no id (only an optional raw_uid in research, absent from entities.md Flow). "distinct violating flows" is thus not computable from the declared data structures.
- **Evidence:** entities.md:30 field list; entities.md:29 "a flow may carry both"; DI-015 identity wording "distinct violating flows"; Flow entity (entities.md:25) has no identity field.
- **Proposed Fix:** (a) Add a stable per-flow identity (the DI-009 total-order key, or an explicit flow_index) to Flow/Verdict/Violation. (b) Store violations grouped per flow (Vec<(FlowId, Vec<ViolationKind>)>) or add an explicit distinct_violating_flows: usize field. (c) Make the DI-015 property test reference that field, not violations.len().

#### ADV-ZWGF-P02-HIGH-002: Purdue ordering / "<=L3" / ">=L4" semantics relative to IDMZ (3.5) and the L5/EXTERNAL collision under-specified; DI-006 evaluable two ways
- **Severity:** HIGH
- **Category:** ambiguous-language
- **Location:** entities.md:35 (PurdueLevel), invariants.md:27 (DI-006), ubiquitous-language.md:26-27
- **Description:** DI-006 keys on "managed zone <=L3" and ">=L4" with IDMZ (3.5) strictly between. (1) IDMZ is neither <=L3 nor >=L4; non-bypass cases (L2<->IDMZ, IDMZ<->L4, IDMZ<->IDMZ, L3<->IDMZ) are only inferred, never stated. (2) EXTERNAL fixed at L5 (>=L4) is excluded by name, but the spec never says whether EXTERNAL's L5 is comparable/equal to a declared managed L5 zone; a managed L1<->declared-L5 flow IS a bypass but no fixture distinguishes "managed L5" from "EXTERNAL L5," inviting conflation.
- **Evidence:** DI-006 "managed (non-EXTERNAL) zone"; entities.md:35 EXTERNAL purdue_level=L5; no truth table enumerating endpoint-level/zone-kind pairs.
- **Proposed Fix:** Add an explicit truth table to DI-006/edge-cases enumerating endpoint-level pairs x {managed/EXTERNAL/IDMZ} -> bypass yes/no (incl. IDMZ<->L1, IDMZ<->L4, managed-L5<->L1, EXTERNAL<->L1). State IDMZ is excluded from both the <=L3 and >=L4 predicates by definition. Clarify EXTERNAL's L5 is a sentinel and a declared managed L5 zone is a normal managed endpoint for DI-006.

#### ADV-ZWGF-P02-HIGH-003: Equal-length-prefix "tie" rule (DI-010/DEC-002) does not specify cross-family ties, identical-vs-disjoint same-length CIDRs, or host-equals-network/broadcast
- **Severity:** HIGH
- **Category:** missing-edge-cases
- **Location:** invariants.md:25 (DI-004), invariants.md:31 (DI-010), edge-cases.md:20 (DEC-002), entities.md:23 (AssetMatcher)
- **Description:** DI-004/DI-010 make equal-length-prefix ties a fatal policy error and the DI-004 resolver totality is a Kani proof target depending on a total, unambiguous tie definition. Gaps: (3) same prefix length, different IP family can never collide — are they a "tie"? Rule unscoped. (4) two identical CIDRs across zones (tie) vs overlapping-but-disjoint same-length CIDRs (10.0.0.0/24 vs 10.0.1.0/24, must be legal) could be conflated under "any two equal-length prefixes." Also a /32 host equal to another zone's network/broadcast address is unaddressed.
- **Evidence:** DI-010 "equal-length prefix membership ties"; DEC-012 implies family-scoping but DI-010 does not say so.
- **Proposed Fix:** Define "tie" precisely in DI-010: two AssetMatchers, in the same IP family, that match a common address with equal prefix length. State the tie check is per-address-family and per-overlap, not a global equal-length check. Add edge cases for disjoint same-length CIDRs (legal) and host == network/broadcast of another zone's CIDR.

### MEDIUM

#### ADV-ZWGF-P02-MED-001: ICMP / non-port protocols: Proto enum and conduit PortSet matching undefined for portless protocols
- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** entities.md:38 (Proto), entities.md:39 (PortSet), entities.md:24 (Conduit "ports non-empty"), DEC-011, research 3.1
- **Description:** ICMP/Other flows have no transport port. Flow allows missing ports (DEC-011) but Conduit requires ports non-empty (entities.md:24) and conduit matching is on (zone-pair, proto, port, direction). A portless ICMP flow can never match a non-empty-ports conduit — so either ICMP cross-zone flows are always NoMatchingConduit (never stated) or PortSet::Any is meant to cover it (Any-vs-no-port not reconciled). Determinism gap for a very common OT flow type (ICMP liveness across the IDMZ).
- **Evidence:** entities.md:24 "ports non-empty"; entities.md:39 PortSet; research:259 proto includes icmp.
- **Proposed Fix:** State how portless protocols match conduits (e.g. a conduit with ports: Any matches a flow with no port; a conduit with explicit ports never matches a portless flow). Add an edge case for an ICMP flow crossing a zone boundary. Reconcile "ports non-empty" with PortSet::Any.

#### ADV-ZWGF-P02-MED-002: direction value-name drift (model Forward/Bidirectional vs YAML unidirectional/bidirectional); YAML parse contract undefined
- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** entities.md:37 and ubiquitous-language.md:44 (Forward | Bidirectional), research 2.5 YAML (direction: unidirectional), CAP-001
- **Description:** Internal Direction enum is Forward | Bidirectional; the only concrete policy syntax (research:229) uses direction: unidirectional. CAP-001 promises YAML->Policy parsing but no document states the YAML token set or mapping. The policy file is the primary human-authored git-reviewed artifact (core differentiator); undefined keyword set is a correctness/UX gap and fuzz-target ambiguity.
- **Evidence:** entities.md:37 enum; research:229 YAML token.
- **Proposed Fix:** Pin the canonical YAML direction token in the domain/ubiquitous-language doc (or defer to PRD but pin the model<->YAML mapping). Decide whether the keyword is forward/unidirectional and document any alias.

#### ADV-ZWGF-P02-MED-003: Service inference table omits multi-transport OT services (DNP3 UDP, EtherNet/IP UDP); "mark Unknown on doubt" for S7comm/102 is untestable as written
- **Severity:** MEDIUM
- **Category:** completeness
- **Location:** entities.md:43-56 (canonical Service/Port table), entities.md:41 (Service enum), research 3.4
- **Description:** The MVP table pins single (port,transport) defaults but research 3.4 documents DNP3 as TCP-also-UDP and EtherNet/IP explicit as TCP&UDP. Per DEC-008 a DNP3-over-UDP:20000 flow classifies Unknown though legitimately DNP3 — an unacknowledged fidelity regression. The S7comm/102 note says "mark Unknown on doubt" but there is no MMS in the Service enum and no flow signal to detect the collision, so the rule always resolves to PortHeuristic=S7comm (untestable).
- **Evidence:** entities.md:49-56 single-transport pins; research 3.4 multi-transport notes.
- **Proposed Fix:** Either (a) document MVP intentionally pins single-transport defaults and multi-transport variants resolve to Unknown (deliberate, tested), or (b) allow multiple (port,transport) pairs per service. Replace "mark Unknown on doubt" for S7comm/102 with a concrete testable rule.

#### ADV-ZWGF-P02-MED-004: Multicast/broadcast destination zone-resolution unspecified; DEC-013 "resolve normally (likely EXTERNAL)" produces systematic false positives
- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** edge-cases.md:31 (DEC-013), entities.md:27 (ResolvedEndpoint), invariants.md:24 (DI-003), research 4
- **Description:** DI-003 requires every endpoint to resolve to exactly one zone; DEC-013 says multicast/broadcast "resolve normally (likely EXTERNAL)." Research (3.4, 4) calls out EtherNet/IP implicit I/O (UDP 2222) and BACnet as multicast-destination protocols — the dst_ip is a multicast group, not a plant asset. Resolving to EXTERNAL (L5) makes a managed L1 producer -> 239.x look like L1<->EXTERNAL (NoMatchingConduit), generating systematic false positives on the cyclic I/O traffic that dominates OT networks. Acknowledged that multicast exists but the EXTERNAL outcome is likely wrong and not flagged as a limitation the way NAT/VLAN is (ASM-008).
- **Evidence:** DEC-013 wording; research 3.4 / 4 multicast pitfall table.
- **Proposed Fix:** Add an explicit decision: classify multicast/broadcast destinations as a distinct verdict/skip category (not NoMatchingConduit), or document as a known v1 limitation/false-positive source analogous to ASM-008 with a risk entry. Do not leave it as "resolve normally."

#### ADV-ZWGF-P02-MED-005: policy_digest has no defined computation/input domain/algorithm, yet DI-009 promises byte-stable output
- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** entities.md:30 (policy_digest: String), invariants.md:30 (DI-009), events.md:28 (ST-7)
- **Description:** ConformanceResult.policy_digest appears in output and ST-7 but no document defines what it digests (raw YAML bytes vs canonical model), the algorithm, or the encoding. DI-009 requires byte-identical output for identical inputs; hashing raw bytes vs canonical model gives observably different behavior for the edit-and-rerun demo (product-brief.md:83). Undefined digest = untestable determinism claim for the report header.
- **Evidence:** entities.md:30 field; DI-009; ST-7.
- **Proposed Fix:** Define policy_digest precisely: input (canonical Policy model vs raw bytes), algorithm (e.g. SHA-256), encoding (hex/base64); state whether comments/formatting affect it; add an invariant/test for two byte-different but model-identical policies.

#### ADV-ZWGF-P02-MED-006: conn_state-graded severity (research decision #5) silently dropped from the domain spec; blocked attempts will be flagged as full violations
- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** research 3.1 / 4 / decision #5 (research:433); absent from capabilities.md, entities.md, invariants.md
- **Description:** Research repeatedly recommends grading violations by conn_state (S0/REJ attempt across a non-conduit = evidence the control worked; SF = confirmed violation) and lists "treating blocked attempts as violations" as a pitfall (research:411). The Flow entity (entities.md:25) drops conn_state and no Violation/Verdict field grades severity, so the MVP will flag blocked connection attempts as full violations — a major false-positive source the research warned against — with no acknowledgment.
- **Evidence:** research:299-302, research:411, research:433; entities.md Flow has no conn_state.
- **Proposed Fix:** Decide explicitly. If descoped: add to product-brief Out-of-Scope plus an ASM/risk ("MVP treats S0/REJ and SF flows identically; may over-report blocked attempts"). If in scope: add a conn_state/severity field to Flow and Violation plus a capability/invariant.

### LOW

#### ADV-ZWGF-P02-LOW-001: service_source enum name drift (research SensorDpi vs spec DpiConfirmed)
- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** entities.md:40 (DpiConfirmed | PortHeuristic | Unknown), research 3.3 (SensorDpi | PortHeuristic | Unknown)
- **Description:** Provenance enum variant is DpiConfirmed in spec, SensorDpi in research. Spec is source of truth, so not a defect in the spec, but the divergence confuses implementers cross-referencing research. Traceability hygiene only.
- **Evidence:** entities.md:40; research 3.3.
- **Proposed Fix:** Note in entities.md that DpiConfirmed supersedes the research SensorDpi, or accept research is non-normative. No functional change.

#### ADV-ZWGF-P02-LOW-002: Exit-code undefined for a clean run that skipped flows (partial ingest)
- **Severity:** LOW
- **Category:** completeness
- **Location:** events.md:38 (ST-8), invariants.md:34 (DI-013), DEC-010/DEC-014, FM-004
- **Description:** ST-8 maps 0=conformant, 1=violations, 2=policy/usage. DI-013/FM-004 skip+count malformed lines and never abort. The mapping does not say what a conformant run with some skipped flows returns. A CI gate trusting exit 0 could pass when most flows failed to parse. The empty-but-valid case (DEC-014) is handled; non-empty partially-parsed zero-violations is not.
- **Evidence:** events.md:38; DI-013; DEC-014.
- **Proposed Fix:** Define behavior: exit 0 with skipped>0 plus a mandatory warning, or a distinct exit/flag (--fail-on-skipped) for strict CI. At minimum state the chosen behavior so determinism/CI claims are testable.

#### ADV-ZWGF-P02-LOW-003: ResolvedEndpoint ImplicitExternal vs EXTERNAL's role in DI-004 longest-prefix unreconciled (does EXTERNAL match at prefix length 0?)
- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** entities.md:27 (ResolvedEndpoint MatchKind: Explicit{prefix_len} | ImplicitExternal), invariants.md:24-25 (DI-003/DI-004)
- **Description:** Resolution is longest-prefix; unmatched -> EXTERNAL. Two mental models: EXTERNAL as a 0.0.0.0/0 default matcher (part of longest-prefix) vs EXTERNAL as a membership-less special-case fallback outside longest-prefix. The spec uses the membership-less model but never reconciles it; DI-004's totality (Kani target) depends on which is canonical.
- **Evidence:** entities.md:27 MatchKind; DI-004/DI-005.
- **Proposed Fix:** State the canonical model in DI-004/DI-005: EXTERNAL is the result when the matcher set yields no match (not a 0/0 matcher), OR an implicit lowest-priority 0/0 matcher. Pick one so the Kani proof target is unambiguous.

#### ADV-ZWGF-P02-LOW-004: Flow.ts name drift (ts vs start_ts) and undefined precision/timezone affect the DI-009 order key
- **Severity:** LOW
- **Category:** completeness
- **Location:** ubiquitous-language.md:30 and entities.md:25 (ts), invariants.md:30 (DI-009 key uses ts), research 3.3 (start_ts: OffsetDateTime)
- **Description:** DI-009's order key starts with ts; Flow field is ts; research proposed start_ts: OffsetDateTime. (1) name drift; (2) ts type/precision/timezone never pinned. Zeek ts is fractional epoch seconds; ms-truncation could collapse sub-ms differences. The input-index tiebreaker preserves uniqueness (hence LOW) but cross-reparse determinism depends on fixed precision.
- **Evidence:** entities.md:25 ts; DI-009; research 3.3 start_ts.
- **Proposed Fix:** Pin Flow.ts type and precision (e.g. nanosecond OffsetDateTime normalized UTC) in entities.md; reconcile ts/start_ts; confirm DI-009 uses full-precision value.

#### ADV-ZWGF-P02-LOW-005: events.md purity claim ("ST-4-ST-7 pure") in tension with FM-007 streaming (ST-3 I/O-backed lazy iterator)
- **Severity:** LOW
- **Category:** purity-boundary-violations
- **Location:** events.md:35-36, FM-007
- **Description:** Stage Notes say ST-1/ST-3 do I/O, ST-2/ST-4-ST-7 pure. ST-3 yields a stream of raw records and ST-4 pulls per record; FM-007 mandates streaming (no materializing), so ST-4 reads from an I/O-backed lazy iterator. The clean split is real only as a testing contract (ST-4+ pure given an in-memory record), which should be stated rather than implied as a property of the running pipeline.
- **Evidence:** events.md:35-36; FM-007 streaming mandate.
- **Proposed Fix:** Rephrase: "ST-4-ST-7 are pure functions of their in-memory inputs (a raw record / a Flow); streaming I/O lives in the ST-3 iterator; tests exercise ST-4+ with in-memory records."

#### ADV-ZWGF-P02-LOW-006: DEC-018 mandates a zero-member-zone warning but warnings have no defined channel, determinism, or exit-code effect
- **Severity:** LOW
- **Category:** completeness
- **Location:** edge-cases.md:36 (DEC-018); absence of any warning model in events.md/failure-modes.md
- **Description:** DEC-018 requires a load-time warning (the only warning concept in the spec). No definition of channel (stderr? JSON report?), whether warnings are part of DI-009 byte-stable output, or exit-code effect. ASM-001 and R-003 also "degrade with a warning." Undefined warning channel risks breaking byte-identical-output tests that capture stderr.
- **Evidence:** DEC-018; ASM-001; R-003; no warning model in events.md.
- **Proposed Fix:** Add a one-line warning model: channel (stderr and/or warnings: Vec<String> in ConformanceResult), determinism (ordered + stable, part of DI-009), exit-code effect (warnings do not change exit code). Reference from DEC-018, ASM-001, R-003.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 1 |
| HIGH | 3 |
| MEDIUM | 6 |
| LOW | 6 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision (PRD authoring remains blocked)

## Trajectory Note (Monotonicity)

Pass 1 = 14 findings; Pass 2 = 16 new findings. Finding count INCREASED, which the adversarial-review protocol flags as a trajectory regression to investigate before continuing. Root-cause assessment: this is a PERIMETER-EXPANSION regression, not a fix-induced-defect regression. Pass-1 fixes verified RESOLVED (8) or PARTIALLY_RESOLVED (4) with zero UNRESOLVED — no Pass-1 fix introduced a new defect. The new findings cluster in two areas the Pass-1 pass did not reach: (a) the verdict-cardinality/accounting model (CRIT-001, HIGH-001) exposed precisely BECAUSE the v1.1 revision added DI-015 verdict-totality and the IDMZ-in-addition-to-conduit rule (DEC-005/DI-006), and (b) OT-traffic-shape realities documented in research but not propagated to the L2 spec (multicast, ICMP, conn_state grading, multi-transport services). The new DI-015/accounting scope was added in v1.1 without pre-validation against an invariant list, which is the documented cause class. Convergence clock has NOT started; minimum 3 consecutive clean passes still required after these are fixed.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings** | 16 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | 2.5 (between MED and HIGH) |
| **Trajectory** | 14 -> 16 |
| **Verdict** | FINDINGS_REMAIN |

## Process Observation

[process-gap] Research document makes two normative-strength recommendations (conn_state severity grading, decision #5; multicast endpoint handling, section 4 pitfall) silently dropped during L2 authoring with no Out-of-Scope or ASM record. No traceability check ensures each research "Recommended Technical Decision" lands as a CAP/DI/ASM or an explicit Out-of-Scope entry. Recommend a research-decision -> domain-spec coverage audit step.
