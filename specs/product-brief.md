# Product Brief: zonewarden

**Author:** Luke Adamson
**Date:** 2026-06-17
**Status:** draft

> **One-liner:** Prove your OT network's real state matches its intended IEC 62443
> zone/conduit policy — or get a precise, actionable list of where it doesn't.
> *"Batfish / `terraform plan` for OT segmentation."*

## Problem Statement

Operational Technology (OT/ICS) network segmentation — the IEC 62443 model of **zones**
(groups of assets sharing security requirements) connected only through sanctioned
**conduits** — is the primary control protecting industrial processes from compromise.
In practice that segmentation lives in tribal knowledge, network diagrams that rot, and
hand-maintained firewall rules. **No one can prove that the network's actual behavior matches
the intended policy**, and drift (a new flow, a forgotten firewall exception, an IT/OT
boundary bypass) goes undetected until an assessment — or an incident.

- **Who has it:** asset owners and the assessors/integrators who serve them (e.g. OT security
  consultancies such as 1898 & Co.). Segmentation conformance is a recurring, manual,
  evidence-light part of nearly every OT security assessment.
- **Stakes:** undetected segmentation drift is how IT-side compromise reaches the process
  network. The scariest OT incidents depend on reaching controllers that *should* have been
  unreachable. Today the gap between "documented policy" and "observed reality" is checked by
  eye, if at all.

## Target Users

- **Primary (builder + first user):** Luke Adamson — OT/ICS security practitioner moving into a
  software-developer role. This product is both a genuinely useful assessment tool and a
  portfolio piece demonstrating disciplined, AI-driven software engineering in Rust.
- **Secondary:**
  - **OT security architects** — author and version segmentation policy as code; validate designs.
  - **Assessors / auditors** (consultancy context) — run repeatable, evidence-producing
    segmentation conformance checks against captured flow data, offline.
  - **Plant / OT network engineers** — catch drift between intended policy and what the network
    is actually doing.
- **Technical sophistication:** high domain knowledge (networks, protocols, 62443/Purdue);
  comfortable on the CLI and with git/text-based config. Not necessarily developers — so the
  policy format and output must be readable and reviewable.

## Value Proposition

**What makes it different:** Research confirmed a genuine whitespace. Existing tools cluster into
config-verification (Batfish, FireMon, AlgoSec, Tufin), runtime enforcement (Illumio, Guardicore,
Elisity), OT monitoring platforms (Claroty, Nozomi, Dragos, Forescout), and general policy-as-code
(OPA, Sentinel) — but **none are an open, declarative, git-versioned tool that validates a
62443 zone/conduit policy against *observed network flows*.** zonewarden owns that intersection:

> open + declarative + observed-flow + 62443/Purdue-native + OT-protocol-aware + offline/CI-friendly.

**The one thing it must do well:** given an intended segmentation policy and a record of observed
flows, **deterministically and explainably identify every flow that violates the policy** —
including the headline Purdue Level 3.5 IDMZ no-bypass rule — with zero false silence
(deny-by-default).

**Engineering thesis (portfolio angle):** segmentation conformance is fundamentally a
**parsing + set-membership + policy-evaluation** problem — an ideal showcase for Rust and the
factory's verification stack. The policy/flow parsers are fuzzable (`cargo-fuzz`); the core
zone-resolution and conduit-matching logic is small, pure, and provable (Kani); correctness is
the product, and the toolchain proves it.

## Success Criteria

**MVP "it works" (the smallest version worth demoing / putting on a resume):**
- Parse a YAML segmentation policy (zones with Purdue level + members; conduits with
  allowed proto/port/direction; deny-by-default).
- Ingest observed flows via a normalized `Flow` record using a **Zeek `conn.log` adapter**.
- Resolve each flow's source/destination to zones and **classify it** as allowed-by-conduit or
  a violation; implement the **Purdue 3.5 IDMZ no-bypass** built-in check.
- Emit (a) a clear **violations report** (human-readable + machine-readable JSON) and
  (b) a **Mermaid** zone/conduit diagram with violations highlighted.
- Ship as a single `zonewarden` CLI binary; runs fully **offline** on files.

**Measurable outcomes:**
- Correctness: a curated test corpus of policy + flow fixtures (incl. a realistic reference
  topology) where every expected violation/allow is asserted; 100% of expected verdicts correct.
- Determinism: identical inputs always produce identical output (stable ordering).
- Verification gates pass: tests green, `cargo-fuzz` finds no parser panics in a bounded run,
  Kani proofs hold on the core matching invariants, mutation score above the factory threshold.
- Demo: edit a policy, re-run against a flow log, watch a violation appear and the diagram update.

**Full vision:** a pluggable, multi-source segmentation conformance engine (flows + firewall
configs), a purpose-built policy DSL, and publication-quality SVG topology output.

## Scope

### In Scope

**MVP (Phase 1):**
- YAML policy format (serde) → internal policy model (zones, conduits, deny-by-default).
- Minimal domain model: `Zone { id, name, purdue_level, sl_t, members }`,
  `Conduit { from_zone, to_zone, proto, ports, direction }`.
