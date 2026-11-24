//! **The faces whose region the ruled reader cannot read**
//! ([`FaceGeo::PartialWall`], [`FaceGeo::PartialCone`]): held as the
//! ball about a point that holds the face ([`partial_wall`],
//! [`partial_cone`]), a query refuses only where that ball could decide
//! it, and answers past it.
//!
//! No door builds such a face here: a wall or cone edge that is not a
//! line, a circle or an ellipse certifies only as an intersection with
//! a surface of its other face, which a sheet on one carrier does not
//! have (a spline ruling re-described on the wall's own chart is
//! `Unimplemented`). So each row builds the record with the same doors
//! `face_geo` builds it with, on a face whose region is readable, and
//! reads it where `face_geo` would hand it on.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::*;
use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness band")
}

/// The unit wall over a quarter turn `u ∈ [0, π/2]`, `z ∈ [0, 1]`, as a
/// partial wall: the ball about its first boundary point, which is
/// `(1, 0, 0)`.
fn wall<T: Decide + crate::props::AtRestPolicy>() -> (Body<T>, FaceKey, FaceGeo<T>) {
    let mut body = Body::<T>::new();
    let face = cyl_wall_sheet::<T>(
        &mut body,
        CylFrame::canonical(1.0),
        (0.0, core::f64::consts::FRAC_PI_2),
        (0.0, 1.0),
        Tol::witness(),
    );
    let axis = geom::Line {
        origin: Point3::origin(),
        dir: UnitVec3::exact_z(),
    };
    let geo = partial_wall(&body, face, (axis, T::one()), band()).expect("the partial wall");
    (body, face, geo)
}

fn on_wall<T: Decide>(u: f64, z: f64) -> Point3<T> {
    let f = T::from_f64;
    Point3::new(f(u.cos()), f(u.sin()), f(z))
}

/// **A partial wall refuses a point on its carrier only inside the ball
/// holding it**: the face's far corner from its anchor, `(0, 1, 1)`,
/// its middle, and a point of the wall off the face but within the ball
/// refuse; a point 50 m up the carrier is off the face. At `f64` and on
/// the `Interval` lane. Red against a ball that does not hold the face
/// (half its radius, or moved off the anchor), which passes the far
/// corner as off the face.
#[test]
fn a_partial_wall_refuses_only_inside_its_ball() {
    fn at<T: Decide + crate::props::AtRestPolicy>() {
        let (body, face, geo) = wall::<T>();
        for (what, q) in [
            (
                "the far corner",
                on_wall::<T>(core::f64::consts::FRAC_PI_2, 1.0),
            ),
            (
                "the face's middle",
                on_wall::<T>(core::f64::consts::FRAC_PI_4, 0.5),
            ),
            ("off the face within the ball", on_wall::<T>(-0.3, 0.5)),
        ] {
            assert!(
                matches!(
                    face_holds(&body, face, &geo, q, band()),
                    Err(PointInSolidError::WallOutlineUnsupported { .. })
                ),
                "{what} refuses"
            );
        }
        assert!(
            matches!(
                face_holds(&body, face, &geo, on_wall::<T>(0.7, 50.0), band()),
                Ok(false)
            ),
            "a point 50 m up is off the face"
        );
    }
    at::<f64>();
    at::<geom_core::Interval>();
}

/// **A ray past a partial wall's ball crosses it; one through the face
/// is blocked**: along `x` at `y = 0.5`, 50 m up, the ray meets the
/// wall only outside the ball holding the face, and its crossings read;
/// at `z = 0.5` it meets the face, which blocks it. At `f64` and on the
/// `Interval` lane. Red against a ball that does not hold the face, and
/// against the stand-off read Negative everywhere.
#[test]
fn a_ray_past_a_partial_walls_ball_crosses_it() {
    fn at<T: Decide + crate::props::AtRestPolicy>() {
        let (body, face, geo) = wall::<T>();
        let geos = [(face, geo)];
        let f = T::from_f64;
        let ray = |z: f64| geom::Ray::new(Point3::new(f(-3.0), f(0.5), f(z)), UnitVec3::exact_x());
        let arm = geom_core::test_support::fixture_lever(f(100.0));
        assert!(
            ray_crossings(&body, &geos, &ray(50.0), arm, band(), SCHEDULE_ROWS).is_ok(),
            "a ray past the ball crosses"
        );
        assert!(
            matches!(
                ray_crossings(&body, &geos, &ray(0.5), arm, band(), SCHEDULE_ROWS),
                Err(RayFault::Blocked(
                    PointInSolidError::WallOutlineUnsupported { .. }
                ))
            ),
            "a ray through the face is blocked"
        );
    }
    at::<f64>();
    at::<geom_core::Interval>();
}

