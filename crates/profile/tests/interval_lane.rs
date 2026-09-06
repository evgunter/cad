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

use crate::common;

use common::{annulus, lift, near_tangent_hole, profile, rect, tangent_hole, tol};
use geom_core::Tol;
use geom_core::{Interval, MarginDiag, Real, Sign};
use profile::{LoopRole, ProfileError, RawLoop, SegmentKind};

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
    let v0 = vp.loops()[0].vertices()[0].pos();
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
    let eps = tol().eps();
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
        near_tangent_hole(tol().eps()),
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

    let undeclared = common::bracket().with_tangent_joints(Vec::new());
    // joint 1 is a corner
    let contradicted = common::bracket().with_tangent_joints(vec![1, 3, 4]);
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
        .line_to(ip2(0.0, 0.0), Tol::witness())
        .expect("the straight run down to the ray's origin")
        .toward(one(1.0), one(0.0), Tol::witness())
        .expect("the incoming ray runs +x")
        .fillet_arc(
            one(0.5),
            profile::Center {
                c: ip2(0.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: profile::Start,
            },
            Tol::witness(),
        )
        .expect("the arc-carrier fillet constructs at Interval")
        .loop_;
    assert_eq!(lp.tangent_joints(), [2, 3]);
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
        .line_to(ip2(0.0, 0.0), Tol::witness())
        .expect("the straight run down to the ray's origin")
        .toward(one(1.0), one(0.0), Tol::witness())
        .expect("the incoming ray runs +x")
        .fillet_arc(
            one(1.0),
            profile::Center {
                c: ip2(0.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: profile::Start,
            },
            Tol::witness(),
        )
        .expect_err("the knife-edge fit must escalate at Interval");
    match err {
        profile::PathError::Escalated { ref source } => {
            assert_eq!(source.predicate, Some("fillet_leg_fit"), "{err}");
        }
        other => panic!("expected an escalation, got {other:?}"),
    }
}

/// The S8 two-survivor vesica corner at the interval scalar — **the
/// near pick is recovered, and it encloses the `f64` lane's**.
///
/// The §2c door DERIVES its corners from the two carriers, and a lens'
/// two carriers cross at BOTH tips — so the entry anchor is itself one
/// of the derived candidates and its advance along the incoming carrier
/// is exactly zero. `f64` classifies that Zero (not ahead, discarded);
/// an enclosure of it straddles zero, which is the configuration the
/// signed sweep's fold has to survive. It does: the fold reduces the
/// raw difference once, into the window centred on zero, so a hairline
/// difference stays a hairline and the gate classifies Zero on the
/// enclosure too. The remaining candidate is picked and the loop
/// validates.
///
/// The row asserts the two lanes AGREE, which is the claim worth
/// making: the enclosure of each emitted coordinate contains the `f64`
/// lane's own answer for the same authored numbers. It consults no
/// tolerance of its own — containment is exact arithmetic on the
/// emitted bounds.
#[test]
fn vesica_near_pick_agrees_with_the_f64_lane_at_interval() {
    use geom_core::Bounds;
    let s3 = 3.0f64.sqrt();
    let iv = profile::Open
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
            Tol::witness(),
        )
        .expect("the coincident candidate classifies Zero on its enclosure too");
    let f = profile::Open
        .arc_fillet_arc(
            profile::Center {
                c: common::p2(-1.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: common::p2(0.0, -s3),
            },
            0.5f64,
            profile::Center {
                c: common::p2(1.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: profile::Start,
            },
            Tol::witness(),
        )
        .expect("the f64 twin picks the near candidate");
    assert_eq!(
        iv.loop_.vertices().len(),
        f.loop_.vertices().len(),
        "the two lanes must emit the same pocket"
    );
    for (k, (a, b)) in f
        .loop_
        .vertices()
        .iter()
        .zip(iv.loop_.vertices())
        .enumerate()
    {
        for (what, exact, enc) in [
            ("x", a.pos().x, b.pos().x),
            ("y", a.pos().y, b.pos().y),
            ("bulge", a.bulge(), b.bulge()),
        ] {
            assert!(
                enc.lo() <= exact && exact <= enc.hi(),
                "vertex {k}: the {what} enclosure [{}, {}] excludes the f64 lane's {exact}",
                enc.lo(),
                enc.hi()
            );
        }
    }
}
