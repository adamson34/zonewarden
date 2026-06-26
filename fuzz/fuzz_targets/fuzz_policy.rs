#![no_main]
//! Fuzz the YAML policy parser (load_str → DTO → core Policy). Arbitrary UTF-8
//! input must never panic: it parses to a Policy or returns a typed error.

use libfuzzer_sys::fuzz_target;
use zonewarden::policy::load_str;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let _ = load_str(text); // Ok or Err, never a panic
    }
});
