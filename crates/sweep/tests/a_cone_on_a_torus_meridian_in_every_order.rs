//! **Two cones on a torus's meridian circle, in every operation and
//! order.** The torus `R = 2`, `r = 1` about `y`, against the triangle
//! `(1, 0), (3/2, 1/2), (1/2, 1/2)` revolved about the line through
//! `(2, 0, 0)` along `z`: its vertex sweeps the torus's meridian circle
//! in the `xy` plane, and its two cones (apexes `(2, 0, ±1)`, both
//! inside the tube, half-angle 45°) cross the tube there and along a
//! second curve each. Each pair is a cone off the torus's axis with its
//! apex clear of the torus, the section value's general cone × torus
//! pose, marched.
//!
//! Each run builds a valid body, and the volumes close as the operations
//! must: `|A ∪ B| + |A ∩ B| = |A| + |B|` and `|A ∖ B| = |A| − |A ∩ B|`,
//! within the measured pads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body};

use crate::revolve_common::axis_y;

fn revolved(points: &[(f64, f64, f64)], axis: RevolveAxis<f64>, tol: Tol) -> Body<f64> {
    let lp = bulge_loop(
        points
            .iter()
            .map(|&(x, y, bulge)| (Point2::new(x, y), bulge))
            .collect(),
    );
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    revolve(&vp, axis, Revolution::Full, tol)
        .expect("the profile revolves")
        .body
}

/// The torus `R = 2`, `r = 1` about `y`.
fn torus(tol: Tol) -> AtRestBody<f64> {
    finished(
        "the torus",
        revolved(&[(2.0, -1.0, 1.0), (2.0, 1.0, 1.0)], axis_y(), tol),
        tol,
    )
}

/// The triangle revolved about the line `x = 2` along the sketch's `y`,
/// then turned a quarter about the `x` axis through `(2, 0, 0)`, so that
/// line runs along `z` and the triangle's vertex `(1, 0)` sweeps the
/// meridian circle in `z = 0`.
fn triangle(tol: Tol) -> AtRestBody<f64> {
    let body = revolved(
        &[(1.0, 0.0, 0.0), (1.5, 0.5, 0.0), (0.5, 0.5, 0.0)],
        RevolveAxis {
            origin: Point2::new(2.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        tol,
    );
    let turn = Affine3::rotation_about_axis(Point3::new(2.0, 0.0, 0.0), Vec3::unit_x(), FRAC_PI_2);
    let turned = topo::transform_rigid(&body, &geom_core::test_support::rigid(&turn, tol)).unwrap();
    finished("the meridian triangle", turned, tol)
}

fn volume(body: &Body<f64>, tol: Tol) -> (f64, f64) {
    let m = topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("the body does not measure: {e:?}"))
        .enclosure;
    (m.volume, m.volume_pad)
}

/// The measured volume of a run's result, zero for an empty one.
fn result_volume(
    label: &str,
    got: Result<topo::BooleanResult<f64>, topo::BooleanError>,
    tol: Tol,
) -> (f64, f64) {
    let got = got.unwrap_or_else(|e| panic!("{label}: refused: {e:?}"));
    got.body().map_or((0.0, 0.0), |b| volume(&b.body, tol))
}

#[test]
fn the_meridian_triangle_and_its_torus_build_in_every_order() {
    let tol = Tol::witness();
    let (t, k) = (torus(tol), triangle(tol));
    let (vt, pt) = volume(&t.body, tol);
    let (vk, pk) = volume(&k.body, tol);
    assert!(
        (vt - 4.0 * PI * PI).abs() <= pt + 1e-9 && (vk - PI / 2.0).abs() <= pk + 1e-9,
        "the operands measure as Pappus has them: {vt} ± {pt}, {vk} ± {pk}"
    );
    for (name, a, b, va, pa) in [
        ("torus first", &t, &k, vt, pt),
        ("triangle first", &k, &t, vk, pk),
    ] {
        let (vu, pu) = result_volume(&format!("{name}: ∪"), topo::union(a, b, tol), tol);
        let (vi, pi) = result_volume(&format!("{name}: ∩"), topo::intersect(a, b, tol), tol);
        let (vd, pd) = result_volume(&format!("{name}: ∖"), topo::subtract(a, b, tol), tol);
        assert!(
            vi > 0.0 && vi < vk,
            "{name}: the triangle crosses the tube, so ∩ is a part of it ({vi})"
        );
        assert!(
            (vu + vi - vt - vk).abs() <= pu + pi + pt + pk + 1e-9,
            "{name}: |A ∪ B| + |A ∩ B| = {} misses |A| + |B| = {}",
            vu + vi,
            vt + vk
        );
        assert!(
            (vd - (va - vi)).abs() <= pd + pa + pi + 1e-9,
            "{name}: |A ∖ B| = {vd} misses |A| − |A ∩ B| = {}",
            va - vi
        );
    }
}
