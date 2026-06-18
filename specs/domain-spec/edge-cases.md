---
document_type: domain-spec-section
level: L2
section: edge-cases
version: "1.8"
status: draft
producer: business-analyst
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[live-state]"
traces_to: L2-INDEX.md
---

# Domain-Level Edge Cases

| ID | Capability | Edge Case | Expected Behavior |
|----|-----------|-----------|-------------------|
| DEC-001 | CAP-005 | Endpoint IP matches multiple zones (nested CIDRs) | Longest-prefix wins (DI-004); resolves to the most specific zone |
| DEC-002 | CAP-002 | Two zones declare the *same* prefix (equal-length tie) | Policy validation error (DI-010); reject at load |
| DEC-003 | CAP-005 | Endpoint IP matches no zone (e.g. `8.8.8.8`) | Resolve to implicit `EXTERNAL` zone (DI-005) |
| DEC-004 | CAP-007 | Both endpoints in the same zone | Verdict `IntraZone`, allowed (DI-002) |
| DEC-005 | CAP-008 | Flow between managed L1↔L4 *explicitly allowed by a conduit* but with no IDMZ endpoint | `VerdictKind = Allowed` AND `idmz_bypass = true` (additive, DI-006). The flow appears in the `allowed` tally **and** as a `Violation{IdmzBypass}`; counted once in `distinct_violating_flows` |
| DEC-006 | CAP-006 | Allowed conduit `A→B` but flow initiated B→A | `Violation{WrongDirection}` unless conduit is `Bidirectional` (DI-007) |
| DEC-007 | CAP-004 | Known OT service on a non-default port (e.g. Modbus on 1502) | `service_source = Unknown` (non-default port is never `PortHeuristic`); do not assert the service authoritatively |
| DEC-008 | CAP-004 | Service runs on its default port but transport mismatches (e.g. BACnet expects UDP, flow is TCP) | Service inference must consider proto; mismatch → `Unknown` |
| DEC-009 | CAP-006 | Conduit `ports = Any` or a port range | Match accordingly; range bounds inclusive |
| DEC-010 | CAP-003 | Malformed / truncated `conn.log` line | Skip line, increment `skipped` counter; never abort the run |
| DEC-011 | CAP-004 | Flow with missing/zero port or missing proto | Treat as `Unknown` service; still resolve zones & classify on what's present |
| DEC-012 | CAP-005 | IPv6 flows / mixed v4+v6 policy; **IPv4-mapped IPv6** (`::ffff:a.b.c.d`) | Family-aware longest-prefix; IPv4-mapped addresses are **canonicalized to IPv4 before resolution** (so they match IPv4 matchers and count as the IPv4 family for tie scoping) |
| DEC-013 | CAP-005 | Loopback / link-local destination | Resolve via longest-prefix (likely `EXTERNAL` unless a zone claims them); documented, not special-cased silently |
| DEC-014 | CAP-009 | Empty flow input (zero flows) | Valid run: `ConformanceResult` with all-zero tallies, exit 0; not an error |
| DEC-015 | CAP-006 | Multiple conduits match the same flow, or duplicate/overlapping conduits between a zone-pair (e.g. `502` and range `500-510`, or `Forward` plus `Bidirectional`) | Any-match union (DI-014): flow is `Allowed` if ≥1 conduit permits it; overlaps are legal, not a load-time error |
| DEC-016 | CAP-003 | A Zeek `conn.log` line represents a full bidirectional connection (orig/resp) | One connection record = exactly **one** `Flow`, keyed on the originator (`src` = initiator). Return/responder traffic is implicit and NOT treated as a separate `B→A` flow — avoids false `WrongDirection` violations (DI-007) |
| DEC-017 | CAP-008 | Internal managed asset (≤L3) → unmatched/EXTERNAL IP (e.g. Internet egress) | NOT an `IdmzBypass` (EXTERNAL is excluded from DI-006). Governed by conduit matching: `Violation{NoMatchingConduit}` unless a conduit permits the egress |
| DEC-018 | CAP-002 | A declared (non-EXTERNAL) zone with zero members | Legal but inert (a "dead" zone); emit a load-time **warning**, not an error |
| DEC-019 | CAP-007 | Policy with zones but **zero conduits** | Legal: strict deny-all. Every cross-zone flow is `NoMatchingConduit`; intra-zone flows still allowed (DI-002) |
| DEC-020 | CAP-006 | Conduit with `EXTERNAL` as `from` (ingress from Internet) | Legal; models permitted inbound flows. Matched like any conduit per DI-007/DI-014 |
| DEC-021 | CAP-006 | Portless protocol flow (ICMP/`Other`) crossing a zone boundary | Matches a conduit **only** if its `ports: Any`; a conduit with explicit ports never matches a portless flow. So ICMP liveness across the IDMZ requires an `Any`-port conduit, else `NoMatchingConduit` |
| DEC-022 | CAP-002 | Two zones with disjoint same-length CIDRs (e.g. `10.0.0.0/24` vs `10.0.1.0/24`) | **Legal** — not a tie (DI-010 ties are per-family, per common-address). No load error |
| DEC-023 | CAP-005 | A zone's host `/32` equals another zone's network or broadcast address | Legal (different prefix lengths → longest-prefix `/32` host wins); resolves deterministically to the host's zone |
| DEC-024 | CAP-003 | Clean run (zero violations) but some flow lines were skipped (`skipped > 0`) | Exit `0` **with a mandatory warning** (DI-013/DI-019); `--fail-on-skipped` instead makes `skipped > 0` a non-zero exit for strict CI |
| DEC-025 | CAP-007 | **Multicast / broadcast destination** (EtherNet/IP implicit I/O, BACnet) | `MatchKind::MulticastBroadcast` → verdict `MulticastExempt` (DI-016); never `NoMatchingConduit`. Avoids false positives on dominant cyclic OT I/O |
| DEC-026 | CAP-009 | Flow with one EXTERNAL endpoint vs flow with both endpoints EXTERNAL | `external_endpoints` increments by **one per flow** with ≥1 EXTERNAL endpoint (both-EXTERNAL counts once); it is diagnostic and excluded from the DI-015 accounting identity |
| DEC-027 | CAP-007 | Intra-zone flow whose destination is a multicast/broadcast address | `MulticastExempt` wins — precedence is the ST-6 order, so multicast short-circuits before `IntraZone` (DI-016) |
| DEC-028 | CAP-001 | Duplicate YAML mapping keys in the policy file | Load-time **error** (no silent serde last-wins) — protects the digest-trust model (DI-010) |
| DEC-029 | CAP-002 | A zone declares a `0.0.0.0/0` or `::/0` member | Load-time **error** (DI-010): a /0 catch-all would shadow the implicit EXTERNAL zone and suppress IDMZ-EXTERNAL exclusions |
| DEC-030 | CAP-005 | Dst IPv4 == the directed-broadcast (all-ones host) of its longest-prefix-matched zone with prefix ≤ /30 (e.g. `10.0.0.255` for `10.0.0.0/24`) | Step-2 override (DI-016): `Explicit` → `MulticastBroadcast` → `MulticastExempt`. (IPv4 only; **excluded for /31 and /32** — a single-host dst is NOT exempted; IPv6 never takes this branch) |
| DEC-031 | CAP-007 | A declared zone's CIDR covers multicast space (e.g. someone declares `224.0.0.0/4`) | Step-1 family-wide multicast test runs **before** zone resolution, so a multicast dst is always `MulticastExempt` even if a zone declares the range (documented) |
| DEC-032 | CAP-007 | Both endpoints resolve to the reserved `EXTERNAL` zone (e.g. `8.8.8.8`→`1.1.1.1`) | `IntraZone` — EXTERNAL is a single zone (DI-002); Internet↔Internet transit is out of scope, not a violation |
| DEC-033 | CAP-003 | Flow with the **unspecified address** `0.0.0.0` or `::` as src **or** dst (e.g. DHCP-discover, spoofed source) | **Skipped + counted + warned** as a malformed flow (DI-013) — a deterministic rule (not "likely EXTERNAL"); an unspecified endpoint is meaningless for segmentation and must never produce a verdict |
