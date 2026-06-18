---
document_type: dependency-graph
level: ops
version: "1.0"
status: draft
producer: story-writer
timestamp: 2026-06-17T00:00:00
phase: 2
traces_to: STORY-INDEX.md
---

# Story Dependency Graph: zonewarden

## Explicit Dependencies

```
# Format: CHILD → PARENT (CHILD depends on PARENT)

S-1.02 → S-1.01    # policy::load needs portset + types
S-1.03 → S-1.02    # validator needs Policy type from load

S-2.01 → S-1.01    # zeek_adapter needs Flow type (defined in types.rs)
S-2.02 → S-2.01    # service inference + cap built on top of zeek parser

S-3.01 → S-1.03    # resolver needs ValidatedPolicy + PrefixIndex
S-3.02 → S-3.01    # multicast needs PrefixIndex for directed-broadcast check

S-4.01 → S-1.01    # severity::grade needs ConnState type (defined in types.rs)
S-4.02 → S-3.01    # idmz::check needs ResolvedEndpoint + PurdueLevel lookup
S-4.03 → S-3.02    # classifier needs multicast::classify_dst (DstKind)
S-4.03 → S-4.02    # classifier calls idmz::check
S-4.03 → S-4.01    # classifier calls severity::grade
S-4.04 → S-4.03    # MulticastExempt short-circuit added to classifier

S-5.01 → S-1.03    # digest needs ValidatedPolicy + Policy types
S-5.02 → S-4.04    # aggregator needs final Verdict type from classifier
S-5.02 → S-5.01    # aggregator calls digest::compute for policy_digest
S-5.03 → S-5.02    # sort step added to aggregator after identity established

S-6.01 → S-5.03    # reporter needs ConformanceResult (with sorted violations)
S-6.02 → S-6.01    # atomic write + warnings added to reporter
S-6.03 → S-6.02    # CLI wires reporter (needs atomic write)
S-6.03 → S-2.02    # CLI wires zeek_adapter (needs service inference + cap)
```

## Independent Groups Per Wave

**Wave 1 (no dependencies):**
- `[S-1.01]` — workspace + portset

**Wave 2 (depend on Wave 1 only):**
- `[S-1.02]` — policy YAML load (depends on S-1.01)
- `[S-2.01]` — zeek parser (depends on S-1.01)
- `[S-4.01]` — severity grading (depends on S-1.01)

**Wave 3 (depend on Wave 2 only):**
- `[S-1.03]` — policy validation (depends on S-1.02)
- `[S-2.02]` — service inference + cap (depends on S-2.01)

**Wave 4 (depend on Wave 3 only):**
- `[S-3.01]` — zone resolver (depends on S-1.03)
- `[S-5.01]` — policy digest (depends on S-1.03)

**Wave 5 (depend on Wave 4):**
- `[S-3.02]` — multicast detection (depends on S-3.01)
- `[S-4.02]` — IDMZ truth table (depends on S-3.01)

**Wave 6 (depend on Wave 5):**
- `[S-4.03]` — classifier core (depends on S-3.02 + S-4.02 + S-4.01)

**Wave 7 (depend on Wave 6):**
- `[S-4.04]` — MulticastExempt + totality (depends on S-4.03)

**Wave 8 (depend on Wave 7):**
- `[S-5.02]` — aggregator identity (depends on S-4.04 + S-5.01)

**Wave 9 (depend on Wave 8):**
- `[S-5.03]` — aggregator sort (depends on S-5.02)

**Wave 10 (depend on Wave 9):**
- `[S-6.01]` — reporter formatters (depends on S-5.03)

**Wave 11 (depend on Wave 10):**
- `[S-6.02]` — atomic write + warnings (depends on S-6.01)
- `[S-6.03]` — CLI integration (depends on S-6.02 + S-2.02) ← S-2.02 complete by Wave 3; no blocking

> **Note on collapsed waves:** For scheduling purposes, consecutive waves with only a single
> story may be collapsed into combined waves to reduce scheduling overhead. The schedule in
> `wave-schedule.md` uses 5 consolidated waves. The graph above shows the full dependency
> resolution order for acyclicity verification.

## Acyclicity Verification

**Method:** Topological sort via Kahn's algorithm (BFS from nodes with in-degree 0).

**Adjacency list (CHILD → PARENT edge = CHILD depends on PARENT):**

