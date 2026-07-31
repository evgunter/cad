//! Interval-scalar validation (feature `interval`): the Q1 story end to
//! end — exact fixtures decide definitely from point enclosures, and a
//! near-tangent profile escalates through an enclosure lying wholly
//! inside the sliver band (the subdivision-terminal case).
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

// ------------------------------- arc-leg fillet corners at the interval
//
// The exact-order predicates `fillet_leg_fit` / `fillet_leg_reach` have
// no in-band row at `f64` — no representable `f64` lies strictly inside
// the hairline band — so their third trio row lives here, where an
// enclosure straddling the hairline escalates honestly (M5 S2).

/// An `Interval` sketch point from exact `f64` coordinates.
fn ip2(x: f64, y: f64) -> geom_core::Point2<Interval> {
    geom_core::Point2::new(Interval::from_f64(x), Interval::from_f64(y))
}

fn iarc(cx: f64, cy: f64, sweep: profile::ArcSweep) -> profile::FilletLegShape<Interval> {
    profile::FilletLegShape::Arc {
        center: ip2(cx, cy),
        sweep,
    }
}

/// The line×arc fillet corner built directly at `Interval`: the
/// definite rows of every gate decide from enclosures, and the declared
/// tangencies verify (the residual is zero by construction in ℝ, and its
/// enclosure is narrow enough to classify Zero).
#[test]
fn arc_leg_fillet_constructs_and_validates_at_interval() {
    let lp = profile::ProfileLoop::builder(ip2(0.0, 0.0))
        .fillet_corner(
            profile::FilletLegShape::Line,
            ip2(2.0, 0.0),
            iarc(0.0, 0.0, profile::ArcSweep::Ccw),
            ip2(0.0, 2.0),
            Interval::from_f64(0.5),
            tol(),
        )
        .expect("the arc-leg fillet constructs at Interval")
        .arc_to_center(ip2(0.0, 2.0), ip2(0.0, 0.0), profile::ArcSweep::Ccw)
        .close();
    assert_eq!(lp.tangent_joints, vec![1, 2]);
    profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the arc-leg fillet validates at Interval");
}

/// `fillet_leg_fit`, in-band row: at r = 1 both legs are consumed
/// EXACTLY (a bit-zero margin at `f64`), so at the interval scalar the
/// margin's enclosure straddles the hairline and the gate escalates
/// rather than claiming an exact fit it cannot certify.
#[test]
fn exact_fit_arc_fillet_escalates_at_interval() {
    let err = profile::ProfileLoop::builder(ip2(0.0, 0.0))
        .fillet_corner(
            profile::FilletLegShape::Line,
            ip2(2.0, 0.0),
            iarc(0.0, 0.0, profile::ArcSweep::Ccw),
            ip2(0.0, 2.0),
            Interval::from_f64(1.0),
            tol(),
        )
        .expect_err("the knife-edge fit must escalate at Interval");
    match err {
        ProfileError::Escalated {
            site: profile::EscalationSite::Fillet,
            ref source,
        } => assert_eq!(source.predicate, Some("fillet_leg_fit"), "{err}"),
        other => panic!("expected a fillet-site escalation, got {other:?}"),
    }
    // The escalation renders the SAME recourse sentence as the definite
    // `FilletDoesNotFit` refusal (one situation, one message: D4 ¶1's
    // two-tolerance addendum).
    assert!(
        err.to_string().contains("smaller radius or longer legs"),
        "recourse: {err}"
    );
}

/// `fillet_leg_reach`, in-band row: r = 0 puts both tangent points ON
/// the corner (a bit-zero setback at `f64`); at the interval scalar the
/// arc leg's setback encloses across the periodic wrap and the gate
/// escalates instead of deciding the corner-side question.
#[test]
fn zero_radius_arc_fillet_escalates_at_interval() {
    let err = profile::ProfileLoop::builder(ip2(0.0, 0.0))
        .fillet_corner(
            profile::FilletLegShape::Line,
            ip2(2.0, 0.0),
            iarc(0.0, 0.0, profile::ArcSweep::Ccw),
            ip2(0.0, 2.0),
            Interval::from_f64(0.0),
            tol(),
        )
        .expect_err("the zero-radius knife edge must escalate at Interval");
    match err {
        ProfileError::Escalated {
            site: profile::EscalationSite::Fillet,
            ref source,
        } => assert_eq!(source.predicate, Some("fillet_leg_reach"), "{err}"),
        other => panic!("expected a fillet-site escalation, got {other:?}"),
    }
    // Trio parity (review MINOR-1): `fillet_leg_reach`'s situation is
    // whether a corner of this radius exists on the corner side, and its
    // definite refusal is `NoCornerForFillet` — so the in-band row must
    // render the no-corner recourse, NOT the radius-does-not-fit one.
    let text = err.to_string();
    assert!(text.contains("can sit in the corner"), "recourse: {text}");
    assert!(
        !text.contains("longer legs"),
        "reach must not render the fit recourse: {text}"
    );
}

/// The S8 two-survivor vesica corner at the interval scalar: the
/// nearest-corner selection is a diagnostic-channel choice (enclosure
/// lower bounds), so this lane picks the SAME near candidate as the
/// f64 lane — `symmetric_lens_pick_is_bit_deterministic_across_runs`'s
/// cross-lane half.
#[test]
fn vesica_near_pick_agrees_at_interval() {
    let s3 = 3.0f64.sqrt();
    let lp = profile::ProfileLoop::builder(ip2(0.0, -s3))
        .fillet_corner(
            iarc(-1.0, 0.0, profile::ArcSweep::Ccw),
            ip2(0.0, s3),
            iarc(1.0, 0.0, profile::ArcSweep::Ccw),
            ip2(0.0, -s3),
            Interval::from_f64(0.5),
            tol(),
        )
        .expect("the two-survivor vesica corner resolves at Interval")
        .close_arc_center(ip2(1.0, 0.0), profile::ArcSweep::Ccw);
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
