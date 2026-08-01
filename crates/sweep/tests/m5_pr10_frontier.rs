//! M5 PR 10 §3/§5 — the NURBS-wall B-rep frontier, DEMONSTRATED.
//!
//! §3 asks the lofted body to validate at tier 3. It cannot, at this
//! PR's merge time, and the reason is not a gap in this PR's geometry:
//! the certification layer refuses every non-analytic surface outright.
//! These rows pin the refusal so the claim is a demonstration rather
//! than an assertion, and so the PR that opens the frontier (M5 plan
//! line 9's curved booleans / line 11's curved tessellation, which own
//! `implicit`'s NURBS forms) sees exactly which doors it must open.
//!
//! Never a silent skip: the walls this PR produces are real, and the
//! rows below evaluate them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::SketchSegment;
use geom_core::{Affine3, Point2, Tolerance, Vec3};
use geom_surfaces::Surface;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::skin::{SectionSegments, lift_surface, loft_geometry};
use sweep::{Extrusion, extrude};
use topo::{FaceSurface, validate_geometric};

/// A three-section loft of a square, middle section scaled — the
/// acceptance shape's wall set (§5's "at least one non-affine pair").
fn geometry() -> sweep::LoftGeometry {
    let chain = |s: f64| -> SectionSegments {
        let p = |x: f64, y: f64| Point2::new(x * s, y * s);
        vec![vec![
            SketchSegment::Line {
                a: p(0.0, 0.0),
                b: p(2.0, 0.0),
            },
            SketchSegment::Arc {
                a: p(2.0, 0.0),
                b: p(2.0, 1.0),
                bulge: 0.25,
            },
            SketchSegment::Line {
                a: p(2.0, 1.0),
                b: p(0.0, 1.0),
            },
            SketchSegment::Line {
                a: p(0.0, 1.0),
                b: p(0.0, 0.0),
            },
        ]]
    };
    let places = [0.0, 1.0, 2.0].map(|z| Affine3::translation(Vec3::new(0.0, 0.0, z)));
    loft_geometry(&[chain(1.0), chain(1.6), chain(1.0)], &places, 2).expect("the loft skins")
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

/// A REAL, tier-3-valid solid, with one side wall's surface replaced
/// by a genuine (non-placeholder) skinned NURBS: tier 3 then refuses
/// that face outright. Nothing about the wall is wrong — the gate
/// rejects the KIND.
#[test]
fn tier_three_refuses_a_real_nurbs_wall_by_kind() {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(2.0, 1.0),
            Point2::new(0.0, 1.0),
        ])],
    )
    .validate(Tolerance::get())
    .expect("the square validates");
    let built = extrude(&profile, Extrusion::Distance(2.0)).expect("extrudes");
    let mut body = built.body;
    // The baseline is a genuine tier-3 solid.
    validate_geometric(&body).expect("the extrusion validates at tier 3");

    let g = geometry();
    let wall = lift_surface::<f64>(&g.walls[0][1]).expect("lifts");
    let face = built.side_faces[0][1];
    body.set_face_surface(face, FaceSurface::New(Surface::Nurbs(wall.into())))
        .expect("the arena takes a real NURBS surface");

    let errors = validate_geometric(&body).expect_err("tier 3 must refuse the NURBS face");
    assert!(
        errors.iter().any(|e| matches!(
            e,
            topo::ValidationError::UncertifiableSurface { face: f } if *f == face
        )),
        "expected UncertifiableSurface on the NURBS face, got {errors:?}"
    );
}
