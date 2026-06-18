---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-06-17T00:00:00
phase: "1d"
inputs: [product-brief.md, "domain-spec/"]
traces_to: domain-spec/L2-INDEX.md
pass: 3
previous_review: pass-2.md
---

# Adversarial Review: zonewarden (Pass 3)

> Scope: FULL (product-brief.md + domain-spec/ v1.2). Fresh-context pass — no prior review inherited by the adversary.

## Part A — Fix Verification

### A.1 v1.2 Invariant Verification (10 items + ID-count check)

| # | Invariant | Verdict | Citation | Note |
|---|-----------|---------|----------|------|
| 1 | IdmzBypass additive; VerdictKind closed enum of 5; `flow_index` identity; accounting over conduit verdict only; `idmz_bypasses` separate & may overlap `allowed`; `distinct_violating_flows` de-dups by `flow_index` | VERIFIED | invariants.md:27 (DI-006), :36 (DI-015); entities.md:28 (Verdict), :30 (ConformanceResult) | All five clauses present and mutually consistent. Accounting equation identical in DI-015 and ConformanceResult invariant. |
| 2 | L3/L4 zone predicate truth table; IDMZ excluded both sides; EXTERNAL L5 sentinel distinct from declared managed L5 | VERIFIED | invariants.md:42-56 (truth table); entities.md:35 (PurdueLevel) | Truth-table rows distinguish declared-L5 from sentinel-L5; IDMZ-IDMZ row present. |
| 3 | CIDR tie-breaking per-family/per-common-address/equal-prefix; disjoint same-length legal (DEC-022); host==network/broadcast (DEC-023) | VERIFIED | invariants.md:31 (DI-010); edge-cases.md:41 (DEC-022), :42 (DEC-023) | Tie definition precise; bound to IP family + common address + equal prefix. |
| 4 | Multicast/broadcast dst -> MulticastExempt, not a violation (DI-016) | VERIFIED | invariants.md:37 (DI-016); edge-cases.md:32 (DEC-013b) | See ADV-ZWGF-P03-MED-002 (ordering ambiguity multicast vs intra-zone). |
| 5 | Connection-state severity Attempted vs Established (DI-017) | VERIFIED | invariants.md:38 (DI-017); entities.md:42 (Severity), :41 (ConnState) | Conservative default (absent -> Established) stated in both places. |
| 6 | policy_digest = SHA-256 over canonical model (DI-018) | PARTIAL | invariants.md:39 (DI-018); ubiquitous-language.md:48 | Canonicalization function never defined (field ordering, set ordering, optional-field encoding). Folds into ADV-ZWGF-P03-HIGH-001. |
| 7 | Warning model: stderr AND `warnings`, deterministic, no exit-code effect (DI-019) | VERIFIED | invariants.md:40 (DI-019) | Consistent with DEC-024 and Exit semantics in events.md:40-43. |
| 8 | ICMP/portless: `Proto::Icmp`, `PortSet::Any` matches portless, explicit ports never match portless (DEC-021) | VERIFIED | entities.md:38-39; edge-cases.md:40 (DEC-021) | Stated in three locations consistently. |
| 9 | Single-transport service pins deliberate (ASM-009); canonical service/port/transport table | VERIFIED | entities.md:45-56 (table); assumptions.md:27 (ASM-009) | Table present (6 services). See ADV-ZWGF-P03-MED-001 (transport-mismatch conflict). |
| 10 | Nanosecond timestamps; YAML direction tokens pinned; exit-with-skipped + `--fail-on-skipped` (DEC-024) | VERIFIED | entities.md:43 (Timestamp), :37 (Direction); edge-cases.md:43 (DEC-024); invariants.md:34 (DI-013) | All three present. |
| - | ID inventory: DI=19, DEC=25, ASM=9 | PARTIAL | L2-INDEX.md:66-72 | DI 001-019 = 19 OK; ASM 001-009 = 9 OK; DEC = 001-024 + 013b = 25 rows but max ordinal is 024 (non-contiguous). Folds into ADV-ZWGF-P03-LOW-001. |

Result: 8/10 VERIFIED, 2 PARTIAL. The CRIT-001 decoupling and all four Pass-2 partials' subject areas (accounting, Purdue predicate, CIDR ties, multicast) are confirmed resolved in v1.2 text; the two PARTIAL items are new gaps surfaced at the next level of detail, carried as HIGH-001 and LOW-001 below.

## Part B — New Findings

### CRITICAL

None. The core accounting/enum/IDMZ machinery is sound and internally consistent.

### HIGH

