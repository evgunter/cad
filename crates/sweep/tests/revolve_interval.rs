//! Interval-lane revolve (feature `interval`): the acceptance shapes
//! REQUIRED to build tier-valid at the certified scalar (the lane is
//! fully live post-B1 — refusals here are defects, not honesty).
//! Trigonometry (rotation matrices, full-period rim samples) runs in
//! enclosures throughout; the full-period-is-identity convention keeps
//! the seam coincidences exact at Interval too.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Interval, Point2, Real, Tolerance, Vec2};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{validate, validate_closed, validate_geometric};
use geom_core::Tol;

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(Interval::from_f64(x), Interval::from_f64(y))
}

fn validated(loops: Vec<ProfileLoop<Interval>>) -> ValidatedProfile<Interval> {
    Profile::new(SketchPlane::<Interval>::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}

fn axis_y() -> RevolveAxis<Interval> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(Interval::from_f64(0.0), Interval::from_f64(1.0)),
    }
}

fn assert_tiers(body: &topo::Body<Interval>) {
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));
    assert_eq!(validate_geometric(body), Ok(()));
}

#[test]
fn interval_washer_builds_tier_valid() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    assert_tiers(&t.body);
    assert_eq!(t.body.vertices().count(), 4);
    assert_eq!(t.body.edges().count(), 8);
    assert_eq!(t.body.faces().count(), 4);
}

#[test]
fn interval_ball_builds_tier_valid() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), Interval::from_f64(1.0)),
        ProfileVertex::new(p2(0.0, 1.0), Interval::from_f64(0.0)),
    ]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    assert_tiers(&t.body);
    assert_eq!(t.body.faces().count(), 2);
}

#[test]
fn interval_cone_builds_tier_valid() {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    assert_tiers(&t.body);
    assert_eq!(t.body.faces().count(), 4);
}

#[test]
fn interval_partial_wedge_builds_tier_valid() {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    // π/2 is not dyadic: the wedge exercises non-dyadic rotation
    // enclosures end-to-end.
    let theta = Interval::from_f64(core::f64::consts::FRAC_PI_2);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(theta)).unwrap();
    assert_tiers(&t.body);
    assert_eq!(t.body.faces().count(), 5);
}
