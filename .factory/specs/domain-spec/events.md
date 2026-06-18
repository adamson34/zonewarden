---
document_type: domain-spec-section
level: L2
section: events
version: "1.8"
status: draft
producer: business-analyst
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[live-state]"
traces_to: L2-INDEX.md
---

# Domain Processing Stages

> zonewarden is a batch pipeline (no live events). Stages run in order; the boundary between
> ST-3/ST-4 is the `RealitySource` seam. Each stage is a pure transform of its inputs.

| Stage | Trigger | Preconditions | Outcomes |
|-------|---------|---------------|----------|
| **ST-1 Load policy** | CLI invocation with `--policy` | Policy file readable | Parsed Policy (pre-validation) or load error |
| **ST-2 Validate policy** | ST-1 success | Policy parsed | Valid Policy (DI-010/011) or fail-fast diagnostic; builds the longest-prefix resolution index |
| **ST-3 Ingest flows** | ST-2 success | `--flows` source available, adapter selected | Stream of raw adapter records; unparseable lines skipped + counted |
| **ST-4 Normalize + infer service** | per record from ST-3 | raw record | `Flow` with `proto`, optional `service`, and `service_source` set (DI-008) |
| **ST-5 Resolve endpoints** | per `Flow` | valid Policy index | `ResolvedEndpoint` for src and dst — exactly one zone each, longest-prefix or `ImplicitExternal` (DI-003/004/005) |
| **ST-6 Classify flow** | per resolved `Flow` | both endpoints resolved | Exactly one `VerdictKind`: MulticastExempt (dst multicast/broadcast, DI-016) \| IntraZone (DI-002) \| Allowed if ≥1 conduit permits, any-match union (DI-007/DI-014) \| WrongDirection \| NoMatchingConduit (DI-001). **Independently**, set `idmz_bypass` for managed ≤L3↔≥L4 single flows (DI-006), regardless of the verdict. Grade violation `severity` from `conn_state` (DI-017) |
| **ST-7 Aggregate** | all flows classified | verdict stream complete | `ConformanceResult` with canonical ordering (DI-009), tallies, policy digest |
| **ST-8 Render** | ST-7 complete | ConformanceResult | Violations report (text + JSON) + Mermaid diagram + ordered warnings to stderr (DI-019); exit code reflects conformance (see Exit semantics) |

## Stage Notes

- **Seam (ST-3/4):** the engine downstream of ST-4 knows only `Flow`. Adding NetFlow (CAP-011) or
  firewall-config (CAP-014) adapters touches only ST-3/ST-4.
- **Purity boundary (testing contract):** I/O lives in the ST-3 iterator (streaming, FM-007) and
  in ST-1 (read policy) and ST-8 (write artifacts). ST-2 and ST-4–ST-7 are **pure functions of their
  in-memory inputs** (a raw record / a `Flow` / the resolved set) — tests exercise them with in-memory
  values, which is the Kani/property-test target. The running pipeline pulls flows lazily from the
  I/O-backed ST-3 iterator; purity is a property of the per-item transform, not of the whole stream.
- **Exit semantics:** ST-8 maps result → process exit code so the tool is CI-friendly:
  `0` conformant (no violations; `skipped > 0` still exits 0 but warns — DI-013/DEC-024),
  `1` violations present, `2` policy/usage/I-O error. With `--fail-on-skipped`, `skipped > 0` forces
  a non-zero exit. Exact numeric codes fixed in PRD.
