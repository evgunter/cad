//! **BLEND-3 — the concave plane–plane chamfer.**
//!
//! The convex fixture every chamfer row has run on is the cube: twelve
//! convex edges meeting at eight trivalent corners, every corner's
//! three edges requested. This suite's fixture is that body's MIRROR —
//! a rectangular cavity inside a block, whose twelve edges are all
//! concave and whose eight corners are all-concave trihedra — and the
//! rows here are what the two verbs do with it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, CornerConfig, RunOutPolicy};
use sweep::chamfer::chamfer_edges;
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

/// **The fixture: a block with a rectangular cavity, vented.**
///
/// The block is `[0,4]³`; the cavity is `[1,3]³`; and a square chimney
/// `[1.5,2.5]² × [2.5,5]` is cut first, so the cavity is a VENT rather
/// than a void — one shell, which is what the surgery's body door
/// admits, with the vent's mouth strictly inside the cavity's ceiling.
///
/// The cavity's twelve edges are all concave and its eight corners are
/// all-concave trihedra: the mirror of the chamfered cube. Nothing
/// else in the body touches them — the vent pierces the ceiling face's
/// interior — so the twelve are a whole component of the edge graph,
/// which is what makes the request a complete one.
fn vented_cavity() -> Body<f64> {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = brick(Point3::new(1.5, 1.5, 2.5), Point3::new(2.5, 2.5, 5.0));
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

/// **The refusal both verbs give this request, measured**: the corner
/// configuration door reads three concave edges at a trivalent vertex
/// and calls it a MIXED-convexity corner — `convex: 0` of 3 — whose
/// recourse is the run-out door that does not exist.
///
/// This is the row the widening flips. It is written for both verbs
/// against one shared expectation, so the day one of them changes, the
/// row says which.
#[test]
fn a_concave_corner_reads_as_mixed_convexity_at_both_doors() {
    let body = vented_cavity();
    let edges = cavity_edges(&body);
    let chamfer = chamfer_edges(&body, &edges, D, band(), Tol::witness())
        .expect_err("the concave chamfer refuses");
    let fillet = fillet_edges(&body, &edges, D, band(), Tol::witness())
        .expect_err("the concave fillet refuses");
    for (verb, err) in [("chamfer", &chamfer), ("fillet", &fillet)] {
        match err.error {
            BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { convex },
                policy,
                ..
            } => {
                assert_eq!(convex, 0, "{verb}: none of the three edges is convex");
                assert_eq!(policy, Some(RunOutPolicy::RunOutFeather));
            }
            ref other => panic!("{verb}: expected a corner-configuration refusal, got {other:?}"),
        }
    }
}
