//! Zeek `conn.log` adapter (effectful shell, S-2.01).
//!
//! Streams a Zeek `conn.log` (TSV) line-by-line and normalizes each data record
//! into a [`Flow`] (BC-1.02.001). Columns are located by name from the Zeek
//! `#fields` header, so the adapter is robust to column reordering. The parse is
//! resilient (DI-013): comment/header and blank lines are skipped silently;
//! malformed records (E-FLW-001) and unspecified-address records (E-FLW-002,
//! BC-1.02.003) surface as `Err` items and never abort the run (BC-1.02.002).
//!
//! `flow_index` is dense and gap-free over successfully-normalized flows only
//! (BC-1.02.001 invariant 4). The Zeek `conn_state` token is bucketed through
//! [`zonewarden_core::severity::conn_state_from_token`] — the single DI-017
//! source of that mapping (S-4.01).

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::IpAddr;
use std::path::Path;

use zonewarden_core::errors::{FlowParseError, IngestError, IoError, SysError};
use zonewarden_core::severity;
use zonewarden_core::types::{ConnState, Flow, Proto, Service, ServiceSource, Timestamp};
use zonewarden_core::RealitySource;

/// Default ingest cap: the maximum number of successfully-normalized flows before
/// the run aborts (BC-1.02.006). Far below `u64::MAX`; raise with `--max-flows`.
pub const DEFAULT_MAX_FLOWS: u64 = 1_000_000;

/// Validate a `--max-flows` value: the cap must be at least 1 (BC-1.02.006 inv 2 /
/// AC-007). Called at CLI arg-parse time (S-6.03) before any flows are read.
pub fn validate_max_flows(n: u64) -> Result<u64, SysError> {
    if n == 0 {
        Err(SysError::ZeroMaxFlows)
    } else {
        Ok(n)
    }
}

/// Maps Zeek `#fields` column names to their positional index in each data line.
struct FieldMap {
    cols: HashMap<String, usize>,
}

impl FieldMap {
    /// Parse the portion of a `#fields` line after the literal `#fields` token.
    /// Columns are tab-separated; their order defines the positional index used
    /// to read each data line.
    fn parse(rest: &str) -> FieldMap {
        let cols = rest
            .split('\t')
            .filter(|s| !s.is_empty())
            .enumerate()
            .map(|(i, name)| (name.to_string(), i))
            .collect();
        FieldMap { cols }
    }

    fn index(&self, name: &str) -> Option<usize> {
        self.cols.get(name).copied()
    }
}

/// Streaming Zeek `conn.log` adapter. Holds the reader, the source line counter,
/// the dense `flow_index` cursor, and the column map parsed from `#fields`.
pub struct ZeekAdapter<R: BufRead> {
    reader: R,
    line_no: u64,
    next_index: u64,
    fields: Option<FieldMap>,
    buf: Vec<u8>,
    /// Ingest cap: abort once a successful flow would exceed this count
    /// (BC-1.02.006). Defaults to [`DEFAULT_MAX_FLOWS`].
    max_flows: u64,
    /// Set once a fatal error (cap breach) has been yielded, so iteration stops
    /// cleanly instead of emitting more records.
    done: bool,
}

impl ZeekAdapter<BufReader<File>> {
    /// Open a Zeek `conn.log` file for streaming. Only the open is effectful here;
    /// parsing happens lazily as the iterator is driven (no `read_to_string` —
    /// ADR-001, OQ-003).
    pub fn open(path: &Path) -> Result<Self, IoError> {
        let file = File::open(path).map_err(|e| io_error(path, &e))?;
        Ok(ZeekAdapter::from_reader(BufReader::new(file)))
    }
}

impl<R: BufRead> ZeekAdapter<R> {
    /// Build an adapter over any buffered reader (used by tests and by [`open`]).
    ///
    /// [`open`]: ZeekAdapter::open
    pub fn from_reader(reader: R) -> Self {
        ZeekAdapter {
            reader,
            line_no: 0,
            next_index: 0,
            fields: None,
            buf: Vec::new(),
            max_flows: DEFAULT_MAX_FLOWS,
            done: false,
        }
    }

    /// Override the ingest cap (BC-1.02.006). The CLI validates the value with
    /// [`validate_max_flows`] before calling this.
    pub fn with_max_flows(mut self, max: u64) -> Self {
        self.max_flows = max;
        self
    }

