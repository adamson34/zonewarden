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
subsystem: "SS-04"
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

# BC-1.04.002: Deny-by-default: flow with no matching conduit → NoMatchingConduit

## Description

This is the core security property of zonewarden: any cross-zone flow that is not explicitly permitted by a matching conduit, and is not intra-zone, and is not multicast-exempt, receives the `NoMatchingConduit` verdict — a violation. There is no implicit permitting. This implements DI-001. The deny-by-default principle means silence is not consent: absence of a conduit always means the flow is a violation.

## Preconditions

1. Both endpoints have been resolved to zones (not same zone — that would be IntraZone per BC-1.04.001).
2. Neither endpoint is MulticastBroadcast (that would be MulticastExempt per BC-1.04.011).
3. No conduit in the policy permits this flow's (zone-pair, proto, port, direction) combination.

## Postconditions

1. Verdict = `NoMatchingConduit`.
2. A `Violation { kind: ViolationKind::NoMatchingConduit, severity: ..., explanation: ... }` is produced.
3. `Severity` is set per BC-1.04.009 (from conn_state).
4. `idmz_bypass` is evaluated independently and may be true for this flow simultaneously.
5. `no_matching_conduit` tally incremented by 1.

## Invariants

1. `NoMatchingConduit` is always a violation.
2. A `NoMatchingConduit` verdict can coexist with `idmz_bypass = true` (both findings apply independently).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Policy with zero conduits; cross-zone TCP flow | NoMatchingConduit (DEC-019) |
| EC-002 | Flow `field → control`, port 502 TCP; conduit exists only for port 502 UDP | NoMatchingConduit (proto mismatch) |
| EC-003 | Flow from EXTERNAL → managed zone; no conduit | NoMatchingConduit (EXTERNAL is governed by conduit matching per DI-005) |
| EC-004 | Flow `L1 → L4` with no conduit and no IDMZ | NoMatchingConduit AND idmz_bypass=true; both violations recorded (DEC-005, additive) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Cross-zone flow; no matching conduit; conn_state SF | Verdict `NoMatchingConduit`; Severity `Established`; counted in violations | happy-path |
| Cross-zone flow; conduit exists for different proto | NoMatchingConduit | edge-case |
| Cross-zone flow; empty conduit list | NoMatchingConduit (DEC-019) | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.002-a | No cross-zone, non-multicast, non-intrazone flow is ever silently allowed without a matching conduit | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Classify (deny-by-default): deny-by-default (DI-001)") per capabilities.md §CAP-007 |
| L2 Domain Invariants | DI-001 ("Any flow not explicitly permitted or intra-zone is a Violation → NoMatchingConduit"), DI-014 (any-match union) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-007 ("Classify (deny-by-default)") per capabilities.md §CAP-007 — deny-by-default (NoMatchingConduit) is the primary listed requirement of CAP-007 per DI-001 |

## Related BCs

- BC-1.04.003 — any-match conduit union (the allowing gate that prevents NoMatchingConduit)
- BC-1.04.009 — severity grading (applied to NoMatchingConduit violations)
- BC-1.04.007 — IDMZ bypass (independent additive finding)

## Architecture Anchors

- `architecture/SS-04-classification.md#deny-by-default` — deny-by-default logic

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.002-a — no silent cross-zone allow without conduit
