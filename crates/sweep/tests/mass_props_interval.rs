//! M2 PR 7 interval lane: the SAME closed-form mass-property formulas
//! instantiated at the certified interval scalar — the enclosures ARE
//! the certified bounds (Q1). Asserted: each enclosure CONTAINS the
//! independent analytic closed form, and the f64-lane value lies
//! within the enclosure's width of it.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2};

use geom_core::{Bounds, Interval, Point2, Real, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, mass_properties};

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(Interval::from_f64(x), Interval::from_f64(y))
}

fn validated(loops: Vec<ProfileLoop<Interval>>) -> ValidatedProfile<Interval> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(geom_core::Tolerance::get())
        .unwrap()
}

fn axis_y() -> RevolveAxis<Interval> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(Interval::from_f64(0.0), Interval::from_f64(1.0)),
    }
}

/// Enclosure contains the analytic value; enclosure is tight; the
/// midpoint (the f64-lane comparable) is within the width of it.
fn assert_encloses(what: &str, enclosure: Interval, analytic: f64) {
    let (lo, hi) = (enclosure.lo(), enclosure.hi());
    assert!(
        lo <= analytic && analytic <= hi,
        "{what}: enclosure [{lo}, {hi}] must contain {analytic}"
    );
    let width = hi - lo;
    assert!(
        width <= 1e-9 * analytic.abs().max(1.0),
        "{what}: enclosure width {width} is not tight"
    );
}

fn check(body: &Body<Interval>, what: &str, volume: f64, area: f64) {
    let props = mass_properties(body).expect("interval mass properties must compute");
    assert_encloses(&format!("{what} volume"), props.volume, volume);
    assert_encloses(&format!("{what} area"), props.surface_area, area);
}

#[test]
fn l_prism_interval_encloses_closed_forms() {
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ]);
    let body = extrude(&validated(vec![lp]), Extrusion::Distance(Interval::from_f64(1.0)))
        .unwrap()
        .body;
    check(&body, "L-prism", 3.0, 14.0);
}

#[test]
fn ball_interval_encloses_closed_forms() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(0.0, -1.0),
            bulge: Interval::from_f64(1.0),
        },
        ProfileVertex {
            pos: p2(0.0, 1.0),
            bulge: Interval::from_f64(0.0),
        },
    ]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    check(&t.body, "ball", 4.0 * PI / 3.0, 4.0 * PI);
}

#[test]
fn cone_interval_encloses_closed_forms() {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    check(&t.body, "cone", PI / 3.0, PI * (SQRT_2 + 1.0));
}

#[test]
fn washer_interval_encloses_closed_forms() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    check(&t.body, "washer", 3.0 * PI, 12.0 * PI);
}

#[test]
fn wedge_interval_encloses_closed_forms() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(Interval::from_f64(FRAC_PI_2)),
    )
    .unwrap();
    check(&t.body, "wedge", 3.0 * PI / 4.0, 3.0 * PI + 2.0);
}

/// The f64 lane's value sits inside (a hair beyond) the certified
/// enclosure: cross-scalar agreement.
#[test]
fn f64_value_within_interval_enclosure() {
    // Ball at both scalars.
    let lp_i = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(0.0, -1.0),
            bulge: Interval::from_f64(1.0),
        },
        ProfileVertex {
            pos: p2(0.0, 1.0),
            bulge: Interval::from_f64(0.0),
        },
    ]);
    let t_i = revolve(&validated(vec![lp_i]), axis_y(), Revolution::Full).unwrap();
    let enc = mass_properties(&t_i.body).unwrap().volume;

    let lp_f = ProfileLoop::new(vec![
        ProfileVertex {
            pos: Point2::new(0.0, -1.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: Point2::new(0.0, 1.0),
            bulge: 0.0,
        },
    ]);
    let vp_f = Profile::new(SketchPlane::xy(), vec![lp_f])
        .validate(geom_core::Tolerance::get())
        .unwrap();
    let t_f = revolve(
        &vp_f,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
    )
    .unwrap();
    let v_f = mass_properties(&t_f.body).unwrap().volume;
    let width = enc.hi() - enc.lo();
    assert!(
        enc.lo() - width <= v_f && v_f <= enc.hi() + width,
        "f64 volume {v_f} vs enclosure [{}, {}]",
        enc.lo(),
        enc.hi()
    );
}
