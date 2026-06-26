//! Integration tests for S-6.02 — atomic write + deterministic warnings
//! (BC-1.06.005/007/008).
//!
//! Test names intentionally use the upper-case `BC` contract id.
#![allow(non_snake_case)]

use std::io;
use std::path::PathBuf;

use zonewarden::reporter::{emit_to_file, emit_warnings};
use zonewarden_core::errors::IoError;

/// A unique path in the temp dir (not created yet). Caller cleans up.
fn temp_target(tag: &str) -> PathBuf {
    let name = format!("zw_atomic_{}_{}_{tag}.out", std::process::id(), tag.len());
    std::env::temp_dir().join(name)
}

/// Whether any temp sidecar (`.{name}.tmp.*`) for `target` is left behind.
fn temp_leftover_exists(target: &std::path::Path) -> bool {
    let prefix = format!(".{}.tmp.", target.file_name().unwrap().to_string_lossy());
    let dir = target.parent().unwrap();
    std::fs::read_dir(dir)
        .map(|rd| {
            rd.flatten()
                .any(|e| e.file_name().to_string_lossy().starts_with(&prefix))
        })
        .unwrap_or(false)
}

// ── AC-001: atomic write via temp-then-rename ────────────────────────────────

#[test]
fn test_BC_1_06_008_atomic_write_uses_temp_then_rename() {
    let target = temp_target("ok");
    let _ = std::fs::remove_file(&target);

    emit_to_file(&target, |w| w.write_all(b"hello zonewarden")).expect("write ok");

    assert_eq!(std::fs::read(&target).unwrap(), b"hello zonewarden");
    // the temp sidecar must be gone after the rename
    assert!(
        !temp_leftover_exists(&target),
        "temp file should be renamed away"
    );

    let _ = std::fs::remove_file(&target);
}

// ── AC-002: no partial file (or temp) left on write error ────────────────────

#[test]
fn test_BC_1_06_008_no_partial_file_on_write_error() {
    let target = temp_target("err");
    let _ = std::fs::remove_file(&target);

    let result = emit_to_file(&target, |w| {
        w.write_all(b"partial...")?; // some bytes written to temp
        Err(io::Error::other("simulated mid-stream failure"))
    });

    assert!(matches!(result, Err(IoError::OutputWrite { .. })));
    assert!(
        !target.exists(),
        "target must not exist after a write error"
    );
    assert!(
        !temp_leftover_exists(&target),
        "temp file must be cleaned up"
    );
}

// ── AC-001 EC-001: temp creation failure in a nonexistent directory ──────────

#[test]
fn test_BC_1_06_008_temp_create_failure_is_io_005() {
    let target = std::env::temp_dir()
        .join("zw_nonexistent_dir_xyz")
        .join("out.json");
    let result = emit_to_file(&target, |w| w.write_all(b"x"));
    assert!(
        matches!(result, Err(IoError::TempCreate { .. })),
        "got {result:?}"
    );
}

// ── AC-003/004: warnings emitted to the writer in order, WARNING:-prefixed ────

#[test]
fn test_BC_1_06_005_warnings_emitted_in_order() {
    let warnings = vec![
        "3 flow records skipped".to_string(),
        "zone 'staging' has no members".to_string(),
    ];
    let mut buf = Vec::new();
    emit_warnings(&warnings, &mut buf).expect("ok");
    let s = String::from_utf8(buf).unwrap();
    assert_eq!(
        s,
        "WARNING: 3 flow records skipped\nWARNING: zone 'staging' has no members\n"
    );
}

#[test]
fn test_BC_1_06_005_zero_warnings_no_output() {
    // EC-003: zero warnings → nothing written.
    let mut buf = Vec::new();
    emit_warnings(&[], &mut buf).expect("ok");
    assert!(buf.is_empty());
}

#[test]
fn test_BC_1_06_005_warning_special_chars_emitted_verbatim() {
    // EC-005: special characters are emitted as-is (no escaping).
    let mut buf = Vec::new();
    emit_warnings(&[r#"weird "quoted" value"#.to_string()], &mut buf).expect("ok");
    assert_eq!(
        String::from_utf8(buf).unwrap(),
        "WARNING: weird \"quoted\" value\n"
    );
}

// ── AC-006: reporter is offline (no std::net), verified against the source ────

#[test]
fn test_BC_1_06_007_reporter_has_no_network_access() {
    let src = include_str!("../src/reporter.rs");
    assert!(!src.contains("std::net"), "reporter must not use std::net");
    assert!(
        !src.contains("TcpStream"),
        "reporter must not open TCP sockets"
    );
    assert!(
        !src.contains("UdpSocket"),
        "reporter must not open UDP sockets"
    );
}
