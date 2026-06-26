//! Integration tests for S-2.01 — Zeek conn.log parser + flow normalization
//! (BC-1.02.001 / 002 / 003 / 005).
//!
//! Test names intentionally use the upper-case `BC` contract id from the story
//! acceptance criteria.
#![allow(non_snake_case)]

use std::net::IpAddr;
use std::path::{Path, PathBuf};

use proptest::prelude::*;

use zonewarden::adapters::zeek::ZeekAdapter;
use zonewarden_core::errors::{FlowParseError, IngestError, IoError};
use zonewarden_core::types::{ConnState, Flow, Proto, Service, ServiceSource, Timestamp};
use zonewarden_core::RealitySource;

// ── helpers ──────────────────────────────────────────────────────────────────

/// A standard Zeek `#fields`/`#types` preamble (plus a couple of other comment
/// lines that must be skipped silently). Columns: ts, uid, id.orig_h, id.orig_p,
/// id.resp_h, id.resp_p, proto, service, conn_state.
const HEADER: &str = "#separator \\x09\n\
#path\tconn\n\
#fields\tts\tuid\tid.orig_h\tid.orig_p\tid.resp_h\tid.resp_p\tproto\tservice\tconn_state\n\
#types\ttime\tstring\taddr\tport\taddr\tport\tenum\tstring\tstring\n";

/// Build a conn.log string: the standard header followed by the given data
/// lines (each terminated by `\n`).
fn log(lines: &[&str]) -> String {
    let mut s = String::from(HEADER);
    for l in lines {
        s.push_str(l);
        s.push('\n');
    }
    s
}

/// A valid 9-column Zeek data line.
fn line(ts: &str, sip: &str, sp: &str, dip: &str, dp: &str, proto: &str, cs: &str) -> String {
    format!("{ts}\tC1\t{sip}\t{sp}\t{dip}\t{dp}\t{proto}\t-\t{cs}")
}

/// The drained outcome of running the adapter: produced flows plus the skip
/// signals (the consumer derives `skipped`/`warnings` from these).
struct Drained {
    flows: Vec<Flow>,
    errs: Vec<IngestError>,
}

impl Drained {
    fn skipped(&self) -> usize {
        self.errs.len()
    }
    fn warnings(&self) -> Vec<String> {
        self.errs.iter().map(|e| e.to_string()).collect()
    }
}

fn drain_str(s: &str) -> Drained {
    drain(ZeekAdapter::from_reader(s.as_bytes()))
}

fn drain<R: std::io::BufRead>(adapter: ZeekAdapter<R>) -> Drained {
    let mut flows = Vec::new();
    let mut errs = Vec::new();
    for item in adapter {
        match item {
            Ok(f) => flows.push(f),
            Err(e) => errs.push(e),
        }
    }
    Drained { flows, errs }
}

