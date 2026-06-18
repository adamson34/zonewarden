---
document_type: prd-supplement-interface-definitions
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

# Interface Definitions: zonewarden

> PRD supplement — extracted from PRD Section 3.
> Referenced by: implementer, test-writer, devops-engineer.

## CLI Interface

```
zonewarden — OT Segmentation-as-Code conformance validator

USAGE:
    zonewarden [OPTIONS] --policy <PATH> --flows <PATH>

REQUIRED:
    --policy <PATH>      Path to YAML segmentation policy file
    --flows <PATH>       Path to Zeek conn.log flow capture file

OPTIONS:
    --format <FORMAT>    Output format [default: text]
                         Possible values: text, json, mermaid
    --output <PATH>      Write output to file instead of stdout
                         (Atomic write: write-to-temp then rename)
    --fail-on-skipped    Exit 1 if any flow records were skipped
                         (default: exit 0 with a warning when skipped > 0)
    --max-flows <N>      Maximum number of flows to process [default: 1000000]
                         Abort with exit 2 if the cap is reached. N must be > 0.
    -h, --help           Print help
    -V, --version        Print version

EXAMPLES:
    # Run against captured flows; text report to stdout
    zonewarden --policy policy.yaml --flows capture.log

    # Machine-readable JSON report; CI strict mode
    zonewarden --policy policy.yaml --flows capture.log --format json --fail-on-skipped

    # Generate Mermaid diagram
    zonewarden --policy policy.yaml --flows capture.log --format mermaid --output topology.md

    # Custom flow cap
    zonewarden --policy policy.yaml --flows large.log --max-flows 5000000
```

## Exit Code Semantics

| Code | Meaning | Condition |
|------|---------|-----------|
| 0 | Conformant | Policy valid; flows processed; `distinct_violating_flows == 0 && idmz_bypasses == 0` |
| 1 | Violations found | Policy valid; flows processed; `distinct_violating_flows > 0 || idmz_bypasses > 0`; OR `--fail-on-skipped` with `skipped > 0` |
| 2 | Error | Policy parse/validation error; I/O error (missing/unreadable file); usage error; ingest cap breach; u64 overflow |

## YAML Policy Schema

```yaml
# zonewarden policy schema v1.0
# Top-level structure
zones:
  - id: <string>                    # required; unique; not "EXTERNAL"
    name: <string>                  # required; human-readable label
    purdue_level: <level>           # required; one of: L0, L1, L2, L3, IDMZ, L4, L5
    sl_t: <sl_target>               # optional; IEC 62443 Security Level Target
    members:                        # optional (zero members is legal, emits warning)
      - <cidr>                      # e.g. "10.0.1.0/24" or "192.168.1.5/32"
      # Note: 0.0.0.0/0 and ::/0 are rejected (policy error)

conduits:
  - from_zone: <zone_id>            # required; zone id or "EXTERNAL"
    to_zone: <zone_id>              # required; zone id or "EXTERNAL"
    direction: <direction>          # required; one of: forward, bidirectional, unidirectional (=forward alias)
    proto: <proto>                  # required; one of: tcp, udp, icmp, other:<u8>
    ports: <portset>                # required; "any" or list of ports/ranges e.g. [502, 44818, 4840-4843]

# sl_t format (two forms):
# Scalar: sl_t: 3            (overall SL target)
# Vector: sl_t:              (full 7-element FR vector)
#   overall: 3
#   fr_vector: [2, 3, 3, 2, 2, 3, 2]   # FR1..FR7, each 0..=4

# PortSet expressions:
#   any              -- matches any port AND portless protocols (ICMP/Other)
#   [502]            -- single port
#   [500-510]        -- inclusive range
#   [502, 4840-4843] -- mixed list; adjacent entries coalesced at load
#   Note: [0-65535] is distinct from "any" -- does NOT match portless protocols
```

