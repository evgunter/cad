//! **BLEND-3 — the concave plane–plane chamfer.**
//!
//! The convex fixture every chamfer row has run on is the cube: twelve
//! convex edges meeting at eight trivalent corners, every corner's
//! three edges requested. This suite's fixture is that body's MIRROR —
//! a rectangular cavity inside a block, whose twelve edges are all
//! concave and whose eight corners are all-concave trihedra — and the
//! rows here are what the two verbs do with it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{cavity_edges, vented_cavity};
use crate::common::oracles::chamfered_cube_removed;
use geom::Surface;
use geom_core::{Point3, Tol, Vec3};
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::cube;
use topo::{Body, EdgeKey, validate, validate_closed};

/// The chamfer setback, meters.
const D: f64 = 0.25;

/// **The fixture is what it claims**: one solid, one shell, tiers 1–2,
/// and twelve cavity edges.
#[test]
fn the_vented_cavity_is_one_shell_with_twelve_cavity_edges() {
    let body = vented_cavity();
    assert_eq!(validate(&body), Ok(()), "tier 1");
    assert_eq!(validate_closed(&body), Ok(()), "tier 2");
    assert_eq!(body.solids().count(), 1, "one solid");
    assert_eq!(body.shells().count(), 1, "the cavity vents, so one shell");
    assert_eq!(cavity_edges(&body).len(), 12, "the cavity's twelve edges");
}

/// **The volume a chamfered cavity of side `a` encloses** — the
/// fixture's own arithmetic, which is what stays here: the block,
/// less the cavity, less the part of the vent above the cavity's
/// ceiling, plus what the carve adds.
///
/// The chamfer of a concave edge ADDS material, and what it adds is
/// congruent to what the same chamfer removes from a convex block:
/// the cavity's twelve edges and eight corners are the mirror of a
/// cube's, and neither the strip nor the corner patch has a side to
/// pick. So the added term is the chamfered cube's own removed volume
/// at that side, which is [`chamfered_cube_removed`]'s derivation and
/// lives there.
fn chamfered_cavity_volume(a: f64, d: f64) -> f64 {
    let block = 4.0_f64.powi(3);
    // The vent removes material only ABOVE the cavity's ceiling; below
    // it the cavity term already has that space.
    let vent = core::f64::consts::PI * 0.5 * 0.5 * (4.0 - 3.0);
    block - a.powi(3) - vent + chamfered_cube_removed(a, d)
}

