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

# BC-1.04.009: Violation severity graded from conn_state: Attempted bucket → Attempted; otherwise → Established

## Description

Every `Violation` entry carries a `Severity` field: `Attempted` or `Established`. This grades whether the violating flow represents a confirmed data exchange (control was NOT effective) or a blocked attempt (control WAS effective). The mapping is determined by the flow's `ConnState` against the bucket definition from entities.md. Severity exists only on Violation entries — it is never computed for non-violating verdicts. A flow with no `conn_state` field or `ConnState::Other(_)` defaults to `Established` (conservative: never under-reports a breach). The complete 13-state Zeek mapping is resolved in OQ-001.

**ConnState → Severity bucket (from entities.md + OQ-001 resolution):**

| Zeek conn_state | Severity |
|-----------------|---------|
| S0 | Attempted |
| S1 | Attempted |
| S2 | Established |
| S3 | Established |
| SF | Established |
| REJ | Attempted |
| RSTO | Established |
| RSTR | Established |
| RSTOS0 | Attempted |
| RSTRH | Attempted |
| SH | Attempted |
| SHR | Attempted |
| OTH | Established |
| None / Other(_) | Established (conservative default) |

## Preconditions

1. A violation has been determined (VerdictKind ∈ {NoMatchingConduit, WrongDirection} OR idmz_bypass = true).
2. The flow may or may not have a `conn_state` value.

## Postconditions

1. Each `Violation` entry has `severity` ∈ {Attempted, Established}.
2. If `conn_state` maps to the Attempted bucket: `severity = Attempted`.
3. If `conn_state` maps to the Established bucket, is None, or is Other(_): `severity = Established`.
4. Severity is stored on the `Violation` struct, not on the `Verdict` (non-violating flows do not carry severity).

## Invariants

1. Severity is only assigned on Violation entries (never stored on Verdict for non-violating flows).
2. The Conservative default: absence of conn_state always gives `Established` (never under-reports).
3. `ConnState::Other(String)` (unrecognized Zeek state tokens) → `Established` (conservative).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | conn_state = `S0`; violating flow | `severity = Attempted` |
| EC-002 | conn_state = `SF`; violating flow | `severity = Established` |
| EC-003 | conn_state = None (field absent) | `severity = Established` (conservative default) |
| EC-004 | conn_state = `Other("CUSTOM")` (unknown state) | `severity = Established` (conservative) |
| EC-005 | conn_state = `RSTO` | `severity = Established` (connection was established before reset; OQ-001 resolution) |
| EC-006 | Allowed flow (non-violating) | No severity computed or stored |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Violating flow; `conn_state = S0` | `Violation { severity: Attempted }` | happy-path |
| Violating flow; `conn_state = SF` | `Violation { severity: Established }` | happy-path |
| Violating flow; `conn_state = None` | `Violation { severity: Established }` (conservative) | edge-case |
| Allowed flow; any conn_state | No severity field in Verdict | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.04.009-a | Non-violating flows never have a Severity value | unit test |
| VP-1.04.009-b | conn_state = None → severity = Established | unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Classify: severity grading (DI-017)") per capabilities.md §CAP-007 |
| L2 Domain Invariants | DI-017 ("A violation's severity is Attempted iff the flow's ConnState is in the Attempted bucket as defined in entities.md ConnState; otherwise Established (covers Established bucket, Other(_), and absent conn_state — conservative). entities.md ConnState is the single source of bucket membership.") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-007 ("Classify (deny-by-default): severity grading (DI-017)") per capabilities.md §CAP-007 — severity grading is an explicitly listed component of the classification capability |

## Related BCs

- BC-1.04.002 — NoMatchingConduit violation (this BC applies to its violations)
- BC-1.04.004 — WrongDirection violation (this BC applies)
- BC-1.04.007 — IdmzBypass violation (this BC applies)

## Architecture Anchors

- `architecture/SS-04-classification.md#severity` — severity grading

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.04.009-a — non-violating flows have no severity
- VP-1.04.009-b — None → Established (conservative)
