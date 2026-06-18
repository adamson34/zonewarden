---
document_type: domain-spec-section
level: L2
section: risks
version: "1.8"
status: draft
producer: business-analyst
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[live-state]"
traces_to: L2-INDEX.md
---

# Risk Register

| ID | Risk | Likelihood | Impact | Mitigation | Status | Category | Traced To |
|----|------|-----------|--------|-----------|--------|----------|-----------|
| R-001 | **False confidence**: port-based service inference mislabels traffic, and a user trusts it as ground truth | Medium | High | `service_source` provenance on every flow (DI-008); reports visibly flag heuristics; never assert protocol from port alone | open | reliability | DI-008, DEC-007 |
| R-002 | **Silent allow / mis-resolve**: a flow slips through unclassified, or resolves to the *wrong* zone (NAT translation address; VLAN-only zoning sharing a subnet) and is treated as conformant | Low | High | Total resolution (DI-003) + deny-by-default (DI-001) + verdict totality (DI-015); EXTERNAL catches unmatched IPs. NAT/VLAN mis-resolution is a documented v1 limitation (ASM-008), not silently mitigated | open | security | DI-001, DI-003, DI-015, ASM-008 |
| R-003 | **Direction misattribution**: initiator can't be derived from the flow source, inverting directional verdicts | Medium | Medium | Validate ASM-001; if unreliable, derive initiator from `conn_state`/history or treat as bidirectional with a warning | open | reliability | DI-007, ASM-001 |
| R-004 | **Scope creep** into full 62443 risk assessment or live capture/enforcement | Medium | Medium | Hard out-of-scope in brief & invariants (DI-012); FR5-only framing | open | business | DI-012 |
| R-005 | **No realistic test data** (real OT flows are confidential) | Medium | Medium | Synthetic reference topology + flow generator (ASM-006); strictly non-confidential | open | business | ASM-006 |
| R-006 | **Diagram unreadable** at realistic plant scale (Mermaid) | Medium | Low | Prototype early (ASM-005); SVG fallback (CAP-012); allow filtering to violations-only view | open | reliability | CAP-010, ASM-005 |
