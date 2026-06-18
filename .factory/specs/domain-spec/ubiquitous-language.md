---
document_type: domain-spec-section
level: L2
section: ubiquitous-language
version: "1.8"
status: draft
producer: business-analyst
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[live-state]"
traces_to: L2-INDEX.md
---

# Ubiquitous Language (Glossary)

> Precise definitions of every domain term used across the spec. Where a term is ambiguous in
> the wider industry, the zonewarden meaning is fixed here.

| Term | Definition (as used in zonewarden) |
|------|------------------------------------|
| **Zone** | A grouping of assets that share security requirements (IEC 62443-3-2 ZCR3). Has a Purdue level and a set of asset matchers. |
| **Conduit** | The only sanctioned communication channel between two zones. Permits a specific protocol/port and direction. |
| **Policy** | The complete declared intent: all zones + all conduits + the implicit deny-by-default rule. The unit of validation. |
| **Purdue level** | Position in the Purdue Enterprise Reference Architecture: L0 (process) … L5 (enterprise/Internet). |
| **IDMZ** | Industrial DMZ at Purdue **Level 3.5**, the mandated buffer between the OT (≤L3) and IT (≥L4) sides. |
| **IDMZ no-bypass** | The rule that a single flow between a managed OT zone (≤L3) and a managed IT zone (≥L4) must have an IDMZ endpoint; a direct ≤L3↔≥L4 flow with neither endpoint in the IDMZ is a bypass (DI-006). Single-flow check; multi-hop transit is out of scope. |
| **SL-T (Security Level Target)** | The IEC 62443 target security level for a zone/conduit. Modeled as an optional scalar `0..=4` (overall target), with an optional full 7-element FR vector as metadata. MVP uses the scalar; FR5 is the focus. |
| **FR5 (Restricted Data Flow)** | The IEC 62443 Foundational Requirement governing segmentation; zonewarden is effectively an automated FR5 conformance checker. |
| **Flow** | One observed network communication, normalized to `{flow_index, ts, src_ip, src_port?, dst_ip, dst_port?, proto, service?, service_source, conn_state?}` (see entities.md for the authoritative attribute list). The `src` is the connection **initiator**. |
| **Reality source / adapter** | A pluggable input that translates a vendor-specific record (Zeek conn.log, NetFlow, firewall config) into the neutral `Flow`. The "observed reality" side of the core loop. |
| **Asset (matcher)** | A rule assigning IPs to a zone — a CIDR, single IP (a /32 or /128), or (future) hostname. |
| **Service** | The application-layer protocol (e.g. Modbus, DNP3), distinct from the transport `proto` (TCP/UDP). |
| **Service inference** | Deriving `service` from port when not authoritatively provided — always **heuristic**. |
| **service_source (provenance)** | Where `service` came from: `DpiConfirmed` (authoritative), `PortHeuristic` (guessed from port), or `Unknown`. |
| **Zone resolution** | Mapping a flow endpoint's IP to exactly one zone via longest-prefix match; unmatched → implicit EXTERNAL. |
| **Conduit matching** | Deciding whether an allowed conduit permits a flow's (zone-pair, proto, port, direction). |
| **Deny-by-default** | Anything not explicitly allowed is a violation. The core security posture. |
| **EXTERNAL zone** | The reserved, membership-less zone (fixed `purdue_level = L5`) that catches IPs matching no declared zone (Internet/unmanaged). Flows to/from it are governed by conduit matching (deny-by-default), not the IDMZ rule. |
| **Intra-zone flow** | A flow whose endpoints are in the same zone; allowed without a conduit. |
| **Violation** | A flow that breaches the policy: `NoMatchingConduit`, `WrongDirection`, or `IdmzBypass`. One flow may yield up to two entries (a conduit violation + an additive `IdmzBypass`); `distinct_violating_flows` de-dups by `flow_index`. |
| **Verdict** | The single, mutually-exclusive per-flow conduit/zone classification: `IntraZone \| Allowed \| NoMatchingConduit \| WrongDirection \| MulticastExempt`. |
| **idmz_bypass** | An **independent additive** per-flow finding (NOT a Verdict): a managed ≤L3↔≥L4 single flow with no IDMZ endpoint. A flow can be both `Allowed` and `idmz_bypass` (DI-006). |
| **MulticastExempt** | Verdict for a flow whose destination is a multicast/broadcast address; reported but never a violation (DI-016). |
| **Severity** | Violation grading from `conn_state`: `Established` (confirmed flow) vs `Attempted` (blocked/rejected attempt — the control held) (DI-017). |
| **conn_state** | Adapter-provided connection state (e.g. Zeek `SF`/`S0`/`REJ`) used to derive `Severity`. |
| **flow_index** | The stable per-flow identity (ingest position); used as the DI-009 tiebreaker and to de-dup `distinct_violating_flows`. |
| **policy_digest** | `SHA-256` (lowercase hex) over the **canonical Policy model**, not raw file bytes — formatting/comments don't change it (DI-018). |
| **ConformanceResult** | The aggregate output of a run: typed tallies (intra_zone / allowed / no_matching_conduit / wrong_direction / multicast_exempt / idmz_bypasses / distinct_violating_flows / external_endpoints / skipped), violations, warnings, policy_digest. |
| **Direction** | Conduit semantics: `Forward` (initiator in `from`) or `Bidirectional` (either side may initiate). Canonical YAML: `forward` / `bidirectional` (`unidirectional` aliases `forward`). |
