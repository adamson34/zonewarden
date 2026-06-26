//! `zonewarden` — the effectful shell (ADR-002). Owns I/O and wire formats
//! (YAML policy loading, flow ingest, reporting). The `main` binary is a thin
//! wrapper over these modules; integration tests exercise them directly.

pub mod adapters;
pub mod cli;
pub mod policy;
pub mod reporter;
