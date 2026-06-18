---
document_type: adversarial-review-index
level: ops
version: "1.0"
status: in-review
producer: adversary
timestamp: 2026-06-17T00:00:00
phase: "1d"
pass: 3
inputs: [product-brief.md, "domain-spec/"]
traces_to: domain-spec/L2-INDEX.md
total_findings: 11
severity_distribution: { CRIT: 0, HIGH: 2, MED: 5, LOW: 4 }
---

# Adversarial Review -- Pass 3 (Cycle: zonewarden-greenfield)

> Covers Pass 3 of cycle `zonewarden-greenfield`. Part A verified the v1.2 invariants (8/10 VERIFIED, 2 PARTIAL); the Pass-2 CRIT-001 decoupling and all four Pass-1 partials' subject areas are confirmed resolved. All 11 catalogued below are NEW findings. Trajectory 14 -> 16 -> 11 (Pass-2 rise was a documented perimeter expansion; Pass-3 resumes monotonic decrease). NOT a clean pass — convergence counter at 0/3.

## Finding Catalog

| ID | Severity | Category | Title | Status | Depends On | Blocks |
|----|----------|----------|-------|--------|------------|--------|
| ADV-ZWGF-P03-HIGH-001 | HIGH | verification-gaps | policy_digest canonicalization undefined; DI-018 untestable | open | -- | -- |
| ADV-ZWGF-P03-HIGH-002 | HIGH | ambiguous-language | external_endpoints tally undefined / orphaned from accounting | open | -- | -- |
| ADV-ZWGF-P03-MED-001 | MEDIUM | spec-fidelity | DEC-007 vs entities.md:47 transport/non-default-port inference contradiction | open | -- | -- |
| ADV-ZWGF-P03-MED-002 | MEDIUM | missing-edge-cases | Verdict precedence MulticastExempt vs IntraZone vs idmz_bypass ambiguous | open | -- | -- |
| ADV-ZWGF-P03-MED-003 | MEDIUM | missing-edge-cases | Broadcast detection undefined (directed broadcast, IPv6 asymmetry) | open | -- | -- |
| ADV-ZWGF-P03-MED-004 | MEDIUM | verification-gaps | Other(String)/Other(u8) determinism & normalization undefined | open | -- | -- |
| ADV-ZWGF-P03-MED-005 | MEDIUM | verification-gaps | usize tally overflow / scale bounds unspecified vs streaming design | open | -- | -- |
| ADV-ZWGF-P03-LOW-001 | LOW | completeness | DEC numbering non-contiguous (013b, max 024) vs count 25 [process-gap] | open | -- | -- |
| ADV-ZWGF-P03-LOW-002 | LOW | completeness | Unknown direction/proto/PortSet token handling undefined | open | -- | -- |
| ADV-ZWGF-P03-LOW-003 | LOW | ambiguous-language | flow_index ingest-position semantics (dense vs sparse) ambiguous | open | -- | -- |
| ADV-ZWGF-P03-LOW-004 | LOW | completeness | Stale glossary Flow definition omits flow_index/conn_state | open | -- | -- |

## Dependency Graph

```text
All findings are independent (no blocking edges this pass).
Affinity note: HIGH-001 (digest canonicalization) and MED-004 (Other(String) normalization) both touch serialization determinism and should be fixed together for a coherent canonical-encoding rule.
```

## Category Groups

| Category | Finding IDs | Can Triage in Parallel? |
|----------|-------------|------------------------|
| verification-gaps | ADV-ZWGF-P03-HIGH-001, ADV-ZWGF-P03-MED-004, ADV-ZWGF-P03-MED-005 | Yes (HIGH-001 + MED-004 share canonical-encoding affinity) |
| ambiguous-language | ADV-ZWGF-P03-HIGH-002, ADV-ZWGF-P03-LOW-003 | Yes |
| spec-fidelity | ADV-ZWGF-P03-MED-001 | Yes |
| missing-edge-cases | ADV-ZWGF-P03-MED-002, ADV-ZWGF-P03-MED-003 | Yes |
| completeness | ADV-ZWGF-P03-LOW-001, ADV-ZWGF-P03-LOW-002, ADV-ZWGF-P03-LOW-004 | Yes |
