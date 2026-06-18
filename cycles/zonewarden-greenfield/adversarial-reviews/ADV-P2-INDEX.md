---
document_type: adversarial-review-index
level: ops
version: "1.0"
status: in-review
producer: adversary
timestamp: 2026-06-17T00:00:00
phase: "1d"
pass: 2
inputs: [product-brief.md, "domain-spec/"]
traces_to: domain-spec/L2-INDEX.md
total_findings: 16
severity_distribution: { CRIT: 1, HIGH: 3, MED: 6, LOW: 6 }
---

# Adversarial Review -- Pass 2 (Cycle: zonewarden-greenfield)

> Covers Pass 2 of cycle `zonewarden-greenfield`. Part A verified Pass-1 fixes (8 RESOLVED, 4 PARTIALLY_RESOLVED, 0 UNRESOLVED). All 16 catalogued below are NEW findings. Trajectory 14 -> 16 is a perimeter-expansion regression (new v1.1 accounting scope + un-propagated research decisions), not fix-induced; see pass-2.md Trajectory Note.

## Finding Catalog

| ID | Severity | Category | Title | Status | Depends On | Blocks |
|----|----------|----------|-------|--------|------------|--------|
| ADV-ZWGF-P02-CRIT-001 | CRITICAL | contradictions | DI-015 one-VerdictKind vs DI-006 IdmzBypass-in-addition; accounting identity unsound | open | -- | ADV-ZWGF-P02-HIGH-001 |
| ADV-ZWGF-P02-HIGH-001 | HIGH | verification-gaps | "distinct violating flows" not computable; Flow has no identity | open | ADV-ZWGF-P02-CRIT-001 | -- |
| ADV-ZWGF-P02-HIGH-002 | HIGH | ambiguous-language | Purdue <=L3/>=L4 vs IDMZ and L5/EXTERNAL collision under-specified | open | -- | -- |
| ADV-ZWGF-P02-HIGH-003 | HIGH | missing-edge-cases | Equal-length-prefix "tie" rule omits cross-family / disjoint / host-eq-network | open | -- | -- |
| ADV-ZWGF-P02-MED-001 | MEDIUM | missing-edge-cases | ICMP/portless protocols vs non-empty-ports conduit matching undefined | open | -- | -- |
| ADV-ZWGF-P02-MED-002 | MEDIUM | spec-fidelity | direction model token vs YAML token drift; parse contract undefined | open | -- | -- |
| ADV-ZWGF-P02-MED-003 | MEDIUM | completeness | Service table omits multi-transport OT services; S7comm "Unknown on doubt" untestable | open | -- | -- |
| ADV-ZWGF-P02-MED-004 | MEDIUM | missing-edge-cases | Multicast/broadcast destination resolves to EXTERNAL -> systematic false positives | open | -- | -- |
| ADV-ZWGF-P02-MED-005 | MEDIUM | verification-gaps | policy_digest computation/algorithm undefined vs DI-009 byte-stable claim | open | -- | -- |
| ADV-ZWGF-P02-MED-006 | MEDIUM | spec-fidelity | conn_state severity grading (research dec #5) silently dropped; blocked attempts = violations | open | -- | -- |
| ADV-ZWGF-P02-LOW-001 | LOW | spec-fidelity | service_source enum name drift (SensorDpi vs DpiConfirmed) | open | -- | -- |
| ADV-ZWGF-P02-LOW-002 | LOW | completeness | Exit code undefined for clean run with skipped flows | open | -- | -- |
| ADV-ZWGF-P02-LOW-003 | LOW | ambiguous-language | ImplicitExternal vs EXTERNAL longest-prefix model unreconciled | open | -- | -- |
| ADV-ZWGF-P02-LOW-004 | LOW | completeness | Flow.ts name drift + undefined precision/timezone for DI-009 key | open | -- | -- |
| ADV-ZWGF-P02-LOW-005 | LOW | purity-boundary-violations | ST-4-ST-7 purity claim vs FM-007 streaming I/O tension | open | -- | -- |
| ADV-ZWGF-P02-LOW-006 | LOW | completeness | DEC-018 warning has no defined channel/determinism/exit effect | open | -- | -- |

## Dependency Graph

```text
ADV-ZWGF-P02-CRIT-001 --blocks--> ADV-ZWGF-P02-HIGH-001
  (decouple IdmzBypass from VerdictKind before adding flow identity / fixing the accounting identity)

All other findings are independent.
```

## Category Groups

| Category | Finding IDs | Can Triage in Parallel? |
|----------|-------------|------------------------|
| contradictions | ADV-ZWGF-P02-CRIT-001 | Yes |
| verification-gaps | ADV-ZWGF-P02-HIGH-001, ADV-ZWGF-P02-MED-005 | HIGH-001 after CRIT-001; MED-005 independent |
| ambiguous-language | ADV-ZWGF-P02-HIGH-002, ADV-ZWGF-P02-LOW-003 | Yes |
| missing-edge-cases | ADV-ZWGF-P02-HIGH-003, ADV-ZWGF-P02-MED-001, ADV-ZWGF-P02-MED-004 | Yes |
| spec-fidelity | ADV-ZWGF-P02-MED-002, ADV-ZWGF-P02-MED-006, ADV-ZWGF-P02-LOW-001 | Yes |
| completeness | ADV-ZWGF-P02-MED-003, ADV-ZWGF-P02-LOW-002, ADV-ZWGF-P02-LOW-004, ADV-ZWGF-P02-LOW-006 | Yes |
| purity-boundary-violations | ADV-ZWGF-P02-LOW-005 | Yes |