    /// Normalize one already-stripped data line into a `Flow`, assigning the next
    /// dense `flow_index` only on success. Parse failures surface as
    /// `IngestError::Parse` (skip signals); reaching the ingest cap surfaces as a
    /// fatal `IngestError::Sys` (BC-1.02.006).
    fn parse_data(&mut self, line: &str) -> Result<Flow, IngestError> {
        let line_no = self.line_no;
        let fields = self
            .fields
            .as_ref()
            .ok_or_else(|| malformed(line_no, "data line before #fields header"))?;
        let cols: Vec<&str> = line.split('\t').collect();

        let ts = parse_ts(col(&cols, fields, "ts", line_no)?, line_no)?;
        let src_ip = parse_ip(col(&cols, fields, "id.orig_h", line_no)?, line_no)?;
        let src_port = parse_port(col(&cols, fields, "id.orig_p", line_no)?, line_no)?;
        let dst_ip = parse_ip(col(&cols, fields, "id.resp_h", line_no)?, line_no)?;
        let dst_port = parse_port(col(&cols, fields, "id.resp_p", line_no)?, line_no)?;
        let proto = parse_proto(col(&cols, fields, "proto", line_no)?, line_no)?;
        let conn_state = parse_conn_state(col(&cols, fields, "conn_state", line_no)?);

        // Unspecified-address check runs AFTER IPv4-mapped canonicalization
        // (BC-1.02.005 invariant 2) and gates the record out before any Flow is
        // produced (BC-1.02.003). One skip per record (EC-006): src checked first.
        if src_ip.is_unspecified() {
            return Err(unspecified(line_no, "src").into());
        }
        if dst_ip.is_unspecified() {
            return Err(unspecified(line_no, "dst").into());
        }

        // Ingest cap (BC-1.02.006): breach when a successful flow would push the
        // count beyond max_flows. The cap counts successful flows only — skips
        // above never reach here. Strict `>`: exactly max_flows flows is OK.
        if self.next_index >= self.max_flows {
            self.done = true;
            return Err(IngestError::Sys(SysError::CapExceeded {
                max: self.max_flows,
                count: self.next_index,
            }));
        }

        let flow_index = self.next_index;
        self.next_index += 1;
        let (service, service_source) = infer_service(&proto, dst_port);
        Ok(Flow {
            flow_index,
            ts,
            src_ip,
            src_port,
            dst_ip,
            dst_port,
            proto,
            service,
            service_source,
            conn_state,
        })
    }
}

/// Infer the application-layer service from `(proto, dst_port)` against the
/// canonical table (BC-1.02.004 + D-010). A match is always `PortHeuristic`
/// (heuristic, never authoritative — DI-008); no match, a transport mismatch, or
/// a portless flow (`dst_port == None`) is `Unknown` with `service == None`.
/// `DpiConfirmed` is reserved for future DPI adapters and never produced here.
pub fn infer_service(proto: &Proto, dst_port: Option<u16>) -> (Option<Service>, ServiceSource) {
    let Some(port) = dst_port else {
        return (None, ServiceSource::Unknown);
    };
    // D-010 (human decision overriding BC-1.02.004 EC-004): DNP3 and EtherNet/IP
    // match TCP *and* UDP; IT services (HTTP/HTTPS/DNS/NTP) are included.
    let service = match (proto, port) {
        (Proto::Tcp, 502) => Some(Service::Modbus),
        (Proto::Tcp | Proto::Udp, 20000) => Some(Service::Dnp3),
        (Proto::Tcp | Proto::Udp, 44818) => Some(Service::EtherNetIp),
        (Proto::Tcp, 102) => Some(Service::S7comm),
        (Proto::Udp, 47808) => Some(Service::Bacnet),
        (Proto::Tcp, 4840) => Some(Service::OpcUa),
        (Proto::Tcp, 80) => Some(Service::Other("HTTP".to_string())),
        (Proto::Tcp, 443) => Some(Service::Other("HTTPS".to_string())),
        (Proto::Udp, 53) => Some(Service::Other("DNS".to_string())),
        (Proto::Udp, 123) => Some(Service::Other("NTP".to_string())),
        _ => None,
    };
    match service {
        Some(s) => (Some(s), ServiceSource::PortHeuristic),
        None => (None, ServiceSource::Unknown),
    }
}

impl<R: BufRead> Iterator for ZeekAdapter<R> {
    type Item = Result<Flow, IngestError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            // A fatal error (cap breach) was already yielded — stop cleanly.
            return None;
        }
        loop {
            self.buf.clear();
            // Read raw bytes (not `read_line`) so a non-UTF-8 byte does not abort
            // the stream: it is lossily decoded and that single line is skipped as
            // malformed, while the rest of the file still parses (BC-1.02.002 /
            // DI-013). A genuine I/O error ends the stream.
            let read = self.reader.read_until(b'\n', &mut self.buf);
            match read {
                Ok(0) | Err(_) => return None,
                Ok(_) => {}
            }
            self.line_no += 1;

            // Lossy UTF-8 decode, then strip the line terminator, tolerating
            // Windows CRLF (EC-007/AC-008).
            let decoded = String::from_utf8_lossy(&self.buf);
            let line = decoded.trim_end_matches(['\n', '\r']).to_string();

            if line.trim().is_empty() {
                continue; // blank line → silent skip (not counted)
            }
            if line.starts_with('#') {
                // Zeek metadata. Capture the column layout from `#fields`; all
                // `#` lines are skipped silently and never counted (EC-001).
                if let Some(rest) = line.strip_prefix("#fields") {
                    self.fields = Some(FieldMap::parse(rest));
                }
                continue;
            }

            return Some(self.parse_data(&line));
        }
    }
}