#### ADV-ZWGF-P03-HIGH-001: `policy_digest` canonicalization function is undefined, making DI-018 untestable
- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** invariants.md:39 (DI-018); ubiquitous-language.md:48; entities.md:30
- **Description:** DI-018 requires SHA-256 over the "canonical Policy model" and asserts model-identical policies produce the same digest, but the canonical serialization is never specified: field ordering, whether zones/conduits are sorted (and by what key), how Option fields (sl_t, FR vector) are encoded when absent, how PortSet ranges are normalized (is `{502, 500-510}` equal to `{500-510}`?), and the byte encoding fed to SHA-256. DI-014 explicitly allows duplicate/overlapping conduits, so two "model-identical" policies could differ in conduit multiplicity or ordering with no rule for whether those collapse before hashing.
- **Evidence:** invariants.md:39 states the SHA-256-over-canonical-model rule and flags it as a proof target; ubiquitous-language.md:48 references "canonical Policy model" without defining the canonicalization; DI-014 permits duplicate conduits.
- **Proposed Fix:** Add a DI/entities note pinning canonicalization: explicit field order, deterministic sort keys for zone and conduit collections, normalized PortSet representation (sorted, range-merged), explicit Option encoding, exact byte serialization (e.g. canonical JSON with sorted keys, UTF-8). State whether duplicate conduits are de-duplicated before hashing.

#### ADV-ZWGF-P03-HIGH-002: `external_endpoints` tally has no definition, no governing invariant, and is absent from the accounting equation
- **Severity:** HIGH
- **Category:** ambiguous-language
- **Location:** entities.md:30 (ConformanceResult field `external_endpoints: usize`); ubiquitous-language.md:49
- **Description:** `external_endpoints` appears as a typed counter in the aggregate output, but nothing defines what it counts: (a) flows with >=1 EXTERNAL endpoint, (b) endpoint resolutions landing on EXTERNAL (up to 2x flows), or (c) distinct EXTERNAL IPs. No DI governs it, no edge case exercises it, and it does not appear in any accounting identity. DI-015's totality equation correctly omits it (orthogonal to verdicts), leaving it entirely unspecified.
- **Evidence:** entities.md:30 types the field; ubiquitous-language.md:49 lists it as a tally; no DI references it; DI-015 accounting identity omits it.
- **Proposed Fix:** Add a one-line definition (recommend: "count of flows with >=1 endpoint resolving to EXTERNAL; endpoints are not double-counted") and a DEC exercising it, or remove the field from the MVP ConformanceResult.

### MEDIUM

#### ADV-ZWGF-P03-MED-001: Transport-mismatch service-inference behavior contradicts between DEC-008 and ASM-009/the canonical table
- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** edge-cases.md:26 (DEC-008); entities.md:47 (table preamble); edge-cases.md:25 (DEC-007); assumptions.md:27 (ASM-009)
- **Description:** DEC-008 and the table preamble agree transport mismatch -> Unknown. But DEC-007 says a known service on a non-default port yields "service_source = Unknown/PortHeuristic" (two outcomes via slash), while entities.md:47 says non-default port is unambiguously Unknown. For "Modbus on 1502," DEC-007 permits PortHeuristic but entities.md:47 forbids it.
- **Evidence:** edge-cases.md:25 contains the "Unknown/PortHeuristic" alternative; entities.md:47 states non-default port -> Unknown.
- **Proposed Fix:** Make DEC-007 consistent with entities.md:47 — non-default port -> Unknown (no PortHeuristic option). Remove the `/PortHeuristic` alternative in DEC-007.

#### ADV-ZWGF-P03-MED-002: Verdict precedence between MulticastExempt, IntraZone, and idmz_bypass for a multicast flow is ambiguous
- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** events.md:27 (ST-6 ordering); invariants.md:37 (DI-016); invariants.md:27 (DI-006)
- **Description:** ST-6 lists MulticastExempt first, implying short-circuit, but the precedence is never stated outside that list. DI-006's predicate references managed-zone levels; a MulticastBroadcast destination has no Purdue level, so idmz_bypass should be false — but DI-006 never says so and the truth table (invariants.md:47-56) has no MulticastBroadcast row. Separately, an intra-zone flow with a multicast dst could satisfy both DI-002 (IntraZone) and DI-016 (MulticastExempt) with no stated precedence.
- **Evidence:** events.md:27 ST-6 ordering; invariants.md:37 DI-016 (dst multicast -> MulticastExempt); invariants.md:27 DI-006 predicate; truth table invariants.md:47-56 lacks a MulticastBroadcast row.
- **Proposed Fix:** State explicitly in DI-016 (or DI-015) that verdict precedence is the ST-6 order and MulticastBroadcast destination short-circuits before zone-pair classification (multicast wins over IntraZone). Add to DI-006: "if either endpoint is MulticastBroadcast, idmz_bypass = false."

