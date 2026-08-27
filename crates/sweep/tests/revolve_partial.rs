//! Acceptance (d): the partial wedge — an axis-touching unit square
//! revolved by θ = ±π/2. The on-axis segment is an ordinary boundary
//! edge shared by the start and end wedge caps (no wall, no struts at
//! its endpoints); it upgrades to `Intersection { start cap, end cap }`
//! (the caps are definitely transverse for θ ≠ π).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use core::f64::consts::{FRAC_PI_2, PI};
use profile::RawLoop;

use geom_brep::EdgeDescription;
use geom_core::Tol;
use profile::ProfileLoop;
use revolve_common::*;
use sweep::{Revolution, RevolvedKind, revolve};

/// The unit square with its left edge ON the axis. Canonical segments:
/// 0 bottom (⊥ axis, plane), 1 right (∥ axis, cylinder), 2 top
/// (⊥ axis, plane), 3 left (ON axis, shared by the caps).
fn square() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)])
}

fn check_wedge(theta: f64) {
    let vp = validated(vec![square()]);
    let t = revolve(&vp, axis_y(), Revolution::Partial(theta), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    // V6 E9 F5 R0 (2 shared axis vertices + 2 × 2 swept corners;
    // 4 chain + 3 copied chain + 2 latitude struts; 2 caps + 3 walls):
    // v − e + f − r = 2 ⇒ genus 0.
    assert_eq!(counts(&t.body), (6, 9, 5, 0));
    let RevolvedKind::Partial {
        start_cap,
        end_cap,
        start_meridians,
        end_meridians,
    } = &t.kind
    else {
        panic!("partial revolve");
    };
    assert_ne!(start_cap, end_cap);
    // The axis segment (canonical 3) has no wall and ONE shared edge.
    assert_eq!(t.walls[0][3], None);
    assert_eq!(start_meridians[0][3], end_meridians[0][3]);
    let axis_edge = start_meridians[0][3];
    assert!(matches!(
        description(&t.body, axis_edge),
        EdgeDescription::Intersection { .. }
    ));
    // Walled segments have distinct start/end meridians, upgraded to
    // cap–wall Intersections.
    for j in 0..3 {
        assert!(t.walls[0][j].is_some());
        assert_ne!(start_meridians[0][j], end_meridians[0][j]);
        for e in [start_meridians[0][j], end_meridians[0][j]] {
            assert!(matches!(
                description(&t.body, e),
                EdgeDescription::Intersection { .. }
            ));
        }
    }
    // Latitude struts exist exactly at the off-axis corners (canonical
    // vertices 1, 2) and are wall–wall Intersections (plane × cylinder).
    assert_eq!(t.rims[0][0], None);
    assert_eq!(t.rims[0][3], None);
    for v in [1, 2] {
        let e = t.rims[0][v].expect("off-axis strut");
        assert!(matches!(
            description(&t.body, e),
            EdgeDescription::Intersection { .. }
        ));
    }
    // Orientation oracle holds for BOTH sweep directions.
    assert!(signed_volume(&t.body) > 0.0, "theta = {theta}");
}

#[test]
fn wedge_positive_angle() {
    check_wedge(FRAC_PI_2);
}

#[test]
fn wedge_negative_angle() {
    check_wedge(-FRAC_PI_2);
}

#[test]
fn half_revolve_axis_edge_stays_conventional() {
    // θ = π: the two cap planes are coplanar — the axis edge's
    // dihedral is definitely smooth, so it keeps its conventional
    // MappedCurve description (the D2 split; tier 3 permits it).
    let vp = validated(vec![square()]);
    let t = revolve(&vp, axis_y(), Revolution::Partial(PI), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let RevolvedKind::Partial {
        start_meridians, ..
    } = &t.kind
    else {
        panic!("partial revolve");
    };
    assert!(matches!(
        description(&t.body, start_meridians[0][3]),
        EdgeDescription::Scaffold(_)
    ));
    assert!(signed_volume(&t.body) > 0.0);
}

#[test]
fn off_axis_wedge_with_hole_is_extrude_shaped() {
    // A profile away from the axis with a hole: full extrude-shaped
    // machinery (hole planted through kemr + kfmrh in the start cap).
    // Square x ∈ [1, 3], y ∈ [0, 2]; square hole x ∈ [1.5, 2.5],
    // y ∈ [0.5, 1.5] (holes are canonicalized clockwise by validation).
    let outer = ProfileLoop::polygon([p2(1.0, 0.0), p2(3.0, 0.0), p2(3.0, 2.0), p2(1.0, 2.0)]);
    let hole = ProfileLoop::polygon([p2(1.5, 0.5), p2(2.5, 0.5), p2(2.5, 1.5), p2(1.5, 1.5)]);
    let vp = validated(vec![outer, hole]);
    let t = revolve(
        &vp,
        axis_y(),
        Revolution::Partial(FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    // Wedge with a through-tunnel: genus 1 (v − e + f − r = 0 with
    // r = 2 cap rings): V16 E24 F10 R2.
    assert_eq!(counts(&t.body), (16, 24, 10, 2));
    assert!(signed_volume(&t.body) > 0.0);
}
