//! Integration tests for S-2.02 — service inference (BC-1.02.004 + D-010).
//!
//! D-010 (human decision overriding BC-1.02.004 EC-004): DNP3 and EtherNet/IP
//! match TCP *and* UDP, and the table includes the IT services HTTP/HTTPS/DNS/NTP.
//!
//! Test names intentionally use the upper-case `BC` contract id.
#![allow(non_snake_case)]

use std::path::{Path, PathBuf};

use zonewarden::adapters::zeek::{infer_service, ZeekAdapter};
use zonewarden_core::types::{Proto, Service, ServiceSource};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

// ── AC-001: Modbus / TCP 502 ─────────────────────────────────────────────────

#[test]
fn test_BC_1_02_004_port_502_tcp_infers_modbus() {
    let (svc, src) = infer_service(&Proto::Tcp, Some(502));
    assert_eq!(svc, Some(Service::Modbus));
    assert_eq!(src, ServiceSource::PortHeuristic);
}

// ── AC-002: EtherNet/IP / TCP 44818 ──────────────────────────────────────────

#[test]
fn test_BC_1_02_004_port_44818_tcp_infers_ethernet_ip() {
    let (svc, src) = infer_service(&Proto::Tcp, Some(44818));
    assert_eq!(svc, Some(Service::EtherNetIp));
    assert_eq!(src, ServiceSource::PortHeuristic);
}

// ── AC-003: the canonical OT table (D-010: DNP3 & EtherNet/IP on TCP+UDP) ─────

#[test]
fn test_BC_1_02_004_canonical_ot_ports_covered() {
    let cases: &[(Proto, u16, Service)] = &[
        (Proto::Tcp, 502, Service::Modbus),
        (Proto::Tcp, 20000, Service::Dnp3),
        (Proto::Udp, 20000, Service::Dnp3), // D-010: UDP also matches
        (Proto::Tcp, 44818, Service::EtherNetIp),
        (Proto::Udp, 44818, Service::EtherNetIp), // D-010
        (Proto::Tcp, 102, Service::S7comm),
        (Proto::Udp, 47808, Service::Bacnet),
        (Proto::Tcp, 4840, Service::OpcUa),
    ];
    for (proto, port, expected) in cases {
        let (svc, src) = infer_service(proto, Some(*port));
        assert_eq!(svc.as_ref(), Some(expected), "{proto:?}/{port}");
        assert_eq!(src, ServiceSource::PortHeuristic, "{proto:?}/{port}");
    }
}

// ── IT services (D-010) + transport-mismatch / non-default-port edge cases ────

#[test]
fn test_D010_it_services_inferred() {
    for (proto, port, name) in [
        (Proto::Tcp, 80u16, "HTTP"),
        (Proto::Tcp, 443, "HTTPS"),
        (Proto::Udp, 53, "DNS"),
        (Proto::Udp, 123, "NTP"),
    ] {
        let (svc, src) = infer_service(&proto, Some(port));
        assert_eq!(svc, Some(Service::Other(name.to_string())), "{name}");
        assert_eq!(src, ServiceSource::PortHeuristic, "{name}");
    }
}

#[test]
fn test_BC_1_02_004_transport_mismatch_is_unknown() {
    // EC-002: Modbus is TCP-only — 502/UDP does not match.
    let (svc, src) = infer_service(&Proto::Udp, Some(502));
    assert_eq!(svc, None);
    assert_eq!(src, ServiceSource::Unknown);
}

#[test]
fn test_BC_1_02_004_non_default_port_is_unknown() {
    // EC-003: Modbus on a non-default port is not inferred.
    let (svc, src) = infer_service(&Proto::Tcp, Some(1502));
    assert_eq!(svc, None);
    assert_eq!(src, ServiceSource::Unknown);
}

// ── AC-004: unknown port → Unknown ───────────────────────────────────────────

#[test]
fn test_BC_1_02_004_unknown_port_produces_unknown_source() {
    let (svc, src) = infer_service(&Proto::Tcp, Some(9999));
    assert_eq!(svc, None);
    assert_eq!(src, ServiceSource::Unknown);
}

// ── AC-005: service_source always set; portless → Unknown; no DpiConfirmed ────

#[test]
fn test_BC_1_02_004_service_source_always_present() {
    // ICMP / portless (dst_port None) → None + Unknown.
    let (svc, src) = infer_service(&Proto::Icmp, None);
    assert_eq!(svc, None);
    assert_eq!(src, ServiceSource::Unknown);

    // A matched flow is PortHeuristic, never DpiConfirmed (VP-1.02.004-b). Across
    // a sweep of ports the source is only ever PortHeuristic or Unknown.
    for port in [0u16, 80, 102, 502, 4840, 20000, 44818, 47808, 65535] {
        for proto in [Proto::Tcp, Proto::Udp] {
            let (_, src) = infer_service(&proto, Some(port));
            assert_ne!(src, ServiceSource::DpiConfirmed, "{proto:?}/{port}");
        }
    }
}

// ── Integration: the adapter populates Flow.service via the table ─────────────

#[test]
fn test_BC_1_02_004_adapter_populates_service() {
    // minimal.log: flow 0 = 502/tcp (Modbus), flow 1 = 44818/udp (EtherNet/IP via D-010).
    let adapter = ZeekAdapter::open(&fixture("minimal.log")).expect("open minimal.log");
    let flows: Vec<_> = adapter.filter_map(Result::ok).collect();
    assert_eq!(flows.len(), 2);
    assert_eq!(flows[0].service, Some(Service::Modbus));
    assert_eq!(flows[0].service_source, ServiceSource::PortHeuristic);
    assert_eq!(flows[1].service, Some(Service::EtherNetIp));
    assert_eq!(flows[1].service_source, ServiceSource::PortHeuristic);
}