/// The cone of half-angle π/4 about `z` from the origin, its face up to
/// `z = 2` ([`crate::test_support_fixtures::cone_cap`]), as a partial
/// cone: the ball about the apex, out to the rim's `2√2`.
fn cone() -> (Body<f64>, FaceKey, FaceGeo<f64>) {
    let alpha = core::f64::consts::FRAC_PI_4;
    let (body, face) = crate::test_support_fixtures::cone_cap(alpha, 2.0, Tol::witness());
    let axis = geom::Line {
        origin: Point3::origin(),
        dir: UnitVec3::exact_z(),
    };
    let geo = partial_cone(&body, face, (axis, alpha), band()).expect("the partial cone");
    (body, face, geo)
}

/// **A partial cone refuses a point on its carrier only inside the ball
/// holding it, and a ray past that ball crosses it**: the rim point
/// `(2, 0, 2)` and a point on the face refuse, a point 50 m up the cone
/// is off the face; a ray along `x` 50 m up meets the cone only outside
/// the ball and crosses, one at `z = 1` meets the face and is blocked.
/// At `f64` only: the cone fixture is `f64`'s. Red against a ball that
/// does not hold the face, and against the stand-off read Negative
/// everywhere (the ray 50 m up then blocks).
#[test]
fn a_partial_cone_refuses_only_inside_its_ball() {
    let (body, face, geo) = cone();
    for (what, q) in [
        ("the rim", Point3::new(2.0, 0.0, 2.0)),
        ("the face", Point3::new(0.0, 1.0, 1.0)),
    ] {
        assert!(
            matches!(
                face_holds(&body, face, &geo, q, band()),
                Err(PointInSolidError::PartialConeFace { .. })
            ),
            "{what} refuses"
        );
    }
    assert!(
        matches!(
            face_holds(&body, face, &geo, Point3::new(0.0, 50.0, 50.0), band()),
            Ok(false)
        ),
        "a point 50 m up is off the face"
    );
    let geos = [(face, geo)];
    let ray = |z: f64| geom::Ray::new(Point3::new(-2.0 * z - 1.0, 0.3, z), UnitVec3::exact_x());
    let arm = |z: f64| geom_core::test_support::fixture_lever(4.0 * z + 2.0);
    assert!(
        ray_crossings(&body, &geos, &ray(50.0), arm(50.0), band(), SCHEDULE_ROWS).is_ok(),
        "a ray past the ball crosses"
    );
    let through = ray_crossings(&body, &geos, &ray(1.0), arm(1.0), band(), SCHEDULE_ROWS);
    assert!(
        matches!(
            through,
            Err(RayFault::Blocked(PointInSolidError::PartialConeFace { .. }))
        ),
        "a ray through the face is blocked: {:?}",
        through.err()
    );
}

/// **A trimmable cone face's roots are read over the run that reaches
/// its ball about the apex**: on the cone face of half-angle π/4 up to
/// its rim at `z = 2` (the ball's radius `2√2`), a ray along `x` at
/// `y = 0.3`, `z = 1.9` meets the cone about `2.7` from the apex, inside
/// the ball, and both roots are read. Red against the ball read at half
/// its radius, whose run stops short of them.
#[test]
fn a_trimmable_cones_roots_are_read_over_its_balls_run() {
    let (body, face) =
        crate::test_support_fixtures::cone_cap(core::f64::consts::FRAC_PI_4, 2.0, Tol::witness());
    let charts = SolidFaces::of_body(&body).expect("its faces").charts;
    let geos = [(
        face,
        face_geo(&body, face, &charts, band()).expect("the face's geometry"),
    )];
    assert!(
        matches!(geos[0].1, FaceGeo::Cone { .. }),
        "a trimmable cone face"
    );
    let ray = geom::Ray::new(Point3::new(-5.0, 0.3, 1.9), UnitVec3::exact_x());
    let (roots, missed) =
        carrier_roots(&geos, &ray, 10.0, band(), SCHEDULE_ROWS).expect("the roots decide");
    assert!(missed.is_empty(), "the ray meets the cone");
    assert_eq!(roots.len(), 2, "both roots are read");
}
