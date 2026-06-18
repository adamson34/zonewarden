---
document_type: domain-spec-section
level: L2
section: entities
version: "1.8"
status: draft
producer: business-analyst
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[live-state]"
traces_to: L2-INDEX.md
---

# Domain Entities

> The data structures flowing through the conformance pipeline. Implementation-independent.

| Entity | Description | Key Attributes | Invariants |
|--------|-------------|----------------|------------|
| **Policy** | The intended segmentation, declared as code | `zones: [Zone]`, `conduits: [Conduit]`, (implicit `deny_by_default = true`) | Aggregate root; zone ids unique; every Conduit endpoint references an existing zone or the reserved `EXTERNAL` zone (DI-010) |
| **Zone** | A group of assets sharing security requirements (IEC 62443) | `id`, `name`, `purdue_level: PurdueLevel`, `sl_t: Option<SlTarget>`, `members: [AssetMatcher]` | id unique; `members` must not produce equal-length prefix ties with another zone — ties **rejected at load** (DI-010); overlapping non-tie prefixes resolved longest-prefix (DI-004); the reserved `EXTERNAL` zone is membership-less and fixed at `purdue_level = L5` (DI-005) |
| **AssetMatcher** | How an asset is assigned to a zone | one of: `Cidr(v4/v6)`, `IpAddr`, (future: `Hostname`) | A single IP is a /32 (v4) or /128 (v6) CIDR — unifies matching as longest-prefix. A host /32 may coincide with another zone's network/broadcast address (see DEC-023) |
| **Conduit** | The only sanctioned channel between two zones | `from_zone`, `to_zone`, `direction: Direction`, `proto: Proto`, `ports: PortSet` | Endpoints exist; `ports` is non-empty as a *set value* (`Any` is a valid non-empty value); semantics per DI-007/DI-014. Portless protocols match only `ports: Any` (DEC-021) |
| **Flow** | One observed communication (normalized) | `flow_index: u64` (stable identity = dense, gap-free position `0..n` among **successfully-normalized** flows; skipped records do not consume an index — DI-013), `ts: Timestamp`, `src_ip`, `src_port: Option<u16>`, `dst_ip`, `dst_port: Option<u16>`, `proto: Proto`, `service: Option<Service>`, `service_source: ServiceSource`, `conn_state: Option<ConnState>` | `src` is the connection **initiator** (DEC-016); `service_source` always set (DI-008); `flow_index` is the stable per-flow identity used by DI-009/DI-015 |
| **Service** | App-layer protocol identity | `Modbus \| Dnp3 \| EtherNetIp \| S7comm \| Bacnet \| OpcUa \| Other(String)` | Distinct from transport `proto`; may be inferred (heuristic) or authoritative |
| **ResolvedEndpoint** | A flow endpoint after zone resolution | `ip`, `zone_id`, `match: MatchKind` | `MatchKind ∈ {Explicit{prefix_len}, ImplicitExternal, MulticastBroadcast}`; exactly one per endpoint (DI-003). A directed-broadcast dst is detected *after* zone resolution and overrides `Explicit` → `MulticastBroadcast` (DI-016) |
| **Verdict** | Per-flow classification (the mutually-exclusive conduit/zone outcome) | `flow_index`, `src_zone`, `dst_zone`, `kind: VerdictKind`, `idmz_bypass: bool` | `VerdictKind ∈ {IntraZone, Allowed(conduit_id), NoMatchingConduit, WrongDirection, MulticastExempt}` — **exactly one per resolved flow** (DI-015). `idmz_bypass` is an **independent additive** finding (DI-006), NOT a VerdictKind — a flow can be both `Allowed` and `idmz_bypass = true` |
| **Violation** | A reported breach (derived view over Verdicts) | `flow_index`, `src_zone`, `dst_zone`, `kind: ViolationKind`, `severity: Severity`, `explanation` | A flow is violating iff `kind ∈ {NoMatchingConduit, WrongDirection}` OR `idmz_bypass`. `ViolationKind ∈ {NoMatchingConduit, WrongDirection, IdmzBypass}`. One flow may yield up to two Violation entries (a conduit violation + an IdmzBypass) — `distinct_violating_flows` de-dups by `flow_index` |
| **ConformanceResult** | The full run output (aggregate) | `total_flows: u64`, `intra_zone: u64`, `allowed: u64`, `no_matching_conduit: u64`, `wrong_direction: u64`, `multicast_exempt: u64`, `idmz_bypasses: u64`, `distinct_violating_flows: u64`, `violations: Vec<Violation>`, `external_endpoints: u64` (count of **flows with ≥1 endpoint resolving to `EXTERNAL`**; both-EXTERNAL counts once — DEC-026; diagnostic only, outside the DI-015 identity), `skipped: u64`, `warnings: Vec<String>`, `policy_digest: String` (DI-018) | Deterministic, stable ordering (DI-009); accounting holds (DI-015): `total_flows == intra_zone + allowed + no_matching_conduit + wrong_direction + multicast_exempt`. `idmz_bypasses` is independent and may overlap `allowed`. `skipped` excluded from `total_flows`. All tallies and `flow_index` are **`u64`** (canonical, not platform `usize`); overflow handled per FM-009 (checked arithmetic → abort, not silent wrap); input bounded by the FM-008 `max_flows` ingest cap |
| **RealitySource** | The pluggable adapter seam (bounded-context boundary) | trait: `fn flows(&self) -> Iterator<Flow>` | Adapters translate vendor formats → `Flow`; the engine knows only `Flow` |

