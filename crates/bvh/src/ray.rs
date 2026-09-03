//! The ray query's vocabulary and its conservative slab test.
//!
//! Picking support (crate docs: the viewport-picking duty): a [`Ray`]
//! is origin + direction over `t ∈ [0, ∞)`, and [`Ray::slab_enter`] is
//! the per-box test [`crate::Bvh::ray`] traverses with. Everything here
//! is plain `f64` with conservative comparisons — picking is a UI
//! concern with no D9 predicate obligation (GQ6 re-survey §3 note),
//! and the crate's conservative-superset contract is what the test is
//! written against: it may answer "candidate" for a box the ray misses,
//! and it never answers "disjoint" for a box the ray truly intersects —
//! at any magnitude, overflow included ([`Ray::slab_enter`] carries
//! the corner-by-corner argument).

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
    /// ([`Ray::slab_enter`] docs), never NaN, never `+∞`. This is what
    /// licenses a consumer early-out — once a confirmed hit at
    /// `t* < t_enter` exists, this box cannot contain a nearer hit.
    pub t_enter: f64,
}

/// The slab test's outward widening for the PER-ITEM query (4 ULP
/// steps per endpoint): strictly covers the ≤ 2 roundings each
/// endpoint carries ([`slab_t`]), with margin.
const ITEM_WIDEN_STEPS: u32 = 4;

/// The slab test's outward widening for INTERNAL-NODE hulls (8 ULP
/// steps): [`ITEM_WIDEN_STEPS`] plus enough slack to dominate the
/// ≤ ~2-ULP disagreement the overflow-recompute seam in [`slab_t`]
/// can introduce between a hull's endpoint and an item's, so a hull
/// prune never drops an item whose own test would accept
/// ([`crate::Bvh::ray`] states the resulting invariant).
const HULL_WIDEN_STEPS: u32 = 8;

/// One slab endpoint: the parameter at which the ray's coordinate
/// reaches `bound`, computed as `(bound − o) / d` with `d ≠ 0`.
///
/// Two properties this spelling buys over the classic
/// `(bound − o) · (1/d)`:
///
/// - **Division cannot mint a fake infinity.** `fl(s / d)` is the
///   correctly-rounded true quotient of the (exactly-rounded)
///   difference, so it overflows to `±∞` only when the true quotient
///   really is beyond `f64` range. The reciprocal composition could
///   lie: a subnormal `|d| < ~5.6e−309` makes `1/d = ±∞` while the
///   true quotient is moderate.
/// - **An overflowed subtraction is recomputed exactly in halves.**
///   When `bound − o` overflows with both operands finite, both
///   operands (or all but a subnormal one) are huge normals, whose
///   halving is exact; the halved difference is ≤ `MAX/1` in
///   magnitude, so it rounds instead of overflowing, and the final
///   `× 2` is exact (or a GENUINE overflow). A subnormal operand's
///   halving error is ≤ 2⁻¹⁰⁷⁵ absolute against a ≈ 2⁵¹¹-magnitude
///   difference — dwarfed by the caller's ULP widening.
///
/// Every result is therefore within 2 roundings of the true value
/// (subtract + divide; the halved lane adds only the negligible term
/// above and the exact `× 2`), or a genuine `±∞` (true value beyond
/// `f64` range, or an infinite input), or NaN (a NaN input, or `∞/∞`
/// from a non-finite ray — the caller skips those conservatively).
fn slab_t(bound: f64, o: f64, d: f64) -> f64 {
    let s = bound - o;
    if s.is_infinite() && bound.is_finite() && o.is_finite() {
        ((bound * 0.5 - o * 0.5) / d) * 2.0
    } else {
        s / d
    }
}

/// `steps` ULP steps downward. The outward-widening idiom of the ray
/// query — the ray-parameter-space sibling of [`Aabb::padded`]'s
/// 1-ULP-per-bound coordinate-space widening: different budgets
/// because they cover different arithmetic (`padded` covers its own
/// two rounding steps per bound; this covers [`slab_t`]'s per-endpoint
/// error under [`ITEM_WIDEN_STEPS`]/[`HULL_WIDEN_STEPS`]).
fn widen_down(v: f64, steps: u32) -> f64 {
    let mut v = v;
    for _ in 0..steps {
        v = v.next_down();
    }
    v
}

/// [`widen_down`]'s upward dual.
fn widen_up(v: f64, steps: u32) -> f64 {
    let mut v = v;
    for _ in 0..steps {
        v = v.next_up();
    }
    v
}

