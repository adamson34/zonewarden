#![no_main]
//! Fuzz the Zeek conn.log streaming parser (WAVE2-003 / DI-013). Arbitrary bytes
//! must never panic: every line is either normalized, skipped-and-counted, or
//! ends the stream — and the per-line read is bounded (P5-IO-001).

use libfuzzer_sys::fuzz_target;
use zonewarden::adapters::zeek::ZeekAdapter;

fuzz_target!(|data: &[u8]| {
    let adapter = ZeekAdapter::from_reader(data);
    for item in adapter {
        // Parse errors (Err) are expected on garbage; a panic is a bug.
        let _ = item;
    }
});
