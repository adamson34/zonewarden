//! Report formatters — JSON, text, Mermaid (effectful shell, S-6.01).
//!
//! Renders a `ConformanceResult` to a `&mut dyn Write` in three formats. The
//! formatting logic is pure string/serde building; only the final write is
//! effectful. Per ADR-002 the JSON wire format lives here in the shell (via
//! shell-side `Serialize` DTOs), not in the pure core. Mermaid is hand-built
//! string generation — no render library (ADR-007). All output is deterministic
//! for identical input (violations are pre-sorted by S-5.03; Mermaid nodes and
//! edges are sorted here).

use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use serde::Serialize;

use zonewarden_core::errors::IoError;
use zonewarden_core::types::{
    ConformanceResult, ConnState, Proto, PurdueLevel, Service, ServiceSource, Severity,
    ValidatedPolicy, Violation, ViolationKind,
};

const SCHEMA_VERSION: &str = "1.0";

// ── JSON (BC-1.06.002, schema in interface-definitions.md) ───────────────────

#[derive(Serialize)]
struct JsonReport<'a> {
    schema_version: &'static str,
    policy_digest: &'a str,
    total_flows: u64,
    intra_zone: u64,
    allowed: u64,
    no_matching_conduit: u64,
    wrong_direction: u64,
    multicast_exempt: u64,
    idmz_bypasses: u64,
    distinct_violating_flows: u64,
    external_endpoints: u64,
    skipped: u64,
    warnings: &'a [String],
    violations: Vec<JsonViolation<'a>>,
}

/// A violation row matching the JSON schema. Note: there is no `idmz_bypass`
/// field — an IDMZ bypass is its own row with `kind: "IdmzBypass"`.
#[derive(Serialize)]
struct JsonViolation<'a> {
    flow_index: u64,
    src_zone: &'a str,
    dst_zone: &'a str,
    kind: &'static str,
    severity: &'static str,
    explanation: &'a str,
    src_ip: String,
    dst_ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dst_port: Option<u16>,
    proto: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<String>,
    service_source: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    conn_state: Option<String>,
}

fn json_violation(v: &Violation) -> JsonViolation<'_> {
    JsonViolation {
        flow_index: v.flow_index,
        src_zone: &v.src_zone.0,
        dst_zone: &v.dst_zone.0,
        kind: violation_kind_str(&v.kind),
        severity: severity_str(v.severity),
        explanation: &v.explanation,
        src_ip: v.src_ip.to_string(),
        dst_ip: v.dst_ip.to_string(),
        src_port: v.src_port,
        dst_port: v.dst_port,
        proto: proto_str(&v.proto),
        service: v.service.as_ref().map(service_str),
        service_source: service_source_str(v.service_source),
        conn_state: v.conn_state.as_ref().map(conn_state_str),
    }
}

/// Emit the machine-readable JSON report (BC-1.06.002), schema v1.0.
pub fn emit_json(result: &ConformanceResult, out: &mut dyn Write) -> io::Result<()> {
    let report = JsonReport {
        schema_version: SCHEMA_VERSION,
        policy_digest: &result.policy_digest,
        total_flows: result.total_flows,
        intra_zone: result.intra_zone,
        allowed: result.allowed,
        no_matching_conduit: result.no_matching_conduit,
        wrong_direction: result.wrong_direction,
        multicast_exempt: result.multicast_exempt,
        idmz_bypasses: result.idmz_bypasses,
        distinct_violating_flows: result.distinct_violating_flows,
        external_endpoints: result.external_endpoints,
        skipped: result.skipped,
        warnings: &result.warnings,
        violations: result.violations.iter().map(json_violation).collect(),
    };
    serde_json::to_writer(&mut *out, &report).map_err(io::Error::other)?;
    out.write_all(b"\n")
}

// ── Text (BC-1.06.003) ───────────────────────────────────────────────────────

