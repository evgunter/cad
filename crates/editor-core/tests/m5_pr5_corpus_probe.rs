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

/// The Probe lane (K-funnel registration): the document evaluates green
/// at the recording scalar, which is the probe sweep's precondition —
/// the new predicates' margins join the telemetry without a registration
/// step.
///
/// **Greenness is what this asserts; bit-identity is what it is reaching
/// for.** Nothing here compares a `Probe` result against an f64 one, and
/// greenness is tolerance-dependent, so the claim holds at the ε the run
/// used. `m4_pr8_k_probe`'s `run_doc` asserts the same predicate over
/// every corpus document — this one included — at three ε on every
/// merge; what this row adds is that this file executes.
#[test]
fn cut_cylinder_replays_at_probe() {
    let doc = corpus::cut_cylinder::document();
    let ev = corpus::eval::<geom_core::k_stats::Probe>(&doc.doc);
    assert_eq!(corpus::failures(&ev), Vec::<String>::new());
}
