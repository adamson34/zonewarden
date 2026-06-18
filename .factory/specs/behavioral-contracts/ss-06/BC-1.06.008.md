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

# BC-1.06.008: Output artifacts written atomically (write-then-rename; no partial files on error)

## Description

When zonewarden writes output to a file (`--output <PATH>`), it must use an atomic write strategy: write to a temporary file in the same directory, then atomically rename to the target path. This ensures that an interrupted run (disk full, crash) never produces a partial/corrupt output file. The target path either has the complete new content or the previous content — never a partially-written state. This implements FM-006.

## Preconditions

1. `--output <PATH>` is specified.
2. The output has been fully assembled in memory.

## Postconditions

1. The output is written to a temporary file in the same directory as the target path.
2. The temporary file is atomically renamed to the target path on success.
3. If any error occurs during write or rename: the temporary file is deleted; the original target (if any) is unchanged; exit 2 with diagnostic.

## Invariants

1. The target path is never in a partially-written state.
2. Write-then-rename is atomic on POSIX systems (same filesystem).
3. On error: stderr identifies which artifact failed and why.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Disk full during write of temp file | Write fails; temp file cleaned up; exit 2; original target unchanged |
| EC-002 | Target directory not writable | Write fails at temp file creation; exit 2 |
| EC-003 | Output to stdout (no `--output` flag) | Atomic write not applicable; stdout is streamed directly |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `--output report.json`; successful run | `report.json` contains complete JSON output | happy-path |
| `--output /nonexistent/dir/report.json` | Exit 2; "cannot write output: /nonexistent/dir/ is not writable" | error |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.06.008-a | No partial output files on any error path | unit test (mock filesystem; inject errors) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-010 ("Render outputs") per capabilities.md §CAP-010 |
| L2 Domain Invariants | FM-006 ("Output path unwritable / disk full: Exit code 2; never corrupt a partial file (write-then-rename)") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-010 ("Render outputs") per capabilities.md §CAP-010 — atomic output write is a required behavior of the render/output stage per FM-006 |

## Related BCs

- BC-1.06.001 — exit code on I/O error
- BC-1.06.007 — offline invariant (output is the only write)

## Architecture Anchors

- `architecture/SS-06-reporting.md#atomic-write` — write-then-rename implementation

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.06.008-a — no partial output on error
