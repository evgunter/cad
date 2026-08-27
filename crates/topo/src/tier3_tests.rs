//! In-crate tier-3 tests (M2 PR 3): the corruption directions only the
//! raw arenas can reach — a certified body whose stored geometry is
//! then made wrong must be caught by [`crate::validate_geometric`]'s
//! re-checks at rest (the other half of the attachment/at-rest pair;
//! the attachment-side rejections live in `tests/geometric_cube.rs`).
//!
//! The corruptions here replace whole arena entries (an [`EdgeCurve`]
//! is only constructible certified, so "a wrong cache" means "a
//! certified cache for *different* data sitting at this edge's slot" —
//! exactly what re-certification against the edge's own endpoints
//! exists to catch).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Vec3};

use crate::euler::FaceSurface;
use crate::fixtures::test_curve;
use crate::validate::{ValidationError, validate, validate_geometric};
use crate::{Body, MefSite, MevSite};
use geom_core::Tol;

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// A geometric digon pillow: two vertices, two chord edges, two
/// coplanar faces (the z = 0 plane on both sides) — the minimal
/// tier-3-clean body, and the coplanar-split smooth-dihedral case.
fn coplanar_pillow(tol: Tol) -> (Body<f64>, crate::MefCreated) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let plane = Surface::Plane {
        origin: pt(0.0, 0.0, 0.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let split = body
        .mef(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)),
            FaceSurface::New(plane.clone()),
            tol,
        )
        .unwrap();
    // The seed face shares the same geometric plane under its own key
    // (identical-by-construction would share the KEY in a sweep; here
    // the point is the smooth-dihedral classification, which compares
    // the surfaces' tangent planes, not their keys).
    body.set_face_surface(seed.face, FaceSurface::New(plane))
        .unwrap();
    // Both chords were built through the SCAFFOLDING door, because
    // neither face's surface existed when its `mev`/`mef` ran. The
    // body is at rest now and both faces have charts, so both edges
    // are re-described where they rest (D3's transience fence — tier
    // 3 refuses a scaffold on a body with faces).
    let chart = body.get_face(split.face).unwrap().surface;
    for e in [seg.edge, split.edge] {
        let spec = EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0))
            .at_rest_in_chart(chart, false);
        body.set_edge_curve(e, spec, tol).unwrap();
    }
    (body, split)
}

#[test]
fn coplanar_split_is_smooth_and_tier3_clean() {
    let tol = Tol::witness();
    let (body, _) = coplanar_pillow(tol);
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// **The transience fence** (U2's Q2 as corrected, 2026-08-27), red
/// then green on ONE edge of one body.
///
/// RED: the scaffolding door describes a locus for an edge whose
/// surfaces do not exist yet. Put that description back on an edge of
/// a body AT REST — two faces, two charts — and tier 3 names it.
///
/// GREEN: the same edge, same carrier, same interval, described where
/// it rests (an image in the chart it lies in) validates clean. Only
/// the description moves, which is the whole content of the fence.
#[test]
fn a_scaffold_at_rest_is_refused_and_the_chart_description_is_not() {
    let tol = Tol::witness();
    let (mut body, split) = coplanar_pillow(tol);
    assert_eq!(validate_geometric(&body, tol), Ok(()));

    // RED — back through the scaffolding door.
    let scaffolded = EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0));
    body.set_edge_curve(split.edge, scaffolded.clone(), tol)
        .expect("the door itself is legal: certification is not where the fence lives");
    let errors = validate_geometric(&body, tol).expect_err("a scaffold at rest is refused");
    assert!(
        errors.contains(&ValidationError::ScaffoldAtRest { edge: split.edge }),
        "tier 3 must name the scaffolded edge, got {errors:?}",
    );

    // GREEN — the same edge described where it rests.
    let chart = body.get_face(split.face).unwrap().surface;
    body.set_edge_curve(split.edge, scaffolded.at_rest_in_chart(chart, false), tol)
        .unwrap();
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// The other half of the fence: the door it exists to keep open. An
/// edge whose surfaces genuinely do not exist yet — a `mev` chord in a
/// half-built ring — carries a scaffolding description and is NOT
/// refused, because the fence is TRANSIENCE and this edge is transient.
#[test]
fn the_scaffolding_door_still_passes_mid_construction() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    body.mev_line(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        pt(1.0, 0.0, 0.0),
        tol,
    )
    .unwrap();
    // No surface anywhere yet, so the chord could not name a chart
    // even in principle — and tier 3 says nothing about it.
    assert_eq!(validate(&body), Ok(()));
}

#[test]
fn wrong_cache_at_rest_is_rejected_by_tier3() {
    let tol = Tol::witness();
    // Certified body; then swap one edge's stored curve for a certified
    // curve of DIFFERENT data (the scaffolding circle at a far point).
    // Attachment can't see it (raw arenas); tier 3's re-certification
    // must.
    let (mut body, split) = coplanar_pillow(tol);
    let curve_key = body.get_edge(split.edge).unwrap().curve;
    *body.curves.get_mut(curve_key).unwrap() =
        crate::null::CurveGeom::Certified(test_curve(pt(50.0, 0.0, 0.0), tol));
    assert_eq!(validate(&body), Ok(()), "structurally still coherent");
    let errs = validate_geometric(&body, tol).unwrap_err();
    assert!(
        errs.iter().any(|e| matches!(
            e,
            ValidationError::EdgeCertification { edge, .. } if *edge == split.edge
        )),
        "{errs:?}"
    );
}

