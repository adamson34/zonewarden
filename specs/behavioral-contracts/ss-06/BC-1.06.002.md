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
subsystem: "SS-06"
capability: "CAP-010"
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

# BC-1.06.002: Emit JSON violations report with all required fields

## Description

When `--format json` is specified (or when output format defaults include JSON), zonewarden emits a JSON serialization of the ConformanceResult to stdout or the specified `--output` path. The JSON schema is fixed and versioned. Every field in ConformanceResult must be present in the JSON output; optional/None fields are omitted (not null). The JSON output is deterministic (same inputs → byte-identical JSON) because the underlying ConformanceResult is ordered per BC-1.05.002.

**Required top-level fields:** `schema_version`, `policy_digest`, `total_flows`, `intra_zone`, `allowed`, `no_matching_conduit`, `wrong_direction`, `multicast_exempt`, `idmz_bypasses`, `distinct_violating_flows`, `external_endpoints`, `skipped`, `warnings`, `violations`.

Each violation: `flow_index`, `src_zone`, `dst_zone`, `kind`, `severity`, `explanation`, `src_ip`, `dst_ip`, `src_port` (optional), `dst_port` (optional), `proto`, `service` (optional), `service_source`.

## Preconditions

1. A ConformanceResult has been assembled (ST-7 complete).
2. `--format json` is active.

## Postconditions

1. Valid JSON is written to stdout or the `--output` file.
2. JSON is deterministic: same ConformanceResult → same JSON bytes.
3. The `schema_version` field is present with the current schema version string.
4. The output is valid against the JSON schema defined in `prd-supplements/interface-definitions.md`.

## Invariants

1. The JSON is always valid UTF-8.
2. Violations are ordered per BC-1.05.002 (deterministic order).
3. `schema_version` is a string in `"major.minor"` format; current: `"1.0"`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Zero violations | `violations: []`; all tallies 0 |
| EC-002 | Violation with no `src_port` (portless ICMP) | `src_port` field omitted from violation JSON object |
| EC-003 | `service` is None | `service` field omitted |
| EC-004 | `idmz_bypass` AND conduit violation for same flow | Two separate violation objects (both in `violations` array; same `flow_index`) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| ConformanceResult with 1 NoMatchingConduit violation | JSON with `total_flows: 1`, `no_matching_conduit: 1`, `violations: [{flow_index: 0, kind: "NoMatchingConduit", ...}]` | happy-path |
| All-zero ConformanceResult | JSON with all tallies 0, `violations: []` | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.06.002-a | JSON output validates against the schema in interface-definitions.md | integration test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-010 ("Render outputs: Emit a human-readable + JSON violations report") per capabilities.md §CAP-010 |
| L2 Domain Invariants | DI-009 (deterministic output), DI-019 (deterministic warning order) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-010 ("Render outputs") per capabilities.md §CAP-010 — machine-readable JSON report is an explicitly listed output of CAP-010 |

## Related BCs

- BC-1.05.002 — deterministic ordering (precondition for byte-identical JSON)
- BC-1.06.008 — atomic write (JSON output must be written atomically)

## Architecture Anchors

- `architecture/SS-06-reporting.md#json-report` — JSON serialization

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.06.002-a — JSON validates against schema
