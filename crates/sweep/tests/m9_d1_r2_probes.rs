//! M9-D1 review probes (R2): the pole export's canonical
//! correspondence, attacked from the shapes the spec recorded as the
//! rejected direction-inference's failure modes.
//!
//! Ground truth is derived INDEPENDENTLY of the export: the validated
//! profile's canonical segment order gives each canonical vertex its
//! authored sketch point (`segments()[v].start`), and the exported
//! pole's body vertex must sit at that point's world position — for
//! the reversed traversal (full, and partial θ > 0) and the
//! non-reversed one (partial θ < 0) alike.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom_core::Point2;
use geom_core::Tol;
use profile::{ProfileLoop, ProfileVertex, RawLoop, ValidatedProfile};
use revolve_common::*;
use sweep::{Revolution, Revolved, revolve};
use topo::{validate, validate_closed};

/// The canonical vertex index whose authored sketch point is `p`.
fn canon_index(vp: &ValidatedProfile<f64>, li: usize, p: Point2<f64>) -> usize {
    let segs = vp.loops()[li].segments();
    segs.iter()
        .position(|s| s.start.distance(p) < 1e-12)
        .expect("authored point is a canonical vertex")
}

/// Asserts the exported pole at loop `li`, canonical vertex `v` is a
/// live body vertex at the world position of sketch point `p`
/// (XY-plane placement: world = (x, y, 0)).
fn assert_pole_at(t: &Revolved<f64>, li: usize, v: usize, p: Point2<f64>) {
    let pv = t.poles[li][v].expect("pole exported");
    let q = t
        .body
        .get_point(t.body.get_vertex(pv).unwrap().point)
        .unwrap();
    assert!(
        (q.x - p.x).abs() < 1e-12 && (q.y - p.y).abs() < 1e-12 && q.z.abs() < 1e-12,
        "pole pv({li},{v}) at {q:?}, expected {p:?}"
    );
}

/// An OFF-CENTER asymmetric ball: semicircle (0, 0.5) → (0, 2.5)
/// (bulge 1), closed on-axis. The two poles are at distinct axial
/// heights, so a swapped export cannot pass.
fn off_center_meridian() -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.5), 1.0),
        ProfileVertex::new(p2(0.0, 2.5), 0.0),
    ])
}

#[test]
fn full_off_center_ball_poles_land_on_their_authored_canonical_indices() {
    let vp = validated(vec![off_center_meridian()]);
    let lo = canon_index(&vp, 0, p2(0.0, 0.5));
    let hi = canon_index(&vp, 0, p2(0.0, 2.5));
    assert_ne!(lo, hi);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_pole_at(&t, 0, lo, p2(0.0, 0.5));
    assert_pole_at(&t, 0, hi, p2(0.0, 2.5));
    assert_eq!(t.poles[0].iter().flatten().count(), 2);
}

/// Direction safety: the SAME profile swept θ > 0 (reversed chains —
/// the rejected inference's failure shape) and θ < 0 (canonical
/// traversal) must export the SAME canonical correspondence.
#[test]
fn partial_wedge_pole_export_is_direction_safe_both_signs() {
    for theta in [core::f64::consts::FRAC_PI_2, -core::f64::consts::FRAC_PI_2] {
        let vp = validated(vec![off_center_meridian()]);
        let lo = canon_index(&vp, 0, p2(0.0, 0.5));
        let hi = canon_index(&vp, 0, p2(0.0, 2.5));
        let t = revolve(&vp, axis_y(), Revolution::Partial(theta), Tol::witness()).unwrap();
        // Tiers 1-2 only: a band face with NO rims and non-coplanar
        // meridians is volume-uncomputable (props_band_coplanar), a
        // mass-props scope limit orthogonal to the pole export —
        // issue #542. Tier 3 goes in here when that class computes.
        assert_eq!(validate(&t.body), Ok(()));
        assert_eq!(validate_closed(&t.body), Ok(()));
        assert_pole_at(&t, 0, lo, p2(0.0, 0.5));
        assert_pole_at(&t, 0, hi, p2(0.0, 2.5));
        assert_eq!(
            t.poles[0].iter().flatten().count(),
            2,
            "theta = {theta}: exactly the two pinned vertices export"
        );
    }
}

