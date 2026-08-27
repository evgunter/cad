//! The ray query's vocabulary and its conservative slab test.
//!
//! Picking support (crate docs: the viewport-picking duty): a [`Ray`]
//! is origin + direction over `t ∈ [0, ∞)`, and [`Ray::slab_enter`] is
//! the per-box test [`crate::Bvh::ray`] traverses with. Everything here
//! is plain `f64` with conservative comparisons — picking is a UI
//! concern with no D9 predicate obligation (GQ6 re-survey §3 note),
//! and the crate's conservative-superset contract is what the test is
//! written against: it may answer "candidate" for a box the ray misses,
//! and must never answer "disjoint" for a box the ray truly intersects.

use geom_core::{Point3, Vec3};

use crate::aabb::Aabb;

/// A ray `origin + t · dir` over `t ∈ [0, ∞)`.
///
/// `dir` need not be normalized — no API here silently depends on unit
/// length. The parameter `t` is therefore in units of `|dir|`: all
/// entry parameters and hit parameters produced from one ray are
/// mutually comparable, and rescaling `dir` rescales them all by the
/// same factor.
///
/// A non-finite component (NaN/∞ origin or direction) is legal input
/// and fail-safe: it can only ever *lose* constraints in the slab test
/// below, so a poisoned ray prunes nothing (every box becomes a
/// candidate) rather than silently missing geometry.
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    /// The ray origin (`t = 0`).
    pub origin: Point3<f64>,
    /// The ray direction (not necessarily unit length).
    pub dir: Vec3<f64>,
}

/// One candidate from [`crate::Bvh::ray`]: an input index plus the
/// conservative entry parameter of the ray into that item's box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayCandidate {
    /// The item's input (arena) index — the same identity
    /// [`crate::Bvh::overlapping`] hands back.
    pub item: usize,
    /// A conservative LOWER bound on every `t ≥ 0` at which the ray
    /// is inside the item's box: never above the true entry parameter
    /// ([`Ray::slab_enter`] docs). This is what licenses a consumer
    /// early-out — once a confirmed hit at `t* < t_enter` exists, this
    /// box cannot contain a nearer hit.
    pub t_enter: f64,
}

/// One slab-test axis: the conservative `[near, far]` parameter
/// interval in which the ray is inside `[lo, hi]`, or `None` when the
/// axis imposes no constraint (the conservative reading of every NaN).
///
/// The IEEE corners, and which way each falls:
///
/// - `inv = 1/d` is `±∞` at `d = ±0.0` (never a division check).
/// - **The `0 × ∞ → NaN` trap**: with `d = 0` and the origin exactly
///   on a slab bound, `(bound − o) · inv = 0 · ∞ = NaN`. The ray then
///   lies in the (closed) boundary plane, so the honest constraint is
///   "no constraint" — and that is exactly what returning `None`
///   (skip the axis) implements. The same arm absorbs a poison box
///   (NaN bound) and a poison ray (NaN origin/direction component):
///   skipping an axis only widens the candidate set, so **NaN can
///   never witness disjointness**.
/// - `d = 0` with the origin strictly outside the slab: both products
///   are the same infinity, so `near = far = ±∞` and the caller's
///   `t_min ≤ t_max` verdict prunes — the ray is truly parallel to and
///   outside the slab, so the prune is exact, not just legal.
/// - A zero-extent axis (`lo == hi`) degenerates to `near == far`
///   before widening — a plane box stays hittable.
/// - An inverted axis (`lo > hi`, the crate's empty-box convention)
///   is swept up by the `near ≤ far` swap below and behaves like the
///   un-inverted slab: possibly extra candidates, never a wrongly
///   pruned true intersection (an empty box has none to miss).
///
/// **Rounding**: each endpoint is the result of ≤ 3 roundings (the
/// subtraction, the reciprocal, the product), so its relative error is
/// `< (1 + 2⁻⁵³)³ − 1 < 4·2⁻⁵³`, and 4 ULP steps cover that with
/// margin (1 ULP of `v` is ≥ `2⁻⁵³·|v|` for finite `v`; in the
/// subnormal range the per-step absolute half-ULP errors are covered
/// the same way). `near` is pushed 4 ULPs **down** and `far` 4 ULPs
/// **up**, so the returned interval always contains the exact-real
/// one; on ±∞ the outward direction is a fixed point and the inward
/// `next_down(+∞) = MAX` only ever loosens.
fn axis_interval(o: f64, d: f64, lo: f64, hi: f64) -> Option<(f64, f64)> {
    let inv = 1.0 / d;
    let t0 = (lo - o) * inv;
    let t1 = (hi - o) * inv;
    if t0.is_nan() || t1.is_nan() {
        return None;
    }
    let (near, far) = if t0 <= t1 { (t0, t1) } else { (t1, t0) };
    Some((widen_down(near), widen_up(far)))
}

/// 4 ULP steps downward ([`axis_interval`]'s rounding cover).
fn widen_down(v: f64) -> f64 {
    v.next_down().next_down().next_down().next_down()
}

/// 4 ULP steps upward ([`widen_down`]'s dual).
fn widen_up(v: f64) -> f64 {
    v.next_up().next_up().next_up().next_up()
}

impl Ray {
    /// The conservative ray-box test: `Some(t_enter)` unless the box
    /// is **definitely** disjoint from the ray over `t ∈ [0, ∞)`.
    ///
    /// The slab intersection: fold the per-axis intervals of
    /// [`axis_interval`] into `[t_min, t_max]`, starting from the
    /// ray's own domain `[0, ∞)` (so `t_enter` is never negative: a
    /// box containing the origin enters at exactly `0`). The verdict
    /// is `t_min ≤ t_max` — **closed**, so a grazing touch stays a
    /// candidate, matching the crate's closed-box convention
    /// ([`Aabb::overlaps`]: touching boxes overlap).
    ///
    /// `t_enter = t_min` is a conservative lower bound on the true
    /// entry parameter: every per-axis `near` is widened downward,
    /// skipped (NaN) axes only lower the fold, and the `0` floor
    /// lower-bounds the domain itself. A box entirely behind the
    /// origin has some exact `far < 0`; its widened `far` keeps the
    /// sign (rounding and the ULP steps cannot carry a strictly
    /// negative product past `0` from below the widening margin — and
    /// if widening lands it at `≥ 0` the box simply stays a
    /// candidate), so behind-boxes are pruned exactly when they are
    /// definitely disjoint from the `t ≥ 0` domain.
    pub fn slab_enter(&self, b: &Aabb) -> Option<f64> {
        let mut t_min = 0.0f64;
        let mut t_max = f64::INFINITY;
        for (o, d, lo, hi) in [
            (self.origin.x, self.dir.x, b.min_x, b.max_x),
            (self.origin.y, self.dir.y, b.min_y, b.max_y),
            (self.origin.z, self.dir.z, b.min_z, b.max_z),
        ] {
            if let Some((near, far)) = axis_interval(o, d, lo, hi) {
                if near > t_min {
                    t_min = near;
                }
                if far < t_max {
                    t_max = far;
                }
            }
        }
        (t_min <= t_max).then_some(t_min)
    }
}
