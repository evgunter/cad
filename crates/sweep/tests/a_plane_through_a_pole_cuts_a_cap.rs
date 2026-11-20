//! **A plane through a sphere face's pole, tilted off its axis, cuts a
//! cap.** The unit hemisphere `y ≥ 0`, with its pole either where its two
//! meridians meet or at a wrap tip, cut by a plane through the pole
//! `(0, 1, 0)` with unit normal `n̂`. The section is a circle through the
//! pole. The join leaves the pole as a vertex, so the circle reaches the
//! pole as an edge's end, where the chart's azimuth names no point. The
//! edge's image reads its azimuth there off the reduced radial
//! (`geom_brep::PoleEnd`).
//!
//! The cap beyond the plane has height `h = 1 − d`, where `d = n̂_y` is the
//! plane's distance from the centre, and volume `π h² (3 − h)/3`. At each
//! normal the plane cuts the sphere inside `y ≥ 0`. At `(1, 1, 0)` and
//! `(0, 1, 1)` the circle touches the base rim at one point, and at
//! `(1, 2, 0)` it stays clear of it. Each side holds its closed form,
//! passes tiers 3 and 3′, and places a lattice of points as the
//! analytic region does. The one exception is the hemisphere less the
//! cap where the circle touches the rim: the face left there has a
//! zero-angle corner at the touch, and its measure refuses
//! (`work/nearpoint/a-sphere-face-with-a-zero-angle-corner-refuses-its-measure.md`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region, ball};
use crate::common::oracles::cap_volume;
use crate::common::wrap_tip::{NORTH, hemisphere, one_wrap_edge};
use core::f64::consts::PI;
use geom_core::{Point3, RigidMotion, Tol, UnitVec3, Vec3};
use sweep::test_support::finished;
use topo::{AtRestBody, BooleanResult, intersect, subtract};

fn tol() -> Tol {
    Tol::witness()
}

/// The box `[−3, 3] × [0, 3] × [−3, 3]`, turned so that its `+y` lies
/// along `n` and moved so that its floor passes through the pole. Its
/// floor is then the plane `n · (p − NORTH) = 0`, and the box holds the
/// side `n` points to, out past the unit ball.
fn tilted_box(n: Vec3<f64>) -> AtRestBody<f64> {
    let b = topo::test_support::brick::<f64>((-3.0, 3.0), (0.0, 3.0), (-3.0, 3.0), tol());
    let turn = if n.z == 0.0 {
        RigidMotion::rotation(UnitVec3::exact_z(), -n.x.atan2(n.y)).unwrap()
    } else {
        assert_eq!(n.x, 0.0, "a normal in the xy or the yz plane");
        RigidMotion::rotation(UnitVec3::exact_x(), n.z.atan2(n.y)).unwrap()
    };
    let b = topo::transform_rigid(&b, &turn).unwrap();
    let b = topo::transform_rigid(
        &b,
        &RigidMotion::translation(NORTH - Point3::origin()).unwrap(),
    )
    .unwrap();
    finished("the tilted box", b, tol())
}

fn body_of(r: Result<BooleanResult<f64>, topo::BooleanError>, what: &str) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{what}: {other:?}"),
    }
}

/// The lattice the rows place points on.
const LATTICE: (Point3<f64>, Point3<f64>) = (
    Point3::new(-1.15, -0.15, -1.15),
    Point3::new(1.15, 1.15, 1.15),
);

/// **Each hemisphere, with and less the half-space beyond a plane through
/// its pole, at the cap's closed form**, but for the rim-touching
/// remainders (module docs).
#[test]
fn a_plane_through_the_pole_cuts_each_hemisphere_at_its_cap() {
    let plain = finished("the hemisphere", hemisphere(), tol());
    let tipped = finished(
        "the tipped hemisphere",
        one_wrap_edge(hemisphere(), true).0,
        tol(),
    );
    for (n, touches_rim) in [
        (Vec3::new(1.0, 2.0, 0.0), false),
        (Vec3::new(1.0, 1.0, 0.0), true),
        (Vec3::new(0.0, 1.0, 1.0), true),
    ] {
        let unit = n * (1.0 / n.norm());
        let cap = cap_volume(1.0, 1.0 - unit.y);
        let tool = tilted_box(n);
        let beyond = move |p: Point3<f64>| unit.dot(p - NORTH);
        let hemi = || {
            let b = ball(Point3::origin(), 1.0);
            move |p: Point3<f64>| b(p).min(p.y)
        };
        for (which, body) in [("plain", &plain), ("tipped", &tipped)] {
            let what = format!("the {which} hemisphere ∩ the half-space along {n:?}");
            let h = hemi();
            let region: Region = Box::new(move |p| h(p).min(beyond(p)));
            let built = body_of(intersect(body, &tool, tol()), &what);
            membership::holds(&what, &built, cap, &region, LATTICE, 5);
            let what = format!("the {which} hemisphere ∖ the half-space along {n:?}");
            let h = hemi();
            let region: Region = Box::new(move |p| h(p).min(-beyond(p)));
            let rest = subtract(body, &tool, tol());
            if touches_rim {
                assert!(
                    matches!(
                        &rest,
                        Err(topo::BooleanError::ResultInvalid { findings })
                            if format!("{findings:?}").contains("props_sphere_loop_cusp")
                    ),
                    "{what}: the face left with a zero-angle corner at the rim refuses its \
                     measure, got {rest:?}"
                );
                continue;
            }
            let built = body_of(rest, &what);
            membership::holds(&what, &built, 2.0 * PI / 3.0 - cap, &region, LATTICE, 5);
        }
    }
}

/// **A cap through the pole, cut again through the pole.** The cap at
/// `(1, 2, 0)` is symmetric about `z = 0`, a plane that holds the pole and
/// the circle's far point. A second Boolean splits the circle at the far
/// point, and each child keeps its end on the pole, its image read off
/// that end again. Each side is half the cap.
#[test]
fn a_cap_cut_again_through_its_pole_halves() {
    let n = Vec3::new(1.0, 2.0, 0.0);
    let unit = n * (1.0 / n.norm());
    let half = cap_volume(1.0, 1.0 - unit.y) / 2.0;
    let tool = tilted_box(n);
    let slab = finished(
        "the half-space z > 0",
        topo::test_support::brick::<f64>((-3.0, 3.0), (-3.0, 3.0), (0.0, 3.0), tol()),
        tol(),
    );
    for (which, body) in [
        ("plain", finished("the hemisphere", hemisphere(), tol())),
        (
            "tipped",
            finished(
                "the tipped hemisphere",
                one_wrap_edge(hemisphere(), true).0,
                tol(),
            ),
        ),
    ] {
        let cap = body_of(intersect(&body, &tool, tol()), "the cap");
        let in_cap = move |p: Point3<f64>| {
            let b = ball(Point3::origin(), 1.0);
            b(p).min(p.y).min(unit.dot(p - NORTH))
        };
        let what = format!("the {which} cap ∩ z > 0");
        let region: Region = Box::new(move |p| in_cap(p).min(p.z));
        let built = body_of(intersect(&cap, &slab, tol()), &what);
        membership::holds(&what, &built, half, &region, LATTICE, 5);
        let what = format!("the {which} cap ∖ z > 0");
        let region: Region = Box::new(move |p| in_cap(p).min(-p.z));
        let built = body_of(subtract(&cap, &slab, tol()), &what);
        membership::holds(&what, &built, half, &region, LATTICE, 5);
    }
}