## Value Objects & Enums

- **PurdueLevel** — `L0, L1, L2, L3, IDMZ (3.5), L4, L5`. Totally ordered. `IDMZ` sits strictly between L3 and L4 and is, by definition, **neither ≤L3 nor ≥L4** for the DI-006 predicates. The reserved `EXTERNAL` zone is fixed at `L5` as a **sentinel**; a *declared, managed* L5 zone is a normal managed endpoint for DI-006 (see the DI-006 truth table).
- **SlTarget** — IEC 62443 Security Level Target. Optional scalar `0..=4` (overall target), with an optional full 7-element FR vector (`[u8; 7]`, each `0..=4`) as metadata. MVP uses only the scalar; FR5 is the focus.
- **Direction** — `Forward` (initiator in `from`) \| `Bidirectional`. **Canonical YAML tokens:** `forward` and `bidirectional`; `unidirectional` is accepted as an alias for `forward`. (CAP-001 / PRD pin the full grammar.)
- **Proto** — transport: `Tcp \| Udp \| Icmp \| Other(u8)`. `Icmp` and `Other` are **portless** (see PortSet / DEC-021).
- **PortSet** — `Any` \| a non-empty set of individual ports and/or inclusive ranges; canonical form per **DI-020** (sorted non-overlapping non-adjacent `[lo,hi]`, adjacents coalesced; `Any` distinct from `0-65535`). `Any` matches any port **and** matches portless flows; an explicit port set never matches a portless flow (DEC-021).
- **ServiceSource** — `DpiConfirmed \| PortHeuristic \| Unknown` (provenance of `service`). *(`DpiConfirmed` is the normative name; supersedes the research draft's `SensorDpi`.)*
- **ConnState** — connection state from the adapter (e.g. Zeek `conn_state`): `Established` (e.g. `SF`, and `RSTO`/`RSTR` — connection *was* established then reset) \| `Attempted` (e.g. `S0`, `REJ`, `SH` — handshake never completed) \| `Other(String)`. Drives `Severity`. **BACKLOG (PRD):** the complete 13-state Zeek `conn_state` → bucket mapping is pinned at the PRD/impl layer; the examples here fix the previously-wrong `RSTO` bucket.
- **Severity** — violation grading from `conn_state` (DI-017): `Established` (confirmed flow across a non-conduit) \| `Attempted` (blocked/rejected attempt — evidence the control held). `Severity` exists **only on `Violation` entries** (never computed/stored for non-violating verdicts). A violating flow with no `conn_state`, or `ConnState::Other(_)`, defaults to `Established` (conservative — never under-reports a breach).
- **Timestamp** — UTC instant at **nanosecond precision** (normalized; Zeek fractional-epoch seconds widened, not truncated). DI-009's order key uses the full-precision value.

### Canonical Service / Port / Transport Table (MVP)

> Used by CAP-004 service inference. Inference is **heuristic** (DI-008): a default `(port, transport)` match → `service_source = PortHeuristic`; any mismatch or non-default port → `Unknown`. MVP **intentionally pins a single `(port, transport)` per service**; multi-transport variants (e.g. DNP3-over-UDP, EtherNet/IP-over-UDP) resolve to `Unknown` by design (ASM-009), not mislabeled.

| Service | Default Port | Transport | Notes |
|---------|-------------|-----------|-------|
| Modbus | 502 | TCP | |
| DNP3 | 20000 | TCP | UDP/20000 variant → `Unknown` in MVP (ASM-009) |
| EtherNet/IP | 44818 | TCP | explicit messaging; implicit I/O (UDP 2222, multicast) → `MulticastExempt`/`Unknown` |
| S7comm | 102 | TCP | port shared with IEC 61850 MMS; MVP has no MMS detection signal, so 102/TCP always infers `S7comm` (`PortHeuristic`). Known ambiguity (ASM-009) |
| BACnet/IP | 47808 | UDP | often multicast/broadcast dst → `MulticastExempt` |
| OPC UA | 4840 | TCP | |

## Relationships & Aggregates

- `Policy` 1—* `Zone`; `Policy` 1—* `Conduit`. **Policy is the consistency boundary** — validated as a whole.
- `Conduit` references two `Zone`s (or `EXTERNAL`).
- `Flow` is independent input; resolution links it to two `Zone`s to produce exactly one `Verdict` (+ optional additive `idmz_bypass`).
- `RealitySource` is a *seam*: many implementations (Zeek=MVP, NetFlow=P1, firewall=P2), one `Flow` contract.