/// **THE CHAMFERED CAVITY** — all twelve concave edges at equal
/// setback: twelve strips, eight flat corner patches, tiers 1–3, the
/// census and the Euler relation, and the certified volume against the
/// closed form above.
#[test]
fn the_chamfered_cavity() {
    let body = vented_cavity();
    let out = chamfer_edges(&body, &cavity_edges(&body), D, Tol::witness())
        .expect("the cavity's twelve concave edges chamfer");
    let out_body = out.body;

    assert_eq!(validate(&out_body), Ok(()), "tier 1");
    assert_eq!(validate_closed(&out_body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&out_body, Tol::witness()),
        Ok(()),
        "tier 3"
    );

    assert_eq!(out.blend_faces.len(), 12, "one strip per concave edge");
    assert_eq!(out.corner_faces.len(), 8, "one patch per concave corner");
    assert_eq!(out.band_faces.len(), 0, "no chain of this request closes");

    let (v, e, f) = (
        out_body.vertices().count(),
        out_body.edges().count(),
        out_body.faces().count(),
    );
    assert_eq!((v, e, f), (36, 66, 34), "census");
    // Two faces carry an inner ring — the vent's mouth in the cavity's
    // ceiling, and the same vent's mouth in the block's top — and a
    // ringed face is not a disk, so each costs the alternating sum one.
    assert_eq!(
        v as i64 - e as i64 + f as i64 - 2,
        2,
        "Euler–Poincaré, ring-corrected"
    );

    let want = chamfered_cavity_volume(2.0, D);
    let props = topo::mass_properties(&out_body, Tol::witness()).expect("closed-form props");
    assert!(
        (props.volume - want).abs() <= 1e-12 * want,
        "volume {} vs closed form {want}",
        props.volume
    );

    let mesh = mesh::tessellate(&out_body, 5e-3, Tol::witness()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// A face's OUTWARD normal: the stored plane normal folded through the
/// stored sense bit — read, never sampled.
fn outward(body: &Body<f64>, face: topo::FaceKey) -> Vec3<f64> {
    let f = body.get_face(face).expect("a minted face");
    let Some(Surface::Plane { normal, .. }) = body.get_surface(f.surface) else {
        panic!("every face this carve mints is a plane");
    };
    *normal * f.sense_sign::<f64>()
}

/// A point on a face's carrier plane.
fn plane_origin(body: &Body<f64>, face: topo::FaceKey) -> Point3<f64> {
    let f = body.get_face(face).expect("a minted face");
    let Some(Surface::Plane { origin, .. }) = body.get_surface(f.surface) else {
        panic!("every face this carve mints is a plane");
    };
    *origin
}

/// **THE ORIENTATION ROW: every minted face faces the VOID.**
///
/// The carve reads one orientation convention — the interior-left
/// half-edge traversal that gives a trimline its in-plane direction,
/// and the outward sum of the supports' own normals that gives a strip
/// or patch its chart normal — and a convex fixture cannot tell a
/// convention that follows the material from one that merely points
/// away from the body's middle. Here they disagree: the cavity's void
/// is at the CENTRE, so a face minted outward-from-the-material has the
/// cavity centre on its outward side, and one minted by any
/// away-from-the-centre rule has it on the other.
///
/// So the assertion is signed against the centre, for all twenty minted
/// faces, and the same claim is checked on the chamfered CUBE with the
/// sign the other way — one row, both material sides, so neither half
/// can be satisfied by a convention that ignores the traversal.
#[test]
fn every_minted_face_of_a_concave_carve_faces_the_void() {
    let body = vented_cavity();
    let out = chamfer_edges(&body, &cavity_edges(&body), D, Tol::witness())
        .expect("the cavity's twelve concave edges chamfer");
    let centre = Point3::new(2.0, 2.0, 2.0);
    for face in out.blend_faces.iter().chain(out.corner_faces.iter()) {
        let n = outward(&out.body, *face);
        let reach = (centre - plane_origin(&out.body, *face)).dot(n);
        assert!(
            reach > 0.5 * D,
            "a concave carve's face {face:?} must face the void it fills: \
             the cavity centre is {reach} m along its outward normal"
        );
    }

    // The same claim on the convex fixture, where the material is at
    // the centre and every minted face must therefore face AWAY from
    // it. A single-sided convention passes one of these two, never
    // both.
    let cube_body = cube(2.0, Tol::witness());
    let cube_edges: Vec<EdgeKey> = cube_body.edges().map(|(k, _)| k).collect();
    let cut = chamfer_edges(&cube_body, &cube_edges, D, Tol::witness())
        .expect("a cube's twelve edges chamfer");
    let cube_centre = Point3::new(1.0, 1.0, 1.0);
    for face in cut.blend_faces.iter().chain(cut.corner_faces.iter()) {
        let n = outward(&cut.body, *face);
        let reach = (cube_centre - plane_origin(&cut.body, *face)).dot(n);
        assert!(
            reach < -0.5 * D,
            "a convex carve's face {face:?} must face away from its material: \
             the cube centre is {reach} m along its outward normal"
        );
    }
}

/// **THE DIFFERENTIAL, retired by issue 644's widening: both verbs
/// now carve this fixture.** This suite once pinned the fillet's
/// refusal here — `MixedConvexity { convex: 0 }`, the vocabulary's
/// poor-fit name for the uniform concave trihedron — as the boundary
/// the chamfer's widening did not move. The convexity-parametric
/// fillet corner then moved it deliberately: the concave rolling
/// ball's own carve, census, volume and orientation rows live in the
/// concave-fillet suite, and the corner recourse's followability —
/// now shipped only at genuinely MIXED corners — is pinned there on
/// both of its clauses. What this row keeps is the two verbs' shared
/// half: one fixture, both verbs, both carve.
#[test]
fn both_verbs_carve_the_cavity_the_fillet_once_refused() {
    let body = vented_cavity();
    let edges = cavity_edges(&body);
    chamfer_edges(&body, &edges, D, Tol::witness())
        .expect("the chamfer carves its twelve concave edges");
    fillet_edges(&body, &edges, D, Tol::witness())
        .expect("the fillet carves the same twelve, on its own arms");
}
