//! The Probe-lane half of `review_s2.rs` (adversarial e2e review probes
//! for M5 S2, arc-leg fillet sugar): the F3 finding — k_stats gate
//! sequence invariance within a corner class.
//!
//! Split out because `Probe` — the K-telemetry recording scalar — is
//! gated behind the `probe` cargo feature: it is a `Real` instantiation,
//! so every generic-over-`Real` body monomorphizes at it, and the
//! default build has no reason to pay for a diagnostics scalar. The
//! rest of the review suite (the F1/F2 fuzz, the MAJOR-1 attribution
//! pin, the enclosing case) is f64 and stays ungated in `review_s2.rs`;
//! only this file carries the whole-file gate.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::tol;
use geom_core::Point2;
use profile::{ArcSweep, FilletLegShape, ProfileLoop};

/// F3: the recorded predicate-name sequence must be data-independent
/// within a corner class (fixed classification order, all four
/// per-candidate gates always fired).
#[test]
fn gate_sequence_is_data_independent_within_a_class() {
    use geom_core::Real;
    use profile::k_stats::{self, Probe};

    let pp = |x: f64, y: f64| Point2::new(Probe::from_f64(x), Probe::from_f64(y));
    let seq_line_arc = |cx: f64, r: f64| {
        k_stats::start_recording();
        ProfileLoop::builder(pp(0.0, 0.0))
            .fillet_corner(
                FilletLegShape::Line,
                pp(2.0, 0.0),
                FilletLegShape::Arc {
                    center: pp(cx, 0.0),
                    sweep: ArcSweep::Ccw,
                },
                pp(cx, -(cx - 2.0).abs()),
                Probe::from_f64(r),
                tol(),
            )
            .expect("constructs");
        k_stats::take_samples()
            .iter()
            .map(|s| s.predicate)
            .collect::<Vec<_>>()
    };
    let a = seq_line_arc(3.0, 0.5);
    let b = seq_line_arc(3.5, 0.25);
    assert_eq!(a, b, "line-by-arc gate sequence varies with data");
    let s3 = 3.0f64.sqrt();
    let seq_arc_arc = |r: f64| {
        k_stats::start_recording();
        ProfileLoop::builder(pp(1.0, 0.0))
            .fillet_corner(
                FilletLegShape::Arc {
                    center: pp(-1.0, 0.0),
                    sweep: ArcSweep::Ccw,
                },
                pp(0.0, s3),
                FilletLegShape::Arc {
                    center: pp(1.0, 0.0),
                    sweep: ArcSweep::Ccw,
                },
                pp(-1.0, 0.0),
                Probe::from_f64(r),
                tol(),
            )
            .expect("constructs");
        k_stats::take_samples()
            .iter()
            .map(|s| s.predicate)
            .collect::<Vec<_>>()
    };
    let c = seq_arc_arc(0.5);
    let d = seq_arc_arc(0.3);
    assert_eq!(c, d, "arc-by-arc gate sequence varies with data");
    // Per-candidate block: reach, reach, fit, fit — all four, both
    // candidates, no short-circuit.
    let tail: Vec<_> = c.iter().rev().take(8).rev().copied().collect();
    assert_eq!(
        tail,
        [
            "fillet_leg_reach",
            "fillet_leg_reach",
            "fillet_leg_fit",
            "fillet_leg_fit",
            "fillet_leg_reach",
            "fillet_leg_reach",
            "fillet_leg_fit",
            "fillet_leg_fit"
        ],
        "full sequence: {c:?}"
    );
}
