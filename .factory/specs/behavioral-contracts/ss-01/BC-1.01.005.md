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
subsystem: "SS-01"
capability: "CAP-002"
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

# BC-1.01.005: Reject equal-prefix-length ties (same-family, same address)

## Description

Two `AssetMatcher` entries in the same IP family that both match a common address with equal prefix length constitute a "tie" and must be rejected at policy validation time. Ties are undefined behavior for the longest-prefix-match algorithm — they would make zone resolution non-deterministic. Note: disjoint same-length CIDRs (e.g., `10.0.0.0/24` and `10.0.1.0/24`) are NOT ties since they share no common address; they are legal and produce no error (DEC-022).

## Preconditions

1. Structural parse (BC-1.01.001) and basic semantic validation (BC-1.01.004) have not yet detected a fatal error.
2. The policy contains two or more `AssetMatcher` entries (across any zones) in the same IP family that overlap with equal prefix lengths.

## Postconditions

1. Exit 2; stderr identifies the overlapping CIDRs and the zones they belong to.
2. No flow processing occurs.

## Invariants

1. A tie is defined per IP family: an IPv4 CIDR never ties with an IPv6 CIDR.
2. A `/32` (single host) in one zone and the same host's `/24` network in another zone is NOT a tie (different prefix lengths → longest-prefix-match resolves deterministically to the `/32`).
3. The EXTERNAL zone has no declared matchers and cannot participate in a tie.
4. An `IpAddr` matcher is treated as a `/32` (IPv4) or `/128` (IPv6) for tie detection purposes.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Zone A: `10.0.0.0/24`, Zone B: `10.0.0.0/24` (identical CIDR) | Tie (same family, same length, same address space); exit 2 |
| EC-002 | Zone A: `10.0.0.0/24`, Zone B: `10.0.1.0/24` | Not a tie (disjoint); both valid (DEC-022) |
| EC-003 | Zone A: `10.0.0.5/32`, Zone B: `10.0.0.0/24` | Not a tie (different prefix lengths); longest-prefix selects `/32` deterministically |
| EC-004 | Zone A: `10.0.0.5/32`, Zone B: `10.0.0.5/32` | Tie; exit 2 |
| EC-005 | Zone A: `10.0.0.0/24` (IPv4), Zone B: `::ffff:10.0.0.0/120` (IPv6 mapped — after canonicalization becomes IPv4 `10.0.0.0/24`) | After IPv4-mapped canonicalization (BC-1.02.005), treated as IPv4 tie; exit 2 |
| EC-006 | Zone A: `10.0.0.0/24`, Zone B: `10.0.0.255/32` (broadcast of A's network) | Not a tie (different prefix lengths); `/32` wins over `/24` per longest-prefix |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Policy with zone `field: [10.0.1.0/24]`, zone `control: [10.0.1.0/24]` | Exit 2; "tie: 10.0.1.0/24 in zones field and control" | error |
| Policy with zone `field: [10.0.0.0/24]`, zone `control: [10.0.1.0/24]` | Validation passes (no tie) | happy-path |
| Policy with zone `a: [192.168.1.5/32]`, zone `b: [192.168.1.5/32]` | Exit 2; tie on host address | error |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.01.005-a | Any two matchers in the same family with the same prefix length and overlapping address space → validation error | proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 ("Validate policy: Statically validate the loaded policy (unique zone ids, conduit endpoints exist, valid Purdue levels, no equal-prefix membership ties)") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-010 ("no membership ties; a tie = two AssetMatchers in the same IP family that match a common address with equal prefix length"), DI-004 (deterministic overlap resolution via longest-prefix) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-002 ("Validate policy") per capabilities.md §CAP-002 — tie detection is an explicitly listed validation requirement of CAP-002 |

## Related BCs

- BC-1.01.004 — companion semantic validation (depends on)
- BC-1.03.001 — longest-prefix match (this BC ensures its precondition: no ties)

## Architecture Anchors

- `architecture/SS-01-policy.md#tie-detection` — tie detection algorithm

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.01.005-a — tie always exits 2
