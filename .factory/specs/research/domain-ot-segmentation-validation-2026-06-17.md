---
document_type: domain-research
level: L2
version: "1.0"
status: draft
producer: research-agent
timestamp: 2026-06-17T00:00:00
phase: 1a
research_type: domain
inputs: [research/RESEARCH-INDEX.md]
traces_to: product-brief.md
output_dir: .factory/specs/research/
---

# Domain Research: OT Segmentation-as-Code (SaC) Validation Tool

> **Validity note:** Findings are current as of **2026-06-17**. IEC/ISA standards editions,
> Zeek versions, vendor capabilities, and IANA port assignments change over time and should be
> re-verified before locking the spec. Where a claim rests on model knowledge rather than a cited
> source, it is flagged inline.

## Executive Context

The tool under research is a Rust **Segmentation-as-Code (SaC)** validator. It (1) ingests network
flow data, (2) models a desired IEC 62443-3-2 zone/conduit segmentation policy as code, and
(3) checks observed flows against the declared policy, emitting violations. The tool **validates
conformance to a declared policy** — it does **not** perform a full IEC 62443-3-2 risk assessment.
This scoping is the key differentiator and the source of the whitespace opportunity (see Section 1).

---

## Section 1 — Competitive Landscape (Prior Art)

Five categories of prior art are relevant. None occupies the exact niche: an **open, declarative,
git-versioned policy-as-code tool that takes a 62443 zone/conduit policy + observed flows and
reports violations**.

### 1.1 Firewall Policy Verification / Network Reachability Analysis