/// Emit the human-readable text report (BC-1.06.003): a full summary block
/// (all tallies + policy_digest) and a per-violation line carrying the
/// endpoints, service (heuristic-flagged — DI-008), and explanation. Warnings
/// are NOT written here — they go to stderr via the CLI (BC-1.06.005 inv 1).
pub fn emit_text(result: &ConformanceResult, out: &mut dyn Write) -> io::Result<()> {
    writeln!(out, "Summary:")?;
    writeln!(out, "  policy_digest: {}", result.policy_digest)?;
    writeln!(out, "  total_flows: {}", result.total_flows)?;
    writeln!(out, "  intra_zone: {}", result.intra_zone)?;
    writeln!(out, "  allowed: {}", result.allowed)?;
    writeln!(out, "  no_matching_conduit: {}", result.no_matching_conduit)?;
    writeln!(out, "  wrong_direction: {}", result.wrong_direction)?;
    writeln!(out, "  multicast_exempt: {}", result.multicast_exempt)?;
    writeln!(out, "  idmz_bypasses: {}", result.idmz_bypasses)?;
    writeln!(
        out,
        "  distinct_violating_flows: {}",
        result.distinct_violating_flows
    )?;
    writeln!(out, "  external_endpoints: {}", result.external_endpoints)?;
    writeln!(out, "  skipped: {}", result.skipped)?;

    if result.violations.is_empty() {
        writeln!(out, "No violations found")?;
        return Ok(());
    }

    for v in &result.violations {
        let provenance = match v.service_source {
            ServiceSource::PortHeuristic => " [heuristic]",
            ServiceSource::DpiConfirmed => " [confirmed]",
            ServiceSource::Unknown => "",
        };
        let service = v
            .service
            .as_ref()
            .map(|s| format!(" {}{provenance}", service_str(s)))
            .unwrap_or_default();
        writeln!(
            out,
            "VIOLATION [{sev}] flow_index={fi} {sz} -> {dz} [{kind}] {sip}{sp} -> {dip}{dp} {proto}{service} — {explanation}",
            sev = severity_str(v.severity),
            fi = v.flow_index,
            sz = v.src_zone.0,
            dz = v.dst_zone.0,
            kind = violation_kind_str(&v.kind),
            sip = v.src_ip,
            sp = v.src_port.map(|p| format!(":{p}")).unwrap_or_default(),
            dip = v.dst_ip,
            dp = v.dst_port.map(|p| format!(":{p}")).unwrap_or_default(),
            proto = proto_str(&v.proto),
            explanation = v.explanation,
        )?;
    }
    Ok(())
}

// ── Mermaid (BC-1.06.004, ADR-007 string generation) ─────────────────────────

/// Emit a Mermaid `graph LR` topology with zones as nodes and conduits as edges;
/// violated zones get the `:::violation` class. Deterministic (sorted).
///
/// Zone ids are arbitrary strings (a space, quote, or bracket would break a raw
/// Mermaid node identifier — WAVE5-001), so every node gets a synthetic alias
/// `z{i}` (sorted-index, collision-free) and the raw id appears only inside the
/// quoted, escaped label. Edges reference aliases.
pub fn emit_mermaid(
    result: &ConformanceResult,
    policy: &ValidatedPolicy,
    out: &mut dyn Write,
) -> io::Result<()> {
    writeln!(out, "graph LR")?;

    // All node ids: declared zones plus any conduit endpoint (e.g. EXTERNAL).
    let mut ids: BTreeSet<&str> = policy
        .policy
        .zones
        .iter()
        .map(|z| z.id.0.as_str())
        .collect();
    for c in &policy.policy.conduits {
        ids.insert(c.from_zone.0.as_str());
        ids.insert(c.to_zone.0.as_str());
    }
    // Stable alias per id (sorted order → deterministic).
    let alias: std::collections::HashMap<&str, String> = ids
        .iter()
        .enumerate()
        .map(|(i, id)| (*id, format!("z{i}")))
        .collect();
    let level: std::collections::HashMap<&str, PurdueLevel> = policy
        .policy
        .zones
        .iter()
        .map(|z| (z.id.0.as_str(), z.purdue_level))
        .collect();

    // Declared zones appearing in any violation get highlighted.
    let mut violated: BTreeSet<&str> = BTreeSet::new();
    for v in &result.violations {
        if level.contains_key(v.src_zone.0.as_str()) {
            violated.insert(v.src_zone.0.as_str());
        }
        if level.contains_key(v.dst_zone.0.as_str()) {
            violated.insert(v.dst_zone.0.as_str());
        }
    }

    // Nodes, sorted by id (BTreeSet iteration → deterministic). Declared zones
    // carry their Purdue level; edge-only endpoints (EXTERNAL) just the id.
    for id in &ids {
        let label = match level.get(id) {
            Some(lvl) => format!("{} ({})", mermaid_escape(id), purdue_str(*lvl)),
            None => mermaid_escape(id),
        };
        let class = if violated.contains(id) {
            ":::violation"
        } else {
            ""
        };
        writeln!(out, "    {}[\"{label}\"]{class}", alias[id])?;
    }

    // Edges, sorted by (from, to), referencing aliases.
    let mut conduits: Vec<_> = policy.policy.conduits.iter().collect();
    conduits.sort_by(|a, b| (&a.from_zone.0, &a.to_zone.0).cmp(&(&b.from_zone.0, &b.to_zone.0)));
    for c in &conduits {
        writeln!(
            out,
            "    {} --> {}",
            alias[c.from_zone.0.as_str()],
            alias[c.to_zone.0.as_str()]
        )?;
    }

    writeln!(out, "    classDef violation fill:#f66")
}

