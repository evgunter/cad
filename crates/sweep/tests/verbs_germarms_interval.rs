//! The curved pierce RING lane at the CERTIFIED scalar (feature
//! `interval`) — the two-arm pattern for the lane's new decide sites.
//!
//! Three predicates are new or newly reached here and all three are
//! metered as LENGTHS (the root-span gaps and the chart certificate;
//! the discriminant is the flagged dimensionless one it has always
//! been, and is not re-metered here), so the lane's honesty depends on
//! the enclosures being tight rather than lucky: `bool_pierce_normal_on_chart` (the
//! point-on-chart certificate behind the per-point outward normal),
//! `bool_wall_root_in_span` (a root's two gaps to the span's ends), and
//! `bool_ray_cylinder_disc` reached from an EDGE rather than a ray for
//! the first time.
//!
//! Both arms are pinned: the pierce arm must reach the same door it
//! reaches at `f64` (the lane BUILDS at the certified scalar rather
//! than escalating out of it), and the clearance arm must still answer
//! with a volume — asserted with an explicit WIDTH bound, because an
//! enclosure that contains the truth and spans a metre would pass a
//! containment check while saying nothing.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Bounds, Interval, Point2, Real, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(iv(x), iv(y))
}

/// The `f64` suite's pipe at the certified scalar: radius 1 about `z`,
/// `z ∈ [−2, 2]`, built through the same public doors so the two
/// lanes differ in the SCALAR and in nothing else. Every coordinate is
/// dyadic, so the operands' enclosures are points and the margins below
/// are the lane's own width rather than the fixture's.
fn pipe() -> Body<Interval> {
    let tol = Tol::witness();
    let lp = profile::circle(p2(0.0, 0.0), iv(1.0), tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(iv(0.0), iv(0.0), iv(-2.0))));
    let vp = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&vp, Extrusion::Distance(iv(4.0)), tol)
        .unwrap()
        .body
}

fn bar(x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64) -> Body<Interval> {
    let tol = Tol::witness();
    let lp: ProfileLoop<Interval> =
        RawLoop::polygon([p2(x0, y0), p2(x1, y0), p2(x1, y1), p2(x0, y1)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(iv(0.0), iv(0.0), iv(z0))));
    let vp = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&vp, Extrusion::Distance(iv(z1 - z0)), tol)
        .unwrap()
        .body
}

/// **The build arm.** The bar's crossings are found at the certified
/// scalar too, so the union walks past the crossing layer and refuses
/// at the ring's absent JOIN arm (#1291) — the same door the `f64` lane
/// reaches. An escalation here would mean the enclosures, not the
/// geometry, decided the lane.
///
/// The bar is short for the same reason its `f64` twin is: a pierce
/// vertex's sector arms are the split edge's fragments, and a fragment
/// past the wall's radius makes the sector-side curvature charge
/// refuse (`boolean::sectors::side_code`). Every coordinate here is
/// dyadic — `±1.125` and `±0.25` exactly — so the enclosures stay
/// points and this row measures the LANE rather than the fixture.
#[test]
fn the_ring_lane_builds_at_the_certified_scalar() {
    let err = topo::union(
        &pipe(),
        &bar(-1.125, 1.125, -0.25, 0.25, -0.25, 0.25),
        Tol::witness(),
    )
    .expect_err("no join arm for a pierce ring");
    assert!(
        matches!(
            err,
            BooleanError::Join(topo::SplitJoinError::SectionArcWindow {
                case: topo::ArcWindowCase::NoChartedRun,
                ..
            })
        ),
        "{err:?}"
    );
}

/// **The clearance arm, with a width bound.** A bar definitely clear of
/// the wall still answers, and the enclosure of the answer has to be
/// narrow enough to be a claim: the truth is inside it AND the interval
/// is under a micrometre wide.
#[test]
fn a_clear_bar_still_answers_and_the_enclosure_is_narrow() {
    let tol = Tol::witness();
    let topo::BooleanResult::Body(out) =
        topo::union(&pipe(), &bar(1.5, 2.5, -0.25, 0.25, -0.25, 0.25), tol)
            .expect("no crossing to route")
    else {
        panic!("two clear solids union into a two-shell body");
    };
    assert_eq!(out.body.shells().count(), 2);
    let v = topo::mass_properties(&out.body, tol).unwrap().volume;
    let truth = PI * 4.0 + 1.0 * 0.5 * 0.5;
    assert!(
        v.lo() <= truth && truth <= v.hi(),
        "the enclosure must contain the truth: {v:?} vs {truth}"
    );
    assert!(
        v.hi() - v.lo() < 1e-6,
        "the enclosure must be a claim, not a shrug: width {}",
        v.hi() - v.lo()
    );
}