| Tool | License | Input | What it does well | Gap vs. SaC tool |
|------|---------|-------|-------------------|------------------|
| **Batfish** | Apache-2.0 (open) | Device **configs** (routers, firewalls, ACLs) | Builds a formal model of network behavior from configs; answers reachability questions; finds unreachable ACL lines; validates user-defined policies offline without device access. | Operates on *configurations*, not *observed flows*. No 62443 zone/conduit semantics, no Purdue model, no OT protocol awareness. Heavy JVM/pybatfish footprint. [batfish.org](https://batfish.org/), [github.com/batfish/batfish](https://github.com/batfish/batfish), [analyzing-acls notebook](https://batfish.readthedocs.io/en/latest/notebooks/linked/analyzing-acls-and-firewall-rules.html) |
| **FireMon** | Commercial | Live firewall/cloud policy | Continuous NSPM; aligns access to "intent/risk"; automated compliance & anomaly reports; segmentation governance. | Config/rule-centric, not flow-conformance. Closed, no git-native declarative policy. Not 62443 zone/conduit native. [firemon.com](https://www.firemon.com/firemon-vs-algosec-vs-tufin/) |
| **AlgoSec** | Commercial | Firewalls, switches, proxies, LBs | Multi-vendor policy orchestration & change management; integrates broad device set. | Same as FireMon — rule orchestration, not declarative observed-flow conformance. [aimultiple.com](https://aimultiple.com/network-security-policy-management-solutions) |
| **Tufin** | Commercial | Firewalls, SD-WAN, SASE, hybrid cloud | Accurate topology; zone/microsegmentation modeling; ZTNA. | Rule/topology-centric; closed; not 62443-policy-as-code over flows. [tufin.com](https://www.tufin.com/why-tufin/algosec-alternative) |

**Category gap:** These verify *configurations/rules*; the SaC tool verifies *observed traffic*
against a *declared intent*. Batfish is the closest open analog but is config-driven and
OT-agnostic.

### 1.2 Intent-Based / Microsegmentation Platforms

| Tool | License | Model | Strength | Gap vs. SaC tool |
|------|---------|-------|----------|------------------|
| **Illumio** | Commercial | Agent-based; central Policy Compute Engine pushes rules to host firewalls | Workload-centric segmentation across hybrid cloud; strong policy compute. | Agents fail on PLCs/RTUs/legacy OT devices; enforcement-focused not validation; not 62443-native. [elisity.com](https://www.elisity.com/blog/what-are-the-top-microsegmentation-solutions-for-2026) |
| **Cisco TrustSec / Secure Workload** | Commercial | SGT tags / agent-based | Identity/tag-based segmentation; Cisco fabric integration. | Enforcement infra, not a declarative offline validator; agent model poor for OT. [elisity.com](https://www.elisity.com/blog/what-are-the-top-microsegmentation-solutions-for-2026) |
| **Akamai Guardicore** | Commercial | Hybrid agent/agentless | East-west visibility; converts app intelligence into enforcement-ready policy; ransomware blast-radius containment. | Visibility+enforcement product, not git-versioned 62443 conformance checking. [akamai.com](https://www.akamai.com/products/akamai-guardicore-segmentation) |
| **Elisity** | Commercial | Switch-enforced, identity-based, agentless | Non-disruptive OT microsegmentation at the switch — works where agents can't. | Enforcement platform; not an open declarative validation/CI tool. [elisity.com](https://www.elisity.com/blog/leading-vendors-for-securing-ot-and-industrial-control-systems-in-2026) |

**Category gap:** These *enforce* segmentation. They are not designed as offline, file-based,
git-diffable **validators** of a 62443 policy against captured flows, and most are agent-based
(a non-starter for Level 0–2 OT devices).

### 1.3 OT-Specific Visibility / Segmentation Platforms

| Tool | License | Approach | Strength | Gap vs. SaC tool |
|------|---------|----------|----------|------------------|
| **Claroty** | Commercial | Passive + active; 450+ industrial protocols | Deep OT asset discovery; **auto-recommends segmentation policies from observed comms**; vuln mgmt. | Closed appliance; policy lives in the product, not as portable git-versioned code; recommendation engine ≠ declarative conformance test. [cybermagazine.com](https://cybermagazine.com/top10/top-10-ot-security-platforms) |
| **Nozomi Networks (Guardian)** | Commercial | Passive monitoring + Smart Polling; 500+ protocols | Broad OT visibility; baseline/anomaly detection. | Visibility/IDS, not declarative 62443 policy-as-code conformance over flows. [cybermagazine.com](https://cybermagazine.com/top10/top-10-ot-security-platforms) |
| **Dragos** | Commercial | OT threat detection + "defensible architecture" | Strong ICS threat intel; architecture guidance. | Detection/advisory, not a portable open SaC validator. [iotsecurityinstitute.com](https://iotsecurityinstitute.com/iotsec/iot-security-institute-cyber-security-articles/247-navigating-the-ot-security-landscape-a-comparison-of-claroty,-nozomi-networks,-and-dragos) |
| **Forescout / SilentDefense** | Commercial | Agentless asset compliance + OT monitoring | Vendor-agnostic at scale; asset policy compliance. | Closed; not git-native declarative 62443 zone/conduit-as-code. [cybermagazine.com](https://cybermagazine.com/top10/top-10-ot-security-platforms) |

**Category gap:** These platforms *observe* OT networks and can *recommend* or *baseline*
segmentation, but the policy artifact is locked inside a commercial appliance. There is **no open,
declarative, version-controlled policy file** that a security architect can author, diff, review in
a PR, and run in CI against flow exports.

### 1.4 Policy-as-Code Frameworks (general-purpose)

| Tool | License | Language | Strength | Gap vs. SaC tool |
|------|---------|----------|----------|------------------|
| **Open Policy Agent (OPA) / Rego** | Apache-2.0 (open) | Rego (declarative) | General-purpose policy engine; declarative "what is true under what conditions"; can express subnet-egress rules and PCI segmentation across topologies. | Generic — no OT/62443/Purdue domain model, no flow ingestion, no protocol-port semantics. Could be a *backend* but not the *product*. [openpolicyagent.org/docs](https://www.openpolicyagent.org/docs), [policy-language](https://www.openpolicyagent.org/docs/policy-language) |
| **HashiCorp Sentinel** | Commercial (embedded in HashiCorp products) | Sentinel | Policy-as-code for Terraform/Vault/etc.; guardrails in IaC pipelines. | Tied to HashiCorp ecosystem; no OT domain model. [apiiro.com](https://apiiro.com/glossary/policy-as-code-2/) |

**Category gap:** These prove the *pattern* (declarative, git-versioned, CI-enforced policy) but
have **zero domain model** for OT segmentation. They validate the architectural bet that
"segmentation as code" is viable, while leaving the OT-specific product space open.

### 1.5 Gap Matrix

Columns are the capabilities a 62443 SaC validator needs. ● = strong, ◐ = partial/adjacent,
○ = absent.

| Capability | Batfish | FireMon/AlgoSec/Tufin | Illumio/Guardicore/Elisity | Claroty/Nozomi/Dragos/Forescout | OPA/Sentinel | **SaC tool (target)** |
|---|:---:|:---:|:---:|:---:|:---:|:---:|
| Open source | ● | ○ | ○ | ○ | ● | ● |
| Declarative, git-versioned policy file | ◐ | ○ | ○ | ○ | ● | ● |
| Validates **observed flows** (not just config) | ○ | ○ | ◐ | ● | ○ | ● |
| Native IEC 62443-3-2 zone/conduit model | ○ | ○ | ○ | ◐ | ○ | ● |
| Purdue level (0–5, 3.5) awareness | ○ | ○ | ○ | ◐ | ○ | ● |
| OT protocol/port "service" classification | ○ | ○ | ◐ | ● | ○ | ● |
| Offline / file-based / CI-friendly | ● | ○ | ○ | ○ | ● | ● |
| Multi-source flow ingest (Zeek + NetFlow/IPFIX) | ○ | ○ | ○ | ◐ | ○ | ● |
| No agents required on OT devices | ● | ◐ | ○ | ● | ● | ● |
| Vendor-neutral / no appliance lock-in | ● | ○ | ○ | ○ | ● | ● |

**Whitespace / differentiation:** The intersection — *open + declarative/git-versioned + observed-flow
conformance + native 62443 zone/conduit + Purdue-aware + OT-protocol-aware + offline/CI-friendly* —
is **unoccupied**. Commercial OT platforms (Claroty etc.) have the domain depth but lock policy in an
appliance; general PaC tools (OPA) have the pattern but no domain; reachability tools (Batfish) are
open but config-centric and OT-blind. The SaC tool's pitch — **"terraform-plan / Batfish for OT
62443 segmentation, driven by real flows"** — is genuinely novel.

---

## Section 2 — Domain Model / Standards: IEC 62443-3-2 Zones & Conduits

### 2.1 Zone vs. Conduit

- **Zone:** A grouping of logical or physical assets that share common security requirements,
  based on factors such as criticality and consequence. Assets with the same target security level
  are grouped into a zone.
- **Conduit:** A logical grouping of communication channels connecting two or more zones that share
  common security requirements. **All inter-zone communication must traverse a defined conduit** —
  this is the core invariant a SaC validator enforces.

Sources: [Novesh — Understanding IEC 62443-3-2 Zones, Conduits, and Risk Assessments](https://novesh.com/blog/novesh-blog-7/understanding-iec-62443-3-2-zones-conduits-and-risk-assessments-27),
[MDPI — Security Aspects of Zones and Conduits in IEC 62443](https://www.mdpi.com/2624-800X/6/2/52).

### 2.2 The Zone & Conduit Requirements (ZCR 1–7) Workflow

IEC 62443-3-2:2020 defines a 7-step security risk-assessment workflow. (The tool implements **only
the conformance-checking outcome of ZCR 3 and ZCR 6** — it does not run the risk assessment.)

| Step | Name | Summary |
|------|------|---------|
| ZCR 1 | Identify the System Under Consideration (SUC) | Define scope/boundary of the IACS being assessed. |
| ZCR 2 | Initial cybersecurity risk assessment | High-level risk to identify worst-case unmitigated risk. |
| ZCR 3 | **Partition the SUC into zones & conduits** | Group assets by shared security requirements; define conduits between zones. *(Produces the artifact the SaC tool validates against.)* |
| ZCR 4 | Initial risk exceeds tolerable risk? | Decision gate — if not, stop; if yes, proceed to detailed assessment. |
| ZCR 5 | Detailed cybersecurity risk assessment | Per-zone/conduit detailed risk → derive **SL-T (target security level)**. |
| ZCR 6 | **Document cybersecurity requirements, assumptions & constraints** | Capture SL-T and the security requirements per zone/conduit. *(Source of the declared policy.)* |
| ZCR 7 | Asset owner approval | Sign-off. |

Sources: [IEC 62443-3-2 preview (ANSI)](https://webstore.ansi.org/preview-pages/IEC/preview_iec62443-3-2%7Bed1.0%7Den.pdf),
[ISA GCA white paper — Leveraging ISA 62443-3-2](https://isaorgwebsite.blob.core.windows.net/media/isa/media/pdf/isagca/gca-leveraging-isa62443-7-wht-paper_fin.pdf),
[Medium — Practical Guide to ICS/OT Risk Assessment using IEC 62443-3-2](https://medium.com/@sathish95/a-practical-guide-to-risk-assessment-in-ics-ot-using-iec-62443-3-2-c3fb43471e67).

> **Key scoping takeaway:** ZCR3 (partition into zones/conduits) + ZCR6 (documented requirements,
> incl. SL-T) **are** the declared policy. The SaC tool consumes that artifact and checks observed
> flows for conformance. ZCR1–2, ZCR4–5, ZCR7 are out of scope (they produce the policy; the tool
> does not).

### 2.3 Security Levels and the SL Vector

- **Security Levels (SL 1–4):** SL 1 = protection against casual/coincidental violation;
  SL 2 = intentional violation with simple means/low resources; SL 3 = sophisticated means/moderate
  resources (sector-specific skills); SL 4 = sophisticated means/extended resources (nation-state).
- **SL-T (target SL):** The desired security level allocated to a zone or conduit, derived in ZCR5.
  Other variants exist (SL-C capability, SL-A achieved) but the **policy declares SL-T**.
- **SL vector:** A zone's/conduit's SL is expressed as a **7-element vector**, one element per
  Foundational Requirement, e.g. `SL = (3,3,3,1,2,1,3)`.

**The 7 Foundational Requirements (FRs):**

| # | FR | Abbrev |
|---|----|--------|
| 1 | Identification & Authentication Control | IAC |
| 2 | Use Control | UC |
| 3 | System Integrity | SI |
| 4 | Data Confidentiality | DC |
| 5 | Restricted Data Flow | RDF |
| 6 | Timely Response to Events | TRE |
| 7 | Resource Availability | RA |

Sources: [Wikipedia — IEC 62443](https://en.wikipedia.org/wiki/IEC_62443),
[Cisco — ISA/IEC 62443-3-3](https://www.cisco.com/c/en/us/products/collateral/security/isaiec-62443-3-3-wp.html),
[Emerson/SPE — Foundational Requirements of IEC 62443](https://www.spe-aberdeen.org/wp-content/uploads/2019/11/0930_3_Foundational-Reqmts-IEC62443-Emerson.pdf).

> **Relevance to FR 5 (RDF):** Restricted Data Flow is the foundational requirement that
> conduit/zone segmentation most directly satisfies — the SaC tool is, in effect, an automated
> FR5 conformance checker. The other six FRs are about controls (auth, integrity, etc.) the tool
> cannot observe from flows alone.

### 2.4 Purdue Model → Zone Mapping

| Purdue Level | Name | Typical assets | Typical zone role |
|---|---|---|---|
| 0 | Physical Process | Sensors, actuators, drives | Process zone(s) |
| 1 | Intelligent Devices / Basic Control | PLCs, RTUs, IEDs | Control zone(s) |
| 2 | Supervisory Control | HMIs, SCADA, engineering workstations | Supervisory zone(s) |
| 3 | Site Operations | MES/MOMS, historians, batch mgmt | Operations zone |
| **3.5** | **Industrial DMZ (IDMZ)** | Jump hosts, proxies, replicated historian, patch/AV servers | **Conduit/DMZ zone** — mandatory broker between OT and IT |
| 4 | Site Business / Enterprise | ERP, business apps, site IT | IT zone |
| 5 | Enterprise / Cloud | Corporate IT, cloud, business planning | Enterprise zone |

Sources: [Palo Alto — Purdue Model for ICS Security](https://www.paloaltonetworks.com/cyberpedia/what-is-the-purdue-model-for-ics-security),
[SentinelOne — What is the Purdue Model](https://www.sentinelone.com/cybersecurity-101/cybersecurity/what-is-the-purdue-model/),
[Check Point — Purdue Model](https://www.checkpoint.com/cyber-hub/network-security/what-is-industrial-control-systems-ics-security/purdue-model-for-ics-security/).

The Level **3.5 IDMZ is the canonical conduit**: the rule "no flow may cross directly between
Level 3 (OT) and Level 4 (IT) — it must terminate in 3.5" is one of the highest-value built-in
checks the tool can ship. A flow from a Level-1 PLC directly to a Level-4 historian on :502 is the
textbook conduit violation.

### 2.5 Practical Minimum Subset to Model

The tool should **not** model the full SL vector or risk-assessment math in v1. Recommended minimum:

**Zone (illustrative YAML):**
```yaml
zones:
  - id: zone-control-a          # stable identifier
    name: "Control Network A"
    purdue_level: 1             # 0,1,2,3,3.5,4,5
    sl_t: 3                     # optional scalar SL-T (0-4); full vector deferred
    members:                    # how flow endpoints map into this zone
      - cidr: 10.10.1.0/24
      - host: 10.10.1.50
```

**Conduit (illustrative YAML):**
```yaml
conduits:
  - id: conduit-scada-to-control
    name: "SCADA to PLCs"
    from_zone: zone-supervisory
    to_zone: zone-control-a
    direction: unidirectional   # or bidirectional
    allow:
      - service: modbus         # symbolic service name (preferred)
        proto: tcp
        ports: [502]
      - service: s7comm
        proto: tcp
        ports: [102]
policy:
  default: deny                 # deny-by-default; any inter-zone flow not in a conduit = violation
```

This captures the practical essence: zones with `id/name/purdue_level/SL-T`, conduits as allowed
inter-zone channels with `proto/port/service` constraints, and **deny-by-default**. The SL vector
and FR breakdown can be added later as optional metadata without changing the validation core.

---

## Section 3 — Data Ingestion / Flow Schema

### 3.1 Zeek `conn.log` Field Schema

| Field | Type | Meaning |
|---|---|---|
| `ts` | time | Connection start (epoch). |
| `uid` | string | Unique connection ID for cross-log correlation. |
| `id.orig_h` | addr | Originator (source) IP. |
| `id.orig_p` | port | Originator (source) port. |
| `id.resp_h` | addr | Responder (destination) IP. |
| `id.resp_p` | port | Responder (destination) port. |
| `proto` | enum | Transport proto (`tcp`,`udp`,`icmp`). |
| `service` | string | App-layer service identified by Zeek's DPD (`http`,`dns`,`modbus`,…) — may be empty. |
| `duration` | interval | Connection length (s). |
| `orig_bytes` | count | App-layer payload bytes from originator. |
| `resp_bytes` | count | App-layer payload bytes from responder. |
| `conn_state` | string | Connection summary state (see below). |
| `local_orig` | bool | Originator is local (optional). |
| `local_resp` | bool | Responder is local (optional). |
| `missed_bytes` | count | Bytes missed (gaps in capture). |
| `history` | string | Event sequence; CAPS=originator, lowercase=responder (S=SYN, h=SYN-ACK, A=ACK, D=data, F=FIN, R=RST…). |
| `orig_pkts` | count | IP packets from originator. |
| `orig_ip_bytes` | count | IP-layer bytes from originator. |
| `resp_pkts` | count | IP packets from responder. |
| `resp_ip_bytes` | count | IP-layer bytes from responder. |
| `ip_proto` | count | IANA IP protocol number (newer Zeek). |

Source: [Book of Zeek — conn.log (LTS 8.0.6)](https://docs.zeek.org/en/lts/logs/conn.html),
[zeek-docs main.zeek.rst](https://github.com/zeek/zeek-docs/blob/master/scripts/base/protocols/conn/main.zeek.rst).

**`conn_state` value set:**

| Value | Meaning |
|---|---|
| `S0` | Connection attempt seen, no reply. |
| `S1` | Established, not terminated. |
| `SF` | Normal establishment and termination (byte counts present). |
| `REJ` | Connection attempt rejected. |
| `S2` | Established + originator close attempt, no responder reply. |
| `S3` | Established + responder close attempt, no originator reply. |
| `RSTO` | Established, originator aborted (sent RST). |
| `RSTR` | Responder sent RST. |
| `RSTOS0` | Originator sent SYN then RST; no SYN-ACK seen. |
| `RSTRH` | Responder sent SYN-ACK then RST; no originator SYN seen. |
| `SH` | Originator sent SYN then FIN; no responder SYN-ACK ("half" open). |
| `SHR` | Responder sent SYN-ACK then FIN; no originator SYN. |
| `OTH` | No SYN seen, midstream traffic ("other"). |

Source: [Book of Zeek — conn.log](https://docs.zeek.org/en/lts/logs/conn.html) (Conn::Info type),
[zeek_conn_states reference](https://rdrr.io/github/hrbrmstr/hrbrmisc/man/zeek_conn_states.html).

> **Why `conn_state` matters for SaC:** `S0`/`REJ`/`RSTO` flows represent *attempted-but-blocked*
> connections. A blocked attempt across a non-conduit is *evidence the existing firewall is working*,
> while an `SF` flow across a non-conduit is a *confirmed* violation. The tool can grade violation
> severity by `conn_state` (attempted vs. established).

### 3.2 NetFlow v5/v9 and IPFIX Basics (for the adapter)

- **NetFlow v5:** Fixed-format records; carries the classic 5-tuple (`src/dst IP`, `src/dst port`,
  `proto`), plus packet/byte counts, flow start/end (`SysUptime`-relative), TCP flags, ToS, AS/iface.
- **NetFlow v9:** Template-based, extensible; IE definitions are forward-compatible with IPFIX.
- **IPFIX (RFC 7011; info model RFC 5102/7012):** IETF standardization of NetFlow v9; protocol
  version 10. ~500 standardized Information Elements registered with IANA.

Key IANA IPFIX Information Elements relevant to a Flow record:

| IE name | Maps to |
|---|---|
| `sourceIPv4Address` / `sourceIPv6Address` | src IP |
| `destinationIPv4Address` / `destinationIPv6Address` | dst IP |
| `sourceTransportPort` | src port |
| `destinationTransportPort` | dst port |
| `protocolIdentifier` | IP proto number |
| `octetDeltaCount` / `octetTotalCount` | byte count |
| `packetDeltaCount` / `packetTotalCount` | packet count |
| `flowStartMilliseconds` / `flowEndMilliseconds` | start/end time |
| `tcpControlBits` | TCP flags |

Sources: [RFC 5102 — IPFIX Information Model](https://www.rfc-editor.org/rfc/rfc5102),
[IANA IPFIX registry](https://www.iana.org/assignments/ipfix/ipfix.xhtml),
[nProbe Flow Information Elements](https://www.ntop.org/guides/nprobe/flow_information_elements.html).

> Note: NetFlow/IPFIX **do not carry an app-layer `service` field** the way Zeek does. The adapter
> must derive `service` from port heuristics (Section 3.4) when ingesting flow telemetry. This is a
> structural reason to keep a neutral Flow record where `service` is *optional/derived*.

### 3.3 Recommendation: Neutral Flow Record + Adapter Layer (NOT raw Zeek)

**Recommendation: ADOPT a neutral normalized `Flow` record with an adapter layer. Do NOT bind the
core to Zeek's `conn.log` schema.**

Rationale / tradeoffs:

| Option | Pros | Cons |
|---|---|---|
| **Adopt Zeek `conn.log` directly** | Fastest v1; rich fields (`service`, `conn_state`, `history`); Zeek is the de-facto OT IDS sensor (Corelight, Security Onion). | **Vendor lock-in** to Zeek's schema; can't ingest NetFlow/IPFIX, firewall logs, or pcap-derived flows without contortion; couples the validation engine to one tool's evolving field set. |
| **Neutral `Flow` + adapters** | Decouples core from sources; supports Zeek **and** NetFlow/IPFIX **and** future firewall-log adapter (the brainstorm's v2); clean seam for fuzzing/unit tests; matches the "pluggable RealitySource" decision in the brief. | Slightly more upfront design; must define a careful normalization (esp. `service` derivation for NetFlow). |

The neutral record is the right call: it directly serves the brief's "pluggable reality adapter"
goal and avoids tying the Rust validation engine (the Kani-provable core) to any one telemetry
vendor.

**Proposed minimal neutral `Flow` schema:**

| Field | Type (Rust-ish) | Required | Source mapping |
|---|---|---|---|
| `start_ts` | `OffsetDateTime` | yes | Zeek `ts` / IPFIX `flowStartMilliseconds` / NetFlow start |
| `duration` | `Option<Duration>` | no | Zeek `duration` / derived end−start |
| `src_ip` | `IpAddr` | yes | Zeek `id.orig_h` / IPFIX `sourceIPv4/6Address` |
| `src_port` | `Option<u16>` | no | Zeek `id.orig_p` / IPFIX `sourceTransportPort` |
| `dst_ip` | `IpAddr` | yes | Zeek `id.resp_h` / IPFIX `destinationIPv4/6Address` |
| `dst_port` | `Option<u16>` | no | Zeek `id.resp_p` / IPFIX `destinationTransportPort` |
| `proto` | `Proto` (tcp/udp/icmp/other(u8)) | yes | Zeek `proto` / IPFIX `protocolIdentifier` |
| `service` | `Option<String>` | no | Zeek `service` (direct) / derived from port table for NetFlow |
| `service_source` | `enum {SensorDpi, PortHeuristic, Unknown}` | yes | Provenance of `service` — critical for confidence (see pitfalls) |
| `orig_bytes` | `Option<u64>` | no | Zeek `orig_bytes` / IPFIX octet counts |
| `resp_bytes` | `Option<u64>` | no | Zeek `resp_bytes` |
| `orig_pkts` | `Option<u64>` | no | Zeek `orig_pkts` / IPFIX packet counts |
| `resp_pkts` | `Option<u64>` | no | Zeek `resp_pkts` |
| `conn_state` | `Option<ConnState>` | no | Zeek `conn_state`; absent for NetFlow (use `tcpControlBits` if mapping) |
| `raw_uid` | `Option<String>` | no | Zeek `uid` for traceback |

The `service_source` discriminator is the design's safety valve: it lets the violation report say
"DPI-confirmed Modbus" vs. "assumed Modbus from port 502" and lets users gate strict mode.

### 3.4 OT/ICS Protocol → Port Seed Table

Confirmed/corrected from sources. **Port-based detection is heuristic** (Section 5).

| Protocol | Default port(s) | Transport | Notes |
|---|---|---|---|
| **Modbus TCP** | **502** | TCP | Confirmed. The canonical OT demo case. |
| **DNP3** | **20000** | TCP (also UDP) | Confirmed. |
| **EtherNet/IP (CIP)** | **44818** (explicit) | TCP & UDP | Confirmed. Request/response config/diagnostics; routes through firewalls like normal TCP. |
| **EtherNet/IP implicit I/O** | **2222** | **UDP** (often multicast) | Confirmed. Cyclic produced/consumed I/O at the RPI. Multicast — important: dst may be a multicast group, not a host. |
| **S7comm (Siemens)** | **102** | TCP | Confirmed. ISO-TSAP / COTP over TPKT. **Port 102 is shared with IEC 61850 MMS** — port alone cannot disambiguate. |
| **BACnet/IP** | **47808** (0xBAC0) | **UDP** | Confirmed. |
| **OPC UA (binary)** | **4840** | TCP (TLS variants) | Confirmed. |
| **IEC 60870-5-104** | **2404** | TCP | Confirmed. |
| **PROFINET (RT)** | (Layer-2, no IP port) / **34962–34964** for PNIO-CM/context mgmt | Ethernet / UDP | Real-time PROFINET is **Layer 2 (EtherType 0x8892)** and won't appear in IP flow logs; acyclic/context-mgmt uses UDP 34962–34964. *Model knowledge — verify.* |
| **FINS (Omron)** | **9600** | TCP & UDP | *Model knowledge — verify against vendor docs.* |
| **MQTT (IIoT)** | **1883** (8883 TLS) | TCP | Common in IIoT/edge; relevant for Level 3.5/4 broker patterns. |

Sources: [PacketViper — Known SCADA/ICS Network Ports](https://packetviper.com/scada-ics-network-ports/),
[Course Hero — ICS protocols cheat sheet](https://www.coursehero.com/file/203205896/ICS-protocols-cheat-sheet-v1-0pdf/),
[scadaprotocols.com — CIP Protocol Ports 44818 and 2222](https://scadaprotocols.com/cip-protocol-ports/),
[Software Toolbox — What is EtherNet/IP](https://softwaretoolbox.com/resources/what-is-ethernetip),
[Wikipedia — DNP3](https://en.wikipedia.org/wiki/DNP3).

Ship this as **seed reference data** (e.g. an embedded TOML/YAML the user can extend), not
hard-coded logic.

---

## Section 4 — Common Pitfalls & Mitigations

| Pitfall | Why it bites | Mitigation in the tool |
|---|---|---|
| **Port-based `service` detection is wrong** | Protocols run on non-default ports; ports are shared (102 = S7comm *or* IEC 61850 MMS); EtherNet/IP implicit I/O is multicast UDP; encapsulation/tunneling hides the real protocol. Zeek's own DPD exists precisely because port→protocol is unreliable. ([Zeek DPD](https://old.zeek.org/development/howtos/dpd.html)) | Track `service_source` (DPI vs. port-heuristic). Prefer Zeek's `service` field when present. Offer a "strict" mode that treats port-only matches as lower-confidence. Report confidence per violation. |
| **SL vector complexity** | Modeling the full 7-element SL vector and FR semantics in v1 is heavy and not needed for flow conformance (only FR5/RDF is observable from flows). | Model SL-T as an optional scalar (0–4) per zone/conduit; allow an optional full vector as metadata; do not block validation on it. |
| **Multicast / broadcast endpoints** | EtherNet/IP implicit I/O and BACnet use UDP multicast/broadcast — "dst host" may be a group, breaking naive zone membership lookups. | Zone membership matcher must handle multicast/broadcast ranges and "any-in-zone" semantics, or explicitly classify multicast flows. |
| **Asymmetric/unidirectional flow data** | NetFlow often records one direction; Zeek pairs both. Direction matters for unidirectional conduits. | Neutral Flow keeps orig/resp distinction; document that NetFlow may need bidirectional stitching; allow conduits to be direction-agnostic when data is unidirectional. |
| **IP→zone mapping ambiguity** | DHCP, NAT, overlapping CIDRs, devices that move zones. A flow's IP may match multiple zones or none. | Define deterministic, longest-prefix / explicit-host-wins resolution; emit an "unmapped endpoint" finding rather than silently dropping. |
| **Treating blocked attempts as violations** | An `S0`/`REJ` flow across a non-conduit means the control *worked*. | Grade by `conn_state`: established (`SF`,`S1`) = confirmed violation; rejected/aborted = informational/positive evidence. |
| **Layer-2 protocols invisible in flow logs** | PROFINET RT, PROFINET IRT, GOOSE are Ethernet-layer; never appear in IP flow exports. | Document scope limit: the tool validates **routable/IP** segmentation; L2 segmentation needs pcap/L2 telemetry (out of v1 scope). |
| **Standard/version drift** | IEC 62443 editions, IANA assignments, Zeek field sets evolve. | Keep port table and conn_state set as data files with a version stamp; cite source + date; re-verify before release. |

---

## Recommended Technical Decisions

1. **Adopt a neutral `Flow` record + adapter layer — YES.** Do not couple the core to Zeek's
   `conn.log`. First adapter = Zeek TSV/JSON; second adapter = NetFlow/IPFIX; later = firewall logs.
   Track `service_source` provenance on every flow.
2. **Minimal zone/conduit model to implement (v1):**
   - Zone: `id`, `name`, `purdue_level` (0/1/2/3/3.5/4/5), optional scalar `sl_t` (0–4),
     `members` (CIDRs/hosts). Optional full 7-element SL vector as deferred metadata.
   - Conduit: `id`, `name`, `from_zone`, `to_zone`, `direction`, `allow[]` of
     `{service, proto, ports}`.
   - Global `policy.default: deny` (deny-by-default; any inter-zone flow not matched by a conduit
     is a violation).
3. **Ship the OT protocol/port table as seed data** (extensible TOML/YAML, version-stamped), not
   hard-coded — so users can add site-specific ports.
4. **Built-in high-value check:** enforce the **Purdue 3.5 IDMZ** rule (no direct OT↔IT flow that
   skips Level 3.5). This is the headline demo violation.
5. **Grade violations by `conn_state`** (established vs. attempted-and-blocked) and by
   `service_source` confidence (DPI vs. port-heuristic).
6. **Policy format = YAML now (serde), DSL later** — consistent with the brainstorm decision; the
   neutral model is the compile target for both.
7. **Scope guardrail:** validate **IP/routable** segmentation only in v1; flag L2-only protocols
   (PROFINET RT, GOOSE) as out of scope; do **not** implement IEC 62443-3-2 risk assessment
   (ZCR1–2, 4–5, 7) — consume the policy, don't compute it.

---

## Summary of Recommendations

- **Whitespace confirmed.** No open, declarative, git-versioned tool validates a **62443
  zone/conduit policy against observed flows**. Batfish (open, config-centric), OPA (open, no
  domain), and Claroty/Nozomi/Dragos (OT-deep, closed appliance) each cover a different corner. The
  SaC tool's combination of *open + declarative + observed-flow + 62443/Purdue-native +
  OT-protocol-aware + offline/CI* is unoccupied — a credible "Batfish/terraform-plan for OT
  segmentation."
- **Use a neutral `Flow` record + adapters**, not raw Zeek schema, with a `service_source`
  provenance discriminator.
- **Model a pragmatic 62443 subset:** zones (`id/name/purdue_level/sl_t/members`) + conduits
  (allowed inter-zone `service/proto/ports`) + deny-by-default. Defer the full SL vector; treat
  conformance as an FR5 (Restricted Data Flow) check.
- **Ship the OT port table as extensible seed data** and treat port→service as a *heuristic* with
  confidence, never ground truth.
- **Lead the demo with the Purdue 3.5 IDMZ bypass violation** and `conn_state`-graded severity.
- **Re-verify** standards editions, Zeek versions, and IANA/port assignments before locking the
  spec — all findings are as of 2026-06-17.

---

## References

- ISA/IEC 62443-3-2 preview — https://webstore.ansi.org/preview-pages/IEC/preview_iec62443-3-2%7Bed1.0%7Den.pdf
- EN IEC 62443-3-2:2020 catalog — https://standards.iteh.ai/catalog/standards/clc/d1892b9c-d2e4-4e2f-8639-350ec364387e/en-iec-62443-3-2-2020
- ISA GCA — Leveraging ISA 62443-3-2 — https://isaorgwebsite.blob.core.windows.net/media/isa/media/pdf/isagca/gca-leveraging-isa62443-7-wht-paper_fin.pdf
- Novesh — 62443-3-2 Zones, Conduits, Risk Assessments — https://novesh.com/blog/novesh-blog-7/understanding-iec-62443-3-2-zones-conduits-and-risk-assessments-27
- MDPI — Security Aspects of Zones and Conduits — https://www.mdpi.com/2624-800X/6/2/52
- Medium (Sathish) — Practical Guide to ICS/OT Risk Assessment — https://medium.com/@sathish95/a-practical-guide-to-risk-assessment-in-ics-ot-using-iec-62443-3-2-c3fb43471e67
- Wikipedia — IEC 62443 — https://en.wikipedia.org/wiki/IEC_62443
- Cisco — ISA/IEC 62443-3-3 — https://www.cisco.com/c/en/us/products/collateral/security/isaiec-62443-3-3-wp.html
- Emerson/SPE — Foundational Requirements — https://www.spe-aberdeen.org/wp-content/uploads/2019/11/0930_3_Foundational-Reqmts-IEC62443-Emerson.pdf
- Palo Alto — Purdue Model — https://www.paloaltonetworks.com/cyberpedia/what-is-the-purdue-model-for-ics-security
- SentinelOne — Purdue Model — https://www.sentinelone.com/cybersecurity-101/cybersecurity/what-is-the-purdue-model/
- Check Point — Purdue Model — https://www.checkpoint.com/cyber-hub/network-security/what-is-industrial-control-systems-ics-security/purdue-model-for-ics-security/
- Book of Zeek — conn.log — https://docs.zeek.org/en/lts/logs/conn.html
- zeek-docs conn main.zeek — https://github.com/zeek/zeek-docs/blob/master/scripts/base/protocols/conn/main.zeek.rst
- zeek_conn_states reference — https://rdrr.io/github/hrbrmstr/hrbrmisc/man/zeek_conn_states.html
- Zeek Dynamic Protocol Detection — https://old.zeek.org/development/howtos/dpd.html
- RFC 5102 — IPFIX Information Model — https://www.rfc-editor.org/rfc/rfc5102
- IANA IPFIX registry — https://www.iana.org/assignments/ipfix/ipfix.xhtml
- nProbe Flow Information Elements — https://www.ntop.org/guides/nprobe/flow_information_elements.html
- PacketViper — Known SCADA/ICS Network Ports — https://packetviper.com/scada-ics-network-ports/
- scadaprotocols.com — CIP Protocol Ports 44818/2222 — https://scadaprotocols.com/cip-protocol-ports/
- Software Toolbox — What is EtherNet/IP — https://softwaretoolbox.com/resources/what-is-ethernetip
- Wikipedia — DNP3 — https://en.wikipedia.org/wiki/DNP3
- Batfish — https://batfish.org/ , https://github.com/batfish/batfish , https://batfish.readthedocs.io/en/latest/notebooks/linked/analyzing-acls-and-firewall-rules.html
- FireMon vs AlgoSec vs Tufin — https://www.firemon.com/firemon-vs-algosec-vs-tufin/
- NSPM solutions overview — https://aimultiple.com/network-security-policy-management-solutions
- Tufin AlgoSec alternative — https://www.tufin.com/why-tufin/algosec-alternative
- Elisity — Top microsegmentation 2026 — https://www.elisity.com/blog/what-are-the-top-microsegmentation-solutions-for-2026
- Elisity — Leading OT/ICS vendors 2026 — https://www.elisity.com/blog/leading-vendors-for-securing-ot-and-industrial-control-systems-in-2026
- Akamai Guardicore Segmentation — https://www.akamai.com/products/akamai-guardicore-segmentation
- Cybersecurity Magazine — Top 10 OT Security Platforms — https://cybermagazine.com/top10/top-10-ot-security-platforms
- IoT Security Institute — Claroty/Nozomi/Dragos comparison — https://iotsecurityinstitute.com/iotsec/iot-security-institute-cyber-security-articles/247-navigating-the-ot-security-landscape-a-comparison-of-claroty,-nozomi-networks,-and-dragos
- Open Policy Agent docs — https://www.openpolicyagent.org/docs , https://www.openpolicyagent.org/docs/policy-language
- Apiiro — What is Policy-as-Code — https://apiiro.com/glossary/policy-as-code-2/

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| WebSearch | 9 | IEC 62443-3-2 ZCR steps & risk workflow; SL vector & 7 FRs; Zeek conn.log fields & conn_state; Purdue model levels; NetFlow/IPFIX information elements; OT protocol ports; EtherNet/IP CIP ports; Batfish; OT platforms (Claroty/Nozomi/Dragos/Forescout); microsegmentation (Illumio/Guardicore/Elisity); OPA/Sentinel PaC; FireMon/AlgoSec/Tufin NSPM; Zeek DPD vs port detection. |
| WebFetch | 1 | Authoritative Book of Zeek conn.log field/type table and conn_state value set. |
| Read | 2 | Local brainstorming report (selected direction & scoping decisions); attempted product-brief & research index (not yet present). |
| Glob | 1 | Locate existing `.factory` artifacts. |
| Perplexity (search/reason/deep_research) | 0 | MCP Perplexity tools were not available in this environment. |
| Context7 | 0 | MCP Context7 not invoked — no library-version verification was in scope (research is domain, not package selection). |
| Training data | ~3 areas | FINS port 9600, PROFINET context-mgmt UDP 34962–34964, and some SL-level wording came partly from model knowledge; each is flagged inline as "verify." |

**Total external tool calls:** 13 (9 WebSearch + 1 WebFetch + 3 local Read/Glob).
**Training data reliance:** low–medium — the substantive standards, schema, port, and tool claims
are web-cited; only the explicitly flagged port details (FINS, PROFINET context-mgmt) and SL-level
phrasing lean on model knowledge and are marked for verification.

**Limitations / inconclusive items:** (a) The exact `conn_state` definitions for `RSTRH`, `SH`,
`SHR`, `OTH` are listed from the Zeek value set; full per-state prose should be confirmed against the
`Conn::Info` type reference. (b) IEC 62443-3-2 text is paywalled; ZCR step names/order are
corroborated across three secondary sources but should be checked against the purchased standard.
(c) FINS and PROFINET port specifics are model-knowledge and flagged. (d) Perplexity/Context7 MCP
tools were unavailable, so cross-referencing relied on multiple independent web sources instead.
