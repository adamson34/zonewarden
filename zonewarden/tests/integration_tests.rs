//! End-to-end integration tests for S-6.03 — CLI (BC-1.06.001/006/007).
//!
//! Each test runs the built `zonewarden` binary against fixtures and asserts the
//! process exit code + output. Exit-code logic is also unit-tested via
//! `cli::exit_status`.
//!
//! Test names intentionally use the upper-case `BC` contract id.
#![allow(non_snake_case)]

use std::path::PathBuf;
use std::process::{Command, Output};

use zonewarden::cli::exit_status;
use zonewarden_core::types::ConformanceResult;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Run the binary with args; return (exit_code, stdout, stderr).
fn run(args: &[&str]) -> (i32, String, String) {
    let out: Output = Command::new(env!("CARGO_BIN_EXE_zonewarden"))
        .args(args)
        .output()
        .expect("run zonewarden");
    (
        out.status.code().expect("exit code"),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn policy() -> String {
    fixture("cli_policy.yaml").display().to_string()
}

// ── AC-001: conformant → exit 0 ──────────────────────────────────────────────

#[test]
fn test_BC_1_06_001_exit_0_conformant() {
    let (code, _out, _err) = run(&[
        "--policy",
        &policy(),
        "--flows",
        &fixture("conformant.log").display().to_string(),
    ]);
    assert_eq!(code, 0);
}

// ── AC-002: violations → exit 1 ──────────────────────────────────────────────

#[test]
fn test_BC_1_06_001_exit_1_violations() {
    let (code, _out, _err) = run(&[
        "--policy",
        &policy(),
        "--flows",
        &fixture("violating.log").display().to_string(),
    ]);
    assert_eq!(code, 1);
}

// ── AC-011: IDMZ bypass only → exit 1 ────────────────────────────────────────

#[test]
fn test_BC_1_06_001_exit_1_idmz_bypass_only() {
    let (code, out, _err) = run(&[
        "--policy",
        &policy(),
        "--flows",
        &fixture("idmz_bypass.log").display().to_string(),
    ]);
    assert_eq!(code, 1);
    assert!(out.contains("IdmzBypass"), "out: {out}");
}

// ── AC-003: policy error → exit 2 ────────────────────────────────────────────

#[test]
fn test_BC_1_06_001_exit_2_policy_error() {
    let (code, _out, err) = run(&[
        "--policy",
        &fixture("invalid_direction.yaml").display().to_string(),
        "--flows",
        &fixture("conformant.log").display().to_string(),
    ]);
    assert_eq!(code, 2);
    assert!(err.contains("E-POL-003"), "stderr: {err}");
}

// ── AC-004: missing policy file → exit 2, E-IO-001 ───────────────────────────

#[test]
fn test_BC_1_06_001_exit_2_missing_policy_file() {
    let (code, _out, err) = run(&[
        "--policy",
        &fixture("does_not_exist.yaml").display().to_string(),
        "--flows",
        &fixture("conformant.log").display().to_string(),
    ]);
    assert_eq!(code, 2);
    assert!(err.contains("E-IO-001"), "stderr: {err}");
}

// ── AC-008: ingest cap breach → exit 2 ───────────────────────────────────────

#[test]
fn test_BC_1_06_001_exit_2_cap_breach() {
    let (code, _out, err) = run(&[
        "--policy",
        &policy(),
        "--flows",
        &fixture("huge.log").display().to_string(),
        "--max-flows",
        "5",
    ]);
    assert_eq!(code, 2);
    assert!(
        err.contains("E-SYS-001") && err.contains('5'),
        "stderr: {err}"
    );
}

// ── AC-010: --max-flows 0 → exit 2, E-SYS-002 ────────────────────────────────

#[test]
fn test_BC_1_06_006_max_flows_zero_rejected() {
    let (code, _out, err) = run(&[
        "--policy",
        &policy(),
        "--flows",
        &fixture("conformant.log").display().to_string(),
        "--max-flows",
        "0",
    ]);
    assert_eq!(code, 2);
    assert!(err.contains("E-SYS-002"), "stderr: {err}");
}

// ── AC-005/006: --fail-on-skipped + skipped warning ──────────────────────────

#[test]
fn test_BC_1_06_006_fail_on_skipped_upgrades_exit_0_to_1() {
    let flows = fixture("conformant_with_skips.log").display().to_string();
    // without the flag: conformant despite skips → exit 0
    let (code0, _o, err0) = run(&["--policy", &policy(), "--flows", &flows]);
    assert_eq!(code0, 0);
    // AC-006: the skipped warning is always emitted to stderr
    assert!(err0.to_lowercase().contains("skipped"), "stderr: {err0}");
    // with the flag: exit upgraded to 1
    let (code1, _o, _e) = run(&[
        "--policy",
        &policy(),
        "--flows",
        &flows,
        "--fail-on-skipped",
    ]);
    assert_eq!(code1, 1);
}

// ── AC-009: full pipeline, JSON output, schema-valid, exit 1 ─────────────────

#[test]
fn test_full_pipeline_end_to_end() {
    let (code, out, _err) = run(&[
        "--policy",
        &policy(),
        "--flows",
        &fixture("violating.log").display().to_string(),
        "--format",
        "json",
    ]);
    assert_eq!(code, 1);
    let j: serde_json::Value = serde_json::from_str(&out).expect("valid JSON on stdout");
    assert_eq!(j["schema_version"], "1.0");
    assert_eq!(j["distinct_violating_flows"], 1);
    assert_eq!(j["violations"][0]["kind"], "NoMatchingConduit");
}

// ── EC-008: empty flows + valid policy → exit 0, all-zero ────────────────────

#[test]
fn test_empty_flows_exit_0() {
    let (code, out, _err) = run(&[
        "--policy",
        &policy(),
        "--flows",
        &fixture("empty_flows.log").display().to_string(),
        "--format",
        "json",
    ]);
    assert_eq!(code, 0);
    let j: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(j["total_flows"], 0);
    assert!(j["violations"].as_array().unwrap().is_empty());
}

// ── EC-003: --output writes the report atomically to a file ──────────────────

#[test]
fn test_output_to_file() {
    let target = std::env::temp_dir().join(format!("zw_cli_out_{}.md", std::process::id()));
    let _ = std::fs::remove_file(&target);
    let (code, out, _err) = run(&[
        "--policy",
        &policy(),
        "--flows",
        &fixture("violating.log").display().to_string(),
        "--format",
        "mermaid",
        "--output",
        &target.display().to_string(),
    ]);
    assert_eq!(code, 1);
    assert!(out.is_empty(), "report goes to file, not stdout");
    let written = std::fs::read_to_string(&target).expect("output file written");
    assert!(written.starts_with("graph LR"));
    let _ = std::fs::remove_file(&target);
}

// ── P5-IO-006: --output equal to an input file is refused (exit 2) ───────────

#[test]
fn test_output_equal_to_input_refused() {
    let flows = fixture("conformant.log").display().to_string();
    let (code, _out, err) = run(&[
        "--policy",
        &policy(),
        "--flows",
        &flows,
        "--output",
        &flows, // output == input → must refuse, never overwrite the input
    ]);
    assert_eq!(code, 2, "must exit 2");
    assert!(err.contains("input"), "stderr should explain: {err}");
    // the input file must be untouched (still a valid Zeek log)
    let after = std::fs::read_to_string(&flows).unwrap();
    assert!(
        after.starts_with("#fields"),
        "input must not be overwritten"
    );
}

// ── AC-007: binary crate has no network access (source-level) ────────────────

#[test]
fn test_BC_1_06_007_no_network_in_binary() {
    for src in [
        include_str!("../src/cli.rs"),
        include_str!("../src/main.rs"),
    ] {
        assert!(!src.contains("std::net"));
        assert!(!src.contains("TcpStream"));
        assert!(!src.contains("UdpSocket"));
    }
}

// ── exit_status unit logic (AC-001/002/005/011) ──────────────────────────────

#[test]
fn test_exit_status_logic() {
    let mut r = ConformanceResult::default();
    assert_eq!(exit_status(&r, false), 0); // clean

    r.distinct_violating_flows = 1;
    assert_eq!(exit_status(&r, false), 1); // violations

    r = ConformanceResult::default();
    r.idmz_bypasses = 1;
    assert_eq!(exit_status(&r, false), 1); // AC-011: idmz only → 1

    r = ConformanceResult::default();
    r.skipped = 3;
    assert_eq!(exit_status(&r, false), 0); // skips alone → 0
    assert_eq!(exit_status(&r, true), 1); // AC-005: --fail-on-skipped → 1
}