impl Ray {
    /// The conservative ray-box test: `Some(t_enter)` unless the box
    /// is **definitely** disjoint from the ray over `t ∈ [0, ∞)`.
    ///
    /// The slab intersection folds per-axis parameter intervals into
    /// `[t_min, t_max]`, starting from the ray's own domain `[0, ∞)`
    /// (so `t_enter = t_min` is never negative: a box containing the
    /// origin enters at exactly `0`). The verdict is `t_min ≤ t_max` —
    /// **closed**, so a grazing touch stays a candidate, matching the
    /// crate's closed-box convention ([`Aabb::overlaps`]: touching
    /// boxes overlap).
    ///
    /// # The corners, and which way each falls
    ///
    /// - **`d = ±0.0` is its own arm, with no arithmetic at all**: the
    ///   ray's coordinate is the constant `o`, so the axis either
    ///   never constrains `t` (`o` inside the closed slab — including
    ///   exactly ON a bound, the case that used to be the `0 × ∞ →
    ///   NaN` trap) or is disjoint outright (`o < lo` or `o > hi`,
    ///   an EXACT prune on **both** sides — comparisons, not
    ///   products). A NaN anywhere (poison bound, poison origin)
    ///   satisfies neither comparison, so poison never prunes here.
    /// - **`d ≠ 0` endpoints come from [`slab_t`]** — division, plus
    ///   the exact halved recompute of an overflowed subtraction — so
    ///   an infinity here is GENUINE: an infinite box bound (an
    ///   unbounded slab side), or a true parameter beyond `f64` range.
    /// - **NaN endpoints skip the axis** (no constraint): NaN arises
    ///   only from poison inputs or a non-finite ray (`∞/∞`), and
    ///   skipping only widens the answer — **NaN can never witness
    ///   disjointness**.
    /// - **Genuine infinities are handled by the widening itself**:
    ///   `widen_down(+∞)` is ≈ `MAX`, a *valid lower bound* for an
    ///   entry that truly lies beyond `f64` range — the box stays a
    ///   candidate with `t_enter ≈ MAX` (also why `t_enter` is never
    ///   `+∞`); `widen_up(−∞)` is ≈ `−MAX`, a *valid upper bound* for
    ///   an exit truly below `−MAX` — the box is entirely behind the
    ///   origin and prunes against `t_min ≥ 0`. `−∞ near` / `+∞ far`
    ///   are fixed points and simply never tighten the fold.
    /// - **Zero-extent axes** (`lo == hi`) degenerate to
    ///   `near == far` before widening — a plane box stays hittable.
    ///   **Inverted boxes** (`lo > hi`, the crate's empty-box
    ///   convention) fall into the `near ≤ far` swap and behave like
    ///   the un-inverted slab: possibly extra candidates, never a
    ///   wrongly pruned true intersection (an empty box has none to
    ///   miss).
    /// - **Rounding**: each finite endpoint carries ≤ 2 roundings
    ///   ([`slab_t`]), relative error < `3·2⁻⁵³`; the 4-ULP outward
    ///   widening ([`ITEM_WIDEN_STEPS`]) strictly covers it (1 ULP of
    ///   finite `v` is ≥ `2⁻⁵³·|v|`; in the subnormal range the
    ///   per-step absolute half-ULP errors are covered the same way).
    ///
    /// Together: **no magnitude precondition** — the returned interval
    /// always contains the true one, so a truly intersected box is
    /// never refused, and `t_enter` is a conservative lower bound on
    /// the true entry parameter (each per-axis `near` is a widened
    /// lower bound, skipped axes only lower the fold, and the `0`
    /// floor lower-bounds the domain itself).
    pub fn slab_enter(&self, b: &Aabb) -> Option<f64> {
        self.slab_enter_widened(b, ITEM_WIDEN_STEPS)
    }

    /// [`Ray::slab_enter`] with the hull widening ([`HULL_WIDEN_STEPS`])
    /// — the internal-node test of [`crate::Bvh::ray`], deliberately
    /// WEAKER than the per-item test so a hull prune can never
    /// out-prune the items under it (the constant's docs).
    pub(crate) fn slab_enter_hull(&self, b: &Aabb) -> bool {
        self.slab_enter_widened(b, HULL_WIDEN_STEPS).is_some()
    }

    /// The fold shared by the two entry points above; `steps` is the
    /// per-endpoint outward widening.
    fn slab_enter_widened(&self, b: &Aabb, steps: u32) -> Option<f64> {
        let mut t_min = 0.0f64;
        let mut t_max = f64::INFINITY;
        for (o, d, lo, hi) in [
            (self.origin.x, self.dir.x, b.min_x, b.max_x),
            (self.origin.y, self.dir.y, b.min_y, b.max_y),
            (self.origin.z, self.dir.z, b.min_z, b.max_z),
        ] {
            if d == 0.0 {
                // The exact zero-direction arm (docs above): inside
                // the closed slab ⇒ no constraint; strictly outside ⇒
                // definitely disjoint, both sides; NaN ⇒ no constraint.
                if o < lo || o > hi {
                    return None;
                }
                continue;
            }
            let t0 = slab_t(lo, o, d);
            let t1 = slab_t(hi, o, d);
            if t0.is_nan() || t1.is_nan() {
                continue;
            }
            let (near, far) = if t0 <= t1 { (t0, t1) } else { (t1, t0) };
            let near = widen_down(near, steps);
            let far = widen_up(far, steps);
            if near > t_min {
                t_min = near;
            }
            if far < t_max {
                t_max = far;
            }
        }
        (t_min <= t_max).then_some(t_min)
    }
}

// TCOST-1 gate demonstration — this branch is evidence for PR #1612 and is
// never merged.
