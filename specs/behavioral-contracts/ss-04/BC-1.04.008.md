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
capability: "CAP-008"
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

# BC-1.04.008: IDMZ bypass is NOT raised for flows involving EXTERNAL or MulticastBroadcast endpoints

## Description

Two categories of flows are excluded from the IDMZ bypass check (DI-006 truth table): (1) flows where either endpoint resolves to the reserved `EXTERNAL` zone, and (2) flows where the destination is `MulticastBroadcast`. For EXTERNAL: unmanaged IPs are governed by conduit matching (DI-005), not IDMZ segmentation logic — Internet egress from a PLC is a conduit violation, not an IDMZ bypass. For MulticastBroadcast: the flow receives `MulticastExempt` verdict; IDMZ bypass is forced false (DI-006 / DI-016). This BC documents these exclusions explicitly to prevent implementation errors.

## Preconditions

1. A flow has been resolved.
2. Either: (a) at least one endpoint resolved to `EXTERNAL` (MatchKind::ImplicitExternal), OR (b) the dst endpoint is `MulticastBroadcast`.

## Postconditions

1. `idmz_bypass = false` for this flow.
2. No `IdmzBypass` violation is generated.
3. The flow is still evaluated for conduit violations (unless it is MulticastExempt, which short-circuits entirely).

## Invariants

1. EXTERNAL is excluded from IDMZ bypass by zone identity, not by its sentinel purdue_level.
2. The "MulticastBroadcast dst forces false" rule is the single exception to the otherwise verdict-independent nature of idmz_bypass (DI-006, bullet: "if either endpoint is MulticastBroadcast, idmz_bypass = false — the single exception to idmz_bypass's otherwise verdict-independent evaluation").

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Flow `managed L1 → EXTERNAL` | `idmz_bypass = false` (EXTERNAL excluded; DEC-017); still checked for conduit violation |
| EC-002 | Flow `EXTERNAL → managed L4` | `idmz_bypass = false` (EXTERNAL endpoint) |
| EC-003 | Flow `managed L1 → MulticastBroadcast` | `idmz_bypass = false`; verdict `MulticastExempt` |
| EC-004 | Flow `managed L1 → managed L4` (no EXTERNAL, no multicast) | `idmz_bypass` evaluated normally; may be true (BC-1.04.007) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Flow `plc (L1) → 8.8.8.8 (EXTERNAL)` | `idmz_bypass = false`; conduit violation possible | happy-path |
| Flow `plc (L1) → 224.0.0.5 (multicast)` | `idmz_bypass = false`; verdict `MulticastExempt` | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.008-a | EXTERNAL endpoint → idmz_bypass always false | kani |
| VP-1.04.008-b | MulticastBroadcast dst → idmz_bypass always false | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-008 ("IDMZ no-bypass check") per capabilities.md §CAP-008 |
| L2 Domain Invariants | DI-006 ("EXTERNAL endpoints are excluded; if either endpoint is MulticastBroadcast, idmz_bypass = false — the single exception to idmz_bypass's verdict-independent evaluation"), DI-005 |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-008 ("IDMZ no-bypass check") per capabilities.md §CAP-008 — the exclusion of EXTERNAL and MulticastBroadcast from the IDMZ check is a required behavior of CAP-008 per DI-006 and DI-005 |

## Related BCs

- BC-1.04.007 — IDMZ bypass detection (the positive case)
- BC-1.03.003 — MulticastBroadcast detection

## Architecture Anchors

- `architecture/SS-04-classification.md#idmz-bypass` — IDMZ exclusion logic

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.008-a — EXTERNAL → idmz_bypass false
- VP-1.04.008-b — MulticastBroadcast → idmz_bypass false