impl<R: BufRead> RealitySource for ZeekAdapter<R> {
    fn flows(&mut self) -> impl Iterator<Item = Result<Flow, IngestError>> {
        self.by_ref()
    }
}

// ── field + token parsers (pure) ─────────────────────────────────────────────

/// Read the column named `name` from a split data line.
fn col<'a>(
    cols: &[&'a str],
    fields: &FieldMap,
    name: &str,
    line: u64,
) -> Result<&'a str, FlowParseError> {
    let idx = fields
        .index(name)
        .ok_or_else(|| malformed(line, &format!("missing column `{name}` in #fields")))?;
    cols.get(idx)
        .copied()
        .ok_or_else(|| malformed(line, "truncated line (fewer columns than #fields)"))
}

/// Parse a Zeek fractional-epoch-seconds timestamp into nanoseconds since the
/// Unix epoch, widening (never truncating) the seconds (BC-1.02.001 postcond 1).
fn parse_ts(s: &str, line: u64) -> Result<Timestamp, FlowParseError> {
    let (sec_str, frac_str) = s.split_once('.').unwrap_or((s, ""));
    let secs: i128 = sec_str
        .parse()
        .map_err(|_| malformed(line, "unparseable ts seconds"))?;

    // The fractional part must be plain digits — a signed/garbage fraction is
    // malformed (guards against e.g. `1.-5` computing a wrong value).
    if !frac_str.bytes().all(|b| b.is_ascii_digit()) {
        return Err(malformed(line, "unparseable ts fraction"));
    }
    // Normalize the fractional part to exactly 9 digits (nanoseconds).
    let mut frac = String::with_capacity(9);
    frac.push_str(frac_str);
    frac.truncate(9);
    while frac.len() < 9 {
        frac.push('0');
    }
    let nanos: i128 = frac.parse().unwrap_or(0);

    // Checked arithmetic: an out-of-range timestamp is a malformed record
    // (skipped + counted), never a panic/abort or a silently-wrapped Flow
    // (BC-1.02.002 / DI-013).
    secs.checked_mul(1_000_000_000)
        .and_then(|s| s.checked_add(nanos))
        .map(Timestamp)
        .ok_or_else(|| malformed(line, "timestamp out of range"))
}

/// Parse a Zeek port field. `-` and `0` both mean "no port" (EC-004 / sentinel).
fn parse_port(s: &str, line: u64) -> Result<Option<u16>, FlowParseError> {
    if s == "-" {
        return Ok(None);
    }
    let p: u16 = s.parse().map_err(|_| malformed(line, "unparseable port"))?;
    Ok(if p == 0 { None } else { Some(p) })
}

/// Parse an IP address and canonicalize IPv4-mapped IPv6 to IPv4 (BC-1.02.005).
fn parse_ip(s: &str, line: u64) -> Result<IpAddr, FlowParseError> {
    let ip: IpAddr = s.parse().map_err(|_| malformed(line, "unparseable IP"))?;
    Ok(canonicalize(ip))
}

/// Canonicalize an `::ffff:a.b.c.d` IPv4-mapped IPv6 address to IPv4; leave all
/// other addresses unchanged (idempotent for already-IPv4 — BC-1.02.005 inv 3).
fn canonicalize(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
            Some(v4) => IpAddr::V4(v4),
            None => IpAddr::V6(v6),
        },
        v4 => v4,
    }
}

/// Map a Zeek transport `proto` token to the core `Proto`. Numeric tokens become
/// `Other(u8)`; anything else is a malformed record.
fn parse_proto(s: &str, line: u64) -> Result<Proto, FlowParseError> {
    Ok(match s {
        "tcp" => Proto::Tcp,
        "udp" => Proto::Udp,
        "icmp" => Proto::Icmp,
        other => match other.parse::<u8>() {
            Ok(n) => Proto::Other(n),
            Err(_) => return Err(malformed(line, &format!("unsupported proto `{other}`"))),
        },
    })
}

/// Map a Zeek `conn_state` token to the core `ConnState` bucket via the DI-017
/// single source (S-4.01). The Zeek sentinel `-` means "absent" → `None`.
fn parse_conn_state(s: &str) -> Option<ConnState> {
    if s == "-" {
        None
    } else {
        Some(severity::conn_state_from_token(s))
    }
}

fn malformed(line: u64, detail: &str) -> FlowParseError {
    FlowParseError::Malformed {
        line,
        detail: detail.to_string(),
    }
}

fn unspecified(line: u64, role: &str) -> FlowParseError {
    FlowParseError::UnspecifiedAddress {
        line,
        role: role.to_string(),
    }
}

fn io_error(path: &Path, e: &std::io::Error) -> IoError {
    let p = path.display().to_string();
    match e.kind() {
        std::io::ErrorKind::NotFound => IoError::NotFound { path: p },
        std::io::ErrorKind::PermissionDenied => IoError::PermissionDenied { path: p },
        _ => IoError::Read {
            path: p,
            detail: e.to_string(),
        },
    }
}
