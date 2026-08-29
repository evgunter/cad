//! Which door a crossing sphere pair reaches, per offset direction
//! relative to the operand's own chart.
//!
//! `ball_at` revolves an XY-plane profile about world Y, so the polar
//! axis is **Y** and the seam great circle is the circle of radius `r`
//! in the plane `z = c_z` (azimuth 0 = +X and its π copy). That single
//! fact splits the configuration space in two, and the split is
//! geometric, not tolerance-shaped:
//!
//! * **Offset along Z.** Every point of one ball's seam circle sits at
//!   the *same* distance `√(r² + Δz²)` from the other centre, so the
//!   seam is wholly inside or wholly outside the other sphere and can
//!   never cross it — at any depth and at any radius ratio. No crossing
//!   is found, the containment fallback runs, and its curved-extent
//!   scan refuses `FallbackExtentUnsupported`.
//! * **Offset along X or Y.** The seam circle now meets the other
//!   sphere, so a seam edge pierces a CURVED face and the reduce layer
//!   refuses `CurvedPierceUnsupported` — the pierce door, which is a
//!   layer *above* any germ-pair join arm. A sphere×sphere crossing
//!   therefore cannot reach the join in this build, whatever the germ
//!   frame knows about the section circle.
//!
//! Nested balls answer, and must keep answering.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, BooleanError};

/// A radius-`r` ball at `centre`, poles on world Y (the pip corpus's
/// constructor chart).
fn ball_at(r: f64, centre: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, -r), 1.0),
        ProfileVertex::new(Point2::new(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    topo::transform_rigid(&ball, &Affine3::translation(centre), Tol::witness()).unwrap()
}

fn union_err(a: &Body<f64>, b: &Body<f64>) -> BooleanError {
    topo::union(a, b, Tol::witness()).expect_err("a crossing sphere pair has no join lane")
}

/// A Z offset keeps both seams clear of the other sphere at every depth
/// and every radius ratio, so the scan — not the pierce — is the door.
#[test]
fn z_offset_pairs_refuse_at_the_curved_extent_scan() {
    for (r2, z2, label) in [
        (1.0, 1.9, "shallow, equal radii"),
        (1.0, 1.1, "deep, equal radii"),
        (0.2, 1.4, "unequal radii, seam wholly inside the big ball"),
    ] {
        let err = union_err(
            &ball_at(1.0, Vec3::new(2.0, 2.0, 0.5)),
            &ball_at(r2, Vec3::new(2.0, 2.0, z2)),
        );
        let BooleanError::FallbackExtentUnsupported { what, .. } = err else {
            panic!("{label}: expected the scan's typed refusal, got {err:?}");
        };
        assert!(what.contains("sphere"), "{label}: {what}");
    }
}

/// An in-seam-plane or polar-axis offset drives a seam meridian through
/// the other ball's sphere face: the curved PIERCE door, above the join.
/// The polar-axis row is the one a polar-aligned germ pair would take,
/// so no sphere×sphere section reaches a germ frame in this build.
#[test]
fn seam_crossing_pairs_refuse_at_the_curved_pierce() {
    for (centre, label) in [
        (
            Vec3::new(3.4, 2.0, 0.5),
            "offset along X, in the seam plane",
        ),
        (Vec3::new(2.0, 3.4, 0.5), "offset along Y, the polar axis"),
    ] {
        let err = union_err(
            &ball_at(1.0, Vec3::new(2.0, 2.0, 0.5)),
            &ball_at(1.0, centre),
        );
        let BooleanError::CurvedPierceUnsupported { .. } = err else {
            panic!("{label}: expected the curved pierce door, got {err:?}");
        };
    }
}

/// Nested balls never reach either door; the outer ball is the answer.
#[test]
fn nested_balls_still_answer() {
    let joined = topo::union(
        &ball_at(1.0, Vec3::new(2.0, 2.0, 0.5)),
        &ball_at(0.4, Vec3::new(2.0, 2.0, 0.5)),
        Tol::witness(),
    )
    .expect("nested balls answer");
    let joined = &joined.body().expect("a body").body;
    let v = topo::mass_properties(joined, Tol::witness())
        .unwrap()
        .volume;
    assert!((v - 4.0 * PI / 3.0).abs() < 1e-9, "{v}");
}
