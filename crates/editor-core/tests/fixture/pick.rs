//! **The pick seam's test vocabulary**: the rays a pick suite aims, the
//! one near-tangent candidate the certified test is probed with, and the
//! door's answer read as a list.
//!
//! Reached from two binaries. This crate's suites name it as
//! `crate::fixture::pick`, and `viewer`'s aggregated test binary mounts
//! this whole tree through its `fixture` symlink, so its corpus pick
//! suites and its `tests/common` read the same definitions — a ray
//! spelled here is spelled once for both crates.
//!
//! Whether a door here carries an oracle is asked door by door. The
//! rays do not: a wrong ray misses, or meets another face, and the row
//! that aimed it reds on its own premise; [`listed`] reads the door and
//! decides nothing about what it answered. [`det_and_conditioning`]
//! does: it is the UNCERTIFIED determinant the review rows read the
//! certified test against, and it shares no code with that test.

use editor_core::{HitTestError, PickHit, Ray};
use geom_core::{Point3, Vec3};

/// **The six axis directions**, `+x, -x, +y, -y, +z, -z` in that order —
/// what every aim that walks a mesh from outside it fires along.
///
/// The order is part of the value: a sweep that caps its printed
/// examples keeps the first few it meets, so a reordering changes what
/// a failure message shows and nothing a row counts.
pub const AXES: [Vec3<f64>; 6] = [
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, -1.0),
];

/// A ray from its origin and direction as coordinate arrays.
pub fn ray(origin: [f64; 3], dir: [f64; 3]) -> Ray {
    Ray {
        origin: Point3::new(origin[0], origin[1], origin[2]),
        dir: Vec3::new(dir[0], dir[1], dir[2]),
    }
}

/// A ray straight down through `(x, y)` from height `z`.
pub fn down_from(x: f64, y: f64, z: f64) -> Ray {
    Ray {
        origin: Point3::new(x, y, z),
        dir: Vec3::new(0.0, 0.0, -1.0),
    }
}

/// **A ray aimed at `target`**: along `dir`, from `reach` directions
/// back, so the target is at parameter `reach` — and at distance
/// `reach` when `dir` is a unit vector.
///
/// The origin is `target - dir * reach` computed once, here; a suite
/// that compares a hit's `t` against `reach` bit for bit is comparing
/// against the rounding of exactly this expression.
pub fn aimed(target: Point3<f64>, dir: Vec3<f64>, reach: f64) -> Ray {
    Ray {
        origin: target - dir * reach,
        dir,
    }
}

/// **The near-tangent candidate**: a determinant certified at `k / 6`
/// of its own bound, `u = v = 0.5` exactly, `t = 1.5`.
///
/// `editor-core`'s `src/resolve/pick.rs` holds the same construction in
/// its own unit-test module, which cannot import a `tests/` tree; this
/// is the one the integration suites share.
pub fn near_tangent(k: f64) -> (Ray, [Point3<f64>; 3]) {
    let zeta = 2f64.powi(-20);
    let xi = k * zeta * f64::EPSILON;
    let tri = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, zeta + xi),
        Point3::new(0.0, 1.0, 0.0),
    ];
    let ray = ray([-1.0, -1.0, 0.5 * xi - zeta], [1.0, 1.0, zeta]);
    (ray, tri)
}

/// **A pick door's whole answer as a list**: the one hit, the empty
/// miss, or the tied faces of a refusal.
///
/// A certified tie between faces is an ANSWER about the ray — every hit
/// in it is true — so a row that reads the door reads it this way
/// rather than unwrapping past it.
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
/// conditioning** `|det| / (|e1|·|e2|·|d|)` — the sine of the
/// ray/plane angle up to a constant.
///
/// The reference a row reads `crossing`'s refusals against: a refused
/// candidate whose conditioning is far above the certification's own
/// bound is a genuine crossing the door failed to certify. It is
/// computed here in plain `f64`, not through the door, so the rows
/// reading it compare two computations rather than one with itself.
pub fn det_and_conditioning(ray: &Ray, tri: &[Point3<f64>; 3]) -> (f64, f64) {
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let det = e1.dot(ray.dir.cross(e2));
    (det, det.abs() / (e1.norm() * e2.norm() * ray.dir.norm()))
}
