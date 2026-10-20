//! **A plane root's window beside a found meeting** (the parent
//! module's docs, "A crossing of the other's plane"): whether the arc of a
//! conic `a` where its height `H(θ)` above conic `b`'s plane is within the
//! band's escalation `e` holds no meeting but those already found. Past
//! that window `a` is off `b`. Each found meeting `J` claims its ring, the
//! arc within `e` of it (C5's ring rule). The window, read modulo `a`'s
//! period over the part of it on `a`'s span, is the found meetings' own
//! when each arc of it past the rings, read outward from them, is cut into
//! pieces that each read:
//!
//! - **(i) off** (`curve_pair_window_off`): a lower bound on the distance
//!   `d` to `b`'s carrier past the zero band — the greater of the middle's
//!   [`Curve3::clearance`] less the piece's reach, and the least `|H|`;
//! - **on** (`curve_pair_window_on`): an upper bound inside the zero band —
//!   the middle's foot gap plus the reach, `√(|H|² + (L·|F|)²)` at most,
//!   or the foot gap carried along `b` (below);
//! - **(ii) receding** (`curve_pair_window_recedes`): `d` strictly growing
//!   away from the ring.
//!
//! An arc bounded by no ring must read off throughout.
//!
//! **(ii).** `b` is planar, so `d² = H² + D²`, `D` the distance of `a(θ)`'s
//! image `Q` in `b`'s plane to `b`. With `F` `b`'s level at `Q`
//! (`level_on`), `s ≤ L` `b`'s semi-axes, `q*` a foot of `Q` and
//! `Q − q* = D·ν`: `F` has Hessian `2M`, `M = diag(1/A², 1/B²)`, so
//! `∇F(Q) = ∇F(q*) + 2D·Mν` with `∇F(q*)` along `±ν`, of length in
//! `[2/L, 2/s]`; hence `(D²)′ = 2D·ν·Q′`,
//! `ν·Q′ = sign(F)·(F′ − 2D·Mν·Q′)/|∇F(q*)|`, `|2D·Mν·Q′| ≤ 2D·|Q′|/s²`,
//! `|Q′|` at most `a`'s greatest speed. And `L·|F| ≥ D ≥ min(s/4, |F|·s/4)`:
//! the point of `b` along `Q`'s unit-coordinate direction lies
//! `≤ L·|√(F + 1) − 1| ≤ L·|F|` from `Q`, and the lower bound is
//! [`geom::ellipse_level_clearance`]'s, held to the coplanar ellipse
//! pair's ceiling. A piece recedes (for every foot) where `H` and `H′`
//! keep the sign away from the ring and either `F` and `F′` do too with
//! `|F′| > 2L·|F|·|Q′|/s²`, or `2|H|·|H′|` exceeds the level's drift
//! `D·L·(|F′| + 2D·|Q′|/s²)`; or where `F` and `F′` keep that sign and
//! `D·s·(|F′| − 2L·|F|·|Q′|/s²)` exceeds `2|H|·|H′|`. `H`, `F` and their
//! slopes are bounded over a piece by Taylor's theorem as
//! [`crate::half_angle::certified_subdivision`] bounds its harmonic.
//!
//! **On, along `b`.** Where `a` touches `b`, tangent to it, `d` stays
//! within the zero band `z` over a run about `√(2z/κ)` long, `κ` their
//! relative bend, far past the ring's `e`. The two first-order bounds
//! need pieces of reach under `z` to read it, about `1/√(zκ)` of them,
//! and the level's is floored by its harmonics' rounding at `L` times
//! their charge, so at a fine band no piece reads on and the window runs
//! out of pieces. Carried along `b` instead: with `φ` the middle's foot
//! on `b`'s carrier and `g(δ) = a(m + δ) − b(φ + kδ)` for any slope `k`,
//! `d(a(m + δ)) ≤ |g(δ)|`, and Taylor's theorem gives
//! `|g(δ)| ≤ |g(0)| + |δ|·|g′(0)| + δ²/2·sup|g″|`. A conic's `|C″|` is
//! at most its larger semi-axis (`speed_hi`), so
//! `sup|g″| ≤ A + k²·B` over every `δ`, and `k = a′·b′/|b′|²`, the slope
//! that matches the tangents, leaves `|g′(0)|` the skew between them:
//! near zero at a touch, where one piece then reads the run on.
//!
//! **No second meeting.** A second meeting `P` past a ring would lie
//! beyond a point `Q` that is not on `b`. `Q`'s piece is receding or off,
//! so `d` is past the zero band at its far end, and every piece after it
//! keeps it there: off throughout, receding by growing, and on cannot
//! follow. So the window's points on `b`'s carrier past a ring adjoin that
//! ring through points on it. Where their feet on `b` leave `b`'s span,
//! `b`'s end lies within the band of `a`, so the ends, read first, found
//! it as a meeting with its own ring, or escalated.

#[cfg(test)]
mod oracle;

use geom::Curve3;
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign};

use super::{Conic, level_on};
use crate::half_angle::TrigPoly;

/// The most turns of a window or a ring a span is read over.
const MAX_TURNS: usize = 64;

/// What one window reads: the two conics, `a`'s span, `b`'s carrier,
/// `a`'s height above `b`'s plane as `(c₀, c₁, s₁)` and its rounding.
pub(super) struct Window<'w, T: geom_core::Real> {
    pub(super) a: &'w Conic<T>,
    pub(super) span_a: (T, T),
    pub(super) b: &'w Conic<T>,
    pub(super) carrier_b: &'w Curve3<T>,
    pub(super) height: (T, T, T),
    pub(super) noise: T,
}

/// A found meeting: its point and its parameter on `a`.
#[derive(Clone, Copy)]
pub(super) struct Joint<T: geom_core::Real> {
    pub(super) point: Point3<T>,
    pub(super) ta: T,
}

