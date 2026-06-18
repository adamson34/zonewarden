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
subsystem: "SS-02"
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

# BC-1.02.005: Canonicalize IPv4-mapped IPv6 addresses to IPv4 before resolution

## Description

IPv4-mapped IPv6 addresses (in the form `::ffff:a.b.c.d`) must be canonicalized to their IPv4 form before zone resolution. This ensures they match IPv4 zone matchers and are counted as the IPv4 address family for tie-scoping purposes. Without this canonicalization, a flow from `::ffff:10.0.1.5` would fail to match a zone declaring `10.0.1.0/24`, producing a spurious EXTERNAL resolution.

## Preconditions

1. A parsed flow record contains an IP address in `src_ip` or `dst_ip` that is an IPv4-mapped IPv6 address (`::ffff:a.b.c.d` form).

## Postconditions

1. The address is converted to its IPv4 form (`a.b.c.d`) before any zone resolution or unspecified-address check.
2. After canonicalization, the address is treated as IPv4 for all matching and tie-detection purposes.
3. Non-mapped IPv6 addresses (e.g., `2001:db8::1`) are NOT affected — they remain IPv6.
4. The canonical IPv4 form is what appears in all downstream processing (zone resolution, verdict, report).

## Invariants

1. The canonicalization is applied to both `src_ip` and `dst_ip`.
2. Canonicalization occurs before unspecified-address detection (so `::ffff:0.0.0.0` → `0.0.0.0` → skipped by BC-1.02.003).
3. Canonicalization is idempotent: already-IPv4 addresses are unchanged.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `src_ip = ::ffff:10.0.1.5` | Canonicalized to `10.0.1.5` |
| EC-002 | `src_ip = ::ffff:0.0.0.0` (IPv4-mapped unspecified) | Canonicalized to `0.0.0.0` → then skipped by BC-1.02.003 |
| EC-003 | `src_ip = 2001:db8::1` (non-mapped IPv6) | Unchanged; remains IPv6 |
| EC-004 | `src_ip = ::ffff:192.168.1.100` | Canonicalized to `192.168.1.100`; matched against IPv4 zone matchers |
| EC-005 | Policy with a zone `members: [10.0.1.0/24]`; flow with `src_ip = ::ffff:10.0.1.5` | Canonicalized to `10.0.1.5`; resolves to the `10.0.1.0/24` zone |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Flow with `src_ip = ::ffff:10.0.1.5` against zone `10.0.1.0/24` | Resolves to the declared zone (not EXTERNAL) | happy-path |
| Flow with `dst_ip = ::ffff:0.0.0.0` | After canonicalization → `0.0.0.0` → BC-1.02.003 skip | edge-case |
| Flow with `src_ip = 2001:db8::1` | No canonicalization; processed as IPv6 | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.02.005-a | After canonicalization, no IPv4-mapped addresses remain in the flow | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-005 ("Resolve endpoints to zones") per capabilities.md §CAP-005 |
| L2 Domain Invariants | DEC-012 ("IPv4-mapped IPv6 (::ffff:a.b.c.d): canonicalized to IPv4 before resolution so they match IPv4 matchers and count as the IPv4 family for tie scoping") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-005 ("Resolve endpoints to zones") per capabilities.md §CAP-005 — IPv4-mapped canonicalization is a required pre-processing step for total endpoint resolution (DI-003) |

## Related BCs

- BC-1.02.003 — unspecified address skip (applied after this BC)
- BC-1.03.001 — zone resolution (benefits from canonicalization)

## Architecture Anchors

- `architecture/SS-02-flow-ingest.md#ipv4-mapped` — IPv4-mapped address canonicalization

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.02.005-a — no IPv4-mapped addresses after canonicalization
