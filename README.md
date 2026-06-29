# zonewarden

**Segmentation-as-code for OT/ICS networks.** Prove that an industrial network's *real* traffic
matches its *intended* IEC 62443 zone/conduit policy — or get a precise, actionable list of where it
doesn't.

> Think **`terraform plan` / Batfish, but for OT segmentation.**

zonewarden takes a declarative segmentation **policy** (zones, conduits, allowed flows) plus a record
of **observed network flows** (e.g. Zeek `conn.log`) and deterministically classifies every flow as
allowed or violating — including the headline **Purdue Level 3.5 IDMZ no-bypass** check — then emits a
violations report and a Purdue-tiered topology diagram. It runs fully **offline** on captured files, so
it's safe to run anywhere and trivial to wire into CI.

It is, in IEC 62443 terms, an automated **FR5 (Restricted Data Flow)** conformance checker.

---

## Why this exists

OT network segmentation is the primary control protecting industrial processes, yet it lives in tribal
knowledge, aging diagrams, and hand-maintained firewall rules. Nobody can *prove* the network's actual
behavior matches the intended policy, and drift goes unnoticed until an assessment — or an incident.
The tooling landscape has config-verification tools, runtime enforcement platforms, and OT monitoring
suites, but **no open, declarative, git-versioned tool that validates a 62443 zone/conduit policy
against observed flows.** zonewarden fills that gap.

## Status

✅ **Working MVP.** The full pipeline — load → validate → ingest → classify → aggregate → report —
runs end-to-end on Zeek `conn.log` input, with JSON / text / Mermaid output and a deterministic
policy digest. All 17 stories across 5 waves are implemented and the suite is green.

| Phase | Status |
|-------|--------|
| Spec crystallization (domain model, PRD, behavioral contracts, architecture) | ✅ complete |
| Story decomposition (epics, stories, waves, holdout scenarios) | ✅ complete |
| Test-first implementation (Rust) | ✅ complete — 200 tests, all 5 wave gates passed |
| Formal hardening (Kani proofs, fuzzing, mutation, supply-chain) | ✅ complete — 7 Kani proofs, 3.4M fuzz runs clean |

## Usage

```sh
# Build the CLI
cargo build --release

# Validate observed flows against a policy (exit 0 = conformant, 1 = violations, 2 = error)
zonewarden --policy policy.yaml --flows conn.log

# JSON report for CI pipelines
zonewarden --policy policy.yaml --flows conn.log --format json

# Purdue-tiered topology diagram (violations highlighted)
zonewarden --policy policy.yaml --flows conn.log --format mermaid

# Bound ingest for very large captures
zonewarden --policy policy.yaml --flows conn.log --max-flows 1000000
```

**Exit codes** (ST-8): `0` conformant · `1` violations found · `2` usage/policy/limit error.
The text summary reports per-verdict tallies (allowed, no-matching-conduit, wrong-direction,
multicast-exempt, IDMZ bypasses, …); JSON adds the full violation list with a stable
`schema_version` and `policy_digest`.

## How it's being built

This project is a deliberate exercise in **rigorous, specification-driven development**. Before any
code was written, the problem was modeled end to end and the design was stress-tested:

- **A formal L2 domain model** — entities, 20 domain invariants, 33 edge cases, ubiquitous language —
  hardened through **8 rounds of adversarial review** (every finding tracked and resolved).
- **An L3 PRD with 44 behavioral contracts** (`BC-S.SS.NNN`), each with testable pre/postconditions and
  canonical test vectors, fully traced back to the domain invariants.
- **An architecture** with Architecture Decision Records and an explicit **pure-core / effectful-shell
  boundary** enforced at the Cargo crate level — so the correctness-critical logic is a pure library
  that can be **formally verified** (Kani) and property-tested.
- **10 verification properties** mapping invariants to proof methods (Kani / proptest / fuzz).
- **17 dependency-ordered stories** across 5 waves, every acceptance criterion traced to a contract.

The full paper trail lives in [`.factory/`](.factory/) — including the design decisions
([`specs/architecture/decisions/`](.factory/specs/architecture/decisions/)) and the complete
adversarial-review history ([`cycles/`](.factory/cycles/)).

## Design at a glance

```
  Policy (zones, conduits)  ─┐
                             ├─►  validate ─► resolve ─► classify ─► aggregate ─►  report + diagram
  Observed flows (adapter)  ─┘
```

- **Deterministic & offline** — identical inputs always yield byte-identical output; no network access.
- **Deny-by-default** — anything not explicitly allowed by a conduit (or intra-zone) is a violation.
- **OT-protocol aware, honestly heuristic** — recognizes Modbus / DNP3 / EtherNet-IP / S7comm / BACnet /
  OPC UA, with transparent provenance on every inference.
- **Pluggable reality sources** — Zeek `conn.log` first; NetFlow/IPFIX and firewall configs on the roadmap.

## Tech

- **Rust** — a pure `zonewarden-core` library crate (the verification target) and a thin `zonewarden` CLI binary.
- Verification: **Kani** (proofs), **proptest** (properties), **cargo-fuzz** (parsers), **cargo-mutants** (mutation testing).

## Standards

- IEC 62443-3-2 (zones & conduits; FR5 Restricted Data Flow)
- Purdue Enterprise Reference Architecture (levels 0–5, IDMZ at 3.5)

## License

[MIT](LICENSE).
