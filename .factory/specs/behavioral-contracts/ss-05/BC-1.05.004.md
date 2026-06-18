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
subsystem: "SS-05"
capability: "CAP-009"
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

# BC-1.05.004: u64 tally overflow: checked arithmetic aborts with exit 2; never silent-wraps

## Description

All tally counters in ConformanceResult (`total_flows`, `intra_zone`, `allowed`, `no_matching_conduit`, `wrong_direction`, `multicast_exempt`, `idmz_bypasses`, `distinct_violating_flows`, `skipped`, `external_endpoints`) are `u64`. Incrementing these counters uses **checked arithmetic** (`checked_add`). If any counter would overflow (reach beyond `u64::MAX`), the run aborts immediately with exit code 2 and a diagnostic message. This is FM-009: a defense-in-depth backstop that is unreachable in practice because the FM-008 ingest cap (BC-1.02.006) makes it impossible to process enough flows to overflow a u64. But the checked arithmetic must still be present.

## Preconditions

1. Aggregation is in progress (ST-7).
2. A counter increment would exceed `u64::MAX` (a theoretical condition prevented by the FM-008 ingest cap).

## Postconditions

1. Process exits with code `2`.
2. Stderr: `"Internal error: tally counter overflow (u64::MAX exceeded). This should be unreachable due to the max_flows cap. Please report a bug."`.
3. No ConformanceResult is emitted.
4. No output artifacts are written.

## Invariants

1. All counter increments use `checked_add`, never `wrapping_add` or plain `+`.
2. The overflow abort (FM-009) is structurally distinct from the ingest cap abort (FM-008/BC-1.02.006).
3. No silent wrap: Rust's debug arithmetic panics are insufficient — the checked arithmetic must be explicit code, not a reliance on debug-mode panics.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Counter at `u64::MAX - 1`, incremented by 1 | Succeeds; counter = `u64::MAX` |
| EC-002 | Counter at `u64::MAX`, incremented by 1 | Overflow detected; exit 2 |
| EC-003 | Ingest cap (BC-1.02.006) fires before this | FM-008 aborts first; FM-009 is never reached in practice |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Normal run within cap | All counts correct; no overflow | happy-path |
| Artificially overflowed counter (unit test mock) | Exit 2; FM-009 diagnostic | error |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.05.004-a | No counter increment uses unchecked arithmetic | unit test (code audit / clippy deny wrapping_add) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 ("Aggregate result") per capabilities.md §CAP-009 |
| L2 Domain Invariants | FM-009 ("u64 tally counter overflow: Checked arithmetic (checked_add); on overflow abort with exit code 2 + diagnostic (never saturate, never silent wrap). The FM-008 ingest cap makes this an unreachable defense-in-depth backstop") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-009 ("Aggregate result") per capabilities.md §CAP-009 — overflow safety is a required property of the aggregation stage per FM-009 |

## Related BCs

- BC-1.02.006 — ingest cap (makes FM-009 unreachable in practice)
- BC-1.05.001 — accounting identity (relies on correct counter values)

## Architecture Anchors

- `architecture/SS-05-aggregation.md#overflow` — checked arithmetic implementation

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.05.004-a — no unchecked arithmetic
