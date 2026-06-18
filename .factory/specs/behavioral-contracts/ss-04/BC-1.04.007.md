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

# BC-1.04.007: IDMZ no-bypass: managed ≤L3 ↔ managed ≥L4 without IDMZ endpoint → additive IdmzBypass finding

## Description

The IDMZ no-bypass rule implements the Purdue 3.5 Industrial DMZ requirement: a direct flow between an OT-side zone (≤ L3) and an IT-side zone (≥ L4) without routing through the IDMZ (L3.5) is a security violation. This is an **additive finding** — it is independent of the conduit verdict. A flow can be simultaneously `Allowed` by a conduit AND have `idmz_bypass = true`. The IDMZ check is performed on every resolved flow (not just violations) and is tallied separately in `idmz_bypasses`. The "managed" determination is by zone identity: EXTERNAL is excluded regardless of its sentinel purdue_level; a declared zone at L5 is still "managed."

**Truth table per DI-006 (see entities.md):**
- managed ≤L3 ↔ managed ≥L4, neither in IDMZ → `idmz_bypass = true`
- Any flow involving EXTERNAL → `idmz_bypass = false`
- Any flow involving IDMZ endpoint → `idmz_bypass = false`
- Both endpoints same side (both ≤L3 or both ≥L4) → `idmz_bypass = false`
- MulticastBroadcast dst → `idmz_bypass = false` (the single exception to verdict-independence; DI-016)

## Preconditions

1. Both flow endpoints have been resolved to zones.
2. Neither endpoint resolved to `MulticastBroadcast` (that forces `idmz_bypass = false`; DI-016).
3. Neither endpoint is the reserved `EXTERNAL` zone.
4. One endpoint's zone has `purdue_level` ∈ {L0, L1, L2, L3} (≤L3) AND the other has `purdue_level` ∈ {L4, L5} (≥L4).
5. Neither endpoint's zone is the IDMZ (L3.5).

## Postconditions

1. `idmz_bypass = true` for this flow's Verdict.
2. A `Violation { kind: ViolationKind::IdmzBypass, severity: ..., explanation: "Direct OT-IT flow without IDMZ endpoint" }` is added to the violation list.
3. `idmz_bypasses` tally incremented by 1.
4. This violation is additive: the conduit-based VerdictKind is unaffected (`Allowed`/`NoMatchingConduit`/`WrongDirection` as determined independently).
5. `distinct_violating_flows` is incremented for this flow's `flow_index` (if not already counted by a conduit violation).

## Invariants

1. `idmz_bypass` is evaluated for every resolved flow, independently of the conduit verdict.
2. The EXTERNAL zone is excluded by zone identity, not by purdue_level.
3. A declared zone at L5 (managed, not EXTERNAL sentinel) is a valid ≥L4 endpoint for this check.
4. MulticastBroadcast dst forces `idmz_bypass = false` — the single special-cased exception to verdict-independence (DI-006 truth table; DI-016).
5. IDMZ (L3.5) is "neither ≤L3 nor ≥L4" per the PurdueLevel definition — neither endpoint in IDMZ means the check triggers; one endpoint in IDMZ means it does not.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Flow managed L1 → managed L4; conduit permits it | `Allowed` AND `idmz_bypass = true`; appears in `allowed` tally AND `idmz_bypasses` tally AND violations list (DEC-005) |
| EC-002 | Flow managed L1 → managed L5 (declared zone, not EXTERNAL) | `idmz_bypass = true` (declared L5 is managed; DI-006 truth table row 2) |
| EC-003 | Flow managed L1 → EXTERNAL (L5 sentinel) | `idmz_bypass = false` (EXTERNAL excluded; DI-006 truth table row 3; DEC-017) |
| EC-004 | Flow managed L2 → IDMZ (L3.5) | `idmz_bypass = false` (one endpoint IS the IDMZ) |
| EC-005 | Flow IDMZ → managed L4 | `idmz_bypass = false` (one endpoint IS the IDMZ) |
| EC-006 | Flow managed L2 → managed L3 | `idmz_bypass = false` (both ≤L3; no IT side) |
| EC-007 | Flow managed L4 → managed L5 | `idmz_bypass = false` (both ≥L4; no OT side) |
| EC-008 | Flow managed L1 → MulticastBroadcast dst | `idmz_bypass = false` (multicast exception; DI-016) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Flow `field (L2) → historian (L4)`; conduit exists | `Allowed`; `idmz_bypass = true`; violation `IdmzBypass` also recorded | happy-path (DEC-005) |
| Flow `plc (L1) → dmz (L3.5)` | `idmz_bypass = false`; IDMZ endpoint present | edge-case |
| Flow `plc (L1) → internet (EXTERNAL)` | `idmz_bypass = false`; EXTERNAL excluded | edge-case |
| Flow `plc (L1) → historian (L4)`; no conduit | `NoMatchingConduit` AND `idmz_bypass = true`; both violations | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.007-a | IDMZ bypass truth table: all 8 cases produce correct idmz_bypass value | kani (truth table enumeration) |
| VP-1.04.007-b | IdmzBypass violation is additive: does not change the conduit VerdictKind | kani |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-008 ("IDMZ no-bypass check: Flag any single flow between two managed zones where one is ≤L3 and the other ≥L4 and neither endpoint is in the IDMZ (L3.5)") per capabilities.md §CAP-008 |
| L2 Domain Invariants | DI-006 ("IDMZ no-bypass (single-flow, additive): idmz_bypass = true iff one endpoint resolves to managed ≤L3 and the other to managed ≥L4, and neither is IDMZ. Independent additive finding, NOT a VerdictKind. ⊢") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-008 ("IDMZ no-bypass check") per capabilities.md §CAP-008 — this BC is the direct implementation of CAP-008's IDMZ bypass detection rule, which is exactly what CAP-008 defines |

## Related BCs

- BC-1.04.008 — IDMZ bypass exclusion for EXTERNAL and multicast (companion)
- BC-1.04.010 — verdict totality (idmz_bypass is independent; counted separately)

## Architecture Anchors

- `architecture/SS-04-classification.md#idmz-bypass` — IDMZ no-bypass rule

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.007-a — truth table: all 8 cases
- VP-1.04.007-b — additive independence
