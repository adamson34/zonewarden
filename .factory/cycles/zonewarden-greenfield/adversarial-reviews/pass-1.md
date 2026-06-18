---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-06-17T00:00:00
phase: 1d
inputs: [product-brief.md, "domain-spec/"]
input-hash: "[live-state]"
traces_to: domain-spec/L2-INDEX.md
pass: 1
previous_review: null
---

# Adversarial Review: ZoneWarden (Pass 1)

> **SCOPE:** specs --scope=full, limited to product-brief.md + domain-spec/ (PRD/architecture/BC/VP not yet authored — mid-Phase-1 review). This was a comprehensive review of the existing corpus.

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix identifying adversarial findings
- Cycle prefix: `ZWGF` (derived from `zonewarden-greenfield`)
- Pass: `P01`
- Severity: `CRIT`, `HIGH`, `MED`, `LOW`
- Sequence: three-digit, within-severity ordering

Examples: `ADV-ZWGF-P01-CRIT-001`, `ADV-ZWGF-P01-HIGH-002`

## Part A — Fix Verification

_Pass 1 — no prior pass to verify._

## Part B — New Findings

### CRITICAL

#### ADV-ZWGF-P01-CRIT-001: SL-T data model contradiction across three documents

- **Severity:** CRITICAL
- **Category:** contradictions
- **Location:** entities.md:22, ubiquitous-language.md:28, product-brief.md:94, domain-ot-segmentation-validation-2026-06-17.md:216,407,423
- **Description:** `entities.md:22` declares `sl_t: SlTarget`; `ubiquitous-language.md:28` defines SL-T as a 7-element vector across IEC 62443 Foundational Requirements; research (`domain-ot-segmentation-validation-2026-06-17.md:216,407,423`) recommends modeling SL-T as an optional scalar (0-4) per zone/conduit with optional full vector as metadata; brief (`product-brief.md:94`) lists bare `sl_t`. Glossary mandates 7-vector, research mandates scalar, entity names undefined `SlTarget` type — cannot all be true. Will produce contradictory BCs and an unauthorable YAML schema.
- **Evidence:**
  - `entities.md:22`: `sl_t: SlTarget`
  - `ubiquitous-language.md:28`: SL-T defined as 7-element vector across IEC 62443 Foundational Requirements
  - `product-brief.md:94`: bare `sl_t` field listed without type
  - `domain-ot-segmentation-validation-2026-06-17.md:216,407,423`: recommends optional scalar 0-4 per zone/conduit + optional full vector as metadata
