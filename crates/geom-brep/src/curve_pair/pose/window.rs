//! **A plane root's window beside a found meeting** (the parent module's
//! docs, "deflation"): whether the arc of the first conic within the
//! band's escalation of the second's plane holds no meeting but the ones
//! already found.
//!
//! A conic `a` crossing or touching the plane of a conic `b` has a first
//! harmonic `H(θ)`, its height above that plane, and a root `θ_r` of it
//! marks a window, the arc about `θ_r` where `|H| ≤ e` (`Range::band_arc`).
//! Past the window `a` lies more than `e` from `b`'s plane, so definitely
//! off `b`. Inside it, a found meeting `J` on `a` claims its ring, the
//! arc within `e` of it (C5: a candidate there cannot be decided distinct
//! from `J`). The window is the found meetings' own when every point of it
//! outside their rings is certified one of:
//!
//! - **(i) off** (`curve_pair_window_off`): a piece whose lower bound on
//!   its distance from `b`'s carrier exceeds the zero band, read exactly:
//!   the greater of the middle's [`Curve3::clearance`] less the piece's
//!   reach (its half-width at `a`'s greatest speed) and the least `|H|`
//!   over the piece (below). No point of it is on `b`, though it may lie
//!   within the band's escalation;
//! - **on** (`curve_pair_window_on`): a piece every point of which is
//!   on `b`'s carrier, read in the zero band: an upper bound on its
//!   distance, the middle's foot gap plus the reach, or
//!   `√(|H|² + (L·|F|)²)` at their most over the piece (below);
//! - **(ii) receding** (`curve_pair_window_recedes`): a piece over which
//!   the distance `d(θ)` from `b`'s carrier strictly grows away from `J`.
//!
//! **Why (ii) holds.** `b` lies in its plane, so `d² = H² + D²`, `D` the
//! distance from `Q(θ)`, `a(θ)`'s image in that plane, to `b`. `|H|` grows
//! away from `J` where `H` and `H′` keep one sign, their product's sign
//! the direction away from `J`. For `D`, let `F(θ) = X²/A² + Y²/B² − 1` be
//! `b`'s level at `Q` (`level_on`), `q*` a foot of `Q` on `b` and `ν` the
//! unit vector from `q*` to `Q`, so `Q − q* = D·ν` and `(D²)′ = 2D·ν·Q′`.
//! `F` is quadratic in the plane, its Hessian `2M`,
//! `M = diag(1/A², 1/B²)`, so `∇F(Q) = ∇F(q*) + 2D·Mν`, and `∇F(q*)` is
//! normal to `b` at `q*`, along `ν` outside (`F > 0`) and against it
//! inside. Hence `ν·Q′ = sign(F)·(F′ − 2D·Mν·Q′)/|∇F(q*)|`, and with
//! `|2D·Mν·Q′| ≤ 2D·|Q′|/s²`, `s` and `L` `b`'s smaller and larger
//! semi-axes: where `F` and `F′` keep one sign, their product's sign the
//! direction away from `J`, and `|F′| > 2D·|Q′|/s²`, `D²` strictly grows
//! away from `J` (for every foot, so the one-sided derivatives agree).
//! `D ≤ L·|F|`: the point of `b` along `Q`'s unit-coordinate direction lies
//! `|ρ − 1|·|Q − c|/ρ ≤ L·|ρ − 1|` from `Q`, `ρ = √(F + 1)`, and
//! `|ρ − 1| = |F|/(ρ + 1) ≤ |F|`. So the test is `|F′| > 2L·|F|·|Q′|/s²`,
//! `|Q′|` at most `a`'s greatest speed. Where `b`'s level is no more than
//! its rounding, `D ≤ L·|F|` is as small, and so is its drift: on `b`,
//! `|∇F| = 2√(X²/A⁴ + Y²/B⁴) ≥ 2/L`, so
//! `|(D²)′| = 2D·|ν·Q′| ≤ D·L·(|F′| + 2D·|Q′|/s²)`, and `d²` still grows
//! where `(H²)′ = 2|H|·|H′|` exceeds that. An on piece's every point lies within
//! `√(|H|² + (L·|F|)²)` of `b`, each at its most over the piece, where
//! that reads in the zero band. Each of `H`, `H′`, `F`, `F′` is
//! bounded over the piece by Taylor's theorem about its middle with the
//! harmonics' exact top-derivative bound, `Σ k⁴·A_k` for `F` and `A` for
//! `H`, each charged its rounding (Bernstein's inequality), and each
//! reading is decided exactly (the least band): these are signs of
//! bounds, not distances the band resolves.
//!
//! **Why the three make the window the found meetings'.** Walk outward
//! from a found meeting's ring, to the window's end or to the middle of
//! the arc before the next ring: its pieces must read on first, then
//! receding or off, never on again. A second meeting `P` there would be
//! a point of `a` on `b` beyond a point `Q` that is not, both past the
//! ring. `Q` lies in no on piece, so every piece from `Q` to `P` is
//! receding or off. `P` in an off piece is not on, and past an off piece
//! `d` only grows from past the zero band; with none, `d` grows from
//! `Q`'s. Either way `P` is not on. So every point of the window on `b`'s
//! carrier past a ring is connected to that ring through points on it;
//! whether two found meetings so connected are one meeting is the
//! stretch reading's to decide (`curve_pair::stretches`).
//!
//! **Those points are on `b`'s span or past its end together**: a point on
//! `b`'s carrier moves its foot along `b` at `a′·b̂′` over a positive
//! factor (the foot is within the zero band, far inside `b`'s curvature
//! radius), and that keeps its sign over the window where the unit
//! tangents at `J`, `â′` and `b̂′`, have `|â′·b̂′|` past the turn each can
//! take: `a`'s curvature bound times the window's arc, and `b`'s times
//! `π/2` of the feet's chord, the window's arc and twice the escalation
//! (an arc of `b` with its curvature times its chord at most one is at
//! most `π/2` of that chord). So the feet move one way from `J`'s, and
//! the points of the window on `b`'s span past the ring, if any, adjoin
//! the ring. Read only where a piece reads on or receding.

