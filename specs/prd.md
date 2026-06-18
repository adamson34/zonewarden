---
document_type: prd
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [domain-spec/L2-INDEX.md, specs/research/domain-ot-segmentation-validation-2026-06-17.md]
input-hash: "[live-state]"
traces_to: domain-spec/L2-INDEX.md
supplements: [prd-supplements/interface-definitions.md, prd-supplements/error-taxonomy.md, prd-supplements/nfr-catalog.md, prd-supplements/module-criticality.md]
---

# Product Requirements Document: zonewarden

**Date:** 2026-06-17
**Status:** draft
**Source:** product-brief.md, domain-spec/L2-INDEX.md (v1.8, FROZEN)

---

## 1. Product Overview

### 1.1 Problem Statement

OT/ICS network segmentation — the IEC 62443 model of zones connected only through sanctioned conduits — is the primary control protecting industrial processes from IT-side compromise. In practice that segmentation lives in tribal knowledge and hand-maintained firewall rules that drift over time. No tool currently provides open, declarative, offline validation of a 62443 zone/conduit policy against *observed network flows*: config tools (Batfish, Tufin) validate rule intent; monitoring platforms (Claroty, Nozomi) detect anomalies but lack policy-as-code semantics; general policy engines (OPA) are not OT/flow-aware. Assessment teams must check conformance by eye, if at all, producing minimal and non-repeatable evidence.

### 1.2 Solution Vision

zonewarden is a Rust CLI binary that, given a declarative YAML segmentation policy and a captured flow log, deterministically classifies every observed flow as allowed or violating, explains why, and produces both a machine-readable JSON report and a Mermaid zone/conduit diagram with violations highlighted. It is a pure function of its file inputs, fully offline, and CI-friendly. The core zone-resolution and conduit-matching logic is small and pure enough for formal verification (Kani proofs); the parsers are fuzz targets. Correct verdicts are the product; the verification toolchain proves it.

### 1.3 Key Differentiators

| ID | Differentiator | Description |
|----|---------------|-------------|
| KD-001 | Validates policy against observed flows | Validates a declarative 62443 policy against actual captured flow data — the confirmed market whitespace; config tools and monitoring platforms don't combine both |
| KD-002 | 62443/Purdue-native | Speaks the asset owner's language: zones, conduits, IDMZ, FR5, Purdue levels 0–5; not generic ACLs |
| KD-003 | OT-protocol-aware (honestly heuristic) | Recognizes Modbus/DNP3/EtherNet-IP/S7comm/BACnet/OPC UA while being transparent about inference confidence via `service_source` provenance |
| KD-004 | Open + declarative + git-versioned | YAML policy-as-code: reviewable, diffable, CI-runnable; fully open source |
| KD-005 | Deterministic and offline | Byte-identical output for identical inputs; no network calls; repeatable assessment evidence |
| KD-006 | Extensible reality sources | Pluggable RealitySource adapter seam: Zeek conn.log now, NetFlow/firewall-config later, without re-architecting the engine |
| KD-007 | Verification-grade correctness | Kani-proven core (zone resolution, conduit matching, accounting identity) + fuzzed parsers; verdicts are provably correct |

### 1.4 Target Users

| Persona | Description | Volume | Pain Level |
|---------|-------------|--------|------------|
| OT security practitioner / builder | Luke Adamson: primary builder; also a genuine assessment tool and portfolio piece | 1 | Extreme |
| OT security architects | Author and version segmentation policy as code; validate designs before deployment | Small (10s) | High |
| Assessors / auditors | Run repeatable, evidence-producing conformance checks against captured flow data, offline | Small (10s) | High |
| Plant / OT network engineers | Catch drift between intended policy and what the network is actually doing | Medium (100s) | Medium |

### 1.5 Out of Scope

> These items are explicitly excluded. No story acceptance criterion may implement them.