#### ADV-ZWGF-P03-MED-003: Broadcast detection is undefined for non-trivial broadcast addresses and is family-asymmetric
- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** invariants.md:37 (DI-016 "multicast or broadcast address"); edge-cases.md:42 (DEC-023)
- **Description:** Directed/subnet broadcast (e.g. `10.0.0.255` for a /24) is only a broadcast relative to a subnet prefix length the flow record does not carry; the spec gives no rule for recognizing it. IPv6 has no broadcast, so the broadcast clause is silently v4-only. DEC-023 contemplates a host /32 colliding with "another zone's broadcast," implying broadcast is computed from declared CIDRs, but DI-016 concerns the flow destination and never says broadcast is derived from the matched zone's prefix.
- **Evidence:** invariants.md:37 "multicast or broadcast address"; edge-cases.md:42 references network/broadcast addresses computed from zone CIDRs.
- **Proposed Fix:** Define broadcast detection precisely: limited broadcast 255.255.255.255 and (if supported) directed broadcast against the longest-prefix-matched zone's CIDR; state IPv6 has no broadcast and only multicast (ff00::/8) triggers exemption. Pin the v4 multicast range (224.0.0.0/4).

#### ADV-ZWGF-P03-MED-004: `Other(String)`/`Other(u8)` enum variants leave determinism/ordering and equality undefined for non-ASCII / case variants
- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** entities.md:26 (Service `Other(String)`), :38 (Proto `Other(u8)`), :41 (ConnState `Other(String)`)
- **Description:** Three enums carry free-string/byte variants. DI-009's total-order key (invariants.md:30) sorts by proto, but Service::Other(String) and ConnState::Other(String) are not in the order key yet appear in JSON output. No rule covers string normalization (Unicode NFC vs NFD, case-folding, trimming). Two Zeek logs with `s7comm` vs `S7comm` would produce byte-different JSON for "the same" flow.
- **Evidence:** entities.md:26,38,41 (free-string/byte variants); invariants.md:30 (DI-009 order key omits Service/ConnState strings).
- **Proposed Fix:** State that Other(String) payloads are preserved verbatim (no normalization) and are byte-stable iff the input is; OR pin a normalization. Confirm Other variants do not participate in any equality used for tallying.

#### ADV-ZWGF-P03-MED-005: Integer-overflow / scale bounds on `usize` tallies unspecified despite streaming design for very large inputs
- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** entities.md:30 (tallies `usize`); failure-modes.md:29 (FM-007); product-brief.md:131
- **Description:** FM-007 anticipates very large flow inputs and mandates streaming, but no invariant bounds the tally counters. On a 32-bit target usize is 32 bits; total_flows and per-bucket counts could overflow. The product brief constrains the platform to Linux/macOS cross-platform without pinning 64-bit. No stated max flow count or overflow behavior (saturate/panic/error).
- **Evidence:** entities.md:30 (all tallies usize); failure-modes.md:29 (FM-007 very large input); product-brief.md:131 (platform constraint, no bitness pin).
- **Proposed Fix:** Pin 64-bit usize as a platform constraint, or add a DI/NFR stating overflow behavior and a documented max flow count; note the DI-015 property test must bound input below the overflow threshold.

### LOW

#### ADV-ZWGF-P03-LOW-001: DEC numbering is non-contiguous (`DEC-013b`, max ordinal 024) while the index reports a flat count of 25 [process-gap]
- **Severity:** LOW
- **Category:** completeness
- **Location:** L2-INDEX.md:69 (DEC count 25); edge-cases.md:32 (DEC-013b)
- **Description:** DEC IDs are 001-024 plus inserted 013b = 25 rows. The index Count column reports 25 (matches row count), but max(NNN)=024, so a reader doing arithmetic concludes one is missing and any future "DEC-025" reference silently fails. The lettered-insertion convention is unique in the corpus (DI/ASM/CAP all contiguous) with no governing convention.
- **Evidence:** L2-INDEX.md:69 (count 25); edge-cases.md:32 (DEC-013b); other registries contiguous.
- **Proposed Fix:** Renumber DEC-013b -> DEC-025, or document the `Nb` insertion convention in L2-INDEX.md so ID-resolution tooling treats it as canonical.

