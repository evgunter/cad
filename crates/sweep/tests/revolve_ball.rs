//! Acceptance (a): the ball — a half-disc profile (semicircular arc
//! plus an on-axis diameter segment) revolved fully. The axis edge is
//! OMITTED (it sweeps to nothing); its endpoints become the two
//! poles of a two-band sphere patch (the wire case's structure).
//!
//! Patch structure (documented per the PR 5 spec): V2 E2 F2 R0 —
//! two pole vertices at (0, ±1, 0), the angle-0 meridian (the profile
//! arc, `Seam { sphere }`) plus its angle-π copy (conventional
//! `MappedCurve` — the π half-plane is not the seam), and two
//! half-sphere band faces sharing ONE sphere surface key (the
//! two-band wire sweep: each pole has valence 2 — a one-band sphere
//! would leave valence-1 poles, which tier 2 bans as struts).
//! Component E–P: 2 − 2 + 2 − 0 = 2 = 2(1 − g) ⇒ g = 0 (an
//! axis-anchored wire adds no handle — no `kfmrh` in this path).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom::Surface;
use geom_core::Tol;
use profile::RawLoop;
use profile::{ProfileLoop, ProfileVertex};
use revolve_common::*;
use sweep::{Revolution, RevolvedKind, revolve};

/// The half-disc: semicircle from (0, −1) through (1, 0) to (0, 1)
/// (bulge tan(π/4) = 1), closed by the on-axis diameter. CCW.
fn half_disc() -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ])
}

#[test]
fn ball_full_revolve_omits_the_axis_edge_and_certifies() {
    let vp = validated(vec![half_disc()]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(counts(&t.body), (2, 2, 2, 0));
    // Both band faces share ONE sphere surface (the only surface).
    assert_eq!(t.body.surfaces().count(), 1);
    // Every face lies on the unit sphere centered at the origin.
    let (_, face) = t.body.faces().next().unwrap();
    let Surface::Sphere { center, radius, .. } = *t.body.get_surface(face.surface).unwrap() else {
        panic!("ball face is a sphere");
    };
    assert!(center.distance(geom_core::Point3::origin()).abs() < 1e-12);
    assert!((radius - 1.0).abs() < 1e-12);
    // The two vertices are the poles (0, ±1, 0).
    let mut ys: Vec<f64> = t
        .body
        .vertices()
        .map(|(_, v)| t.body.get_point(v.point).unwrap().y)
        .collect();
    ys.sort_by(f64::total_cmp);
    assert!((ys[0] + 1.0).abs() < 1e-12 && (ys[1] - 1.0).abs() < 1e-12);
    // Key bundle: the axis segment is omitted; the arc survives as the
    // sphere's Seam meridian; no rims (both vertices are poles).
    let RevolvedKind::Full {
        meridians,
        pi_walls,
        pi_meridians,
        pi_rims,
    } = &t.kind
    else {
        panic!("full revolve");
    };
    let meridians = &meridians[0];
    assert_eq!(meridians.len(), 2);
    let arc_edge = meridians[0].expect("canonical segment 0 is the arc");
    assert!(meridians[1].is_none(), "axis segment omitted");
    // The sphere key both bands share — the chart both meridians below
    // are images in.
    let sphere = t.body.get_face(t.walls[0][0].unwrap()).unwrap().surface;
    // The angle-0 meridian IS the sphere's parameterization seam:
    // derived by the kernel, carrying D1's seam obligation.
    assert_seam_of(&t.body, arc_edge, sphere);
    assert_eq!(t.walls[0][1], None);
    assert!(t.rims[0].iter().all(Option::is_none));
    // Both poles are EXPORTED (M9-D1), in canonical vertex order —
    // this body's only two vertices, south first.
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
    // The π band: wall + conventional π meridian; no rims (no interior
    // wire vertices).
    assert!(pi_walls[0].is_some() && pi_walls[1].is_none());
    let pi_arc = pi_meridians[0].expect("pi copy of the arc");
    // The angle-π copy is an ordinary image in the SAME chart, and the
    // profile's arc declared its locus.
    //
    // **Re-expressed at PCURVE P-1b.** This pair used to read
    // `IsoCurve` vs `MappedCurve`; U2 collapsed both into the one
    // conventional form, so a mechanical rewrite would have compared
    // `Chart` against `Chart` and stopped discriminating anything at
    // all. What actually told the two meridians apart was never the
    // variant: it was that one is the chart's seam and the other is a
    // profile entity's pushforward. Both facts are still stored — the
    // seam flag and the authority record (U2 Q3) — and the row now
    // reads them, pinning the shared chart as well.
    assert_declared_image_in(&t.body, pi_arc, sphere);
    assert!(pi_rims.iter().all(Option::is_none));
    // Orientation: positive volume. The two band faces' boundaries
    // (both meridians) are coplanar, so each face is fanned from its
    // interior pole-of-band point: band 1 covers z < 0 (a +θ rotation
    // about +y carries +x toward −z), band 2 covers z > 0.
    let v = signed_volume_lifted(
        &t.body,
        &[
            (
                t.walls[0][0].unwrap(),
                geom_core::Point3::new(0.0, 0.0, -1.0),
            ),
            (pi_walls[0].unwrap(), geom_core::Point3::new(0.0, 0.0, 1.0)),
        ],
    );
    assert!(v > 0.0, "ball volume {v}");
}
