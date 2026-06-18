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

# BC-1.03.005: Both-EXTERNAL flow: two endpoints both resolving to EXTERNAL → IntraZone

## Description

When both the source and destination endpoints of a flow resolve to the reserved `EXTERNAL` zone (i.e., neither has a matching declared CIDR), the flow satisfies the "same zone" predicate of DI-002 and receives the `IntraZone` verdict. EXTERNAL is a single zone — two unmanaged endpoints communicating is Internet-to-Internet transit, which is out of scope for managed segmentation. This resolves P08-HIGH-001 and P08-HIGH-002 from the pass-8 adversarial review by explicitly defining the both-EXTERNAL case as IntraZone.

## Preconditions

1. A flow's src endpoint has resolved to `EXTERNAL` (MatchKind::ImplicitExternal).
2. The same flow's dst endpoint has also resolved to `EXTERNAL` (MatchKind::ImplicitExternal, or the dst is MulticastBroadcast — see edge cases).

## Postconditions

1. Verdict = `IntraZone`.
2. `idmz_bypass = false` (EXTERNAL is excluded from DI-006).
3. The flow is counted in the `intra_zone` tally.
4. `external_endpoints` is incremented by 1 for this flow (DEC-026 — the counter counts flows with ≥1 EXTERNAL endpoint; both-EXTERNAL counts once).

## Invariants

1. The "same zone" predicate in DI-002 applies to the EXTERNAL zone: two EXTERNAL endpoints = same zone = IntraZone.
2. An IntraZone verdict for a both-EXTERNAL flow is NOT a violation — it is allowed (DI-002).
3. IDMZ bypass never applies to flows involving EXTERNAL endpoints (DI-006 truth table).
4. The `external_endpoints` counter is diagnostic and NOT part of the DI-015 accounting identity.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `8.8.8.8 → 1.1.1.1`; no matching zones for either | Both resolve to EXTERNAL; verdict `IntraZone`; not a violation (DEC-032) |
| EC-002 | One endpoint EXTERNAL, one in a declared zone | NOT both-EXTERNAL; this is a normal cross-zone flow; conduit matching applies |
| EC-003 | Both endpoints EXTERNAL; a conduit `EXTERNAL → EXTERNAL` exists | IntraZone verdict still takes precedence over conduit evaluation (same-zone short-circuit) |
| EC-004 | `8.8.8.8 → 224.0.0.5` (EXTERNAL src, multicast dst) | Dst is MulticastBroadcast (Step-1); verdict is `MulticastExempt` (MulticastExempt precedence wins; DI-016 order) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Flow `8.8.8.8 → 1.1.1.1`; no declared zones cover either IP | `IntraZone` verdict; `external_endpoints = 1`; no violation | happy-path |
| Flow `10.0.1.5 → 8.8.8.8`; zone covers `10.0.1.0/24` | NOT both-EXTERNAL; src resolves to declared zone; verdict depends on conduit matching | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.03.005-a | Both-EXTERNAL flow → IntraZone verdict (never NoMatchingConduit or violation) | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-005 ("Resolve endpoints to zones") per capabilities.md §CAP-005 |
| L2 Domain Invariants | DI-002 ("Two endpoints resolve to the same zone → IntraZone; the reserved EXTERNAL zone is a single zone, so both-EXTERNAL satisfies 'same zone' → IntraZone"), DI-005 (implicit EXTERNAL zone) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-005 ("Resolve endpoints to zones") per capabilities.md §CAP-005 — the both-EXTERNAL IntraZone rule is a required behavior of the resolution stage, encoding the DI-002 "same zone" predicate for the EXTERNAL sentinel |

## Related BCs

- BC-1.03.002 — implicit EXTERNAL fallback (precondition)
- BC-1.04.001 — intra-zone allowed (depends on)

## Architecture Anchors

- `architecture/SS-03-zone-resolution.md#external-intrazone` — both-EXTERNAL IntraZone rule

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.03.005-a — both-EXTERNAL → IntraZone
