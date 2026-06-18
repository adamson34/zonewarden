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

# ADR-005: YAML Parser — `serde_norway` for Duplicate-Key Detection

**Status:** accepted

## Context

DI-010 requires that duplicate YAML mapping keys be detected and rejected as a policy
error (E-POL-004, BC-1.01.003). This is a security property: a duplicate key could be
used to smuggle a second `members` list or shadow a conduit directive. The standard
`serde_yaml` crate (v0.9) silently accepts the last value for duplicate keys and is
additionally deprecated as of late 2024.

## Options Considered

1. **`serde_yaml` 0.9** — Deprecated; silent last-wins on dup keys. Rejected: violates
   DI-010 and BC-1.01.003.
2. **`serde_norway`** (libyaml-safer fork) — Strict mode errors on duplicate mapping
   keys. Maintained; supports serde derive. **Chosen.**
3. **`yaml-rust2` + hand-rolled serde** — Full control; can detect dups by walking the
   event stream. High implementation effort; higher bug surface. Fallback if (2) proves
   problematic.
4. **`marked-yaml`** — YAML with source-location annotations; dup detection in warn mode
   only. Not strict enough for our requirement.

## Decision

Use `serde_norway` (the `libyaml-safer`-based serde backend) in strict mode. This crate
surfaces a `DeserializerConfig::strict_keys(true)` option that returns an error when a
mapping key appears twice.

**Fallback:** If `serde_norway` is not available on crates.io at implementation time or
has unresolved issues, fall back to `yaml-rust2` event-stream approach (manual dup
detection before serde). This fallback is documented so the implementer does not default
back to `serde_yaml`.

## Rationale

- DI-010 is a security property; silent key clobbering is an attack surface.
- `serde_norway` requires minimal implementation delta vs `serde_yaml` — mostly a crate
  swap.
- `serde_yaml` deprecation means its vulnerability window grows over time; switching now
  avoids a forced migration later.

## Consequences

- `policy` module uses `serde_norway::Deserializer::from_str` with strict mode.
- Policy struct derives `serde::Deserialize`; field names match YAML schema exactly.
- The `purdue_level`, `direction`, and `proto` fields deserialize via custom `Visitor`
  impls that reject unrecognized tokens (E-POL-010/011/013; BC-1.01.007).

## Verification Feasibility

YAML parsing is effectful (string → struct); it is the **fuzz target** `fuzz_policy_parse`,
not a Kani target. The fuzz harness exercises `policy::load_from_str(input)` with arbitrary
UTF-8 byte sequences. NFR-009: zero panics in 10-minute run.