#[test]
fn offset_plane_at_rest_fails_planar_residuals() {
    let tol = Tol::witness();
    // Move a face's stored plane 100·ε off its vertices: the
    // Newell-cache re-check (tier 3, check 3) reports every vertex of
    // that face.
    let (mut body, split) = coplanar_pillow(tol);
    let eps = tol.eps();
    let surface_key = body.get_face(split.face).unwrap().surface;
    *body.surfaces.get_mut(surface_key).unwrap() = Surface::Plane {
        origin: pt(0.0, 0.0, 100.0 * eps),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let errs = validate_geometric(&body, tol).unwrap_err();
    let off_plane = errs
        .iter()
        .filter(|e| matches!(e, ValidationError::PlanarFaceResidual { face, .. } if *face == split.face))
        .count();
    assert_eq!(off_plane, 2, "both pillow vertices reported: {errs:?}");
}

#[test]
fn sliver_dihedral_at_rest_is_rejected() {
    let tol = Tol::witness();
    // Tilt one face's plane by the run-scaled sliver angle 3ε: both
    // MappedCurve attachments were legal (conventional descriptions
    // carry no dihedral requirement), but at rest the wedge is
    // indeterminate — the material wedge-angle predicate escalates.
    let (mut body, split) = coplanar_pillow(tol);
    let eps = tol.eps();
    let theta = 3.0 * eps;
    let surface_key = body.get_face(split.face).unwrap().surface;
    *body.surfaces.get_mut(surface_key).unwrap() = Surface::Plane {
        origin: pt(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, theta.sin(), theta.cos()),
        u_ref: Vec3::unit_x(),
    };
    let errs = validate_geometric(&body, tol).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::SliverDihedral { .. })),
        "{errs:?}"
    );
}

#[test]
fn dangling_description_is_a_tier1_error() {
    let tol = Tol::witness();
    // An Intersection description whose surface key is ripped out of
    // the arena: the geometry-to-geometry reference check (tier 1,
    // pass 1) fires — alongside the face's own dangling reference.
    let (mut body, split) = coplanar_pillow(tol);
    let s_plus = body.get_face(split.face).unwrap().surface;
    let seed_face = body
        .get_loop(body.get_half_edge(split.he_plus).unwrap().parent_loop)
        .unwrap()
        .face;
    let s_seed = body.get_face(seed_face).unwrap().surface;
    let mut spec = EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0));
    spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
        s1: s_seed,
        s2: s_plus,
        witness: pt(0.5, 0.0, 0.0),
    };
    // NB: coincident planes would fail transversality — so tilt the
    // split face definitively first (a legal corner), then upgrade.
    *body.surfaces.get_mut(s_plus).unwrap() = Surface::Plane {
        origin: pt(0.0, 0.0, 0.0),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    };
    body.set_edge_curve(split.edge, spec, tol).unwrap();
    // Rip the referenced surface out (raw removal past the hygiene
    // guard).
    body.surfaces.remove(s_plus);
    let errs = validate(&body).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::DanglingDescription { .. })),
        "{errs:?}"
    );
}

#[test]
fn description_references_keep_a_surface_alive() {
    let tol = Tol::witness();
    // A surface referenced ONLY by an edge description is not orphaned:
    // the hygiene guard refuses to remove it and tier 1 does not report
    // OrphanGeometry.
    let (mut body, split) = coplanar_pillow(tol);
    let s_plus = body.get_face(split.face).unwrap().surface;
    let seed_face = body
        .get_loop(body.get_half_edge(split.he_plus).unwrap().parent_loop)
        .unwrap()
        .face;
    let s_seed = body.get_face(seed_face).unwrap().surface;
    // Make the corner genuine, then describe the edge intrinsically.
    *body.surfaces.get_mut(s_plus).unwrap() = Surface::Plane {
        origin: pt(0.0, 0.0, 0.0),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    };
    let mut spec = EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0));
    spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
        s1: s_seed,
        s2: s_plus,
        witness: pt(0.5, 0.0, 0.0),
    };
    body.set_edge_curve(split.edge, spec, tol).unwrap();
    // Repoint the split face to a NEW surface: the old one is now
    // referenced only by the description — and must survive.
    body.set_face_surface(
        split.face,
        FaceSurface::New(Surface::Plane {
            origin: pt(0.0, 0.0, 0.0),
            normal: Vec3::unit_y(),
            u_ref: Vec3::unit_x(),
        }),
    )
    .unwrap();
    assert!(body.get_surface(s_plus).is_some(), "kept alive");
    assert_eq!(validate(&body), Ok(()), "no OrphanGeometry");
    // (Tier 3 now reports the adjacency incoherence — the description
    // names a surface that is no longer the face's — which is exactly
    // the loud trail this state should leave.)
    let errs = validate_geometric(&body, tol).unwrap_err();
    assert!(
        errs.iter().any(|e| matches!(
            e,
            ValidationError::DescriptionNotAdjacent { edge } if *edge == split.edge
        )),
        "{errs:?}"
    );
}