```
S-1.01: in-degree 0 (root)
S-1.02: in-degree 1 (S-1.01)
S-2.01: in-degree 1 (S-1.01)
S-4.01: in-degree 1 (S-1.01)
S-1.03: in-degree 1 (S-1.02)
S-2.02: in-degree 1 (S-2.01)
S-3.01: in-degree 1 (S-1.03)
S-5.01: in-degree 1 (S-1.03)
S-3.02: in-degree 1 (S-3.01)
S-4.02: in-degree 1 (S-3.01)
S-4.03: in-degree 3 (S-3.02, S-4.02, S-4.01)
S-4.04: in-degree 1 (S-4.03)
S-5.02: in-degree 2 (S-4.04, S-5.01)
S-5.03: in-degree 1 (S-5.02)
S-6.01: in-degree 1 (S-5.03)
S-6.02: in-degree 1 (S-6.01)
S-6.03: in-degree 2 (S-6.02, S-2.02)
```

**Topological order (one valid order):**
`S-1.01 → S-1.02, S-2.01, S-4.01 → S-1.03, S-2.02 → S-3.01, S-5.01 → S-3.02, S-4.02 → S-4.03 → S-4.04 → S-5.02 → S-5.03 → S-6.01 → S-6.02 → S-6.03`

**Result: ACYCLIC** ✓ — Kahn's algorithm processes all 17 stories with zero remaining edges; no cycle detected.

## BC to Stories Traceability Matrix

| BC ID | Title (short) | Stories | Full Coverage? |
|-------|--------------|---------|---------------|
| BC-1.01.001 | Parse valid YAML policy | S-1.02 | Yes |
| BC-1.01.002 | Reject malformed YAML | S-1.02 | Yes |
| BC-1.01.003 | Reject duplicate YAML keys | S-1.02 | Yes |
| BC-1.01.004 | Validate zone IDs unique + conduit endpoints | S-1.03 | Yes |
| BC-1.01.005 | Reject equal-prefix-length ties | S-1.03 | Yes |
| BC-1.01.006 | Reject /0 catch-all members | S-1.03 | Yes |
| BC-1.01.007 | Reject unrecognized direction/proto tokens | S-1.03 | Yes |
| BC-1.01.008 | Warn on zero-member zone | S-1.03 | Yes |
| BC-1.01.009 | PortSet canonical form | S-1.01 | Yes |
| BC-1.02.001 | Parse valid Zeek conn.log line | S-2.01 | Yes |
| BC-1.02.002 | Skip malformed flow lines | S-2.01 | Yes |
| BC-1.02.003 | Skip unspecified-address flows + warn | S-2.01 | Yes |
| BC-1.02.004 | Service inference via canonical table | S-2.02 | Yes |
| BC-1.02.005 | Canonicalize IPv4-mapped IPv6 | S-2.01 | Yes |
| BC-1.02.006 | Enforce max_flows ingest cap | S-2.02 | Yes |
| BC-1.03.001 | Longest-prefix zone resolution | S-3.01 | Yes |
| BC-1.03.002 | Unmatched endpoint → EXTERNAL | S-3.01 | Yes |
| BC-1.03.003 | Multicast/broadcast dst short-circuit | S-3.02 | Yes |
| BC-1.03.004 | Directed-broadcast override ≤/30 | S-3.02 | Yes |
| BC-1.03.005 | Both-EXTERNAL → IntraZone | S-3.01 | Yes |
| BC-1.04.001 | IntraZone flows allowed without conduit eval | S-4.03 | Yes |
| BC-1.04.002 | Deny-by-default: no match → NoMatchingConduit | S-4.03 | Yes |
| BC-1.04.003 | Any-match conduit union | S-4.03 | Yes |
| BC-1.04.004 | Forward conduit directionality | S-4.03 | Yes |
| BC-1.04.005 | Bidirectional conduit | S-4.03 | Yes |
| BC-1.04.006 | Portless protocol matches only ports:Any | S-4.03 | Yes |
| BC-1.04.007 | IDMZ no-bypass truth table | S-4.02 | Yes |
| BC-1.04.008 | IDMZ bypass NOT raised for EXTERNAL/Multicast | S-4.02 | Yes |
| BC-1.04.009 | Violation severity from conn_state | S-4.01, S-4.03 | Yes |
| BC-1.04.010 | Verdict totality | S-4.04 | Yes |
| BC-1.04.011 | MulticastExempt short-circuits first | S-4.04 | Yes |
| BC-1.05.001 | DI-015 accounting identity | S-5.02 | Yes |
| BC-1.05.002 | Output ordered by total-order key | S-5.03 | Yes |
| BC-1.05.003 | Stable policy digest SHA-256 | S-5.01 | Yes |
| BC-1.05.004 | u64 overflow abort | S-5.02 | Yes |
| BC-1.05.005 | Empty input → all-zero result | S-5.02 | Yes |
| BC-1.06.001 | Exit codes 0/1/2 | S-6.03 | Yes |
| BC-1.06.002 | JSON violations report | S-6.01 | Yes |
| BC-1.06.003 | Text violations report | S-6.01 | Yes |
| BC-1.06.004 | Mermaid zone/conduit diagram | S-6.01 | Yes |
| BC-1.06.005 | Warnings to stderr deterministic order | S-6.02 | Yes |
| BC-1.06.006 | --fail-on-skipped flag | S-6.03 | Yes |
| BC-1.06.007 | No network socket; offline invariant | S-6.02, S-6.03 | Yes |
| BC-1.06.008 | Atomic write (temp+rename) | S-6.02 | Yes |

