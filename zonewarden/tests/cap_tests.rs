//! Integration tests for S-2.02 — ingest cap (BC-1.02.006).
//!
//! Test names intentionally use the upper-case `BC` contract id.
#![allow(non_snake_case)]

use zonewarden::adapters::zeek::{validate_max_flows, ZeekAdapter};
use zonewarden_core::errors::{IngestError, SysError, ZonewardenError};
use zonewarden_core::types::Flow;

const HEADER: &str =
    "#fields\tts\tuid\tid.orig_h\tid.orig_p\tid.resp_h\tid.resp_p\tproto\tservice\tconn_state\n";

/// Build a conn.log with `n` distinct valid flow lines.
fn log_with(n: usize) -> String {
    let mut s = String::from(HEADER);
    for i in 0..n {
        s.push_str(&format!(
            "1717200000.0\tC{i}\t10.0.1.5\t{p}\t10.0.2.10\t502\ttcp\t-\tSF\n",
            p = 1000 + i
        ));
    }
    s
}

fn drain(input: &str, max: u64) -> (Vec<Flow>, Vec<IngestError>) {
    let adapter = ZeekAdapter::from_reader(input.as_bytes()).with_max_flows(max);
    let mut flows = Vec::new();
    let mut errs = Vec::new();
    for item in adapter {
        match item {
            Ok(f) => flows.push(f),
            Err(e) => errs.push(e),
        }
    }
    (flows, errs)
}

// ── AC-006: cap breach → CapExceeded; abort ──────────────────────────────────

#[test]
fn test_BC_1_02_006_cap_breach_aborts_with_exit_2() {
    // EC-006: max_flows = 1, two valid lines → breach on the 2nd flow.
    let (flows, errs) = drain(&log_with(2), 1);
    assert_eq!(flows.len(), 1, "exactly max_flows flows are emitted");
    assert_eq!(errs.len(), 1, "one fatal cap error");
    match &errs[0] {
        IngestError::Sys(s @ SysError::CapExceeded { max, count }) => {
            assert_eq!(*max, 1);
            assert_eq!(*count, 1);
            // A cap breach maps to exit 2 (BC-1.02.006).
            assert_eq!(ZonewardenError::from(s.clone()).exit_code(), 2);
        }
        other => panic!("expected E-SYS-001 CapExceeded, got {other:?}"),
    }
}

// ── EC-002: exactly max_flows flows → no breach (strict `>`) ──────────────────

#[test]
fn test_BC_1_02_006_exactly_max_flows_is_ok() {
    let (flows, errs) = drain(&log_with(2), 2);
    assert_eq!(flows.len(), 2);
    assert!(
        errs.is_empty(),
        "exactly max flows must not breach: {errs:?}"
    );
}

// ── AC-007: --max-flows 0 is a usage error (E-SYS-002) ───────────────────────

#[test]
fn test_BC_1_02_006_zero_max_flows_is_usage_error() {
    assert_eq!(validate_max_flows(0), Err(SysError::ZeroMaxFlows));
    assert_eq!(validate_max_flows(1), Ok(1));
    assert_eq!(validate_max_flows(1_000_000), Ok(1_000_000));
}

// ── AC-008: cap breach yields nothing after the error (no partial stream) ─────

#[test]
fn test_BC_1_02_006_no_records_after_cap_breach() {
    // 5 valid lines, cap 2: emit 2 flows, one cap error, then the stream is done —
    // no further flows leak out (the basis for "no partial report", finished in S-6.03).
    let input = log_with(5);
    let adapter = ZeekAdapter::from_reader(input.as_bytes()).with_max_flows(2);
    let items: Vec<_> = adapter.collect();
    assert_eq!(items.len(), 3, "2 flows + 1 cap error, nothing after");
    assert!(items[0].is_ok());
    assert!(items[1].is_ok());
    assert!(matches!(
        items[2],
        Err(IngestError::Sys(SysError::CapExceeded { .. }))
    ));
}

// ── EC-004: many skips, few valid flows → cap counts valid flows only ─────────

#[test]
fn test_BC_1_02_006_cap_counts_valid_flows_only() {
    // Interleave malformed lines among 2 valid flows; cap 2 must not breach
    // because skips don't consume cap quota.
    let mut s = String::from(HEADER);
    s.push_str("garbage\n");
    s.push_str("1717200000.0\tC0\t10.0.1.5\t1000\t10.0.2.10\t502\ttcp\t-\tSF\n");
    s.push_str("also garbage\n");
    s.push_str("1717200000.0\tC1\t10.0.1.6\t1001\t10.0.2.11\t502\ttcp\t-\tSF\n");
    let (flows, errs) = drain(&s, 2);
    assert_eq!(flows.len(), 2);
    // the two errors are the malformed skips, not a cap breach
    assert_eq!(errs.len(), 2);
    assert!(errs.iter().all(|e| matches!(e, IngestError::Parse(_))));
}
