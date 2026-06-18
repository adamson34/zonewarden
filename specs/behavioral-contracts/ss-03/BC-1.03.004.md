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

# BC-1.03.004: Directed-broadcast destination override: all-ones host of ≤/30 zone → MulticastBroadcast

## Description

This is DI-016 Step-2: after a destination IP has been resolved to a declared zone via longest-prefix match (Step-1 did not fire), if the matched zone's IPv4 prefix length is ≤ 30 AND the dst IP equals the all-ones host address of that zone's prefix (the directed broadcast), the `Explicit` MatchKind is overridden to `MulticastBroadcast`. This allows directed broadcasts (common in OT protocols like BACnet) to receive the `MulticastExempt` verdict instead of a false-positive `NoMatchingConduit`. The /31 and /32 cases are explicitly excluded — they have no usable broadcast address. This is IPv4-only; IPv6 has no broadcast.

## Preconditions

1. Step-1 multicast detection (BC-1.03.003) did NOT fire (dst is not in 224/4, not 255.255.255.255, not ff00::/8).
2. The dst IP is IPv4.
3. Longest-prefix zone resolution (BC-1.03.001) has matched the dst to a declared zone with prefix length ≤ 30.
4. The dst IP equals the all-ones host address of the matched CIDR (e.g., dst = `10.0.0.255` for a zone with CIDR `10.0.0.0/24`).

## Postconditions

1. The `Explicit` MatchKind is overridden to `MulticastBroadcast`.
2. The flow receives `MulticastExempt` verdict.
3. `idmz_bypass = false` (forced, as for all multicast/broadcast dst).
4. The override is logged/reported as `MatchKind::MulticastBroadcast` (not `Explicit`).

## Invariants

1. This override is IPv4-only; IPv6 dsts never take this path.
2. /31 and /32 prefixes are excluded: a host in a /32 zone or a point-to-point /31 is never silently exempted by this rule (closes silent-allow vector R-002).
3. The override only applies when the dst IP is the broadcast address of the matched zone — not when it is merely the network address or any other address in the zone.
4. If the dst is EXTERNAL (no matched declared CIDR), the directed-broadcast check never applies.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Zone `10.0.0.0/24`; dst = `10.0.0.255` | Step-2 override fires: `MulticastBroadcast`; `MulticastExempt` (DEC-030) |
| EC-002 | Zone `10.0.0.0/30` (prefix=30); dst = `10.0.0.3` (all-ones host of /30) | Step-2 fires (≤/30 allowed; this is the boundary case) |
| EC-003 | Zone `10.0.0.0/31` (point-to-point); dst = `10.0.0.1` | Step-2 does NOT fire (/31 excluded) — normal Explicit verdict |
| EC-004 | Zone `10.0.0.5/32`; dst = `10.0.0.5` | Step-2 does NOT fire (/32 excluded) — normal Explicit verdict |
| EC-005 | Zone `10.0.0.0/24`; dst = `10.0.0.0` (network address, all-zeros host) | Step-2 does NOT fire (not all-ones host) — normal Explicit verdict (OQ-005) |
| EC-006 | Zone `2001:db8::/32` (IPv6); any dst | Step-2 does NOT fire (IPv6 only); any multicast dst detected by Step-1 |
| EC-007 | Zone `10.0.0.0/24`; dst = `10.0.0.128` (middle of range) | Not broadcast; normal Explicit verdict |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Zone `10.0.0.0/24`; dst = `10.0.0.255` | Override → `MulticastBroadcast`; verdict `MulticastExempt` | happy-path |
| Zone `10.0.0.0/31`; dst = `10.0.0.1` | Normal Explicit; no override | edge-case |
| Zone `10.0.0.0/32`; dst = `10.0.0.0` | Normal Explicit; no override | edge-case |
| Zone `10.0.0.0/24`; dst = `10.0.0.0` (network address) | Normal Explicit; no override | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.03.004-a | For prefix_len ≤ 30, dst == broadcast → override to MulticastBroadcast | kani |
| VP-1.03.004-b | For prefix_len ∈ {31, 32}, no override regardless of dst value | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-005 ("Resolve endpoints to zones") per capabilities.md §CAP-005 |
| L2 Domain Invariants | DI-016 ("Step 2: if the matched zone's IPv4 prefix length is ≤ 30 AND dst equals that zone's directed-broadcast (all-ones host) address, override the Explicit MatchKind to MulticastBroadcast. /31 and /32 excluded.") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-005 ("Resolve endpoints to zones") per capabilities.md §CAP-005 — the directed-broadcast override is part of the endpoint resolution logic that feeds the classification stage, as defined by DI-016 Step-2 |

## Related BCs

- BC-1.03.003 — Step-1 multicast detection (this BC handles only the non-Step-1 dst case)
- BC-1.04.011 — MulticastExempt verdict

## Architecture Anchors

- `architecture/SS-03-zone-resolution.md#directed-broadcast` — Step-2 directed-broadcast override

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.03.004-a — ≤/30 broadcast → MulticastBroadcast
- VP-1.03.004-b — /31 /32 never overridden
