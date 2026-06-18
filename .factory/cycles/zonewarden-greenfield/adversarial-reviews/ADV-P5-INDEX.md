---
document_type: adversarial-review-index
level: ops
version: "1.0"
status: in-review
producer: adversary
timestamp: 2026-06-17T13:30:00
phase: 1d
pass: 5
inputs: ["product-brief.md", "domain-spec/"]
traces_to: domain-spec/L2-INDEX.md
total_findings: 9
severity_distribution: { CRIT: 1, HIGH: 1, MED: 4, LOW: 3 }
---

# Adversarial Review -- Pass 5

## Finding Catalog

| ID | Severity | Category | Title | Status | Depends On | Blocks |
|----|----------|----------|-------|--------|-----------|--------|
| ADV-ZWGREEN-P05-CRIT-001 | CRITICAL | spec-fidelity | ConformanceResult cites FM-007 for overflow (should be FM-008) | open | -- | convergence |
| ADV-ZWGREEN-P05-HIGH-001 | HIGH | contradictions | FM-008 "checked/saturating" contradicts abort semantics | open | -- | convergence |
| ADV-ZWGREEN-P05-MED-001 | MED | missing-edge-cases | DI-016 step-2 directed-broadcast undefined for IPv6 | open | -- | -- |
| ADV-ZWGREEN-P05-MED-002 | MED | spec-fidelity | Verification differentiator cites incomplete provable-invariant set | open | -- | -- |
| ADV-ZWGREEN-P05-MED-003 | MED | ambiguous-language | Direction legal-token set split across documents | open | -- | -- |
| ADV-ZWGREEN-P05-MED-004 | MED | contradictions | DI-006 managed-L5 vs EXTERNAL-L5 not decidable from data model | open | -- | -- |
| ADV-ZWGREEN-P05-LOW-001 | LOW | spec-fidelity | Version frontmatter skew (1.0 vs 1.4) | open | -- | -- |
| ADV-ZWGREEN-P05-LOW-002 | LOW | ambiguous-language | FM-008 ingest cap relationship to u64 overflow unstated | open | -- | -- |
| ADV-ZWGREEN-P05-LOW-003 | LOW | coverage-gap | No DEC exercises DI-016 directed-broadcast override | open | -- | -- |

## Dependency Graph

```text
ADV-ZWGREEN-P05-CRIT-001 --blocks--> convergence
ADV-ZWGREEN-P05-HIGH-001 --blocks--> convergence
[All other findings are independent and can be triaged in parallel]
```

## Category Groups

| Category | Finding IDs | Can Triage in Parallel? |
|----------|------------|------------------------|
| spec-fidelity | CRIT-001, MED-002, LOW-001 | Yes |
| contradictions | HIGH-001, MED-004 | Yes |
| missing-edge-cases | MED-001 | Yes |
| ambiguous-language | MED-003, LOW-002 | Yes |
| coverage-gap | LOW-003 | Yes |
