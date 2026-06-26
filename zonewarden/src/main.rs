//! `zonewarden` CLI entry point (effectful shell, ADR-002). Thin wrapper over
//! [`zonewarden::cli`]: parse args, run the pipeline, map the outcome to a
//! process exit code (0 conformant / 1 violations / 2 error — BC-1.06.001).

use std::process::ExitCode;

use clap::Parser;

use zonewarden::cli::{self, CliArgs};

fn main() -> ExitCode {
    let args = CliArgs::parse();
    match cli::run(&args) {
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(err.exit_code())
        }
    }
}
