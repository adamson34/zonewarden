---
title: Brainstorming Report — OT/ICS Cybersecurity Portfolio Project
date: 2026-06-17
facilitator: Claude (vsdd-factory:brainstorming)
participant: Luke Adamson (OT/ICS security background, 1898 & Co.)
techniques: [trade-off matrix, probing questions, reverse brainstorming, constraint framing]
status: direction-selected
---

# Brainstorming Report

## Session Summary

- **When:** 2026-06-17
- **Domain:** OT/ICS cybersecurity tooling
- **Greenfield?** Yes — empty `devtraining/` workspace, no prior artifacts.
- **Goal:** Build a portfolio project that demonstrates strong software-engineering
  skill + AI-driven development, to support a career move into a software developer
  role. Leverage existing OT/ICS security domain expertise as a credibility moat.
- **Locked constraints (from prior decisions):**
  - **Language:** Rust (pairs with the factory's verification stack — Kani, cargo-fuzz, cargo-mutants).
  - **Pipeline scope:** Start with brainstorming (this report), then proceed through the factory.
- **Techniques used:** feasibility/impact trade-off matrix; probing "war story / demo / data-reality"
  questions; a reverse-brainstorm ("how would this be useless?") to surface scope traps.

## All Ideas Generated

Captured for reference, including directions not selected.

### First set (initial candidates)
1. **ICS honeypot + live dashboard** — simulate a PLC (Modbus TCP/DNP3), capture attacker
   interactions, stream to a web dashboard, map to MITRE ATT&CK for ICS.
   *Strength:* best live-demo "wow." *Trade-off:* UI surface adds scope.
2. **OT asset discovery + CVE-correlated Purdue risk view** — discover devices, build inventory,
   correlate CVEs, render a Purdue risk map.
   *Strength:* safest to scope. *Trade-off:* active scanning needs a lab; least flashy.
3. **OT protocol analyzer / IDS** — parse Modbus/DNP3 traffic, baseline, flag anomalies.
   *Strength:* strongest Rust-parser fit. *Trade-off:* needs sample PCAPs to feel alive.

### Second set (fresh alternatives)
4. **Segmentation-as-Code — OT Zone/Conduit Policy Compiler & Validator** ⭐ SELECTED
   — declare intended IEC 62443 zones/conduits/flows, validate observed reality against them,
   report violations + render Purdue diagram. "Terraform-plan for OT segmentation."
5. **ICS Protocol Robustness Fuzzer & Device Test Harness** — mutate Modbus/DNP3 frames against
   a target, detect mishandling, produce a robustness report. *Rust's home turf (cargo-fuzz native).*
6. **PLC Config Integrity & Drift Monitor** — fingerprint a controller's logic/config export,
   detect & diff unauthorized changes, map to ATT&CK ICS (T0889 Modify Program). *Strongest
   real-world narrative (Stuxnet/TRITON); offline-friendly.*

## Themes & Groupings

- **Live/interactive demo** — ideas 1, 5 (something happens on screen in real time).
- **Detection on captured/observed data** — ideas 3, 6 (analyze inputs, find the bad thing).
- **Policy & architecture validation** — ideas 2, 4 (does reality match intent?).

Idea 4 sits in the "policy & architecture validation" theme but is the most
architecturally rich and the most aligned with the participant's day-to-day OT
assessment work — making it both the strongest *engineering* story and the strongest
*domain-credibility* story.

## Selected Direction

### Segmentation-as-Code — OT Zone/Conduit Policy Compiler & Validator

- **Problem:** OT network segmentation (IEC 62443 zones & conduits; Purdue levels) lives in
  tribal knowledge and hand-maintained firewall rules. Teams cannot *prove* that the network's
  actual state matches the intended segmentation policy, and drift goes unnoticed.
- **Solution:** A Rust tool that (1) ingests an intended segmentation **policy**, (2) ingests
  **observed reality** via pluggable adapters, (3) **validates** reality against policy and emits
  a precise violations report, and (4) **renders** the Purdue/zone-conduit topology as a diagram.
- **Audience:** OT security architects, assessors/auditors (incl. 1898 & Co. engagements),
  plant network engineers.
- **Differentiator:** Policy-as-code for a domain that has essentially none; anchored to
  IEC 62443-3-2. Validation/parsing-heavy core is an ideal showcase for Rust + the factory's
  formal-verification gates (Kani proofs on the rule engine, cargo-fuzz on parsers).

### Value proposition (one line)
> Prove your OT network's real state matches its intended IEC 62443 zone/conduit policy — or get a precise, actionable list of where it doesn't.

### Core loop
```
  Intended policy (zones, conduits, allowed flows)    ─┐
                                                        ├─►  Validator  ─►  Violations report + Purdue diagram
  Observed reality (pluggable "reality adapter")       ─┘
```

### Scoping decisions made this session
- **Reality source:** *Both, flows-primary.* Build a `RealitySource`-style pluggable adapter layer.
  - **v1 (ship first):** network **flow logs** (e.g. Zeek `conn.log` / NetFlow-style flows) →
    check each observed `src→dst:port/proto` against allowed zone-to-zone conduits.
  - **v2 (phase 2 / stretch):** **firewall config** parsing (e.g. iptables or simplified vendor
    syntax) → detect rules that over-permit beyond allowed conduits.
- **Policy format:** *YAML now, custom DSL later* (recommended default; confirm with participant).
  - **Phase 1:** declarative YAML policy via serde → internal policy model.
  - **Phase 2:** purpose-built DSL front-end compiling to the same internal model
    (the Kani-provable / cargo-fuzz'd parser flex).
- **Standards anchor:** IEC 62443-3-2 zones & conduits; Purdue Enterprise Reference Architecture
  levels 0–5.
- **Offline-first:** no live network access required for the core demo — validates files.

### Demo moment (interview screen-share)
Edit a policy, run it against a flow log, watch violations light up
(e.g. *"Level-1 PLC → IT historian on :502 = CONDUIT VIOLATION"*) and the Purdue
diagram redraw with the offending conduit highlighted.

## Reverse-Brainstorm Findings (scope traps to avoid)
- **Fake input = just a linter.** The "observed reality" must come from realistic flow data, not
  hand-typed examples, or the tool has no real signal. → Source/define representative sample flow logs.
- **Over-rigid policy = unusable.** The policy format must express a realistic plant
  (multiple zones, wildcard/range conduits, deny-by-default). → Validate the schema against a
  realistic reference topology early.

## Open Questions for Research / Brief
1. Confirm policy-format decision (YAML-first vs DSL-first). *Default assumed: YAML-first.*
2. Exact flow-log input format for v1 — adopt Zeek `conn.log` field schema, or define a neutral
   normalized flow record (and provide adapters)? Recommend a normalized internal `Flow` record.
3. Diagram output target — SVG, Mermaid, or `.excalidraw`? (Factory has excalidraw tooling.)
4. Source of a realistic reference topology + sample flow logs for tests/demo (synthetic generator
   vs. sanitized real capture). Must be non-confidential.
5. IEC 62443-3-2 conformance depth — how much of the zone/conduit risk-assessment model to model
   vs. a pragmatic subset.
6. Working product name (candidates: `conduit`, `zonewarden`, `segcheck`, `purview-ot`).

## Recommended Next Step
Proceed to **brief creation** (`vsdd-factory:create-brief` / guided brief), carrying the
selected direction, the locked scoping decisions, and the open questions above. A short
targeted research pass (IEC 62443-3-2 zone/conduit model + Zeek conn.log schema) may run first
if needed to firm up Open Questions 2 and 5.
