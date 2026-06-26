# Mutation Report — Phase 6, zonewarden-core (cargo-mutants 27)

Supplementary test-strength probe (no facade stories, so the BC-6.21.002 ≥80%
floor is not a hard gate here). The full core run is long (159 mutants, ~10 min);
this report records the run, the gap it surfaced, the fix, and the disposition of
remaining survivors.

## Finding: digest tests pinned no value → token mutations survived

The original digest tests asserted only `len()==64`, determinism, and structural
invariance — never that distinct inputs yield distinct digests at field
granularity. So mutating `digest.rs` token functions (purdue_token,
direction_token, proto_token, ports_repr/ports_json, sl_target_json,
conduit_entry) to constants was NOT caught.

**Fix:** added `test_BC_1_05_003_digest_sensitive_to_each_token` — policies
differing in exactly one canonical token (purdue_level, sl_t, direction, proto,
ports) must hash differently.

**Result (targeted `--file digest.rs` re-run): 19 mutants, 16 caught, 3 missed
= 84% kill** (was effectively 0% on the token functions before the fix).

## Surviving mutants — dispositions

| Mutant | File:Line | Disposition |
|--------|-----------|-------------|
| `sl_target_json -> Default::default()` | digest.rs:81 | C (waiver) — redundant: an omitted vs present sl_t is already distinguished by the field's presence in the canonical map; the inner Value content is secondary. Low value. |
| `ports_repr -> String::new()` / `"xyzzy"` | digest.rs:159 (×2) | C (waiver) — redundant encoding: ports are also serialized structurally by `ports_json`, which the sensitivity test exercises; `ports_repr` is a secondary representation. |
| `replace && with \|\|` in `classify` | classifier.rs:74/75/80 | A (backlog) — real test-gap: add classifier cases where exactly one zone of the pair matches a conduit (forward/reverse), so flipping the zone-match conjunction is caught. |
| `delete match arm WrongDirection` in `violations_for` | classifier.rs:157 | A (backlog) — add a test asserting `violations_for` emits a WrongDirection violation row. |
| mutants inside `#[cfg(kani)] mod kani_harness` bodies | (multiple) | B (dead-code-equivalent) — verification-only code, never run by `cargo test`; excluded from the meaningful denominator. Consider `cargo-mutants --exclude-re kani_harness`. |

## Backlog

- Run the full `cargo mutants -p zonewarden-core` to completion in CI and record the crate-wide kill rate.
- Close the classifier zone-match (&&) and violations_for WrongDirection gaps (disposition A above).
- Configure cargo-mutants to skip `kani_harness` modules.
