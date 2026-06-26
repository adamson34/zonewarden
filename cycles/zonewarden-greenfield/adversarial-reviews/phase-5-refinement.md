---
document_type: phase-5-adversarial-refinement
cycle: zonewarden-greenfield
status: complete
date: 2026-06-26
producer: adversarial-review
---

# Phase 5 — Adversarial Refinement (full-codebase)

First whole-system fresh-context pass (the wave gates only ever saw per-wave
diffs). Three parallel adversaries, distinct lenses, over both crates + specs:
(1) pure core domain logic, (2) untrusted-input robustness / effectful shell,
(3) cross-cutting consistency / architecture / spec fidelity.

## Outcome

12 substantive findings (0 CRITICAL, 4 HIGH, 6 MEDIUM-ish, plus LOWs). 4 fixed
immediately (clear bugs), 2 resolved by PO adjudication, the rest backlogged
(Phase 6 / spec hygiene). **190 tests green, clippy -D + fmt clean.**

## Fixed — clear bugs (commit 106cf42)

| ID | Sev | Lens | Issue | Fix |
|----|-----|------|-------|-----|
| P5-CORE-001 | HIGH (security) | core | `0.0.0.0/0` catch-all rejection bypassed by an IPv4-mapped IPv6 `/96` member (`::ffff:x/96` folds to `/0` *after* the pre-fold guard) → silently admits a catch-all shadowing EXTERNAL | `validator.rs` rejects `prefix_len==0` **post-fold** (E-POL-008) + regression test |
| P5-IO-001 | HIGH (DoS) | io | `read_until('\n')` buffered an entire line before the ingest cap → newline-free line OOMs the process (defeats FM-007) | `zeek.rs` bounds the read to `MAX_LINE_BYTES` (1 MiB); over-length line drained in bounded chunks + skipped-and-counted (DI-013) + regression test |
| P5-IO-006 | MED | io | `--output` == `--policy`/`--flows` would overwrite an input on the final rename (violates DI-012 / NFR-006) | `cli.rs` refuses it (exit 2) before any work + integration test |
| P5-IO-005 | MED | io | predictable temp name + `File::create` (follows symlinks) → symlink/PID-collision risk | `reporter.rs` uses O_EXCL (`create_new`) + per-call nonce with retry |

## Resolved — PO adjudication (commit 9462510)

| ID | Sev | Decision | Change |
|----|-----|----------|--------|
| P5-CORE-002 | MED | **Revert to spec** (NoMatchingConduit) | This session's HS-010 fix (src-port reverse match → WrongDirection) contradicted BC-1.04.004 inv 2 + DEC-016. The src-port is an unreliable return-traffic signal (ephemeral ports collide → false WrongDirection). Reverted: reverse match on **responder (dst) port only**. HS-010 holdout corrected to a spec-conforming WrongDirection fixture (Flow B = `control:40000→field:502`, dst:502). |
| P5-CORE-003 | MED | **S1 → Established** | Zeek `S1` = "established, not terminated" = a confirmed established connection. Grading it Attempted under-reported a breach (violates DI-017). Fixed `severity.rs` + the 13-state table tests; spec corrected in BC-1.04.009 + PRD OQ-001. |

## Backlog filed (non-blocking)

**Phase 6 — formal hardening:**
- **P5-XC-001 (HIGH, verification-fidelity):** 3 of 7 declared P0 Kani VPs have no harness — VP-002 (longest-prefix uniqueness), VP-006 (tally overflow / FM-009), VP-010 (multicast exemption / DI-016). Alternative test coverage exists (proptest/unit), so not a runtime blocker, but the "formally verified" claim is overstated. Add the 3 harnesses or reclassify the VPs' `proof_method`.
- **P5-XC-002:** the VP-006/010/002 proof-harness skeletons reference non-existent/drifted APIs (`increment_tally`, `classify_dst(...Option<&ZoneMatch>)`, `PrefixIndex::from_pairs_v4`); regenerate against real signatures.
- **P5-IO-004:** confirm `serde_norway` bounds YAML anchor/alias expansion (billion-laughs); add a bomb regression test, else cap expansion.
- **WAVE5-BL-002 (carried):** offline invariant (BC-1.06.007/DI-012) hard-proof on Linux via `strace -e trace=network` + `unshare --net` + `cargo deny` (macOS can't prove it).

**Robustness (post-MVP):**
- **P5-IO-002 (MED):** `cli.rs` retains all verdicts in memory (O(max_flows)); SS-00-overview specifies "only violations retained." Fold tallies inline during ingest so memory is O(violations).
- **P5-IO-003 (MED):** policy YAML read via `read_to_string` with no size bound; add a size guard.
- **P5-IO-007/008/009 (LOW):** negative fractional timestamp mis-parse (Zeek never emits); `skipped += 1` unchecked (cosmetic vs FM-009); duplicate `#fields` column last-wins.

**Spec hygiene:**
- **P5-XC-003:** VP-INDEX references `verification-architecture.md` + `verification-coverage-matrix.md` which don't exist — create or drop the propagation clause.
- **P5-XC-004:** VP-INDEX Kani count (6) ≠ enumerated Kani VPs (7).
- **P5-XC-005:** BC-1.06.002 violation field list omits `conn_state` (schema + code emit it).
- **P5-XC-006 / ADR-002 wording:** "core has no std::net" should read "no sockets/TcpStream/UdpSocket" (core legitimately uses `std::net::IpAddr` value types).
- **WAVE5-BL-001 (carried):** per-flow skip warnings naming the unspecified address.
- Carried for PO/spec: WAVE4-001 (multicast DstKind vs MatchKind), WAVE3-001 (validator warnings-on-error), WAVE3-005 (dup-id spans).

## Verified clean (coverage confirmation)

Architecture boundary (ADR-002, `#![forbid(unsafe_code)]`, no serde/IO in core);
JSON schema fidelity (BC-1.06.002 field-for-field); determinism end-to-end
(digest, sort, Mermaid emission driven by sorted structures); accounting
identity DI-015 (by construction, checked arithmetic); ingest cap (BC-1.02.006);
IDMZ truth table (DI-006 7×7); resolver/multicast longest-prefix; portset
canonicalization; service inference (D-010 v1.1); no panic/unwrap on any
untrusted-input path; offline (no network deps/symbols).