- Live network capture or active scanning — zonewarden consumes captured/exported files only; never touches a live network
- Enforcement or blocking — the tool reports conformance; it does not configure firewalls or drop traffic
- Full IEC 62443-3-2 risk assessment — limited to FR5 (Restricted Data Flow) conformance checking; not a risk-assessment platform
- GUI or web application — CLI-first; no web UI in any phase
- Layer-2-only protocols (e.g., PROFINET RT) invisible in IP flow logs — acknowledged as undetectable via this data source
- Asset discovery, inventory, vulnerability/CVE correlation, IDS/anomaly detection — explicitly not this product
- Multi-hop transit IDMZ analysis — IDMZ bypass is a single-flow check only; multi-hop relay detection is out of scope

---

## 2. Behavioral Contracts Index

> Individual BC files live in `behavioral-contracts/ss-NN/` shard directories, one shard per subsystem.
> Grouped by L2 domain subsystem (CAP-NNN). Numbering: BC-1.SS.NNN where 1 = PRD section 1, SS = subsystem, NNN = sequential.
> Subsystem IDs SS-01..SS-06 are **placeholders** — architect will assign final IDs in ARCH-INDEX.

### 2.1 SS-01 Policy (Load + Validate + Model)

> CAP-001 (Load policy), CAP-002 (Validate policy)
> Entities: Policy, Zone, Conduit, AssetMatcher, PortSet, Direction, SlTarget

| BC ID | Title | Priority |
|-------|-------|----------|
| BC-1.01.001 | Parse valid YAML policy into internal Policy model | P0 |
| BC-1.01.002 | Reject malformed YAML with precise diagnostic (fail-fast) | P0 |
| BC-1.01.003 | Reject duplicate YAML mapping keys at load time | P0 |
| BC-1.01.004 | Validate zone IDs unique and conduit endpoints exist | P0 |
| BC-1.01.005 | Reject equal-prefix-length ties (same-family, same address) | P0 |
| BC-1.01.006 | Reject 0.0.0.0/0 or ::/0 catch-all member declarations | P0 |
| BC-1.01.007 | Reject unrecognized direction/proto tokens (no permissive default) | P0 |
| BC-1.01.008 | Warn (not error) on declared zone with zero members | P0 |
| BC-1.01.009 | PortSet canonical form: sorted non-overlapping non-adjacent ranges | P0 |

> Full contracts: `behavioral-contracts/ss-01/BC-1.01.001.md` through `BC-1.01.009.md`

### 2.2 SS-02 Flow Ingest and Normalization

> CAP-003 (Ingest flows), CAP-004 (Normalize + infer service)
> Entities: Flow, Service, ServiceSource, ConnState, Timestamp; canonical service-port table

| BC ID | Title | Priority |
|-------|-------|----------|
| BC-1.02.001 | Parse valid Zeek conn.log line into normalized Flow | P0 |
| BC-1.02.002 | Skip and count malformed flow lines; never abort the run | P0 |
| BC-1.02.003 | Skip and warn flows with unspecified address (0.0.0.0 or ::) as src or dst | P0 |
| BC-1.02.004 | Assign service and service_source via canonical port/proto table | P0 |
| BC-1.02.005 | Canonicalize IPv4-mapped IPv6 addresses to IPv4 before resolution | P0 |
| BC-1.02.006 | Enforce max_flows ingest cap; abort with exit 2 on breach | P0 |

> Full contracts: `behavioral-contracts/ss-02/BC-1.02.001.md` through `BC-1.02.006.md`

### 2.3 SS-03 Zone Resolution

> CAP-005 (Resolve endpoints to zones)
> Entities: ResolvedEndpoint, MatchKind, PurdueLevel

