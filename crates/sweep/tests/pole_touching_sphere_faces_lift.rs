//! **A sphere face with a pole on its boundary lifts to one sheet, and a
//! boolean on it does not refuse.** The section certificate's W2 reads a
//! loop through a pole as meeting the chart's singular set, and a face
//! with one such loop, every other loop winding nothing and no pole in
//! its interior, as lifting: its interior is a disc off the poles, so no
//! closed curve in it winds. The octant (a quarter disc revolved a
//! quarter turn) has one pole vertex, the lune (a half disc revolved a
//! half turn) two on one loop, and the full hemisphere's two half-bands
//! one each.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};
use geom_core::{Point2, Point3, Tol};
use sweep::Revolution;
use sweep::test_support::{brick, finished, hemisphere_on_flat_base, revolved_about_y};
use topo::{Body, BooleanResult, FaceKey, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

/// A quarter disc of radius 1 in the `xy` plane revolved by `turn` about
/// `y`.
fn octant(turn: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), (FRAC_PI_2 / 4.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Partial(turn),
        tol(),
    )
}

/// A half disc of radius 1 on the `y` axis revolved by `turn` about it.
fn lune(turn: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, -1.0), FRAC_PI_4.tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Partial(turn),
        tol(),
    )
}

/// Every sphere face of `body`, each asserted to be charted about `y`
/// (so its poles are the revolve's axis points) and to carry a pole
/// vertex.
fn pole_touching_sphere_faces(name: &str, body: &Body<f64>) -> Vec<FaceKey> {
    let poles = [Point3::new(0.0, 1.0, 0.0), Point3::new(0.0, -1.0, 0.0)];
    let faces: Vec<FaceKey> = body
        .faces()
        .filter_map(|(k, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Sphere { axis, .. }) => {
                assert!(
                    axis.x.abs() < 1e-12 && axis.z.abs() < 1e-12,
                    "{name}: the sphere is charted about y, {axis:?}"
                );
                Some(k)
            }
            _ => None,
        })
        .collect();
    assert!(!faces.is_empty(), "{name}: a sphere face");
    for &face in &faces {
        let fd = body.get_face(face).unwrap();
        let touches = core::iter::once(fd.outer)
            .chain(fd.rings.iter().copied())
            .filter_map(|l| match body.get_loop(l).unwrap().boundary {
                topo::LoopBoundary::Cycle { first } => Some(body.loop_cycle(first).unwrap()),
                topo::LoopBoundary::Empty { .. } => None,
            })
            .flatten()
            .any(|he| {
                let v = body.get_half_edge(he).unwrap().start;
                let at = topo::readback::vertex_point(body, v).unwrap();
                poles.iter().any(|p| (at - *p).norm() < 1e-12)
            });
        assert!(touches, "{name}: face {face:?} has a pole vertex");
    }
    faces
}

#[test]
fn a_sphere_face_with_a_pole_on_its_boundary_lifts_to_one_sheet() {
    for (name, body) in [
        ("the octant", octant(FRAC_PI_2)),
        ("the lune", lune(PI)),
        ("the hemisphere", hemisphere_on_flat_base(1.0, tol())),
    ] {
        for face in pole_touching_sphere_faces(name, &body) {
            assert!(
                topo::test_support::face_describes(&body, face, band()),
                "{name}: face {face:?} lifts to one sheet"
            );
        }
    }
}

/// The octant less, and with, a slab that cuts it along the latitude
/// `y = 0.99` near its pole and a box at its corner: each a body.
#[test]
fn a_boolean_on_a_pole_touching_octant_does_not_refuse() {
    let o = finished("the octant", octant(FRAC_PI_2), tol());
    for (name, b) in [
        (
            "the polar slab",
            brick::<f64>((-2.0, 2.0), (0.99, 2.0), (-2.0, 2.0), tol()),
        ),
        (
            "the corner box",
            brick::<f64>((0.2, 1.5), (0.2, 1.5), (-1.5, -0.2), tol()),
        ),
    ] {
        let b = finished(name, b, tol());
        for (op, r) in [("∖", subtract(&o, &b, tol())), ("∪", union(&o, &b, tol()))] {
            assert!(
                matches!(r, Ok(BooleanResult::Body(_))),
                "the octant {op} {name}: {r:?}"
            );
        }
    }
}
