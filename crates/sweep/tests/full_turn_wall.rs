//! **A face that alone wraps its carrier's azimuth gets a verdict.**
//!
//! The fixture is a drilled BEAD: the lens `ρ = 1/2` (the bore line)
//! and the arc of the unit circle through `ρ = 1` revolved a full turn
//! about `y`. That is two faces, each the whole turn of its carrier: a
//! bore CYLINDER of radius `1/2` with a self-mated seam, and a SPHERE
//! zone between the latitudes `y = ±√3/2`. Their two rim circles are
//! the whole boundary, so neither face has an azimuth to trim by.
//!
//! - The face door ([`topo::curved_face_containment`]) places a point
//!   on either carrier by its height (latitude) window alone: `In`
//!   inside it, `Out` past a rim.
//! - The solid door ([`topo::point_in_solid`]) reads the bore through
//!   the same class, so a ray through the bore wall is a crossing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use crate::revolve_common::{axis_y, validated};
use geom::Surface;
use geom_core::{Point2, Point3, Tol};
use profile::test_support::bulge_loop;
use sweep::{Revolution, revolve};
use topo::{Body, FaceContainment, FaceKey, SolidContainment, curved_face_containment, point_in_solid};

const BORE: f64 = 0.5;

/// The rims' height: the unit sphere meets the bore at `y = ±√3/2`.
fn rim() -> f64 {
    (1.0 - BORE * BORE).sqrt()
}

/// The drilled bead, and its (cylinder, sphere) faces.
fn bead() -> (Body<f64>, FaceKey, FaceKey) {
    let h = rim();
    // The arc from (½, −h) to (½, h) through ρ = 1 turns CCW about the
    // origin by 2φ, φ = atan(h/½): bulge tan(φ/2).
    let phi = (h / BORE).atan();
    let lp = bulge_loop(vec![
        (Point2::new(BORE, -h), (phi / 2.0).tan()),
        (Point2::new(BORE, h), 0.0),
    ]);
    let body = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    let of_kind = |cyl: bool| {
        let found: Vec<FaceKey> = body
            .faces()
            .filter(|(_, f)| match body.get_surface(f.surface) {
                Some(Surface::Cylinder { .. }) => cyl,
                Some(Surface::Sphere { .. }) => !cyl,
                other => panic!("the bead has only a bore and a zone, got {other:?}"),
            })
            .map(|(k, _)| k)
            .collect();
        assert_eq!(found.len(), 1, "one full-turn face per carrier: {found:?}");
        found[0]
    };
    let (wall, zone) = (of_kind(true), of_kind(false));
    (body, wall, zone)
}

/// A point at radius `rho` from the `y` axis, height `y`, azimuth `a`
/// (clear of the seam at `a = 0`).
fn at(rho: f64, y: f64, a: f64) -> Point3<f64> {
    Point3::new(rho * a.cos(), y, rho * a.sin())
}

#[test]
fn the_face_door_places_points_on_a_full_turn_bore() {
    let (body, wall, _) = bead();
    let place = |p| curved_face_containment(&body, wall, p, band()).unwrap();
    for a in [2.0, 4.0] {
        assert_eq!(
            place(at(BORE, 0.3, a)),
            Some(FaceContainment::In),
            "bore, inside the height window, azimuth {a}"
        );
        assert_eq!(
            place(at(BORE, rim() + 0.1, a)),
            Some(FaceContainment::Out),
            "bore carrier, past the top rim, azimuth {a}"
        );
    }
}

#[test]
fn the_face_door_places_points_on_a_full_turn_zone() {
    let (body, _, zone) = bead();
    let place = |face, p| curved_face_containment(&body, face, p, band()).unwrap();
    for a in [2.0, 4.0] {
        let rho = |y: f64| (1.0 - y * y).sqrt();
        assert_eq!(
            place(zone, at(rho(0.2), 0.2, a)),
            Some(FaceContainment::In),
            "zone, inside the latitude window, azimuth {a}"
        );
        assert_eq!(
            place(zone, at(rho(0.95), 0.95, a)),
            Some(FaceContainment::Out),
            "sphere carrier, in the cap the bore removed, azimuth {a}"
        );
    }
}

#[test]
fn the_solid_door_reads_the_full_turn_bore() {
    let (body, _, _) = bead();
    let ask = |p| point_in_solid(&body, p, band(), Tol::witness());
    assert_eq!(
        ask(at(0.75, 0.1, 2.0)).unwrap(),
        SolidContainment::In,
        "in the bead's material"
    );
    assert_eq!(
        ask(at(0.0, 0.1, 0.0)).unwrap(),
        SolidContainment::Out,
        "in the bore"
    );
    assert_eq!(
        ask(at(BORE, 0.1, 2.0)).unwrap(),
        SolidContainment::OnBoundary,
        "on the bore wall"
    );
}

#[test]
fn probe_full_turn_collar_rest() {
    use crate::mate2_common::{peg, volume, wall_decls};
    use geom_core::{Affine3, Vec3};
    let lp = bulge_loop(vec![
        (Point2::new(0.5, 1.0), 0.0),
        (Point2::new(1.5, 1.0), 0.0),
        (Point2::new(1.5, 2.0), 0.0),
        (Point2::new(0.5, 2.0), 0.0),
    ]);
    let collar = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    let to_y = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        -core::f64::consts::FRAC_PI_2,
    );
    for (z0, h) in [(1.0, 1.0), (0.5, 2.0)] {
        let p = topo::transform_rigid(&peg(z0, h), &to_y, Tol::witness()).unwrap();
        let decls = wall_decls(&collar, &p);
        eprintln!("PROBE decls {}", decls.coincident_faces.len());
        match topo::union_with(&collar, &p, &decls, Tol::witness()) {
            Ok(r) => {
                let b = crate::mate2_common::body_of(r);
                eprintln!(
                    "PROBE ({z0},{h}) OK vol {} vs {} + {} ; valid {:?}",
                    volume(&b),
                    volume(&collar),
                    volume(&p),
                    topo::validate_geometric(&b, Tol::witness()).is_ok()
                );
            }
            Err(e) => eprintln!("PROBE ({z0},{h}) ERR {e:?}"),
        }
    }
}
