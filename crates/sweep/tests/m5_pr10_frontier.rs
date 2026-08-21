//! M5 PR 10 §3/§5 — the NURBS-wall B-rep frontier, demonstrated and
//! then **FLIPPED (M6-3, the S9 pattern — history kept)**.
//!
//! As written at PR 10, these rows pinned tier 3's by-KIND refusal of
//! every non-analytic surface, so the PR that opened the frontier
//! would see exactly which doors it had to open. M6-3 opened them:
//! check 1 now refuses only the mvfs PLACEHOLDER, a described NURBS
//! face passes, and the loft BODY assembles tier-3 green
//! (`m6_loft_body.rs`). The second row below therefore asserts the
//! POST-flip truth: swapping a real skinned wall onto an extruded box
//! no longer draws `UncertifiableSurface` — what refuses now is the
//! honest GEOMETRY (the box's rims do not lie on that wall), which is
//! exactly the difference between rejecting a KIND and certifying a
//! surface.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::skin::{lift_surface, loft_geometry};
use sweep::{Extrusion, extrude};
use topo::{FaceSurface, validate_geometric};
use geom_core::Tol;

mod common;

/// A three-section loft of a square, middle section scaled — the
/// acceptance shape's wall set (§5's "at least one non-affine pair").
fn geometry() -> sweep::LoftGeometry {
    let chain = common::chain;
    let places = [0.0, 1.0, 2.0].map(|z| Affine3::translation(Vec3::new(0.0, 0.0, z)));
    loft_geometry(&[chain(1.0), chain(1.6), chain(1.0)], &places, 2, Tol::witness()).expect("the loft skins")
}

/// The walls exist, are real NURBS (not the placeholder), and carry
/// the segment structure §3 specifies: one wall per profile segment.
#[test]
fn the_loft_produces_one_real_nurbs_wall_per_profile_segment() {
    let g = geometry();
    assert_eq!(g.walls.len(), 1, "one loop");
    assert_eq!(g.walls[0].len(), 4, "one wall per segment");
    assert_eq!(g.section_params.len(), 3);
    assert_eq!(g.section_params[0], 0.0);
    assert_eq!(g.section_params[2], 1.0);
    for wall in &g.walls[0] {
        let (nu, nv) = wall.control_counts();
        assert!(nu >= 2 && nv == 3, "wall structure {nu}×{nv}");
        // Real geometry, not the poison placeholder.
        assert!(wall.eval(0.5, 0.5).x.is_finite());
    }
}

/// S9 FLIP (M6-3) of `tier_three_refuses_a_real_nurbs_wall_by_kind`:
/// the same construction — a tier-3-valid extruded box with one side
/// wall's surface replaced by a genuine skinned NURBS — now draws NO
/// `UncertifiableSurface` (check 1 refuses only the placeholder).
/// Tier 3 still refuses the BODY, for the honest geometric reason:
/// the box's rim/strut carriers do not lie on the swapped wall, so
/// re-certification and the +V lane report the mismatch. Kind-refusal
/// retired; geometry-refusal demonstrated.
#[test]
fn tier_three_certifies_the_kind_and_refuses_the_geometry() {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(2.0, 1.0),
            Point2::new(0.0, 1.0),
        ])],
    )
    .validate(Tol::witness())
    .expect("the square validates");
    let built = extrude(&profile, Extrusion::Distance(2.0), Tol::witness()).expect("extrudes");
    let mut body = built.body;
    // The baseline is a genuine tier-3 solid.
    validate_geometric(&body, Tol::witness()).expect("the extrusion validates at tier 3");

    let g = geometry();
    let wall = lift_surface::<f64>(&g.walls[0][1]).expect("lifts");
    let face = built.side_faces[0][1];
    body.set_face_surface(face, FaceSurface::New(Surface::Nurbs(wall.into())))
        .expect("the arena takes a real NURBS surface");

    let errors = validate_geometric(&body, Tol::witness()).expect_err("tier 3 must refuse the mismatched geometry");
    assert!(
        !errors
            .iter()
            .any(|e| matches!(e, topo::ValidationError::UncertifiableSurface { .. })),
        "M6-3 flip A: a described NURBS surface is no longer refused by KIND — got {errors:?}"
    );
}
