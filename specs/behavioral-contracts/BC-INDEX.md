---
document_type: behavioral-contract-index
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-17T00:00:00
phase: 1a
traces_to: specs/prd.md
---

# Behavioral Contract Index: zonewarden

**Total contracts:** 44
**Status:** All active v0.1.0

## SS-01: Policy (Load + Validate + Model)

> CAP-001, CAP-002 | Criticality: CRITICAL

| BC ID | Title | Priority | Status | File |
|-------|-------|----------|--------|------|
| BC-1.01.001 | Parse valid YAML policy into internal Policy model | P0 | active | ss-01/BC-1.01.001.md |
| BC-1.01.002 | Reject malformed YAML with precise diagnostic (fail-fast) | P0 | active | ss-01/BC-1.01.002.md |
| BC-1.01.003 | Reject duplicate YAML mapping keys at load time | P0 | active | ss-01/BC-1.01.003.md |
| BC-1.01.004 | Validate zone IDs unique and conduit endpoints exist | P0 | active | ss-01/BC-1.01.004.md |
| BC-1.01.005 | Reject equal-prefix-length ties (same-family, same address) | P0 | active | ss-01/BC-1.01.005.md |
| BC-1.01.006 | Reject 0.0.0.0/0 or ::/0 catch-all member declarations | P0 | active | ss-01/BC-1.01.006.md |
| BC-1.01.007 | Reject unrecognized direction/proto tokens (no permissive default) | P0 | active | ss-01/BC-1.01.007.md |
| BC-1.01.008 | Warn (not error) on declared zone with zero members | P0 | active | ss-01/BC-1.01.008.md |
| BC-1.01.009 | PortSet canonical form: sorted non-overlapping non-adjacent ranges | P0 | active | ss-01/BC-1.01.009.md |

## SS-02: Flow Ingest and Normalization

> CAP-003, CAP-004 | Criticality: HIGH

| BC ID | Title | Priority | Status | File |
|-------|-------|----------|--------|------|
| BC-1.02.001 | Parse valid Zeek conn.log line into normalized Flow | P0 | active | ss-02/BC-1.02.001.md |
| BC-1.02.002 | Skip and count malformed flow lines; never abort the run | P0 | active | ss-02/BC-1.02.002.md |
| BC-1.02.003 | Skip and warn flows with unspecified address (0.0.0.0 or ::) as src or dst | P0 | active | ss-02/BC-1.02.003.md |
| BC-1.02.004 | Assign service and service_source via canonical port/proto table | P0 | active | ss-02/BC-1.02.004.md |
| BC-1.02.005 | Canonicalize IPv4-mapped IPv6 addresses to IPv4 before resolution | P0 | active | ss-02/BC-1.02.005.md |
| BC-1.02.006 | Enforce max_flows ingest cap; abort with exit 2 on breach | P0 | active | ss-02/BC-1.02.006.md |

## SS-03: Zone Resolution

> CAP-005 | Criticality: CRITICAL

| BC ID | Title | Priority | Status | File |
|-------|-------|----------|--------|------|
| BC-1.03.001 | Resolve endpoint to exactly one zone via longest-prefix match | P0 | active | ss-03/BC-1.03.001.md |
| BC-1.03.002 | Resolve unmatched endpoint to implicit EXTERNAL zone | P0 | active | ss-03/BC-1.03.002.md |
| BC-1.03.003 | Multicast/broadcast destination short-circuits to MulticastBroadcast before zone resolution | P0 | active | ss-03/BC-1.03.003.md |
| BC-1.03.004 | Directed-broadcast destination override: all-ones host of ≤/30 zone → MulticastBroadcast | P0 | active | ss-03/BC-1.03.004.md |
| BC-1.03.005 | Both-EXTERNAL flow: two endpoints both resolving to EXTERNAL → IntraZone | P0 | active | ss-03/BC-1.03.005.md |

## SS-04: Classification and Verdict

> CAP-006, CAP-007, CAP-008 | Criticality: CRITICAL

| BC ID | Title | Priority | Status | File |
|-------|-------|----------|--------|------|
| BC-1.04.001 | Intra-zone flows are allowed without conduit evaluation | P0 | active | ss-04/BC-1.04.001.md |
| BC-1.04.002 | Deny-by-default: flow with no matching conduit → NoMatchingConduit | P0 | active | ss-04/BC-1.04.002.md |
| BC-1.04.003 | Any-match conduit union: flow allowed if ≥1 conduit permits it | P0 | active | ss-04/BC-1.04.003.md |
| BC-1.04.004 | Forward conduit enforces directionality; reverse-initiated flow → WrongDirection | P0 | active | ss-04/BC-1.04.004.md |
| BC-1.04.005 | Bidirectional conduit permits initiation from either zone | P0 | active | ss-04/BC-1.04.005.md |
| BC-1.04.006 | Portless protocol (ICMP/Other) matches only ports: Any conduit | P0 | active | ss-04/BC-1.04.006.md |
| BC-1.04.007 | IDMZ no-bypass: managed ≤L3 ↔ managed ≥L4 without IDMZ endpoint → additive IdmzBypass finding | P0 | active | ss-04/BC-1.04.007.md |
| BC-1.04.008 | IDMZ bypass is NOT raised for flows involving EXTERNAL or MulticastBroadcast endpoints | P0 | active | ss-04/BC-1.04.008.md |
| BC-1.04.009 | Violation severity graded from conn_state: Attempted bucket → Attempted; otherwise → Established | P0 | active | ss-04/BC-1.04.009.md |
| BC-1.04.010 | Verdict totality: every resolved flow receives exactly one VerdictKind | P0 | active | ss-04/BC-1.04.010.md |
| BC-1.04.011 | MulticastExempt short-circuits before IntraZone and conduit evaluation | P0 | active | ss-04/BC-1.04.011.md |

