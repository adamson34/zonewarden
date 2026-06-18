---
document_type: adversarial-review-index
level: ops
version: "1.0"
status: in-review
producer: adversary
timestamp: 2026-06-17T00:00:00
phase: 1d
pass: 7
inputs: ["product-brief.md", "domain-spec/"]
traces_to: domain-spec/L2-INDEX.md
total_findings: 9
severity_distribution: { CRIT: 0, HIGH: 2, MED: 4, LOW: 3 }
---

# Adversarial Review -- Pass 7

> Scope: FULL (product-brief.md + domain-spec/ v1.6). D-008 convergence rule.
> Result: 0 CRIT, 2 HIGH — convergence clock does NOT start.
> TRAJECTORY REGRESSION: Pass 6 total was 5; Pass 7 total is 9 — finding count INCREASED (+4). Clean-pass streak RESET to 0.

## Finding Catalog

| ID | Severity | Category | Title | Status | Depends On | Blocks |
|----|----------|----------|-------|--------|-----------|--------|
| ADV-P07-HIGH-001 | HIGH | contradictions / core-types | Product brief contradicts entities.md on canonical counter type (`usize` vs `u64`) | open | -- | implementation of FM-008 / DI-009 / ConformanceResult tallies |
| ADV-P07-HIGH-002 | HIGH | contradictions / security-surface | DI-017 and entities.md disagree on `RSTO`/`RSTR`/`SH` severity bucket (reset-after-established) | open | -- | implementation of DI-017 / Severity grading |
| ADV-P07-MED-001 | MED | interface-gaps / verification-gaps | `max_flows` cap boundary (at-exactly vs over) and cap-reached response undefined; exit code overloaded | open | -- | -- |
| ADV-P07-MED-002 | MED | spec-fidelity / traceability | FM-008 Subsystem column cites ST-7 but mandated enforcement is ST-3; cap's home stage ambiguous | open | -- | -- |
| ADV-P07-MED-003 | MED | missing-edge-cases / security-surface | DI-016 step-1 silent-allow for multicast-inside-a-declared-zone; `0.0.0.0` dst unspecified | open | -- | -- |
| ADV-P07-MED-004 | MED | verification-gaps / accounting | No invariant bounding `distinct_violating_flows ≤ total_flows` or relating it to component tallies | open | -- | -- |
| ADV-P07-LOW-001 | LOW | spec-fidelity / traceability | DEC-030 anchored to CAP-007 (Classify) but directed-broadcast detection executes at CAP-005 (Resolve) | open | -- | -- |
| ADV-P07-LOW-002 | LOW | ambiguous-language / coverage-gap | Glossary "Violation" does not note that one flow can produce up to two Violation entries | open | -- | -- |
| ADV-P07-LOW-003 | LOW | ambiguous-language / framing-drift | DI-010 lists `unidirectional` as equal-footing legal token; entities.md/glossary/DI-018 call it an alias only | open | -- | -- |

## Dependency Graph

```text
ADV-P07-HIGH-001 --blocks--> implementation of ConformanceResult tallies, FM-008 overflow arithmetic, DI-009 byte-stable output
ADV-P07-HIGH-002 --blocks--> implementation of Severity grading (ST-6), property tests for DI-017
[All MED and LOW findings are independent of each other and of the HIGHs]
```

## Category Groups

| Category | Finding IDs | Can Triage in Parallel? |
|----------|------------|------------------------|
| contradictions / core-types | ADV-P07-HIGH-001 | Yes |
| contradictions / security-surface | ADV-P07-HIGH-002 | Yes |
| interface-gaps / verification | ADV-P07-MED-001 | Yes |
| spec-fidelity / traceability | ADV-P07-MED-002, ADV-P07-LOW-001 | Yes |
| missing-edge-cases / security | ADV-P07-MED-003 | Yes |
| verification-gaps / accounting | ADV-P07-MED-004 | Yes |
| ambiguous-language | ADV-P07-LOW-002, ADV-P07-LOW-003 | Yes |

## Part A Fix-Verification Result

All 5 Pass-6 fixes VERIFIED.

| Fix | Description | Result |
|-----|-------------|--------|
| Fix 1 | Directed-broadcast override restricted to prefix ≤ /30; /31 and /32 excluded | VERIFIED |
| Fix 2 | Concrete `max_flows` ingest cap enforced at ST-3 | VERIFIED (MED caveat → ADV-P07-MED-001) |
| Fix 3 | Overflow aborts with exit code 2 | VERIFIED |
| Fix 4 | CAP-006/009/010 cite DI-020, DI-015, DI-018, DI-019; all cited DIs exist and are contiguous | VERIFIED |
| Fix 5 | ConnState::Other defaults to Established | VERIFIED |

## Convergence Status

| Metric | Value |
|--------|-------|
| Convergence rule | D-008: 2 consecutive passes with 0 CRIT and 0 HIGH |
| This pass | CRIT: 0, HIGH: 2 — NOT CLEAN |
| Clean-pass streak | RESET to 0 |
| 2-consecutive-clean requirement | NOT MET |
| Trajectory (passes 1–7) | 14 → 16 → 11 → 15 → 9 → 5 → **9** |
| Trajectory note | REGRESSION: 5 → 9 is an INCREASE of +4 findings vs Pass 6 |
