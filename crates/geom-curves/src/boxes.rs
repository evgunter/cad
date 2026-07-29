//! Certified-conservative [`Aabb`] constructors for curve carriers
//! (C10, M5 PR 8): a box is a cache with a **containment contract** —
//! the carrier's true locus over the stated span lies inside it — and
//! every computation here errs outward only.
//!
//! These land now and are consumed later (the planar boolean consumes
//! only vertex-extent boxes in PR 8); they live HERE, not in the `bvh`
//! crate, so the tree stays below the geometry crates (PR 7's SSI
//! subdivision duty inside them must be able to consume it — see
//! `bvh`'s crate docs) and each constructor sits next to the invariant
//! it cites.
//!
//! This is certification/driver code, so [`Bounds`] is in charter (the
//! L7 evaluation-code discipline names exactly this territory): every
//! scalar enters as its `[lo(), hi()]` bracket, poison (NaN) flows to
//! the poison box, which never prunes.

use bvh::Aabb;
use geom_core::{Bounds, Point3};

use crate::Curve3;
use crate::nurbs::NurbsCurve3;

/// A one-dimensional outward bracket: plain `f64` interval arithmetic
/// where every ring operation widens its result by one ulp per side.
/// `+`, `−`, `×`, `√` are correctly rounded at `f64` (true result
/// within half an ulp of the returned one), so the widening makes each
/// bracket a true enclosure; NaN propagates (poison stays loud).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Brk {
    pub(crate) lo: f64,
    pub(crate) hi: f64,
}

impl Brk {
    pub(crate) fn of<T: Bounds>(x: T) -> Self {
        Self {
            lo: x.lo(),
            hi: x.hi(),
        }
    }

    fn out(lo: f64, hi: f64) -> Self {
        Self {
            lo: lo.next_down(),
            hi: hi.next_up(),
        }
    }

    pub(crate) fn add(self, o: Self) -> Self {
        Self::out(self.lo + o.lo, self.hi + o.hi)
    }

    pub(crate) fn sub(self, o: Self) -> Self {
        Self::out(self.lo - o.hi, self.hi - o.lo)
    }

    pub(crate) fn mul(self, o: Self) -> Self {
        let (a, b, c, d) = (
            self.lo * o.lo,
            self.lo * o.hi,
            self.hi * o.lo,
            self.hi * o.hi,
        );
        // NaN-propagating min/max: `f64::min` ignores NaN, which would
        // silently narrow poison away (D4 ¶2).
        let lo = pfold(a, pfold(b, pfold(c, d, f64::min), f64::min), f64::min);
        let hi = pfold(a, pfold(b, pfold(c, d, f64::max), f64::max), f64::max);
        Self::out(lo, hi)
    }

    /// √ of the nonnegative part (used on sums of squares; a slightly
    /// negative `lo` from outward rounding clamps to 0 — outward-safe
    /// for a true value that is a square sum, hence ≥ 0).
    pub(crate) fn sqrt_nonneg(self) -> Self {
        if self.lo.is_nan() || self.hi.is_nan() || self.hi < 0.0 {
            return Self {
                lo: f64::NAN,
                hi: f64::NAN,
            };
        }
        Self::out(self.lo.max(0.0).sqrt(), self.hi.sqrt())
    }
}

fn pfold(a: f64, b: f64, f: fn(f64, f64) -> f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        f(a, b)
    }
}

/// Angular slack (radians) added on BOTH sides of every span-membership
/// test below: it absorbs `libm::atan2`'s deviation from the exact
/// value (observed ≤ 4 ulps in the geom-core census — this is 6+ orders
/// more) plus the membership arithmetic's own rounding. Slack only ever
/// *includes* more extrema, so it errs outward (a looser box).
const ANGLE_SLOP: f64 = 1e-6;

/// Whether some representative `phi + 2πk` possibly lies in
/// `[lo, hi]` (already slop-widened). Conservative-inclusive: any NaN
/// answers `true` (poison never excludes).
fn angle_in_span(phi: f64, lo: f64, hi: f64) -> bool {
    if phi.is_nan() || lo.is_nan() || hi.is_nan() {
        return true;
    }
    if hi - lo >= core::f64::consts::TAU {
        return true;
    }
    let k = ((lo - phi) / core::f64::consts::TAU).ceil();
    let rep = phi + core::f64::consts::TAU * k;
    // NaN-inclusive: a poisoned representative cannot prove exclusion.
    rep.is_nan() || rep <= hi
}

