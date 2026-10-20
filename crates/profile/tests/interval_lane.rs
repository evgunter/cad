//! Interval-scalar validation: the Q1 story end to
//! end — exact fixtures decide definitely from point enclosures, and a
//! near-tangent profile escalates through an enclosure lying wholly
//! inside the sliver band (the subdivision-terminal case).
//!
//! The second half runs the §2c fused fillet family at `Interval`: an
//! arc-carrier corner whose gates all decide from enclosures, the
//! knife-edge fit whose enclosure straddles the hairline and escalates,
//! and the two-survivor vesica whose pick agrees with the f64 lane.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{annulus, lift, near_tangent_hole, profile, rect, tangent_hole, tol};
use geom_core::{ErrorTextReading, Interval, Real, Sign};
use geom_core::{Point2, Tol};
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
    let v0 = vp.loops()[0].vertices()[0];
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
    // subdivision driver cannot refine (MarginKind::Enclosure carries
    // the bounds; escalation is the only sound outcome, Q1).
    let eps = tol().eps();
    let err = lift::<Interval>(&near_tangent_hole(eps))
        .validate(tol())
        .expect_err("near-tangency must escalate at Interval");
    match err {
        ProfileError::Escalated { source, .. } => {
            assert_eq!(source.predicate, Some("carrier_circles_internal"));
            match source.margin.diagnostic_f64_for_error_text() {
                ErrorTextReading::Enclosure { lo, hi } => {
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
fn tangent_joints_are_derived_alike_at_interval() {
    // D1's profile tangency at the interval scalar: the fillet-authored
    // bracket's rounding-level tangency margins enclose inside the Zero
    // region (definite Zero, no escalation), so as a table its two
    // joints are decided and recorded at both scalars alike, and a
    // contradicted construction refuses at both variant-for-variant.
    fn decided<T: Real>(vp: &profile::ValidatedProfile<T>) -> Vec<usize> {
        vp.loops()[0]
            .decided_joints()
            .iter()
            .map(|d| d.joint)
            .collect()
    }
    let table = profile(vec![common::bracket()]);
    let at_f64 = table.validate(tol()).expect("the bracket validates at f64");
    let at_iv = lift::<Interval>(&table)
        .validate(tol())
        .expect("the bracket validates at Interval");
    assert_eq!(decided(&at_f64), vec![3, 4]);
    assert_eq!(
        decided(&at_iv),
        decided(&at_f64),
        "one derived set at both scalars"
    );

    // Joint 1 is a corner.
    fn lie<T: Real>(lp: profile::ProfileLoop<T>) -> profile::ConstructedLoop<T> {
        profile::ConstructedLoop::fixture(lp, vec![1, 3, 4])
    }
    let at_f64 =
        profile::ConstructedProfile::new(profile::SketchPlane::xy(), vec![lie(common::bracket())])
            .validate(tol())
            .expect_err("must refuse at f64");
    let at_iv = profile::ConstructedProfile::new(
        profile::SketchPlane::xy(),
        vec![lie(common::bracket().map_scalar(Interval::from_f64))],
    )
    .validate(tol())
    .expect_err("must refuse at Interval");
    assert!(
        matches!(at_f64, ProfileError::TangencyContradicted { joint: 1, .. }),
        "f64: {at_f64:?}"
    );
    assert_eq!(at_iv, at_f64, "interval: {at_iv:?}");
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
    assert_eq!(lp.constructed_joints(), [2, 3]);
    let vp = profile::ConstructedProfile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the arc-leg fillet validates at Interval");
    assert!(
        vp.loops()[0].decided_joints().is_empty(),
        "both constructed"
    );
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
                c: Point2::new(-1.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: Point2::new(0.0, -s3),
            },
            0.5f64,
            profile::Center {
                c: Point2::new(1.0, 0.0),
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
        let mut channels = vec![("x", a.x, b.x), ("y", a.y, b.y)];
        if let (profile::Segment::Arc(e), profile::Segment::Arc(i)) =
            (f.loop_.segments()[k], iv.loop_.segments()[k])
        {
            channels.push(("sweep", e.sweep, i.sweep));
        }
        for (what, exact, enc) in channels {
            assert!(
                enc.lo() <= exact && exact <= enc.hi(),
                "vertex {k}: the {what} enclosure [{}, {}] excludes the f64 lane's {exact}",
                enc.lo(),
                enc.hi()
            );
        }
    }
}

/// **A canonical form's value channel is minted only where every value
/// is a point.** A quad validated at `Interval` with one corner a point
/// gives its `f64` form, value for value. Widen that corner, or the
/// plane's height, and the mint refuses by name, naming where, rather
/// than handing out a validated profile holding NaN.
#[test]
fn the_value_channel_refuses_a_value_with_no_single_point() {
    use geom_core::Point3;
    use profile::test_support::bulge_loop;
    use profile::{Profile, SketchPlane, ValueNotAPoint};
    let quad = |corner: Interval, z: Interval| {
        let p = |x: f64, y: f64| Point2::new(Interval::from_f64(x), Interval::from_f64(y));
        let lp = bulge_loop(vec![
            (p(0.0, 0.0), Interval::zero()),
            (p(2.0, 0.0), Interval::zero()),
            (
                Point2::new(corner, Interval::from_f64(1.0)),
                Interval::zero(),
            ),
            (p(0.0, 1.0), Interval::zero()),
        ]);
        let plane = SketchPlane::xy_at(Point3::new(Interval::zero(), Interval::zero(), z));
        Profile::new(plane, vec![lp])
            .validate(Tol::witness())
            .expect("the quad validates at Interval")
    };
    let w = Tol::witness().eps() / 256.0;
    let point = quad(Interval::from_f64(2.5), Interval::from_f64(1.0))
        .value_channel()
        .expect("a form of points has a value channel");
    let corner = point.loops()[0]
        .vertices()
        .iter()
        .find(|v| v.y == 1.0 && v.x > 0.0)
        .expect("the moving corner");
    assert_eq!((corner.x, point.plane().origin().z), (2.5, 1.0));
    let wide = Interval::from_bounds(2.5 - w, 2.5 + w);
    match quad(wide, Interval::from_f64(1.0)).value_channel() {
        Err(ValueNotAPoint::Vertex {
            loop_index: 0,
            vertex: 2,
        }) => {}
        other => panic!(
            "a wide corner refuses at its vertex, got {:?}",
            other.map(|_| ())
        ),
    }
    match quad(
        Interval::from_f64(2.5),
        Interval::from_bounds(1.0 - w, 1.0 + w),
    )
    .value_channel()
    {
        Err(ValueNotAPoint::Plane) => {}
        other => panic!(
            "a wide plane refuses as the plane, got {:?}",
            other.map(|_| ())
        ),
    }
}

/// **An arc whose carrier is no single point refuses at its segment.**
/// A half-disc validated at `Interval` with its arc's bulge a point
/// still holds a carrier computed from the bulge, a radius and sweep
/// from `atan` that are enclosures: the mint names the arc's segment
/// (canonical loop 0), where its vertices, both points, pass.
#[test]
fn the_value_channel_refuses_an_arc_carrier_at_its_segment() {
    use profile::test_support::bulge_loop;
    use profile::{Profile, ValueNotAPoint};
    let p = |x: f64, y: f64| Point2::new(Interval::from_f64(x), Interval::from_f64(y));
    let form = Profile::new(
        profile::SketchPlane::xy(),
        vec![bulge_loop(vec![
            (p(0.0, 0.0), Interval::zero()),
            (p(2.0, 0.0), Interval::from_f64(0.5)),
        ])],
    )
    .validate(Tol::witness())
    .expect("the arc-capped chord validates at Interval");
    let arc = form.loops()[0]
        .segments()
        .iter()
        .position(|s| matches!(s.kind, SegmentKind::Arc { .. }))
        .expect("the bulge is an arc");
    match form.value_channel() {
        Err(ValueNotAPoint::Segment {
            loop_index: 0,
            segment,
        }) => assert_eq!(segment, arc, "the refusal names the arc's segment"),
        other => panic!(
            "an arc carrier with no point refuses at its segment, got {:?}",
            other.map(|_| ())
        ),
    }
}

/// **A chord heading along −x from a computed point reads its turn as an
/// enclosure of the nominal's.** `line(2.9)` along `π/2` from `(1, 0.1)`
/// lands on a thin enclosure of `(1, 3)`, so every chord from it to an
/// authored point on `y = 3` runs along −x with a `y` component
/// straddling zero — `atan2`'s branch cut, where an angle enclosure is
/// the whole circle. The chord's director is its components, so each
/// door below classifies the junction it reaches from a narrow ray: the
/// line and arc legs, the fused arc incomings and the closing chord
/// construct (a definite left turn), and the leg arriving along −x
/// refuses as the tangent it is, with a margin narrow around zero.
#[test]
fn a_chord_along_minus_x_from_a_computed_point_reads_a_narrow_turn() {
    use geom_core::Bounds;
    use std::f64::consts::{FRAC_PI_2, PI};
    let t = Tol::witness();
    let one = |v: f64| Interval::from_f64(v);
    let up_then_along = |from: Point2<Interval>| {
        profile::Open
            .at(from)
            .angle(one(FRAC_PI_2), t)
            .expect("the director up")
            .line(one(2.9), t)
            .expect("the computed point beside (1, 3)")
    };

    let mut wrong = Vec::new();
    if let Err(e) = up_then_along(ip2(1.0, 0.1)).line_to(ip2(0.0, 3.0), t) {
        wrong.push(format!("the line leg along -x: {e:?}"));
    }
    if let Err(e) = up_then_along(ip2(1.0, 0.1)).arc_to(
        profile::Bulge {
            p: ip2(0.0, 3.0),
            b: one(0.2),
        },
        t,
    ) {
        wrong.push(format!("the arc leg whose chord runs along -x: {e:?}"));
    }
    if let Err(e) = up_then_along(ip2(1.0, 0.1)).arc_fillet(
        profile::Bulge {
            p: ip2(0.0, 3.0),
            b: one(0.2),
        },
        one(0.1),
        t,
    ) {
        wrong.push(format!(
            "the fused bulge incoming whose chord runs along -x: {e:?}"
        ));
    }
    if let Err(e) = up_then_along(ip2(1.0, 0.1)).arc_fillet(
        profile::Center {
            c: ip2(0.5, 2.0),
            winding: profile::ArcSweep::Ccw,
            p: ip2(0.0, 3.0),
        },
        one(0.1),
        t,
    ) {
        wrong.push(format!(
            "the fused centre incoming whose chord runs along -x: {e:?}"
        ));
    }
    let closing = profile::Open
        .at(ip2(0.0, 3.0))
        .line_to(ip2(1.0, 0.0), t)
        .expect("the first leg")
        .line_to(ip2(1.0, 0.1), t)
        .expect("the leg up")
        .tangent()
        .line(one(2.9), t)
        .expect("the computed point beside (1, 3)")
        .line_to(profile::Start, t);
    if let Err(e) = closing {
        wrong.push(format!("the closing chord along -x: {e:?}"));
    }
    assert!(
        wrong.is_empty(),
        "each chord's junction turns definitely left: {wrong:#?}"
    );

    let err = profile::Open
        .at(ip2(2.0, 3.0))
        .angle(one(PI), t)
        .expect("the director along -x")
        .line(one(1.0), t)
        .expect("the computed point beside (1, 3)")
        .line_to(ip2(0.0, 3.0), t)
        .expect_err("a chord continuing the incoming direction is a tangent junction");
    let profile::PathError::JunctionTangent { margin, .. } = err else {
        panic!("expected the tangent refusal, got {err:?}");
    };
    assert!(
        margin.lo() <= 0.0 && 0.0 <= margin.hi() && margin.hi() - margin.lo() < 1e-12,
        "the tangent junction's turn margin encloses zero narrowly, got {margin:?}"
    );
}

/// **A direction a later step turns is turned as a ray, never rebuilt
/// from an angle read back off it.** Each case ends a leg heading −x at
/// `Interval`, where an angle read off that ray by `atan2` straddles its
/// branch cut and encloses the whole circle, and then turns it: `.turn`
/// after a bulge arc whose end tangent heads −x (`b` = 1, ½ and 2, the
/// chord `2·unit(π − 2·atan b)`), the `Sweep` extension after the same
/// arc with a leg following, and `.turn` after a chord along −x from a
/// computed point. Each junction then reads a definite turn.
#[test]
fn a_turn_after_a_ray_along_minus_x_turns_the_ray() {
    use std::f64::consts::{FRAC_PI_2, PI};
    let t = Tol::witness();
    let one = |v: f64| Interval::from_f64(v);
    let mut wrong = Vec::new();
    for b in [1.0, 0.5, 2.0] {
        let gamma = PI - 2.0 * f64::atan(b);
        let arc = || {
            profile::Open
                .at(ip2(0.0, 0.0))
                .arc_to(
                    profile::Bulge {
                        p: ip2(2.0 * gamma.cos(), 2.0 * gamma.sin()),
                        b: one(b),
                    },
                    t,
                )
                .expect("the arc")
        };
        match arc().turn(one(0.5), t) {
            Ok(p) => {
                if let Err(e) = p.line(one(1.0), t) {
                    wrong.push(format!("b = {b}: the leg after .turn: {e:?}"));
                }
            }
            Err(e) => wrong.push(format!("b = {b}: .turn after the arc: {e:?}")),
        }
        let sweep = arc().tangent().arc_to(
            profile::Sweep {
                r: one(1.0),
                side: profile::ArcSide::Left,
                angle: one(0.5),
            },
            t,
        );
        match sweep {
            Ok(p) => {
                if let Err(e) = p.line_to(ip2(-10.0, 5.0), t) {
                    wrong.push(format!("b = {b}: the leg after the Sweep extension: {e:?}"));
                }
            }
            Err(e) => wrong.push(format!("b = {b}: the Sweep extension after the arc: {e:?}")),
        }
    }
    let chord_then_turn = profile::Open
        .at(ip2(1.0, 0.1))
        .angle(one(FRAC_PI_2), t)
        .expect("the director up")
        .line(one(2.9), t)
        .expect("the computed point beside (1, 3)")
        .line_to(ip2(0.0, 3.0), t)
        .expect("the chord along -x")
        .turn(one(1.0), t);
    match chord_then_turn {
        Ok(p) => {
            if let Err(e) = p.line(one(1.0), t) {
                wrong.push(format!("the leg after .turn after the chord: {e:?}"));
            }
        }
        Err(e) => wrong.push(format!(".turn after the chord: {e:?}")),
    }
    assert!(
        wrong.is_empty(),
        "every turned ray reads a definite turn: {wrong:#?}"
    );
}
