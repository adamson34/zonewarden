---
document_type: adversarial-review-index
level: ops
version: "1.0"
status: in-review
producer: adversary
timestamp: 2026-06-17T00:00:00
phase: 1d
pass: 8
inputs: ["product-brief.md", "domain-spec/"]
traces_to: domain-spec/L2-INDEX.md
total_findings: 15
severity_distribution: { CRIT: 0, HIGH: 4, MED: 7, LOW: 4 }
---

# Adversarial Review -- Pass 8

> Scope: FULL (product-brief.md + domain-spec/ v1.7). D-008 convergence rule.
> Result: 0 CRIT, 4 HIGH — convergence clock does NOT start; clean-pass streak remains 0.
> TRAJECTORY REGRESSION: Pass 7 total was 9; Pass 8 total is 15 — finding count INCREASED (+6). This is the SECOND consecutive regression (5 -> 9 -> 15). Per protocol, root cause must be investigated before further convergence passes.

## Finding Catalog

| ID | Severity | Category | Title | Status | Depends On | Blocks |
|----|----------|----------|-------|--------|-----------|--------|
| ADV-P08-HIGH-001 | HIGH | contradictions / security-surface | both-EXTERNAL flow verdict bucket undefined (deny-by-default gap) | open | ADV-P08-HIGH-002 | DI-015 accounting, ConformanceResult external_endpoints |
| ADV-P08-HIGH-002 | HIGH | ambiguous-language / security-surface | "same zone" undefined for reserved EXTERNAL sentinel (ambiguous IntraZone gate) | open | -- | ADV-P08-HIGH-001, IDMZ exclusion logic |
| ADV-P08-HIGH-003 | HIGH | missing-edge-cases / security-surface | 0.0.0.0/:: as dst "likely EXTERNAL" is a hedge not a rule; reachable via legal /1-/8 | open | -- | DI-016, DEC-031 |
| ADV-P08-HIGH-004 | HIGH | missing-edge-cases / security-surface | 0.0.0.0/:: as src/initiator entirely unspecified | open | ADV-P08-HIGH-003 | DI-007 directional verdict |
| ADV-P08-MED-001 | MED | ambiguous-language / verification-gaps | idmz_bypass verdict-independence vs forced-false-on-multicast-dst exception under-emphasized | open | -- | -- |
| ADV-P08-MED-002 | MED | ambiguous-language / spec-fidelity | severity scoped to flows vs violations inconsistency | open | -- | -- |
| ADV-P08-MED-003 | MED | spec-fidelity / coverage-gap | DpiConfirmed dangling enum variant; no MVP producer, precedence open | open | -- | -- |
| ADV-P08-MED-004 | MED | verification-gaps / performance | FM-007 bounded-memory vs DI-009 full-set sort at max_flows=1e9 unreconciled | open | -- | -- |
| ADV-P08-MED-005 | MED | missing-edge-cases / security-surface | DI-010 rejects /0 only; /1-/7 broad prefixes shadow EXTERNAL unwarned | open | -- | -- |
| ADV-P08-MED-006 | MED | missing-edge-cases | all-zeros network-address dst not exempted, asymmetric with directed-broadcast | open | -- | -- |
| ADV-P08-MED-007 | MED | interface-gaps / verification-gaps | max_flows cap partial-output policy unspecified | open | -- | -- |
| ADV-P08-LOW-001 | LOW | process-gap | L2-INDEX hardcoded ID counts have no automated drift check | open | -- | -- |
| ADV-P08-LOW-002 | LOW | ambiguous-language | direction token grammar asserted in 3 docs + deferred to PRD (SSOT smell) | open | -- | -- |
| ADV-P08-LOW-003 | LOW | spec-fidelity | entities.md "previously-wrong RSTO" dangling provenance comment | open | -- | -- |
| ADV-P08-LOW-004 | LOW | spec-fidelity / traceability | brief cites FM-008 for u64 rationale; overflow story is FM-009 | open | -- | -- |

## Dependency Graph

```text
ADV-P08-HIGH-002 --blocks--> ADV-P08-HIGH-001 (IntraZone predicate ambiguity is root cause of both-EXTERNAL verdict gap)
ADV-P08-HIGH-003 --blocks--> ADV-P08-HIGH-004 (dst rule must be established before src rule can be specified consistently)
[All MED and LOW findings are independent of each other and of the HIGHs]
```

## Category Groups

| Category | Finding IDs | Can Triage in Parallel? |
|----------|------------|------------------------|
| contradictions / security-surface | ADV-P08-HIGH-001 | After HIGH-002 resolved |
| ambiguous-language / security-surface | ADV-P08-HIGH-002 | Yes (root cause; resolve first) |
| missing-edge-cases / security-surface | ADV-P08-HIGH-003, ADV-P08-HIGH-004, ADV-P08-MED-005, ADV-P08-MED-006 | HIGH-003 before HIGH-004; MED-005/006 independent |
| ambiguous-language / verification-gaps | ADV-P08-MED-001, ADV-P08-MED-002, ADV-P08-LOW-002 | Yes |
| spec-fidelity / coverage-gap | ADV-P08-MED-003 | Yes |
| verification-gaps / performance | ADV-P08-MED-004 | Yes |
| interface-gaps / verification-gaps | ADV-P08-MED-007 | Yes |
| process-gap | ADV-P08-LOW-001 | Yes |
| spec-fidelity | ADV-P08-LOW-003, ADV-P08-LOW-004 | Yes |

## Part A Fix-Verification Result

9 Pass-7 fixes verified. One partial.

| Fix | Description | Result |
|-----|-------------|--------|
| P07-HIGH-001 | Brief now u64 not usize | VERIFIED (Part A #1) |
| P07-HIGH-002 | DI-017 defers to entities ConnState bucket as single source | VERIFIED (Part A #2) |
| FM-008 split: max_flows ingest cap @ST-3 | FM-008 cap | VERIFIED (Part A #3) |
| FM-009 overflow @ST-7 | FM-009 overflow | VERIFIED (Part A #3) |
| DI-015 count invariants | distinct_violating_flows<=total_flows and idmz_bypasses<=total_flows | VERIFIED (Part A #4) |
| DEC-031 multicast-in-declared-zone + 0.0.0.0/:: | multicast/unspecified handling | PARTIALLY VERIFIED — multicast/broadcast resolved; 0.0.0.0/:: dst hedged + src missing (see ADV-P08-HIGH-003/004) |
| DEC-030 re-anchored to CAP-005 | broadcast anchor | VERIFIED |
| DEC-031 differentiator anchored to CAP-005 | CAP anchor | VERIFIED (Part A #6) |
| Glossary Violation note | Violation coherence | VERIFIED (Part A #7) |

## Convergence Status

| Metric | Value |
|--------|-------|
| Convergence rule | D-008: 2 consecutive passes with 0 CRIT and 0 HIGH |
| This pass | CRIT: 0, HIGH: 4 — NOT CLEAN |
| Clean-pass streak | 0 (unchanged) |
| 2-consecutive-clean requirement | NOT MET |
| Trajectory (passes 1-8) | 14 -> 16 -> 11 -> 15 -> 9 -> 5 -> 9 -> 15 |
| Trajectory note | REGRESSION: SECOND consecutive increase (5->9->15); investigate root cause |
