//! Acceptance (b): the cone — the right triangle (0,0), (1,0), (0,1)
//! revolved fully about the y-axis. Exact profile: base edge
//! (0,0)→(1,0) (⊥ axis ⇒ plane disc), slant edge (1,0)→(0,1)
//! (oblique ⇒ cone, apex at (0,1)), closing edge (0,1)→(0,0) ON the
//! axis (omitted). Two-band wire structure: V4 E6 F4 R0 — the apex
//! and the disc center (each valence 2: the angle-0 and angle-π
//! meridians) plus the base-rim corner at angles 0 and π; the base
//! rim is two half-period arcs (`Intersection { plane, cone }`); the
//! angle-0 slant meridian is `Seam { cone }`, the angle-0 base
//! meridian conventional (plane wall), the angle-π copies likewise.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use core::f64::consts::FRAC_PI_4;
use profile::RawLoop;

use geom::Surface;
use geom_brep::EdgeGeometry;
use profile::ProfileLoop;
use revolve_common::*;
use sweep::{Revolution, RevolvedKind, revolve};
use geom_core::Tol;

fn triangle() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)])
}

#[test]
fn cone_full_revolve_has_an_apex_and_certifies() {
    let vp = validated(vec![triangle()]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(counts(&t.body), (4, 6, 4, 0));
    // Two surfaces total (one plane, one cone), each shared by its
    // two band faces.
    assert_eq!(t.body.surfaces().count(), 2);
    // Walls: segment 0 (base) a plane, segment 1 (slant) a cone with
    // apex (0, 1, 0) and half-angle π/4; segment 2 (axis) omitted.
    let base = t.walls[0][0].expect("base wall");
    assert!(matches!(
        t.body.get_surface(t.body.get_face(base).unwrap().surface),
        Some(Surface::Plane { .. })
    ));
    let slant = t.walls[0][1].expect("slant wall");
    let Some(&Surface::Cone {
        apex, half_angle, ..
    }) = t.body.get_surface(t.body.get_face(slant).unwrap().surface)
    else {
        panic!("slant wall is a cone");
    };
    assert!(apex.distance(geom_core::Point3::new(0.0, 1.0, 0.0)) < 1e-12);
    assert!((half_angle - FRAC_PI_4).abs() < 1e-12);
    assert_eq!(t.walls[0][2], None);
    let RevolvedKind::Full {
        meridians, pi_rims, ..
    } = &t.kind
    else {
        panic!("full revolve");
    };
    // The base rim (canonical vertex 1, the profile corner (1, 0)) is
    // two half-period Intersections; the pole/apex vertices have none.
    let rim = t.rims[0][1].expect("base rim, first half");
    assert!(matches!(
        description(&t.body, rim),
        EdgeGeometry::Intersection { .. }
    ));
    let rim2 = pi_rims[1].expect("base rim, second half");
    assert!(matches!(
        description(&t.body, rim2),
        EdgeGeometry::Intersection { .. }
    ));
    assert_eq!(t.rims[0][0], None);
    assert_eq!(t.rims[0][2], None);
    // Meridians: base conventional (plane wall), slant Seam, axis
    // omitted.
    assert!(matches!(
        description(&t.body, meridians[0].unwrap()),
        EdgeGeometry::MappedCurve(_)
    ));
    assert!(matches!(
        description(&t.body, meridians[1].unwrap()),
        EdgeGeometry::Seam { .. }
    ));
    assert!(meridians[2].is_none());
    // Orientation oracle: interior lift points per band face (the
    // band boundaries are coplanar — see the ball suite): band 1
    // covers z < 0, band 2 covers z > 0.
    let RevolvedKind::Full { pi_walls, .. } = &t.kind else {
        panic!("full revolve");
    };
    let v = signed_volume_lifted(
        &t.body,
        &[
            (base, geom_core::Point3::new(0.3, 0.0, -0.3)),
            (slant, geom_core::Point3::new(0.0, 0.5, -0.5)),
            (pi_walls[0].unwrap(), geom_core::Point3::new(0.3, 0.0, 0.3)),
            (pi_walls[1].unwrap(), geom_core::Point3::new(0.0, 0.5, 0.5)),
        ],
    );
    assert!(v > 0.0, "cone volume {v}");
}
