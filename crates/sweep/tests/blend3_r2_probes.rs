//! **BLEND-3 R2 review probes** — adversarial pins against PR #1347's
//! claims, self-contained (the fixture is re-authored here through the
//! public API rather than imported from the unit's own suite).
//!
//! - P1: the THIRD door of the PR's narrative, executed — a SQUARE
//!   vent's ring refuses at the exact ring-clearance pass once the two
//!   convexity doors are widened, which is the stated reason the
//!   shipped fixture's vent is round.
//! - P2: the fixture's own claim, checked geometrically rather than by
//!   the kernel's classifier — all twelve cavity edges are concave
//!   (their supports' outward-normal sum points INTO the cavity void)
//!   and every one of the eight cavity corners is trivalent with all
//!   three incident edges in the cavity set.
//! - P3: the impossibility argument behind the fixture's size, attacked
//!   with the smaller shape a reviewer would reach for first — an open
//!   POCKET. Its floor corners ARE all-concave trihedra, so the corner
//!   door is not what stops it: the floor alone walks into a closed
//!   sharp chain (G1 refusal), and any completion walks up the struts
//!   into the rim's MIXED corners. Both refusals are pinned.
//! - P4: the differential half of "the fillet's admission does not
//!   move": the same L-bracket mixed corner refuses the fillet with the
//!   same MixedConvexity tag as before the widening.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, CornerConfig};
use sweep::chamfer::chamfer_edges;
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, subtract, validate_closed};

const D: f64 = 0.25;

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

fn cut(base: &Body<f64>, tool: &Body<f64>) -> Body<f64> {
    subtract(base, tool, Tol::witness())
        .expect("the cut succeeds")
        .body()
        .expect("the cut leaves material")
        .body
        .clone()
}

/// Edges whose both endpoints have every coordinate on the given pair
/// of grid values — the cavity/pocket corner finder, by endpoints.
fn edges_with_corners(body: &Body<f64>, on: impl Fn(Point3<f64>) -> bool) -> Vec<EdgeKey> {
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
                (Some(a), Some(b)) => on(a) && on(b),
                _ => false,
            }
        })
        .map(|(k, _)| k)
        .collect();
    found.sort_unstable();
    found
}

fn cavity_corner(p: Point3<f64>) -> bool {
    [p.x, p.y, p.z]
        .iter()
        .all(|c| (c - 1.0).abs() < 1e-12 || (c - 3.0).abs() < 1e-12)
}

/// The unit's fixture with the FIRST DRAFT's vent: a square chimney
/// instead of the round one.
fn square_vented_cavity() -> Body<f64> {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = brick(Point3::new(1.5, 1.5, 2.5), Point3::new(2.5, 2.5, 5.0));
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    cut(&cut(&block, &vent), &cavity)
}

/// **P1 — the third door, executed.** With both convexity doors
/// widened, the square-vented fixture's twelve concave edges no longer
/// refuse on convexity at all: the request runs all the way to the
/// exact ring-clearance pass, which reads the vent's mouth as a ring of
/// the cavity ceiling and refuses because its carrier edges are LINES,
/// not a circle. This is the measured reason the shipped fixture's vent
/// is round, and it pins the door ORDER the PR narrates (corner config,
/// then chain admission, then ring clearance).
#[test]
fn p1_a_square_vent_refuses_at_the_ring_clearance_door_not_a_convexity_one() {
    let body = square_vented_cavity();
    let edges = edges_with_corners(&body, cavity_corner);
    assert_eq!(edges.len(), 12, "the square-vented cavity's twelve edges");
    let err = chamfer_edges(&body, &edges, D, Tol::witness())
        .expect_err("the square vent's ring must refuse");
    let text = err.error.to_string();
    assert!(
        text.contains("not a circle"),
        "expected the ring-clearance circle refusal, got: {text}"
    );
    assert!(
        !matches!(
            err.error,
            BlendError::UnsupportedCorner { .. } | BlendError::UnsupportedChain { .. }
        ),
        "the refusal must come from PAST both convexity doors, got {:?}",
        err.error
    );
}

