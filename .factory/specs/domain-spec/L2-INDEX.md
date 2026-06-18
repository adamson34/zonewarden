---
document_type: domain-spec-index
level: L2
version: "1.8"
status: draft
producer: business-analyst
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[live-state]"
traces_to: product-brief.md
sections:
  - capabilities.md
  - entities.md
  - invariants.md
  - events.md
  - edge-cases.md
  - assumptions.md
  - risks.md
  - failure-modes.md
  - differentiators.md
  - ubiquitous-language.md
---

# L2 Domain Specification: zonewarden

> **Sharded artifact (DF-021).** Navigation + summary. Detail lives in the
> per-section files listed below.

## Domain Summary

zonewarden models **OT network segmentation conformance**: given a declarative IEC
62443 zone/conduit **Policy** and a set of observed network **Flows**, it deterministically
classifies every flow as allowed or violating and explains why — an automated
**FR5 (Restricted Data Flow)** conformance checker for the Purdue model.

## Document Map

| Section | File | Primary Consumer | Purpose |
|---------|------|-----------------|---------|
| Domain Capabilities | capabilities.md | product-owner, architect, story-writer | CAP-NNN capability catalog |
| Domain Entities | entities.md | architect, product-owner | Entity model: attributes + invariants |
| Domain Invariants | invariants.md | product-owner, architect | DI-NNN business rules |
| Domain Events / Stages | events.md | architect | Processing pipeline stages |
| Edge Cases | edge-cases.md | story-writer, test-writer | DEC-NNN domain edge cases |
| Assumptions | assumptions.md | product-owner, test-writer | ASM-NNN + validation methods |
| Risks | risks.md | product-owner, architect | R-NNN risk register |
| Failure Modes | failure-modes.md | architect, test-writer | FM-NNN runtime failure catalog |
| Differentiators | differentiators.md | product-owner | Differentiator → CAP-NNN mapping |
| Ubiquitous Language | ubiquitous-language.md | all | Glossary of domain terms |

## Cross-References

| If you need... | Read these together |
|----------------|---------------------|
| BC creation input | capabilities.md + invariants.md + edge-cases.md + assumptions.md + risks.md + differentiators.md |
| Architecture design input | capabilities.md + entities.md + invariants.md + events.md + risks.md + failure-modes.md |
| Story decomposition input | capabilities.md + edge-cases.md |
| Holdout scenario generation | assumptions.md + risks.md + failure-modes.md |
| Full domain review | ALL sections |

## ID Registry Summary

| ID Format | Count | Section |
|-----------|-------|---------|
| CAP-NNN | 14 | capabilities.md |
| DI-NNN | 20 | invariants.md |
| ST-N | 8 | events.md |
| DEC-NNN | 33 | edge-cases.md |
| ASM-NNN | 9 | assumptions.md |
| R-NNN | 6 | risks.md |
| FM-NNN | 9 | failure-modes.md |

> **ID-count is authoritative as listed here** (verified contiguous: DI-001..020, DEC-001..033, FM-001..009, ASM-001..009, CAP-001..014, ST-1..8, R-001..006). Any earlier "target" count is superseded by this table.

## Priority Distribution

| Priority | Count | Items |
|----------|-------|-------|
| P0 (must-have / MVP) | 10 | CAP-001…CAP-010 |
| P1 (should-have) | 2 | CAP-011, CAP-012 |
| P2 (nice-to-have) | 2 | CAP-013, CAP-014 |
