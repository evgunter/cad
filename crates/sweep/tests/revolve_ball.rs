//! Acceptance (a): the ball — a half-disc profile (semicircular arc
//! plus an on-axis diameter segment) revolved fully. The axis edge is
//! OMITTED (it sweeps to nothing); its endpoints become the two
//! poles of a one-face sphere (the wire case's structure).
//!
//! Patch structure: V2 E1 F1 R0 — two pole vertices at (0, ±1, 0), the
//! meridian (the profile arc, the sphere's seam, a wrap edge whose two
//! ends are the poles, each a wrap tip of valence 1: DESIGN
//! `[topology]`'s seam clause), and one sphere face whose loop walks it
//! pole to pole and back. Component E–P: 2 − 1 + 1 − 0 = 2 = 2(1 − g) ⇒
//! g = 0 (an axis-anchored wire adds no handle — no `kfmrh` in this
//! path).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom::Surface;
use geom_core::{Point2, Tol};
use profile::{ProfileLoop, test_support::bulge_loop};
use revolve_common::*;
use sweep::{Revolution, RevolvedKind, revolve};

/// The half-disc: semicircle from (0, −1) through (1, 0) to (0, 1)
/// (bulge tan(π/4) = 1), closed by the on-axis diameter. CCW.
fn half_disc() -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(0.0, -1.0), 1.0),
        (Point2::new(0.0, 1.0), 0.0),
    ])
}

#[test]
fn ball_full_revolve_omits_the_axis_edge_and_certifies() {
    let vp = validated(vec![half_disc()]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(counts(&t.body), (2, 1, 1, 0));
    // The one face's sphere is the only surface.
    assert_eq!(t.body.surfaces().count(), 1);
    // Every face lies on the unit sphere centered at the origin.
    let (face, _) = t.body.faces().next().unwrap();
    let Surface::Sphere { frame, radius, .. } = *t.body.face_surface(face).unwrap() else {
        panic!("ball face is a sphere");
    };
    let center = frame.origin();
    assert!(center.distance(geom_core::Point3::origin()).abs() < 1e-12);
    assert!((radius - 1.0).abs() < 1e-12);
    // The two vertices are the poles (0, ±1, 0).
    let mut ys: Vec<f64> = t.body.vertex_points().map(|(_, p)| p.y).collect();
    ys.sort_by(f64::total_cmp);
    assert!((ys[0] + 1.0).abs() < 1e-12 && (ys[1] - 1.0).abs() < 1e-12);
    // Key bundle: the axis segment is omitted; the arc survives as the
    // sphere's seam meridian; no rims (both vertices are poles).
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full revolve");
    };
    let meridians = &meridians[0];
    assert_eq!(meridians.len(), 2);
    let arc_edge = meridians[0].expect("canonical segment 0 is the arc");
    assert!(meridians[1].is_none(), "axis segment omitted");
    let wall = t.walls()[0][0].unwrap();
    let sphere = t.body.get_face(wall).unwrap().surface;
    // Both halves of the meridian bound the one face, so it is the
    // sphere's seam: a wrap edge, derived by the kernel.
    assert_seam_of(&t.body, arc_edge, sphere);
    assert_eq!(t.walls()[0][1], None);
    assert!(t.rims[0].iter().all(Option::is_none));
    // Both poles are EXPORTED (M9-D1), in canonical vertex order —
    // this body's only two vertices, south first — and each is a tip of
    // the meridian: the one edge at it.
    let poles: Vec<_> = t.poles[0].iter().map(|p| p.expect("pole")).collect();
    assert_eq!(poles.len(), 2);
    assert_ne!(poles[0], poles[1]);
    let py = |v| {
        t.body
            .get_point(t.body.get_vertex(v).unwrap().point)
            .unwrap()
            .y
    };
    assert!((py(poles[0]) + 1.0).abs() < 1e-12 && (py(poles[1]) - 1.0).abs() < 1e-12);
    let ends = t.body.edge_vertices(arc_edge).unwrap();
    assert!(
        [ends.0, ends.1] == [poles[0], poles[1]] || [ends.1, ends.0] == [poles[0], poles[1]],
        "the meridian runs pole to pole: {ends:?} vs {poles:?}"
    );
    // Orientation: the volume, signed (each face's flux under its
    // stored sense), positive and at the closed form.
    let v = measured_volume(&t.body);
    let exact = 4.0 * core::f64::consts::PI / 3.0;
    assert!(
        v > 0.0 && (v - exact).abs() / exact < 1e-9,
        "ball volume {v} vs {exact}"
    );
}
