<!-- Thanks for the PR. Please fill in the sections below. -->

## Summary

<!-- One-paragraph description of what this PR does and why. -->

## Spec / contract

<!-- Link the behavioral contract(s) or story this traces to (BC-S.SS.NNN),
     or the .factory/specs/ doc. If it's a trivial fix or config tweak, say so
     and skip. -->

## What this is NOT

<!-- Naming the related thing this PR specifically doesn't do helps scope the
     review. Optional but encouraged. -->

## Determinism / core-purity (delete if not applicable)

<!-- If this PR touches zonewarden-core or any output-shaping code:
       - Does output stay byte-identical for identical input (sort keys,
         digest, Mermaid emission driven by sorted structures)?
       - Did you keep the pure core free of I/O / serde / sockets?
     If neither applies, delete this section. -->

## Test plan

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-features`
- [ ] New tests added if a new module / verdict / edge case landed
- [ ] New tests trace to a behavioral contract id (`test_BC_...`)
- [ ] Verified end-to-end on a fixture if user-facing (CLI output / exit code)