/// How a piece reads (module docs).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Read {
    Off = 0,
    On = 1,
    Recedes = 2,
}

/// `p` and its first three derivatives at `t`, and the bound on its
/// fourth over the whole turn (`Σ k⁴·A_k`), each widened by its share of
/// `noise` (Bernstein's inequality for degree `N`).
fn jet<T: Decide>(p: &TrigPoly<T>, t: T, noise: T) -> ([T; 4], [T; 4], T) {
    let n = usize::from(p.degree.min(4));
    let mut d = [p.cos[0], T::zero(), T::zero(), T::zero()];
    let mut top = T::zero();
    let mut kf = T::zero();
    for k in 1..=n {
        kf = kf + T::one();
        let (s, c) = (kf * t).sin_cos();
        let (ck, sk) = (p.cos[k], p.sin[k]);
        let (even, odd) = (ck * c + sk * s, sk * c - ck * s);
        d[0] = d[0] + even;
        d[1] = d[1] + kf * odd;
        d[2] = d[2] - kf.powi(2) * even;
        d[3] = d[3] - kf.powi(3) * odd;
        top = top + kf.powi(4) * (ck.powi(2) + sk.powi(2)).sqrt();
    }
    let deg = kf;
    let err = [noise, deg * noise, deg.powi(2) * noise, deg.powi(3) * noise];
    (d, err, top + deg.powi(4) * noise)
}

/// `p` over a piece: lower and upper bounds on `|p|` and on `|p′|`, and
/// `p` and `p′` at the middle.
struct Bounds<T> {
    lo: T,
    hi: T,
    slope_lo: T,
    slope_hi: T,
    at: T,
    slope_at: T,
}

/// [`Bounds`] of `p` over the piece of half-width `w` about `m`: Taylor's
/// theorem about `m` with the turn's top-derivative bound, as
/// [`crate::half_angle::certified_subdivision`] reads its clear and
/// monotone tests. That walk keeps its own arithmetic, in the order its
/// rows and three topo doors were measured on, rather than share this.
fn bounds<T: Decide>(p: &TrigPoly<T>, m: T, w: T, noise: T) -> Bounds<T> {
    let ([v, d1, d2, d3], [e0, e1, e2, e3], top) = jet(p, m, noise);
    let (two, six, tf) = (T::from_f64(2.0), T::from_f64(6.0), T::from_f64(24.0));
    let (a1, a2, a3) = (d1.abs() + e1, d2.abs() + e2, d3.abs() + e3);
    let fall = a1 * w + a2 * w.powi(2) / two + a3 * w.powi(3) / six + top * w.powi(4) / tf;
    let turn = a2 * w + a3 * w.powi(2) / two + top * w.powi(3) / six;
    Bounds {
        lo: v.abs() - e0 - fall,
        hi: v.abs() + e0 + fall,
        slope_lo: d1.abs() - e1 - turn,
        slope_hi: d1.abs() + e1 + turn,
        at: v,
        slope_at: d1,
    }
}

/// `x`, non-negative and computed in `N` roundings to nearest, raised to
/// an upper bound on the real value it stands for. Each rounding is
/// within `u = ε/2` relative, so `x̂ ≥ x·(1 − u)ᴺ`, and the factor
/// `c = 1 + (N + 1)·2u`, a constant exact in `f64` for any `N` short of
/// `2⁵¹`, lifts it past `x` in one more rounding:
/// `x̂·c·(1 − u) ≥ x·(1 − (N + 1)·u)·(1 + 2(N + 1)·u) ≥ x` while
/// `(N + 1)·u ≤ ½`. At an enclosure `c` is a point and the product is
/// outward already, so this only widens it.
fn up<const N: u32, T: Decide>(x: T) -> T {
    x * T::from_f64(1.0 + f64::EPSILON * (N + 1) as f64)
}

/// **The bend term** `(A + k²·B)·w²/2` of the on reading along `b`
/// (module docs), `A` and `B` the conics' larger semi-axes: six
/// roundings (`k²`, `·B`, `+ A`, `w²`, the product, the halving, inexact
/// only below the normal range), raised past them.
fn bent<T: Decide>(a: T, b: T, k: T, w: T) -> T {
    up::<6, _>((a + k.powi(2) * b) * w.powi(2) * T::from_f64(0.5))
}

/// Whether `x` is definitely positive, read exactly
/// ([`super::super::compare`]).
fn positive<T: Decide>(row: &'static str, x: T) -> bool {
    super::super::compare(row, x, T::zero()) == Some(Sign::Positive)
}

/// Whether `x` and `y` are definitely of the sign `away` names
/// (`true` for positive) when multiplied.
fn same_way<T: Decide>(row: &'static str, x: T, y: T, away: bool) -> bool {
    let product = x * y;
    let product = if away { product } else { T::zero() - product };
    positive(row, product)
}