#### ADV-ZWGF-P03-LOW-002: `unidirectional` aliases `forward` but no rule for an unknown direction/proto/PortSet token
- **Severity:** LOW
- **Category:** completeness
- **Location:** entities.md:37; ubiquitous-language.md:50; invariants.md:31 (DI-010)
- **Description:** Direction tokens forward/bidirectional are canonical with unidirectional aliasing forward, but there is no behavior for an unrecognized direction token (e.g. reverse, inbound) — policy-validation error or silent ignore? DI-010 enumerates validity checks but does not include valid direction/proto tokens or PortSet syntax. Deny-by-default depends on parse strictness; an unknown token defaulting permissively could silently widen a conduit.
- **Evidence:** entities.md:37 (alias rule); invariants.md:31 (DI-010 validity checks omit token validity).
- **Proposed Fix:** Add to DI-010 (or note in CAP-002) that unrecognized enum tokens (direction, proto, port syntax) are policy-validation errors -> fail-fast exit, not silent defaults.

#### ADV-ZWGF-P03-LOW-003: `flow_index` typed as "ingest position" but ingest order with skipped lines is ambiguous
- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** entities.md:25 (`flow_index: usize` = position in ingest order); invariants.md:34 (DI-013); :30 (DI-009)
- **Description:** flow_index is "position in ingest order" and is the determinism tiebreaker and de-dup key, but DI-013 skips malformed lines. Is flow_index the index among successfully parsed flows (dense 0..n) or the raw line/record position (sparse, with gaps)? Both are deterministic but produce different index values, and any downstream artifact citing flow_index would differ.
- **Evidence:** entities.md:25 (ingest-position definition); invariants.md:34 (DI-013 skips malformed lines).
- **Proposed Fix:** State explicitly whether flow_index counts only successfully-normalized flows (recommended: dense, gap-free) or raw record positions.

#### ADV-ZWGF-P03-LOW-004: Authoritative glossary Flow definition is stale — omits `flow_index` and `conn_state`
- **Severity:** LOW
- **Category:** completeness
- **Location:** product-brief.md:96-97; ubiquitous-language.md:30 vs entities.md:25
- **Description:** The glossary describes Flow as `{ts, src, dst, ports, proto, service, service_source}` while the authoritative entity (entities.md:25) adds flow_index, conn_state, and splits src_port/dst_port as Option<u16>. The glossary claims to be the fixed definition (ubiquitous-language.md:18) yet omits determinism-critical flow_index (DI-009, DI-015) and severity-critical conn_state (DI-017).
- **Evidence:** ubiquitous-language.md:30 (partial Flow) vs entities.md:25 (full Flow); ubiquitous-language.md:18 ("fixed here").
- **Proposed Fix:** Update the ubiquitous-language Flow entry to match entities.md (include flow_index, conn_state), or add "(see entities.md for full attribute list)".

### Observations (no finding)

- DI-005 / DEC-005 / DEC-017 / DEC-020 form a clean, non-contradictory EXTERNAL treatment.
- DEC-016 (one conn.log record = one Flow, no synthetic B->A) correctly forecloses the likely false-WrongDirection source; consistent with ASM-001.
- [process-gap] ASM-001 is Medium confidence and flagged the #1 false-positive source, with DI-007 directional verdicts downstream, yet no gating story/BC linkage ensures the P0 validation task blocks before DI-007 verdicts are trusted. Process-linkage concern for the PRD/story layer, noted for downstream traceability.
- Empty-input (DEC-014), zero-conduit (DEC-019), zero-member-zone (DEC-018), single-element and duplicate-conduit (DEC-015) cases all covered. Good edge coverage.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 5 |
| LOW | 4 |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate (NOT a clean pass; convergence counter does not advance)
**Readiness:** requires revision — HIGH-001 (digest canonicalization) and HIGH-002 (external_endpoints) are untestable-as-written and blocking before the digest determinism and accounting properties are provable.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 11 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | 2.5 (MED) |
| **Trajectory** | 14 -> 16 -> 11 |
| **Verdict** | FINDINGS_REMAIN |

Trajectory note: Pass 2's 14->16 rise was a documented perimeter-expansion regression (v1.1 accounting scope + un-propagated research decisions), not fix-induced. Pass 3's 16->11 resumes the expected monotonic decrease. All 11 Pass-3 findings are genuinely new (no Pass-1/Pass-2 retread); the gaps cluster at the serialization/encoding boundary and an orphaned output field — seams a converged-looking spec leaves loose. This is NOT one of the 3 consecutive clean passes; the convergence counter stands at 0 clean passes.
