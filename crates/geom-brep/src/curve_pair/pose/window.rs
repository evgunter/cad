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
//!   the middle's foot gap plus the reach, or `√(|H|² + (L·|F|)²)` at most;
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
        let f = bounds(poly, m, w, crate::rounding_charge(*terms));
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
        let gap = self.carrier_b.foot(q, self.foot_seed(q)).gap() + reach;
        let close = (h.hi.powi(2) + d_hi.powi(2)).sqrt();
        let on = |x: T| decide("curve_pair_window_on", Margin::of(x), band) == Ok(Sign::Zero);
        if on(close) || on(gap) {
            return Some(Read::On);
        }
        None
    }

    /// A seed for `q`'s foot on `b`: its angle about `b`'s centre in
    /// `b`'s unit coordinates.
    fn foot_seed(&self, q: Point3<T>) -> T {
        let d = q - self.b.center;
        let (x, y) = (
            d.dot(self.b.u_ref) / self.b.semi_u,
            d.dot(self.b.v_ref()) / self.b.semi_v,
        );
        y.atan2(x)
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
        let noise =
            crate::rounding_charge((ka.center - o).norm() + (kb.center - o).norm() + lever + lever);
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
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::print_stderr)]
mod r3_probes {
    use core::f64::consts::TAU;

    use geom::Curve3;
    use geom_core::{Band, Point3};

    use super::{Joint, Window};
    use crate::Conic;

    pub(super) fn holds_at(
        (a, span_a): (&Curve3<f64>, (f64, f64)),
        b: &Curve3<f64>,
        window: (f64, f64),
        joints: &[(f64, f64)],
        band: Band,
    ) -> bool {
        let (ka, kb) = (Conic::of(a).unwrap(), Conic::of(b).unwrap());
        let lever = ka.speed_hi().max(kb.speed_hi());
        let o = Point3::origin();
        let noise =
            crate::rounding_charge((ka.center - o).norm() + (kb.center - o).norm() + lever + lever);
        let w = Window {
            a: &ka,
            span_a,
            b: &kb,
            carrier_b: b,
            height: super::super::height(&ka, kb.center, kb.axis),
            noise,
        };
        let js: Vec<_> = joints
            .iter()
            .map(|&(ta, _tb)| Joint {
                point: a.eval(ta),
                ta,
            })
            .collect();
        w.holds_only(window, &js, band)
    }

    /// The pose of `a_window_holding_a_second_meeting_is_not_the_joints`,
    /// but `a` read as a whole turn whose seam sits on the plane root: the
    /// second meeting lies across the seam.
    #[test]
    fn probe_window_across_the_seam() {
        let t = 3e-8_f64;
        let a = super::super::super::meet_rows_circle_tilted(t);
        let cb = 0.5 + 1e-4;
        let b = Curve3::Circle {
            center: Point3::new(cb, 0.0, 0.0),
            axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
            radius: 0.5,
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
        };
        let xp = (1.0 + cb * cb - 0.25) / (2.0 * cb);
        let th = ((1.0 - xp * xp).sqrt() / t.cos()).asin();
        let p = a.eval(th);
        let tb = (p.y).atan2(p.x - cb);
        let band = Band::new(1e-9, 1e-8).unwrap();
        let mut rows = Vec::new();
        for (label, span, window) in [
            ("no seam (control)", (-3.0, 3.0), (-0.02, 0.02)),
            ("seam at 0, window about 0", (0.0, TAU), (-0.02, 0.02)),
            ("seam at 0, window about tau", (0.0, TAU), (TAU - 0.02, TAU + 0.02)),
            ("seam at 0, window 0.05", (0.0, TAU), (-0.05, 0.05)),
            ("seam at 0, window 0.1", (0.0, TAU), (-0.1, 0.1)),
            ("seam at 0, window 0.2", (0.0, TAU), (-0.2, 0.2)),
            ("seam at 0, window 0.34", (0.0, TAU), (-0.3398, 0.3398)),
            ("open span, ends 0.002 and -0.003 either side of the root", (0.002, TAU - 0.003), (-0.02, 0.02)),
        ] {
            let got = holds_at((&a, span), &b, window, &[(th, tb)], band);
            eprintln!("{label}: holds_only = {got}");
            rows.push((label, got));
        }
        assert!(
            rows.iter().all(|&(_, g)| !g),
            "a window holding a second meeting across the seam certified: {rows:?}"
        );
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::panic,
    clippy::print_stderr,
    clippy::too_many_lines,
    clippy::many_single_char_names
)]
mod r3_oracle {
    //! Each piece reading checked against a dense independent distance.

