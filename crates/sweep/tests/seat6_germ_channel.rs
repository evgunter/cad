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
//! The fixture is deliberately the germarms one, re-derived here rather
//! than shared: that file is the germ lane's acceptance and this is a
//! statement about the channel, so a change to either must not silently
//! move the other.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_brep::RadiusEvidence;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError, ParamSource, SurfaceField};

/// A cylinder about `z`, radius `r`, `z ∈ [−h, h]`, through the public
/// extrude door.
fn cyl(r: f64, h: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -h)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(2.0 * h), tol)
        .unwrap()
        .body
}

fn spin(b: &Body<f64>, axis: Vec3<f64>, angle: f64) -> Body<f64> {
    topo::transform_rigid(
        b,
        &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
        Tol::witness(),
    )
    .unwrap()
}

/// The Steinmetz surfaces with both seams turned off the pinch — the
/// pose that reaches the join.
fn pair(h: f64, phi: f64) -> (Body<f64>, Body<f64>) {
    let a = cyl(1.0, h);
    let b = spin(&cyl(1.0, h), Vec3::new(1.0, 0.0, 0.0), PI / 2.0);
    (
        spin(&a, Vec3::new(0.0, 0.0, 1.0), phi),
        spin(&b, Vec3::new(0.0, 1.0, 0.0), phi),
    )
}

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
    let (mut a, mut b) = pair(1.2, PI / 4.0);
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
    let (a, b) = pair(1.2, PI / 4.0);
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
    let (mut a, mut b) = pair(1.2, PI / 4.0);
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
    let (mut a, b) = pair(1.2, PI / 4.0);
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
    let (mut a, mut b) = pair(1.2, PI / 4.0);
    declare(&mut a, &token);
    declare(&mut b, &token);
    let repose = |body: &Body<f64>| {
        let r = Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 2.0, 3.0).normalize(),
            0.7,
        );
        topo::transform_rigid(
            body,
            &Affine3::from_parts(r.linear, r.translation + Vec3::new(0.3, -0.45, 0.6)),
            Tol::witness(),
        )
        .unwrap()
    };
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