## SS-05: Aggregation and Determinism

> CAP-009 | Criticality: HIGH

| BC ID | Title | Priority | Status | File |
|-------|-------|----------|--------|------|
| BC-1.05.001 | DI-015 accounting identity holds for every ConformanceResult | P0 | active | ss-05/BC-1.05.001.md |
| BC-1.05.002 | Output ordered by total-order key (ts, src_ip, src_port, dst_ip, dst_port, proto, flow_index) | P0 | active | ss-05/BC-1.05.002.md |
| BC-1.05.003 | Stable policy digest via canonical JSON serialization (SHA-256) | P0 | active | ss-05/BC-1.05.003.md |
| BC-1.05.004 | u64 tally overflow: checked arithmetic aborts with exit 2; never silent-wraps | P0 | active | ss-05/BC-1.05.004.md |
| BC-1.05.005 | Empty flow input yields all-zero ConformanceResult; exits 0 | P0 | active | ss-05/BC-1.05.005.md |

## SS-06: Reporting and CLI

> CAP-010 | Criticality: MEDIUM

| BC ID | Title | Priority | Status | File |
|-------|-------|----------|--------|------|
| BC-1.06.001 | Exit codes: 0 conformant, 1 violations, 2 error/policy/usage | P0 | active | ss-06/BC-1.06.001.md |
| BC-1.06.002 | Emit JSON violations report with all required fields | P0 | active | ss-06/BC-1.06.002.md |
| BC-1.06.003 | Emit human-readable text violations report | P0 | active | ss-06/BC-1.06.003.md |
| BC-1.06.004 | Emit Mermaid zone/conduit diagram with violations highlighted | P0 | active | ss-06/BC-1.06.004.md |
| BC-1.06.005 | Warnings emitted to stderr in deterministic order; never change exit code | P0 | active | ss-06/BC-1.06.005.md |
| BC-1.06.006 | --fail-on-skipped: skipped > 0 forces non-zero exit | P0 | active | ss-06/BC-1.06.006.md |
| BC-1.06.007 | zonewarden never opens a network socket or mutates input files (offline invariant) | P0 | active | ss-06/BC-1.06.007.md |
| BC-1.06.008 | Output artifacts written atomically (write-then-rename; no partial files on error) | P0 | active | ss-06/BC-1.06.008.md |

## Summary Statistics

| Subsystem | BC Count | Criticality | Primary Capabilities |
|-----------|----------|-------------|----------------------|
| SS-01 Policy | 9 | CRITICAL | CAP-001, CAP-002 |
| SS-02 Flow Ingest | 6 | HIGH | CAP-003, CAP-004 |
| SS-03 Zone Resolution | 5 | CRITICAL | CAP-005 |
| SS-04 Classification | 11 | CRITICAL | CAP-006, CAP-007, CAP-008 |
| SS-05 Aggregation | 5 | HIGH | CAP-009 |
| SS-06 Reporting | 8 | MEDIUM | CAP-010 |
| **Total** | **44** | | |

## Verification Property Count by Subsystem

| Subsystem | VP Count | Primary Proof Methods |
|-----------|----------|----------------------|
| SS-01 | 7 | kani, proptest, fuzz |
| SS-02 | 5 | kani, proptest, fuzz |
| SS-03 | 6 | kani |
| SS-04 | 11 | kani, proptest |
| SS-05 | 6 | kani, proptest |
| SS-06 | 8 | unit, integration |
| **Total** | **43** | |

## Invariant Coverage (all DI-NNN enforced by ≥1 BC)

| DI ID | Enforcing BC(s) |
|-------|-----------------|
| DI-001 | BC-1.04.002 |
| DI-002 | BC-1.04.001, BC-1.03.005 |
| DI-003 | BC-1.03.001, BC-1.03.002, BC-1.02.005 |
| DI-004 | BC-1.03.001 |
| DI-005 | BC-1.03.002 |
| DI-006 | BC-1.04.007, BC-1.04.008 |
| DI-007 | BC-1.04.004, BC-1.04.005 |
| DI-008 | BC-1.02.004, BC-1.06.003 |
| DI-009 | BC-1.05.002 |
| DI-010 | BC-1.01.003, BC-1.01.004, BC-1.01.005, BC-1.01.006, BC-1.01.007 |
| DI-011 | BC-1.01.002, BC-1.01.004 |
| DI-012 | BC-1.06.007 |
| DI-013 | BC-1.02.002, BC-1.02.003, BC-1.06.006 |
| DI-014 | BC-1.04.003 |
| DI-015 | BC-1.05.001, BC-1.04.010 |
| DI-016 | BC-1.03.003, BC-1.03.004, BC-1.04.011 |
| DI-017 | BC-1.04.009 |
| DI-018 | BC-1.05.003 |
| DI-019 | BC-1.06.005 |
| DI-020 | BC-1.01.009, BC-1.04.006 |