fn ip(s: &str) -> IpAddr {
    s.parse().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

// ── AC-001: valid line → Flow with all fields ───────────────────────────────

#[test]
fn test_BC_1_02_001_valid_line_produces_flow() {
    let d = drain_str(&log(&[&line(
        "1717200000.123456",
        "10.0.1.5",
        "1234",
        "10.0.2.10",
        "502",
        "tcp",
        "SF",
    )]));
    assert_eq!(d.skipped(), 0);
    assert_eq!(d.flows.len(), 1);

    let f = &d.flows[0];
    assert_eq!(f.flow_index, 0);
    assert_eq!(f.ts, Timestamp(1_717_200_000_123_456_000));
    assert_eq!(f.src_ip, ip("10.0.1.5"));
    assert_eq!(f.src_port, Some(1234));
    assert_eq!(f.dst_ip, ip("10.0.2.10"));
    assert_eq!(f.dst_port, Some(502));
    assert_eq!(f.proto, Proto::Tcp);
    // SF buckets to Established via the S-4.01 DI-017 single source.
    assert_eq!(f.conn_state, Some(ConnState::Established));
    // Service inference (BC-1.02.004, S-2.02): 502/tcp → Modbus, PortHeuristic.
    assert_eq!(f.service, Some(Service::Modbus));
    assert_eq!(f.service_source, ServiceSource::PortHeuristic);
}

#[test]
fn test_BC_1_02_001_port_and_conn_state_sentinels() {
    // EC-002 (conn_state `-` → None) and EC-004 (orig_p 0 → None).
    let d = drain_str(&log(&[&line(
        "1717200000.0",
        "10.0.1.5",
        "0",
        "10.0.2.10",
        "502",
        "udp",
        "-",
    )]));
    assert_eq!(d.flows.len(), 1);
    let f = &d.flows[0];
    assert_eq!(f.src_port, None);
    assert_eq!(f.conn_state, None);
    assert_eq!(f.proto, Proto::Udp);
}

// ── AC-002: flow_index dense + gap-free over successful flows only ───────────

proptest! {
    #[test]
    fn test_BC_1_02_001_flow_index_dense_gap_free(spec in prop::collection::vec(any::<bool>(), 0..40)) {
        // true → valid line, false → malformed line. flow_index must be the dense
        // sequence 0,1,2,... counting successful flows only (not malformed ones).
        let lines: Vec<String> = spec.iter().map(|&ok| {
            if ok {
                line("1717200000.0", "10.0.1.5", "1234", "10.0.2.10", "502", "tcp", "SF")
            } else {
                "totally bogus".to_string()
            }
        }).collect();
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let d = drain_str(&log(&refs));

        let valid = spec.iter().filter(|&&b| b).count();
        prop_assert_eq!(d.flows.len(), valid);
        prop_assert_eq!(d.skipped(), spec.len() - valid);
        for (i, f) in d.flows.iter().enumerate() {
            prop_assert_eq!(f.flow_index, i as u64);
        }
    }
}

// ── AC-003: `#` header/comment lines skipped silently (no skip count) ────────

#[test]
fn test_BC_1_02_001_header_lines_silently_skipped() {
    // Comment lines interspersed among data lines must not count as skipped and
    // must not perturb flow_index.
    let l0 = line(
        "1717200000.0",
        "10.0.1.5",
        "1",
        "10.0.2.10",
        "502",
        "tcp",
        "SF",
    );
    let l1 = line(
        "1717200001.0",
        "10.0.1.6",
        "2",
        "10.0.2.11",
        "502",
        "tcp",
        "SF",
    );
    let d = drain_str(&log(&[&l0, "#mid-stream comment", &l1]));
    assert_eq!(d.skipped(), 0);
    assert_eq!(d.flows.len(), 2);
    assert_eq!(d.flows[0].flow_index, 0);
    assert_eq!(d.flows[1].flow_index, 1);
}

// ── AC-004: malformed line skipped + counted; run continues ──────────────────

#[test]
fn test_BC_1_02_002_malformed_line_skipped_counted() {
    let l0 = line(
        "1717200000.0",
        "10.0.1.5",
        "1",
        "10.0.2.10",
        "502",
        "tcp",
        "SF",
    );
    let l1 = line(
        "1717200002.0",
        "10.0.1.7",
        "2",
        "10.0.2.12",
        "80",
        "tcp",
        "RSTO",
    );
    let d = drain_str(&log(&[&l0, "10.0.1.5\t1234", &l1]));
    assert_eq!(d.flows.len(), 2);
    assert_eq!(d.skipped(), 1);
    assert!(matches!(
        d.errs[0],
        IngestError::Parse(FlowParseError::Malformed { .. })
    ));
    // dense index continues across the skip
    assert_eq!(d.flows[0].flow_index, 0);
    assert_eq!(d.flows[1].flow_index, 1);
}

// ── AC-005: 100% malformed → zero flows, never aborts ────────────────────────

#[test]
fn test_BC_1_02_002_all_malformed_never_aborts() {
    let d = drain_str(&log(&["bad one", "bad\ttwo", "definitely not a flow"]));
    assert_eq!(d.flows.len(), 0);
    assert_eq!(d.skipped(), 3);
    // Draining completed without panic or a terminal error — the run survives.
}

// ── AC-006: unspecified address (0.0.0.0 / ::) skipped with a warning ────────

#[test]
fn test_BC_1_02_003_unspecified_src_skipped_with_warning() {
    let d = drain_str(&log(&[&line(
        "1717200000.0",
        "0.0.0.0",
        "1",
        "10.0.2.10",
        "502",
        "tcp",
        "SF",
    )]));
    assert_eq!(d.flows.len(), 0);
    assert_eq!(d.skipped(), 1);
    assert!(matches!(
        d.errs[0],
        IngestError::Parse(FlowParseError::UnspecifiedAddress { ref role, .. }) if role == "src"
    ));
    let w = &d.warnings()[0];
    assert!(w.contains("unspecified"), "warning was: {w}");
}

#[test]
fn test_BC_1_02_003_unspecified_dst_skipped_with_warning() {
    // IPv6 unspecified `::` as dst.
    let d = drain_str(&log(&[&line(
        "1717200000.0",
        "10.0.1.5",
        "1",
        "::",
        "502",
        "tcp",
        "SF",
    )]));
    assert_eq!(d.flows.len(), 0);
    assert_eq!(d.skipped(), 1);
    assert!(matches!(
        d.errs[0],
        IngestError::Parse(FlowParseError::UnspecifiedAddress { ref role, .. }) if role == "dst"
    ));
}

// ── AC-007: IPv4-mapped IPv6 canonicalized to IPv4 ───────────────────────────

#[test]
fn test_BC_1_02_005_ipv4_mapped_canonicalized() {
    let d = drain_str(&log(&[&line(
        "1717200000.0",
        "::ffff:10.0.1.5",
        "1",
        "::ffff:10.0.2.10",
        "502",
        "tcp",
        "SF",
    )]));
    assert_eq!(d.flows.len(), 1);
    let f = &d.flows[0];
    assert_eq!(f.src_ip, ip("10.0.1.5"));
    assert_eq!(f.dst_ip, ip("10.0.2.10"));
    assert!(f.src_ip.is_ipv4() && f.dst_ip.is_ipv4());
}

#[test]
fn test_BC_1_02_005_mapped_unspecified_canonicalized_then_skipped() {
    // EC-002 of BC-1.02.005 / EC-004 of BC-1.02.003: ::ffff:0.0.0.0 → 0.0.0.0 → skip.
    let d = drain_str(&log(&[&line(
        "1717200000.0",
        "::ffff:0.0.0.0",
        "1",
        "10.0.2.10",
        "502",
        "tcp",
        "SF",
    )]));
    assert_eq!(d.flows.len(), 0);
    assert!(matches!(
        d.errs[0],
        IngestError::Parse(FlowParseError::UnspecifiedAddress { .. })
    ));
}

#[test]
fn test_BC_1_02_005_nonmapped_ipv6_unchanged() {
    let d = drain_str(&log(&[&line(
        "1717200000.0",
        "2001:db8::1",
        "1",
        "2001:db8::2",
        "502",
        "tcp",
        "SF",
    )]));
    assert_eq!(d.flows.len(), 1);
    assert_eq!(d.flows[0].src_ip, ip("2001:db8::1"));
    assert!(d.flows[0].src_ip.is_ipv6());
}

// ── AC-008: CRLF line endings handled ────────────────────────────────────────

#[test]
fn test_BC_1_02_001_crlf_line_endings_handled() {
    // Build the same content but with Windows CRLF terminators.
    let body = log(&[&line(
        "1717200000.0",
        "10.0.1.5",
        "1234",
        "10.0.2.10",
        "502",
        "tcp",
        "SF",
    )]);
    let crlf = body.replace('\n', "\r\n");
    let d = drain_str(&crlf);
    assert_eq!(d.flows.len(), 1);
    let f = &d.flows[0];
    // The trailing \r must not corrupt the final field (conn_state).
    assert_eq!(f.conn_state, Some(ConnState::Established));
    assert_eq!(f.dst_port, Some(502));
}

// ── RealitySource trait surface ──────────────────────────────────────────────

#[test]
fn test_reality_source_trait_streams_flows() {
    let body = log(&[
        &line(
            "1717200000.0",
            "10.0.1.5",
            "1",
            "10.0.2.10",
            "502",
            "tcp",
            "SF",
        ),
        &line(
            "1717200001.0",
            "10.0.1.6",
            "2",
            "10.0.2.11",
            "502",
            "tcp",
            "SF",
        ),
    ]);
    let mut adapter = ZeekAdapter::from_reader(body.as_bytes());
    let ok = adapter.flows().filter(|r| r.is_ok()).count();
    assert_eq!(ok, 2);
}

// ── Fixture + IO tests ───────────────────────────────────────────────────────

#[test]
fn test_open_minimal_fixture() {
    let adapter = ZeekAdapter::open(&fixture("minimal.log")).expect("open minimal.log");
    let d = drain(adapter);
    assert_eq!(d.skipped(), 0);
    assert_eq!(d.flows.len(), 2);
    assert_eq!(d.flows[0].src_ip, ip("10.0.1.5"));
    assert_eq!(d.flows[1].src_port, None); // orig_p 0 → None
    assert_eq!(d.flows[1].proto, Proto::Udp);
}

#[test]
fn test_open_malformed_fixture_mixes_skips_and_flows() {
    let adapter = ZeekAdapter::open(&fixture("malformed.log")).expect("open malformed.log");
    let d = drain(adapter);
    // 2 valid (10.0.1.5, 10.0.1.7), 1 truncated, 1 unspecified (0.0.0.0).
    assert_eq!(d.flows.len(), 2);
    assert_eq!(d.skipped(), 2);
    assert_eq!(d.flows[0].flow_index, 0);
    assert_eq!(d.flows[1].flow_index, 1);
}

#[test]
fn test_open_missing_file_is_io_not_found() {
    match ZeekAdapter::open(Path::new("does/not/exist.log")) {
        Err(e) => assert!(matches!(e, IoError::NotFound { .. }), "got {e:?}"),
        Ok(_) => panic!("expected E-IO-001 NotFound for a missing file"),
    }
}

#[test]
fn test_empty_input_yields_nothing() {
    // EC-009: empty input → zero flows, zero skipped, no error.
    let d = drain_str("");
    assert_eq!(d.flows.len(), 0);
    assert_eq!(d.skipped(), 0);
}

// ── Property: accounting identity + panic-free parse ─────────────────────────

#[derive(Clone, Debug)]
enum LineKind {
    Valid,
    Malformed,
    Comment,
    Blank,
}

proptest! {
    #[test]
    fn test_BC_1_02_002_accounting_identity(
        kinds in prop::collection::vec(
            prop_oneof![
                Just(LineKind::Valid),
                Just(LineKind::Malformed),
                Just(LineKind::Comment),
                Just(LineKind::Blank),
            ],
            0..60,
        )
    ) {
        let lines: Vec<String> = kinds.iter().map(|k| match k {
            LineKind::Valid => line("1717200000.0", "10.0.1.5", "1", "10.0.2.10", "502", "tcp", "SF"),
            LineKind::Malformed => "garbage line".to_string(),
            LineKind::Comment => "#a comment".to_string(),
            LineKind::Blank => String::new(),
        }).collect();
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let d = drain_str(&log(&refs));

        let data_lines = kinds.iter().filter(|k| matches!(k, LineKind::Valid | LineKind::Malformed)).count();
        // total_flows + skipped == count of data (non-comment, non-blank) lines.
        prop_assert_eq!(d.flows.len() + d.skipped(), data_lines);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]
    #[test]
    fn test_parser_panic_free_on_arbitrary_input(raw in ".*") {
        // VP-1.02.001-c / VP-1.02.002-a: parsing arbitrary text must never panic.
        let _ = drain_str(&raw);
    }
}

// ── Gate-3 regression tests (Wave 2 adversarial review) ──────────────────────

// WAVE2-001: an out-of-range timestamp must be skipped as malformed, never
// panic (dev) or silently wrap into a bogus Flow (release). BC-1.02.002 / DI-013.
#[test]
fn test_BC_1_02_002_overflowing_timestamp_skipped_not_panic() {
    let huge = "999999999999999999999999999999"; // secs * 1e9 overflows i128
    let l = line(huge, "10.0.1.5", "1", "10.0.2.10", "502", "tcp", "SF");
    let valid = line(
        "1717200000.0",
        "10.0.1.6",
        "2",
        "10.0.2.11",
        "502",
        "tcp",
        "SF",
    );
    let d = drain_str(&log(&[&l, &valid]));
    assert_eq!(d.flows.len(), 1, "overflow line must not produce a flow");
    assert_eq!(d.skipped(), 1);
    assert!(matches!(
        d.errs[0],
        IngestError::Parse(FlowParseError::Malformed { .. })
    ));
    assert_eq!(d.flows[0].src_ip, ip("10.0.1.6"));
}

// WAVE2-002: a non-UTF-8 byte on one line must skip only that line and still
// parse the rest — never silently truncate the remainder. BC-1.02.002 / EC-003.
#[test]
fn test_BC_1_02_002_non_utf8_line_skipped_rest_parsed() {
    let mut bytes = HEADER.as_bytes().to_vec();
    bytes.extend_from_slice(&[0xFF, 0xFE, b'\n']); // garbage, invalid UTF-8
    bytes.extend_from_slice(
        line(
            "1717200000.0",
            "10.0.1.5",
            "1",
            "10.0.2.10",
            "502",
            "tcp",
            "SF",
        )
        .as_bytes(),
    );
    bytes.push(b'\n');
    let d = drain(ZeekAdapter::from_reader(bytes.as_slice()));
    assert_eq!(
        d.flows.len(),
        1,
        "valid line after a bad byte must still parse"
    );
    assert_eq!(
        d.skipped(),
        1,
        "bad-byte line is one skip, not a truncation"
    );
    assert_eq!(d.flows[0].src_ip, ip("10.0.1.5"));
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]
    // Panic-freedom over the real byte domain (VP-1.02.001-c is about arbitrary
    // *bytes*): a valid header followed by arbitrary bytes must never panic.
    #[test]
    fn test_parser_panic_free_on_arbitrary_bytes(data in prop::collection::vec(any::<u8>(), 0..512)) {
        let mut buf = HEADER.as_bytes().to_vec();
        buf.extend_from_slice(&data);
        let _ = drain(ZeekAdapter::from_reader(buf.as_slice()));
    }
}
