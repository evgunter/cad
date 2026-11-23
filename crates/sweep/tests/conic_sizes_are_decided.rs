//! **A door handed a size with no finite value refuses it typed.** The
//! blend arms and the offset doors mint circles from the size they are
//! handed, and `geom::Circle::new` panics on a radius that is not
//! finite.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/sweep/src/blend/",
    "crates/topo/src/replace_face.rs",
    "crates/topo/src/offset_axial.rs",
    "crates/geom-brep/src/intersect.rs",
    "crates/geom/src/curves/conic.rs",
];

use geom::Surface;
use geom_core::{Band, Tol, Vec3};
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{ball_poled_z, dome, finished, lantern, rim_arcs_at, sphere_zone};
use topo::{Body, FaceKey};

/// The lantern's lip is a cone × cap rim: two straight traces, which
/// cross at every radius, so no room is decided before the arm reads
/// the ball's centre.
///
/// Mutant: `nonpositive_size_gate` without its finite upper end panics
/// in `Circle::new: radius NaN` on the fillet.
#[test]
fn a_blend_of_infinite_size_is_refused_at_the_door() {
    let t = Tol::witness();
    let body = finished("the lantern", lantern(t), t);
    let lip = rim_arcs_at(&body, 0.2, 1.2);
    assert!(!lip.is_empty(), "the lantern has its lip rim");
    let f = fillet_edges(&body, &lip, f64::INFINITY, t).expect_err("no ball is infinite");
    assert!(
        matches!(f.error, BlendError::NonpositiveSize { size } if size == f64::INFINITY),
        "fillet: {f:?}"
    );
    let c = chamfer_edges(&body, &lip, f64::INFINITY, t).expect_err("no setback is infinite");
    assert!(
        matches!(c.error, BlendError::NonpositiveSize { size } if size == f64::INFINITY),
        "chamfer: {c:?}"
    );
}

fn sphere_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .map(|(k, _)| k)
        .filter(|&k| matches!(body.face_surface(k), Some(Surface::Sphere { .. })))
        .collect()
}

/// A sphere moved by an infinite distance, or by one whose section
/// radius is past the float range, has no circle to meet its cap in.
///
/// Mutant: `plane_sphere_section_read` without the read of the radius'
/// width panics in `Circle::new: radius inf` at the first move.
#[test]
fn a_sphere_face_moved_past_every_finite_radius_refuses() {
    let t = Tol::witness();
    for (name, body) in [
        ("the dome", dome(1.0, t)),
        ("the zone", sphere_zone(0.6, Revolution::Full, t)),
    ] {
        let faces = sphere_faces(&body);
        assert!(!faces.is_empty(), "{name} has a sphere face");
        for face in faces {
            for d in [f64::INFINITY, f64::MAX, 1e306] {
                let mut moved = body.clone();
                let got = topo::replace_face_offset(&mut moved, face, d, t);
                assert!(got.is_err(), "{name}, {d:e}: the move refuses: {got:?}");
            }
        }
    }
}

/// A ball's seam is a great circle of its sphere, moved concentrically:
/// its radius is the stored circle's plus the distance.
///
/// Mutant: `offset_axial::moved_seam` storing its radius undecided
/// panics in `Circle::new: radius inf`.
#[test]
fn a_ball_moved_by_an_infinite_distance_refuses() {
    let t = Tol::witness();
    let band = Band::linear(t).unwrap();
    let body = ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), t);
    let faces = sphere_faces(&body);
    assert!(!faces.is_empty(), "a ball has its sphere faces");
    let mut moved = body.clone();
    let got = topo::offset_charts_together(
        &mut moved,
        &[topo::ChartMove {
            faces,
            distance: f64::INFINITY,
        }],
        band,
        t,
    );
    assert!(got.is_err(), "the move refuses: {got:?}");
}
