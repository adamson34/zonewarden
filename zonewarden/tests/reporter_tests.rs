//! Integration tests for S-6.01 — reporter formatters (BC-1.06.002/003/004).
//!
//! JSON is parsed and asserted structurally; text/Mermaid use structural +
//! determinism assertions rather than brittle full-string goldens.
//!
//! Test names intentionally use the upper-case `BC` contract id.
#![allow(non_snake_case)]

use zonewarden::reporter::{emit_json, emit_mermaid, emit_text};
use zonewarden_core::types::{
    AssetMatcher, Conduit, ConformanceResult, ConnState, Direction, Proto, PurdueLevel, Service,
    ServiceSource, Severity, Timestamp, ValidatedPolicy, Violation, ViolationKind, Zone, ZoneId,
};
use zonewarden_core::validator::validate;

// ── fixtures ─────────────────────────────────────────────────────────────────

fn zone(id: &str, level: PurdueLevel, cidr: &str) -> Zone {
    let (addr, plen) = cidr.split_once('/').unwrap();
    Zone {
        id: ZoneId(id.to_string()),
        name: format!("{id} zone"),
        purdue_level: level,
        sl_t: None,
        members: vec![AssetMatcher::Cidr {
            addr: addr.parse().unwrap(),
            prefix_len: plen.parse().unwrap(),
        }],
    }
}

fn vp() -> ValidatedPolicy {
    validate(zonewarden_core::types::Policy {
        zones: vec![
            zone("plc", PurdueLevel::L1, "10.0.1.0/24"),
            zone("hist", PurdueLevel::L3, "10.0.3.0/24"),
        ],
        conduits: vec![Conduit {
            from_zone: ZoneId("plc".to_string()),
            to_zone: ZoneId("hist".to_string()),
            direction: Direction::Forward,
            proto: Proto::Tcp,
            ports: zonewarden_core::portset::PortSet::from_pairs(&[(502, 502)]).unwrap(),
        }],
    })
    .expect("valid policy")
}

/// A ConformanceResult with one NoMatchingConduit violation (plc -> hist), a
/// PortHeuristic service, and a real policy digest.
fn result(p: &ValidatedPolicy) -> ConformanceResult {
    ConformanceResult {
        total_flows: 3,
        intra_zone: 1,
        allowed: 1,
        no_matching_conduit: 1,
        wrong_direction: 0,
        multicast_exempt: 0,
        idmz_bypasses: 0,
        distinct_violating_flows: 1,
        external_endpoints: 0,
        skipped: 0,
        warnings: vec![],
        policy_digest: zonewarden_core::digest::compute(&p.policy),
        violations: vec![Violation {
            flow_index: 2,
            src_zone: ZoneId("plc".to_string()),
            dst_zone: ZoneId("hist".to_string()),
            kind: ViolationKind::NoMatchingConduit,
            severity: Severity::Established,
            idmz_bypass: false,
            explanation: "no conduit permits flow plc -> hist".to_string(),
            ts: Timestamp(1_717_200_000_000_000_000),
            src_ip: "10.0.1.5".parse().unwrap(),
            dst_ip: "10.0.3.9".parse().unwrap(),
            src_port: Some(40000),
            dst_port: Some(9999),
            proto: Proto::Tcp,
            service: Some(Service::Other("HTTP".to_string())),
            service_source: ServiceSource::PortHeuristic,
            conn_state: Some(ConnState::Established),
        }],
    }
}

fn json_value(r: &ConformanceResult) -> serde_json::Value {
    let mut buf = Vec::new();
    emit_json(r, &mut buf).expect("emit_json");
    serde_json::from_slice(&buf).expect("valid JSON")
}

fn text_of(r: &ConformanceResult) -> String {
    let mut buf = Vec::new();
    emit_text(r, &mut buf).expect("emit_text");
    String::from_utf8(buf).unwrap()
}

fn mermaid_of(r: &ConformanceResult, p: &ValidatedPolicy) -> String {
    let mut buf = Vec::new();
    emit_mermaid(r, p, &mut buf).expect("emit_mermaid");
    String::from_utf8(buf).unwrap()
}

// ── AC-001/003/009: JSON schema top-level ────────────────────────────────────

#[test]
fn test_BC_1_06_002_json_schema_required_fields_present() {
    let p = vp();
    let j = json_value(&result(&p));
    for field in [
        "schema_version",
        "policy_digest",
        "total_flows",
        "intra_zone",
        "allowed",
        "no_matching_conduit",
        "wrong_direction",
        "multicast_exempt",
        "idmz_bypasses",
        "distinct_violating_flows",
        "external_endpoints",
        "skipped",
        "warnings",
        "violations",
    ] {
        assert!(j.get(field).is_some(), "missing JSON field: {field}");
    }
    assert_eq!(j["total_flows"], 3);
    assert!(j["warnings"].is_array());
    assert!(j["violations"].is_array());
}