impl<T: Decide> Window<'_, T> {
    /// `a`'s height above `b`'s plane, a first harmonic.
    fn height_poly(&self) -> TrigPoly<T> {
        let (c0, c1, s1) = self.height;
        let z = T::zero();
        TrigPoly {
            cos: [c0, c1, z, z, z],
            sin: [z, s1, z, z, z],
            degree: 1,
        }
    }

    /// **How the piece `(l, r)` reads**, `away` whether `θ` increasing
    /// leaves the found meeting (module docs).
    fn read(
        &self,
        (l, r): (T, T),
        away: bool,
        level: &(TrigPoly<T>, T),
        band: Band,
    ) -> Option<Read> {
        let two = T::from_f64(2.0);
        let (m, w) = ((l + r) / two, (r - l) / two);
        let speed = self.a.speed_hi();
        let reach = speed * w;
        let q = self.a.point(m);
        let lower = self.carrier_b.clearance(q).unwrap_or_else(T::zero) - reach;
        let row = "curve_pair_window_recedes";
        let h = bounds(&self.height_poly(), m, w, self.noise);
        let (poly, terms) = level;
        // The charge covers the f64 harmonics' distance from the true
        // level, below what an f64 reading can resolve, so no row pins it:
        // it rests on `rounding_charge`'s estimate, as the subdivision's
        // does, and an Interval reading carries its own enclosure.
        let f = bounds(poly, m, w, geom_core::rounding_charge(*terms));
        let (big, small) = (
            self.b.semi_u.abs().max(self.b.semi_v.abs()),
            self.b.semi_u.abs().min(self.b.semi_v.abs()),
        );
        let height_recedes = positive(row, h.lo)
            && positive(row, h.slope_lo)
            && same_way(row, h.at, h.slope_at, away);
        // `|(D²)′| ≤ D·L·(|F′| + 2D·|Q′|/s²)`, `D ≤ L·|F|`.
        let d_hi = big * f.hi;
        let level_drift = d_hi * big * (f.slope_hi + two * d_hi * speed / small.powi(2));
        let level_negligible = positive(row, two * h.lo * h.slope_lo - level_drift);
        let level_recedes = positive(row, f.lo)
            && positive(row, f.slope_lo)
            && same_way(row, f.at, f.slope_at, away)
            && positive(row, f.slope_lo - two * big * f.hi * speed / small.powi(2));
        // `(D²)′ ≥ D·s·(|F′| − 2L·|F|·|Q′|/s²)`, `D ≥ min(s/4, |F|·s/4)`.
        let level_way = positive(row, f.lo) && same_way(row, f.at, f.slope_at, away);
        let four = T::from_f64(4.0);
        let d_lo = (small / four).min(f.lo * small / four);
        let level_growth = d_lo * small * (f.slope_lo - two * big * f.hi * speed / small.powi(2));
        let level_dominates = level_way
            && positive(row, f.slope_lo)
            && positive(row, level_growth - two * h.hi * h.slope_hi);
        let z = T::from_f64(band.zero());
        if positive("curve_pair_window_off", h.lo.max(lower) - z) {
            return Some(Read::Off);
        }
        if (height_recedes && (level_recedes || level_negligible)) || level_dominates {
            return Some(Read::Recedes);
        }
        let foot = self.carrier_b.foot(q, self.foot_seed(q));
        let gap = foot.gap() + reach;
        let close = (h.hi.powi(2) + d_hi.powi(2)).sqrt();
        let on = |x: T| decide("curve_pair_window_on", Margin::of(x), band) == Ok(Sign::Zero);
        if on(close) || on(gap) || on(self.along_b(m, w, &foot)) {
            return Some(Read::On);
        }
        None
    }

    /// **An upper bound on `d` over the piece of half-width `w` about `m`,
    /// carried along `b` from the middle's foot** (module docs, "On,
    /// along `b`"). The anchor `|g(0)|` is the foot's gap, read as the
    /// first-order on reading reads it. The rest is f64 arithmetic,
    /// raised past its rounding ([`up`]): the half-width past the
    /// rounding of `m` and `w`, the skew past each velocity's evaluation,
    /// and each term and the sum past their own; an Interval reading
    /// encloses each step and is raised alike.
    fn along_b(&self, m: T, w: T, foot: &geom::Foot<'_, T>) -> T {
        let eps = T::from_f64(f64::EPSILON);
        let a = Curve3::Ellipse {
            center: self.a.center,
            axis: self.a.axis,
            semi_u: self.a.semi_u,
            semi_v: self.a.semi_v,
            u_ref: self.a.u_ref,
        };
        let (_, va) = a.ders1(m);
        let (_, vb) = self.carrier_b.ders1(foot.t());
        let k = va.dot(vb) / vb.dot(vb);
        let (sa, sb) = (self.a.speed_hi(), self.b.speed_hi());
        // `m` and `w` are each within two roundings of `(l ± r)/2`, so
        // the piece lies within `w·(1 + 3ε) + 2ε·|m|` of `m`.
        let reach = up::<1, _>(up::<2, _>(w) + T::from_f64(2.0) * eps * m.abs());
        // Each velocity's coordinates are a handful of roundings of
        // products at most its larger semi-axis.
        let skew = up::<4, _>((va - vb * k).norm() + T::from_f64(16.0) * eps * (sa + k.abs() * sb));
        up::<2, _>(foot.gap() + up::<1, _>(skew * reach) + bent(sa, sb, k, reach))
    }

    /// A seed for `q`'s foot on `b`: its angle about `b`'s centre in
    /// `b`'s unit coordinates.
    fn foot_seed(&self, q: Point3<T>) -> T {
        let d = q - self.b.center;
        let (x, y) = (
            d.dot(self.b.u_ref) / self.b.semi_u,
            d.dot(self.b.v_ref()) / self.b.semi_v,
        );
        super::angle(y, x)
    }

    /// **The pieces of `(l, r)`, each read**, halved until each reads or
    /// one is narrower than the band; `None` past the budget or at a piece
    /// that cannot read.
    fn pieces(
        &self,
        (l, r): (T, T),
        away: bool,
        level: &(TrigPoly<T>, T),
        band: Band,
    ) -> Option<Vec<Read>> {
        let speed = self.a.speed_hi();
        let mut out = Vec::new();
        let mut stack = vec![(l, r)];
        let mut budget = geom_core::bisect::PIECE_BUDGET;
        while let Some((pl, pr)) = stack.pop() {
            budget = budget.checked_sub(1)?;
            if let Some(read) = self.read((pl, pr), away, level, band) {
                out.push(read);
                continue;
            }
            if !positive(
                "curve_pair_window_width",
                speed * (pr - pl) - T::from_f64(band.zero()),
            ) {
                return None;
            }
            let mid = (pl + pr) / T::from_f64(2.0);
            stack.push((mid, pr));
            stack.push((pl, mid));
        }
        Some(out)
    }

    /// **The half-width of `j`'s ring on `a`, in `a`'s parameter**: the arc
    /// about `ta` every point of which lies certainly within the band's
    /// escalation `e` of `j`'s point, `(e − g)/(2·speed)`, `g` the gap from
    /// `a(ta)` to it and `speed` `a`'s greatest. `None` where `g` leaves
    /// no such arc.
    fn ring(&self, j: &Joint<T>, ta: T, band: Band) -> Option<T> {
        let gap = (self.a.point(ta) - j.point).norm();
        let e = T::from_f64(band.escalate());
        let radius = (e - gap) / (T::from_f64(2.0) * self.a.speed_hi());
        positive("curve_pair_window_ring", radius).then_some(radius)
    }

    /// **Whether the window `(lo, hi)` holds no meeting but the found
    /// `joints`** (module docs). `false` where a reading cannot certify it:
    /// the root is then polished as with no meeting found.
    ///
    /// The window is read modulo `a`'s period: each of its turns that
    /// meets `a`'s span is read over that part, so a window across the
    /// span's seam is read on both sides of it, and the part of a window
    /// past an open span's end, which is no point of the span, is not.
    /// The turns are counted from the one nearest the span's start, the
    /// window's centre reduced there, up to the span's end: one or two on a
    /// span of at most a turn, wherever its parameters lie. A span longer
    /// than `MAX_TURNS` turns is not read (`false`).
    pub(super) fn holds_only(&self, (lo, hi): (T, T), joints: &[Joint<T>], band: Band) -> bool {
        let two = T::from_f64(2.0);
        if !positive("curve_pair_window_width", T::pi() / two - (hi - lo) / two) {
            return false;
        }
        let (t0, t1) = self.span_a;
        let level = level_on(self.a, self.b);
        let tau = T::tau();
        // The turns of `t` from the one nearest the span's start up to
        // the span's end, at most `MAX_TURNS`; `None` past that.
        let copies = |t: T, reach: T| -> Option<Vec<T>> {
            let mut at = t0 + (t - t0).reduce_periodic_centred(tau);
            let mut out = Vec::new();
            while positive("curve_pair_window_width", t1 - (at - reach)) {
                if out.len() == MAX_TURNS {
                    return None;
                }
                out.push(at);
                at = at + tau;
            }
            Some(out)
        };
        let half = (hi - lo) / two;
        let Some(centres) = copies((lo + hi) / two, half) else {
            return false;
        };
        let mut anchored = false;
        for centre in centres {
            let (l, h) = ((centre - half).max(t0), (centre + half).min(t1));
            if !positive("curve_pair_window_width", h - l) {
                continue;
            }
            // The found meetings on this part, each with its ring, in order.
            let mut rings: Vec<(T, T)> = Vec::new();
            for j in joints {
                let Some(radius) = self.ring(j, j.ta, band) else {
                    return false;
                };
                let Some(tas) = copies(j.ta, radius) else {
                    return false;
                };
                for ta in tas {
                    if positive("curve_pair_window_ring", ta + radius - l)
                        && positive("curve_pair_window_ring", h - (ta - radius))
                    {
                        let at = rings
                            .iter()
                            .position(|&(rl, _)| before(ta - radius, rl))
                            .unwrap_or(rings.len());
                        rings.insert(at, (ta - radius, ta + radius));
                    }
                }
            }
            anchored |= !rings.is_empty();
            // The arcs between the rings and the part's ends, each read
            // outward from the rings that bound it; one no ring bounds
            // anchors no meeting, and must read off throughout.
            let mut sides: Vec<((T, T), Option<bool>)> = Vec::new();
            let mut from = (l, false);
            for &(rl, rr) in &rings {
                if positive("curve_pair_window_ring", rl - from.0) {
                    sides.extend(outward((from.0, rl), from.1, true));
                }
                from = (from.0.max(rr), true);
            }
            if positive("curve_pair_window_ring", h - from.0) {
                sides.extend(outward((from.0, h), from.1, false));
            }
            for (arc, away) in sides {
                let Some(reads) = self.pieces(arc, away.unwrap_or(true), &level, band) else {
                    return false;
                };
                if away.is_none() && reads.iter().any(|&x| x != Read::Off) {
                    return false;
                }
            }
        }
        anchored
    }
}

