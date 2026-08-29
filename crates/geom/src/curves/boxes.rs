//! Certified-conservative [`Aabb`] constructors for curve carriers
//! (C10, M5 PR 8): a box is a cache with a **containment contract** —
//! the carrier's true locus over the stated span lies inside it — and
//! every computation here errs outward only.
//!
//! **Outward is the only sound direction; it is not a free one.** A
//! wider box is a weaker answer, not a cheaper one: a consumer that
//! reads non-overlap as its ANSWER loses that answer to width, and
//! which consumers read a box that way is not visible from here —
//! `topo` depends on `geom`, not the reverse. So these docs state the
//! containment contract and stop. What looseness costs is the reading
//! door's to state, per door, where the doors are.
//!
//! These land now and are consumed later (the planar boolean consumes
//! only vertex-extent boxes in PR 8); they live HERE, not in the `bvh`
//! crate, so the tree stays below the geometry crates (PR 7's SSI
//! subdivision duty inside them must be able to consume it — see
//! `bvh`'s crate docs) and each constructor sits next to the invariant
//! it cites.
//!
//! This is certified-box driver code, a **sole**-bound [`Bounds`] seam
//! under the 2026-07-29 amendment (geom-core `real.rs`, Bounds scope
//! rule): every scalar enters as its `[lo(), hi()]` bracket, and
//! poison (NaN) flows to the poison box, which overlaps everything —
//! the honest answer when no cheap superset is known.
//!
//! **Not "allowlisted", which is what this said before.** The amendment
//! ratifies the box constructors to write the COMPOUND `Decide + Bounds`
//! form; these write the sole form, which needs no ratification, so
//! `scripts/gates/bounds-allowlist.sh` neither lists this file nor is
//! able to see it — a sole bracket bound is its planted must-not-fire
//! case. The rule covers this module; the gate does not.

use bvh::Aabb;
use geom_core::{Bounds, Point3};

use crate::curves::Curve3;
use crate::curves::nurbs::NurbsCurve3;

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
/// *includes* more extrema, so it errs outward — the sound direction,
/// and not a free one (module docs).
const ANGLE_SLOP: f64 = 1e-6;

/// Whether some 2πk-translate of the angle INTERVAL `[phi_lo, phi_hi]`
/// possibly intersects `[lo, hi]` (already slop-widened).
/// Conservative-inclusive: any NaN answers `true` (poison never
/// excludes), and an interval spanning a full period is always in.
fn angle_interval_in_span(phi_lo: f64, phi_hi: f64, lo: f64, hi: f64) -> bool {
    if phi_lo.is_nan() || phi_hi.is_nan() || lo.is_nan() || hi.is_nan() {
        return true;
    }
    let tau = core::f64::consts::TAU;
    if hi - lo >= tau || phi_hi - phi_lo >= tau {
        return true;
    }
    // Smallest k with phi_hi + τ·k ≥ lo; intersects iff the same
    // translate's lower end clears hi from below.
    let k = ((lo - phi_hi) / tau).ceil();
    let rep_lo = phi_lo + tau * k;
    // NaN-inclusive: a poisoned representative cannot prove exclusion.
    rep_lo.is_nan() || rep_lo <= hi
}

/// The extremal-angle INTERVAL of `atan2(v, u)` over the bracket
/// rectangle `u × v` (fix-pass item 3 — the reviewer's wide-bracket
/// gap): evaluated on the four corners, which carry the angular
/// extremes of a convex region not containing the origin (a
/// supporting ray through the origin touches a polygon at a vertex).
/// `None` means "no bound" — the rectangle possibly contains the
/// origin (amplitude sign unknown) or crosses the atan2 branch cut
/// (the wedge wraps ±π): the caller must include BOTH extrema.
fn extremal_angle_interval(u: Brk, v: Brk) -> Option<(f64, f64)> {
    if u.lo.is_nan() || u.hi.is_nan() || v.lo.is_nan() || v.hi.is_nan() {
        return None; // poison: no exclusion possible
    }
    let u_straddles = u.lo <= 0.0 && u.hi >= 0.0;
    let v_straddles = v.lo <= 0.0 && v.hi >= 0.0;
    if v_straddles && (u_straddles || u.hi <= 0.0) {
        // Origin possibly inside, or the wedge crosses the ±π cut.
        return None;
    }
    let corners = [
        geom_core::Real::atan2(v.lo, u.lo),
        geom_core::Real::atan2(v.lo, u.hi),
        geom_core::Real::atan2(v.hi, u.lo),
        geom_core::Real::atan2(v.hi, u.hi),
    ];
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for c in corners {
        if c.is_nan() {
            return None;
        }
        lo = lo.min(c);
        hi = hi.max(c);
    }
    Some((lo, hi))
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
/// poison bounds, which overlap everything.
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
    // The extremal angle as an INTERVAL over the bracket corners
    // (wide input brackets shift the extremum by up to the bracket's
    // angular width — a midpoint angle would under-cover exactly
    // there); ANGLE_SLOP still absorbs libm error and the membership
    // arithmetic. `None` = no exclusion possible: include both.
    let (include_max, include_min) = match extremal_angle_interval(u, v) {
        None => (true, true),
        Some((p_lo, p_hi)) => (
            angle_interval_in_span(p_lo, p_hi, lo, hi),
            angle_interval_in_span(
                p_lo + core::f64::consts::PI,
                p_hi + core::f64::consts::PI,
                lo,
                hi,
            ),
        ),
    };
    if include_max {
        *max = pfold(*max, c.add(amp).hi, f64::max);
    }
    if include_min {
        *min = pfold(*min, c.sub(amp).lo, f64::min);
    }
}