| BC ID | Title | Priority |
|-------|-------|----------|
| BC-1.03.001 | Resolve endpoint to exactly one zone via longest-prefix match | P0 |
| BC-1.03.002 | Resolve unmatched endpoint to implicit EXTERNAL zone | P0 |
| BC-1.03.003 | Multicast/broadcast destination short-circuits to MulticastBroadcast before zone resolution | P0 |
| BC-1.03.004 | Directed-broadcast destination override: all-ones host of ≤/30 zone → MulticastBroadcast | P0 |
| BC-1.03.005 | Both-EXTERNAL flow: two endpoints both resolving to EXTERNAL → IntraZone | P0 |

> Full contracts: `behavioral-contracts/ss-03/BC-1.03.001.md` through `BC-1.03.005.md`

### 2.4 SS-04 Classification and Verdict

> CAP-006 (Match conduits), CAP-007 (Classify deny-by-default), CAP-008 (IDMZ no-bypass)
> Entities: Verdict, VerdictKind, Violation, ViolationKind, Severity; DI-006 truth table

| BC ID | Title | Priority |
|-------|-------|----------|
| BC-1.04.001 | Intra-zone flows are allowed without conduit evaluation | P0 |
| BC-1.04.002 | Deny-by-default: flow with no matching conduit → NoMatchingConduit | P0 |
| BC-1.04.003 | Any-match conduit union: flow allowed if ≥1 conduit permits it | P0 |
| BC-1.04.004 | Forward conduit enforces directionality; reverse-initiated flow → WrongDirection | P0 |
| BC-1.04.005 | Bidirectional conduit permits initiation from either zone | P0 |
| BC-1.04.006 | Portless protocol (ICMP/Other) matches only ports: Any conduit | P0 |
| BC-1.04.007 | IDMZ no-bypass: managed ≤L3 ↔ managed ≥L4 without IDMZ endpoint → additive IdmzBypass finding | P0 |
| BC-1.04.008 | IDMZ bypass is NOT raised for flows involving EXTERNAL or MulticastBroadcast endpoints | P0 |
| BC-1.04.009 | Violation severity graded from conn_state: Attempted bucket → Attempted; otherwise → Established | P0 |
| BC-1.04.010 | Verdict totality: every resolved flow receives exactly one VerdictKind | P0 |
| BC-1.04.011 | MulticastExempt short-circuits before IntraZone and conduit evaluation | P0 |

> Full contracts: `behavioral-contracts/ss-04/BC-1.04.001.md` through `BC-1.04.011.md`

### 2.5 SS-05 Aggregation and Determinism

> CAP-009 (Aggregate result)
> Entities: ConformanceResult

| BC ID | Title | Priority |
|-------|-------|----------|
| BC-1.05.001 | DI-015 accounting identity holds for every ConformanceResult | P0 |
| BC-1.05.002 | Output ordered by total-order key (ts, src_ip, src_port, dst_ip, dst_port, proto, flow_index) | P0 |
| BC-1.05.003 | Stable policy digest via canonical JSON serialization (SHA-256) | P0 |
| BC-1.05.004 | u64 tally overflow: checked arithmetic aborts with exit 2; never silent-wraps | P0 |
| BC-1.05.005 | Empty flow input yields all-zero ConformanceResult; exits 0 | P0 |

> Full contracts: `behavioral-contracts/ss-05/BC-1.05.001.md` through `BC-1.05.005.md`

### 2.6 SS-06 Reporting and CLI

> CAP-010 (Render outputs); exit codes 0/1/2; Mermaid diagram; JSON/text report; offline (DI-012)

| BC ID | Title | Priority |
|-------|-------|----------|
| BC-1.06.001 | Exit codes: 0 conformant, 1 violations, 2 error/policy/usage | P0 |
| BC-1.06.002 | Emit JSON violations report with all required fields | P0 |
| BC-1.06.003 | Emit human-readable text violations report | P0 |
| BC-1.06.004 | Emit Mermaid zone/conduit diagram with violations highlighted | P0 |
| BC-1.06.005 | Warnings emitted to stderr in deterministic order; never change exit code | P0 |
| BC-1.06.006 | --fail-on-skipped: skipped > 0 forces non-zero exit | P0 |
| BC-1.06.007 | zonewarden never opens a network socket or mutates input files (offline invariant) | P0 |
| BC-1.06.008 | Output artifacts written atomically (write-then-rename; no partial files on error) | P0 |

