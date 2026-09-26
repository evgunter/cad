//! **Test fixtures for the pick door**, behind the `test-support`
//! feature (on only through dev-dependency edges): the rays a pick row
//! aims, the near-tangent candidate the certified test is probed with,
//! the door's answer read as a list, and the uncertified determinant the
//! review rows read the certified one against.
//!
//! One home for three readers: `resolve::pick`'s own unit tests, this
//! crate's pick suites, and `viewer`'s corpus pick suites (which enable
//! the feature through their own dev-dependency). The ray from arrays is
//! `bvh::test_support::ray` — the ray IS `bvh`'s — and not restated.
//!
//! Whether a door here carries an oracle is asked door by door, in each
//! door's own docs.

// Panicking is a fixture's failure mechanism (workspace lint note).
#![allow(clippy::panic)]

use bvh::Ray;
use bvh::test_support::ray;
use geom_core::{Point3, Vec3};

use crate::{HitTestError, PickHit};

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