// ── token helpers (stable strings; verbatim payloads — DI-009) ───────────────

fn violation_kind_str(k: &ViolationKind) -> &'static str {
    match k {
        ViolationKind::NoMatchingConduit => "NoMatchingConduit",
        ViolationKind::WrongDirection => "WrongDirection",
        ViolationKind::IdmzBypass => "IdmzBypass",
    }
}

fn severity_str(s: Severity) -> &'static str {
    match s {
        Severity::Established => "Established",
        Severity::Attempted => "Attempted",
    }
}

fn service_source_str(s: ServiceSource) -> &'static str {
    match s {
        ServiceSource::DpiConfirmed => "DpiConfirmed",
        ServiceSource::PortHeuristic => "PortHeuristic",
        ServiceSource::Unknown => "Unknown",
    }
}

fn proto_str(p: &Proto) -> String {
    match p {
        Proto::Tcp => "tcp".to_string(),
        Proto::Udp => "udp".to_string(),
        Proto::Icmp => "icmp".to_string(),
        Proto::Other(n) => format!("other:{n}"),
    }
}

fn service_str(s: &Service) -> String {
    match s {
        Service::Modbus => "Modbus".to_string(),
        Service::Dnp3 => "DNP3".to_string(),
        Service::EtherNetIp => "EtherNet/IP".to_string(),
        Service::S7comm => "S7comm".to_string(),
        Service::Bacnet => "BACnet".to_string(),
        Service::OpcUa => "OPC-UA".to_string(),
        Service::Other(name) => name.clone(),
    }
}

fn conn_state_str(c: &ConnState) -> String {
    match c {
        ConnState::Established => "Established".to_string(),
        ConnState::Attempted => "Attempted".to_string(),
        ConnState::Other(s) => s.clone(),
    }
}

/// Escape a string for use inside a Mermaid quoted label. Node identity is
/// handled separately by aliasing, so only the label-breaking `"` needs escaping
/// (Mermaid renders the `#quot;` entity).
fn mermaid_escape(s: &str) -> String {
    s.replace('"', "#quot;")
}

fn purdue_str(l: PurdueLevel) -> &'static str {
    match l {
        PurdueLevel::L0 => "L0",
        PurdueLevel::L1 => "L1",
        PurdueLevel::L2 => "L2",
        PurdueLevel::L3 => "L3",
        PurdueLevel::Idmz => "IDMZ",
        PurdueLevel::L4 => "L4",
        PurdueLevel::L5 => "L5",
    }
}

// ── atomic file write + warnings (S-6.02) ────────────────────────────────────

/// Write output atomically (FM-006 / BC-1.06.008): the `write_fn` writes into a
/// temporary file in the same directory as `path`, which is then atomically
/// renamed onto `path`. On any failure the temp file is removed and the target
/// is left untouched (never partially written).
pub fn emit_to_file<F>(path: &Path, write_fn: F) -> Result<(), IoError>
where
    F: FnOnce(&mut dyn Write) -> io::Result<()>,
{
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "output".to_string());
    // Same directory as the target → same filesystem → rename is atomic.
    let temp = parent.join(format!(".{file_name}.tmp.{}", std::process::id()));

    let out_write = |detail: String| IoError::OutputWrite {
        path: path.display().to_string(),
        detail,
    };

    let mut file = File::create(&temp).map_err(|e| IoError::TempCreate {
        path: temp.display().to_string(),
        detail: e.to_string(),
    })?;

    if let Err(e) = write_fn(&mut file).and_then(|()| file.flush()) {
        let _ = fs::remove_file(&temp);
        return Err(out_write(e.to_string()));
    }
    drop(file); // close before rename

    if let Err(e) = fs::rename(&temp, path) {
        let _ = fs::remove_file(&temp);
        return Err(out_write(e.to_string()));
    }
    Ok(())
}

/// Emit run warnings to a writer (the CLI passes stderr), one `WARNING:`-prefixed
/// line each, in the deterministic order supplied (BC-1.06.005). Never affects
/// the exit code.
pub fn emit_warnings(warnings: &[String], out: &mut dyn Write) -> io::Result<()> {
    for w in warnings {
        writeln!(out, "WARNING: {w}")?;
    }
    Ok(())
}
