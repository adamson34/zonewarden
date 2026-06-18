---
document_type: wave-schedule
level: ops
version: "1.0"
status: draft
producer: story-writer
timestamp: 2026-06-17T00:00:00
phase: 2
inputs: [stories/STORY-INDEX.md, stories/dependency-graph.md]
traces_to: stories/STORY-INDEX.md
---

# Wave Schedule: zonewarden

## Summary

| Metric | Value |
|--------|-------|
| Total stories | 17 |
| Total consolidated waves | 5 |
| Max parallelism (stories per wave) | 3 |
| Estimated agent spawns | 17 |

> **Note on consolidation:** The acyclic dependency graph resolves to 11 sequential layers.
> Adjacent single-story layers are consolidated into 5 delivery waves for scheduling efficiency.
> Within each wave, listed stories can proceed in parallel once prior wave is complete.

## Wave Plan

### Wave 1 — Workspace Scaffold + Foundation Types

> **Unlock condition:** No dependencies.

| Group | Stories | Points | Complexity | Agent Scope |
|-------|---------|--------|-----------|-------------|
| A | S-1.01 | 5 | M | 1 story/agent |

**Deliverable:** Compilable `zonewarden-core` + `zonewarden` workspace; `portset` module with canonical form; all shared data types defined. This is the foundation for ALL subsequent waves.

**Wave gate:** `cargo check --workspace` passes; `cargo test -p zonewarden-core` passes all portset tests.

---

### Wave 2 — Policy Model + Flow Parser + Severity

> **Unlock condition:** Wave 1 complete (S-1.01 done).

| Group | Stories | Points | Complexity | Agent Scope |
|-------|---------|--------|-----------|-------------|
| A | S-1.02 | 5 | M | 1 story/agent |
| B | S-2.01 | 8 | L | 1 story/agent |
| C | S-4.01 | 3 | S | 1 story/agent |

**Parallelism:** S-1.02, S-2.01, and S-4.01 can all proceed in parallel — each depends only on S-1.01 (types defined).

**Deliverable:** Policy YAML load; Zeek conn.log streaming parser; severity::grade with full OQ-001 13-state table.

**Wave gate:** `cargo test -p zonewarden` policy_load tests pass; `cargo test -p zonewarden` zeek_adapter tests pass; severity tests pass.

---

### Wave 3 — Validation + Service Inference + Resolver + Digest

> **Unlock condition:** Wave 2 complete (S-1.02, S-2.01, S-4.01 done).

| Group | Stories | Points | Complexity | Agent Scope |
|-------|---------|--------|-----------|-------------|
| A | S-1.03 | 8 | L | 1 story/agent |
| B | S-2.02 | 5 | M | 1 story/agent |
| C | S-5.01 | 5 | M | 1 story/agent |

> Note: S-1.03 must complete before Wave 4's S-3.01 can start. S-2.02 and S-5.01 have no Wave 4 blockers and can proceed in parallel with S-1.03.

**Parallelism:** S-1.03, S-2.02, and S-5.01 can proceed in parallel (S-2.02 depends on S-2.01 ✓; S-5.01 depends on S-1.03 ✓ — wait: S-5.01 → S-1.03. S-5.01 must wait for S-1.03. Correction: S-2.02 can run in parallel with S-1.03; S-5.01 must wait for S-1.03.)

**Revised Wave 3 parallelism:**
- Group A: S-1.03 (must complete first; others unblock after)
- Group B: S-2.02 (parallel with S-1.03; only needs S-2.01 ✓)

**After S-1.03 completes (within Wave 3):** S-5.01 can start (also Wave 3 or promoted to Wave 4).

**Deliverable:** Policy semantic validation; service table + ingest cap; policy SHA-256 digest.

**Wave gate:** Validator tests pass; service inference tests pass; digest tests pass.

---

### Wave 4 — Zone Resolution + IDMZ + Multicast + Classifier

