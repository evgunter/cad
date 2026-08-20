//! The Probe-lane half of the M5 S2 review probes (arc-carrier fillet
//! corners): the F3 finding — k_stats gate sequence invariance within a
//! corner class, driven through the §2c fused family.
//!
//! Split out because `Probe` — the K-telemetry recording scalar — is
//! gated behind the `probe` cargo feature: it is a `Real` instantiation,
//! so every generic-over-`Real` body monomorphizes at it, and the
//! default build has no reason to pay for a diagnostics scalar; only
//! this file carries the whole-file gate.
//!
//! A corner is DERIVED here, so a carrier pair that meets twice
//! contributes both roots' gate blocks. The invariance claim is
//! unaffected — it is about the sequence being the same for the same
//! corner CLASS, not about its length.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point2;
use profile::{ArcSweep, Center, Open};

/// F3: the recorded predicate-name sequence must be data-independent
/// within a corner class (fixed classification order, all four
/// per-candidate gates always fired).
///
/// Both rows leave the tip ON the arrival carrier rather than closing:
/// the gate sequence is the fused verb's own, and a closing segment
/// would only add predicates from a different door.
#[test]
fn gate_sequence_is_data_independent_within_a_class() {
    use geom_core::Real;
    use geom_core::k_stats::{self, Probe};

    let pp = |x: f64, y: f64| Point2::new(Probe::from_f64(x), Probe::from_f64(y));
    let pr = Probe::from_f64;
    // The incoming ray leaves the origin along +x; the arrival carrier
    // is about (cx, 0) through the point below it, so the corner (2, 0)
    // is derived rather than authored.
    let seq_line_arc = |cx: f64, r: f64| {
        k_stats::start_recording();
        Open.at(pp(0.0, 0.0))
            .toward(pr(1.0), pr(0.0))
            .expect("the incoming ray runs +x")
            .fillet_arc(
                pr(r),
                Center {
                    c: pp(cx, 0.0),
                    winding: ArcSweep::Ccw,
                    p: pp(cx, -(cx - 2.0).abs()),
                },
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
    let seq_arc_arc = |r: f64| {
        k_stats::start_recording();
        Open.arc_fillet_arc(
            Center {
                c: pp(-1.0, 0.0),
                winding: ArcSweep::Ccw,
                p: pp(1.0, 0.0),
            },
            pr(r),
            Center {
                c: pp(1.0, 0.0),
                winding: ArcSweep::Ccw,
                p: pp(-1.0, 0.0),
            },
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
    // The last derived corner's per-candidate block: reach, reach,
    // fit, fit — all four, both candidates, no short-circuit.
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
