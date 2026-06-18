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

# BC-1.06.001: Exit codes: 0 conformant, 1 violations, 2 error/policy/usage

## Description

zonewarden uses a three-value exit code scheme that makes it CI-friendly and scriptable. The exit code is the primary machine-readable signal: `0` means the network conforms to policy (no violations); `1` means violations were found; `2` means the tool could not complete a conformance check (policy error, I/O error, usage error, or cap breach). These codes are fixed and must not change between versions without a major version bump.

| Code | Meaning | Condition |
|------|---------|-----------|
| 0 | Conformant | Policy valid; flows processed; `distinct_violating_flows == 0` and `idmz_bypasses == 0` |
| 1 | Violations found | Policy valid; flows processed; `distinct_violating_flows > 0` OR `idmz_bypasses > 0` |
| 2 | Error | Policy load/validation error; I/O error; usage error; ingest cap breach; overflow |

## Preconditions

1. The tool has completed its processing (or aborted on error).

## Postconditions

1. Process exits with the correct code per the table above.
2. With `--fail-on-skipped`: if `skipped > 0` and exit would have been 0, exit 1 instead (BC-1.06.006).
3. Warnings (skipped flows) do NOT change the exit code on their own — only `--fail-on-skipped` does.

## Invariants

1. Exit code 2 is always used for infrastructure/error conditions, never for conformance results.
2. Exit code 0 means BOTH `distinct_violating_flows == 0` AND `idmz_bypasses == 0`.
3. Exit code 1 fires if EITHER has violations or IDMZ bypasses.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Policy error | Exit 2; no flows processed |
| EC-002 | I/O error on flow file | Exit 2 |
| EC-003 | Zero flows; no violations | Exit 0 |
| EC-004 | Flows with violations but `skipped > 0` | Exit 1 (violations take precedence; skipped doesn't change) |
| EC-005 | No violations; `skipped > 0`; `--fail-on-skipped` set | Exit 1 (flag upgrades exit from 0 to 1) |
| EC-006 | No violations; `skipped = 0` | Exit 0 |
| EC-007 | IDMZ bypass only (all conduit verdicts = Allowed) | Exit 1 (idmz_bypasses > 0) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Valid policy; 100 flows; no violations | Exit 0 | happy-path |
| Valid policy; 100 flows; 3 violations | Exit 1 | happy-path |
| Invalid YAML policy | Exit 2 | error |
| Missing flow file | Exit 2 | error |
| Valid policy; flows with IDMZ bypass but all Allowed conduit verdicts | Exit 1 | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.06.001-a | `distinct_violating_flows == 0 && idmz_bypasses == 0` → exit 0 | proptest |
| VP-1.06.001-b | Any error condition → exit 2 (never exit 0 or 1) | proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-010 ("Render outputs: exit code reflects conformance") per capabilities.md §CAP-010 |
| L2 Domain Invariants | FM-001..FM-006 (error conditions → exit 2), DI-013 (skipped > 0 → warning, not exit change without flag) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-010 ("Render outputs") per capabilities.md §CAP-010 — exit code semantics are a required output behavior of CAP-010 |

## Related BCs

- BC-1.06.006 — --fail-on-skipped flag (modifies exit code)
- BC-1.05.001 — accounting identity (determines whether violations > 0)

## Architecture Anchors

- `architecture/SS-06-reporting.md#exit-codes` — exit code mapping

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.06.001-a — conformant exit 0
- VP-1.06.001-b — error → exit 2