- **Proposed Fix:** Pick one representation (research's optional scalar 0-4 + optional vector metadata is most defensible for FR5-only MVP), update `ubiquitous-language.md:28`, and define `SlTarget` concretely in `entities.md` Value Objects.

---

### HIGH

#### ADV-ZWGF-P01-HIGH-001: `AmbiguousMembership` ViolationKind is unreachable and contradicts the policy-validity model

- **Severity:** HIGH
- **Category:** contradictions
- **Location:** entities.md:29, ubiquitous-language.md:41, invariants.md:31 (DI-010), invariants.md:25 (DI-004), edge-cases.md:20 (DEC-002), capabilities.md:23 (CAP-002)
- **Description:** `entities.md:29` and `ubiquitous-language.md:41` list `AmbiguousMembership` as a per-flow `ViolationKind`, but equal-length prefix ties are a policy load-time error that rejects the whole policy (DI-010, DI-004, DEC-002, CAP-002). If policy is rejected at load, no `Flow` is classified, so no flow-level `AmbiguousMembership` can ever be emitted. No stage in `events.md` (ST-1..ST-8) produces it. Dead enum variant contradicting DI-011 "no partial state."
- **Evidence:**
  - `entities.md:29`: `AmbiguousMembership` listed as a `ViolationKind` variant
  - `ubiquitous-language.md:41`: same variant restated
  - `invariants.md:31 (DI-010)`, `invariants.md:25 (DI-004)`: equal-prefix ties cause policy load failure
  - `edge-cases.md:20 (DEC-002)`, `capabilities.md:23 (CAP-002)`: tie at load time rejects whole policy
- **Proposed Fix:** Remove `AmbiguousMembership` from `ViolationKind`, OR reconcile DI-004/DI-010/DEC-002 to make a runtime path reachable. Pick one model.

#### ADV-ZWGF-P01-HIGH-002: EXTERNAL zone has no defined Purdue level, but IDMZ no-bypass check (DI-006) requires every endpoint's level

- **Severity:** HIGH
- **Category:** missing-edge-cases
- **Location:** invariants.md:27 (DI-006), capabilities.md:29 (CAP-008), entities.md:22, invariants.md:26 (DI-005), ubiquitous-language.md:39
- **Description:** DI-006 (`invariants.md:27`) flags flows connecting a zone <=L3 with a zone >=L4 without an IDMZ endpoint; CAP-008 (`capabilities.md:29`) restates as "one <=L3, one >=L4." EXTERNAL zone (`entities.md:22`, DI-005 `invariants.md:26`, `ubiquitous-language.md:39`) is "conceptually L5/Internet" but membership-less with no declared `purdue_level` value, yet the `Zone` entity requires `purdue_level: PurdueLevel`. If EXTERNAL=L5, every flow from internal L0-L3 to an unmatched/external IP becomes an IDMZ-bypass violation including legitimate egress. Headline check undefined for the most common real case.
- **Evidence:**
  - `invariants.md:27 (DI-006)`: requires zone level comparison to evaluate bypass
  - `entities.md:22`: EXTERNAL zone declared without `purdue_level` assignment
  - `ubiquitous-language.md:39`: EXTERNAL described as "conceptually L5/Internet" — not normatively assigned
- **Proposed Fix:** Explicitly assign EXTERNAL a `purdue_level` (L5), add an edge case/invariant for how DI-006 treats EXTERNAL-endpoint flows, add a DEC for internal asset -> EXTERNAL IP expected verdict.

#### ADV-ZWGF-P01-HIGH-003: IDMZ no-bypass (DI-006) cannot be evaluated from a single flow record; "without transiting the IDMZ" is unobservable per-flow

- **Severity:** HIGH
- **Category:** ambiguous-language
- **Location:** invariants.md:27 (DI-006), capabilities.md:29 (CAP-008), entities.md:25, events.md:27 (ST-6), edge-cases.md:23 (DEC-005)
- **Description:** DI-006 (`invariants.md:27`) / CAP-008 (`capabilities.md:29`). A single `Flow` (`entities.md:25`) has exactly two endpoints. "Transited the IDMZ" is a path property across multiple flows, not one connection. With per-flow data the check can only mean "neither endpoint of this flow is in the IDMZ" (as ST-6 `events.md:27` and DEC-005 `edge-cases.md:23` imply). "transiting"/"bypass" overstates what the data supports.
- **Evidence:**
  - `invariants.md:27 (DI-006)`: uses "without transiting the IDMZ" — implies multi-hop path check
  - `entities.md:25`: `Flow` has exactly two endpoints (src, dst) — single hop only
  - `events.md:27 (ST-6)`, `edge-cases.md:23 (DEC-005)`: imply single-endpoint membership check in practice
- **Proposed Fix:** Redefine DI-006 in single-flow terms: a single flow is `IdmzBypass` iff one endpoint resolves to <=L3 and the other >=L4 (neither endpoint in IDMZ). Drop "transiting" or scope to single-hop; state multi-hop transit verification is out of scope for flow-based MVP.

#### ADV-ZWGF-P01-HIGH-004: CAP-003's cited resilience invariant ("DI: resilience") does not exist

- **Severity:** HIGH
- **Category:** spec-gap
- **Location:** capabilities.md:24 (CAP-003), invariants.md (DI-001..DI-012), failure-modes.md:26 (FM-004), edge-cases.md:28 (DEC-010)
- **Description:** CAP-003 (`capabilities.md:24`) Business Rule cites "(DI: resilience)" but no flow-ingest resilience DI exists in `invariants.md` (DI-001..DI-012). Rule is load-bearing (restated FM-004 `failure-modes.md:26`, DEC-010 `edge-cases.md:28`) but has no invariant ID to anchor a BC/property test, unlike every other CAP.
- **Evidence:**
  - `capabilities.md:24 (CAP-003)`: Business Rule cites "(DI: resilience)"
  - `invariants.md`: DI-001 through DI-012 listed — no resilience invariant present
  - `failure-modes.md:26 (FM-004)`, `edge-cases.md:28 (DEC-010)`: both reference the resilience rule but cite no DI
- **Proposed Fix:** Add DI-013 "Flow-ingest resilience: malformed flow record skipped+counted, never aborts; malformed policy always aborts" and update CAP-003 to cite it; captures the fail-loud-on-policy/degrade-on-flows asymmetry (`failure-modes.md:18`).

---

### MEDIUM

#### ADV-ZWGF-P01-MED-001: Conflicting-conduit-rules edge case entirely missing

- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** edge-cases.md (DEC-001, DEC-002, DEC-009), capabilities.md:27 (CAP-006), invariants.md:31 (DI-010)
- **Description:** `edge-cases.md` covers overlapping CIDRs (DEC-001), equal-prefix ties (DEC-002), port ranges (DEC-009) but never multiple conduits matching the same flow, or two conduits between the same zone-pair with overlapping/contradictory ports or directions (Forward vs Bidirectional A<->B; port 502 vs range 500-510). CAP-006 (`capabilities.md:27`) implies first-match or any-match but never states which; DI-010 (`invariants.md:31`) silent on conduit overlap/duplication. Conduit analogue of the well-handled zone-overlap case.
- **Evidence:**
  - `capabilities.md:27 (CAP-006)`: conduit matching described without specifying first-match vs any-match semantics
  - `invariants.md:31 (DI-010)`: zone-overlap handling defined; conduit-overlap handling absent
  - `edge-cases.md`: DEC-001, DEC-002, DEC-009 cover zone/prefix/port edge cases but no conduit overlap DEC exists
- **Proposed Fix:** Add a DEC and DI clause defining conduit-match semantics (any-match-allows vs first-match) and whether duplicate/overlapping conduits are a load-time error or silently unioned.

#### ADV-ZWGF-P01-MED-002: Asymmetric/reverse-flow handling under deny-by-default under-specified for response traffic

- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** invariants.md:28 (DI-007), edge-cases.md:24 (DEC-006), assumptions.md:19 (ASM-001), risks.md:21 (R-003)
- **Description:** DI-007 (`invariants.md:28`) + DEC-006 (`edge-cases.md:24`): a Forward conduit A->B permits only initiator-in-A; B->A-initiated is `WrongDirection`. Relies entirely on ASM-001 (`assumptions.md:19`, Medium confidence, unvalidated) that src=initiator. R-003 (`risks.md:21`) acknowledges the risk. Gap: a Zeek `conn.log` line already represents a full bidirectional connection (orig/resp byte counts); treating responder traffic as a distinct B->A flow generates false `WrongDirection` violations. Spec never states whether one `conn.log` line = one `Flow`. Most likely source of false positives in MVP.
- **Evidence:**
  - `assumptions.md:19 (ASM-001)`: "src field = initiator" — Medium confidence, unvalidated
  - `invariants.md:28 (DI-007)`: `WrongDirection` violation depends on initiator identity
  - `risks.md:21 (R-003)`: acknowledges false-positive risk from this assumption
  - `edge-cases.md:24 (DEC-006)`: no clarification of conn.log one-record-per-connection semantics
- **Proposed Fix:** State one `conn.log` connection maps to exactly one `Flow` keyed on originator, responder/return traffic implicit; promote ASM-001 validation to P0 gating task.

#### ADV-ZWGF-P01-MED-003: Service enum / known-service set and OT port->service table referenced but never enumerated in domain spec

- **Severity:** MEDIUM
- **Category:** completeness
- **Location:** entities.md:26, product-brief.md:101-102, edge-cases.md:26 (DEC-008), assumptions.md:21 (ASM-003)
- **Description:** `entities.md:26` defines `Service` as "enum of known OT/IT services + Other(String)" but never lists them. Brief (`product-brief.md:101-102`) names six (Modbus/502, DNP3/20000, EtherNet-IP/44818, S7comm/102, BACnet/47808, OPC UA/4840). DEC-008 (`edge-cases.md:26`) asserts "BACnet expects UDP." No table mapping service<->port<->transport inside spec shards. ASM-003 (`assumptions.md:21`) defers to research.
- **Evidence:**
  - `entities.md:26`: `Service` declared as enum, no members listed
  - `product-brief.md:101-102`: six services named with ports
  - `edge-cases.md:26 (DEC-008)`: "BACnet expects UDP" — asserts transport without a canonical table to verify against
  - `assumptions.md:21 (ASM-003)`: defers service/port enumeration to research
- **Proposed Fix:** Add the canonical service/port/transport table to `entities.md` so DEC-007/DEC-008 and CAP-004 are testable against a fixed list.

#### ADV-ZWGF-P01-MED-004: Determinism invariant (DI-009) names a sort key that omits proto and may not be a total order

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** invariants.md:30 (DI-009), capabilities.md:30-31 (CAP-009, CAP-010)
- **Description:** DI-009 (`invariants.md:30`): "sorted by ts, then src, dst, port." Two distinct flows can share identical (ts, src_ip, dst_ip, dst_port) but differ in proto (TCP vs UDP) or src_port — common in real captures. Proposed key is not a guaranteed total order, so "byte-identical output" (a Kani-proof target) is unprovable as stated. CAP-009/CAP-010 (`capabilities.md:30-31`) lean on DI-009.
- **Evidence:**
  - `invariants.md:30 (DI-009)`: sort key is `(ts, src, dst, port)` — omits proto and src_port
  - `capabilities.md:30-31 (CAP-009, CAP-010)`: determinism guarantees depend on this invariant
- **Proposed Fix:** Specify a complete tiebreaker key (include proto, src_port, final stable tiebreak) that is a total order over `Flow`.

#### ADV-ZWGF-P01-MED-005: Empty-zone and zero-member-policy cases unspecified

- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** edge-cases.md, invariants.md:31 (DI-010)
- **Description:** `edge-cases.md` has no entry for a zone with zero members, nor a policy with zones but zero conduits, nor a conduit referencing EXTERNAL in `from`. DI-010 (`invariants.md:31`) silent on whether a memberless non-EXTERNAL zone is legal (dead policy — warning or error?). Zero-conduit policy = everything cross-zone is a violation (maybe intended strict deny-all) but should be explicit.
- **Evidence:**
  - `edge-cases.md`: no DEC covering memberless zone, zero-conduit policy, or conduit with EXTERNAL as `from`
  - `invariants.md:31 (DI-010)`: defines zone invariants but omits zero-member case
- **Proposed Fix:** Add DECs for (a) memberless declared zone, (b) zero-conduit policy, (c) conduit with EXTERNAL as `from`, with expected behavior (error/warning/valid) each.

---

### LOW

#### ADV-ZWGF-P01-LOW-001: VLAN-vs-subnet / NAT zone-identity limitation acknowledged nowhere despite being a core OT reality

- **Severity:** LOW
- **Category:** completeness
- **Location:** assumptions.md:20 (ASM-002), product-brief.md:124, risks.md:20 (R-002)
- **Description:** ASM-002 (`assumptions.md:20`) assumes IP/CIDR membership sufficient. Brief out-of-scopes only L2-only protocols (`product-brief.md:124`). Neither addresses NAT (observed IP is translation address, silently resolves to wrong zone) nor VLAN-based zoning where two zones share a subnet. Causes silent mis-resolution — the "silent allow" class R-002 (`risks.md:20`) claims to defend, but EXTERNAL/longest-prefix mitigation does not catch a wrong-but-valid zone match.
- **Evidence:**
  - `assumptions.md:20 (ASM-002)`: IP/CIDR membership assumed sufficient — no NAT/VLAN carve-out
  - `product-brief.md:124`: only L2-only protocols listed as out-of-scope; NAT not mentioned
  - `risks.md:20 (R-002)`: "silent allow" risk cited but VLAN/NAT mis-resolution not listed as a sub-case
- **Proposed Fix:** Add an assumption/out-of-scope note that NAT'd flows and VLAN-only zone identity are not modeled in v1 and may mis-resolve.

#### ADV-ZWGF-P01-LOW-002: `external_endpoints` field type/meaning undefined; ConformanceResult tallies don't obviously sum

- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** entities.md:30, risks.md:20 (R-002)
- **Description:** `entities.md:30` `ConformanceResult` lists `total_flows`, `allowed`, `intra_zone`, `violations`, `skipped`, `external_endpoints`, `policy_digest`. `skipped` is `usize`; others untyped. `external_endpoints` ambiguous (count of endpoints? flows touching EXTERNAL? a list?). No accounting invariant (e.g. `total_flows == allowed + intra_zone + violations.len() + skipped`?); skipped flows presumably not in `total_flows`. R-002's "every flow gets exactly one verdict" not anchored to the result struct.
- **Evidence:**
  - `entities.md:30`: `ConformanceResult` fields listed without types or accounting relationship
  - `risks.md:20 (R-002)`: "every flow gets exactly one verdict" asserted but not tied to struct invariant
- **Proposed Fix:** Type each field, define `external_endpoints` precisely, add an accounting invariant tying tallies together as a property test.

#### ADV-ZWGF-P01-LOW-003: Differentiator "Open + declarative + git-versioned" backed partly by a P2 capability

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** differentiators.md:24, capabilities.md:34 (CAP-013), L2-INDEX Priority Distribution
- **Description:** `differentiators.md:24` maps the differentiator to "CAP-001, CAP-013." CAP-013 (Policy DSL) is P2 (`capabilities.md:34`, L2-INDEX Priority Distribution). The git-versioned/diffable claim is real for the YAML (CAP-001) so the differentiator stands, but citing a P2 capability overstates the MVP.
- **Evidence:**
  - `differentiators.md:24`: differentiator cites both CAP-001 and CAP-013
  - `capabilities.md:34 (CAP-013)`: marked P2 priority
- **Proposed Fix:** Drop CAP-013 from this row or mark it roadmap-reinforcing.

#### ADV-ZWGF-P01-LOW-004: ID Registry count: L2-INDEX has no DI-013 slot though CAP-003 needs one

- **Severity:** LOW
- **Category:** completeness
- **Location:** L2-INDEX.md:67-72
- **Description:** `L2-INDEX.md:67-72` declares `DI-NNN: 12`. If ADV-ZWGF-P01-HIGH-004 is accepted (add a resilience DI), this count must increment. ST-N:8 matches `events.md`; DEC-NNN:14 matches; CAP-NNN:14 matches — registry currently internally consistent. Forward-looking note tied to ADV-ZWGF-P01-HIGH-004.
- **Evidence:**
  - `L2-INDEX.md:67-72`: `DI-NNN: 12` — no slot for DI-013
- **Proposed Fix:** When fixing ADV-ZWGF-P01-HIGH-004, update the L2-INDEX DI count to 13.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 1 |
| HIGH     | 4 |
| MEDIUM   | 5 |
| LOW      | 4 |
| **Total** | **14** |

**Overall Assessment:** block — spec contradictions at CRITICAL and multiple HIGH gaps must be resolved before PRD authoring.

**Convergence:** findings remain — iterate.

**Readiness:** requires revision before PRD authoring.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 14 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | 3.0 (HIGH; on a 1=LOW..4=CRIT scale the distribution 1C/4H/5M/4L medians to HIGH) |
| **Trajectory** | 14 |
| **Verdict** | FINDINGS_REMAIN |

<!--
  This section is MANDATORY. The validate-novelty-assessment hook
  blocks adversarial review files missing this section or its required fields.

  Novelty score = new / (new + duplicate). Converged when < 0.15 for 2+ passes.
  See CONVERGENCE.md Dimension 1 for the full quantitative criteria.
-->
