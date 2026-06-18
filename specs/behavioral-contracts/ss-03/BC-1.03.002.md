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

# BC-1.03.002: Resolve unmatched endpoint to implicit EXTERNAL zone

## Description

When no declared zone matcher matches a flow endpoint's IP address, the endpoint is resolved to the reserved `EXTERNAL` zone. This is the fallback of the total resolution guarantee (DI-003): every endpoint resolves somewhere, and `EXTERNAL` is the catch-all for all unmanaged IPs. The `EXTERNAL` zone has a fixed `purdue_level = L5` (sentinel, not a "real" L5 managed zone), and flows to/from it are governed by conduit matching exactly like any other zone (deny-by-default). The `EXTERNAL` zone is never a "managed" zone for IDMZ bypass purposes (DI-006).

## Preconditions

1. Zone resolution via longest-prefix match (BC-1.03.001) found no matching declared matcher.
2. The endpoint is not a multicast/broadcast address (those are handled by BC-1.03.003 before reaching this point).
3. The endpoint is not the unspecified address (those are skipped by BC-1.02.003 before reaching this point).

## Postconditions

1. A `ResolvedEndpoint` is produced with:
   - `zone_id`: the reserved `EXTERNAL` zone ID.
   - `match: MatchKind::ImplicitExternal`.
   - `ip`: the endpoint IP.
2. The flow proceeds to classification against conduits exactly as if it had resolved to a declared zone.
3. `idmz_bypass` is never set `true` for flows involving an EXTERNAL endpoint (DI-006; DI-005).

## Invariants

1. EXTERNAL is a single zone: two endpoints both resolving to EXTERNAL satisfy the "same zone" predicate → `IntraZone` verdict (DI-002; BC-1.03.005).
2. EXTERNAL's `purdue_level = L5` is a sentinel value — it is NOT used for IDMZ bypass computation (DI-006 excludes EXTERNAL regardless of purdue level).
3. EXTERNAL cannot be redeclared in the policy (BC-1.01.004).
4. `ImplicitExternal` match kind is distinct from `Explicit`; reports show which endpoints were unmatched.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Public IP `8.8.8.8` with no matching declared zone | Resolves to EXTERNAL; `MatchKind::ImplicitExternal` (DEC-003) |
| EC-002 | Loopback `127.0.0.1` with no zone claiming it | Resolves to EXTERNAL (DEC-013) |
| EC-003 | Link-local `169.254.x.x` with no zone | Resolves to EXTERNAL (DEC-013) |
| EC-004 | IPv6 global address `2001:db8::1` with only IPv4 zones | Resolves to EXTERNAL (no IPv6 matchers) |
| EC-005 | Flow with both endpoints resolving to EXTERNAL | `IntraZone` verdict (DI-002; BC-1.03.005) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Endpoint `8.8.8.8`; no matching zone | `ResolvedEndpoint { zone_id: EXTERNAL, match: ImplicitExternal }` | happy-path |
| Flow `10.0.1.5 → 8.8.8.8`; zone covers `10.0.1.0/24`; no conduit to EXTERNAL | Src resolves to declared zone; dst resolves to EXTERNAL; verdict `NoMatchingConduit` | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.03.002-a | Every endpoint not matched by a declared CIDR resolves to EXTERNAL (DI-005 + DI-003) | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-005 ("Resolve endpoints to zones: unmatched → implicit EXTERNAL zone") per capabilities.md §CAP-005 |
| L2 Domain Invariants | DI-003 (total resolution ⊢), DI-005 (implicit EXTERNAL zone) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-005 ("Resolve endpoints to zones") per capabilities.md §CAP-005 — the implicit EXTERNAL fallback is explicitly defined as part of CAP-005's "unmatched → implicit EXTERNAL zone" requirement |

## Related BCs

- BC-1.03.001 — longest-prefix match (this BC handles the no-match fallback)
- BC-1.03.005 — both-EXTERNAL IntraZone verdict (depends on)
- BC-1.04.008 — IDMZ bypass exclusion for EXTERNAL (depends on)

## Architecture Anchors

- `architecture/SS-03-zone-resolution.md#external-zone` — EXTERNAL zone sentinel

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.03.002-a — every unmatched endpoint → EXTERNAL
