---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [domain-spec/L2-INDEX.md]
input-hash: "[live-state]"
traces_to: domain-spec/L2-INDEX.md
origin: greenfield
subsystem: "SS-03"
capability: "CAP-005"
lifecycle_status: active
introduced: v0.1.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.03.001: Resolve endpoint to exactly one zone via longest-prefix match

## Description

Each flow endpoint IP address is resolved to exactly one zone by finding the zone whose declared `AssetMatcher` has the longest matching prefix length. This implements the deterministic overlap resolution rule (DI-004). After tie detection at policy load time (BC-1.01.005) ensures no ambiguous overlaps exist, the longest-prefix algorithm is total and unique. The resolution index is built during ST-2 (policy validation) and is a pure lookup at ST-5. If no declared matcher matches the endpoint, the flow falls through to BC-1.03.002 (implicit EXTERNAL). Note: known limitation — IP/CIDR zone membership is not reliable under NAT or VLAN-only zoning (ASM-008).

## Preconditions

1. Policy is validated (BC-1.01.004 through BC-1.01.007 passed); no ties exist in the resolver index.
2. The endpoint IP is not an unspecified address (BC-1.02.003 already filtered those).
3. IPv4-mapped IPv6 addresses have been canonicalized (BC-1.02.005).
4. The endpoint IP is not a multicast/broadcast address (BC-1.03.003 handles those before this step).

## Postconditions

1. A `ResolvedEndpoint` is produced with:
   - `ip`: the endpoint IP.
   - `zone_id`: the ID of the matched zone.
   - `match: MatchKind::Explicit { prefix_len }` where `prefix_len` is the length of the winning matcher.
2. Resolution is deterministic: identical IP + policy always produce the same `ResolvedEndpoint`.
3. If no matcher matches the endpoint → BC-1.03.002 (EXTERNAL) is applied instead.

## Invariants

1. Resolution is total: every non-unspecified, non-multicast endpoint produces exactly one `ResolvedEndpoint` (DI-003).
2. When two matchers in different zones could match the endpoint (different prefix lengths), the longest prefix wins (DI-004).
3. The resolution is a pure function of (IP, validated Policy) — no mutable state, no I/O (DI-012).
4. An `IpAddr` matcher is stored as a /32 (IPv4) or /128 (IPv6) and participates in longest-prefix lookup identically to CIDRs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Zone A: `10.0.0.0/16`, Zone B: `10.0.1.0/24`; endpoint `10.0.1.5` | Resolves to Zone B (`/24` is longer than `/16`) (DEC-001) |
| EC-002 | Zone A: `10.0.0.5/32`, Zone B: `10.0.0.0/24`; endpoint `10.0.0.5` | Resolves to Zone A (`/32` longest-prefix host wins) (DEC-023) |
| EC-003 | Zone A: `10.0.0.0/24` only; endpoint `192.168.1.5` | No match → falls to BC-1.03.002 (EXTERNAL) (DEC-003) |
| EC-004 | Endpoint = `127.0.0.1` (loopback) | Resolves via longest-prefix; if no zone claims it → EXTERNAL (DEC-013) |
| EC-005 | Zone A: `0.0.0.0/8`; endpoint `0.0.0.1` | Resolves to Zone A (matches `/8`); no issue — `/0` is the only rejected prefix |
| EC-006 | IPv6 endpoint `2001:db8::1`; policy has IPv4-only zones | No match → EXTERNAL |
| EC-007 | Zone A: `10.0.0.0/24`; endpoint `10.0.0.0` (network address itself) | Resolves to Zone A (matches `/24`); receives normal verdict (OQ-005: network address is classified normally, not exempted) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Endpoint `10.0.1.5`; Zone A `10.0.0.0/16`, Zone B `10.0.1.0/24` | Resolves to Zone B; `MatchKind::Explicit { prefix_len: 24 }` | happy-path |
| Endpoint `10.0.0.5`; Zone A `10.0.0.5/32` | Resolves to Zone A; `MatchKind::Explicit { prefix_len: 32 }` | happy-path |
| Endpoint `192.168.1.1`; only Zone `10.0.0.0/24` | Falls to EXTERNAL (BC-1.03.002) | edge-case |
| Endpoint `10.0.0.0`; Zone `10.0.0.0/24` | Resolves to declared zone (normal verdict — no network-address exemption) | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.03.001-a | Resolution is total: every non-unspecified, non-multicast endpoint produces exactly one ResolvedEndpoint (DI-003) | kani |
| VP-1.03.001-b | Longest-prefix is unique given a validated (tie-free) policy (DI-004) | kani |
| VP-1.03.001-c | Resolution is a pure function: identical inputs → identical output | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-005 ("Resolve endpoints to zones: Map each flow endpoint IP to exactly one zone via longest-prefix match; unmatched → implicit EXTERNAL zone") per capabilities.md §CAP-005 |
| L2 Domain Invariants | DI-003 (total endpoint resolution ⊢), DI-004 (deterministic overlap resolution ⊢), DI-005 (implicit EXTERNAL zone) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-005 ("Resolve endpoints to zones") per capabilities.md §CAP-005 — this BC is the direct implementation of the longest-prefix-match resolution algorithm that CAP-005 defines |

## Related BCs

- BC-1.01.005 — tie detection (ensures the precondition: no ambiguous overlaps)
- BC-1.03.002 — implicit EXTERNAL fallback
- BC-1.03.003 — multicast/broadcast detection (runs before this BC)

## Architecture Anchors

- `architecture/SS-03-zone-resolution.md#longest-prefix-match` — resolution index and algorithm

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.03.001-a — total resolution (DI-003)
- VP-1.03.001-b — deterministic longest-prefix (DI-004)
- VP-1.03.001-c — pure function