> Full contracts: `behavioral-contracts/ss-06/BC-1.06.001.md` through `BC-1.06.008.md`

### 2.7 Roadmap Subsystems (P1/P2 — No MVP BCs)

The following capabilities are out of MVP BC scope. They appear here as planned roadmap items:

| Capability | Phase | Description |
|-----------|-------|-------------|
| CAP-011 NetFlow/IPFIX adapter | P1 / Phase 2 | Second RealitySource; proves pluggable adapter seam |
| CAP-012 Native SVG rendering | P1 / Phase 2 | Self-rendered SVG topology diagram |
| CAP-013 Policy DSL front-end | P2 / Phase 3 | Custom policy language → same internal model; fuzzable Kani-provable parser |
| CAP-014 Firewall-config adapter | P2 / Phase 3 (stretch) | Validate intent against firewall rules, not flows |

---

## 3. Interface Definition

> **Supplement:** Full interface definitions are in `prd-supplements/interface-definitions.md`.
> Primary consumers: implementer, test-writer.

**CLI summary:** `zonewarden --policy <PATH> --flows <PATH> [--format text|json|mermaid] [--output <PATH>] [--fail-on-skipped]`

**Exit codes:** `0` = conformant (no violations); `1` = one or more violations present; `2` = policy/usage/I-O error.

**Outputs:** text report (stdout), JSON report (`--format json`), Mermaid diagram (`--format mermaid`), warnings (stderr).

See `prd-supplements/interface-definitions.md` for complete CLI help text, JSON schema, YAML policy schema, and flag interactions.

---

## 4. Non-Functional Requirements

> **Supplement:** Full NFR catalog is in `prd-supplements/nfr-catalog.md`.
> Primary consumers: architect, performance-engineer, formal-verifier.

| ID | Category | Requirement | Target |
|----|----------|-------------|--------|
| NFR-001 | Determinism | Identical inputs → byte-identical output | 100% reproducible (Kani proof target) |
| NFR-002 | Performance | Throughput for flow classification | ≥ 100,000 flows/second on a modern laptop |
| NFR-003 | Memory | Memory growth with input size | Streaming ingest; aggregation O(flows); ≤ 2 GB for 1M flows |
| NFR-004 | Ingest cap | Maximum flow count per run | Default cap: 1,000,000 flows; configurable via --max-flows |
| NFR-005 | Offline | No network calls | Zero network I/O by construction (DI-012) |
| NFR-006 | Read-only | No input mutation | Never modifies policy or flow files |
| NFR-007 | Verification | Kani proof targets | DI-003, DI-004, DI-006, DI-009, DI-012, DI-015, DI-016, DI-018, DI-020 |
| NFR-008 | Mutation testing | cargo-mutants kill rate by module tier | CRITICAL ≥ 95%; HIGH ≥ 90%; MEDIUM ≥ 80% |
| NFR-009 | Fuzz testing | cargo-fuzz parser targets | Zero parser panics in 10-minute bounded run |

See `prd-supplements/nfr-catalog.md` for complete catalog with validation methods.

---

## 5. Error Taxonomy

> **Supplement:** Full error taxonomy is in `prd-supplements/error-taxonomy.md`.
> Primary consumers: implementer, test-writer.

**Error code convention:** `E-{CAT}-{NNN}` where CAT ∈ {POL, FLW, IO, SYS}

| Category | Code Prefix | Description |
|----------|-------------|-------------|
| POL | E-POL | Policy file load, parse, and validation errors |
| FLW | E-FLW | Flow ingest and normalization errors |
| IO | E-IO | File I/O errors (missing file, permission denied, disk full) |
| SYS | E-SYS | System-level errors (overflow, cap breach) |

