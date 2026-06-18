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
capability: "CAP-003"
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

# BC-1.02.003: Skip and warn flows with unspecified address (0.0.0.0 or ::) as src or dst

## Description

A flow record carrying the unspecified IPv4 address `0.0.0.0` or the unspecified IPv6 address `::` as either `src_ip` or `dst_ip` is semantically meaningless for segmentation analysis — an unspecified endpoint cannot be zone-resolved or used for directional verdict computation. Such records must be skipped and counted (same `skipped` counter as BC-1.02.002), and a specific warning is emitted. This is a deterministic rule (not "likely EXTERNAL"), resolving P08-HIGH-003 and P08-HIGH-004 from pass-8 adversarial review.

## Preconditions

1. A flow line is otherwise structurally parseable.
2. The `src_ip` or `dst_ip` field (after IPv4-mapped canonicalization per BC-1.02.005) resolves to the unspecified address `0.0.0.0` (IPv4) or `::` (IPv6).

## Postconditions

1. The record is discarded.
2. `skipped` counter incremented by 1.
3. A warning is added to `ConformanceResult.warnings`: e.g., `"Flow skipped: unspecified address (0.0.0.0/::) in src or dst at line N"`.
4. No `Verdict` is produced for this record.
5. Processing continues with the next record.

## Invariants

1. The unspecified address rule applies to BOTH src and dst roles.
2. The check is applied after IPv4-mapped canonicalization (so `::ffff:0.0.0.0` is canonicalized to `0.0.0.0` and then skipped).
3. This rule runs before zone resolution; unspecified addresses never reach the classification stage.
4. The warning is distinct from the generic "malformed flow" warning (this is a well-formed record with a semantically invalid address).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | DHCP-discover record with `src_ip = 0.0.0.0` | Skipped; specific warning about unspecified src; skipped counter +1 |
| EC-002 | Flow with `dst_ip = 0.0.0.0` | Skipped; specific warning about unspecified dst |
| EC-003 | Flow with `src_ip = ::` (IPv6 unspecified) | Skipped; specific warning |
| EC-004 | Flow with `src_ip = ::ffff:0.0.0.0` (IPv4-mapped unspecified) | After canonicalization → `0.0.0.0` → skipped with specific warning |
| EC-005 | Flow with `src_ip = 0.0.0.1` | This is NOT the unspecified address; processed normally |
| EC-006 | Flow with both src and dst as unspecified | Skipped once; skipped counter +1 (one skip per record, not per endpoint) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| conn.log line with `id.orig_h = 0.0.0.0` | `skipped=1`; warning about unspecified src; no Verdict | edge-case |
| conn.log line with `id.resp_h = ::` | `skipped=1`; warning about unspecified dst; no Verdict | edge-case |
| conn.log line with valid non-unspecified src and dst | Processed normally by BC-1.02.001 | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.02.003-a | No flow with src_ip == 0.0.0.0 or :: ever produces a Verdict | kani/proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003 ("Ingest flows") per capabilities.md §CAP-003 |
| L2 Domain Invariants | DI-013 ("malformed flow record is skipped and counted"); DEC-033 ("Flow with the unspecified address 0.0.0.0 or :: as src or dst: Skipped + counted + warned as a malformed flow — a deterministic rule (not 'likely EXTERNAL')") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-003 ("Ingest flows") per capabilities.md §CAP-003 — unspecified address handling is a required behavior of the flow ingest stage per DEC-033 and DI-013 |

## Related BCs

- BC-1.02.001 — normal flow parse (this BC gates unspecified records out before that)
- BC-1.02.002 — generic malformed skip (shares the same `skipped` counter)
- BC-1.02.005 — IPv4-mapped canonicalization (precondition for this BC's address check)

## Architecture Anchors

- `architecture/SS-02-flow-ingest.md#unspecified-addr` — unspecified address detection

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.02.003-a — no Verdict for unspecified address flows
