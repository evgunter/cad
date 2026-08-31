//! Acceptance (c): the washer — a rectangle off the axis revolved
//! fully. Genus 1 via the same-shell `kfmrh` seam closure; component
//! Euler–Poincaré verified; Seam/Intersection descriptions checked
//! end-to-end from public ops.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom::Surface;
use geom_brep::EdgeDescription;
use geom_core::Tol;
use profile::ProfileLoop;
use profile::RawLoop;
use revolve_common::*;
use sweep::{Revolution, RevolvedKind, revolve};

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
            EdgeDescription::Intersection { .. }
        ));
    }
    // Meridians: the cylinder walls' carry their chart's own seam,
    // the plane walls' do not (module-doc exception — a plane chart is
    // not periodic) and stay images the profile segment declared.
    //
    // **Re-expressed at PCURVE P-1b.** The (2, 2) split is unchanged
    // and so is every fact it states; what changed is that the two
    // classes it counted were collapsed into one conventional form, so
    // a variant census would read (4, 0) and discriminate nothing. The
    // distinction that survived is the one this row was always about:
    // the seam obligation, and — the other half, now checked too —
    // whether a profile entity DECLARED the locus (U2 Q3's authority
    // record) or the kernel derived it.
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
    // Lamina case: one full-period band, no π-band entities.
    assert!(pi_walls.iter().all(Option::is_none));
    assert!(pi_meridians.iter().all(Option::is_none));
    assert!(pi_rims.iter().all(Option::is_none));
    let mut seams = 0;
    let mut declared = 0;
    for m in meridians {
        let e = m.expect("no omitted segments");
        let c = chart_image(&t.body, e);
        match (c.seam, authority(&t.body, e).is_declared()) {
            (true, false) => seams += 1,
            (false, true) => declared += 1,
            (seam, decl) => {
                panic!("a meridian that is neither: seam = {seam}, declared = {decl}")
            }
        }
    }
    assert_eq!((seams, declared), (2, 2));
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
    // Full-period rims stay conventional (same surface key each side,
    // so the surfaces under-determine the locus): images in that one
    // torus chart, declared by the profile's revolved vertex — never
    // the chart's seam, which the two meridians below are.
    for r in &t.rims[0] {
        assert_declared_image_in(&t.body, r.unwrap(), k0);
    }
    // Both meridians ARE the torus's seam: derived, seam obligation
    // carried. (Pre-U2 this pair of loops read `MappedCurve` against
    // `IsoCurve`; the taxonomy collapse merged those names, and the
    // seam flag plus the authority record are what the row was
    // discriminating with them.)
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full revolve");
    };
    let meridians = &meridians[0];
    for m in meridians {
        assert_seam_of(&t.body, m.unwrap(), k0);
    }
    // Orientation oracle: interior lift points per face. Both faces'
    // boundaries are the SAME pair of full-period rims (y = 0.5) plus
    // the two halves of the u = 0 seam meridian, so a fan from a
    // boundary vertex spans no volume at all — the two halves' fans
    // are mirror images and cancel identically, and
    // [`signed_volume`] returns a structural zero on this body rather
    // than a measurement. Lifting each face onto its own interior
    // gives the oracle something to measure: wall 0 is the LOWER half
    // (its probes span y ∈ [0, 0.5]), wall 1 the upper, and each lift
    // sits on that half's extreme minor-circle point, off the seam.
    let v = signed_volume_lifted(
        &t.body,
        &[
            (
                t.walls[0][0].unwrap(),
                geom_core::Point3::new(0.0, 0.0, 1.5),
            ),
            (
                t.walls[0][1].unwrap(),
                geom_core::Point3::new(0.0, 1.0, 1.5),
            ),
        ],
    );
    assert!(v > 0.0, "donut volume {v}");
}
