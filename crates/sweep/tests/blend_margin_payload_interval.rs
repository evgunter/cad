//! The blend battery's definite refusals at the CERTIFIED scalar
//! (feature `interval`): what the margin payload says when the margin
//! is an enclosure.
//!
//! At `f64` a margin is one number and a refusal can report it as one
//! number without lying. At the interval scalar it is an ENCLOSURE, and
//! a payload that reports one endpoint of it says something the
//! classifier never saw — the shape this lane exists to catch. The rows
//! below take one predicate through each of its two definite outcomes
//! at `T = Interval` and read the payload back. Both report as the
//! ENCLOSURE they are — a thin one included, because that is what this
//! scalar's own `sign_within` calls a point bracket (E3 review item 1:
//! the spelling is the scalar's, not the bracket's width's), and a
//! wide one because no single endpoint of it is a number anything
//! measured.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

/// **Loud skip.** Without `--features interval` this binary is empty;
/// announce the skip so a lane that silently lost its certified rows
/// stays visible in the battery log.
#[cfg(not(feature = "interval"))]
#[test]
fn interval_lane_skipped_no_certified_coverage_here() {
    println!(
        "SKIPPED (no --features interval): blend_margin_payload_interval.rs \
         contributes NO certified coverage in this run — the enclosure arm \
         of the blend refusals' margin payload runs only in the interval lane."
    );
}

#[cfg(feature = "interval")]
mod certified {
    use geom_core::{Band, Interval, MarginDiag, Real, Sign, Tol};
    use sweep::blend::BlendError;
    use sweep::blend::battery::spine_regularity;
    use sweep::blend::surgery::ring_clearance_for_tests as ring_clearance;
    use topo::FaceKey;

    fn band() -> Band {
        let tol = Tol::witness();
        Band::new(tol.eps(), tol.k() * tol.eps()).unwrap()
    }

    /// A THIN enclosure — a point bracket, which is what an exact
    /// datum is at this scalar — still reports as an ENCLOSURE, with
    /// `lo == hi`.
    ///
    /// This is the scalar's own spelling, not a width test:
    /// `Interval::sign_within` reports a point bracket it cannot
    /// classify as `Enclosure { lo: m, hi: m }` while `f64`'s reports
    /// it as `Value(m)`, and a definite payload has to say what its
    /// own scalar would have said or the two halves of one predicate's
    /// vocabulary disagree about one reading (E3 review item 1, and
    /// `review_fillet_e3_probes`'s first row is the same property from
    /// the other side).
    #[test]
    fn a_point_bracket_margin_reports_as_a_thin_enclosure() {
        let err = ring_clearance(FaceKey::default(), Interval::from_f64(-0.05), band())
            .expect_err("a ring inside the trimline refuses");
        match err {
            BlendError::RingClearance { margin, .. } => {
                assert_eq!(margin.predicate, "fillet3_ring_clearance");
                assert_eq!(margin.sign, Sign::Negative);
                assert_eq!(
                    margin.reading,
                    MarginDiag::Enclosure {
                        lo: -0.05,
                        hi: -0.05
                    },
                    "the interval scalar spells a point bracket as a thin enclosure"
                );
                assert_eq!(
                    margin.value(),
                    None,
                    "and the accessor answers for the shape, not for the width"
                );
            }
            other => panic!("expected RingClearance, got {other}"),
        }
    }

    /// A WIDE enclosure reports as the bracket it is. This is the row
    /// the projection could not carry: `lo()` alone would have said
    /// `-0.2` — a number the classifier never held — and would have
    /// been indistinguishable from a point margin of `-0.2`.
    #[test]
    fn a_wide_enclosure_margin_reports_as_an_enclosure_and_not_an_endpoint() {
        let wide = Interval::from_bounds(-0.2, -0.05);
        let err = ring_clearance(FaceKey::default(), wide, band())
            .expect_err("an enclosure wholly below zero refuses definitely");
        match err {
            BlendError::RingClearance { margin, .. } => {
                assert_eq!(margin.sign, Sign::Negative);
                assert_eq!(
                    margin.reading,
                    MarginDiag::Enclosure {
                        lo: -0.2,
                        hi: -0.05
                    },
                    "the payload is the enclosure the classifier judged"
                );
                assert_eq!(
                    margin.value(),
                    None,
                    "no single number is this reading, and the accessor says so"
                );
                let text = margin.to_string();
                assert!(
                    text.contains("enclosure [-2e-1, -5e-2]"),
                    "the rendering names the bracket: {text}"
                );
            }
            other => panic!("expected RingClearance, got {other}"),
        }
    }

    /// The same two shapes reach a second predicate's payload, so the
    /// property is the payload type's and not one door's: a spine whose
    /// curvature is an enclosure refuses with the enclosure.
    #[test]
    fn a_second_predicate_carries_the_same_enclosure_shape() {
        // `(1 − r·κ)·r` with r = 0.5 exactly and κ an enclosure around
        // 4 — definitely past the fold, and definitely wide.
        let kappa = Interval::from_bounds(3.0, 5.0);
        let err = spine_regularity(kappa, Interval::from_f64(0.5), band())
            .expect_err("a spine curving faster than 1/r folds");
        match err {
            BlendError::SpineIrregular { margin, radius } => {
                assert_eq!(margin.predicate, "fillet3_spine_regularity");
                assert_eq!(margin.sign, Sign::Negative);
                assert!(
                    matches!(margin.reading, MarginDiag::Enclosure { .. }),
                    "a levered enclosure stays one: {:?}",
                    margin.reading
                );
                assert_eq!(margin.value(), None);
                // The companion lever arm is exact here, so the row
                // says nothing about the radius field's own projection.
                assert!((radius - 0.5).abs() < 1e-15);
            }
            other => panic!("expected SpineIrregular, got {other}"),
        }
    }
}
