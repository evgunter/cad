//! MEASURE (scratch): which pair doors refuse the census's apart shapes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_1_SQRT_2;

use geom_core::{Affine3, Point2, Point3, RigidMotion, Tol, Vec3};
use sweep::test_support::finished;
use topo::{AtRestBody, Body};

use crate::common::approx::{box_with_approx_cap, box_with_bumped_cap};

fn tol() -> Tol {
    Tol::witness()
}

fn moved(body: &Body<f64>, turn: Option<(Vec3<f64>, f64)>, by: Vec3<f64>) -> Body<f64> {
    let body = match turn {
        Some((axis, angle)) => topo::transform_rigid(
            body,
            &geom_core::test_support::rigid(
                &Affine3::rotation_about_axis(Point3::origin(), axis, angle),
                tol(),
            ),
        )
        .unwrap(),
        None => body.clone(),
    };
    topo::transform_rigid(&body, &RigidMotion::translation(by).unwrap()).unwrap()
}

/// A rod about `y`, radius 0.5, over `y ∈ [0, 1]`.
fn rod() -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(0.5, 0.0), 0.0),
            (Point2::new(0.5, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

fn caps() -> [(&'static str, Body<f64>); 2] {
    [
        ("nurbs cap", box_with_bumped_cap(0.0).0),
        ("approx cap", box_with_approx_cap(0.05, 1e-9).0),
    ]
}

fn report(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) {
    let r = topo::union(a, b, tol());
    match r {
        Ok(_) => eprintln!("MEASURE {what}: union builds"),
        Err(e) => eprintln!("MEASURE {what}: {e:?}"),
    }
}

#[test]
fn measure() {
    // 1. A rod beside the cap's `x = 0` edge, `gap` clear on the diagonal.
    for gap in [0.04, 1e-3, 0.0, -1e-3] {
        let off = (0.5 + gap) * FRAC_1_SQRT_2;
        let r = finished(
            "rod",
            moved(&rod(), None, Vec3::new(-off, 0.5, 1.0 + off)),
            tol(),
        );
        for (name, cap) in caps() {
            let cap = finished(name, cap, tol());
            report(&format!("rod {gap} off the {name}"), &cap, &r);
        }
    }
    // 2. The two lumps of a bar cut through a ball, the cap box between them.
    let ball = finished(
        "the ball",
        sweep::test_support::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
        tol(),
    );
    let bar = finished(
        "the bar",
        sweep::test_support::brick::<f64>((-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4), tol()),
        tol(),
    );
    let Ok(topo::BooleanResult::Body(lumps)) = topo::subtract(&bar, &ball, tol()) else {
        panic!("bar ∖ ball builds");
    };
    for (name, cap) in caps() {
        // Cap normal +z turned to +x: (x, y, z) ↦ (z, y, −x); the box
        // then spans x ∈ [0, 1], y ∈ [0, 2], z ∈ [−2, 0]; moved to
        // x ∈ [−0.5, 0.5], y ∈ [−1, 1], z ∈ [−0.75, 1.25].
        let b = finished(
            name,
            moved(
                &cap,
                Some((Vec3::unit_y(), core::f64::consts::FRAC_PI_2)),
                Vec3::new(-0.5, -1.0, 1.25),
            ),
            tol(),
        );
        report(&format!("lumps around the {name}"), &lumps.body, &b);
        // The cap moved to the +x patch: 0.05 inside its rim's reach.
        let near = finished(
            name,
            moved(
                &cap,
                Some((Vec3::unit_y(), core::f64::consts::FRAC_PI_2)),
                Vec3::new(-0.17, -1.0, 1.25),
            ),
            tol(),
        );
        report(&format!("lumps by the {name} at x = 0.83"), &lumps.body, &near);
    }
    for x in [0.5, 0.0, 0.92] {
        let plane = topo::test_support::split_plane(
            Point3::new(x, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            tol(),
        );
        let r = topo::split(&lumps.body, &plane, tol());
        eprintln!(
            "MEASURE split lumps at x = {x}: {}",
            match r {
                Ok(_) => "splits".to_string(),
                Err(e) => format!("{e:?}"),
            }
        );
    }
    // 2b. A sliver off the cap's corner edge (the census's sliver shape).
    let (sin, cos) = 4.0f64.sin_cos();
    let u = (-cos, -sin);
    let clear = 0.45 - 0.25 * (u.0 + u.1);
    let c = sweep::test_support::rod_chord_at(-0.45);
    let sliver = [((-0.45, c.half), c.wall_bulge), ((-0.45, -c.half), 0.0)];
    for s in [0.0, 1e-3 - clear] {
        let by = (0.75 + s * u.0, 0.75 + s * u.1);
        let sl = finished("sliver", prism(&sliver, 4.0, by, 0.0, 1.0), tol());
        for (name, cap) in caps() {
            let b = finished(
                name,
                moved(
                    &cap,
                    Some((Vec3::unit_y(), core::f64::consts::FRAC_PI_2)),
                    Vec3::new(0.0, -1.0, 1.0),
                ),
                tol(),
            );
            report(&format!("sliver {} clear of the {name}", clear + s), &b, &sl);
        }
    }
    // 3. The split gate: the ball less `y > 0.1`, cut by `y = 0.5`.
    let knife = finished(
        "the slab",
        sweep::test_support::brick::<f64>((-2.0, 2.0), (0.1, 2.0), (-2.0, 2.0), tol()),
        tol(),
    );
    for spin in [1.0, core::f64::consts::FRAC_PI_2] {
        let turned = finished(
            "the ball",
            moved(
                &sweep::test_support::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
                Some((Vec3::unit_y(), spin)),
                Vec3::new(0.0, 0.0, 0.0),
            ),
            tol(),
        );
        let Ok(topo::BooleanResult::Body(patch)) = topo::subtract(&turned, &knife, tol()) else {
            panic!("the patch builds");
        };
        for y in [0.5, 0.2, -0.5] {
            let plane = topo::test_support::split_plane(
                Point3::new(0.0, y, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                tol(),
            );
            let r = topo::split(&patch.body, &plane, tol());
            eprintln!(
                "MEASURE split spin {spin} at y = {y}: {}",
                match r {
                    Ok(_) => "splits".to_string(),
                    Err(e) => format!("{e:?}"),
                }
            );
        }
    }
}

fn prism(shape: &[((f64, f64), f64)], phi: f64, by: (f64, f64), z0: f64, h: f64) -> Body<f64> {
    let (sin, cos) = phi.sin_cos();
    let lp = profile::test_support::bulge_loop(
        shape
            .iter()
            .map(|&((x, y), b)| {
                (
                    Point2::new(cos * x - sin * y + by.0, sin * x + cos * y + by.1),
                    b,
                )
            })
            .collect(),
    );
    let plane = profile::SketchPlane::xy_at(geom_core::Point3::new(0.0, 0.0, z0));
    let p = profile::Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    let along = sweep::Extrusion::Distance {
        depth: h,
        side: sweep::ExtrudeSide::Along,
    };
    sweep::extrude(&p, along, tol()).unwrap().body.into_body()
}