/// The certified-conservative box of a **circular arc**: the carrier's
/// `Circle` frame over the certified span `[theta0, theta1]`, seeded
/// with the arc's (certified) endpoint points `end0`/`end1`.
///
/// Closed form, outward-only (the private `Brk` outward bracket): per
/// world axis the coordinate
/// is `cᵢ + r·Aᵢ·cos(θ − φᵢ)` with amplitude `Aᵢ = √(uᵢ² + vᵢ²)`
/// (`v = axis × u_ref`, computed bracket-wise), so the span extremum on
/// that axis is `cᵢ ± r·Aᵢ` — included exactly when the extremal angle
/// `φᵢ = atan2(vᵢ, uᵢ)` (or `φᵢ + π`) possibly lies in the span
/// (`ANGLE_SLOP`-widened, conservative-inclusive); otherwise the
/// endpoint hull already bounds the monotone piece. The endpoints
/// enter as brackets; residual padding (vertices sit on carriers only
/// up to certification) is the CALLER's `Aabb::padded` obligation,
/// exactly as for vertex-extent boxes.
///
/// `None` when `carrier` is not a `Circle` (the caller named the wrong
/// lane — refuse loudly rather than guess). Poison anywhere yields
/// poison bounds, which never prune.
pub fn circle_arc_aabb<T: Bounds>(
    carrier: &Curve3<T>,
    theta0: T,
    theta1: T,
    end0: Point3<T>,
    end1: Point3<T>,
) -> Option<Aabb> {
    let Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = carrier
    else {
        return None;
    };
    // Endpoint hull: always inside the box (2 points — never empty).
    let mut b = Aabb::from_points([end0, end1]).unwrap_or_else(Aabb::poison);

    let r = Brk::of(*radius);
    let (ax, ay, az) = (Brk::of(axis.x), Brk::of(axis.y), Brk::of(axis.z));
    let (ux, uy, uz) = (Brk::of(u_ref.x), Brk::of(u_ref.y), Brk::of(u_ref.z));
    // v = axis × u_ref, bracket-wise (the same fixed component order as
    // `Vec3::cross`).
    let vx = ay.mul(uz).sub(az.mul(uy));
    let vy = az.mul(ux).sub(ax.mul(uz));
    let vz = ax.mul(uy).sub(ay.mul(ux));

    // The slop-widened span (orientation-normalized outward).
    let (s0, s1) = (theta0.lo(), theta1.hi());
    let lo = pfold(s0, s1, f64::min) - ANGLE_SLOP;
    let hi = pfold(s0, s1, f64::max) + ANGLE_SLOP;

    axis_extremum(
        &mut b.min_x,
        &mut b.max_x,
        Brk::of(center.x),
        ux,
        vx,
        r,
        lo,
        hi,
    );
    axis_extremum(
        &mut b.min_y,
        &mut b.max_y,
        Brk::of(center.y),
        uy,
        vy,
        r,
        lo,
        hi,
    );
    axis_extremum(
        &mut b.min_z,
        &mut b.max_z,
        Brk::of(center.z),
        uz,
        vz,
        r,
        lo,
        hi,
    );
    Some(b)
}

/// One world axis of [`circle_arc_aabb`]: extends `[min, max]` by the
/// axis extremum `c ± r·√(u² + v²)` when its angle possibly lies in
/// the (widened) span.
#[allow(clippy::too_many_arguments)] // one bracket per named quantity
fn axis_extremum(min: &mut f64, max: &mut f64, c: Brk, u: Brk, v: Brk, r: Brk, lo: f64, hi: f64) {
    let amp = r.mul(u.mul(u).add(v.mul(v)).sqrt_nonneg());
    // Midpoint atan2 is fine: ANGLE_SLOP dwarfs both the bracket
    // widths' effect on atan2 (session-box scale) and libm's error.
    let phi = geom_core::Real::atan2((v.lo + v.hi) * 0.5, (u.lo + u.hi) * 0.5);
    if angle_in_span(phi, lo, hi) {
        *max = pfold(*max, c.add(amp).hi, f64::max);
    }
    if angle_in_span(phi + core::f64::consts::PI, lo, hi) {
        *min = pfold(*min, c.sub(amp).lo, f64::min);
    }
}

/// The certified-conservative box of a NURBS curve: the AABB of its
/// control-point brackets. Sound by the convex-hull property — every
/// curve point is a convex combination of control points because the
/// basis functions are nonnegative and the weights are **strictly
/// positive by construction** (the PR 3 positive-weights invariant,
/// enforced by [`NurbsCurve3::new`]; negative weights would void
/// convexity, Book p. 293). Valid over the whole domain, a fortiori
/// over any certified span (span-tight hulls via knot refinement are a
/// later sharpening; looser is conservative). No arithmetic — brackets
/// only — so no rounding to pad. The placeholder curve's all-poison
/// control points yield the poison box, which never prunes.
pub fn nurbs_curve_aabb<T: Bounds>(curve: &NurbsCurve3<T>) -> Aabb {
    Aabb::from_points(curve.control().iter().copied()).unwrap_or_else(Aabb::poison)
}
