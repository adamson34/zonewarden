---
document_type: behavioral-contract-id-mapping
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-17T00:00:00
phase: 1a
traces_to: specs/prd.md
---

# BC ID Allocation Mapping: zonewarden

> This file tracks BC ID allocations to prevent collisions and support future additions.
> IDs are append-only: never reuse a retired ID (DF-030 append_only_numbering).

## ID Schema

`BC-S.SS.NNN`
- `S = 1` (PRD Section 1 = the primary product spec)
- `SS` = subsystem number (01-06 for MVP; 07+ for future)
- `NNN` = sequential within subsystem (001-999)

## Allocated Ranges

| Subsystem | SS | Allocated Range | Status |
|-----------|-----|----------------|--------|
| SS-01 Policy | 01 | BC-1.01.001 – BC-1.01.009 | active |
| SS-02 Flow Ingest | 02 | BC-1.02.001 – BC-1.02.006 | active |
| SS-03 Zone Resolution | 03 | BC-1.03.001 – BC-1.03.005 | active |
| SS-04 Classification | 04 | BC-1.04.001 – BC-1.04.011 | active |
| SS-05 Aggregation | 05 | BC-1.05.001 – BC-1.05.005 | active |
| SS-06 Reporting | 06 | BC-1.06.001 – BC-1.06.008 | active |

## Next Available IDs

| Subsystem | Next Available ID |
|-----------|------------------|
| SS-01 | BC-1.01.010 |
| SS-02 | BC-1.02.007 |
| SS-03 | BC-1.03.006 |
| SS-04 | BC-1.04.012 |
| SS-05 | BC-1.05.006 |
| SS-06 | BC-1.06.009 |

## Future Subsystem Reservations (P1/P2, no BCs yet)

| Planned Subsystem | SS | Status |
|------------------|-----|--------|
| SS-07 NetFlow/IPFIX adapter (CAP-011) | 07 | reserved for Phase 2 |
| SS-08 SVG rendering (CAP-012) | 08 | reserved for Phase 2 |
| SS-09 Policy DSL (CAP-013) | 09 | reserved for Phase 3 |
| SS-10 Firewall config adapter (CAP-014) | 10 | reserved for Phase 3 |

## Retired IDs

> None retired yet. Retired entries will remain here with `status: retired` and `replaced_by` links.

(none)