/// The shipped fixture (round vent), re-authored here.
fn vented_cavity() -> Body<f64> {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(1.5, 2.0), 1.0),
        ProfileVertex::new(Point2::new(2.5, 2.0), 1.0),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, 2.5) - Point3::origin(),
    ));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("a circle is a valid profile");
    let vent = extrude(&profile, Extrusion::Distance(2.5), Tol::witness())
        .expect("a rod extrudes")
        .body;
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    cut(&cut(&block, &vent), &cavity)
}

/// **P2 — the fixture's concavity, checked geometrically.** For each of
/// the twelve cavity edges, read the two support faces' OUTWARD normals
/// off the stored plane + sense bit and displace the edge midpoint
/// along their sum: on a concave edge of this fixture that lands
/// strictly inside the cavity void `(1,3)³`, which for axis-aligned
/// planes is exactly the material-wedge-greater-than-pi statement. And
/// each of the eight cavity corners is trivalent with all three of its
/// edges in the cavity set — the all-concave trihedron the unit claims,
/// established without asking the battery's own classifier.
#[test]
fn p2_all_twelve_cavity_edges_are_concave_and_the_eight_corners_trivalent() {
    let body = vented_cavity();
    let edges = edges_with_corners(&body, cavity_corner);
    assert_eq!(edges.len(), 12, "twelve cavity edges");
    assert_eq!(validate_closed(&body), Ok(()), "tier 2");

    let outward = |face: topo::FaceKey| -> Vec3<f64> {
        let f = body.get_face(face).expect("a support face");
        let Some(Surface::Plane { normal, .. }) = body.get_surface(f.surface) else {
            panic!("a cavity support face is a plane");
        };
        *normal * f.sense_sign::<f64>()
    };

    let mut corner_edges: std::collections::BTreeMap<(i64, i64, i64), usize> =
        std::collections::BTreeMap::new();
    for e in &edges {
        let ed = body.get_edge(*e).expect("edge");
        let h = body.get_half_edge(ed.he_plus).expect("half-edge");
        let end = body.half_edge_end(ed.he_plus).expect("end");
        let pt = |v| {
            *body
                .get_vertex(v)
                .and_then(|x| body.get_point(x.point))
                .expect("point")
        };
        let (a, b) = (pt(h.start), pt(end));
        let mid = Point3::new(0.5 * (a.x + b.x), 0.5 * (a.y + b.y), 0.5 * (a.z + b.z));
        // The two support faces via the edge's half-edges' parent loops.
        let face_of = |he| {
            let h = body.get_half_edge(he).expect("half-edge");
            body.get_loop(h.parent_loop).expect("loop").face
        };
        let fa = face_of(ed.he_plus);
        let fb = face_of(ed.he_minus);
        let n = outward(fa) + outward(fb);
        let q = mid + n * 0.05;
        assert!(
            q.x > 1.0 && q.x < 3.0 && q.y > 1.0 && q.y < 3.0 && q.z > 1.0 && q.z < 3.0,
            "edge {e:?}: outward-normal sum must point into the cavity void \
             (concave); landed at {q:?}"
        );
        for p in [a, b] {
            let key = (p.x.round() as i64, p.y.round() as i64, p.z.round() as i64);
            *corner_edges.entry(key).or_insert(0) += 1;
        }
    }
    assert_eq!(corner_edges.len(), 8, "eight cavity corners");
    assert!(
        corner_edges.values().all(|c| *c == 3),
        "every cavity corner is trivalent with all three edges in the set: {corner_edges:?}"
    );
}

