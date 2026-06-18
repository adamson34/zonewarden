---
document_type: domain-spec-section
level: L2
section: failure-modes
version: "1.8"
status: draft
producer: business-analyst
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[live-state]"
traces_to: L2-INDEX.md
---

# Failure Modes

> Runtime failures (distinct from edge cases). zonewarden is offline & batch, so failure modes
> are mostly I/O and malformed-input handling. Guiding principle: **fail loud on policy errors,
> degrade gracefully on individual flow errors.**

| ID | Subsystem | Failure Mode | Impact | Detection | Recovery |
|----|-----------|--------------|--------|-----------|----------|
| FM-001 | ST-1 Load policy | Policy file missing / unreadable | Cannot run | I/O error at open | Exit code 2 (usage/config) with path + reason |
| FM-002 | ST-1/ST-2 | Policy malformed (YAML syntax) or schema-invalid (DI-010) | Cannot run safely | Parser / validator error | Fail fast, precise diagnostic (line/field), no flow processing (DI-011) |
| FM-003 | ST-3 Ingest | Flow source missing / unreadable | No flows to check | I/O error at open | Exit code 2; distinguish from "empty but valid" (DEC-014) |
| FM-004 | ST-3/ST-4 | Individual flow line malformed | One record lost | Per-line parse failure | Skip + increment `skipped`; continue (DEC-010); surface count in report |
| FM-005 | ST-4 | Unknown/unmappable transport or service | Reduced fidelity | service inference yields none | `service = None`, `service_source = Unknown`; still classify |
| FM-006 | ST-8 Render | Output path unwritable / disk full | No artifact persisted | I/O error on write | Exit code 2; report which artifact failed; never corrupt a partial file (write-then-rename) |
| FM-007 | ST-3 (ingest) | Very large flow input exhausts memory | Crash / OOM | resource pressure | Stream flows via the ST-3 iterator (don't materialize all); bounded memory per flow; documented scale limits in PRD NFRs |
| FM-008 | ST-3 (ingest guard) | Input exceeds the `max_flows` cap | Unbounded run / overflow risk | flow count reaches `max_flows` | A configured **`max_flows` ingest cap** (constant, default pinned in PRD, e.g. `1_000_000_000`, far below `u64::MAX`) enforced at ST-3; reaching it **aborts with exit code 2** + diagnostic (distinct from a violations exit) |
| FM-009 | ST-7 (aggregate) | `u64` tally counter overflow | Wrong-but-plausible counts (silent in release Rust) | counter near `u64::MAX` | **Checked arithmetic (`checked_add`)**; on overflow **abort with exit code 2** + diagnostic (never saturate, never silent wrap). The FM-008 ingest cap makes this an unreachable defense-in-depth backstop |