/// A full revolve whose axis run spans TWO segments: the run's
/// interior vertex is deleted with the omitted run (`None` export),
/// the run's END vertices are the poles, and the body carries no
/// unaccounted vertex (4 = 2 poles + the off-axis vertex's seam/π
/// copies).
#[test]
fn full_two_segment_axis_run_exports_run_ends_and_only_run_ends() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.5, 1.0), 0.0),
        ProfileVertex::new(p2(0.0, 2.0), 0.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let bot = canon_index(&vp, 0, p2(0.0, 0.0));
    let top = canon_index(&vp, 0, p2(0.0, 2.0));
    let mid = canon_index(&vp, 0, p2(0.0, 1.0));
    let off = canon_index(&vp, 0, p2(1.5, 1.0));
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_pole_at(&t, 0, bot, p2(0.0, 0.0));
    assert_pole_at(&t, 0, top, p2(0.0, 2.0));
    assert!(t.poles[0][mid].is_none(), "run interior vertex is deleted");
    assert!(t.poles[0][off].is_none(), "off-axis vertex is not a pole");
    // No live body vertex is unaccounted: 2 poles + 2 copies (seam/π)
    // of the single off-axis vertex.
    assert_eq!(counts(&t.body).0, 4);
}

/// A partial revolve with a HOLE loop: the outer loop's axis run still
/// exports its poles, the hole loop (never on-axis) exports all-None,
/// and the per-loop export indexing does not cross loops.
#[test]
fn partial_with_hole_exports_outer_poles_and_no_hole_poles() {
    let outer = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -2.0), 1.0),
        ProfileVertex::new(p2(0.0, 2.0), 0.0),
    ]);
    // A small CW square hole around (1, 0), strictly inside.
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.7, -0.3), 0.0),
        ProfileVertex::new(p2(0.7, 0.3), 0.0),
        ProfileVertex::new(p2(1.3, 0.3), 0.0),
        ProfileVertex::new(p2(1.3, -0.3), 0.0),
    ]);
    for theta in [core::f64::consts::FRAC_PI_2, -core::f64::consts::FRAC_PI_2] {
        let vp = validated(vec![outer.clone(), hole.clone()]);
        let lo = canon_index(&vp, 0, p2(0.0, -2.0));
        let hi = canon_index(&vp, 0, p2(0.0, 2.0));
        let t = revolve(&vp, axis_y(), Revolution::Partial(theta), Tol::witness()).unwrap();
        // Tiers 1-2 only: a band face with NO rims and non-coplanar
        // meridians is volume-uncomputable (props_band_coplanar), a
        // mass-props scope limit orthogonal to the pole export —
        // issue #542. Tier 3 goes in here when that class computes.
        assert_eq!(validate(&t.body), Ok(()));
        assert_eq!(validate_closed(&t.body), Ok(()));
        assert_pole_at(&t, 0, lo, p2(0.0, -2.0));
        assert_pole_at(&t, 0, hi, p2(0.0, 2.0));
        assert_eq!(t.poles[0].iter().flatten().count(), 2);
        assert!(
            t.poles[1].iter().all(Option::is_none),
            "hole vertices are never poles"
        );
    }
}

/// The ball with a SUBDIVIDED diameter: all three vertices on-axis,
/// the run spans two segments, and the surviving wire is the single
/// arc (k = 1 — `prev` is `first`). Poles at the run ends; the
/// subdivision vertex is deleted with the run.
#[test]
fn full_ball_with_subdivided_diameter_exports_run_end_poles_only() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let south = canon_index(&vp, 0, p2(0.0, -1.0));
    let north = canon_index(&vp, 0, p2(0.0, 1.0));
    let mid = canon_index(&vp, 0, p2(0.0, 0.0));
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_pole_at(&t, 0, south, p2(0.0, -1.0));
    assert_pole_at(&t, 0, north, p2(0.0, 1.0));
    assert!(t.poles[0][mid].is_none(), "subdivision vertex is deleted");
    assert_eq!(counts(&t.body).0, 2, "V2: the two poles only");
}
