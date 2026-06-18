---
document_type: adversarial-review-index
level: ops
version: "1.0"
status: in-review
producer: adversary
timestamp: 2026-06-17T00:00:00
phase: "1d"
pass: 1
inputs: [product-brief.md, "domain-spec/"]
traces_to: domain-spec/L2-INDEX.md
total_findings: 14
severity_distribution: { CRIT: 1, HIGH: 4, MED: 5, LOW: 4 }
---

# Adversarial Review -- Pass 1 (Cycle: zonewarden-greenfield)

> This index covers Pass 1 of cycle `zonewarden-greenfield`. All 14 findings are new; no prior pass exists.

## Finding Catalog

| ID | Severity | Category | Title | Status | Depends On | Blocks |
|----|----------|----------|-------|--------|------------|--------|
| ADV-ZWGF-P01-CRIT-001 | CRITICAL | contradictions | SL-T data model contradiction across three documents | open | -- | -- |
| ADV-ZWGF-P01-HIGH-001 | HIGH | contradictions | `AmbiguousMembership` ViolationKind is unreachable and contradicts the policy-validity model | open | -- | -- |
| ADV-ZWGF-P01-HIGH-002 | HIGH | missing-edge-cases | EXTERNAL zone has no defined Purdue level; IDMZ no-bypass check requires every endpoint's level | open | -- | -- |
| ADV-ZWGF-P01-HIGH-003 | HIGH | ambiguous-language | IDMZ no-bypass (DI-006) cannot be evaluated from a single flow record | open | ADV-ZWGF-P01-HIGH-002 | -- |
| ADV-ZWGF-P01-HIGH-004 | HIGH | spec-gap | CAP-003's cited resilience invariant ("DI: resilience") does not exist | open | -- | ADV-ZWGF-P01-LOW-004 |
| ADV-ZWGF-P01-MED-001 | MEDIUM | missing-edge-cases | Conflicting-conduit-rules edge case entirely missing | open | -- | -- |
| ADV-ZWGF-P01-MED-002 | MEDIUM | missing-edge-cases | Asymmetric/reverse-flow handling under deny-by-default under-specified for response traffic | open | -- | -- |
| ADV-ZWGF-P01-MED-003 | MEDIUM | completeness | Service enum / known-service set and OT port->service table referenced but never enumerated | open | -- | -- |
| ADV-ZWGF-P01-MED-004 | MEDIUM | verification-gaps | Determinism invariant (DI-009) sort key omits proto and may not be a total order | open | -- | -- |
| ADV-ZWGF-P01-MED-005 | MEDIUM | missing-edge-cases | Empty-zone and zero-member-policy cases unspecified | open | -- | -- |
| ADV-ZWGF-P01-LOW-001 | LOW | completeness | VLAN-vs-subnet / NAT zone-identity limitation acknowledged nowhere | open | -- | -- |
| ADV-ZWGF-P01-LOW-002 | LOW | ambiguous-language | `external_endpoints` field type/meaning undefined; ConformanceResult tallies don't obviously sum | open | -- | -- |
| ADV-ZWGF-P01-LOW-003 | LOW | spec-fidelity | Differentiator "Open + declarative + git-versioned" backed partly by a P2 capability | open | -- | -- |
| ADV-ZWGF-P01-LOW-004 | LOW | completeness | ID Registry count: L2-INDEX has no DI-013 slot though CAP-003 needs one | open | ADV-ZWGF-P01-HIGH-004 | -- |

## Dependency Graph

```text
ADV-ZWGF-P01-HIGH-002 --related--> ADV-ZWGF-P01-HIGH-003
  (both touch DI-006; fix EXTERNAL purdue_level before redefining single-flow semantics)

ADV-ZWGF-P01-HIGH-004 --blocks--> ADV-ZWGF-P01-LOW-004
  (add DI-013 before updating L2-INDEX DI count)

All other findings are independent.
```

## Category Groups

| Category | Finding IDs | Can Triage in Parallel? |
|----------|-------------|------------------------|
| contradictions | ADV-ZWGF-P01-CRIT-001, ADV-ZWGF-P01-HIGH-001 | Yes — independent contradictions |
| missing-edge-cases | ADV-ZWGF-P01-HIGH-002, ADV-ZWGF-P01-MED-001, ADV-ZWGF-P01-MED-002, ADV-ZWGF-P01-MED-005 | Yes after HIGH-002 resolved (HIGH-003 depends on it); others independent |
| ambiguous-language | ADV-ZWGF-P01-HIGH-003, ADV-ZWGF-P01-LOW-002 | Yes after HIGH-002 resolved for HIGH-003; LOW-002 independent |
| spec-gap | ADV-ZWGF-P01-HIGH-004 | Yes |
| completeness | ADV-ZWGF-P01-MED-003, ADV-ZWGF-P01-LOW-001, ADV-ZWGF-P01-LOW-004 | Yes after HIGH-004 resolved for LOW-004; others independent |
| verification-gaps | ADV-ZWGF-P01-MED-004 | Yes |
| spec-fidelity | ADV-ZWGF-P01-LOW-003 | Yes |
