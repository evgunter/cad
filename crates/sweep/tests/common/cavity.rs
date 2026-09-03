//! **The vented-cavity fixture vocabulary** — the bodies the concave
//! blend suites carve, authored the way a user would: a profile on a
//! sketch plane, extruded, then cut.
//!
//! Everything here is BODY AUTHORING and carries no derivation, so
//! sharing it cannot make two suites agree by construction about
//! anything they check. What it does buy is the opposite of
//! independence, and that is the point: the unit suite and its review
//! probes must carve THE SAME BODY or their rows are not about each
//! other. A probe that re-authors the fixture checks a body no other
//! row measures, and a fixture that drifts here reddens every suite at
//! once instead of silently splitting the corpus in two.
//!
//! **Deliberately not absorbed**, and the whole of it:
//!
//! - the SQUARE-vented cavity (`blend3_r2_probes::square_vented_cavity`)
//!   and the SKEWED one (`blend4_r1_probes::skewed_cavity`) — each is
//!   one probe's own fixture, built from the constructors below;
//! - every corner PREDICATE. [`edges_with_corners`] is the traversal;
//!   which points count as corners is the caller's own fixture value
//!   (the `[1,3]³` cavity at 1e-12 in [`cavity_corner`], the block's
//!   `[0,4]³` at 1e-9 in `review_blend3_r1_probes::block_edges`, the
//!   parallelogram's four vertices at 1e-9 in `blend4_r1_probes`), and
//!   a predicate moved here would be a fixture value moved;
//! - the `prism`/`brick` builders of the OTHER crates' suites
//!   (`topo`, `mesh`, `stl`, `step-export`, `editor-core`) — a
//!   cross-crate home is LIB-U6's territory, which this tree's routing
//!   rule says is deliberately not built here.

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, subtract};

/// The XY sketch plane translated to height `z`.
pub fn sketch_at(z: f64) -> SketchPlane<f64> {
    SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z) - Point3::origin(),
    ))
}

/// A polygon on the plane at `z0`, extruded to `z1`.
pub fn prism(poly: &[Point2<f64>], z0: f64, z1: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon(poly.iter().copied());
    let profile = Profile::new(sketch_at(z0), vec![lp])
        .validate(Tol::witness())
        .expect("a convex polygon is a valid profile");
    extrude(&profile, Extrusion::Distance(z1 - z0), Tol::witness())
        .expect("a prism extrudes")
        .body
}

/// An axis-aligned box: the rectangle `lo.xy … hi.xy` on the plane at
/// `lo.z`, extruded to `hi.z`.
pub fn brick(lo: Point3<f64>, hi: Point3<f64>) -> Body<f64> {
    prism(
        &[
            Point2::new(lo.x, lo.y),
            Point2::new(hi.x, lo.y),
            Point2::new(hi.x, hi.y),
            Point2::new(lo.x, hi.y),
        ],
        lo.z,
        hi.z,
    )
}

/// A circular rod: two half-arc profile segments extruded, so its wall
/// is a cylinder and the ring it cuts in a plane is a circle.
pub fn rod(center: Point2<f64>, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(center.x - r, center.y), 1.0),
        ProfileVertex::new(Point2::new(center.x + r, center.y), 1.0),
    ]);
    let profile = Profile::new(sketch_at(z0), vec![lp])
        .validate(Tol::witness())
        .expect("a circle is a valid profile");
    extrude(&profile, Extrusion::Distance(z1 - z0), Tol::witness())
        .expect("a rod extrudes")
        .body
}

/// `base` less `tool`, demanding that the cut succeed and leave
/// material — a fixture step, so a refusal here is a broken fixture
/// rather than a failed claim.
pub fn cut(base: &Body<f64>, tool: &Body<f64>) -> Body<f64> {
    subtract(base, tool, Tol::witness())
        .expect("the cut succeeds")
        .body()
        .expect("the cut leaves material")
        .body
        .clone()
}

/// **A block with a rectangular cavity, vented.**
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
/// which is what makes a request for them a complete one.
///
/// # Why the fixture is a vented cavity, and what that does not claim
///
/// The requirement is a CLASS, not this shape. A concave request must
/// be a whole component of the edge graph, and two doors say so from
/// different sides: a subset with loose ends refuses where those ends
/// are read, and a subset with NO ends never gets that far — the
/// pocket floor's four edges close into a sharp-cornered ring and
/// refuse at the G1 door, because the trivalent-ends clause only
/// speaks once a chain has ends at all. A concave component that
/// reaches the surface always ends at mixed corners, so it must
/// enclose; an enclosed void is two shells, which the body door
/// refuses; hence a vented cavity.
///
/// **That argument does not make this the smallest such body.** A
/// triangular prism cavity carves the same way with nine edges and six
/// corners. Both probe suites hold the bound: the pocket attacks and
/// the nine-edge cavity carves.
pub fn vented_cavity() -> Body<f64> {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.0, 2.0), 0.5, 2.5, 5.0);
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    cut(&cut(&block, &vent), &cavity)
}

/// Every edge both of whose endpoints satisfy `on` — found by
/// POSITION, never by index, so a row cannot silently address a
/// different edge. The result is sorted, so it is a set with a
/// deterministic order.
pub fn edges_with_corners(body: &Body<f64>, on: impl Fn(Point3<f64>) -> bool) -> Vec<EdgeKey> {
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

/// A corner of [`vented_cavity`]'s cavity box `[1,3]³`.
pub fn cavity_corner(p: Point3<f64>) -> bool {
    [p.x, p.y, p.z]
        .iter()
        .all(|c| (c - 1.0).abs() < 1e-12 || (c - 3.0).abs() < 1e-12)
}

/// [`vented_cavity`]'s twelve cavity edges.
pub fn cavity_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    edges_with_corners(body, cavity_corner)
}
