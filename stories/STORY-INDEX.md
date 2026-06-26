---
document_type: story-index
level: ops
version: "1.1"
status: in-progress
producer: story-writer
timestamp: 2026-06-17T00:00:00
phase: 2
traces_to: specs/prd.md
---

# Story Index: zonewarden

**Total stories:** 17
**Total points:** 102
**Total BCs covered:** 44/44

## Story Table

| ID | Title | Epic | Wave | Status | Points | Dependencies | BCs |
|----|-------|------|------|--------|--------|-------------|-----|
| S-1.01 | Workspace scaffold + PortSet canonical form | E-1 | 1 | done | 5 | none | BC-1.01.009 |
| S-1.02 | Policy YAML load + type model | E-1 | 2 | done | 5 | S-1.01 | BC-1.01.001, BC-1.01.002, BC-1.01.003 |
| S-1.03 | Policy semantic validation | E-1 | 3 | done | 8 | S-1.02 | BC-1.01.004, BC-1.01.005, BC-1.01.006, BC-1.01.007, BC-1.01.008 |
| S-2.01 | Zeek conn.log parser + normalization | E-2 | 2 | done | 8 | S-1.01 | BC-1.02.001, BC-1.02.002, BC-1.02.003, BC-1.02.005 |
| S-2.02 | Service inference + ingest cap | E-2 | 3 | done | 5 | S-2.01 | BC-1.02.004, BC-1.02.006 |
| S-3.01 | Zone resolver + longest-prefix match | E-3 | 4 | done | 8 | S-1.03 | BC-1.03.001, BC-1.03.002, BC-1.03.005 |
| S-3.02 | Multicast + directed-broadcast detection | E-3 | 4 | done | 5 | S-3.01 | BC-1.03.003, BC-1.03.004 |
| S-4.01 | Severity grading (conn_state → bucket) | E-4 | 2 | done | 3 | S-1.01 | BC-1.04.009 |
| S-4.02 | IDMZ no-bypass truth table | E-4 | 4 | done | 5 | S-3.01 | BC-1.04.007, BC-1.04.008 |
| S-4.03 | Classifier — IntraZone, any-match, directionality | E-4 | 4 | done | 8 | S-3.02, S-4.02, S-4.01 | BC-1.04.001–006, BC-1.04.009 |
| S-4.04 | Classifier — MulticastExempt + verdict totality | E-4 | 4 | done | 5 | S-4.03 | BC-1.04.010, BC-1.04.011 |
| S-5.01 | Policy digest — canonical JSON + SHA-256 | E-5 | 3 | done | 5 | S-1.03 | BC-1.05.003 |
| S-5.02 | Aggregator — ConformanceResult + DI-015 + overflow | E-5 | 5 | done | 8 | S-4.04, S-5.01 | BC-1.05.001, BC-1.05.004, BC-1.05.005 |
| S-5.03 | Aggregator — deterministic sort + empty-input | E-5 | 5 | ready | 5 | S-5.02 | BC-1.05.002 |
| S-6.01 | Reporter — JSON + text + Mermaid formatters | E-6 | 5 | blocked | 8 | S-5.03 | BC-1.06.002, BC-1.06.003, BC-1.06.004 |
| S-6.02 | Reporter — atomic write + deterministic warnings | E-6 | 5 | blocked | 3 | S-6.01 | BC-1.06.005, BC-1.06.007, BC-1.06.008 |
| S-6.03 | CLI — argument parsing, exit codes, integration | E-6 | 5 | blocked | 8 | S-6.02, S-2.02 | BC-1.06.001, BC-1.06.006, BC-1.06.007 |

## By Wave

| Wave | Stories | Points |
|------|---------|--------|
| 1 | S-1.01 | 5 |
| 2 | S-1.02, S-2.01, S-4.01 | 16 |
| 3 | S-1.03, S-2.02, S-5.01 | 18 |
| 4 | S-3.01, S-3.02, S-4.02, S-4.03, S-4.04 | 31 |
| 5 | S-5.02, S-5.03, S-6.01, S-6.02, S-6.03 | 32 |

## By Priority

All stories: **P0** (all BCs are P0 — no P1/P2 MVP scope).

## Sizing Verification

No story exceeds 13 points. Maximum is 8 points (stories: S-2.01, S-3.01, S-4.03, S-5.02, S-6.01, S-6.03). All within the 13-point cap.
