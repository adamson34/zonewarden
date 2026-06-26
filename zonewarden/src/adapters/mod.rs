//! Reality-source adapters (effectful shell). Each adapter reads observed flows
//! from some external source and normalizes them into the pure-core `Flow` model
//! via the [`zonewarden_core::RealitySource`] trait. The v1 adapter is Zeek
//! `conn.log` ([`zeek`]).

pub mod zeek;
