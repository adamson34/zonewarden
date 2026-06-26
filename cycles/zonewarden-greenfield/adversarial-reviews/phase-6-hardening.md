---
document_type: phase-6-formal-hardening
cycle: zonewarden-greenfield
status: complete
date: 2026-06-26
producer: formal-verifier
---

# Phase 6 — Formal Hardening

Closes the Phase 5 verification-fidelity gap (P5-XC-001) and adds parser fuzzing,
a supply-chain check, and a YAML-bomb guard.

## Kani formal proofs — 7 harnesses, all SUCCESSFUL

Three new harnesses written this phase (the previously-undelivered P0 VPs), each
`VERIFICATION:- SUCCESSFUL` under `cargo kani` 0.67:

| VP | Harness | Module | Property |
|----|---------|--------|----------|
| VP-002 | `longest_prefix_wins` | resolver | Longest-prefix selection: a /24 over an overlapping /16 — every IP in the /24 selects the /24, /16-only selects the /16, else EXTERNAL (DI-004/BC-1.03.001). Asserts on matched `prefix_len` to avoid CBMC memcmp unwinding on zone-id strings. |
| VP-006 | `checked_inc_never_wraps` | aggregator | `checked_inc` returns `Ok(n+1) > n` for `n < u64::MAX` and `Err(TallyOverflow)` exactly at `u64::MAX` — never silently wraps (FM-009/BC-1.05.004). |
| VP-010 | `step1_ipv4_multicast_broadcast_exempt`, `step1_ipv6_multicast_exempt` | multicast | Step-1 family-wide multicast/broadcast exemption is total + correct over the whole IPv4/IPv6 space (DI-016/BC-1.03.003). |

Pre-existing harnesses re-confirmed SUCCESSFUL: VP-001 (resolver totality),
VP-003 (idmz truth table), VP-004 (verdict totality).

**VP-008 (portset canonical form):** the existing Kani harness unwinds a
symbolic-length `sort_by` loop and does not terminate in a practical bound;
the property is covered by proptest. Marked `partial` in VP-INDEX — revisit with
an explicit `#[kani::unwind]`/bounded-length input.

## Fuzzing — cargo-fuzz / libfuzzer, zero crashes

| Target | Surface | Runs | Result |
|--------|---------|------|--------|
| `fuzz_zeek` | `ZeekAdapter::from_reader` over arbitrary bytes | 1,270,392 | no panic / no crash |
| `fuzz_policy` | `policy::load_str` over arbitrary UTF-8 | 2,180,078 | no panic / no crash |

Added `pub policy::load_str` (the pure parse step of `load`) for in-memory
fuzzing. Targets live in `fuzz/`; corpus/artifacts gitignored. Closes WAVE2-003
(cargo-fuzz on the parsers).

## Supply chain — cargo-deny

`cargo deny check` → **advisories ok, bans ok, licenses ok, sources ok** (only
benign unused-license-allowance warnings). No security advisories, no banned
crates, license-clean. Partially closes WAVE5-BL-002 (the cargo-deny half).

## YAML-bomb guard (P5-IO-004)

`deny_unknown_fields` + the typed schema reject an alias "billion-laughs" bomb
before expansion (top-level anchor keys are unknown fields). Locked in by
`test_yaml_alias_bomb_rejected`.

## Mutation testing — cargo-mutants (supplementary)

Run on `zonewarden-core` as a test-strength probe (no facade stories, so the
≥80% floor is not a hard gate here). Results + dispositions: see
`.factory/logs/mutation-report-phase6-core.md`. Notable surviving mutants are
filed as a test-gap backlog item; mutants inside `#[cfg(kani)]` harness bodies
are verification-only and excluded from the meaningful denominator.

## Backlog remaining (carried)

- **WAVE5-BL-002 (offline hard-proof):** `strace -e trace=network` + `unshare --net` on Linux (macOS lacks both). cargo-deny half done.
- **VP-008 Kani:** add a bounded harness so the portset canonical-form proof terminates.
- **Mutation test gaps:** add tests for the surviving classifier/other mutants flagged in the mutation report.
- Spec hygiene (P5-XC-002/003/005/006), robustness (P5-IO-002/003/007), and the carried PO/spec items (WAVE4-001, WAVE3-001/005).
