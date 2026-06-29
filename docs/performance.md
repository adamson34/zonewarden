# Performance Budgets (NFR-PERF)

zonewarden is an **offline batch CLI**: it reads captured files and exits. It has
no real-time SLA and no latency-sensitive request path. Performance is therefore
expressed as **throughput** on the hot path, with a regression guard rather than
a hard wall-clock deadline.

## Hot path

The per-flow `classify` loop (pure core, `zonewarden-core::classifier::classify`)
is run once per observed flow and dominates CPU for large captures. It is the
single throughput-critical function; everything upstream (validate, resolve) is
one-time per run, and reporting is O(violations).

## Budget

| Metric | Budget | Baseline (2026-06-29) | Rationale |
|--------|--------|-----------------------|-----------|
| `classify` throughput (single-threaded, mixed verdicts) | **≥ 5 M flows/sec** | ~20 M flows/sec (`benches/classify.rs`, 10k-flow batch, ~501 µs) | Regression guard at ~4× margin below baseline. At the budget floor, the default `--max-flows` cap of 1,000,000 classifies in < 1 s, so classify is never the bottleneck for a realistic capture (line I/O dominates). |

Memory: the run is bounded by `--max-flows` (default 1,000,000) and the offline
invariant; only violations are retained for reporting (O(violations), not
O(flows)) — see SS-00-overview. (Note: P5-IO-002 tracks folding tallies fully
inline; non-blocking.)

## Measuring

```sh
cargo bench -p zonewarden-core --bench classify
```

Criterion prints `thrpt: [… Melem/s …]`. Compare against the budget floor above.
Update the baseline column here when an intentional change moves it; investigate
any drop below the 5 M flows/sec floor before merging.

## Scope / non-budgets

- **Ingest/parse throughput** (Zeek adapter, shell crate) is I/O-bound and not
  currently budgeted; revisit if a profiling pass shows it dominating on real
  captures.
- **Startup / one-time cost** (policy load + validate + resolver index build) is
  negligible relative to per-flow work for any non-trivial capture and is not
  budgeted.
