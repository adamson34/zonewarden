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
capability: "CAP-004"
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

# BC-1.02.004: Assign service and service_source via canonical port/proto table

## Description

After a flow record is parsed, the app-layer service is inferred by matching the `(dst_port, proto)` pair against the canonical OT service/port table. If the pair matches a table entry exactly, `service_source = PortHeuristic`. If no match, `service_source = Unknown`. `DpiConfirmed` is an enum variant reserved for future DPI-capable adapters; the MVP Zeek conn.log adapter never produces it (OQ-002). Inference is always marked as heuristic (DI-008) — the inferred service is never presented as authoritative. Port-only matching is not performed if the transport does not match the canonical entry.

**Canonical MVP service/port table (from entities.md):**

| Service | Port | Transport |
|---------|------|-----------|
| Modbus | 502 | TCP |
| DNP3 | 20000 | TCP |
| EtherNet/IP | 44818 | TCP |
| S7comm | 102 | TCP |
| BACnet/IP | 47808 | UDP |
| OPC UA | 4840 | TCP |

## Preconditions

1. A flow record has been parsed into a `Flow` (after BC-1.02.001 processing).
2. `dst_port` may be `None` (portless protocol such as ICMP).

## Postconditions

1. `service_source` is set to `PortHeuristic` if `(dst_port, proto)` exactly matches a canonical table entry; `Unknown` otherwise.
2. `service` is set to the matched service variant if `PortHeuristic`; `None` if `Unknown`.
3. `service_source` is never `None` (DI-008).
4. Mismatched transport (e.g., `47808/TCP` when BACnet expects UDP) → `Unknown`, not `PortHeuristic`.
5. Non-default port (e.g., Modbus on port 1502) → `Unknown`.
6. Portless flow (ICMP or Other proto with `dst_port: None`) → `service = None`, `service_source = Unknown`.

## Invariants

1. `service_source` is always set; never `None`.
2. Inference is purely heuristic; no authoritative claim is ever made (DI-008).
3. `DpiConfirmed` is never produced by the Zeek adapter.
4. The canonical table is fixed and version-pinned; changes require a spec revision.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `dst_port=502, proto=TCP` | `service=Modbus, service_source=PortHeuristic` |
| EC-002 | `dst_port=502, proto=UDP` (transport mismatch) | `service=None, service_source=Unknown` (DEC-008) |
| EC-003 | `dst_port=1502, proto=TCP` (Modbus on non-default port) | `service=None, service_source=Unknown` (DEC-007) |
| EC-004 | `dst_port=20000, proto=UDP` (DNP3 UDP variant) | `Unknown` — UDP/20000 is not in the canonical TCP-only DNP3 entry (ASM-009) |
| EC-005 | `dst_port=102, proto=TCP` (S7comm or IEC 61850 MMS) | `service=S7comm, service_source=PortHeuristic` — known ambiguity; port 102/TCP always infers S7comm in MVP (ASM-009) |
| EC-006 | ICMP flow (`proto=ICMP, dst_port=None`) | `service=None, service_source=Unknown` |
| EC-007 | `dst_port=47808, proto=UDP` (BACnet) | `service=BACnet, service_source=PortHeuristic` |
| EC-008 | `dst_port=4840, proto=TCP` (OPC UA) | `service=OpcUa, service_source=PortHeuristic` |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `(dst_port=502, proto=TCP)` | `service=Modbus, service_source=PortHeuristic` | happy-path |
| `(dst_port=502, proto=UDP)` | `service=None, service_source=Unknown` | edge-case |
| `(dst_port=1502, proto=TCP)` | `service=None, service_source=Unknown` | edge-case |
| `(dst_port=None, proto=ICMP)` | `service=None, service_source=Unknown` | edge-case |
| `(dst_port=9999, proto=TCP)` | `service=None, service_source=Unknown` | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.02.004-a | `service_source` is always `Some(_)` on every produced Flow | kani |
| VP-1.02.004-b | `DpiConfirmed` is never produced by the Zeek adapter | unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-004 ("Normalize + infer service: Map adapter-specific records into the neutral Flow; infer app service from port when not provided, stamping service_source provenance") per capabilities.md §CAP-004 |
| L2 Domain Invariants | DI-008 ("service_source is always set; port-derived services are PortHeuristic or Unknown and never reported as authoritative") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-004 ("Normalize + infer service") per capabilities.md §CAP-004 — this BC implements the port/proto-based service inference and service_source provenance stamping that CAP-004 requires |

## Related BCs

- BC-1.02.001 — flow parse (precondition)
- BC-1.06.003 — text report (must surface service_source; heuristics flagged visibly)

## Architecture Anchors

- `architecture/SS-02-flow-ingest.md#service-inference` — canonical service/port table and inference logic

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.02.004-a — service_source always set
- VP-1.02.004-b — DpiConfirmed never produced by Zeek adapter
