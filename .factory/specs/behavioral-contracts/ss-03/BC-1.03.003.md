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
capability: "CAP-007"
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

# BC-1.03.003: Multicast/broadcast destination short-circuits to MulticastBroadcast before zone resolution

## Description

Detection of multicast and broadcast destination addresses (DI-016 Step 1) is performed **before** zone resolution. This is a family-wide test on the destination IP alone — no zone context is needed. A dst satisfying any of the Step-1 conditions immediately receives `MatchKind::MulticastBroadcast` and the flow proceeds to `MulticastExempt` verdict without entering the longest-prefix resolution path. This prevents false `NoMatchingConduit` violations on dominant cyclic OT I/O patterns (e.g., BACnet broadcasts, EtherNet/IP implicit I/O multicast).

**Step-1 detection criteria:**
- IPv4 multicast: `dst ∈ 224.0.0.0/4`
- IPv4 limited broadcast: `dst == 255.255.255.255`
- IPv6 multicast: `dst ∈ ff00::/8`

The Step-2 directed-broadcast override (BC-1.03.004) happens AFTER zone resolution for the non-Step-1 dst case.

## Preconditions

1. A flow record has been parsed and IPv4-mapped addresses have been canonicalized.
2. Unspecified addresses have been filtered (BC-1.02.003).
3. The dst IP satisfies one of the Step-1 multicast/broadcast criteria.

## Postconditions

1. A `ResolvedEndpoint` for the dst is produced with `match: MatchKind::MulticastBroadcast`.
2. No longest-prefix lookup is performed for this dst.
3. The flow proceeds directly to `MulticastExempt` verdict classification (BC-1.04.011).
4. `idmz_bypass` is forced `false` for this flow regardless of src zone (DI-006).

## Invariants

1. Step-1 detection runs before zone resolution — no zone context is consulted.
2. This check applies to the **destination** only; source IP is never checked for multicast.
3. The test is exhaustive for the family-wide criteria: any IPv4 address in `224.0.0.0/4`, any IPv4 `255.255.255.255`, any IPv6 `ff00::/8`.
4. A declared zone that covers multicast address space (e.g., `224.0.0.0/4`) is irrelevant — Step-1 fires before zone matching (DEC-031).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | dst = `224.0.0.1` (IPv4 multicast) | `MulticastBroadcast`; `MulticastExempt` verdict; `idmz_bypass=false` (DEC-025) |
| EC-002 | dst = `255.255.255.255` (limited broadcast) | `MulticastBroadcast`; `MulticastExempt` (DEC-025) |
| EC-003 | dst = `ff02::1` (IPv6 all-nodes multicast) | `MulticastBroadcast`; `MulticastExempt` |
| EC-004 | Zone declaring `224.0.0.0/4` as a member; dst = `224.0.0.100` | Step-1 fires first; `MulticastBroadcast` regardless of zone declaration (DEC-031) |
| EC-005 | dst = `223.255.255.255` (just below IPv4 multicast range) | NOT multicast; Step-1 does not fire; normal zone resolution proceeds |
| EC-006 | Intra-zone flow with multicast dst | `MulticastExempt` wins over `IntraZone` — multicast short-circuits before zone-pair classification (DI-016; DEC-027) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `dst = 224.0.0.5` (OSPF multicast, common in test captures) | `MatchKind::MulticastBroadcast`; verdict `MulticastExempt`; exit 0 | happy-path |
| `dst = 255.255.255.255` | `MulticastBroadcast`; `MulticastExempt` | happy-path |
| `dst = 10.0.1.255`; zone `10.0.1.0/24` (potential directed-broadcast) | NOT Step-1 (not in 224/4 or ff00:/8); goes to zone resolution then Step-2 (BC-1.03.004) | edge-case |
| `dst = 223.0.0.1` | Not multicast; normal zone resolution | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.03.003-a | Any dst in 224.0.0.0/4, dst==255.255.255.255, or dst in ff00::/8 always → MulticastBroadcast before zone resolution | kani |
| VP-1.03.003-b | MulticastBroadcast dst → idmz_bypass always false | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Classify: multicast exemption (DI-016)") per capabilities.md §CAP-007 |
| L2 Domain Invariants | DI-016 ("Step-1 family-wide test, needs no zone — dst ∈ 224.0.0.0/4, dst == 255.255.255.255, or dst ∈ ff00::/8 → MulticastBroadcast immediately"), DI-006 (idmz_bypass false for multicast) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-007 ("Classify (deny-by-default): ...multicast exemption (DI-016)") per capabilities.md §CAP-007 — multicast/broadcast destination detection is an explicitly listed component of the classification capability |

## Related BCs

- BC-1.03.004 — Step-2 directed-broadcast override (runs after zone resolution for non-Step-1 cases)
- BC-1.04.011 — MulticastExempt verdict (depends on this BC)
- BC-1.04.008 — IDMZ bypass exclusion for multicast

## Architecture Anchors

- `architecture/SS-03-zone-resolution.md#multicast-detection` — Step-1 multicast test

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.03.003-a — family-wide multicast → MulticastBroadcast
- VP-1.03.003-b — MulticastBroadcast → idmz_bypass false