See `prd-supplements/error-taxonomy.md` for complete catalog with message formats.

---

## 6. Competitive Differentiator Traceability

> Maps each differentiator (Section 1.3) to the behavioral contracts that implement it.

### 6.1 KD-001 — Validates policy against observed flows

| BC ID | Contribution |
|-------|-------------|
| BC-1.02.001 | Ingest Zeek conn.log as structured Flow records |
| BC-1.03.001 | Resolve each flow's endpoints to zones |
| BC-1.04.002 | Deny-by-default classification — the core policy→flow verdict |
| BC-1.04.003 | Any-match conduit union — the allowing gate |
| BC-1.06.002 | Machine-readable JSON report of all verdicts |

### 6.2 KD-002 — 62443/Purdue-native

| BC ID | Contribution |
|-------|-------------|
| BC-1.01.001 | Loads zones with purdue_level and sl_t from YAML |
| BC-1.03.001 | Zone resolution using Purdue-native zone membership |
| BC-1.04.007 | IDMZ no-bypass rule enforces the Purdue 3.5 boundary |
| BC-1.04.008 | EXTERNAL exclusion from IDMZ rule (correct 62443 semantics) |

### 6.3 KD-003 — OT-protocol-aware (honestly heuristic)

| BC ID | Contribution |
|-------|-------------|
| BC-1.02.004 | Service inference with PortHeuristic / Unknown provenance |
| BC-1.06.003 | Text report surfaces service_source; heuristics visibly flagged |

### 6.4 KD-004 — Open + declarative + git-versioned

| BC ID | Contribution |
|-------|-------------|
| BC-1.01.001 | YAML policy load (serde; human-readable, git-diffable) |
| BC-1.05.003 | Stable policy digest enables reproducible run evidence |

### 6.5 KD-005 — Deterministic and offline

| BC ID | Contribution |
|-------|-------------|
| BC-1.05.002 | Total-order deterministic output |
| BC-1.05.003 | Stable policy digest |
| BC-1.06.007 | No network socket ever opened |
| BC-1.06.005 | Warnings in deterministic order |

### 6.6 KD-006 — Extensible reality sources

| BC ID | Contribution |
|-------|-------------|
| BC-1.02.001 | Zeek conn.log adapter (the first RealitySource implementation) |
| BC-1.02.006 | Ingest cap enforced at the adapter/RealitySource boundary |

### 6.7 KD-007 — Verification-grade correctness

| BC ID | Contribution |
|-------|-------------|
| BC-1.03.001 | Kani proof target: total endpoint resolution (DI-003, DI-004) |
| BC-1.04.010 | Kani proof target: verdict totality |
| BC-1.05.001 | Kani/proptest proof target: DI-015 accounting identity |
| BC-1.05.002 | Kani proof target: determinism (DI-009) |
| BC-1.05.003 | Kani proof target: stable policy digest (DI-018) |
| BC-1.01.009 | Kani proof target: PortSet canonical form (DI-020) |

---

## 7. Requirements Traceability Matrix

