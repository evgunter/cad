//! **Test fixtures**, behind the `test-support` feature (on only
//! through dev-dependency edges): a literal of each dimension, a point
//! of two lengths, the world xy frame a sketch is drawn on, the rays a
//! pick row aims, the near-tangent candidate the certified test is
//! probed with, the door's answer read as a list, the uncertified
//! determinant the review rows read the certified one against, and the
//! recipe walks' pass-through classification a row holds against the
//! evaluator.
//!
//! One home for every reader in this crate and the crates that test
//! against it: the unit-test modules reach it as `crate::test_support`,
//! `tests/fixture` re-exports the authoring doors so a suite reads ONE
//! definition, and `viewer`'s own `test_support` re-exports them in
//! turn. The ray from arrays is `bvh::test_support::ray` — the ray IS
//! `bvh`'s — and not restated.
//!
//! Whether a door here carries an oracle is asked door by door, in each
//! door's own docs.

// Panicking is a fixture's failure mechanism (workspace lint note).
#![allow(clippy::panic, clippy::expect_used)]

use bvh::Ray;
use bvh::test_support::ray;
use geom_core::{Point3, Vec3};

use crate::{Datum, Dimension, Expr, HitTestError, Node, PickHit, ProfileProgram};

// --- literals -------------------------------------------------------

/// A length literal, in canonical metres.
///
/// # Panics
///
/// If `metres` is not finite — the refusal is `Expr::literal`'s, and a
/// row about that refusal spells the call rather than reaching here.
pub fn len(metres: f64) -> Expr {
    Expr::literal(metres, Dimension::Length).expect("a finite length")
}

/// An angle literal, in radians.
///
/// # Panics
///
/// If `radians` is not finite.
pub fn ang(radians: f64) -> Expr {
    Expr::literal(radians, Dimension::Angle).expect("a finite angle")
}

/// A dimensionless literal — a direction component, a bulge, a ratio.
///
/// # Panics
///
/// If `value` is not finite.
pub fn scl(value: f64) -> Expr {
    Expr::literal(value, Dimension::Scalar).expect("a finite scalar")
}

/// Two length literals — a point in a sketch frame's own coordinates.
///
/// # Panics
///
/// If either coordinate is not finite.
pub fn len2(v: [f64; 2]) -> [Expr; 2] {
    [len(v[0]), len(v[1])]
}

// --- the frame a sketch is drawn on ---------------------------------

/// The frame datum a profile is drawn on, as a node to insert: an
/// origin, and the two directions sketch +x and +y point.
///
/// # Panics
///
/// If a component is not finite.
pub fn frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> Node<ProfileProgram> {
    Node::Datum(Datum::Frame {
        origin: origin.map(len),
        u: u.map(scl),
        v: v.map(scl),
    })
}

/// The world xy frame as a node — origin at the world origin, sketch
/// +x along world +x, sketch +y along world +y.
pub fn xy_frame() -> Node<ProfileProgram> {
    frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
}

// --- the pick door --------------------------------------------------

/// **The six axis directions**, `+x, -x, +y, -y, +z, -z` in that order —
/// what every aim that walks a mesh from outside it fires along. No
/// oracle: which rays are fired decides nothing about what one answers.
///
/// The order is part of the value: a sweep that caps its printed
/// examples keeps the first few it meets, so a reordering changes what a
/// failure message shows and nothing a row counts.
pub const AXES: [Vec3<f64>; 6] = [
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, -1.0),
];

/// A ray straight down through `(x, y)` from height `z`. No oracle: a
/// wrong ray misses, and the row that aimed it reds on its own premise.
pub fn down_from(x: f64, y: f64, z: f64) -> Ray {
    ray([x, y, z], [0.0, 0.0, -1.0])
}

