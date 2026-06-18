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

# BC-1.06.004: Emit Mermaid zone/conduit diagram with violations highlighted

## Description

When `--format mermaid` is specified, zonewarden emits a Mermaid diagram showing zones as nodes, conduits as edges, and violations highlighted (e.g., red edges or annotations). The diagram is a `flowchart` or `graph` Mermaid block that renders in GitHub Markdown, issue trackers, and standard Mermaid viewers. The diagram is a topology visualization — it shows the policy structure overlaid with violation evidence, supporting the "edit a policy, rerun, watch a violation appear" demo. The Mermaid output is deterministic: same ConformanceResult → same Mermaid text.

## Preconditions

1. ConformanceResult is assembled.
2. `--format mermaid` is active.

## Postconditions

1. A Mermaid diagram block is written to stdout or `--output`.
2. Each declared zone appears as a node with its id and purdue_level label.
3. Each conduit appears as an edge with direction arrows.
4. Zone-pairs with violations are highlighted (e.g., different edge style or annotation).
5. The diagram renders correctly in standard Mermaid renderers (GitHub, mermaid.js v10+).
6. The output is deterministic.

## Invariants

1. The Mermaid diagram is syntactically valid Mermaid.
2. All declared zones appear in the diagram (including zero-member zones).
3. EXTERNAL zone may appear if there are flows to/from EXTERNAL.
4. The diagram is deterministic.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Zero violations | Diagram shows all zones/conduits; no highlighted edges |
| EC-002 | All conduits violated | All conduit edges highlighted |
| EC-003 | Large policy (20 zones) | Diagram emitted; readability may degrade (ASM-005); no error |
| EC-004 | IDMZ bypass violation | IDMZ bypass shown as an annotation or highlighted cross-level link |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 2-zone policy (field, control); 1 conduit; 0 violations | Mermaid with two nodes and one edge; no highlighted violations | happy-path |
| 2-zone policy; 1 violation on the conduit edge | Mermaid with highlighted/annotated violation edge | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-1.06.004-a | Mermaid output is syntactically valid (parseable by mermaid.js) | integration test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-010 ("Render outputs: a Mermaid zone/conduit diagram with violations highlighted") per capabilities.md §CAP-010 |
| L2 Domain Invariants | DI-009 (deterministic output), DI-019 (deterministic warnings) |
| Architecture Module | [filled by architect] |
| Stories | [filled by story-writer] |
| Capability Anchor Justification | CAP-010 ("Render outputs") per capabilities.md §CAP-010 — Mermaid diagram with violations highlighted is an explicitly listed output of CAP-010 |

## Related BCs

- BC-1.06.002 — JSON output (parallel format)
- BC-1.06.008 — atomic write

## Architecture Anchors

- `architecture/SS-06-reporting.md#mermaid-diagram` — Mermaid diagram generator

## Story Anchor

[S-1.NN] — filled by story-writer

## VP Anchors

- VP-1.06.004-a — valid Mermaid syntax
