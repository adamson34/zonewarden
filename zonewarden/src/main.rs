//! `zonewarden` CLI — the effectful shell (ADR-002). Stub for Wave 1; the full
//! command surface (`validate --policy --flows ...`) is implemented in Wave 5
//! (S-6.03). Exit code 2 = usage/not-implemented, per the ST-8 exit-code model.

use std::process::ExitCode;

fn main() -> ExitCode {
    eprintln!("zonewarden: not yet implemented (Wave 1 scaffold). See the roadmap in README.md.");
    ExitCode::from(2)
}