| BC ID | Source (L2 CAP) | L2 Invariants | Subsystem | Priority | Test Type |
|-------|----------------|---------------|-----------|----------|-----------|
| BC-1.01.001 | CAP-001 | DI-010, DI-011 | SS-01 | P0 | unit/fuzz |
| BC-1.01.002 | CAP-001 | DI-010, DI-011 | SS-01 | P0 | unit/fuzz |
| BC-1.01.003 | CAP-001 | DI-010 | SS-01 | P0 | unit |
| BC-1.01.004 | CAP-002 | DI-010 | SS-01 | P0 | unit |
| BC-1.01.005 | CAP-002 | DI-010 | SS-01 | P0 | unit/property |
| BC-1.01.006 | CAP-002 | DI-010 | SS-01 | P0 | unit |
| BC-1.01.007 | CAP-002 | DI-010 | SS-01 | P0 | unit |
| BC-1.01.008 | CAP-002 | DI-010 | SS-01 | P0 | unit |
| BC-1.01.009 | CAP-006 | DI-020 | SS-01 | P0 | property/kani |
| BC-1.02.001 | CAP-003 | DI-013 | SS-02 | P0 | unit/fuzz |
| BC-1.02.002 | CAP-003 | DI-013 | SS-02 | P0 | unit |
| BC-1.02.003 | CAP-003 | DI-013 | SS-02 | P0 | unit |
| BC-1.02.004 | CAP-004 | DI-008 | SS-02 | P0 | unit |
| BC-1.02.005 | CAP-005 | DI-003 | SS-02 | P0 | unit |
| BC-1.02.006 | CAP-003 | FM-008 | SS-02 | P0 | integration |
| BC-1.03.001 | CAP-005 | DI-003, DI-004 | SS-03 | P0 | unit/kani/property |
| BC-1.03.002 | CAP-005 | DI-005 | SS-03 | P0 | unit |
| BC-1.03.003 | CAP-007 | DI-016 | SS-03 | P0 | unit |
| BC-1.03.004 | CAP-005 | DI-016 | SS-03 | P0 | unit |
| BC-1.03.005 | CAP-005 | DI-002, DI-005 | SS-03 | P0 | unit |
| BC-1.04.001 | CAP-007 | DI-002 | SS-04 | P0 | unit |
| BC-1.04.002 | CAP-007 | DI-001 | SS-04 | P0 | unit |
| BC-1.04.003 | CAP-006 | DI-014 | SS-04 | P0 | unit/property |
| BC-1.04.004 | CAP-006 | DI-007 | SS-04 | P0 | unit |
| BC-1.04.005 | CAP-006 | DI-007 | SS-04 | P0 | unit |
| BC-1.04.006 | CAP-006 | DI-020 | SS-04 | P0 | unit |
| BC-1.04.007 | CAP-008 | DI-006 | SS-04 | P0 | unit/kani |
| BC-1.04.008 | CAP-008 | DI-006 | SS-04 | P0 | unit |
| BC-1.04.009 | CAP-007 | DI-017 | SS-04 | P0 | unit |
| BC-1.04.010 | CAP-007 | DI-015 | SS-04 | P0 | kani/property |
| BC-1.04.011 | CAP-007 | DI-016 | SS-04 | P0 | unit |
| BC-1.05.001 | CAP-009 | DI-015 | SS-05 | P0 | kani/property |
| BC-1.05.002 | CAP-009 | DI-009 | SS-05 | P0 | kani/property |
| BC-1.05.003 | CAP-009 | DI-018 | SS-05 | P0 | kani/property |
| BC-1.05.004 | CAP-009 | FM-009 | SS-05 | P0 | unit |
| BC-1.05.005 | CAP-009 | DI-015 | SS-05 | P0 | unit |
| BC-1.06.001 | CAP-010 | FM-001..003 | SS-06 | P0 | integration |
| BC-1.06.002 | CAP-010 | DI-009 | SS-06 | P0 | integration |
| BC-1.06.003 | CAP-010 | DI-009 | SS-06 | P0 | integration |
| BC-1.06.004 | CAP-010 | DI-009 | SS-06 | P0 | integration |
| BC-1.06.005 | CAP-010 | DI-019 | SS-06 | P0 | unit |
| BC-1.06.006 | CAP-003 | DI-013 | SS-06 | P0 | unit |
| BC-1.06.007 | CAP-010 | DI-012 | SS-06 | P0 | kani/unit |
| BC-1.06.008 | CAP-010 | FM-006 | SS-06 | P0 | unit |

**Total: 44 behavioral contracts across 6 subsystems.**

