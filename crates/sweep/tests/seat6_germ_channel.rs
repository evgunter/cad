//! **The parameter-identity channel read at the boolean germ**, on the
//! one production configuration that reaches it.
//!
//! `verbs_germarms2.rs` established the destination: two equal-radius
//! cylinder walls with intersecting axes and their seams turned off the
//! pinch reach the join, which refuses at the door that names the pinch
//! because a self-crossing ellipse pair is not one conic. That door used
//! to say, in its own message, that WHICH shape the locus has "is a
//! radius-equality question, and radius equality is structural or
//! declared and never inferred from values" — with nothing able to
//! declare it.
//!
//! This file is that sentence closed. The same fixture, with the two
//! walls' radius fields carrying one lowered parameter source, reaches
//! the same door carrying `Declared`, and the equal-radius closed form
//! (`geom_brep::cylinder_cylinder_section`) is constructed and verified
//! on the way. Without the records — or with two different ones — the
//! same geometry answers `None` and routes the general rung, whatever
//! its radii read.
//!
//! The fixture IS the germarms one, shared through
//! `common::germ_pair`: both files read one pair at one door, and a
//! pair that drifted between them would make the two statements about
//! different geometry.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::germ_pair::{cyl, repose, seams_off_the_pinch};
use geom_brep::RadiusEvidence;
use geom_core::{Affine3, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError, ParamSource, SurfaceField};

/// Stamps `token` on the operand's ONE cylinder wall — standing in for
/// the recipe layer's attach-at-mint, which is exercised end to end
/// from a document in `editor-core`'s `seat6_param_source.rs`.
fn declare(body: &mut Body<f64>, token: &ParamSource) {
    let walls: Vec<_> = body
        .surfaces()
        .filter(|(_, s)| matches!(s, topo::Surface::Cylinder { .. }))
        .map(|(k, _)| k)
        .collect();
    assert_eq!(walls.len(), 1, "the fixture has one wall surface");
    body.set_surface_field_source(walls[0], SurfaceField::CylinderRadius, token.clone())
        .expect("a live wall surface");
}

/// The evidence the germ read, off the door it refused at.
fn germ_evidence(a: &Body<f64>, b: &Body<f64>) -> RadiusEvidence {
    match topo::union(a, b, Tol::witness()).expect_err("this family has no join arm") {
        BooleanError::GermFrameCylinderPinch { evidence, .. } => evidence,
        other => panic!("expected the germ frame's pinch door, got {other:?}"),
    }
}

/// **One declared radius on both walls reaches the germ as
/// `Declared`.** This is the channel end to end at the kernel seat: the
/// records ride the operands through the reduction, the germ site reads
/// them off the two bodies' germ faces, and the closed form is reached
/// and verified rather than guessed at.
#[test]
fn one_shared_radius_source_declares_at_the_germ() {
    let token = ParamSource::from_lowered(b"the-document's-r");
    let (mut a, mut b) = seams_off_the_pinch(1.2, PI / 4.0);
    declare(&mut a, &token);
    declare(&mut b, &token);
    assert_eq!(
        germ_evidence(&a, &b),
        RadiusEvidence::Declared,
        "two walls carrying one lowered source are declared-equal at the germ"
    );
}

/// **The same geometry, no channel: `None`, permanently.** The radii
/// are bit-identical to the row above — the only thing removed is the
/// declaration — and the germ routes the general rung. This is the
/// posture of imported and hand-built geometry, and it is the shipped
/// behaviour of every body in the tree, since nothing but a recipe
/// evaluation attaches these records.
#[test]
fn the_undeclared_pair_routes_the_general_rung() {
    let (a, b) = seams_off_the_pinch(1.2, PI / 4.0);
    assert_eq!(
        germ_evidence(&a, &b),
        RadiusEvidence::None,
        "equal values without a declaration never become a declaration"
    );
}

/// **Two DIFFERENT sources are not a declaration either**, which is the
/// row that separates "the germ reads the channel" from "the germ
/// notices records exist".
#[test]
fn two_different_sources_are_not_a_declaration() {
    let (mut a, mut b) = seams_off_the_pinch(1.2, PI / 4.0);
    declare(&mut a, &ParamSource::from_lowered(b"r"));
    declare(&mut b, &ParamSource::from_lowered(b"s"));
    assert_eq!(
        germ_evidence(&a, &b),
        RadiusEvidence::None,
        "two walls built from two expressions are not declared equal"
    );
}

/// **One side declared is not a declaration**: a document wall meeting
/// an imported one falls back exactly as an imported pair does.
#[test]
fn one_declared_side_is_not_a_declaration() {
    let (mut a, b) = seams_off_the_pinch(1.2, PI / 4.0);
    declare(&mut a, &ParamSource::from_lowered(b"r"));
    assert_eq!(germ_evidence(&a, &b), RadiusEvidence::None);
}

