//! **BLEND-3 — the concave plane–plane chamfer.**
//!
//! The convex fixture every chamfer row has run on is the cube: twelve
//! convex edges meeting at eight trivalent corners, every corner's
//! three edges requested. This suite's fixture is that body's MIRROR —
//! a rectangular cavity inside a block, whose twelve edges are all
//! concave and whose eight corners are all-concave trihedra — and the
//! rows here are what the two verbs do with it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Band, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, CornerConfig, FILLET3_CORNER_RECOURSE, RunOutPolicy};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::cube;
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, subtract, validate, validate_closed};

/// The chamfer setback, meters.
const D: f64 = 0.25;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

/// An axis-aligned box, authored the way a user would: a rectangle
/// profile on a translated sketch plane, extruded.
fn brick(lo: Point3<f64>, hi: Point3<f64>) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(lo.x, lo.y),
        Point2::new(hi.x, lo.y),
        Point2::new(hi.x, hi.y),
        Point2::new(lo.x, hi.y),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, lo.z) - Point3::origin(),
    ));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("a rectangle is a valid profile");
    extrude(&profile, Extrusion::Distance(hi.z - lo.z), Tol::witness())
        .expect("a brick extrudes")
        .body
}

/// A circular rod: two half-arc profile segments extruded, so its wall
/// is a cylinder and the ring it cuts in a plane is a circle.
fn rod(center: Point2<f64>, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(center.x - r, center.y), 1.0),
        ProfileVertex::new(Point2::new(center.x + r, center.y), 1.0),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z0) - Point3::origin(),
    ));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("a circle is a valid profile");
    extrude(&profile, Extrusion::Distance(z1 - z0), Tol::witness())
        .expect("a rod extrudes")
        .body
}

/// **The fixture: a block with a rectangular cavity, vented.**
///
/// The block is `[0,4]³`; the cavity is `[1,3]³`; and a round chimney
/// of radius `0.5` on the axis `x = y = 2`, from `z = 2.5` clear of the
/// top, is cut first — so the cavity is a VENT rather than a void, i.e.
/// one shell, which is what the surgery's body door admits, with the
/// vent's mouth strictly inside the cavity's ceiling. The vent is round
/// because the ring the mouth leaves in that ceiling rides through the
/// carve, and the exact ring-clearance check covers circle rings.
///
/// The cavity's twelve edges are all concave and its eight corners are
/// all-concave trihedra: the mirror of the chamfered cube. Nothing
/// else in the body touches them — the vent pierces the ceiling face's
/// interior — so the twelve are a whole component of the edge graph,
/// which is what makes the request a complete one.
fn vented_cavity() -> Body<f64> {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.0, 2.0), 0.5, 2.5, 5.0);
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let vented = subtract(&block, &vent, Tol::witness())
        .expect("the vent cut succeeds")
        .body()
        .expect("the vent cut leaves material")
        .body
        .clone();
    subtract(&vented, &cavity, Tol::witness())
        .expect("the cavity cut succeeds")
        .body()
        .expect("the cavity cut leaves material")
        .body
        .clone()
}

/// The cavity's twelve edges, found by their endpoints — both of them
/// corners of the cavity box `[1,3]³` — never by index.
fn cavity_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let corner = |p: Point3<f64>| {
        [p.x, p.y, p.z]
            .iter()
            .all(|c| (c - 1.0).abs() < 1e-12 || (c - 3.0).abs() < 1e-12)
    };
    let mut found: Vec<EdgeKey> = body
        .edges()
        .filter(|(k, _)| {
            let Some(e) = body.get_edge(*k) else {
                return false;
            };
            let Some(h) = body.get_half_edge(e.he_plus) else {
                return false;
            };
            let Some(end) = body.half_edge_end(e.he_plus) else {
                return false;
            };
            let pt = |v| {
                body.get_vertex(v)
                    .and_then(|x| body.get_point(x.point))
                    .copied()
            };
            match (pt(h.start), pt(end)) {
                (Some(a), Some(b)) => corner(a) && corner(b),
                _ => false,
            }
        })
        .map(|(k, _)| k)
        .collect();
    found.sort_unstable();
    found
}

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