#[test]
fn test_BC_1_06_002_schema_version_is_1_0() {
    let p = vp();
    assert_eq!(json_value(&result(&p))["schema_version"], "1.0");
}

#[test]
fn test_BC_1_06_002_policy_digest_format_valid() {
    let p = vp();
    let d = json_value(&result(&p))["policy_digest"]
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(d.len(), 64);
    assert!(d
        .chars()
        .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c)));
}

// ── AC-002: violation required fields ────────────────────────────────────────

#[test]
fn test_BC_1_06_002_violation_required_fields_present() {
    let p = vp();
    let j = json_value(&result(&p));
    let v = &j["violations"][0];
    for field in [
        "flow_index",
        "src_zone",
        "dst_zone",
        "kind",
        "severity",
        "explanation",
        "src_ip",
        "dst_ip",
        "proto",
        "service_source",
    ] {
        assert!(v.get(field).is_some(), "violation missing field: {field}");
    }
    assert_eq!(v["kind"], "NoMatchingConduit");
    assert_eq!(v["severity"], "Established");
    assert_eq!(v["service_source"], "PortHeuristic");
    assert_eq!(v["src_ip"], "10.0.1.5");
    assert_eq!(v["service"], "HTTP");
}

#[test]
fn test_BC_1_06_002_service_omitted_when_none() {
    // EC-003: a violation with service: None omits the JSON "service" key.
    let p = vp();
    let mut r = result(&p);
    r.violations[0].service = None;
    let j = json_value(&r);
    assert!(j["violations"][0].get("service").is_none());
}

// ── AC-004/005: text report ──────────────────────────────────────────────────

#[test]
fn test_BC_1_06_003_text_report_includes_summary_and_violations() {
    let p = vp();
    let t = text_of(&result(&p));
    assert!(t.contains("Summary"), "text: {t}");
    assert!(t.contains('3'), "summary should mention flow count");
    assert!(t.contains("plc") && t.contains("hist"), "violation zones");
    assert!(t.contains("NoMatchingConduit"));
    assert!(t.contains("Established"));
}

#[test]
fn test_BC_1_06_003_text_report_flags_heuristic_service_source() {
    let p = vp();
    let t = text_of(&result(&p));
    // DI-008: heuristic provenance is visibly flagged.
    assert!(
        t.to_lowercase().contains("heuristic"),
        "PortHeuristic must be flagged in text: {t}"
    );
}

#[test]
fn test_BC_1_06_003_text_no_violations_case() {
    // EC-001: zero violations → "No violations".
    let p = vp();
    let mut r = result(&p);
    r.violations.clear();
    r.no_matching_conduit = 0;
    r.distinct_violating_flows = 0;
    let t = text_of(&r);
    assert!(t.to_lowercase().contains("no violations"), "text: {t}");
}

// ── AC-006/007/008: Mermaid ──────────────────────────────────────────────────

#[test]
fn test_BC_1_06_004_mermaid_has_zone_nodes_and_conduit_edges() {
    let p = vp();
    let m = mermaid_of(&result(&p), &p);
    assert!(m.starts_with("graph LR"), "mermaid: {m}");
    // zone nodes labeled with id + purdue level
    assert!(m.contains("hist[") && m.contains("plc["));
    assert!(m.contains("(L1)") && m.contains("(L3)"));
    // conduit edge plc -> hist
    assert!(m.contains("plc --> hist"));
    assert!(m.contains("classDef violation"));
}

#[test]
fn test_BC_1_06_004_violated_zones_highlighted() {
    // AC-007: zones in a violation get the :::violation class.
    let p = vp();
    let m = mermaid_of(&result(&p), &p);
    assert!(m.contains("plc:::violation") || m.contains("plc[") && m.contains(":::violation"));
    // a violated zone line carries the class
    assert!(m
        .lines()
        .any(|l| l.contains("plc") && l.contains(":::violation")));
    assert!(m
        .lines()
        .any(|l| l.contains("hist") && l.contains(":::violation")));
}

#[test]
fn test_BC_1_06_004_mermaid_nodes_sorted_deterministic() {
    let p = vp();
    let m1 = mermaid_of(&result(&p), &p);
    let m2 = mermaid_of(&result(&p), &p);
    assert_eq!(m1, m2, "mermaid must be deterministic");
    // nodes sorted by id: hist before plc
    let hist = m1.find("hist[").unwrap();
    let plc = m1.find("plc[").unwrap();
    assert!(
        hist < plc,
        "zone nodes must be sorted by id (hist before plc)"
    );
}

#[test]
fn test_BC_1_06_004_no_violations_no_highlight() {
    // EC-001: zero violations → no :::violation in Mermaid.
    let p = vp();
    let mut r = result(&p);
    r.violations.clear();
    let m = mermaid_of(&r, &p);
    assert!(
        !m.contains(":::violation"),
        "no zones should be highlighted: {m}"
    );
}
