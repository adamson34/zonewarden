---
document_type: prd-supplement-error-taxonomy
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [prd.md]
input-hash: "[live-state]"
traces_to: specs/prd.md
---

# Error Taxonomy: zonewarden

> PRD supplement — extracted from PRD Section 5.
> Referenced by: implementer, test-writer.

## Error Categories

| Category Code | Category | Description | Subsystem |
|--------------|----------|-------------|-----------|
| POL | Policy | Policy file load, parse, and semantic validation errors | SS-01 |
| FLW | Flow | Flow ingest, normalization, and unspecified-address handling | SS-02 |
| IO | I/O | File system I/O errors (missing file, permission, disk full) | SS-01, SS-02, SS-06 |
| SYS | System | System-level errors: ingest cap, arithmetic overflow, internal | SS-02, SS-05 |

## Severity Definitions

| Severity | Meaning | Exit Code |
|----------|---------|-----------|
| broken | Cannot continue; complete abort | 2 |
| degraded | Data loss (skipped flows); run continues | 0 (or 1 with --fail-on-skipped) |
| cosmetic | Informational; no data loss | 0 |

## Error Catalog

### POL — Policy Errors (all exit 2)

| Error Code | Severity | Exit Code | Message Format | BC Reference |
|-----------|----------|-----------|---------------|--------------|
| E-POL-001 | broken | 2 | `Policy parse error: YAML syntax error in <PATH>: <detail> (line <N>)` | BC-1.01.002 |
| E-POL-002 | broken | 2 | `Policy parse error: missing required field '<field>' in <context> in <PATH>` | BC-1.01.002 |
| E-POL-003 | broken | 2 | `Policy parse error: invalid field value '<value>' for '<field>': <expected_type> in <PATH>` | BC-1.01.002 |
| E-POL-004 | broken | 2 | `Policy error: duplicate YAML key '<key>' at <PATH>:<line>` | BC-1.01.003 |
| E-POL-005 | broken | 2 | `Policy error: duplicate zone id '<id>' (first at <idx1>, duplicate at <idx2>)` | BC-1.01.004 |
| E-POL-006 | broken | 2 | `Policy error: conduit references unknown zone '<id>' (from_zone or to_zone)` | BC-1.01.004 |
| E-POL-007 | broken | 2 | `Policy error: zone id 'EXTERNAL' is reserved and cannot be declared` | BC-1.01.004 |
| E-POL-008 | broken | 2 | `Policy error: membership tie — zones '<z1>' and '<z2>' both declare <CIDR> (equal-length overlap in IPv<4|6> family)` | BC-1.01.005 |
| E-POL-009 | broken | 2 | `Policy error: zone '<id>' declares <0.0.0.0/0 or ::/0> catch-all member — would shadow EXTERNAL zone` | BC-1.01.006 |
| E-POL-010 | broken | 2 | `Policy error: unrecognized direction token '<token>' in conduit <from>→<to>. Legal values: forward, bidirectional, unidirectional` | BC-1.01.007 |
| E-POL-011 | broken | 2 | `Policy error: unrecognized proto token '<token>' in conduit <from>→<to>. Legal values: tcp, udp, icmp, other:<0-255>` | BC-1.01.007 |
| E-POL-012 | broken | 2 | `Policy error: malformed PortSet '<expression>' in conduit <from>→<to>: <detail>` | BC-1.01.007 |
| E-POL-013 | broken | 2 | `Policy error: purdue_level '<value>' is not a valid level. Legal values: L0, L1, L2, L3, IDMZ, L4, L5` | BC-1.01.002 |

### POL — Policy Warnings (exit 0; cosmetic)

| Warning Code | Severity | Message Format | BC Reference |
|-------------|----------|---------------|--------------|
| W-POL-001 | cosmetic | `WARNING: zone '<id>' has no members and will never match any endpoint` | BC-1.01.008 |
| W-POL-002 | cosmetic | `WARNING: zone '<id>' member '<CIDR>' has a suspiciously broad prefix (/<N> < /8) — review intent` | BC-1.01.004 (OQ-004) |

### FLW — Flow Errors (degraded; run continues)

| Error Code | Severity | Exit Code | Message Format | BC Reference |
|-----------|----------|-----------|---------------|--------------|
| E-FLW-001 | degraded | 0* | `Flow warning: <N> flow record(s) skipped (malformed/unparseable)` | BC-1.02.002 |
| E-FLW-002 | degraded | 0* | `Flow warning: flow at line <N> skipped — unspecified address (0.0.0.0/::) in src or dst` | BC-1.02.003 |

> *Exit 0 unless `--fail-on-skipped`, in which case exit 1.

### IO — I/O Errors (all exit 2)

| Error Code | Severity | Exit Code | Message Format | BC Reference |
|-----------|----------|-----------|---------------|--------------|
| E-IO-001 | broken | 2 | `I/O error: policy file not found: <PATH>` | BC-1.01.002 (FM-001) |
| E-IO-002 | broken | 2 | `I/O error: policy file not readable: <PATH>: <os_error>` | BC-1.01.002 (FM-001) |
| E-IO-003 | broken | 2 | `I/O error: flow file not found: <PATH>` | BC-1.06.001 (FM-003) |
| E-IO-004 | broken | 2 | `I/O error: flow file not readable: <PATH>: <os_error>` | BC-1.06.001 (FM-003) |
| E-IO-005 | broken | 2 | `I/O error: cannot write output to <PATH>: <os_error>` | BC-1.06.008 (FM-006) |
| E-IO-006 | broken | 2 | `I/O error: cannot create temp file in <DIR>: <os_error>` | BC-1.06.008 (FM-006) |

### SYS — System Errors (all exit 2)

| Error Code | Severity | Exit Code | Message Format | BC Reference |
|-----------|----------|-----------|---------------|--------------|
| E-SYS-001 | broken | 2 | `Ingest cap exceeded: max_flows = <N>. Run aborted after <N> flows. Re-run with --max-flows to increase the cap or split the input.` | BC-1.02.006 (FM-008) |
| E-SYS-002 | broken | 2 | `Usage error: --max-flows must be > 0` | BC-1.02.006 |
| E-SYS-003 | broken | 2 | `Internal error: tally counter overflow (u64::MAX exceeded). This should be unreachable due to the max_flows cap. Please report a bug.` | BC-1.05.004 (FM-009) |

## Error Disambiguation Table

> Helps distinguish similar error conditions.

| Situation | Error Code | Exit |
|-----------|------------|------|
| Policy file missing | E-IO-001 | 2 |
| Policy file YAML syntax error | E-POL-001 | 2 |
| Policy file schema mismatch | E-POL-002/003 | 2 |
| Policy semantic error (tie, duplicate id, /0) | E-POL-005 through E-POL-012 | 2 |
| Flow file missing | E-IO-003 | 2 |
| Individual flow line malformed | E-FLW-001 (warning) | 0* |
| Flow with unspecified address | E-FLW-002 (warning) | 0* |
| Flow cap exceeded | E-SYS-001 | 2 |
| Output file write error | E-IO-005/006 | 2 |
| Violations found | (no error; exit 1) | 1 |
| Conformant | (no error) | 0 |
