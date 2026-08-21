//! Acceptance (c): the washer — a rectangle off the axis revolved
//! fully. Genus 1 via the same-shell `kfmrh` seam closure; component
//! Euler–Poincaré verified; Seam/Intersection descriptions checked
//! end-to-end from public ops.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom::Surface;
use geom_brep::EdgeGeometry;
use profile::ProfileLoop;
use profile::RawLoop;
use revolve_common::*;
use sweep::{Revolution, RevolvedKind, revolve};
use geom_core::Tol;

/// The rectangle x ∈ [1, 2], y ∈ [0, 1], counterclockwise.
fn washer_profile() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)])
}

#[test]
fn washer_full_revolve_is_genus_one_and_tier_valid() {
    let vp = validated(vec![washer_profile()]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    // V4 E8 F4 R0: component E–P v − e + f − r = 0 = 2(1 − g) ⇒ g = 1
    // (the kfmrh genus supplier).
    assert_eq!(counts(&t.body), (4, 8, 4, 0));
    // One shell, one solid.
    assert_eq!(t.body.solids().count(), 1);
    assert_eq!(t.body.shells().count(), 1);
    // Four walls: two cylinders (r = 1, r = 2) and two plane annuli.
    let mut cylinders = 0;
    let mut planes = 0;
    for w in &t.walls[0] {
        let f = w.expect("no on-axis segments");
        match t
            .body
            .get_surface(t.body.get_face(f).unwrap().surface)
            .unwrap()
        {
            Surface::Cylinder { .. } => cylinders += 1,
            Surface::Plane { .. } => planes += 1,
            other => panic!("unexpected wall surface {other:?}"),
        }
    }
    assert_eq!((cylinders, planes), (2, 2));
    // Every rim is a full-period Intersection (definitely transverse
    // plane × cylinder), witness at the antipode.
    for r in &t.rims[0] {
        let e = r.expect("no on-axis vertices");
        assert!(matches!(
            description(&t.body, e),
            EdgeGeometry::Intersection { .. }
        ));
    }
    // Meridians: cylinder walls carry Seam, plane walls keep the
    // conventional MappedCurve (module-doc exception).
    let RevolvedKind::Full {
        meridians,
        pi_walls,
        pi_meridians,
        pi_rims,
    } = &t.kind
    else {
        panic!("full revolve");
    };
    // Lamina case: one full-period band, no π-band entities.
    assert!(pi_walls.iter().all(Option::is_none));
    assert!(pi_meridians.iter().all(Option::is_none));
    assert!(pi_rims.iter().all(Option::is_none));
    let mut seams = 0;
    let mut mapped = 0;
    for m in meridians {
        match description(&t.body, m.expect("no omitted segments")) {
            EdgeGeometry::Seam { .. } => seams += 1,
            EdgeGeometry::MappedCurve(_) => mapped += 1,
            other => panic!("unexpected meridian description {other:?}"),
        }
    }
    assert_eq!((seams, mapped), (2, 2));
    // Orientation oracle: positive material volume (exact value
    // 2π·R̄·A = 2π·1.5·1 ≈ 9.42; chordal sampling only bounds it
    // loosely — the SIGN is the oracle).
    assert!(signed_volume(&t.body) > 0.0);
}

#[test]
fn donut_two_arc_profile_shares_one_torus() {
    // A 2-vertex circle profile (two semicircular arcs on one carrier)
    // off the axis: the cosurface run makes BOTH walls share one torus
    // key; both rims are same-key joins (conventional `RevolvedPoint`
    // survives — definitely smooth); both meridian arcs lie on the
    // torus's u = 0 minor circle and carry `Seam { torus }`. Also the
    // minimal (m = 2) exercise of the kfmrh + zip closure.
    let lp = ProfileLoop::new(vec![
        profile::ProfileVertex::new(p2(1.0, 0.5), 1.0),
        profile::ProfileVertex::new(p2(2.0, 0.5), 1.0),
    ]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    // V2 E4 F2 R0: v − e + f − r = 0 ⇒ genus 1.
    assert_eq!(counts(&t.body), (2, 4, 2, 0));
    // One shared torus surface.
    assert_eq!(t.body.surfaces().count(), 1);
    let k0 = t.body.get_face(t.walls[0][0].unwrap()).unwrap().surface;
    let k1 = t.body.get_face(t.walls[0][1].unwrap()).unwrap().surface;
    assert_eq!(k0, k1);
    assert!(matches!(
        t.body.get_surface(k0),
        Some(Surface::Torus { .. })
    ));
    // Full-period rims stay conventional (same surface key each side).
    for r in &t.rims[0] {
        assert!(matches!(
            description(&t.body, r.unwrap()),
            EdgeGeometry::MappedCurve(_)
        ));
    }
    // Both meridians are the torus's seam.
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full revolve");
    };
    for m in meridians {
        assert!(matches!(
            description(&t.body, m.unwrap()),
            EdgeGeometry::Seam { .. }
        ));
    }
    assert!(signed_volume(&t.body) > 0.0);
}
