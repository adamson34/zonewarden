---
document_type: adr
level: L3
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-06-17T00:00:00
phase: 1b
traces_to: ARCH-INDEX.md
---

# ADR-003: Longest-Prefix Match via Sorted Flat Vec

**Status:** accepted

## Context

Zone resolution (DI-003, DI-004) requires mapping an arbitrary IP address to exactly one
zone via longest-prefix match. The data structure for the prefix index is an architectural
decision because it directly affects (a) runtime performance and (b) Kani proof feasibility.

## Options Considered

1. **Sorted flat `Vec<(IpNet, ZoneId)>` descending by prefix length (chosen)**
   - Linear scan; first match wins; longest prefix wins by sort order.
   - For 200 matchers (large OT policy), this is ~200 comparisons worst case — negligible.
   - Trivially modelable in Kani: a bounded-length array with simple loop.

2. **Radix / Patricia trie (e.g., hand-rolled or `ipnet-trie` crate)**
   - O(prefix_length) lookup; optimal for large prefix tables.
   - Requires non-trivial tree invariants; harder to prove in Kani.
   - Needed only if the prefix table exceeds ~10,000 entries — unlikely in any OT policy.

3. **`iptrie` / `ip_network_table` crate**
   - Production-grade; optimized.
   - External dependency with its own internal invariants; Kani cannot look inside.
   - Correctness of the trie is assumed, not proven.

4. **Hash map by exact IP, fallback to prefix scan**
   - Optimization for host (/32 or /128) matchers.
   - Two code paths; more invariants to prove.

## Decision

Use a sorted flat `Vec<(IpNet, ZoneId)>` descending by prefix length, separated into
two Vecs: one for IPv4 (`Vec<(Ipv4Net, ZoneId)>`) and one for IPv6 (`Vec<(Ipv6Net, ZoneId)>`).
Separate Vecs avoid mixed-family comparisons and match the tie-detection constraint (DI-010:
ties are per-family).

Resolution: iterate the appropriate Vec from index 0; return the first `zone_id` where
`ipnet.contains(addr)`. If no match, return `EXTERNAL`.

## Rationale

- Kani feasibility: a bounded loop over a bounded array is the canonical Kani proof shape.
  VP-001 (total resolution) and VP-002 (longest-prefix uniqueness) are directly provable
  with this structure, assuming the Vec is sorted. The sort invariant is established at
  policy validation time (ST-2) and never mutated after that.
- OT policy scale: a 30-zone policy with 5 CIDRs per zone = 150 entries. Linear scan is
  sub-microsecond; well within NFR-002 (≥100k flows/sec).
- No external crate: eliminates a dep with its own unverifiable internals.

## Consequences

- The `PrefixIndex` struct wraps two sorted Vecs (v4, v6); it is built once by `validator`
  and passed immutably to `resolver` and `classifier`.
- Tie detection during policy validation: after sorting, any two adjacent entries with
  equal prefix length that are in the same family and both match a common address are a tie
  → policy error (DI-010, BC-1.01.005).
- The sort key is `(prefix_len DESC, addr_as_u128 ASC)` to ensure deterministic ordering
  among same-length entries (for Kani proof stability).

## Verification Feasibility

VP-001 and VP-002 both target `resolver::resolve`. The proof harness:
```rust
#[kani::proof]
fn verify_total_resolution() {
    let index: PrefixIndex = kani::any();
    kani::assume(index.is_valid()); // sorted, no ties
    let ip: IpAddr = kani::any();
    let result = resolver::resolve(&index, ip);
    assert!(result.is_some()); // always returns a zone or EXTERNAL
}
```
This is feasible because `PrefixIndex::is_valid()` is a bounded-length invariant, and
`resolve` is a bounded loop.
