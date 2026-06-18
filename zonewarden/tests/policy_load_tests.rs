//! Integration tests for S-1.02 — Policy YAML load (BC-1.01.001/002/003).

use std::path::{Path, PathBuf};

use zonewarden::policy::load;
use zonewarden_core::errors::{PolicyError, ZonewardenError};
use zonewarden_core::portset::{PortRange, PortSet};
use zonewarden_core::types::{Direction, Proto, PurdueLevel};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

// AC-001: a valid minimal policy loads with all zones/conduits populated.
#[test]
fn test_bc_1_01_001_valid_minimal_policy_loads() {
    let p = load(&fixture("minimal.yaml")).expect("minimal policy should load");
    assert_eq!(p.zones.len(), 2);
    assert_eq!(p.conduits.len(), 2);
}

// AC-002: zone fields are populated and typed correctly.
#[test]
fn test_bc_1_01_001_zone_fields_populated() {
    let p = load(&fixture("minimal.yaml")).unwrap();
    let plc = &p.zones[0];
    assert_eq!(plc.id.0, "plc");
    assert_eq!(plc.name, "PLC Cell");
    assert_eq!(plc.purdue_level, PurdueLevel::L1);
    assert!(!plc.members.is_empty());

    let hist = &p.zones[1];
    assert_eq!(hist.purdue_level, PurdueLevel::L3);
    let sl = hist.sl_t.expect("historian has sl_t");
    assert_eq!(sl.overall, Some(3));
    assert_eq!(sl.fr_vector, None);
}

// AC-003: conduit fields are populated; direction/proto/ports typed correctly.
#[test]
fn test_bc_1_01_001_conduit_fields_populated() {
    let p = load(&fixture("minimal.yaml")).unwrap();
    let c0 = &p.conduits[0];
    assert_eq!(c0.from_zone.0, "plc");
    assert_eq!(c0.to_zone.0, "historian");
    assert_eq!(c0.direction, Direction::Forward); // from "unidirectional"
    assert_eq!(c0.proto, Proto::Tcp);

    let c1 = &p.conduits[1];
    assert_eq!(c1.direction, Direction::Bidirectional);
    assert_eq!(c1.proto, Proto::Udp);
    assert_eq!(c1.ports, PortSet::Any);
}

// AC-004: PortSet parsed from YAML is canonical (adjacent [500,501] -> [500-501]).
#[test]
fn test_bc_1_01_001_portset_canonical_at_load() {
    let p = load(&fixture("minimal.yaml")).unwrap();
    assert_eq!(
        p.conduits[0].ports,
        PortSet::Ranges(vec![PortRange { lo: 500, hi: 501 }])
    );
}

// AC-005: `unidirectional` is normalized to Direction::Forward.
#[test]
fn test_bc_1_01_001_unidirectional_alias_normalized() {
    let p = load(&fixture("minimal.yaml")).unwrap();
    assert_eq!(p.conduits[0].direction, Direction::Forward);
}

// AC-006a: empty file -> E-POL-001.
#[test]
fn test_bc_1_01_002_empty_file_returns_pol_001() {
    let err = load(&fixture("empty.yaml")).unwrap_err();
    assert!(
        matches!(err, ZonewardenError::Policy(PolicyError::Empty)),
        "expected E-POL-001 Empty, got {err:?}"
    );
}

// AC-006b: missing required field (purdue_level) -> E-POL-002.
#[test]
fn test_bc_1_01_002_missing_field_returns_pol_002() {
    let err = load(&fixture("missing_field.yaml")).unwrap_err();
    assert!(
        matches!(err, ZonewardenError::Policy(PolicyError::MissingField { .. })),
        "expected E-POL-002 MissingField, got {err:?}"
    );
}

// AC-007: duplicate mapping key -> E-POL-004.
#[test]
fn test_bc_1_01_003_duplicate_key_rejected() {
    let err = load(&fixture("dup_key.yaml")).unwrap_err();
    assert!(
        matches!(err, ZonewardenError::Policy(PolicyError::DuplicateKey { .. })),
        "expected E-POL-004 DuplicateKey, got {err:?}"
    );
}

// EC-008: missing file -> E-IO-001.
#[test]
fn test_ec_008_file_not_found_returns_io_001() {
    let err = load(&fixture("does_not_exist.yaml")).unwrap_err();
    assert!(
        matches!(err, ZonewardenError::Io(zonewarden_core::errors::IoError::NotFound { .. })),
        "expected E-IO-001 NotFound, got {err:?}"
    );
}

// AC-009: parsing is deterministic — loading the same file twice yields equal Policy.
#[test]
fn test_bc_1_01_001_parse_is_deterministic() {
    let a = load(&fixture("minimal.yaml")).unwrap();
    let b = load(&fixture("minimal.yaml")).unwrap();
    assert_eq!(a, b);
}