## JSON Output Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://github.com/zonewarden/report-schema/v1.0",
  "type": "object",
  "required": [
    "schema_version", "policy_digest", "total_flows",
    "intra_zone", "allowed", "no_matching_conduit", "wrong_direction",
    "multicast_exempt", "idmz_bypasses", "distinct_violating_flows",
    "external_endpoints", "skipped", "warnings", "violations"
  ],
  "properties": {
    "schema_version": {
      "type": "string",
      "description": "Schema version; current: \"1.0\""
    },
    "policy_digest": {
      "type": "string",
      "pattern": "^[0-9a-f]{64}$",
      "description": "SHA-256 of canonical policy serialization (lowercase hex)"
    },
    "total_flows": { "type": "integer", "minimum": 0 },
    "intra_zone": { "type": "integer", "minimum": 0 },
    "allowed": { "type": "integer", "minimum": 0 },
    "no_matching_conduit": { "type": "integer", "minimum": 0 },
    "wrong_direction": { "type": "integer", "minimum": 0 },
    "multicast_exempt": { "type": "integer", "minimum": 0 },
    "idmz_bypasses": { "type": "integer", "minimum": 0 },
    "distinct_violating_flows": { "type": "integer", "minimum": 0 },
    "external_endpoints": {
      "type": "integer", "minimum": 0,
      "description": "Flows with ≥1 EXTERNAL endpoint; diagnostic only; excluded from DI-015 identity"
    },
    "skipped": { "type": "integer", "minimum": 0 },
    "warnings": {
      "type": "array",
      "items": { "type": "string" }
    },
    "violations": {
      "type": "array",
      "items": { "$ref": "#/$defs/Violation" }
    }
  },
  "$defs": {
    "Violation": {
      "type": "object",
      "required": ["flow_index", "src_zone", "dst_zone", "kind", "severity", "explanation",
                   "src_ip", "dst_ip", "proto", "service_source"],
      "properties": {
        "flow_index": { "type": "integer", "minimum": 0 },
        "src_zone": { "type": "string" },
        "dst_zone": { "type": "string" },
        "kind": {
          "type": "string",
          "enum": ["NoMatchingConduit", "WrongDirection", "IdmzBypass"]
        },
        "severity": {
          "type": "string",
          "enum": ["Established", "Attempted"]
        },
        "explanation": { "type": "string" },
        "src_ip": { "type": "string", "description": "IPv4 or IPv6 address string" },
        "dst_ip": { "type": "string" },
        "src_port": { "type": "integer", "minimum": 0, "maximum": 65535 },
        "dst_port": { "type": "integer", "minimum": 0, "maximum": 65535 },
        "proto": { "type": "string" },
        "service": { "type": "string", "description": "Present only if service inferred" },
        "service_source": {
          "type": "string",
          "enum": ["DpiConfirmed", "PortHeuristic", "Unknown"]
        },
        "conn_state": { "type": "string", "description": "Zeek conn_state if present" }
      }
    }
  }
}
```

## Flag Interactions

| Flag A | Flag B | Interaction | Resolution |
|--------|--------|-------------|------------|
| `--format text` | `--format json` | conflicts (mutually exclusive) | Last flag wins; warn if both provided |
| `--format mermaid` | `--format json` | conflicts | Last flag wins |
| `--output <PATH>` | `--format text` | compatible | Writes text to file atomically |
| `--fail-on-skipped` | (any) | additive | Upgrades exit 0 to exit 1 when `skipped > 0` |
| `--max-flows 0` | (any) | error | Usage error; exit 2: "max_flows must be > 0" |
| `--max-flows <N>` | `--fail-on-skipped` | compatible | Both apply independently |

## Stdout vs Stderr

| Stream | Content |
|--------|---------|
| stdout | Main output (text report, JSON, or Mermaid) |
| stderr | Warnings, diagnostics, error messages |

## Environment Variables

None in v1.0. All configuration is via CLI flags.