/// The certified-conservative box of an **ellipse arc** (M5 PR 5): the
/// carrier's `Ellipse` frame over the certified span
/// `[theta0, theta1]`, seeded with the arc's certified endpoints —
/// [`circle_arc_aabb`]'s corner-evaluated extremal-interval shape,
/// generalized.
///
/// Per world axis the coordinate is
/// `cᵢ + major·uᵢ·cos θ + minor·vᵢ·sin θ = cᵢ + Aᵢ·cos(θ − φᵢ)` with
/// amplitude `Aᵢ = √((major·uᵢ)² + (minor·vᵢ)²)` and extremal angle
/// `φᵢ = atan2(minor·vᵢ, major·uᵢ)` — exactly the circle formula with
/// the frame components pre-scaled by the semi-axes. The same
/// wide-bracket rules apply verbatim: the extremal angle is evaluated
/// as an INTERVAL over the scaled-bracket corners, branch-cut wedges
/// (and possibly-origin rectangles) include BOTH extrema, span
/// membership is `ANGLE_SLOP`-widened and conservative-inclusive, and
/// poison flows to the poison box.
///
/// `None` when `carrier` is not an `Ellipse` (wrong lane — refuse
/// loudly rather than guess). Residual padding stays the caller's
/// `Aabb::padded` obligation, as for every certified box.
pub fn ellipse_arc_aabb<T: Bounds>(
    carrier: &Curve3<T>,
    theta0: T,
    theta1: T,
    end0: Point3<T>,
    end1: Point3<T>,
) -> Option<Aabb> {
    let Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = carrier
    else {
        return None;
    };
    // Endpoint hull: always inside the box (2 points — never empty).
    let mut b = Aabb::from_points([end0, end1]).unwrap_or_else(Aabb::poison);

    let a = Brk::of(*major);
    let bm = Brk::of(*minor);
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

    // Pre-scale the frame brackets by the semi-axes; the unit `r`
    // bracket keeps `axis_extremum`'s amplitude arithmetic outward
    // (an extra [1, 1] multiply only ever widens).
    let unit = Brk { lo: 1.0, hi: 1.0 };
    axis_extremum(
        &mut b.min_x,
        &mut b.max_x,
        Brk::of(center.x),
        a.mul(ux),
        bm.mul(vx),
        unit,
        lo,
        hi,
    );
    axis_extremum(
        &mut b.min_y,
        &mut b.max_y,
        Brk::of(center.y),
        a.mul(uy),
        bm.mul(vy),
        unit,
        lo,
        hi,
    );
    axis_extremum(
        &mut b.min_z,
        &mut b.max_z,
        Brk::of(center.z),
        a.mul(uz),
        bm.mul(vz),
        unit,
        lo,
        hi,
    );
    Some(b)
}

/// The certified-conservative box of a NURBS curve: the AABB of its
/// control-point brackets. Sound by the convex-hull property — every
/// curve point is a convex combination of control points because the
/// basis functions are nonnegative and the weights are **strictly
/// positive by construction** (the PR 3 positive-weights invariant,
/// enforced by [`NurbsCurve3::new`]; negative weights would void
/// convexity, Book p. 293). Valid over the whole domain, a fortiori
/// over any certified span (span-tight hulls via knot refinement are a
/// later sharpening; a wider box still contains the locus). No
/// arithmetic — brackets only — so no rounding to pad. The placeholder
/// curve's all-poison control points yield the poison box, which
/// overlaps everything.
pub fn nurbs_curve_aabb<T: Bounds>(curve: &NurbsCurve3<T>) -> Aabb {
    Aabb::from_points(curve.control().iter().copied()).unwrap_or_else(Aabb::poison)
}
