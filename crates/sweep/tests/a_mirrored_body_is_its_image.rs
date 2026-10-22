//! **A mirrored body is its image** (`topo::mirror_body`,
//! `docs/MIRROR-DESIGN.md` P1–P3): over a body of every surface class
//! the sweeps build (planes, a cylinder, a sphere, a cone, a torus,
//! partial revolves with plane caps, a spline loft), mirrored across a
//! plane along an axis and an oblique one, the result is a finished
//! body at rest, of the source's volume, whose every point is the
//! reflection of the source's point under the same key; and mirroring
//! twice comes back to the source's points within the map's rounding.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Arc2, Point2, Point3, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, Reflection, mirror_body};

fn tol() -> Tol {
    Tol::witness()
}

fn finished(what: &str, body: Body<f64>) -> Body<f64> {
    (*topo::test_support::finished(what, body, tol())).clone()
}

fn prism() -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    let pentagon = bulge_loop(vec![
        p(0.0, 0.0),
        p(2.0, 0.0),
        p(3.0, 1.0),
        p(1.0, 2.0),
        p(0.0, 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![pentagon])
        .validate(tol())
        .unwrap();
    let along = Extrusion::Distance {
        depth: 1.5,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, along, tol()).unwrap().body
}

fn cylinder() -> Body<f64> {
    let circle = RawLoop::new([(
        Point2::new(1.5, 0.25),
        Segment::Arc(Arc2 {
            centre: Point2::new(0.5, 0.25),
            radius: 1.0,
            sweep: TAU,
        }),
    )]);
    let profile = Profile::new(SketchPlane::xy(), vec![circle])
        .validate(tol())
        .unwrap();
    let along = Extrusion::Distance {
        depth: 2.0,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, along, tol()).unwrap().body
}

/// The profile `points` (bulges `0`, a polygon; `1`, a semicircle) on
/// the `xz` sketch, revolved about the sketch's `y` axis through
/// `(0, 0)` by `revolution`.
fn revolved(points: Vec<(Point2<f64>, f64)>, revolution: Revolution<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![bulge_loop(points)])
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, revolution, tol()).unwrap().body
}

fn ball() -> Body<f64> {
    revolved(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
    )
}

fn cone() -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    revolved(
        vec![p(0.0, 0.0), p(1.0, 0.0), p(0.0, 1.5)],
        Revolution::Full,
    )
}

fn torus() -> Body<f64> {
    revolved(
        vec![(Point2::new(1.5, -0.5), 1.0), (Point2::new(1.5, 0.5), 1.0)],
        Revolution::Full,
    )
}

fn half_ring() -> Body<f64> {
    revolved(
        vec![(Point2::new(1.5, -0.5), 1.0), (Point2::new(1.5, 0.5), 1.0)],
        Revolution::Partial(PI * 0.75),
    )
}

fn frustum_wedge() -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    revolved(
        vec![p(0.5, 0.0), p(1.5, 0.0), p(1.0, 1.0), p(0.5, 1.0)],
        Revolution::Partial(-PI / 3.0),
    )
}

/// A square with one arc side, lofted through a narrower copy of
/// itself: its walls are spline nets.
fn loft() -> Body<f64> {
    let section = |h: f64, b: f64| {
        let p = |x: f64, y: f64| Point2::new(x * h, y * h);
        vec![bulge_loop(vec![
            (p(-1.0, -1.0), 0.0),
            (p(1.0, -1.0), b),
            (p(1.0, 1.0), 0.0),
            (p(-1.0, 1.0), 0.0),
        ])]
    };
    let places = [
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(0.25, 0.0, 1.0)),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 2.0)),
    ];
    let sections = [section(1.0, 0.3), section(0.75, 0.2), section(0.5, 0.4)];
    sweep::loft_body(&sections, &places, 2, tol())
        .expect("the arc-sided square lofts")
        .body
}

fn bodies() -> Vec<(&'static str, Body<f64>)> {
    vec![
        ("the prism", prism()),
        ("the cylinder", cylinder()),
        ("the ball", ball()),
        ("the cone", cone()),
        ("the torus", torus()),
        ("the partial ring", half_ring()),
        ("the frustum wedge", frustum_wedge()),
    ]
}

/// Mirror planes: one along an axis, and one oblique to every axis.
fn planes() -> Vec<(&'static str, Reflection<f64>)> {
    vec![
        (
            "x = −0.75",
            Reflection::across(
                Point3::new(-0.75, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                tol(),
            )
            .unwrap(),
        ),
        (
            "the oblique plane",
            Reflection::across(Point3::new(0.1, -0.2, 0.4), Vec3::new(1.0, 2.0, 3.0), tol())
                .unwrap(),
        ),
    ]
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, tol()).unwrap().volume
}

/// The checks on `body` mirrored across each plane.
fn mirrors_to_its_image(what: &str, body: Body<f64>) {
    let body = finished(what, body);
    let v = volume(&body);
    for (plane, reflection) in planes() {
        let label = format!("{what} across {plane}");
        let image = mirror_body(&body, &reflection, tol())
            .unwrap_or_else(|e| panic!("{label}: the mirror refused: {e}"));
        let image = finished(&label, image);
        let w = volume(&image);
        assert!(
            w > 0.0 && (w - v).abs() <= 1e-9 * v,
            "{label}: volume {w} against the source's {v}"
        );
        let map = reflection.to_affine();
        for (k, p) in body.points() {
            let q = image
                .points()
                .find(|(j, _)| *j == k)
                .expect("the key survives")
                .1;
            assert_eq!(
                q.to_array().map(f64::to_bits),
                map.transform_point(*p).to_array().map(f64::to_bits),
                "{label}: point {k:?} is the reflection of the source's"
            );
        }
        let back = mirror_body(&image, &reflection, tol())
            .unwrap_or_else(|e| panic!("{label}: mirroring back refused: {e}"));
        let back = finished(&format!("{label}, mirrored back"), back);
        for (k, p) in body.points() {
            let q = back
                .points()
                .find(|(j, _)| *j == k)
                .expect("the key survives")
                .1;
            assert!(
                (*q - *p).norm() <= 1e-12,
                "{label}: mirrored back, point {k:?} is {q:?}, not {p:?}"
            );
        }
    }
}

#[test]
fn every_analytic_class_mirrors_to_its_image() {
    for (what, body) in bodies() {
        mirrors_to_its_image(what, body);
    }
}

#[test]
fn a_spline_loft_mirrors_to_its_image() {
    mirrors_to_its_image("the loft", loft());
}