/// **The volume a chamfered cavity of side `a` encloses.**
///
/// The chamfer of a concave edge ADDS material, and what it adds is
/// congruent to what the same chamfer removes from a convex block:
/// the cavity's twelve edges and eight corners are the mirror of a
/// cube's, and neither the strip nor the corner patch has a side to
/// pick. So the added volume is the chamfered cube's own removed
/// volume at that side — twelve triangular prisms `d²/2` along the
/// edges and eight corner tetrahedra, less the `2d³` each corner's
/// four sets over-count — and the block keeps its vent.
fn chamfered_cavity_volume(a: f64, d: f64) -> f64 {
    let block = 4.0_f64.powi(3);
    // The vent removes material only ABOVE the cavity's ceiling; below
    // it the cavity term already has that space.
    let vent = core::f64::consts::PI * 0.5 * 0.5 * (4.0 - 3.0);
    let added = 6.0 * a * d * d - (16.0 / 3.0) * d.powi(3);
    block - a.powi(3) - vent + added
}

/// **THE CHAMFERED CAVITY** — all twelve concave edges at equal
/// setback: twelve strips, eight flat corner patches, tiers 1–3, the
/// census and the Euler relation, and the certified volume against the
/// closed form above.
#[test]
fn the_chamfered_cavity() {
    let body = vented_cavity();
    let out = chamfer_edges(&body, &cavity_edges(&body), D, band(), Tol::witness())
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
    let out = chamfer_edges(&body, &cavity_edges(&body), D, band(), Tol::witness())
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
    let cut = chamfer_edges(&cube_body, &cube_edges, D, band(), Tol::witness())
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

/// **THE DIFFERENTIAL: the fillet's refusal here did not move.**
///
/// The chamfer's widening is the chamfer's. A fillet asked for the
/// same twelve edges refuses at the same door with the same payload it
/// carried before any of this — `MixedConvexity { convex: 0 }`,
/// feather — because the corner ball, its contact feet and the octant
/// chart are each derived on the convex side and none of them moved
/// (evgunter/cad issue 644 is that widening).
///
/// The payload is asserted rather than merely the variant, since the
/// variant alone would stay green under a relabelling. Naming the
/// uniform concave trihedron for what it is would read better here and
/// is deliberately NOT done: extending the corner vocabulary is a
/// taxonomy decision OQ6 reserves for Evan, opened as evgunter/cad
/// issue 1355.
#[test]
fn the_concave_fillet_refuses_exactly_as_it_did_before() {
    let body = vented_cavity();
    let edges = cavity_edges(&body);
    let err = fillet_edges(&body, &edges, D, band(), Tol::witness())
        .expect_err("the concave fillet refuses");
    match err.error {
        BlendError::UnsupportedCorner {
            corner: CornerConfig::MixedConvexity { convex },
            policy,
            ..
        } => {
            assert_eq!(convex, 0, "none of the three edges is convex");
            assert_eq!(policy, Some(RunOutPolicy::RunOutFeather));
        }
        ref other => panic!("expected the corner-configuration refusal, got {other:?}"),
    }
    assert!(
        err.to_string()
            .contains("is a mixed-convexity vertex (0 of 3 edges convex)"),
        "the rendered sentence is the one it always was: {err}"
    );
}

/// **The sentence that refusal ships is followable, here, as written.**
///
/// The corner recourse tells a refused caller that a trivalent corner
/// of three CONCAVE edges is carved by a chamfer over plane–plane
/// supports. A recourse that names a door which cannot serve the caller
/// who was just refused is the defect this standard exists for, so the
/// row asserts the sentence and then EXECUTES it — same body, same
/// twelve edges, same size — and requires the promised carve.
#[test]
fn the_concave_corner_recourse_is_followable_as_written() {
    let body = vented_cavity();
    let edges = cavity_edges(&body);
    let refused = fillet_edges(&body, &edges, D, band(), Tol::witness())
        .expect_err("the concave fillet refuses");
    let text = refused.error.to_string();
    assert!(
        text.contains(FILLET3_CORNER_RECOURSE),
        "the corner refusal must ship the corner recourse, got {text}"
    );
    chamfer_edges(&body, &edges, D, band(), Tol::witness())
        .expect("the door that sentence names must carve the request it was refused for");
}