/// An arc's sides as read outward from the rings bounding it: from a
/// ring on its left away up `θ` (`Some(true)`), from one on its right
/// away down it (`Some(false)`), from rings on both to its middle, and
/// with none as a whole (`None`).
fn outward<T: Decide>(
    (l, r): (T, T),
    ring_left: bool,
    ring_right: bool,
) -> Vec<((T, T), Option<bool>)> {
    match (ring_left, ring_right) {
        (true, true) => {
            let m = (l + r) / T::from_f64(2.0);
            vec![((l, m), Some(true)), ((m, r), Some(false))]
        }
        (true, false) => vec![((l, r), Some(true))],
        (false, true) => vec![((l, r), Some(false))],
        (false, false) => vec![((l, r), None)],
    }
}

/// Whether `s` definitely precedes `t` ([`super::super::compare`]).
fn before<T: Decide>(s: T, t: T) -> bool {
    super::super::compare("curve_pair_window_ring", s, t) == Some(Sign::Negative)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod rows {
    //! The window's readings, each on a pose only it certifies.

    use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2};

    use geom::Curve3;
    use geom_core::{Band, Point3, Vec3};

    use super::{Joint, Window};
    use crate::Conic;
    use crate::half_angle::TrigPoly;

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// `a`'s window against `b` over `span_a`, its height and rounding
    /// read as the pose reads them.
    fn holds(
        (a, span_a): (&Curve3<f64>, (f64, f64)),
        b: &Curve3<f64>,
        window: (f64, f64),
        joint: f64,
    ) -> bool {
        let (ka, kb) = (Conic::of(a).unwrap(), Conic::of(b).unwrap());
        let lever = ka.speed_hi().max(kb.speed_hi());
        let o = Point3::origin();
        let noise = geom_core::rounding_charge(
            (ka.center - o).norm() + (kb.center - o).norm() + lever + lever,
        );
        let w = Window {
            a: &ka,
            span_a,
            b: &kb,
            carrier_b: b,
            height: super::super::height(&ka, kb.center, kb.axis),
            noise,
        };
        let j = Joint {
            point: a.eval(joint),
            ta: joint,
        };
        w.holds_only(window, &[j], band())
    }

    /// `v` turned by `t` about the unit `k`.
    fn turn(v: Vec3<f64>, k: Vec3<f64>, t: f64) -> Vec3<f64> {
        let (s, c) = t.sin_cos();
        v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c))
    }

    /// **(i) A window read off past the ring.** The pair of `crossing_at`
    /// crossing square: `J` is a meeting. Over the window
    /// `θ_r ± 10⁻⁴` the two part at their right angle, so every point
    /// past the joint's ring is off the first circle. Toward `θ_r` the
    /// height falls, so nothing there recedes: only the off reading
    /// certifies it. Red where the off reading is dropped.
    #[test]
    fn a_window_read_off_past_the_ring_holds_the_joint_alone() {
        let (a, b, root) = crossing_at(FRAC_PI_2);
        assert!(
            holds((&a, (-0.5, 0.5)), &b, (root - 1e-4, root + 1e-4), 0.0),
            "the window off past the ring holds the joint alone"
        );
    }

    /// The unit circle about the origin in `z = 0`, and a unit circle
    /// through `J = (1, 0, 0)` crossing it there at `ψ`, its plane turned
    /// `10⁻⁴` about its radius at `θ_r = 5·10⁻⁶`, so that it crosses the
    /// first's plane there and `J`, at `θ = 0`, lies `5·10⁻¹⁰` above it.
    fn crossing_at(psi: f64) -> (Curve3<f64>, Curve3<f64>, f64) {
        let b = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let u = Vec3::new(psi.cos(), psi.sin(), 0.0);
        let v = Vec3::new(0.0, 0.0, 1.0).cross(u);
        let c = Point3::new(1.0, 0.0, 0.0) - u;
        let root = 5e-6_f64;
        let k = u * root.cos() + v * root.sin();
        let pivot = c + k;
        let tilt = 1e-4;
        let a = Curve3::Circle {
            center: pivot + turn(c - pivot, k, tilt),
            axis: turn(u.cross(v), k, tilt),
            radius: 1.0,
            u_ref: turn(u, k, tilt),
        };
        (a, b, root)
    }

    /// **(ii) A window read receding as the level outruns the height.**
    /// The pair of `crossing_at` crossing at `ψ = 10⁻²`: within the zero
    /// band of each other for about `10⁻⁷` past the joint, so the window
    /// past the ring reads on first, then leaves the zero band. Toward
    /// `θ_r` the height falls as the two part in the plane, so there the
    /// distance is certified growing by the level's growth outrunning the
    /// height's fall. Red where that reading is dropped.
    #[test]
    fn a_window_read_receding_by_the_level_holds_the_joint_alone() {
        let (a, b, root) = crossing_at(1e-2);
        assert!(
            holds((&a, (-0.5, 0.5)), &b, (root - 1e-4, root + 1e-4), 0.0),
            "the window receding by the level holds the joint alone"
        );
    }

    /// **(ii) A window read receding past the ring.** A rod of radius
    /// `5` cut at 45°, its ellipse's quarter ending at `J`, and a
    /// fillet's circle in the plane `y = 0` ending there tangent to it,
    /// both far from the origin (a swept fillet at a hundred times the
    /// unit). The ellipse touches the circle's plane at `J`, so the two
    /// stay within the band over `6·10⁻⁵` rad. At about `2·10⁻⁵` the
    /// height crosses the zero band's edge, where no piece the band
    /// allows reads on or off; there the distance is certified growing,
    /// the height's growth outrunning the level's drift. Red where the
    /// receding reading is dropped.
    #[test]
    fn a_window_read_receding_past_the_ring_holds_the_joint_alone() {
        let x = -178.819_660_112_501_03;
        let h = FRAC_1_SQRT_2;
        let a = Curve3::Ellipse {
            center: Point3::new(x, -5.0, 95.0),
            axis: Vec3::new(h, h, 0.0),
            semi_u: 7.071_067_811_865_475_5,
            semi_v: 5.0,
            u_ref: Vec3::new(-h, h, 0.0),
        };
        let b = Curve3::Circle {
            center: Point3::new(x, 0.0, 95.0),
            axis: Vec3::new(0.0, 1.0, 0.0),
            radius: 5.0,
            u_ref: Vec3::new(0.0, 0.0, 1.0),
        };
        assert!(
            holds((&a, (0.0, FRAC_PI_2)), &b, (-6.3e-5, 6.3e-5), 0.0),
            "the receding window holds the joint alone"
        );
    }

    /// **A window holding a second meeting is not the joint's.** The
    /// unit circle tilted `3·10⁻⁸` about `x̂`, and a circle of radius `0.5`
    /// in `z = 0` about `(0.5001, 0, 0)`, crossing its image at
    /// `y ≈ ±0.0141`, both within `4·10⁻¹⁰` of the first: two meetings in
    /// the window about the plane root at `θ = 0`. With the first found,
    /// the distance past its ring falls back to the second, and where it
    /// re-enters the zero band no piece reads on, off or receding. Red
    /// where a piece that cannot read is passed by, and a window is
    /// certified with a meeting in it
    /// (`plane_rows::two_circles_at_a_shallow_tilt_meet_twice_near_one_plane_root`
    /// reads the same pose through `meet`).
    #[test]
    fn a_window_holding_a_second_meeting_is_not_the_joints() {
        let t = 3e-8_f64;
        let a = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::new(0.0, -t.sin(), t.cos()),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let cb = 0.5 + 1e-4;
        let b = Curve3::Circle {
            center: Point3::new(cb, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let xp = (1.0 + cb * cb - 0.25) / (2.0 * cb);
        let th = -((1.0 - xp * xp).sqrt() / t.cos()).asin();
        assert!(
            !holds((&a, (th, 0.5)), &b, (th, 0.02), th),
            "a window with the second meeting in it is not the joint's"
        );
    }

    /// **A window across `a`'s seam is read on both sides of it.** The
    /// pose of `a_window_holding_a_second_meeting_is_not_the_joints`, the
    /// tilted circle read as a whole turn `(0, 2π)` whose seam is the plane
    /// root, so the second meeting lies across it; and an open span whose
    /// ends lie `0.002` and `0.003` either side of the root. Over windows
    /// from `±0.02` to `±0.34`, about `0` or about `2π`, none is the
    /// joint's. Red where a window is clipped to the span's parameters
    /// rather than read modulo the turn.
    #[test]
    fn a_window_across_the_seam_is_read_on_both_sides() {
        let t = 3e-8_f64;
        let a = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::new(0.0, -t.sin(), t.cos()),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let cb = 0.5 + 1e-4;
        let b = Curve3::Circle {
            center: Point3::new(cb, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let xp = (1.0 + cb * cb - 0.25) / (2.0 * cb);
        let th = ((1.0 - xp * xp).sqrt() / t.cos()).asin();
        let tau = core::f64::consts::TAU;
        for (label, span, window) in [
            ("about 0", (0.0, tau), (-0.02, 0.02)),
            ("about 2π", (0.0, tau), (tau - 0.02, tau + 0.02)),
            ("±0.05", (0.0, tau), (-0.05, 0.05)),
            ("±0.2", (0.0, tau), (-0.2, 0.2)),
            ("±0.34", (0.0, tau), (-0.3398, 0.3398)),
            ("open about the root", (0.002, tau - 0.003), (-0.02, 0.02)),
        ] {
            assert!(
                !holds((&a, span), &b, window, th),
                "{label}: a window with a meeting across the seam is not the joint's"
            );
        }
    }

    /// **A ring lies within the band's escalation of its meeting.** A
    /// circle of radius `2`, its speed its greatest everywhere, and a found
    /// meeting `g` off its point at `θ = 0`, square to it, for `g` from
    /// nothing to `0.9·e`: every point of the ring is nearer the meeting
    /// than `e`, and its ends are past half of what the gap leaves, so the
    /// ring is not trivially small. Red where the ring reaches past `e`.
    #[test]
    fn a_ring_lies_within_the_escalation_of_its_meeting() {
        let band = band();
        let e = band.escalate();
        let a = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 2.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let b = Curve3::Circle {
            center: Point3::new(5.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 1.0, 0.0),
            radius: 3.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (ka, kb) = (Conic::of(&a).unwrap(), Conic::of(&b).unwrap());
        let w = Window {
            a: &ka,
            span_a: (-1.0, 1.0),
            b: &kb,
            carrier_b: &b,
            height: super::super::height(&ka, kb.center, kb.axis),
            noise: 0.0,
        };
        for g in [0.0, 0.5 * e, 0.9 * e] {
            let joint = Joint {
                point: a.eval(0.0) + Vec3::new(0.0, 0.0, g),
                ta: 0.0,
            };
            let radius = w.ring(&joint, 0.0, band).unwrap();
            let far = (0..=1000)
                .map(|i| (a.eval(radius * (f64::from(i) / 500.0 - 1.0)) - joint.point).norm())
                .fold(0.0, f64::max);
            assert!(
                far < e,
                "g = {g:e}: the ring reaches {far:e} from its meeting"
            );
            assert!(
                (a.eval(radius) - a.eval(0.0)).norm() > 0.4 * (e - g),
                "g = {g:e}: the ring's half-width {radius:e} is not trivially small"
            );
        }
    }

    /// **A harmonic's bounds over a piece hold at its extremes.** Three
    /// polynomials of degree two, one of them flat to third order at `0`,
    /// and one of degree one, over pieces of
    /// half-width `10⁻³` to `1.5` about 64 middles round the turn: the
    /// lower and upper bounds on `|p|` and `|p′|` enclose the least and
    /// greatest sampled. And a polynomial read from harmonics rounded by
    /// `0.9` of its charge, at a middle where it is flat: its lower bound
    /// stays at or under the true polynomial's value. Red where the Taylor
    /// remainder drops its fourth-order term, or the rounding goes
    /// uncharged.
    #[test]
    fn a_harmonics_bounds_over_a_piece_hold_at_its_extremes() {
        let poly = |c: [f64; 3], s: [f64; 3], degree: u8| TrigPoly {
            cos: [c[0], c[1], c[2], 0.0, 0.0],
            sin: [0.0, s[1], s[2], 0.0, 0.0],
            degree,
        };
        let at = |p: &TrigPoly<f64>, t: f64| {
            let (mut v, mut d) = (p.cos[0], 0.0);
            for k in 1..=usize::from(p.degree) {
                let kf = k as f64;
                let (sn, cs) = (kf * t).sin_cos();
                v += p.cos[k] * cs + p.sin[k] * sn;
                d += kf * (p.sin[k] * cs - p.cos[k] * sn);
            }
            (v, d)
        };
        for (n, p) in [
            poly([0.3, 1.0, 2.0], [0.0, -0.5, 1.5], 2),
            poly([2.5, 0.2, -1.0], [0.0, 0.7, 0.1], 2),
            poly([0.1, 1.0, 0.0], [0.0, 2.0, 0.0], 1),
            // Flat to third order at `0`: `−3 + t⁴/2`.
            poly([0.0, -4.0, 1.0], [0.0; 3], 2),
        ]
        .iter()
        .enumerate()
        {
            for i in 0..64 {
                let m = core::f64::consts::TAU * f64::from(i) / 64.0;
                for w in [1e-3, 0.1, 0.5, 1.0, 1.5] {
                    let b = super::bounds(p, m, w, 0.0);
                    let (mut lo, mut hi, mut slo, mut shi) =
                        (f64::INFINITY, 0.0_f64, f64::INFINITY, 0.0_f64);
                    for j in 0..=400 {
                        let (v, d) = at(p, m + w * (f64::from(j) / 200.0 - 1.0));
                        (lo, hi) = (lo.min(v.abs()), hi.max(v.abs()));
                        (slo, shi) = (slo.min(d.abs()), shi.max(d.abs()));
                    }
                    let tiny = 1e-12;
                    assert!(
                        b.lo <= lo + tiny && b.hi >= hi - tiny,
                        "poly {n} about {m} ± {w}: |p| in [{lo}, {hi}], bounds [{}, {}]",
                        b.lo,
                        b.hi
                    );
                    assert!(
                        b.slope_lo <= slo + tiny && b.slope_hi >= shi - tiny,
                        "poly {n} about {m} ± {w}: |p′| in [{slo}, {shi}], bounds [{}, {}]",
                        b.slope_lo,
                        b.slope_hi
                    );
                }
            }
        }
        let noise = 1e-9;
        let truth = poly([1.0, 0.5, 0.0], [0.0; 3], 1);
        let rounded = poly([1.0 + 0.9 * noise, 0.5, 0.0], [0.0; 3], 1);
        let b = super::bounds(&rounded, 0.0, 1e-6, noise);
        let (v, _) = at(&truth, 0.0);
        assert!(
            b.lo <= v,
            "the rounded harmonics' lower bound {} under the true value {v}",
            b.lo
        );
    }

    /// The fine band, `ε = 10⁻¹²`, where a touch's run outlasts the
    /// first-order on readings.
    fn fine() -> Band {
        Band::new(1e-12, 1e-11).unwrap()
    }

    /// The rim of a rod of radius `0.1` about `z`, cut by the plane through
    /// `J = (0.1, 0, 0)` that holds `ŷ` and leans `acos(0.1/6.406)` off
    /// `z = 0`: an ellipse of semi-axes `6.406` and `0.1` whose major
    /// vertex is `J`, tangent to `ŷ` there and touching `z = 0` there
    /// alone (the needle's rim).
    fn needle_rim() -> Curve3<f64> {
        let big = 6.406_f64;
        let (c, s) = (0.1 / big, (1.0 - (0.1 / big) * (0.1 / big)).sqrt());
        let u = Vec3::new(c, 0.0, s);
        Curve3::Ellipse {
            center: Point3::new(0.1, 0.0, 0.0) - u * big,
            axis: u.cross(Vec3::new(0.0, 1.0, 0.0)),
            semi_u: big,
            semi_v: 0.1,
            u_ref: u,
        }
    }

    /// A circle of `radius` in `z = 0` through `J = (0.1, 0, 0)`, tangent
    /// to `ŷ` there, its centre on the `x` axis, `J` at its angle `0`.
    fn rim_circle(radius: f64) -> Curve3<f64> {
        Curve3::Circle {
            center: Point3::new(0.1 - radius, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// `a`'s window `(-reach, reach)` about `J` at `θ = 0` against `b` at
    /// the fine band: whether it holds `J` alone, and the pieces the arc
    /// past `J`'s ring reads in, outward.
    fn touch(a: &Curve3<f64>, b: &Curve3<f64>, reach: f64) -> (bool, Option<usize>) {
        let (ka, kb) = (Conic::of(a).unwrap(), Conic::of(b).unwrap());
        let lever = ka.speed_hi().max(kb.speed_hi());
        let o = Point3::origin();
        let noise = geom_core::rounding_charge(
            (ka.center - o).norm() + (kb.center - o).norm() + lever + lever,
        );
        let w = Window {
            a: &ka,
            span_a: (-0.5, 0.5),
            b: &kb,
            carrier_b: b,
            height: super::super::height(&ka, kb.center, kb.axis),
            noise,
        };
        let j = Joint {
            point: a.eval(0.0),
            ta: 0.0,
        };
        let ring = w.ring(&j, 0.0, fine()).unwrap();
        let level = super::level_on(&ka, &kb);
        let past = w
            .pieces((ring, reach), true, &level, fine())
            .map(|r| r.len());
        (w.holds_only((-reach, reach), &[j], fine()), past)
    }

    /// **(on, along `b`) A touch's run reads on in a few pieces.** The
    /// needle's rim on its fillet's circle of radius `0.1`, and on one of
    /// radius `0.2`, tangent inside it at `J`. The rim touches the
    /// circle's plane at `J`, so at `ε = 10⁻¹²` it stays within the zero
    /// band over about `±5·10⁻⁷` rad on the first and `±4·10⁻⁶` on the
    /// second, against a ring of `8·10⁻¹³`. The first-order on reading
    /// needs pieces of reach under `10⁻¹²` across that run, and the
    /// level's is floored by its harmonics' rounding (`L` times their
    /// charge, about `3·10⁻¹²`): main ran out of pieces, and the root went
    /// to the polish against parallel tangents. Carried along the circle
    /// it reads in at most 16. Red where that reading is dropped, and on
    /// the second circle, whose speed at `J` is twice the rim's, where it
    /// follows `b` at the rim's own rate (`k = 1`) rather than the
    /// tangents' match.
    #[test]
    fn a_touchs_run_reads_on_along_the_other_in_a_few_pieces() {
        for (radius, reach) in [(0.1, 2e-6), (0.2, 1e-5)] {
            let (holds, past) = touch(&needle_rim(), &rim_circle(radius), reach);
            assert!(holds, "radius {radius}: the window holds the joint alone");
            assert!(
                past.is_some_and(|n| n <= 16),
                "radius {radius}: the arc past the ring reads in {past:?} pieces"
            );
        }
    }

    /// **A piece past a touch's run does not read on along `b`.** On the
    /// needle's rim against its circle, the piece `θ ∈ [0, 10⁻⁵]` from the
    /// joint: the rim's projection is the circle, so the gap at the
    /// middle's foot and the tangents' skew both read about zero, yet the
    /// far end lies `6.406·sin β·(1 − cos 10⁻⁵) ≈ 3·10⁻¹⁰` above the
    /// circle's plane, past the fine band's `10⁻¹²`. Only the bend term
    /// keeps the piece from reading on. Red where it is dropped.
    #[test]
    fn a_piece_past_a_touchs_run_does_not_read_on_along_the_other() {
        let (a, b) = (needle_rim(), rim_circle(0.1));
        // The rim's projection is the circle: its distance is its height.
        let far = a.eval(1e-5).z.abs();
        assert!(far > 1e-10, "the far end lies {far:e} off");
        let (ka, kb) = (Conic::of(&a).unwrap(), Conic::of(&b).unwrap());
        let w = Window {
            a: &ka,
            span_a: (-0.5, 0.5),
            b: &kb,
            carrier_b: &b,
            height: super::super::height(&ka, kb.center, kb.axis),
            noise: 0.0,
        };
        let foot = b.foot(a.eval(5e-6), 0.0);
        let along = w.along_b(5e-6, 5e-6, &foot);
        assert!(
            along >= far,
            "the reading along b, {along:e}, is under the far end's distance {far:e}"
        );
        let level = super::level_on(&ka, &kb);
        assert_ne!(
            w.read((0.0, 1e-5), true, &level, fine()),
            Some(super::Read::On),
            "the piece reads on"
        );
    }

    /// **The half-width covers a piece one ulp wide far from zero** (the
    /// #5477 review's M1). `b` is the unit circle in `z = 0`; `a` is the
    /// unit circle about `(1, 0, 1)` in `y = 0`, turned so that
    /// `a(400) = (1, 0, 0)`, a transverse crossing with skew about 1.
    /// Over pieces `(l, r)` one ulp of `400` wide, `(l + r)/2` ties and
    /// rounds to an end, so the far end lies a whole ulp (`5.7·10⁻¹⁴`)
    /// from `m` while `w` is half that. The reading along `b` holds both
    /// ends' true distance to `b`'s carrier. Red where the half-width is
    /// left bare (no charge for the rounding of `m` and `w`): it falls
    /// about `2.9·10⁻¹⁴` under on a third of the pieces.
    #[test]
    fn the_half_width_covers_a_piece_one_ulp_wide_far_from_zero() {
        let b = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let a = Curve3::Circle {
            center: Point3::new(1.0, 0.0, 1.0),
            axis: Vec3::new(0.0, -1.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(-400f64.sin(), 0.0, -400f64.cos()),
        };
        let at = a.eval(400.0);
        assert!(
            (at - Point3::new(1.0, 0.0, 0.0)).norm() < 1e-12,
            "a(400) is {at:?}, not the crossing"
        );
        // `b`'s carrier is the unit circle in `z = 0`.
        let d = |t: f64| {
            let q = a.eval(t);
            (q.z.powi(2) + (q.x.hypot(q.y) - 1.0).powi(2)).sqrt()
        };
        let (ka, kb) = (Conic::of(&a).unwrap(), Conic::of(&b).unwrap());
        let w = Window {
            a: &ka,
            span_a: (399.0, 401.0),
            b: &kb,
            carrier_b: &b,
            height: super::super::height(&ka, kb.center, kb.axis),
            noise: 0.0,
        };
        let mut l = 400.0_f64;
        for _ in 0..64 {
            let r = l.next_up();
            let (m, half) = ((l + r) / 2.0, (r - l) / 2.0);
            let foot = b.foot(a.eval(m), 0.0);
            let along = w.along_b(m, half, &foot);
            let far = d(l).max(d(r));
            assert!(
                along >= far,
                "piece ({l}, {r}): the reading along b, {along:e}, is under \
                 the far end's distance {far:e}"
            );
            l = r;
        }
    }

    /// **The bend term is an upper bound on its real value.** Against
    /// `(A + k²·B)·w²/2` read exactly over drawn semi-axes, slopes and
    /// widths, [`super::bent`] is never below it. Red where the term is
    /// left at its rounding to nearest: about half the draws round low.
    #[test]
    fn the_bend_term_bounds_its_real_value() {
        use crate::exact_q::Q;
        let mut rng = test_utils::fuzz::start("window-bend-term");
        for _ in 0..test_utils::fuzz::scaled(20_000) {
            let (a, b) = (rng.range(1e-3, 10.0), rng.range(1e-3, 10.0));
            let k = rng.range(-4.0, 4.0);
            let w = rng.range(0.0, 1.0) * 10f64.powf(rng.range(-14.0, -2.0));
            let exact = Q::of(a)
                .add(&Q::of(k).mul(&Q::of(k)).mul(&Q::of(b)))
                .mul(&Q::of(w).mul(&Q::of(w)))
                .mul(&Q::of(0.5));
            let got = super::bent(a, b, k, w);
            assert!(
                exact.within(-f64::MAX, got),
                "A {a}, B {b}, k {k}, w {w}: {got} under the real value — {}",
                test_utils::fuzz::replay()
            );
        }
    }
}
