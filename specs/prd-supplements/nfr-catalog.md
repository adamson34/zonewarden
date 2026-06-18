---
document_type: prd-supplement-nfr-catalog
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-17T00:00:00
phase: 1a
inputs: [prd.md, domain-spec/L2-INDEX.md]
input-hash: "[live-state]"
traces_to: specs/prd.md
---

# Non-Functional Requirements Catalog: zonewarden

> PRD supplement — extracted from PRD Section 4.
> Referenced by: architect, performance-engineer, formal-verifier.

## NFR Registry

| ID | Category | Requirement | Target | Validation Method | Priority | Risk Source |
|----|----------|-------------|--------|------------------|----------|-------------|
| NFR-001 | Determinism | Identical inputs (policy + flows) always produce byte-identical outputs | 100% reproducible; any divergence is a bug | Kani proof on ordering + digest; CI runs same input twice and diffs output | P0 | N/A (core invariant DI-009) |
| NFR-002 | Performance | Flow classification throughput | ≥ 100,000 flows/second on a modern laptop (2024-era, single core) | Benchmark via `cargo bench` with a 1M-flow synthetic corpus | P0 | R-005 (no real test data) |
| NFR-003 | Memory | Memory growth with input size (aggregation stage) | ≤ 2 GB peak RSS for 1M flows (violations only retained, not all verdicts per OQ-003 resolution) | Heap profiler (`heaptrack`/`valgrind`) on 1M-flow run | P0 | R-005 |
| NFR-004 | Ingest cap | Maximum flow count per run (safety ceiling) | Default cap: 1,000,000 flows; configurable via `--max-flows` up to `u64::MAX - 1` | Integration test: run with `max_flows+1` input; verify exit 2 | P0 | FM-008, FM-009 |
| NFR-005 | Offline | No network calls during any operation | Zero network I/O; verified by syscall tracing | strace/dtrace in CI: zero `socket()` or `connect()` syscalls from zonewarden process | P0 | DI-012 |
| NFR-006 | Read-only | Input files never modified or truncated | Zero write/truncate syscalls on `--policy` and `--flows` paths | strace/dtrace; file modification time before/after run unchanged | P0 | DI-012 |
| NFR-007 | Formal Verification | Kani proof targets pass | All ⊢-marked invariants (DI-003, DI-004, DI-006, DI-009, DI-012, DI-015, DI-016, DI-018, DI-020) have Kani harnesses that pass | `cargo kani` CI gate: all harnesses reach proof conclusion | P0 | DI-003/004/006 (security) |
| NFR-008 | Mutation Testing | cargo-mutants kill rate by module tier | CRITICAL modules ≥ 95%; HIGH modules ≥ 90%; MEDIUM modules ≥ 80% | `cargo mutants` CI gate; fail build if below threshold | P0 | R-002 (silent allow vector) |
| NFR-009 | Fuzz Testing | Parser panic-freedom | Zero parser panics in a 10-minute bounded cargo-fuzz run on all fuzz targets | `cargo fuzz run` with 10-min timeout; no crash/hang outputs | P0 | DI-010 (policy parser), DI-013 (flow parser) |
| NFR-010 | Correctness | Golden corpus pass rate | 100% of expected verdicts correct for the synthetic reference topology test corpus | Golden test suite (`cargo test`) with hand-authored expected verdicts | P0 | R-001 (false confidence), R-002 (silent allow) |
| NFR-011 | Startup latency | Binary startup and policy load time | ≤ 500ms to first output for a 10-zone, 20-conduit policy with 0 flows | Timed integration test | P1 | N/A |
| NFR-012 | Binary size | Self-contained CLI binary size | ≤ 20 MB on Linux x86_64 (stripped release build) | CI: check binary size in release workflow | P1 | N/A |
| NFR-013 | Cross-platform | Target platforms | Linux x86_64 and macOS aarch64 (Apple Silicon); 64-bit only | CI matrix: ubuntu-latest + macos-latest; all tests pass | P0 | D-002 |

## NFR Categories

| Category | Description | Validation Agent |
|----------|-------------|-----------------|
| Determinism | Output reproducibility | formal-verifier |
| Performance | Throughput, latency, startup | performance-engineer |
| Memory | Peak RSS, streaming bounds | performance-engineer |
| Offline | Network I/O absence | formal-verifier / devops-engineer |
| Read-only | Input immutability | formal-verifier |
| Formal Verification | Kani proof gates | formal-verifier |
| Mutation Testing | cargo-mutants kill rates | formal-verifier |
| Fuzz Testing | cargo-fuzz panic-freedom | formal-verifier |
| Correctness | Golden test corpus | test-writer, holdout-evaluator |

## NFR-to-Module Mapping

| NFR ID | Affected Subsystems | Architectural Impact |
|--------|---------------------|---------------------|
| NFR-001 | SS-05 (ordering), SS-01 (digest) | Sort key must be deterministic; no hash-map iteration in output ordering |
| NFR-002 | SS-02, SS-03, SS-04, SS-05 | Hot path must avoid unnecessary allocations per flow |
| NFR-003 | SS-05 | Aggregate only violations + tallies; stream per-flow verdicts rather than materializing all |
| NFR-004 | SS-02 | Ingest cap check at ST-3; counter incremented per successfully-normalized flow |
| NFR-005 | All (cross-cutting) | No network-access crates in dependency tree of the main binary |
| NFR-006 | SS-01, SS-02 | Open input files with O_RDONLY; no accidental write open |
| NFR-007 | SS-01 (DI-020 PortSet), SS-03 (DI-003/004), SS-04 (DI-006/015/016), SS-05 (DI-009/018) | Pure-function design required for all Kani targets |
| NFR-008 | SS-01, SS-03, SS-04 (CRITICAL); SS-02, SS-05 (HIGH); SS-06 (MEDIUM) | See module-criticality.md for per-module targets |
| NFR-009 | SS-01 (YAML policy parser), SS-02 (conn.log line parser) | Two fuzz targets minimum: `fuzz_policy_parse`, `fuzz_flow_parse` |
| NFR-010 | SS-03, SS-04 (verdict correctness) | Reference topology fixture set; golden expected outputs; run in CI |
| NFR-013 | All | Rust cargo targets: `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin` |

## Module Criticality Summary (from module-criticality.md)

| Subsystem | Criticality | Kill Rate Target |
|-----------|-------------|-----------------|
| SS-01 Policy | CRITICAL | ≥ 95% |
| SS-02 Flow Ingest | HIGH | ≥ 90% |
| SS-03 Zone Resolution | CRITICAL | ≥ 95% |
| SS-04 Classification | CRITICAL | ≥ 95% |
| SS-05 Aggregation | HIGH | ≥ 90% |
| SS-06 Reporting | MEDIUM | ≥ 80% |
