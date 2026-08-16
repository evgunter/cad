//! Interval-scalar validation (feature `interval`): the Q1 story end to
//! end — exact fixtures decide definitely from point enclosures, and a
//! near-tangent profile escalates through an enclosure lying wholly
//! inside the sliver band (the subdivision-terminal case).
//!
//! The second half runs the §2c fused fillet family at `Interval`: an
//! arc-carrier corner whose gates all decide from enclosures, the
//! knife-edge fit whose enclosure straddles the hairline and escalates,
//! and the two-survivor vesica whose pick agrees with the f64 lane.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{annulus, lift, near_tangent_hole, profile, rect, tangent_hole, tol};
use geom_core::{Interval, MarginDiag, Real, Sign};
use profile::{LoopRole, ProfileError, SegmentKind};

#[test]
fn rectangle_validates_at_interval() {
    let vp = lift::<Interval>(&profile(vec![rect(0.0, 0.0, 2.0, 2.0)]))
        .validate(tol())
        .expect("rectangle must validate at Interval");
    assert_eq!(vp.loops().len(), 1);
    assert_eq!(vp.loops()[0].role(), LoopRole::Outer);
    assert!(
        vp.loops()[0]
            .segments()
            .iter()
            .all(|s| matches!(s.kind, SegmentKind::Line))
    );
    // The canonical start's enclosure is the exact point (0, 0).
    let v0 = vp.loops()[0].vertices()[0].pos;
    use geom_core::Bounds;
    assert_eq!((v0.x.lo(), v0.x.hi()), (0.0, 0.0));
    assert_eq!((v0.y.lo(), v0.y.hi()), (0.0, 0.0));
}

#[test]
fn annulus_validates_at_interval_with_roles_and_turns() {
    let vp = lift::<Interval>(&annulus())
        .validate(tol())
        .expect("annulus must validate at Interval");
    assert_eq!(vp.loops()[0].role(), LoopRole::Outer);
    assert_eq!(vp.loops()[1].role(), LoopRole::Hole);
    for (li, want) in [(0usize, Sign::Positive), (1, Sign::Negative)] {
        for s in vp.loops()[li].segments() {
            match s.kind {
                SegmentKind::Arc { turn, .. } => assert_eq!(turn, want),
                SegmentKind::Line => panic!("annulus segments are arcs"),
            }
        }
    }
}

#[test]
fn exact_tangency_is_the_same_typed_error_at_interval() {
    let err = lift::<Interval>(&tangent_hole())
        .validate(tol())
        .expect_err("tangency must be refused at Interval");
    assert!(
        matches!(err, ProfileError::TangentialContact { .. }),
        "got {err:?}"
    );
}

#[test]
fn near_tangency_escalates_via_an_in_band_enclosure() {
    // The internal clearance is −5ε: at Interval the margin enclosure
    // lies wholly inside the open sliver band — the terminal case the
    // subdivision driver cannot refine (MarginDiag::Enclosure carries
    // the bounds; escalation is the only sound outcome, Q1).
    let eps = tol().eps;
    let err = lift::<Interval>(&near_tangent_hole(eps))
        .validate(tol())
        .expect_err("near-tangency must escalate at Interval");
    match err {
        ProfileError::Escalated { source, .. } => {
            assert_eq!(source.predicate, Some("carrier_circles_internal"));
            match source.margin {
                MarginDiag::Enclosure { lo, hi } => {
                    assert!(
                        lo > -10.0 * eps && hi < -eps,
                        "enclosure [{lo:e}, {hi:e}] should sit inside the negative band"
                    );
                }
                other => panic!("expected an enclosure diagnostic, got {other:?}"),
            }
        }
        other => panic!("expected escalation, got {other:?}"),
    }
}

#[test]
fn interval_decisions_match_f64_on_the_fixture_suite() {
    // Value-channel agreement one level up: accept/reject and the
    // error variant agree between f64 and point-enclosure Interval.
    let fixtures: Vec<profile::Profile<f64>> = vec![
        profile(vec![rect(0.0, 0.0, 2.0, 2.0)]),
        annulus(),
        tangent_hole(),
        near_tangent_hole(tol().eps),
    ];
    for p in fixtures {
        let at_f64 = p.validate(tol());
        let at_interval = lift::<Interval>(&p).validate(tol());
        match (at_f64, at_interval) {
            (Ok(_), Ok(_)) => {}
            (Err(a), Err(b)) => {
                assert_eq!(
                    core::mem::discriminant(&a),
                    core::mem::discriminant(&b),
                    "f64: {a:?}, interval: {b:?}"
                );
            }
            (a, b) => panic!("divergent outcomes: f64 {a:?} vs interval {b:?}"),
        }
    }
}

