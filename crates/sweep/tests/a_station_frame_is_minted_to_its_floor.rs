//! **A path sweep's station frame is minted at `Interval` to its floor**:
//! each station's tangent is the path's derivative read through its
//! hodograph (`crates/sweep/README.md` `[sweep.frame]`), so the frame a
//! lane mints at a point path is a few tens of ulps wide, flat along the
//! path. The derivative recursion's own enclosure is about 20 times
//! wider; a frame minted from it reaches 2.6e-13 on these paths.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::TAU;

use geom::NurbsCurve3;
use geom_core::{Bounds, Interval, Point3, Real, RigidMotion, Tol};
use profile::SketchPlane;

use crate::common::normal_start_place;

/// The widest component of a station frame: its origin and its axes.
fn width(m: RigidMotion<Interval>) -> f64 {
    let o = m.origin();
    let mut comps = vec![o.x, o.y, o.z];
    for a in [m.u(), m.v(), m.w()] {
        let g = a.get();
        comps.extend([g.x, g.y, g.z]);
    }
    comps
        .into_iter()
        .map(|c| c.hi() - c.lo())
        .fold(0.0, f64::max)
}

/// A cubic interpolant through `points`.
fn through(points: &[Point3<f64>]) -> NurbsCurve3<f64> {
    NurbsCurve3::<f64>::interpolate(points, 3).expect("the points interpolate")
}

#[test]
fn every_station_frame_is_minted_within_its_floor() {
    let helix: Vec<Point3<f64>> = (0..=64)
        .map(|k| {
            let s = f64::from(k) / 64.0;
            let a = TAU * 2.0 * s;
            Point3::new(a.cos(), a.sin(), 0.8 * s)
        })
        .collect();
    let spine: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let a = TAU * f64::from(k % 32) / 32.0;
            Point3::new(3.0 * a.cos(), 3.0 * a.sin(), 0.0)
        })
        .collect();
    for (name, path, stations) in [
        ("a two-turn helix", through(&helix), 65),
        ("a closed spine of radius 3", through(&spine), 9),
    ] {
        let start = normal_start_place(&path);
        let plane = SketchPlane::from_frame(geom_core::test_support::rigid(
            &start.placement().map(Interval::from_f64),
            Tol::witness(),
        ));
        let places = sweep::sweep_places(plane, &path, stations, Tol::witness())
            .unwrap_or_else(|e| panic!("{name} carries at Interval: {e}"));
        let widest = places.iter().map(|p| width(p.frame())).fold(0.0, f64::max);
        println!("{name}: the widest station frame component is {widest:e}");
        assert!(
            widest < 5e-14,
            "{name}: a station frame is {widest:e} wide, past its floor"
        );
    }
}
