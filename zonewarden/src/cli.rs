//! CLI — argument parsing, pipeline orchestration, exit codes (effectful shell,
//! S-6.03). This is the only module that assembles the full pipeline and decides
//! the process exit code; `main.rs` is a thin wrapper.
//!
//! Pipeline (ST-1..ST-8): validate args → load + validate policy → stream flows
//! → per flow: `classify_dst` → `resolve_pair` → `classify` → `violations_for`
//! → aggregate → emit warnings (stderr) + report (stdout/`--output`) → exit code.
//! Exit codes (BC-1.06.001): 0 conformant, 1 violations (or `--fail-on-skipped`
//! with skips), 2 any infrastructure error (policy/IO/usage/cap/overflow).

use std::io::{self, Write};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use zonewarden_core::aggregator;
use zonewarden_core::classifier::{self, ClassifyCtx};
use zonewarden_core::errors::{IngestError, IoError, ZonewardenError};
use zonewarden_core::types::{ConformanceResult, ValidatedPolicy, Verdict, Violation};
use zonewarden_core::{multicast, resolver, validator};

use crate::adapters::zeek::{validate_max_flows, ZeekAdapter, DEFAULT_MAX_FLOWS};
use crate::{policy, reporter};

/// Output format for the conformance report.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Format {
    Text,
    Json,
    Mermaid,
}

/// `zonewarden` — validate an IEC 62443 zone/conduit policy against observed flows.
#[derive(Parser, Debug)]
#[command(name = "zonewarden", version, about)]
pub struct CliArgs {
    /// Path to the YAML segmentation policy.
    #[arg(long)]
    pub policy: PathBuf,
    /// Path to the Zeek conn.log flow capture.
    #[arg(long)]
    pub flows: PathBuf,
    /// Report format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,
    /// Write the report to this file (atomically) instead of stdout.
    #[arg(long)]
    pub output: Option<PathBuf>,
    /// Exit 1 if any flow records were skipped (even with no violations).
    #[arg(long)]
    pub fail_on_skipped: bool,
    /// Maximum successfully-normalized flows before aborting (must be >= 1).
    #[arg(long, default_value_t = DEFAULT_MAX_FLOWS)]
    pub max_flows: u64,
}

/// The conformance exit code (BC-1.06.001): 1 if there are violations or IDMZ
/// bypasses, or `--fail-on-skipped` with skips; otherwise 0. Infrastructure
/// errors (exit 2) are handled by the caller mapping `ZonewardenError`.
pub fn exit_status(result: &ConformanceResult, fail_on_skipped: bool) -> u8 {
    let has_findings = result.distinct_violating_flows > 0 || result.idmz_bypasses > 0;
    let fail_skip = fail_on_skipped && result.skipped > 0;
    if has_findings || fail_skip {
        1
    } else {
        0
    }
}

/// Run the full pipeline. Returns the conformance exit code (0/1) on success, or
/// a `ZonewardenError` (→ exit 2) for any infrastructure failure.
pub fn run(args: &CliArgs) -> Result<u8, ZonewardenError> {
    // Usage check BEFORE any file is opened (BC-1.06.006 / SS-03).
    let max_flows = validate_max_flows(args.max_flows)?;

    // Refuse to write the report over an input file (DI-012 / NFR-006 — never
    // mutate inputs). If --output resolves to an existing path equal to --policy
    // or --flows, the end-of-run rename would silently destroy that input
    // (P5-IO-006). canonicalize() succeeds only if the path already exists, so a
    // brand-new --output (which cannot be an input) is unaffected.
    if let Some(out) = &args.output {
        if let Ok(out_c) = out.canonicalize() {
            for input in [&args.policy, &args.flows] {
                if input.canonicalize().ok().as_ref() == Some(&out_c) {
                    return Err(ZonewardenError::Io(IoError::OutputWrite {
                        path: out.display().to_string(),
                        detail: "output path is also an input file; refusing to overwrite an input"
                            .to_string(),
                    }));
                }
            }
        }
    }

    // Load + validate policy (ST-1/ST-2).
    let parsed = policy::load(&args.policy)?;
    let validated = validator::validate(parsed)?;

    // Open the flow stream (ST-3).
    let adapter = ZeekAdapter::open(&args.flows)?.with_max_flows(max_flows);

    // Classify each flow (ST-4..ST-6).
    let ctx = ClassifyCtx { policy: &validated };
    let mut items: Vec<(Verdict, Vec<Violation>)> = Vec::new();
    let mut skipped: u64 = 0;
    for record in adapter {
        match record {
            Ok(flow) => {
                let dst_kind = multicast::classify_dst(flow.dst_ip, &validated.prefix_index);
                let pair =
                    resolver::resolve_pair(&validated.prefix_index, flow.src_ip, flow.dst_ip);
                let verdict = classifier::classify(&ctx, &flow, &pair, dst_kind);
                let violations = classifier::violations_for(&flow, &pair, &verdict);
                items.push((verdict, violations));
            }
            // Per-record parse errors are skipped + counted (DI-013).
            Err(IngestError::Parse(_)) => skipped += 1,
            // A fatal limit (ingest cap) aborts the run with exit 2 (BC-1.02.006).
            Err(IngestError::Sys(e)) => return Err(ZonewardenError::Sys(e)),
        }
    }

    // Warnings: policy-level (zero-member, short-prefix) + the aggregate skipped
    // notice (BC-1.02.002 — one warning, not N).
    let mut warnings = validated.warnings.clone();
    if skipped > 0 {
        warnings.push(format!(
            "{skipped} flow records skipped (malformed/unusable)"
        ));
    }

    // Aggregate (ST-7).
    let result = aggregator::aggregate(items, &validated, skipped, warnings)?;

    // Warnings to stderr (BC-1.06.005) — never affects the exit code.
    let _ = reporter::emit_warnings(&result.warnings, &mut io::stderr().lock());

    // Report to stdout or an atomic file (ST-8).
    emit_report(args, &result, &validated)?;

    Ok(exit_status(&result, args.fail_on_skipped))
}

fn emit_report(
    args: &CliArgs,
    result: &ConformanceResult,
    validated: &ValidatedPolicy,
) -> Result<(), ZonewardenError> {
    let render = |w: &mut dyn Write| -> io::Result<()> {
        match args.format {
            Format::Text => reporter::emit_text(result, w),
            Format::Json => reporter::emit_json(result, w),
            Format::Mermaid => reporter::emit_mermaid(result, validated, w),
        }
    };

    match &args.output {
        Some(path) => reporter::emit_to_file(path, render)?,
        None => {
            let mut stdout = io::stdout().lock();
            render(&mut stdout).map_err(|e| {
                ZonewardenError::Io(IoError::OutputWrite {
                    path: "<stdout>".to_string(),
                    detail: e.to_string(),
                })
            })?;
        }
    }
    Ok(())
}
