//! The Probe-lane half of `m5_pr5_corpus.rs` (M5 PR 5 acceptance shape
//! (i), corpus form).
//!
//! Split out because `Probe` — the K-telemetry recording scalar — is
//! gated behind the `probe` cargo feature: it is a `Real` instantiation,
//! so every generic-over-`Real` body monomorphizes at it, and the
//! default build has no reason to pay for a diagnostics scalar. The
//! f64 pins stay ungated in `m5_pr5_corpus.rs`; only this file carries
//! the whole-file gate.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

/// The Probe lane (K-funnel registration): the document replays at the
/// recording scalar with bit-identical decisions — the probe sweep's
/// precondition, so the new predicates' margins join the telemetry
/// without any registration step.
#[test]
fn cut_cylinder_replays_at_probe() {
    let doc = corpus::cut_cylinder::document();
    let ev = corpus::eval::<geom_core::k_stats::Probe>(&doc.doc);
    assert_eq!(corpus::failures(&ev), Vec::<String>::new());
}