- Normalized `Flow` record with a `service_source` provenance field
  (DPI-confirmed vs. port-heuristic vs. unknown).
- **Zeek `conn.log`** reality adapter (the v1 `RealitySource`).
- Validation engine: zone resolution, conduit matching, deny-by-default, Purdue 3.5 IDMZ
  no-bypass built-in check.
- OT protocol/port awareness for service inference: Modbus/502, DNP3/20000, EtherNet-IP/44818,
  S7comm/102, BACnet/47808, OPC UA/4840 — explicitly treated as **heuristic**.
- Outputs: violations report (text + JSON) and **Mermaid** topology diagram.
- `zonewarden` CLI; offline file-based operation.

**Roadmap (time budget is open-ended / learning-first — these are planned, not deferred-forever):**
- Phase 2: **NetFlow/IPFIX** reality adapter (proves the pluggable `RealitySource` design).
- Phase 2: **native SVG** topology rendering.
- Phase 3: **custom policy DSL** front-end (lexer/parser → same internal model) — the
  Kani-provable / `cargo-fuzz`'d parser showcase.
- Phase 3 (stretch): **firewall-config** reality adapter (e.g. iptables / simplified vendor
  syntax) — validate intent against rules, not just flows.

### Out of Scope

- **Live network capture / active scanning** — zonewarden never touches a live network; it
  consumes captured/exported files. (Keeps it safe, legal, and trivially demo-able.)
- **Enforcement / blocking** — it reports conformance; it does not configure firewalls or
  drop traffic.
- **Full IEC 62443-3-2 risk assessment** — the tool consumes the zone/conduit partition (ZCR3)
  and documented requirements/SL-T (ZCR6); it is an automated **FR5 (Restricted Data Flow)**
  conformance checker, not a risk-assessment platform.
- **GUI / web app** — CLI-first for the MVP. (A viewer could come far later; not committed.)
- **Layer-2-only protocols** (e.g. PROFINET RT) that are invisible in IP flow logs — acknowledged
  as undetectable via this data source.
- **Asset discovery / inventory, vulnerability/CVE correlation, IDS/anomaly detection** — adjacent
  ideas considered during brainstorming but explicitly not this product.

## Constraints

- **Language/platform:** Rust; single self-contained CLI binary; cross-platform (Linux/macOS); **64-bit targets only**. Tally counters and `flow_index` are canonical **`u64`** (not platform `usize`); see entities.md / FM-008.
- **Verification:** must pass the factory's quality gates — TDD, adversarial review, `cargo-fuzz`,
  `cargo-mutants`, Kani proofs on core invariants.
- **Offline-first / no secrets:** operates on local files only; no network calls; all sample data
  must be synthetic or sanitized and **non-confidential** (no real client/1898 data).
- **Standards anchor:** IEC 62443-3-2 zones & conduits; Purdue Enterprise Reference Architecture
  levels 0–5. NB: the 62443 standard text is paywalled — model from corroborated public sources.
- **Timeline:** open-ended, learning-first — optimize for engineering rigor and learning the full
  factory pipeline over speed.
- **Resources:** solo builder + AI (Claude / vsdd-factory pipeline).

## Prior Art & References

- **Domain research:** `.factory/specs/research/domain-ot-segmentation-validation-2026-06-17.md`
  (IEC 62443-3-2 model, Zeek/NetFlow schemas, OT ports, prior-art gap matrix).
- **Brainstorming:** `.factory/planning/brainstorming-report.md`.
- **Closest analogues / inspiration:**
  - *Batfish* — network configuration analysis & policy verification (IT-focused; the mental model).
  - *terraform plan* — declarative desired-state vs. observed diff (the UX metaphor).
  - *OPA / Sentinel* — policy-as-code engines (general; not OT/flow-aware).
- **Not replacing any internal tool** — greenfield; complements (does not duplicate) commercial OT
  monitoring platforms by being open, declarative, offline, and 62443-policy-native.

## Open Questions

Resolved during brief creation:
- ✅ Product name → **zonewarden**.
- ✅ Diagram output → **Mermaid for MVP**, native SVG as a Phase-2 stretch.
- ✅ MVP bar → full core loop (validate + report + diagram).
- ✅ Time budget → open-ended / learning-first (full rigor, roadmap stretches in scope).
- ✅ Flow input → **neutral normalized `Flow` record + adapter layer** (Zeek adapter first).

Still open (carry into PRD / domain spec):
1. **Reference topology & fixtures:** define the realistic synthetic plant (zones, conduits,
   sample flows) used for tests/demo — must be non-confidential. Synthetic generator vs.
   hand-authored fixtures?
2. **`service_source` precedence rules:** when port-heuristic and any future DPI signal disagree,
   which wins, and how is it surfaced in the report?
3. **Conduit expressiveness for MVP:** port ranges, protocol wildcards, bidirectional conduits —
   how much of this is in the YAML schema for Phase 1 vs. later?
4. **Verification depth target:** which exact core invariants get Kani proofs, and the
   `cargo-mutants` score threshold to treat as "converged."
5. **`crates.io` name availability** for `zonewarden` (verify before any publish).