/// **The declaration survives a rigid re-pose**, which is the whole
/// motion-invariance argument at the seat that consumes it: the records
/// ride `transform_rigid` verbatim (a map cannot change a radius) even
/// though the same op CLEARS the description-level `GeomSource`
/// records, so a placed instance of a declaring document still
/// declares.
#[test]
fn a_rigid_re_pose_keeps_the_declaration() {
    let token = ParamSource::from_lowered(b"the-document's-r");
    let (mut a, mut b) = seams_off_the_pinch(1.2, PI / 4.0);
    declare(&mut a, &token);
    declare(&mut b, &token);
    assert_eq!(
        germ_evidence(&repose(&a), &repose(&b)),
        RadiusEvidence::Declared,
        "a rigid re-pose must not cost the pair its declaration"
    );
}

/// **The records ride the boolean's cross-body graft into the RESULT.**
///
/// The germ rows above read the channel off the operand clones, which
/// is one step short of the claim the design makes: survivors keep
/// their records, so a boolean RESULT must still declare what its
/// operands declared, or a boolean-of-boolean would silently lose the
/// identity one layer down. The union of two disjoint declaring
/// cylinders is the cheapest configuration that runs the transplant and
/// comes back with a body to read.
#[test]
fn the_records_survive_into_a_boolean_result() {
    let token = ParamSource::from_lowered(b"the-document's-r");
    let mut a = cyl(1.0, 1.0);
    let mut b = topo::transform_rigid(
        &cyl(1.0, 1.0),
        &Affine3::translation(Vec3::new(8.0, 0.0, 0.0)),
        Tol::witness(),
    )
    .unwrap();
    declare(&mut a, &token);
    declare(&mut b, &token);
    let out = topo::union(&a, &b, Tol::witness()).expect("two disjoint solids unite");
    let topo::BooleanResult::Body(bb) = out else {
        panic!("a disjoint union is not empty");
    };
    let walls: Vec<_> = bb
        .body
        .surfaces()
        .filter(|(_, s)| matches!(s, topo::Surface::Cylinder { .. }))
        .map(|(k, _)| k)
        .collect();
    assert_eq!(walls.len(), 2, "the result carries both walls");
    for wall in walls {
        assert_eq!(
            bb.body
                .surface_field_source(wall, SurfaceField::CylinderRadius),
            Some(&token),
            "a survivor kept its geometry and must keep its parameter identity with it"
        );
    }
}

/// **The split's batch orphan sweep reaches BOTH side tables.**
///
/// A split keeps one side's faces and drops the other's; a surface
/// that only the dropped faces referenced is swept out of the arena in
/// one batch (`splitting/finish.rs`'s `carve`), and the side tables
/// keyed on it have to go with it. Generational keys make a stranded
/// row unreachable through any NEW key, so this is not a wrong read
/// waiting to happen; it is the OLD key still answering for a surface
/// the body no longer holds — a stale key reading a dead record — and
/// the parallel tables silently diverging from the arena they mirror.
/// The row holds the pre-split key and asks the surviving half.
///
/// The fixture is the corpus's square-with-an-arc chain, extruded: its
/// one arc wall is a cylinder at the far side of the profile, and a
/// split plane parallel to the extrusion through the square leaves
/// that wall a surface only the dropped faces referenced.
///
/// Two channels, two assertions: the field rows this unit added, and
/// the description-level `GeomSource` row the same sweep now removes
/// too — a change to the pre-existing channel folded in beside it, so
/// it gets its own line rather than riding under the field rows'.
#[test]
fn the_split_orphan_sweep_drops_both_side_tables() {
    let tol = Tol::witness();
    let vp = Profile::new(SketchPlane::xy(), crate::common::chain(1.0))
        .validate(tol)
        .unwrap();
    let mut body = extrude(&vp, Extrusion::Distance(1.0), tol)
        .expect("the chain extrudes")
        .body;
    let token = ParamSource::from_lowered(b"the-document's-r");
    // The arc wall: the one cylinder surface the chain's extrusion
    // mints, which sits at x ≈ 2, wholly beyond the split plane.
    let far_wall = {
        let walls: Vec<_> = body
            .surfaces()
            .filter(|(_, s)| matches!(s, topo::Surface::Cylinder { .. }))
            .map(|(k, _)| k)
            .collect();
        assert_eq!(walls.len(), 1, "the chain has one arc, so one cylinder wall");
        walls[0]
    };
    body.set_surface_field_source(far_wall, SurfaceField::CylinderRadius, token)
        .expect("a live cylinder key");
    body.set_surface_source(far_wall, topo::GeomSource::minted(7, 0))
        .expect("a live wall key");
    let plane = topo::SplitPlane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    };
    let halves = topo::split(&body, &plane, tol).expect("the square splits");
    let near = halves.below.body().expect("the flat side is below");
    assert!(
        near.get_surface(far_wall).is_none(),
        "the arc wall is not a surface of the near half"
    );
    assert!(
        near.surface_field_source(far_wall, SurfaceField::CylinderRadius)
            .is_none(),
        "the field rows outlived the surface the split swept out"
    );
    assert!(
        near.surface_source(far_wall).is_none(),
        "the GeomSource row outlived the surface the split swept out"
    );
}
