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

# BC-1.02.001: Parse valid Zeek conn.log line into normalized Flow

## Description

Each parseable line from a Zeek `conn.log` file is converted into exactly one normalized `Flow` record. The Zeek conn.log uses TSV format with a well-defined field order. The `src` field is set to the connection **originator** (`id.orig_h`/`id.orig_p`); the `dst` is the responder. Each successfully parsed flow receives a dense, gap-free `flow_index` (u64) starting at 0, counting only successfully normalized flows (not skipped records). Zeek-specific fields that have no Flow analog are discarded.

## Preconditions

1. Policy has been validated (ST-2 complete).
2. `--flows <PATH>` specifies a readable Zeek conn.log file.
3. The line being processed is a valid Zeek TSV data row (not a header/comment line starting with `#`).
4. All required fields are present and have correct types: `ts`, `id.orig_h`, `id.orig_p`, `id.resp_h`, `id.resp_p`, `proto`.

## Postconditions

1. A `Flow` is produced with:
   - `flow_index`: the sequential index among successfully-normalized flows (0-based, dense).
   - `ts`: UTC timestamp at nanosecond precision (Zeek fractional-epoch seconds widened, not truncated).
   - `src_ip` = `id.orig_h` (the Zeek originator host).
   - `src_port` = `id.orig_p` (None if 0 or `-`).
   - `dst_ip` = `id.resp_h`.
   - `dst_port` = `id.resp_p` (None if 0 or `-`).
   - `proto` = mapped from Zeek `proto` field (`tcp`/`udp`/`icmp`/`Other(u8)`).
   - `service`: inferred by BC-1.02.004 (may be None).
   - `service_source`: set by BC-1.02.004 (never absent).
   - `conn_state`: mapped from Zeek `conn_state` field (None if absent/`-`).
2. IPv4-mapped IPv6 addresses (`::ffff:a.b.c.d`) in `src_ip` or `dst_ip` are canonicalized to IPv4 (BC-1.02.005).
3. Flows with unspecified addresses (0.0.0.0 or :: in src or dst) are skipped per BC-1.02.003 before this BC is reached.

## Invariants

1. One Zeek conn.log line = exactly one `Flow` (never two, never zero for a parseable line).
2. The connection originator is always `src` (DEC-016); return/responder traffic is NOT treated as a separate flow to avoid false WrongDirection violations.
3. `service_source` is always set (DI-008) — never `None`.
4. `flow_index` values are dense and gap-free among successfully-normalized flows.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Zeek conn.log header line `#fields ts id.orig_h ...` | Skipped (not a data row); does not increment `skipped` counter (it is metadata, not a failed flow parse) |
| EC-002 | Zeek conn.log separator line `#separator ...` | Skipped as metadata |
| EC-003 | Line with `conn_state: -` (Zeek sentinel for missing) | `conn_state: None` in the Flow |
| EC-004 | Line with `id.orig_p: 0` (e.g., ICMP flow) | `src_port: None` |
| EC-005 | Line with `service: dns` in Zeek's own service field | Zeek service field is informational; `service_source` is still `PortHeuristic` or `Unknown` based on port inference (Zeek's service detection is not `DpiConfirmed` in MVP) |
| EC-006 | Line with an IPv4-mapped IPv6 source `::ffff:10.0.1.5` | Canonicalized to `10.0.1.5` before further processing (BC-1.02.005) |
| EC-007 | conn.log with Windows-style CRLF line endings | Parsed correctly; line endings stripped |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Valid Zeek line: `1717200000.123456\t10.0.1.5\t1234\t10.0.2.10\t502\ttcp\tSF\t-\t...` | `Flow { flow_index:0, src_ip:10.0.1.5, src_port:1234, dst_ip:10.0.2.10, dst_port:502, proto:Tcp, service:Some(Modbus), service_source:PortHeuristic, conn_state:Some(Established) }` | happy-path |
| Zeek header line `#fields ts id.orig_h ...` | Skipped; `flow_index` not incremented; `skipped` counter not incremented | edge-case |
| Line with `::ffff:10.0.1.5` as src | Canonicalized to `10.0.1.5`; Flow with IPv4 src | edge-case |
| Line with `conn_state: -` | `conn_state: None` | edge-case |
| Line missing the `proto` field (truncated) | Skipped; `skipped` incremented; BC-1.02.002 applies | error |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.02.001-a | `flow_index` is dense and gap-free for all successfully parsed flows | proptest |
| VP-1.02.001-b | `service_source` is always set on every produced Flow | kani |
| VP-1.02.001-c | Parser is panic-free on arbitrary input bytes | fuzz (cargo-fuzz) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003 ("Ingest flows: Read observed flows from a RealitySource adapter; MVP adapter = Zeek conn.log (one connection record → exactly one Flow, keyed on the originator)") per capabilities.md §CAP-003 |
| L2 Domain Invariants | DI-013 (ingest resilience), DI-008 (service provenance) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-003 ("Ingest flows") per capabilities.md §CAP-003 — this BC implements the Zeek conn.log adapter that is the v1 RealitySource |

## Related BCs

- BC-1.02.002 — malformed line handling (companion)
- BC-1.02.003 — unspecified address skip (precondition)
- BC-1.02.004 — service inference (composes with)
- BC-1.02.005 — IPv4-mapped canonicalization (composes with)

## Architecture Anchors

- `architecture/SS-02-flow-ingest.md#zeek-adapter` — Zeek conn.log adapter

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.02.001-a — dense flow_index
- VP-1.02.001-b — service_source always set
- VP-1.02.001-c — fuzz: no parser panics
