# Security Policy

## Reporting a vulnerability

Please report security-relevant bugs privately via **GitHub's private
vulnerability reporting**:

https://github.com/adamson34/zonewarden/security/advisories/new

Do **not** open a public issue for a security bug.

## In scope

zonewarden runs fully offline on captured files (a YAML policy and a flow log).
The threat model is therefore primarily about handling **untrusted input files**
safely. In scope:

- Memory-safety or panics on crafted policy / flow input (the crate is
  `#![forbid(unsafe_code)]`; the parsers are fuzzed).
- Denial-of-service on untrusted input (unbounded memory/CPU) beyond the
  documented ingest cap (`--max-flows`).
- YAML expansion / "billion laughs" style resource exhaustion.
- Supply-chain issues in dependencies (tracked via `cargo deny` and Dependabot).
- Incorrect conformance verdicts that could cause a real segmentation violation
  to be silently reported as allowed (a correctness issue with security impact).

## Out of scope

- False positives in verdicts, usability, and feature requests — use the bug or
  feature issue templates.
- Anything requiring the operator to run zonewarden against input they don't
  trust *and* act on output without review.

## Supported versions

Pre-1.0: only the latest release on the default branch is supported.
