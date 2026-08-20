//! The Probe-lane half of `validate_ok.rs`: the second-instantiation
//! smoke test of the accepting fixtures.
//!
//! Split out because `Probe` — the K-telemetry recording scalar — is
//! gated behind the `probe` cargo feature: it is a `Real` instantiation,
//! so every generic-over-`Real` body monomorphizes at it, and the
//! default build has no reason to pay for a diagnostics scalar. The f64
//! canonical-form pins stay ungated in `validate_ok.rs`; only this file
//! carries the whole-file gate.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{annulus, lift, tol};

#[test]
fn generic_instantiation_smoke_probe() {
    // The whole pipeline is generic over Decide: run it at the k-stats
    // Probe scalar (delegating wrapper) as a second instantiation.
    let p = lift::<geom_core::k_stats::Probe>(&annulus());
    assert!(p.validate(tol()).is_ok());
}
