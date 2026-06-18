---
document_type: architecture-index
level: L3
version: "1.0"
status: draft
producer: architect
timestamp: 2026-06-17T00:00:00
phase: 1b
inputs: [domain-spec/L2-INDEX.md, specs/prd.md, specs/prd-supplements/nfr-catalog.md, specs/prd-supplements/module-criticality.md]
traces_to: specs/prd.md
deployment_topology: single-service
---

# Architecture Index: zonewarden

> Lightweight index (~300 tokens). Load only the section(s) needed.
> Single-service: one Rust binary workspace; one deployment target.

## Document Map

| Section | File | Primary Consumer | Purpose |
|---------|------|-----------------|---------|
| SS-00 Overview & Principles | SS-00-overview.md | orchestrator, all agents | System overview; purity boundary mandate; offline/determinism constraints; Mermaid component diagram |
| SS-01 Core Pipeline & Components | SS-01-pipeline.md | story-writer, implementer | 13 modules wired as ST-1..8 pipeline; module responsibilities; pure vs effectful classification |
| SS-02 Data Model & Resolution Index | SS-02-data-model.md | implementer, formal-verifier | In-memory types; prefix-trie choice; canonical serialization for digest (DI-018) |
| SS-03 CLI & Adapter Boundary | SS-03-cli-adapter.md | implementer, test-writer | RealitySource trait seam; CLI (clap); YAML parser choice; output formatters; exit codes |
| ADR Directory | decisions/ | architect, all agents | Architecture Decision Records (ADR-001..ADR-007) |
| Verification Properties | ../verification-properties/ | formal-verifier | VP-001..VP-009 one-per-file; VP-INDEX.md |

## Cross-References

| Need | Read Together |
|------|--------------|
| Implement a module | SS-01-pipeline.md + SS-02-data-model.md |
| Verify a module | SS-01-pipeline.md (purity) + VP-INDEX.md + relevant VP file |
| Story decomposition | SS-01-pipeline.md + SS-03-cli-adapter.md |
| ADR rationale | decisions/ADR-00N-slug.md directly |

## Subsystem Registry

> **Source of truth** for subsystem names per Policy 6. BC frontmatter, BC-INDEX subsystem
> column, and story `subsystems:` MUST use the exact Name from this table.

| SS ID | Name | Architecture Doc | Implementing Modules | Phase Introduced |
|-------|------|-----------------|---------------------|-----------------|
| SS-01 | Policy | SS-01-pipeline.md | `policy`, `validator`, `portset` | Phase 1 |
| SS-02 | Flow Ingest | SS-01-pipeline.md | `zeek_adapter` | Phase 1 |
| SS-03 | Zone Resolution | SS-01-pipeline.md | `resolver`, `multicast` | Phase 1 |
| SS-04 | Classification | SS-01-pipeline.md | `classifier`, `idmz`, `severity` | Phase 1 |
| SS-05 | Aggregation | SS-01-pipeline.md | `aggregator`, `digest` | Phase 1 |
| SS-06 | Reporting | SS-01-pipeline.md | `reporter`, `cli` | Phase 1 |

## Architecture Decisions

| ID | Decision | Rationale |
|----|----------|-----------|
| ADR-001 | Sync batch (no async/tokio) | Offline file processing; no I/O concurrency benefit |
| ADR-002 | Cargo workspace: pure `core` lib + thin `cli` binary | Purity boundary as a crate boundary; Kani targets `core` lib |
| ADR-003 | Longest-prefix match via sorted prefix table | Feasible for Kani; simpler than trie; bounded search |
| ADR-004 | Canonical JSON + SHA-256 for policy digest | Implementation-deterministic; DI-018 compliance |
| ADR-005 | `serde_norway` (fork) for YAML with duplicate-key detection | `serde_yaml` deprecated and silently accepts duplicate keys |
| ADR-006 | `thiserror` typed errors mapped to exit codes | PRD error taxonomy; no stringly-typed panics |
| ADR-007 | Mermaid as string generation (no render deps) | Zero extra crate deps; MVP renders in GitHub/VSCode |

## Risk Mitigations (HIGH-Impact)

| Risk | Mitigation in Architecture |
|------|---------------------------|
| R-001 False confidence (service heuristic) | `service_source` field propagated through all pure data types; reporter always renders it |
| R-002 Silent allow / mis-resolve | Total resolution (DI-003) enforced by Kani VP-001; deny-by-default by VP-004; verdict totality by VP-005 |