**BC Coverage: 44/44 — COMPLETE. No gaps.**

## BC Clause Coverage Matrix

| BC-S.SS.NNN | Clause | Type | Covering AC | Story |
|-------------|--------|------|-------------|-------|
| BC-1.01.009 | Postcondition 1 | postcondition | AC-001 | S-1.01 |
| BC-1.01.009 | Postcondition 2 | postcondition | AC-002 | S-1.01 |
| BC-1.01.009 | Postcondition 3 | postcondition | AC-003 | S-1.01 |
| BC-1.01.009 | Postcondition 4 | postcondition | AC-006 | S-1.01 |
| BC-1.01.009 | Invariant 2 | invariant | AC-004 | S-1.01 |
| BC-1.01.009 | Invariant 4 | invariant | AC-005 | S-1.01 |
| BC-1.01.001 | Postcondition 1 | postcondition | AC-001 | S-1.02 |
| BC-1.01.001 | Postcondition 2 | postcondition | AC-002 | S-1.02 |
| BC-1.01.001 | Postcondition 3 | postcondition | AC-003 | S-1.02 |
| BC-1.01.001 | Postcondition 4 | postcondition | AC-004 | S-1.02 |
| BC-1.01.001 | Invariant 1 | invariant | AC-008 | S-1.02 |
| BC-1.01.001 | Invariant 2 | invariant | AC-009 | S-1.02 |
| BC-1.01.002 | error path | postcondition | AC-006 | S-1.02 |
| BC-1.01.003 | dup-key rejection | postcondition | AC-007 | S-1.02 |
| BC-1.01.004 | Postcondition 1 | postcondition | AC-001 | S-1.03 |
| BC-1.01.004 | Postcondition 2 | postcondition | AC-002 | S-1.03 |
| BC-1.01.005 | tie rejection | postcondition | AC-003 | S-1.03 |
| BC-1.01.006 | /0 rejection | postcondition | AC-004 | S-1.03 |
| BC-1.01.007 | token rejection | postcondition | AC-005 | S-1.03 |
| BC-1.01.008 | warn not error | postcondition | AC-006 | S-1.03 |
| BC-1.02.001 | Postcondition 1 | postcondition | AC-001 | S-2.01 |
| BC-1.02.001 | Invariant 4 | invariant | AC-002 | S-2.01 |
| BC-1.02.002 | skip+count | postcondition | AC-004 | S-2.01 |
| BC-1.02.002 | never abort | invariant | AC-005 | S-2.01 |
| BC-1.02.003 | skip+warn | postcondition | AC-006 | S-2.01 |
| BC-1.02.005 | IPv4-mapped canonicalize | postcondition | AC-007 | S-2.01 |
| BC-1.02.004 | service inference | postcondition 1 | AC-001..003 | S-2.02 |
| BC-1.02.004 | unknown port | postcondition 2 | AC-004 | S-2.02 |
| BC-1.02.004 | always set | invariant | AC-005 | S-2.02 |
| BC-1.02.006 | cap breach abort | postcondition | AC-006 | S-2.02 |
| BC-1.02.006 | no partial output | — | AC-008 | S-2.02 |
| BC-1.03.001 | Postcondition 1 | postcondition | AC-001 | S-3.01 |
| BC-1.03.001 | Postcondition 2 | postcondition | AC-002 | S-3.01 |
| BC-1.03.001 | Invariant 1 (totality) | invariant | AC-006 | S-3.01 |
| BC-1.03.001 | Invariant 3 (pure) | invariant | AC-007 | S-3.01 |
| BC-1.03.002 | EXTERNAL fallback | postcondition | AC-004 | S-3.01 |
| BC-1.03.005 | both-EXTERNAL → IntraZone | postcondition | AC-005 | S-3.01 |
| BC-1.03.003 | multicast short-circuit | postcondition | AC-001..003 | S-3.02 |
| BC-1.03.004 | directed-broadcast ≤/30 | postcondition | AC-004 | S-3.02 |
| BC-1.03.004 | /31+/32 exclusion | edge case | AC-005 | S-3.02 |
| BC-1.04.007 | OT-IT bypass | postcondition 1 | AC-001 | S-4.02 |
| BC-1.04.007 | additive | postcondition 4 | AC-002 | S-4.02 |
| BC-1.04.007 | IDMZ endpoint no-bypass | invariant 5 | AC-005 | S-4.02 |
| BC-1.04.008 | EXTERNAL exclusion | postcondition | AC-003 | S-4.02 |
| BC-1.04.008 | Multicast exclusion | postcondition | AC-004 | S-4.02 |
| BC-1.04.009 | Established mapping | postcondition 1 | AC-001 | S-4.01 |
| BC-1.04.009 | Attempted mapping | postcondition 1 | AC-002 | S-4.01 |
| BC-1.04.009 | None → Established | postcondition 2 | AC-003 | S-4.01 |
| BC-1.04.001 | IntraZone | postcondition | AC-001 | S-4.03 |
| BC-1.04.002 | deny-by-default | postcondition | AC-002 | S-4.03 |
| BC-1.04.003 | any-match | postcondition | AC-003 | S-4.03 |
| BC-1.04.004 | forward directionality | postcondition | AC-004 | S-4.03 |
| BC-1.04.005 | bidirectional | postcondition | AC-005 | S-4.03 |
| BC-1.04.006 | portless | postcondition | AC-006 | S-4.03 |
| BC-1.04.011 | MulticastExempt first | postcondition | AC-001 | S-4.04 |
| BC-1.04.011 | before IntraZone | invariant | AC-002 | S-4.04 |
| BC-1.04.011 | no idmz bypass | — | AC-004 | S-4.04 |
| BC-1.04.010 | totality | postcondition | AC-003 | S-4.04 |
| BC-1.05.003 | stable digest | postcondition 1 | AC-001 | S-5.01 |
| BC-1.05.003 | canonical order | postcondition 2 | AC-002 | S-5.01 |
| BC-1.05.003 | None omitted | postcondition 5 | AC-005 | S-5.01 |
| BC-1.05.001 | DI-015 identity | postcondition 1 | AC-001 | S-5.02 |
| BC-1.05.001 | idmz_bypasses ≤ total | postcondition 2 | AC-002 | S-5.02 |
| BC-1.05.001 | distinct_violating ≤ total | postcondition 3 | AC-003 | S-5.02 |
| BC-1.05.001 | skipped excluded | postcondition 4 | AC-004 | S-5.02 |
| BC-1.05.004 | overflow abort | postcondition | AC-006 | S-5.02 |
| BC-1.05.005 | empty input all-zero | postcondition | AC-007 | S-5.02 |
| BC-1.05.002 | total-order sort | postcondition | AC-001 | S-5.03 |
| BC-1.05.002 | flow_index tiebreaker | postcondition | AC-002 | S-5.03 |
| BC-1.06.002 | JSON schema | postcondition | AC-001 | S-6.01 |
| BC-1.06.002 | violation fields | postcondition | AC-002 | S-6.01 |
| BC-1.06.003 | text report | postcondition | AC-004 | S-6.01 |
| BC-1.06.003 | service_source in text | postcondition | AC-005 | S-6.01 |
| BC-1.06.004 | Mermaid diagram | postcondition | AC-006 | S-6.01 |
| BC-1.06.004 | violations highlighted | postcondition | AC-007 | S-6.01 |
| BC-1.06.008 | atomic write | postcondition | AC-001 | S-6.02 |
| BC-1.06.008 | no partial file | postcondition | AC-002 | S-6.02 |
| BC-1.06.005 | warnings to stderr | postcondition | AC-003 | S-6.02 |
| BC-1.06.005 | deterministic order | postcondition | AC-004 | S-6.02 |
| BC-1.06.005 | no exit code change | invariant | AC-005 | S-6.02 |
| BC-1.06.007 | no network socket | postcondition | AC-006 | S-6.02 |
| BC-1.06.001 | exit 0 | postcondition 1 | AC-001 | S-6.03 |
| BC-1.06.001 | exit 1 | postcondition 2 | AC-002 | S-6.03 |
| BC-1.06.001 | exit 2 | postcondition 3 | AC-003 | S-6.03 |
| BC-1.06.006 | --fail-on-skipped | postcondition | AC-005 | S-6.03 |

## Gap Register

No gaps. All 44 BC clauses are covered by at least one AC in at least one story.
