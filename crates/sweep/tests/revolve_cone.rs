//! Acceptance (b): the cone — the right triangle (0,0), (1,0), (0,1)
//! revolved fully about the y-axis. Exact profile: base edge
//! (0,0)→(1,0) (⊥ axis ⇒ plane disc), slant edge (1,0)→(0,1)
//! (oblique ⇒ cone, apex at (0,1)), closing edge (0,1)→(0,0) ON the
//! axis (omitted). One-band wire structure: V2 E2 F2 R0 — the apex
//! (the slant meridian's wrap tip, valence 1) and the base-rim corner;
//! the base rim one full-period circle (`Intersection { plane, cone }`);
//! the slant meridian the cone's seam, a wrap edge; the base disc one
//! face whose centre is no vertex.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use core::f64::consts::FRAC_PI_4;
use profile::RawLoop;

use geom::Surface;
use geom_brep::EdgeDescription;
use geom_core::{Point2, Tol};
use profile::ProfileLoop;
use revolve_common::*;
use sweep::{Revolution, RevolvedKind, revolve};

fn triangle() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ])
}

#[test]
fn cone_full_revolve_has_an_apex_and_certifies() {
    let vp = validated(vec![triangle()]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    // The apex and the base rim's vertex; the rim and the cone's one
    // meridian; the cone and ONE base disc (a plane wall is built whole,
    // its centre no vertex).
    assert_eq!(counts(&t.body), (2, 2, 2, 0));
    // Two surfaces total (one plane, one cone).
    assert_eq!(t.body.described_surfaces().count(), 2);
    // Walls: segment 0 (base) a plane, segment 1 (slant) a cone with
    // apex (0, 1, 0) and half-angle π/4; segment 2 (axis) omitted.
    let base = t.walls()[0][0].expect("base wall");
    assert!(matches!(
        t.body.face_surface(base),
        Ok(Surface::Plane { .. })
    ));
    let slant = t.walls()[0][1].expect("slant wall");
    let Ok(&Surface::Cone {
        frame, half_angle, ..
    }) = t.body.face_surface(slant)
    else {
        panic!("slant wall is a cone");
    };
    let apex = frame.origin();
    assert!(apex.distance(geom_core::Point3::new(0.0, 1.0, 0.0)) < 1e-12);
    assert!((half_angle - FRAC_PI_4).abs() < 1e-12);
    assert_eq!(t.walls()[0][2], None);
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full revolve");
    };
    let meridians = &meridians[0];
    // The base rim (canonical vertex 1, the profile corner (1, 0)) is
    // one full-period Intersection, a self-loop; the apex has none.
    let rim = t.rims[0][1].expect("base rim");
    assert!(matches!(
        description(&t.body, rim),
        EdgeDescription::Intersection { .. }
    ));
    let (a, b) = t.body.edge_vertices(rim).unwrap();
    assert_eq!(a, b, "the full-period rim is a self-loop");
    assert_eq!(t.rims[0][0], None);
    assert_eq!(t.rims[0][2], None);
    // **Meridians.** The base disc has no meridian at all (a plane wall
    // is built whole); the cone's meridian has both halves in its one
    // face, so it is the cone's seam (D1), a wrap edge whose apex end is
    // its tip: the one edge at the apex.
    let slant_key = t.body.get_face(slant).unwrap().surface;
    assert!(meridians[0].is_none(), "a plane disc has no meridian");
    let slant_meridian = meridians[1].unwrap();
    assert_seam_of(&t.body, slant_meridian, slant_key);
    assert!(meridians[2].is_none());
    let tip = t.poles[0][2].expect("the apex is exported");
    let at_tip = t
        .body
        .edges()
        .filter(|&(e, _)| {
            let (a, b) = t.body.edge_vertices(e).unwrap();
            a == tip || b == tip
        })
        .map(|(e, _)| e)
        .collect::<Vec<_>>();
    assert_eq!(
        at_tip,
        vec![slant_meridian],
        "the apex is the meridian's tip"
    );
    // Orientation: the volume, signed (each face's flux under its
    // stored sense), positive and at the closed form.
    let v = measured_volume(&t.body);
    let exact = core::f64::consts::PI / 3.0;
    assert!(
        v > 0.0 && (v - exact).abs() / exact < 1e-9,
        "cone volume {v} vs {exact}"
    );
}