/// **P3 — the impossibility argument, attacked with a pocket.** A
/// pocket `[1,3]² × [2,4]` open to the top face has FOUR floor corners
/// that are genuinely all-concave trihedra — so the smaller fixture is
/// not stopped by the corner door reading them wrong. What stops it is
/// the argument's actual mechanism, and both halves are pinned here:
///
/// - requesting the four floor edges alone leaves each floor corner
///   with exactly two requested edges, so the battery walks them into
///   ONE CLOSED chain and refuses it at the G1 door (the corners are
///   sharp) — a subtly different mechanism from the PR's "chain ends
///   must be trivalent-and-fully-requested" framing, which speaks only
///   once a chain HAS ends, but the same conclusion;
/// - completing the request — here the WHOLE pocket component, floor,
///   struts and even the convex rim — reaches the struts' top ends,
///   which are the rim's mixed corners: the corner door refuses
///   `MixedConvexity`, and no subset of this body's edge graph avoids
///   them, because the concave edges and the convex rim are ONE
///   component.
#[test]
fn p3_a_pocket_cannot_supply_a_complete_concave_request() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let pocket = brick(Point3::new(1.0, 1.0, 2.0), Point3::new(3.0, 3.0, 5.0));
    let body = cut(&block, &pocket);

    let floor_corner = |p: Point3<f64>| {
        (p.z - 2.0).abs() < 1e-12
            && [p.x, p.y]
                .iter()
                .all(|c| (c - 1.0).abs() < 1e-12 || (c - 3.0).abs() < 1e-12)
    };
    let on_pocket_vertical = |p: Point3<f64>| {
        [p.x, p.y]
            .iter()
            .all(|c| (c - 1.0).abs() < 1e-12 || (c - 3.0).abs() < 1e-12)
            && p.z > 2.0 - 1e-12
            && p.z < 4.0 + 1e-12
    };

    let floor = edges_with_corners(&body, floor_corner);
    assert_eq!(floor.len(), 4, "the pocket floor's four concave edges");
    let err = chamfer_edges(&body, &floor, D, Tol::witness())
        .expect_err("the floor alone is an incomplete request");
    assert!(
        matches!(err.error, BlendError::ChainNotG1 { .. }),
        "the floor-only request walks into a closed sharp-cornered chain \
         and refuses at the G1 door, got {:?}",
        err.error
    );

    let full = edges_with_corners(&body, on_pocket_vertical);
    assert_eq!(full.len(), 12, "floor, struts, and the pocket's convex rim");
    let err = chamfer_edges(&body, &full, D, Tol::witness())
        .expect_err("even the whole pocket component meets the rim's mixed corners");
    assert!(
        matches!(
            err.error,
            BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { .. },
                ..
            }
        ),
        "the struts' top ends are mixed corners, got {:?}",
        err.error
    );
}

/// **P4 — the fillet's mixed-corner refusal did not move.** The
/// L-bracket's inner edge (the fixture #919 pinned the OPENING state
/// on) still refuses the fillet at the corner door with the same
/// `MixedConvexity { convex: 2 }` tag the pre-widening classifier
/// raised — the split touched only the uniform `convex == 0` row.
#[test]
fn p4_the_l_bracket_inner_edge_still_refuses_the_fillet_as_mixed() {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(2.0, 1.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 2.0),
        Point2::new(0.0, 2.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the L is a valid profile");
    let body = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .expect("the bracket extrudes")
        .body;
    let inner = edges_with_corners(&body, |p| {
        (p.x - 1.0).abs() < 1e-12 && (p.y - 1.0).abs() < 1e-12
    });
    assert_eq!(inner.len(), 1, "the bracket's one reflex vertical edge");
    let err =
        fillet_edges(&body, &inner, 0.1, Tol::witness()).expect_err("the mixed corner refuses");
    match err.error {
        BlendError::UnsupportedCorner {
            corner: CornerConfig::MixedConvexity { convex },
            ..
        } => assert_eq!(
            convex, 2,
            "two of the three edges at the strut end are convex"
        ),
        ref other => panic!("expected the mixed-corner refusal, got {other:?}"),
    }
}