> **Unlock condition:** Wave 3 complete (S-1.03, S-2.02, S-5.01 done).

| Group | Stories | Points | Complexity | Agent Scope |
|-------|---------|--------|-----------|-------------|
| A | S-3.01 | 8 | L | 1 story/agent |
| A | S-3.02 | 5 | M | 1 story/agent (after S-3.01) |
| B | S-4.02 | 5 | M | 1 story/agent (parallel with S-3.01) |
| C | S-4.03 | 8 | L | 1 story/agent (after S-3.02 + S-4.02 + S-4.01) |
| C | S-4.04 | 5 | M | 1 story/agent (after S-4.03) |

**Sequencing within wave:**
1. Start: S-3.01 (resolver) + S-4.02 (IDMZ) in parallel
2. After S-3.01: start S-3.02 (multicast)
3. After S-3.02 + S-4.02 + S-4.01 (done in Wave 2): start S-4.03 (classifier)
4. After S-4.03: start S-4.04 (MulticastExempt + totality)

**Deliverable:** Full zone resolution pipeline; IDMZ truth table; multicast detection; classifier with all verdict kinds.

**Wave gate:** All resolver, multicast, IDMZ, and classifier tests pass including Kani harness stubs compile.

---

### Wave 5 — Aggregation + Reporting + CLI Integration

> **Unlock condition:** Wave 4 complete (S-4.04, S-5.01 done).

| Group | Stories | Points | Complexity | Agent Scope |
|-------|---------|--------|-----------|-------------|
| A | S-5.02 | 8 | L | 1 story/agent |
| A | S-5.03 | 5 | M | 1 story/agent (after S-5.02) |
| B | S-6.01 | 8 | L | 1 story/agent (after S-5.03) |
| B | S-6.02 | 3 | S | 1 story/agent (after S-6.01) |
| B | S-6.03 | 8 | L | 1 story/agent (after S-6.02 + S-2.02 ✓) |

**Sequencing within wave:** S-5.02 → S-5.03 → S-6.01 → S-6.02 → S-6.03 (sequential within the aggregation+reporting path).

**Deliverable:** Full aggregation with DI-015 identity + overflow guard + deterministic sort; all three output formatters; atomic write; full CLI end-to-end integration.

**Wave gate:** End-to-end integration test passes; `cargo build --release` succeeds; manual smoke test with fixture data.

---

## Pipeline Overlap Plan

| Parallel Activity | When |
|------------------|------|
| S-2.01 (Zeek parser) | Start alongside S-1.02 (Wave 2) |
| S-4.01 (severity) | Start alongside S-1.02 (Wave 2) |
| S-2.02 (service/cap) | Start alongside S-1.03 (Wave 3) |
| S-4.02 (IDMZ) | Start alongside S-3.01 (Wave 4) |
| Fuzz target stubs | Create in Wave 2-3 alongside main parser work |
| Kani harness stubs | Create in Wave 4 alongside classifier work |

## Critical Path

**Longest dependency chain:**
`S-1.01 → S-1.02 → S-1.03 → S-3.01 → S-3.02 → S-4.03 → S-4.04 → S-5.02 → S-5.03 → S-6.01 → S-6.02 → S-6.03`

**Total critical path points:** 5+5+8+8+5+8+5+8+5+8+3+8 = **76 points** (11 stories)

**Non-critical stories:** S-2.01 (8), S-2.02 (5), S-4.01 (3), S-4.02 (5), S-5.01 (5) = 26 points in parallel tracks

## Point Distribution

| Wave | Stories | Points | Running Total |
|------|---------|--------|--------------|
| Wave 1 | 1 | 5 | 5 |
| Wave 2 | 3 | 16 | 21 |
| Wave 3 | 3 | 18 | 39 |
| Wave 4 | 5 | 31 | 70 |
| Wave 5 | 5 | 32 | 102 |
| **Total** | **17** | **102** | |
