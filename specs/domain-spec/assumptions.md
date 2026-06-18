---
document_type: domain-spec-section
level: L2
section: assumptions
version: "1.8"
status: draft
producer: business-analyst
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[live-state]"
traces_to: L2-INDEX.md
---

# Assumptions Requiring Validation

| ID | Assumption | Confidence | Validation Method | Impact if Wrong | Status | Traced To |
|----|------------|------------|-------------------|-----------------|--------|-----------|
| ASM-001 | A flow record's `src` reliably represents the connection **initiator** (so directionality is meaningful); one `conn.log` connection = one `Flow` keyed on the originator (DEC-016) | Medium | **P0 gating task**: validate against Zeek `conn.log` semantics (`conn_state`/history give originator) before directional verdicts are trusted; if unreliable, derive initiator from `conn_state` or downgrade to bidirectional with a warning | Directional conduit logic (DI-007) becomes unreliable; #1 false-positive source | unvalidated | DI-007, DEC-016, R-003 |
| ASM-002 | Zone membership by IP/CIDR is sufficient to model real OT plants for the MVP | High | Author a realistic synthetic reference topology; review with OT domain expert (builder) | Need richer matchers (VLAN, hostname) sooner | unvalidated | entities.md, DEC-001 |
| ASM-003 | Port→service inference covers enough OT protocols to be useful while clearly heuristic | High | Map the confirmed OT port table from research; mark all as heuristic | Misleading service labels; mitigated by `service_source` honesty (DI-008) | unvalidated | DI-008, DEC-007 |
| ASM-004 | IEC 62443 FR5 conformance maps cleanly to "every flow allowed by a conduit or intra-zone" | Medium | Cross-check against research report's FR5 framing | Scope creep into other FRs; mitigated by explicit out-of-scope | unvalidated | invariants.md |
| ASM-005 | Mermaid can express a Purdue-tiered zone/conduit diagram legibly at realistic plant size | Medium | Prototype a diagram from the reference topology early | Diagram unreadable at scale → prioritize SVG (CAP-012) | unvalidated | CAP-010 |
| ASM-006 | Representative, **non-confidential** sample flow data can be synthesized for tests/demo | High | Build a synthetic flow generator from the reference topology | No demo data; mitigated — generator is in scope | unvalidated | risks R-005 |
| ASM-007 | The IDMZ is identifiable purely by Purdue level (L3.5) in the policy | Medium | Confirm policy model lets a zone be declared `IDMZ`; review with builder | IDMZ check (DI-006) misfires if IDMZ isn't level-tagged | unvalidated | DI-006 |
| ASM-008 | IP/CIDR zone membership is NOT reliable under NAT or VLAN-only zoning; v1 explicitly does not model these | High | Document as a known limitation; out-of-scope for v1 | NAT'd flows resolve to the translation address (wrong zone); two VLAN zones sharing a subnet are indistinguishable — silent mis-resolution | unvalidated | R-002, ASM-002 |
| ASM-009 | Single-transport service inference is acceptable for MVP: each service has one canonical `(port, transport)`; multi-transport variants (DNP3/UDP, EtherNet-IP/UDP) and the S7comm/MMS port-102 overlap resolve to `Unknown`/heuristic rather than being mislabeled | High | Test the canonical table (entities.md) explicitly incl. the UDP-variant → `Unknown` cases; mark all heuristic via `service_source` (DI-008) | Some legitimate DNP3/ENIP-over-UDP shows as `Unknown` (reduced fidelity, not a false verdict) | unvalidated | DI-008, entities.md table, DEC-007/DEC-008 |