---

## 8. Assumptions

> From L2 assumptions.md. Those marked *PRD-pin* are specifically resolved or deferred at this layer.

| ID | Assumption | Status | PRD Resolution |
|----|------------|--------|----------------|
| ASM-001 | Flow src = connection initiator; one conn.log record = one Flow | Unvalidated | BC-1.02.001 encodes the rule; DEC-016 is a postcondition; validation against Zeek semantics is a P0 gating task before directional verdicts are trusted |
| ASM-002 | Zone membership by IP/CIDR sufficient for MVP | Unvalidated | Accepted for MVP; BC-1.03.001 and its edge cases define the scope |
| ASM-003 | Port→service inference covers enough OT protocols | Unvalidated | BC-1.02.004 pins the canonical table; all inferences are PortHeuristic |
| ASM-004 | FR5 conformance maps to conduit allow/deny | Unvalidated | Accepted for MVP per research |
| ASM-005 | Mermaid legibility at realistic plant scale | Unvalidated | BC-1.06.004; prototype early; SVG (CAP-012) is the fallback |
| ASM-006 | Synthetic flow data can be produced non-confidentially | Unvalidated | Required before demo; synthetic generator in scope |
| ASM-007 | IDMZ identifiable by purdue_level = IDMZ (L3.5) | Unvalidated | BC-1.04.007 encodes this; the policy YAML schema uses PurdueLevel as a typed field |
| ASM-008 | IP/CIDR zoning not reliable under NAT/VLAN-only | Known limitation | Documented; out of scope for v1; BC-1.03.001 notes this |
| ASM-009 | Single-transport service inference acceptable for MVP | Unvalidated | Accepted; BC-1.02.004 pins the exact table and `Unknown` for all multi-transport variants |

---

## 9. Open Questions (D-009 Backlog)

> These items were explicitly deferred by D-009 (L2 spec frozen at v1.8). They are **not silently dropped**; they are tracked here for resolution at PRD/TDD layer.

### OQ-001 — Full 13-state Zeek conn_state → Severity bucket table

**From:** D-009 backlog; entities.md BACKLOG note; P08-LOW-003 (editorial note about "previously-wrong RSTO")

**Issue:** entities.md pins only examples for the ConnState → Severity mapping: `SF` and `RSTO`/`RSTR` → Established; `S0`/`REJ`/`SH` → Attempted. The complete 13-state Zeek `conn_state` mapping (including `S1`/`S2`/`S3`, `OTH`, `SHR`, `RSTOS0`, etc.) is deferred to PRD/impl layer.

**Decision needed:** Assign every Zeek conn_state token (`S0`, `S1`, `S2`, `S3`, `SF`, `REJ`, `RSTO`, `RSTR`, `RSTOS0`, `RSTRH`, `SH`, `SHR`, `OTH`) to `Established` or `Attempted` bucket.

**Proposed resolution (to be confirmed during TDD):**

| Zeek conn_state | Bucket | Rationale |
|-----------------|--------|-----------|
| S0 | Attempted | SYN sent, no response |
| S1 | Attempted | SYN+SYN/ACK, no FIN (partially established — conservative: handshake not complete) |
| S2 | Established | Connection established; FIN seen but no reply |
| S3 | Established | Connection established; FIN seen from both sides, no final ACK |
| SF | Established | Normal close |
| REJ | Attempted | RST in response to SYN |
| RSTO | Established | Established then reset by originator |
| RSTR | Established | Established then reset by responder |
| RSTOS0 | Attempted | SYN then RST before response |
| RSTRH | Attempted | SYN/ACK then RST (half-open) |
| SH | Attempted | SYN + SYN/ACK, no further |
| SHR | Attempted | SYN + SYN/ACK + RST |
| OTH | Established | No SYN; connection mid-capture (conservative: assume established) |

**Status:** This table is a **PRD-pinned backlog item**; it must be codified as a test fixture before implementing BC-1.04.009.

