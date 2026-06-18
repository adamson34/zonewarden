---
document_type: domain-spec-section
level: L2
section: differentiators
version: "1.8"
status: draft
producer: business-analyst
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[live-state]"
traces_to: L2-INDEX.md
---

# Competitive Differentiator Traceability

> Each product differentiator must be backed by domain capabilities. Seeds PRD Section 6.

| Differentiator | Why It Matters | Supporting Capabilities |
|----------------|---------------|-------------------------|
| **Validates policy against *observed flows*** (not just configs) | The confirmed market whitespace — config tools (Batfish/Tufin) and monitoring platforms don't do declarative flow↔policy conformance | CAP-003, CAP-005, CAP-006, CAP-007 |
| **62443/Purdue-native** | Speaks the asset owner's language (zones, conduits, IDMZ, FR5) instead of generic ACLs | CAP-001, CAP-005, CAP-008 |
| **OT-protocol-aware (honestly heuristic)** | Recognizes Modbus/DNP3/etc. while being transparent about inference confidence — credibility with practitioners | CAP-004 |
| **Open + declarative + git-versioned** | Policy-as-code: reviewable, diffable, CI-runnable — none of the incumbents are open & declarative | CAP-001 (CAP-013 reinforces, roadmap) |
| **Deterministic & offline** | Repeatable evidence for assessments; safe (no live network), trivially demo-able | CAP-009, CAP-010, DI-009, DI-012 |
| **Extensible reality sources** | One engine, many inputs (flows → firewall configs) without re-architecting | CAP-003 (seam), CAP-011, CAP-014 |
| **Verification-grade correctness** | Kani-proven core + fuzzed parsers = trust the verdicts; the engineering portfolio signal | CAP-006, CAP-007, DI-003, DI-004, DI-009, DI-015, DI-016, DI-018, DI-020 (the ⊢ proof-target invariants) |
