---
document_type: domain-spec-section
level: L2
section: capabilities
version: "1.8"
status: draft
producer: business-analyst
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[live-state]"
traces_to: L2-INDEX.md
---

# Domain Capabilities

> zonewarden is a **pipeline-oriented** tool. Capabilities are processing stages plus the
> cross-cutting outputs. P0 = MVP core loop; P1/P2 = roadmap (open-ended/learning-first).

| ID | Capability | Description | Business Rule | Priority |
|----|-----------|-------------|---------------|----------|
| CAP-001 | Load policy | Parse a declarative YAML segmentation policy into the internal Policy model | Malformed/invalid policy fails fast with a precise error; never partially loaded | P0 |
| CAP-002 | Validate policy | Statically validate the loaded policy (unique zone ids, conduit endpoints exist, valid Purdue levels, no equal-prefix membership ties) | Invalid policy → non-zero exit, no flow processing | P0 |
| CAP-003 | Ingest flows | Read observed flows from a RealitySource adapter; MVP adapter = Zeek `conn.log` (one connection record → exactly one `Flow`, keyed on the originator) | Unparseable flow lines are skipped + counted, never abort the run (DI-013) | P0 |
| CAP-004 | Normalize + infer service | Map adapter-specific records into the neutral `Flow`; infer app service from port when not provided, stamping `service_source` provenance | Port-based inference is heuristic and MUST be marked, never presented as authoritative (DI-008) | P0 |
| CAP-005 | Resolve endpoints to zones | Map each flow endpoint IP to exactly one zone via longest-prefix match; unmatched → implicit EXTERNAL zone | Total resolution: every endpoint resolves (DI-003, DI-005) | P0 |
| CAP-006 | Match conduits | Determine whether any allowed conduit permits a flow's (zone-pair, proto, port, direction) | Any-match union (DI-014); directional by initiator, bidirectional if declared (DI-007); PortSet canonical form (DI-020) | P0 |
| CAP-007 | Classify (deny-by-default) | Assign each flow exactly one VerdictKind (intra-zone / allowed / no-matching-conduit / wrong-direction / multicast-exempt) and grade violation severity from conn_state | Verdict totality, exactly one kind (DI-015); intra-zone (DI-002); deny-by-default (DI-001); multicast exemption (DI-016); severity grading (DI-017) | P0 |
| CAP-008 | IDMZ no-bypass check | Flag any single flow between two managed zones where one is ≤L3 and the other ≥L4 and neither endpoint is in the IDMZ (L3.5) | Single-flow check; raised even if a conduit nominally allows the flow (DI-006) | P0 |
| CAP-009 | Aggregate result | Assemble a ConformanceResult: counts, per-flow verdicts, violations, skipped/unresolved tallies, policy digest | Verdict accounting (DI-015); deterministic, stable ordering (DI-009); stable policy digest (DI-018) | P0 |
| CAP-010 | Render outputs | Emit a human-readable + JSON violations report, ordered warnings to stderr, and a **Mermaid** zone/conduit diagram with violations highlighted | Same inputs → byte-identical outputs (DI-009); deterministic warning model (DI-019) | P0 |
| CAP-011 | NetFlow/IPFIX adapter | Second RealitySource adapter for flow records lacking app-layer service | Proves the adapter seam; service_source=unknown/heuristic | P1 |
| CAP-012 | Native SVG rendering | Self-rendered SVG topology diagram (no external viewer) | Visual parity with Mermaid output | P1 |
| CAP-013 | Policy DSL front-end | Purpose-built policy language compiling to the same internal Policy model | Fuzzable + Kani-provable parser; semantics identical to YAML | P2 |
| CAP-014 | Firewall-config adapter | RealitySource that validates intent against firewall rules, not just flows | Detects over-permissive rules beyond allowed conduits | P2 |
