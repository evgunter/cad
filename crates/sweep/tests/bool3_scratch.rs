//! SCRATCH — deleted before the PR.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::{Body, point_in_solid};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).unwrap()
}

fn donut() -> Body<f64> {
    use sweep::{Revolution, RevolveAxis, revolve};
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.5, 1.10), 1.0),
        ProfileVertex::new(p2(0.5, 1.40), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// The vase's torus band alone: a full revolve of the bulge arc with
/// cylinder walls above and below and hemispherical caps.
fn vase() -> Body<f64> {
    use sweep::{Revolution, RevolveAxis, revolve};
    let cap = (core::f64::consts::PI / 8.0).tan();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), cap),
        ProfileVertex::new(p2(0.5, 0.5), 0.0),
        ProfileVertex::new(p2(0.5, 1.0), 0.6),
        ProfileVertex::new(p2(0.5, 1.5), 0.0),
        ProfileVertex::new(p2(0.5, 2.0), cap),
        ProfileVertex::new(p2(0.0, 2.5), 0.0),
    ])
    .with_tangent_joints(vec![1, 4]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// A quarter donut: the same circle profile through a π/2 revolve.
fn quarter_donut() -> Body<f64> {
    use sweep::{Revolution, RevolveAxis, revolve};
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.5, 1.10), 1.0),
        ProfileVertex::new(p2(0.5, 1.40), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    revolve(
        &vp,
        axis,
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn report(name: &str, b: &Body<f64>, pts: &[(&str, Point3<f64>)]) {
    println!("--- {name}");
    for (label, q) in pts {
        println!("  {label}: {:?}", point_in_solid(b, *q, band(), Tol::witness()));
    }
}

#[test]
fn scratch_torus_door() {
    let d = donut();
    report(
        "donut (R=0.5, r=0.15, centre (0,1.25,0))",
        &d,
        &[
            ("spine (0.5,1.25,0) -> In", Point3::new(0.5, 1.25, 0.0)),
            ("hole (0,1.25,0) -> Out", Point3::new(0.0, 1.25, 0.0)),
            ("far (3,1.25,0) -> Out", Point3::new(3.0, 1.25, 0.0)),
            ("above (0,3,0) -> Out", Point3::new(0.0, 3.0, 0.0)),
            (
                "outer equator (0.65,1.25,0) -> OnBoundary",
                Point3::new(0.65, 1.25, 0.0),
            ),
            (
                "inner equator (0.35,1.25,0) -> OnBoundary",
                Point3::new(0.35, 1.25, 0.0),
            ),
            ("top (0.5,1.40,0) -> OnBoundary", Point3::new(0.5, 1.40, 0.0)),
            (
                "spine off-seam (0,1.25,0.5) -> In",
                Point3::new(0.0, 1.25, 0.5),
            ),
            (
                "generic interior (0.35,1.35,0.35) -> ?",
                Point3::new(0.35, 1.35, 0.35),
            ),
        ],
    );
    let v = vase();
    report(
        "vase",
        &v,
        &[
            ("axis (0,1.25,0) -> In", Point3::new(0.0, 1.25, 0.0)),
            ("outside (2,1.25,0) -> Out", Point3::new(2.0, 1.25, 0.0)),
            (
                "just inside the bulge (0.5,1.25,0) -> In",
                Point3::new(0.5, 1.25, 0.0),
            ),
        ],
    );
    let qd = quarter_donut();
    report(
        "quarter donut",
        &qd,
        &[
            ("spine x (0.5,1.25,0) -> ?", Point3::new(0.5, 1.25, 0.0)),
            ("spine z (0,1.25,0.5) -> ?", Point3::new(0.0, 1.25, 0.5)),
            (
                "spine mid (0.354,1.25,0.354) -> ?",
                Point3::new(0.353_553_390_593_273_8, 1.25, 0.353_553_390_593_273_8),
            ),
            (
                "spine mid mirrored (-0.354,1.25,-0.354) -> Out",
                Point3::new(-0.353_553_390_593_273_8, 1.25, -0.353_553_390_593_273_8),
            ),
        ],
    );
}
