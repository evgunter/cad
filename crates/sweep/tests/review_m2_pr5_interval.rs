//! Adversarial review suite for M2 PR 5, interval lane (feature
//! `interval`): the reviewer's shapes that go beyond the shipped
//! four-acceptance set — a multi-segment two-band wire (dome), the
//! wrap-run donut, and a NEGATIVE-angle non-dyadic wedge. Refusals
//! here are defects, not honesty (the lane is fully live post-B1).

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Interval, Point2, Real, Tolerance, Vec2};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{validate, validate_closed, validate_geometric};

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(Interval::from_f64(x), Interval::from_f64(y))
}

fn validated(loops: Vec<ProfileLoop<Interval>>) -> ValidatedProfile<Interval> {
    Profile::new(SketchPlane::xy(), loops)
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
fn survives_interval_dome_two_band_wire() {
    // Four wire segments (plane, cylinder, cone, plane) + axis run:
    // the richest two-band wire, at the certified scalar.
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(1.0, 1.0),
        p2(0.5, 1.5),
        p2(0.0, 1.5),
    ]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    assert_tiers(&t.body);
    assert_eq!(t.body.vertices().count(), 8);
    assert_eq!(t.body.edges().count(), 14);
    assert_eq!(t.body.faces().count(), 8);
    assert_eq!(t.body.surfaces().count(), 4);
}

#[test]
fn survives_interval_donut_wrap_run() {
    // The 2-arc circle profile: cosurface wrap pair + kfmrh/zip at
    // Interval; ONE torus key, both meridians Seam.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(1.0, 0.5), Interval::from_f64(1.0)),
        ProfileVertex::new(p2(2.0, 0.5), Interval::from_f64(1.0)),
    ]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    assert_tiers(&t.body);
    assert_eq!(t.body.surfaces().count(), 1);
    assert_eq!(t.body.edges().count(), 4);
}

#[test]
fn survives_interval_negative_non_dyadic_wedge() {
    // θ = −π/2 (non-dyadic, reversed traversal arm) — the shipped
    // interval suite only sweeps the positive sign.
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let theta = Interval::from_f64(-core::f64::consts::FRAC_PI_2);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(theta)).unwrap();
    assert_tiers(&t.body);
    assert_eq!(t.body.faces().count(), 5);
}
