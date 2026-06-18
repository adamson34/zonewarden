---
document_type: epic-index
level: ops
version: "1.0"
status: draft
producer: story-writer
timestamp: 2026-06-17T00:00:00
phase: 2
traces_to: specs/prd.md
---

# Epics: zonewarden

Total epics: 6
Total stories: 17
Total BCs covered: 44 / 44

## E-1: Foundation — Policy Model, PortSet, and Workspace Scaffold

**Goal:** Bootstrap the Cargo workspace (`zonewarden-core` lib + `zonewarden` binary); implement
the `portset` and `policy` modules (YAML load + type definitions); implement `validator`
(semantic rules). This delivers the pure data model and the validated-policy gate that every
downstream module depends on.

| Field | Value |
|-------|-------|
| BCs covered | BC-1.01.001, BC-1.01.002, BC-1.01.003, BC-1.01.004, BC-1.01.005, BC-1.01.006, BC-1.01.007, BC-1.01.008, BC-1.01.009 |
| Subsystems touched | SS-01 Policy |
| Implementing modules | `portset`, `policy`, `validator` |
| Story count | 3 |
| Wave | 1 |
| Priority | P0 |

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-1.01 | Workspace scaffold + PortSet canonical form | 5 | none | draft |
| S-1.02 | Policy YAML load + type model | 5 | S-1.01 | draft |
| S-1.03 | Policy semantic validation | 8 | S-1.02 | draft |

---

## E-2: Flow Ingest and Normalization

**Goal:** Implement `zeek_adapter` — the first `RealitySource`. Parses Zeek `conn.log`
TSV into normalized `Flow` values, performs service inference, handles malformed/skipped
lines resiliently, canonicalizes IPv4-mapped addresses, and enforces the ingest cap.

| Field | Value |
|-------|-------|
| BCs covered | BC-1.02.001, BC-1.02.002, BC-1.02.003, BC-1.02.004, BC-1.02.005, BC-1.02.006 |
| Subsystems touched | SS-02 Flow Ingest |
| Implementing modules | `zeek_adapter` |
| Story count | 2 |
| Wave | 2 (depends on E-1 for types; but no validator dep needed) |
| Priority | P0 |

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-2.01 | Zeek conn.log parser + normalization | 8 | S-1.01 | draft |
| S-2.02 | Service inference + ingest cap | 5 | S-2.01 | draft |

---

## E-3: Zone Resolution

**Goal:** Implement `resolver` (sorted-prefix index, longest-prefix match, EXTERNAL fallback,
both-EXTERNAL short-circuit) and `multicast` (family-wide multicast/broadcast detection and
directed-broadcast override). Together these implement the ST-5 zone resolution step that
transforms raw IPs into `ResolvedEndpoint` values for the classifier.

| Field | Value |
|-------|-------|
| BCs covered | BC-1.03.001, BC-1.03.002, BC-1.03.003, BC-1.03.004, BC-1.03.005 |
| Subsystems touched | SS-03 Zone Resolution |
| Implementing modules | `resolver`, `multicast` |
| Story count | 2 |
| Wave | 2 (depends on E-1 for types only) |
| Priority | P0 |

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-3.01 | Zone resolver — sorted-prefix index + longest-prefix match | 8 | S-1.03 | draft |
| S-3.02 | Multicast + directed-broadcast detection | 5 | S-3.01 | draft |

---

## E-4: Classification and Verdict

**Goal:** Implement the `classifier` pipeline (IntraZone, any-match conduit union,
WrongDirection, NoMatchingConduit, MulticastExempt short-circuit, verdict totality),
`idmz` (DI-006 truth table, additive finding), and `severity` (conn_state bucket).
This is the security-critical core: every verdict decision is made here.