use geom::Curve3;
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::{Conic, level_on};
use crate::half_angle::TrigPoly;

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

/// A found meeting: its point and its parameter on each carrier.
#[derive(Clone, Copy)]
pub(super) struct Joint<T: geom_core::Real> {
    pub(super) point: Point3<T>,
    pub(super) ta: T,
    pub(super) tb: T,
}

/// How a piece reads (module docs).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Read {
    Off,
    On,
    Recedes,
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

/// [`Bounds`] of `p` over the piece of half-width `w` about `m`.
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

/// Whether `x` is definitely positive, read exactly.
fn positive<T: Decide>(row: &'static str, x: T) -> bool {
    decide(row, Margin::of(x), Band::least()) == Ok(Sign::Positive)
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
        let f = bounds(poly, m, w, crate::rounding_charge(*terms));
        let (big, small) = (
            self.b.semi_u.abs().max(self.b.semi_v.abs()),
            self.b.semi_u.abs().min(self.b.semi_v.abs()),
        );
        let height_recedes = positive(row, h.lo)
            && positive(row, h.slope_lo)
            && same_way(row, h.at, h.slope_at, away);
        let level_recedes = positive(row, f.lo)
            && positive(row, f.slope_lo)
            && same_way(row, f.at, f.slope_at, away)
            && positive(row, f.slope_lo - two * big * f.hi * speed / small.powi(2));
        // `|(D²)′| ≤ D·L·(|F′| + 2D·|Q′|/s²)`, `D ≤ L·|F|`.
        let d_hi = big * f.hi;
        let level_drift = d_hi * big * (f.slope_hi + two * d_hi * speed / small.powi(2));
        let level_negligible = positive(row, two * h.lo * h.slope_lo - level_drift);
        let z = T::from_f64(band.zero());
        if positive("curve_pair_window_off", h.lo.max(lower) - z) {
            return Some(Read::Off);
        }
        if height_recedes && (level_recedes || level_negligible) {
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

    /// **The pieces of `(l, r)` in order of `θ`, each read**, halved until
    /// each reads or one is narrower than the band; `None` past the budget
    /// or at a piece that cannot read.
    fn pieces(&self, (l, r): (T, T), away: bool, band: Band) -> Option<Vec<Read>> {
        let level = level_on(self.a, self.b);
        let speed = self.a.speed_hi();
        let mut out = Vec::new();
        let mut stack = vec![(l, r)];
        let mut budget = geom_core::bisect::PIECE_BUDGET;
        while let Some((pl, pr)) = stack.pop() {
            budget = budget.checked_sub(1)?;
            if let Some(read) = self.read((pl, pr), away, &level, band) {
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
            // Left first: the stack pops the left half next.
            stack.push((mid, pr));
            stack.push((pl, mid));
        }
        Some(out)
    }

    /// **Whether the window `(lo, hi)` holds no meeting but the found
    /// `joints`** (module docs). `false` where a reading cannot certify it:
    /// the root is then polished as with no meeting found.
    pub(super) fn holds_only(&self, (lo, hi): (T, T), joints: &[Joint<T>], band: Band) -> bool {
        let two = T::from_f64(2.0);
        let (t0, t1) = self.span_a;
        let mid = (t0 + t1) / two;
        let near = |t: T| mid + (t - mid).reduce_periodic_centred(T::tau());
        let centre = near((lo + hi) / two);
        let half = (hi - lo) / two;
        if !positive("curve_pair_window_width", T::pi() / two - half) {
            return false;
        }
        let (lo, hi) = (centre - half, centre + half);
        let clip = |t: T| t.max(t0).min(t1);
        let (lo, hi) = (clip(lo), clip(hi));
        let speed = self.a.speed_hi();
        let e = T::from_f64(band.escalate());
        // Each found meeting in the window, with the half-width of the
        // arc about it a ring certainly holds.
        let mut rings: Vec<(T, T, Joint<T>)> = Vec::new();
        for j in joints {
            let ta = near(j.ta);
            if !(positive("curve_pair_window_ring", ta - lo + e)
                && positive("curve_pair_window_ring", hi - ta + e))
            {
                continue;
            }
            let gap = (self.a.point(ta) - j.point).norm();
            let radius = (e - gap) / (two * speed);
            if !positive("curve_pair_window_ring", radius) {
                return false;
            }
            rings.push((ta - radius, ta + radius, *j));
        }
        if rings.is_empty() {
            return false;
        }
        rings.sort_by(|x, y| {
            if super::super::compare("curve_pair_window_ring", x.0, y.0) == Some(Sign::Negative) {
                core::cmp::Ordering::Less
            } else {
                core::cmp::Ordering::Greater
            }
        });
        // The arcs between the rings, and between them and the window's
        // ends; each knows which of its ends a ring bounds.
        let mut arcs: Vec<((T, T), bool, bool)> = Vec::new();
        let mut from = (lo, false);
        for &(rl, rr, _) in &rings {
            if positive("curve_pair_window_ring", rl - from.0) {
                arcs.push(((from.0, rl), from.1, true));
            }
            from = (from.0.max(rr), true);
        }
        if positive("curve_pair_window_ring", hi - from.0) {
            arcs.push(((from.0, hi), from.1, false));
        }
        // An arc between two rings is read outward from each, to its middle.
        let mut sides: Vec<((T, T), bool)> = Vec::new();
        for ((l, r), ring_left, ring_right) in arcs {
            match (ring_left, ring_right) {
                (true, true) => {
                    let m = (l + r) / two;
                    sides.push(((l, m), true));
                    sides.push(((m, r), false));
                }
                (true, false) => sides.push(((l, r), true)),
                (false, true) => sides.push(((l, r), false)),
                (false, false) => return false,
            }
        }
        let mut near_joint = false;
        for ((l, r), away) in sides {
            let Some(reads) = self.pieces((l, r), away, band) else {
                return false;
            };
            near_joint |= reads.iter().any(|&x| x != Read::Off);
            let ok = if away {
                outward(reads.iter().copied())
            } else {
                outward(reads.iter().rev().copied())
            };
            if !ok {
                return false;
            }
        }
        !near_joint
            || rings
                .iter()
                .all(|&(_, _, j)| self.feet_move_one_way(j, hi - lo, band))
    }

    /// **Whether the feet of the window's points on `b` move one way
    /// along `b` from `j`'s** (module docs): `|â′·b̂′|` at `j` past the
    /// turn each unit tangent can take over the window's arc of
    /// `width`.
    fn feet_move_one_way(&self, j: Joint<T>, width: T, band: Band) -> bool {
        let (Some((_, da)), Some((_, db))) = (tangent(self.a, j.ta), tangent(self.b, j.tb)) else {
            return false;
        };
        let arc = self.a.speed_hi() * width;
        let chord = arc + T::from_f64(band.escalate()) * T::from_f64(2.0);
        let kb = self.b.curvature_hi();
        if !positive("curve_pair_window_feet", T::one() - kb * chord) {
            return false;
        }
        let turn =
            self.a.curvature_hi() * arc + kb * chord * T::from_f64(core::f64::consts::FRAC_PI_2);
        positive("curve_pair_window_feet", da.dot(db).abs() - turn)
    }
}

/// `k`'s point and unit tangent at `t`; `None` where the speed is not
/// definitely positive.
fn tangent<T: Decide>(k: &Conic<T>, t: T) -> Option<(Point3<T>, Vec3<T>)> {
    let (s, c) = t.sin_cos();
    let d = k.v_ref() * (k.semi_v * c) - k.u_ref * (k.semi_u * s);
    let speed = d.norm();
    positive("curve_pair_window_feet", speed).then(|| (k.point(t), d * (T::one() / speed)))
}

/// Whether reads in order outward from a ring are on first, then
/// receding or off, never on again.
fn outward(reads: impl Iterator<Item = Read>) -> bool {
    let mut left_on = false;
    for read in reads {
        match read {
            Read::On if left_on => return false,
            Read::On => {}
            Read::Recedes | Read::Off => left_on = true,
        }
    }
    true
}
