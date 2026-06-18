---
document_type: adr
level: L3
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-06-17T00:00:00
phase: 1b
traces_to: ARCH-INDEX.md
---

# ADR-007: Mermaid Diagram — String Generation, No Render Dependencies

**Status:** accepted

## Context

BC-1.06.004 requires a Mermaid zone/conduit diagram with violations highlighted. The
question is whether zonewarden renders the Mermaid to an image internally (requiring
Node.js / puppeteer / headless browser) or emits the Mermaid DSL string and delegates
rendering to the consumer (GitHub Markdown renderer, VS Code, Obsidian, etc.).

## Options Considered

1. **Emit Mermaid DSL string only (chosen)** — `reporter` generates the Mermaid
   `graph LR` string; the consumer renders it. Zero additional deps.
2. **Embed `mermaid-js` via WASM** — Offline rendering; large binary (+10–20 MB);
   complex build; WASM runtime needed. Rejected for MVP.
3. **Invoke `mmdc` (Mermaid CLI) as a subprocess** — Requires Node.js at runtime;
   breaks offline guarantee (Node could phone home). Rejected.
4. **Native SVG (CAP-012)** — Planned for P1. Not available for MVP.

## Decision

`reporter` generates the Mermaid DSL string directly. No Mermaid library or runtime
dependency. The output is a valid Mermaid `graph LR` block that renders correctly in:
- GitHub Markdown (native Mermaid support since 2022)
- VS Code with Mermaid Preview extension
- Obsidian, Notion (Mermaid-aware)

Violations are annotated via Mermaid `:::violation` CSS class (supported in current
Mermaid versions). Zone nodes sorted by zone ID for deterministic output.

## Rationale

- Zero dependencies: satisfies NFR-012 (≤20 MB binary) and DI-012 (offline).
- The value is in the structured data, not the rendering. Most CI pipelines render
  Mermaid in their Markdown preview; no local rendering is needed.
- Native SVG (CAP-012) is the natural upgrade path for rich rendering without Node.js.

## Consequences

- `reporter::emit_mermaid(result, policy)` is a pure string-generating function (no I/O).
  It is testable via golden string comparison.
- The Mermaid string output is deterministic (zones sorted by ID; conduits sorted by
  `(from_zone, to_zone)`; violations annotated inline). This satisfies DI-009 for the
  Mermaid output format.
- Not a Kani target; tested via golden output fixtures in integration tests.

## Verification Feasibility

String generation is not Kani-provable. Tested via:
- Unit tests: golden Mermaid string for a known policy+result fixture.
- Property test: Mermaid output length is O(zones + conduits + violations); no panics.