### OQ-002 — service_source precedence rule (DpiConfirmed vs PortHeuristic)

**From:** product-brief.md Open Question #2; P08-MED-003.

**Issue:** The MVP Zeek conn.log adapter never produces `DpiConfirmed`; only `PortHeuristic` or `Unknown` are reachable in v1. The `DpiConfirmed` enum variant exists for future DPI-capable adapters (NetFlow with NBAR, etc.).

**Resolution for MVP (accepted):** `DpiConfirmed` is an enum variant with no MVP producer. Precedence rule is moot for v1. When a future adapter produces `DpiConfirmed`, it takes precedence over `PortHeuristic` (higher-confidence source wins). This is a non-breaking semantic extension.

**Action:** Document the precedence rule in the interface definitions. No BC needed for v1. Track as a P1 follow-up when CAP-011 (NetFlow) is implemented.

### OQ-003 — P08-MED-004: FM-007/FM-008 memory model reconciliation

**Issue:** FM-007 mandates streaming ingest (don't materialize all). But DI-009 deterministic ordering requires sorting the full result set (all Verdicts/Violations). With `max_flows = 1,000,000`, sorting 1M Verdict structs in memory is ~100–200 MB — acceptable on modern hardware but inconsistent with FM-007's framing.

**Resolution (adopted):** FM-007's "don't materialize all" applies specifically to the *ingest/normalization* stage (ST-3/ST-4). The *aggregation* stage (ST-7) materializes only Violations (not all Verdicts) for ordering; per-VerdictKind counts are streamed as running tallies. This reduces retained memory from O(total_flows) to O(violations), which is a small fraction for any conformant network.

**Action:** NFR-003 and NFR-004 updated to reflect this. BC-1.05.001 postconditions encode it.

### OQ-004 — P08-MED-005: /0 rejection vs broader catch-all prefixes (/1–/7)

**Issue:** DI-010 rejects `/0` catch-all members but allows `/1` through `/7`, which partially shadow EXTERNAL.

**Resolution (adopted for MVP):** Only `/0` (exact match) is rejected. Very short prefixes are legal but unusual; a load-time warning (not error) is emitted for any member with prefix length < 8. This is added to BC-1.01.004 edge cases and the policy validator.

### OQ-005 — P08-MED-006: IPv4 all-zeros network address as destination

**Issue:** DI-016 exempts the all-ones broadcast address but not the all-zeros network address.

**Resolution (adopted):** The all-zeros network address (e.g., `10.0.0.0` for a `/24`) as a destination is classified normally (not exempted). The rationale for asymmetry: the directed broadcast (`10.0.0.255`) is a real traffic pattern (e.g., ARP, OT broadcast I/O); the network address as a destination is a misconfiguration rather than a protocol pattern. It receives a normal verdict (likely `NoMatchingConduit` → violation, which is correct behaviour — it should be flagged). This is documented as a deliberate design choice.

### OQ-006 — P08-MED-007: Partial output on max_flows cap breach

**Resolution (adopted):** On max_flows breach, zonewarden emits no ConformanceResult and no report — clean abort with exit 2 and a diagnostic naming the cap value and the flow count at abort. This is consistent with DI-009 (determinism requires all-or-nothing output) and simplifies the CI contract.

### OQ-007 — Remaining Pass-8 LOW findings

| Finding | Status |
|---------|--------|
| P08-LOW-001 (ID count CI check) | Out of scope for PRD; architect adds CI assertion |
| P08-LOW-002 (Direction grammar authority split) | Resolved: entities.md Value Objects is the canonical grammar source |
| P08-LOW-003 (editorial "previously-wrong" phrasing) | Acknowledged; not a spec defect |
| P08-LOW-004 (brief FM-008 vs FM-009 citation) | Noted; brief is frozen; PRD cites FM-008 and FM-009 correctly |