| Field | Value |
|-------|-------|
| BCs covered | BC-1.04.001, BC-1.04.002, BC-1.04.003, BC-1.04.004, BC-1.04.005, BC-1.04.006, BC-1.04.007, BC-1.04.008, BC-1.04.009, BC-1.04.010, BC-1.04.011 |
| Subsystems touched | SS-04 Classification |
| Implementing modules | `classifier`, `idmz`, `severity` |
| Story count | 4 |
| Wave | 3 (depends on E-3 resolver + multicast) |
| Priority | P0 |

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-4.01 | Severity grading (conn_state → bucket) | 3 | S-1.01 | draft |
| S-4.02 | IDMZ no-bypass truth table | 5 | S-3.01 | draft |
| S-4.03 | Classifier — IntraZone, conduit any-match, directionality, deny-by-default | 8 | S-3.02, S-4.02, S-4.01 | draft |
| S-4.04 | Classifier — MulticastExempt short-circuit + verdict totality | 5 | S-4.03 | draft |

---

## E-5: Aggregation and Determinism

**Goal:** Implement `aggregator` (ConformanceResult assembly, DI-015 accounting identity,
total-order violation sort, u64 checked arithmetic, empty-input case) and `digest`
(canonical JSON serialization + SHA-256 policy digest). Delivers the deterministic,
verifiable result that feeds the reporter.

| Field | Value |
|-------|-------|
| BCs covered | BC-1.05.001, BC-1.05.002, BC-1.05.003, BC-1.05.004, BC-1.05.005 |
| Subsystems touched | SS-05 Aggregation |
| Implementing modules | `aggregator`, `digest` |
| Story count | 3 |
| Wave | 4 (depends on E-4 classifier output) |
| Priority | P0 |

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-5.01 | Policy digest — canonical JSON + SHA-256 | 5 | S-1.03 | draft |
| S-5.02 | Aggregator — ConformanceResult, DI-015 identity, overflow guard | 8 | S-4.04, S-5.01 | draft |
| S-5.03 | Aggregator — deterministic sort + empty-input case | 5 | S-5.02 | draft |

---

## E-6: Reporting and CLI

**Goal:** Implement `reporter` (text/JSON/Mermaid formatters, atomic write, deterministic
warnings), and `cli` (clap argument parsing, exit code emission, flag validation,
--fail-on-skipped, offline invariant enforcement). This wires the pipeline end-to-end
and produces the user-facing artifacts.

| Field | Value |
|-------|-------|
| BCs covered | BC-1.06.001, BC-1.06.002, BC-1.06.003, BC-1.06.004, BC-1.06.005, BC-1.06.006, BC-1.06.007, BC-1.06.008 |
| Subsystems touched | SS-06 Reporting |
| Implementing modules | `reporter`, `cli` |
| Story count | 3 |
| Wave | 5 (depends on E-5 aggregation output) |
| Priority | P0 |

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-6.01 | Reporter — JSON + text + Mermaid formatters | 8 | S-5.03 | draft |
| S-6.02 | Reporter — atomic write + deterministic warnings | 3 | S-6.01 | draft |
| S-6.03 | CLI — argument parsing, exit codes, full pipeline integration | 8 | S-6.02, S-2.02 | draft |

---

## Epic Summary

| Epic | Goal Capsule | BCs | Stories | Wave(s) | Points |
|------|-------------|-----|---------|---------|--------|
| E-1 | Data model + policy load/validate | 9 | 3 | 1–2 | 18 |
| E-2 | Zeek flow ingest + normalization | 6 | 2 | 2 | 13 |
| E-3 | Zone resolution (resolver + multicast) | 5 | 2 | 2–3 | 13 |
| E-4 | Classification + verdict + IDMZ | 11 | 4 | 3–4 | 21 |
| E-5 | Aggregation + determinism + digest | 5 | 3 | 4–5 | 18 |
| E-6 | Reporting + CLI + integration | 8 | 3 | 5 | 19 |
| **Total** | | **44** | **17** | **1–5** | **102** |
