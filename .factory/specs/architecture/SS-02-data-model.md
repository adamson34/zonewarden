---
document_type: architecture-section
level: L3
section: data-model
version: "1.0"
status: draft
producer: architect
timestamp: 2026-06-17T00:00:00
phase: 1b
inputs: [domain-spec/entities.md, domain-spec/invariants.md, specs/prd.md]
traces_to: ARCH-INDEX.md
---

# SS-02: Data Model & Resolution Index

## Core In-Memory Representations

All types live in `zonewarden-core`. No heap allocation beyond `Vec`/`String` unless noted.

### Policy Types

```rust
pub struct Policy {
    pub zones: Vec<Zone>,
    pub conduits: Vec<Conduit>,
}

pub struct Zone {
    pub id: ZoneId,                    // newtype String; interned
    pub name: String,
    pub purdue_level: PurdueLevel,
    pub sl_t: Option<SlTarget>,
    pub members: Vec<AssetMatcher>,    // sorted post-validation for binary search
}

pub enum AssetMatcher {
    Cidr(IpNet),   // covers IpAddr (stored as /32 or /128)
}

pub struct Conduit {
    pub from_zone: ZoneId,
    pub to_zone: ZoneId,
    pub direction: Direction,
    pub proto: Proto,
    pub ports: PortSet,
}

pub enum PortSet {
    Any,
    Ranges(Vec<PortRange>),   // sorted, non-overlapping, non-adjacent (DI-020)
}

pub struct PortRange { pub lo: u16, pub hi: u16 }

pub enum PurdueLevel { L0, L1, L2, L3, Idmz, L4, L5 }
pub enum Direction { Forward, Bidirectional }
pub enum Proto { Tcp, Udp, Icmp, Other(u8) }
```

### Flow Types

```rust
pub struct Flow {
    pub flow_index: u64,
    pub ts: Timestamp,            // nanosecond-precision UTC epoch
    pub src_ip: IpAddr,
    pub src_port: Option<u16>,
    pub dst_ip: IpAddr,
    pub dst_port: Option<u16>,
    pub proto: Proto,
    pub service: Option<Service>,
    pub service_source: ServiceSource,
    pub conn_state: Option<ConnState>,
}

pub enum ConnState { Established | Attempted | Other(String) }
pub enum ServiceSource { DpiConfirmed | PortHeuristic | Unknown }
pub enum Severity { Established | Attempted }
```

### Verdict / Aggregation Types

```rust
pub struct Verdict {
    pub flow_index: u64,
    pub src_zone: ZoneId,
    pub dst_zone: ZoneId,
    pub kind: VerdictKind,
    pub idmz_bypass: bool,
}

pub enum VerdictKind {
    IntraZone,
    Allowed { conduit_idx: usize },
    NoMatchingConduit,
    WrongDirection,
    MulticastExempt,
}

pub struct ConformanceResult {
    pub total_flows: u64,
    pub intra_zone: u64, pub allowed: u64, pub no_matching_conduit: u64,
    pub wrong_direction: u64, pub multicast_exempt: u64,
    pub idmz_bypasses: u64, pub distinct_violating_flows: u64,
    pub external_endpoints: u64,
    pub skipped: u64,
    pub warnings: Vec<String>,
    pub violations: Vec<Violation>,  // sorted by total-order key (DI-009)
    pub policy_digest: String,       // lowercase SHA-256 hex (DI-018)
}
```

## Zone Resolution Index — Design Choice

**Decision (ADR-003):** Sorted flat Vec of `(IpNet, ZoneId)` pairs, sorted descending
by prefix length. Resolution = linear scan returning first match.

**Options evaluated:**

| Option | Pros | Cons | Kani Feasibility |
|--------|------|------|-----------------|
| **Sorted prefix Vec (chosen)** | Simple; bounded search; easy Kani model | O(n) scan; n = total matchers | **Feasible** — bounded array, no pointer chasing |
| Radix/Patricia trie | O(prefix_len) lookup | Complex invariants; harder Kani proof | Needs extra effort; risky for proof |
| `iptrie` crate | Battle-tested | External dep; harder to prove | Depends on crate's internal invariants |

For typical OT policies (10–30 zones, 50–200 matchers) a sorted Vec scan is sub-microsecond
and the prefix-length ordering guarantees correct longest-prefix semantics trivially.

**Verification feasibility:** The sorted-Vec approach exposes the invariant as:
"for any IP, the first matching entry has the longest prefix length." This is a bounded
loop over a bounded array — precisely the input shape Kani handles well. VP-001 and VP-002
target this directly.

## Canonical Serialization — Policy Digest (DI-018)

**Decision (ADR-004):** Canonical JSON via `serde_json` with keys sorted by a custom
serializer, then SHA-256 via `sha2` crate.

**Algorithm:**
1. Sort `zones` by `id` (lexicographic byte-wise UTF-8).
2. For each zone, sort `members` by their CIDR string representation.
3. De-duplicate conduits (same `(from,to,proto,direction,ports)` tuple).
4. Sort conduits by `(from_zone, to_zone, proto_token, direction_token, ports_repr)`.
5. Normalize each `PortSet` to canonical form (DI-020) before serialization.
6. Omit `Option::None` fields entirely.
7. Serialize to UTF-8 JSON with no extraneous whitespace (`serde_json::to_string`).
8. SHA-256 → lowercase 64-char hex string.

**Verification feasibility:** VP-007 proves digest stability via proptest (two
structurally-equal policies serialize to the same digest). Kani is not the right tool
here (SHA-256 internals are too wide for bounded model checking); proptest with a
round-trip generator is the correct approach.

## `RealitySource` Trait Seam

```rust
pub trait RealitySource {
    fn flows(&mut self) -> impl Iterator<Item = Result<Flow, FlowParseError>>;
}
```

The trait lives in `zonewarden-core` (pure side) with no I/O in the trait definition
itself. Implementations (`ZeekAdapter`, future `NetFlowAdapter`) live in the binary crate.
This ensures the core pipeline can be tested with an in-memory `MockRealitySource`
without touching the filesystem.