    use geom::Curve3;
    use geom_core::{Band, Point3, Vec3};

    use super::{Read, Window, level_on};
    use crate::Conic;

    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> f64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 >> 11) as f64 / (1u64 << 53) as f64
        }
        fn range(&mut self, lo: f64, hi: f64) -> f64 {
            lo + (hi - lo) * self.next()
        }
        fn log(&mut self, lo: f64, hi: f64) -> f64 {
            10f64.powf(self.range(lo, hi))
        }
    }

    fn root(r0: f64, z0: f64, z1: f64, mut g: f64) -> f64 {
        let n0 = r0 * z0;
        let mut s0 = z1 - 1.0;
        let mut s1 = if g < 0.0 { 0.0 } else { n0.hypot(z1) - 1.0 };
        let mut s = 0.0;
        for _ in 0..1100 {
            s = 0.5 * (s0 + s1);
            if s == s0 || s == s1 {
                break;
            }
            let (a, b) = (n0 / (s + r0), z1 / (s + 1.0));
            g = a * a + b * b - 1.0;
            if g > 0.0 {
                s0 = s;
            } else if g < 0.0 {
                s1 = s;
            } else {
                break;
            }
        }
        s
    }

    /// Eberly's distance from `(y0, y1)`, first quadrant, to the ellipse
    /// with semi-axes `e0 ≥ e1`.
    fn ellipse_distance(e0: f64, e1: f64, y0: f64, y1: f64) -> f64 {
        if y1 > 0.0 {
            if y0 > 0.0 {
                let (z0, z1) = (y0 / e0, y1 / e1);
                let g = z0 * z0 + z1 * z1 - 1.0;
                if g == 0.0 {
                    return 0.0;
                }
                let r0 = (e0 / e1).powi(2);
                let sb = root(r0, z0, z1, g);
                let (x0, x1) = (r0 * y0 / (sb + r0), y1 / (sb + 1.0));
                (x0 - y0).hypot(x1 - y1)
            } else {
                (y1 - e1).abs()
            }
        } else {
            let (n0, d0) = (e0 * y0, e0 * e0 - e1 * e1);
            if n0 < d0 {
                let x = n0 / d0;
                (e0 * x - y0).hypot(e1 * (1.0 - x * x).sqrt())
            } else {
                (y0 - e0).abs()
            }
        }
    }

    /// Distance from `p` to the ellipse `(big_a, big_b)` about the origin in
    /// `z = 0`, `u_ref = x̂`.
    fn distance(p: Point3<f64>, (big_a, big_b): (f64, f64)) -> f64 {
        let (x, y) = (p.x.abs(), p.y.abs());
        let d = if big_a >= big_b {
            ellipse_distance(big_a, big_b, x, y)
        } else {
            ellipse_distance(big_b, big_a, y, x)
        };
        d.hypot(p.z)
    }

    fn rot(v: Vec3<f64>, k: Vec3<f64>, t: f64) -> Vec3<f64> {
        let (s, c) = t.sin_cos();
        v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c))
    }

    #[test]
    fn probe_each_reading_holds_on_a_dense_distance() {
        let seed: u64 = std::env::var("R3_SEED")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0x5227_0003);
        let draws: usize = std::env::var("R3_DRAWS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(300);
        eprintln!("seed {seed:#x} draws {draws}");
        let mut rng = Rng(seed);
        let band = Band::new(1e-9, 1e-8).unwrap();
        let z = band.zero();
        let (mut counts, mut bad) = ([0usize; 5], Vec::new());
        for draw in 0..draws {
            let scale = rng.log(-1.0, 2.0);
            let big_a = scale * rng.range(0.5, 2.0);
            let ratio = if rng.next() < 0.3 { 1.0 } else { rng.log(-1.5, 0.0) };
            let big_b = big_a * ratio;
            let b = Curve3::Ellipse {
                center: Point3::origin(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                semi_u: big_a,
                semi_v: big_b,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let phi = rng.range(0.0, core::f64::consts::TAU);
            let j = Point3::new(big_a * phi.cos(), big_b * phi.sin(), 0.0);
            let tb = Vec3::new(-big_a * phi.sin(), big_b * phi.cos(), 0.0).normalize();
            let psi = if rng.next() < 0.3 {
                0.0
            } else {
                rng.log(-7.0, 0.0) * if rng.next() < 0.5 { -1.0 } else { 1.0 }
            };
            let zhat = Vec3::new(0.0, 0.0, 1.0);
            let ta = rot(tb, zhat, psi);
            let r = scale * rng.log(-1.0, 1.0);
            let side = if rng.next() < 0.5 { 1.0 } else { -1.0 };
            let inward = zhat.cross(ta) * side;
            let kang = rng.range(0.0, core::f64::consts::PI);
            let k = Vec3::new(kang.cos(), kang.sin(), 0.0);
            let alpha = rng.log(-8.0, -2.0);
            let lift = rng.range(-1.0, 1.0) * z * 0.9;
            let centre = j + inward * r;
            let a_centre = j + rot(centre - j, k, alpha) + zhat * lift;
            let u = rot((j - centre).normalize(), k, alpha);
            let ax = rot(zhat * side, k, alpha);
            let a = if rng.next() < 0.5 {
                Curve3::Circle {
                    center: a_centre,
                    axis: ax,
                    radius: r,
                    u_ref: u,
                }
            } else {
                // An ellipse through the same point with the same tangent:
                // semi-axis `r` along `u`, the other `r·q`; its tangent at
                // θ = 0 lies along `ax × u`, as the circle's.
                let q = rng.log(-1.0, 1.0);
                Curve3::Ellipse {
                    center: a_centre,
                    axis: ax,
                    semi_u: r,
                    semi_v: r * q,
                    u_ref: u,
                }
            };
            let (ka, kb) = (Conic::of(&a).unwrap(), Conic::of(&b).unwrap());
            let lever = ka.speed_hi().max(kb.speed_hi());
            let o = Point3::origin();
            let noise = crate::rounding_charge(
                (ka.center - o).norm() + (kb.center - o).norm() + lever + lever,
            );
            let w = Window {
                a: &ka,
                span_a: (-1.0, 1.0),
                b: &kb,
                carrier_b: &b,
                height: super::super::height(&ka, kb.center, kb.axis),
                noise,
            };
            let level = level_on(&ka, &kb);
            let iv = |x: f64| geom_core::Interval::from_bounds(x, x);
            let (ia, ib) = (a.map_scalar(iv), b.map_scalar(iv));
            let (ika, ikb) = (Conic::of(&ia).unwrap(), Conic::of(&ib).unwrap());
            let iw = Window {
                a: &ika,
                span_a: (iv(-1.0), iv(1.0)),
                b: &ikb,
                carrier_b: &ib,
                height: super::super::height(&ika, ikb.center, ikb.axis),
                noise: iv(0.0),
            };
            let ilevel = level_on(&ika, &ikb);
            for _ in 0..20 {
                let m = rng.log(-10.0, -1.0) * if rng.next() < 0.5 { -1.0 } else { 1.0 };
                let half = m.abs() * rng.log(-4.0, 0.0);
                let away = if rng.next() < 0.85 { m > 0.0 } else { m < 0.0 };
                let piece = (m - half, m + half);
                let reads = [
                    ("f64", w.read(piece, away, &level, band)),
                    ("interval", iw.read((iv(piece.0), iv(piece.1)), away, &ilevel, band)),
                ];
                for (lane, read) in reads {
                let Some(read) = read else {
                    counts[3] += 1;
                    continue;
                };
                if lane == "interval" { counts[4] += 1; }
                let n = 4000;
                let ds: Vec<f64> = (0..=n)
                    .map(|i| {
                        let t = piece.0 + (piece.1 - piece.0) * (i as f64) / (n as f64);
                        distance(a.eval(t), (big_a, big_b))
                    })
                    .collect();
                let tol = 1e-13 * scale.max(1.0);
                let ok = match read {
                    Read::Off => {
                        counts[0] += 1;
                        ds.iter().all(|&d| d > z - tol)
                    }
                    Read::On => {
                        counts[1] += 1;
                        ds.iter().all(|&d| d <= z + tol)
                    }
                    Read::Recedes => {
                        counts[2] += 1;
                        let seq: Vec<f64> =
                            if away { ds.clone() } else { ds.iter().rev().copied().collect() };
                        let mut hi = f64::NEG_INFINITY;
                        let mut ok = true;
                        for d in seq {
                            if d < hi - tol {
                                ok = false;
                            }
                            hi = hi.max(d);
                        }
                        ok
                    }
                };
                if !ok {
                    bad.push(format!(
                        "draw {draw} {lane}: {read:?} on {piece:?} away {away}: a {a:?} b ({big_a}, {big_b}) d range [{:e}, {:e}]",
                        ds.iter().copied().fold(f64::INFINITY, f64::min),
                        ds.iter().copied().fold(0.0, f64::max)
                    ));
                }
                }
            }
        }
        eprintln!("off {} on {} recedes {} unread {} (interval reads {})", counts[0], counts[1], counts[2], counts[3], counts[4]);
        assert!(bad.is_empty(), "{} readings contradicted: {:#?}", bad.len(), &bad[..bad.len().min(8)]);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::print_stderr)]
mod r3_seam_oracle {
    use core::f64::consts::TAU;

    use geom::Curve3;
    use geom_core::{Band, Point3, Vec3};

    use super::r3_probes::holds_at;

    /// The seam family at random: `a` the unit circle tilted `t` about x̂
    /// (its plane root at θ = 0), `b` a circle in z = 0 through `a(0)`
    /// and through `a(−δ)`'s image, so `J = a(0)` and `P = a(−δ)` are two
    /// meetings in two stretches; `J` given, `P` not. The span is moved
    /// and cut at random. Wherever `P` lies on the span inside the window,
    /// a `true` is a missed meeting.
    #[test]
    fn probe_holds_only_never_certifies_a_second_meeting_at_any_seam() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let mut x: u64 = 0x5227_00a3;
        let mut rnd = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            (x >> 11) as f64 / (1u64 << 53) as f64
        };
        let (mut bad, mut certified, mut with_p) = (Vec::new(), 0, 0);
        for draw in 0..20000 {
            let t = 10f64.powf(-7.5 + rnd() * 1.5);
            let rb = 10f64.powf(-0.3 + rnd() * 0.8);
            let delta = (0.1 + 0.4 * rnd()) * 1e-9 / t * if rnd() < 0.5 { 1.0 } else { -1.0 };
            let (s, c) = t.sin_cos();
            let a = Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::new(0.0, -s, c),
                radius: 1.0,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let j = Point3::new(1.0, 0.0, 0.0);
            let pp = Point3::new(delta.cos(), c * delta.sin(), 0.0);
            let m = Point3::new(0.5 * (j.x + pp.x), 0.5 * (j.y + pp.y), 0.0);
            let chord = pp - j;
            let half = 0.5 * chord.norm();
            let n = Vec3::new(chord.y, -chord.x, 0.0).normalize();
            let inside = n.dot(Point3::origin() - m) > 0.0;
            let n = if inside == (rb < 1.0) { n } else { n * -1.0 };
            let b = Curve3::Circle {
                center: m + n * (rb * rb - half * half).sqrt(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: rb,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let w = (1e-8 / t).asin() * 1.05;
            let k = (rnd() * 41.0).floor() - 20.0;
            let turn = k * TAU;
            // The span: a whole turn cut anywhere, or an open arc ending
            // near the root on either side, or longer than a turn.
            let kind = (rnd() * 7.0).floor();
            let cut = (rnd() - 0.5) * 2.5 * w;
            let span = match kind as i32 {
                0 => (turn + cut, turn + cut + TAU),
                1 => (turn + cut, turn + cut + TAU * (0.5 + 0.49 * rnd())),
                2 => (turn + cut - TAU * (0.5 + 0.49 * rnd()), turn + cut),
                3 => (turn + cut, turn + cut + TAU * (1.0 + 0.5 * rnd())),
                // A hair over a whole turn.
                4 => (turn + cut, turn + cut + TAU + 10f64.powf(-12.0 + 6.0 * rnd())),
                // The span's start exactly at the window's edge, either side.
                5 => {
                    let e = if rnd() < 0.5 { -w } else { w };
                    (turn + e, turn + e + TAU * (0.5 + 0.5 * rnd()))
                }
                // The span's end exactly at the window's edge.
                _ => {
                    let e = if rnd() < 0.5 { -w } else { w };
                    (turn + e - TAU * (0.5 + 0.5 * rnd()), turn + e)
                }
            };
            let window_turn = ((rnd() * 41.0).floor() - 20.0) * TAU;
            let joint_turn = ((rnd() * 41.0).floor() - 20.0) * TAU;
            let window = (window_turn - w, window_turn + w);
            let in_span = |th: f64| {
                (-30..=30).any(|q| {
                    let v = th + f64::from(q) * TAU;
                    v >= span.0 && v <= span.1
                })
            };
            if !in_span(0.0) {
                continue;
            }
            // `P` a second meeting only where the two are parted by a
            // point off the zero band between them.
            let dist = |th: f64| {
                let q = a.eval(th);
                let Curve3::Circle { center, .. } = b else { unreachable!() };
                let r = ((q.x - center.x).powi(2) + (q.y - center.y).powi(2)).sqrt();
                (r - rb).hypot(q.z)
            };
            let parted = (1..400).any(|i| dist(delta * f64::from(i) / 400.0) > 1.2e-9);
            let p_on = in_span(delta) && parted && dist(delta) < 0.9e-9;
            // Each J inside the span is given (both ends of a closed cut).
            let mut joints = Vec::new();
            for q in -30..=30 {
                let v = f64::from(q) * TAU;
                if v >= span.0 && v <= span.1 {
                    joints.push((v + joint_turn - joint_turn, 0.0));
                }
            }
            if rnd() < 0.3 {
                joints = vec![(joint_turn, 0.0)];
            }
            let got = holds_at((&a, span), &b, window, &joints, band);
            if got {
                certified += 1;
                if p_on {
                    bad.push(format!(
                        "draw {draw}: t {t:e} rb {rb} δ {delta:e} span {span:?} window {window:?} joints {joints:?}"
                    ));
                }
            }
            if p_on {
                with_p += 1;
            }
        }
        eprintln!("certified {certified}, draws with P on the span {with_p}, bad {}", bad.len());
        assert!(bad.is_empty(), "{:#?}", &bad[..bad.len().min(6)]);
    }
}