/// **A ray aimed at `target`**: along `dir`, from `reach` directions
/// back, so the target is at parameter `reach` — and at distance `reach`
/// when `dir` is a unit vector.
///
/// The origin is `target - dir * reach` computed once, here; a row that
/// compares a hit's `t` against `reach` bit for bit is comparing against
/// the rounding of exactly this expression. No oracle, as [`down_from`].
pub fn aimed(target: Point3<f64>, dir: Vec3<f64>, reach: f64) -> Ray {
    Ray {
        origin: target - dir * reach,
        dir,
    }
}

/// **The near-tangent candidate**: a crossing whose determinant is
/// certified at `k / 6` of its own bound, over a triangle of area `0.5`
/// that is not degenerate: `ζ = 2⁻²⁰` and `ξ = k` ULP of it,
/// `e1 = (1, 0, ζ + ξ)`, `e2 = (0, 1, 0)`, `d = (1, 1, ζ)`, so
/// `p = (−ζ, 0, 1)` and `det = ξ` exactly. The origin is placed a unit
/// away and one unit off-axis, which makes `u = v = 0.5` and `u + v = 1`
/// exactly at every `k` — all three INSIDE the closed range, so what
/// happens to the candidate is INFORM's doing alone, at `t = 1.5`. The
/// bounds are `9/(k − 6)`, `15/(k − 6)` and their sum, so `k` is the
/// dial that moves the intervals without moving the values.
pub fn near_tangent(k: f64) -> (Ray, [Point3<f64>; 3]) {
    let zeta = 2f64.powi(-20);
    let xi = k * zeta * f64::EPSILON;
    let tri = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, zeta + xi),
        Point3::new(0.0, 1.0, 0.0),
    ];
    (ray([-1.0, -1.0, 0.5 * xi - zeta], [1.0, 1.0, zeta]), tri)
}

/// **A pick door's whole answer as a list**: the one hit, the empty
/// miss, or the tied faces of a refusal. No oracle: it reads the door
/// and decides nothing about what it answered.
///
/// A certified tie between faces is an ANSWER about the ray — every hit
/// in it is true — so a row that reads the door reads it this way rather
/// than unwrapping past it.
///
/// # Panics
///
/// On any other refusal: that one is about the targets, not the
/// geometry, and a row reading a list aimed at targets it built.
pub fn listed(answer: Result<Option<PickHit>, HitTestError>) -> Vec<PickHit> {
    match answer {
        Ok(hit) => hit.into_iter().collect(),
        Err(HitTestError::Ambiguous { hits }) => hits,
        Err(other) => panic!("the pick answers or refuses on a tie: {other:?}"),
    }
}

/// **Möller–Trumbore's determinant, uncertified, and its
/// conditioning** `|det| / (|e1|·|e2|·|d|)` — up to a constant the sine
/// of the angle between the ray and the plane.
///
/// This one DOES carry an oracle: it is the reference a row reads
/// `crossing`'s refusals against (a refused candidate whose conditioning
/// is far above the certification's own bound is a genuine crossing the
/// door failed to certify), and a row that picks its well-conditioned
/// cases by it. It is computed here in plain `f64`, sharing no code with
/// the door, so a row reading it compares two computations rather than
/// one with itself.
pub fn det_and_conditioning(ray: &Ray, tri: &[Point3<f64>; 3]) -> (f64, f64) {
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let det = e1.dot(ray.dir.cross(e2));
    (det, det.abs() / (e1.norm() * e2.norm() * ray.dir.norm()))
}

// --- the recipe walks' pass-through set -----------------------------

/// **Whether the recipe walks read `node` as a name-carrying edge**:
/// `names::verbatim_edge` answering `Some`, the one statement of N1's
/// pass-through set that the product's two-roots check and the mate
/// member walk follow.
///
/// Carries no oracle: it IS the classification, lifted out of the
/// crate so a row can hold it against what evaluation publishes
/// (`tests/names_verbatim_edge_evaluator.rs`).
pub fn carries_names_verbatim<P>(node: &Node<P>) -> bool {
    crate::names::verbatim_edge(node).is_some()
}
