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

# BC-1.06.007: zonewarden never opens a network socket or mutates input files (offline invariant)

## Description

zonewarden is strictly offline and read-only. It never opens a network socket for any purpose (no DNS lookups, no HTTP requests, no license checks, no telemetry). It never modifies, truncates, or creates input files (the policy file and flow log are opened read-only). This is the DI-012 invariant — a correctness and safety property, not just a policy choice. It is a Kani proof target: the tool is a pure function of its file inputs.

## Preconditions

1. zonewarden is invoked with valid arguments.
2. Input files exist and are readable.

## Postconditions

1. No network socket is ever opened during the run.
2. Input files (`--policy`, `--flows`) are opened O_RDONLY; no writes, no truncation.
3. Output is written only to stdout, stderr, or the explicitly specified `--output` path.
4. No new files are created except the explicit output artifacts.

## Invariants

1. This invariant holds even in error paths: no network call is made before reporting a policy error.
2. The offline property is enforced by construction (no network-access crates in the dependency tree for the core binary).
3. The property is verifiable by system call tracing (strace/dtrace) in CI.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Run with `--policy` pointing to a network path (SMB/NFS mount) | The OS resolves the path via the filesystem; zonewarden opens the resulting file descriptor read-only; no socket is opened by zonewarden itself |
| EC-002 | Run on a machine with no network interface | Tool works identically |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Normal run | Zero network syscalls made by zonewarden | happy-path (verified via strace/dtrace integration test) |
| Normal run | Input files opened read-only; not modified | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.06.007-a | zonewarden makes no network syscalls (DI-012 offline invariant ⊢) | kani (no socket() syscall reachable from main) / strace integration test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-010 ("Render outputs") per capabilities.md §CAP-010 |
| L2 Domain Invariants | DI-012 ("Read-only / offline: zonewarden never opens a network socket, never mutates inputs, never enforces. Pure function of its file inputs. ⊢") |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-010 ("Render outputs") per capabilities.md §CAP-010 — offline operation is a cross-cutting invariant (DI-012) verified at the CLI layer; listed under CAP-010 as the output/CLI stage where the invariant is most testable |

## Related BCs

- BC-1.06.008 — atomic write (the only file-write operation permitted)

## Architecture Anchors

- `architecture/SS-06-reporting.md#offline` — offline/read-only invariant

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.06.007-a — DI-012: no network socket, no input mutation
