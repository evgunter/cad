//! M2 PR 7 e2e review — interval lane: the review shapes (beyond the
//! implementer's acceptance set) at the certified interval scalar.
//! The enclosure must CONTAIN the reviewer's independently derived
//! closed form and be tight (width ≤ 1e-9 relative) — Q1: the
//! enclosure IS the certified bound, so containment failure is
//! unsoundness, not inaccuracy.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2};
use profile::RawLoop;

use geom_core::{Bounds, Interval, Point2, Real, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, mass_properties};
use geom_core::Tol;

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(Interval::from_f64(x), Interval::from_f64(y))
}

fn v(x: f64, y: f64, b: f64) -> ProfileVertex<Interval> {
    ProfileVertex::new(p2(x, y), Interval::from_f64(b))
}

fn validated(loops: Vec<ProfileLoop<Interval>>) -> ValidatedProfile<Interval> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(geom_core::Tol::witness().get())
        .unwrap()
}

fn axis_y() -> RevolveAxis<Interval> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(Interval::from_f64(0.0), Interval::from_f64(1.0)),
    }
}

fn assert_encloses(what: &str, enclosure: Interval, analytic: f64) {
    let (lo, hi) = (enclosure.lo(), enclosure.hi());
    assert!(
        lo <= analytic && analytic <= hi,
        "{what}: enclosure [{lo}, {hi}] must contain {analytic} (UNSOUND if not)"
    );
    assert!(
        hi - lo <= 1e-9 * analytic.abs().max(1.0),
        "{what}: enclosure width {} not tight",
        hi - lo
    );
}

fn check(body: &Body<Interval>, what: &str, volume: f64, area: f64) {
    let props = mass_properties(body).expect("interval mass properties must compute");
    assert_encloses(&format!("{what} volume"), props.volume, volume);
    assert_encloses(&format!("{what} area"), props.surface_area, area);
}

/// Frustum shell (see the f64 review suite for the derivations).
#[test]
fn frustum_interval_encloses_reviewer_forms() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(1.0, 1.0)]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    check(
        &t.body,
        "frustum",
        4.0 * PI / 3.0,
        (5.0 + 3.0 * SQRT_2) * PI,
    );
}

#[test]
fn cup_interval_encloses_reviewer_forms() {
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 2.0),
        p2(1.5, 2.0),
        p2(1.5, 0.5),
        p2(0.0, 0.5),
    ]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    check(&t.body, "cup", 4.625 * PI, 20.5 * PI);
}

#[test]
fn quarter_donut_interval_encloses_reviewer_forms() {
    let lp = ProfileLoop::new(vec![v(2.0, -0.5, 1.0), v(2.0, 0.5, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(Interval::from_f64(FRAC_PI_2)),
    )
    .unwrap();
    check(&t.body, "quarter donut", PI * PI / 4.0, PI * PI + PI / 2.0);
}

#[test]
fn dome_wedge_interval_encloses_reviewer_forms() {
    let b = (PI / 8.0).tan();
    let lp = ProfileLoop::new(vec![v(0.0, 0.0, 0.0), v(1.0, 0.0, b), v(0.0, 1.0, 0.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(Interval::from_f64(FRAC_PI_2)),
    )
    .unwrap();
    check(&t.body, "dome wedge", PI / 6.0, 5.0 * PI / 4.0);
}

#[test]
fn major_arc_prism_interval_encloses_reviewer_forms() {
    let b = (3.0 * PI / 8.0).tan();
    let lp = ProfileLoop::new(vec![v(0.0, 0.0, 0.0), v(1.0, 0.0, b), v(0.0, -1.0, 0.0)]);
    let t = extrude(
        &validated(vec![lp]),
        Extrusion::Distance(Interval::from_f64(1.0)),
    )
    .unwrap();
    check(&t.body, "pac-man prism", 0.75 * PI, 3.0 * PI + 2.0);
}

#[test]
fn two_hole_plate_interval_encloses_reviewer_forms() {
    let outer = ProfileLoop::polygon([p2(-3.0, -3.0), p2(3.0, -3.0), p2(3.0, 3.0), p2(-3.0, 3.0)]);
    let round = ProfileLoop::new(vec![v(-0.5, 0.0, 1.0), v(-2.5, 0.0, 1.0)]);
    let square = ProfileLoop::polygon([p2(0.5, -1.0), p2(2.5, -1.0), p2(2.5, 1.0), p2(0.5, 1.0)]);
    let t = extrude(
        &validated(vec![outer, round, square]),
        Extrusion::Distance(Interval::from_f64(1.0)),
    )
    .unwrap();
    check(&t.body, "two-hole plate", 32.0 - PI, 96.0);
}