#[test]
fn declared_tangency_discipline_holds_at_interval() {
    // The #101 discipline at the interval scalar: the fillet-authored
    // bracket's rounding-level tangency margins enclose inside the
    // Zero region (definite Zero, no escalation), and the discipline's
    // refusals agree with f64 variant-for-variant.
    let declared = profile(vec![common::bracket()]);
    lift::<Interval>(&declared)
        .validate(tol())
        .expect("declared bracket validates at Interval");

    let mut undeclared = common::bracket();
    undeclared.tangent_joints.clear();
    let mut contradicted = common::bracket();
    contradicted.tangent_joints = vec![1, 3, 4]; // joint 1 is a corner
    for lp in [undeclared, contradicted] {
        let p = profile(vec![lp]);
        let at_f64 = p.validate(tol()).expect_err("must refuse at f64");
        let at_iv = lift::<Interval>(&p)
            .validate(tol())
            .expect_err("must refuse at Interval");
        assert_eq!(
            core::mem::discriminant(&at_f64),
            core::mem::discriminant(&at_iv),
            "f64: {at_f64:?}, interval: {at_iv:?}"
        );
    }
}

// ------------------------------- arc-carrier fillet corners at the interval
//
// The exact-order predicate `fillet_leg_fit` has no in-band row at
// `f64` — no representable `f64` lies strictly inside the hairline band
// — so its third trio row lives here, where an enclosure straddling the
// hairline escalates honestly (M5 S2).

/// An `Interval` sketch point from exact `f64` coordinates.
fn ip2(x: f64, y: f64) -> geom_core::Point2<Interval> {
    geom_core::Point2::new(Interval::from_f64(x), Interval::from_f64(y))
}

/// The line×arc fillet corner built directly at `Interval`: the
/// definite rows of every gate decide from enclosures, and the declared
/// tangencies verify (the residual is zero by construction in ℝ, and its
/// enclosure is narrow enough to classify Zero).
///
/// The chain is the loop rotated so the fillet CLOSES it: the entry
/// (0, 2) is the arrival side's own anchor, the straight run reaches
/// (0, 0), the incoming ray leaves it toward +x, and the arc arrival
/// about the origin closes back onto the entry vertex. The corner
/// (2, 0) is the ray meeting that carrier, never authored.
#[test]
fn arc_leg_fillet_constructs_and_validates_at_interval() {
    let one = |v: f64| Interval::from_f64(v);
    let lp = profile::Open
        .at(ip2(0.0, 2.0))
        .line_to(ip2(0.0, 0.0))
        .expect("the straight run down to the ray's origin")
        .toward(one(1.0), one(0.0))
        .expect("the incoming ray runs +x")
        .fillet_arc(
            one(0.5),
            profile::Center {
                c: ip2(0.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: profile::Start,
            },
        )
        .expect("the arc-carrier fillet constructs at Interval")
        .loop_;
    assert_eq!(lp.tangent_joints, vec![2, 3]);
    profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the arc-leg fillet validates at Interval");
}

/// `fillet_leg_fit`, in-band row: at r = 1 both sides are consumed
/// EXACTLY (a bit-zero margin at `f64`), so at the interval scalar the
/// margin's enclosure straddles the hairline and the gate escalates
/// rather than claiming an exact fit it cannot certify.
#[test]
fn exact_fit_arc_fillet_escalates_at_interval() {
    let one = |v: f64| Interval::from_f64(v);
    let err = profile::Open
        .at(ip2(0.0, 2.0))
        .line_to(ip2(0.0, 0.0))
        .expect("the straight run down to the ray's origin")
        .toward(one(1.0), one(0.0))
        .expect("the incoming ray runs +x")
        .fillet_arc(
            one(1.0),
            profile::Center {
                c: ip2(0.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: profile::Start,
            },
        )
        .expect_err("the knife-edge fit must escalate at Interval");
    match err {
        profile::PathError::Escalated { ref source } => {
            assert_eq!(source.predicate, Some("fillet_leg_fit"), "{err}");
        }
        other => panic!("expected an escalation, got {other:?}"),
    }
}

/// The S8 two-survivor vesica corner at the interval scalar: the
/// nearest-corner selection is a diagnostic-channel choice (enclosure
/// lower bounds), and here the survivors' setback gap is macroscopic
/// next to the enclosure width, so this lane picks the SAME near
/// candidate as the f64 lane —
/// `symmetric_lens_pick_is_bit_deterministic_across_runs`'s cross-lane
/// half.
///
/// Both sides are arcs and the loop has two vertices' worth of authored
/// content, so the whole corner is ONE fused act at the entry: the
/// incoming carrier about (−1, 0) anchored at (0, −√3), and the arrival
/// carrier about (1, 0) closing back onto that same entry vertex.
#[test]
fn vesica_near_pick_agrees_at_interval() {
    let s3 = 3.0f64.sqrt();
    let lp = profile::Open
        .arc_fillet_arc(
            profile::Center {
                c: ip2(-1.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: ip2(0.0, -s3),
            },
            Interval::from_f64(0.5),
            profile::Center {
                c: ip2(1.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: profile::Start,
            },
        )
        .expect("the two-survivor vesica corner resolves at Interval")
        .loop_;
    assert_eq!(lp.tangent_joints, vec![1, 2]);
    use geom_core::Bounds;
    // The near (top-pocket) candidate: both tangent points above the
    // lens' waist, exactly as the f64 row asserts.
    assert!(lp.vertices[1].pos.y.lo() > 0.0);
    assert!(lp.vertices[2].pos.y.lo() > 0.0);
    profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the near-pick vesica validates at Interval");
}
