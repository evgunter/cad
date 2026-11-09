//! **A plane root's window** (the parent module's docs, "A crossing of
//! the other's plane"): whether the arc of a conic `a` where its height
//! `H(θ)` above conic `b`'s plane is within the band's escalation `e`
//! holds no meeting but those already found, none included. Past
//! that window `a` is off `b`. Each found meeting `J` claims its ring, the
//! arc within `e` of it (C5's ring rule). The window, read modulo `a`'s
//! period over the part of it on `a`'s span, is the found meetings' own
//! when each arc of it past the rings, read outward from them, is cut into
//! pieces that each read:
//!
//! - **(i) off** (`curve_pair_window_off`): a lower bound on the distance
//!   `d` to `b`'s carrier past the zero band — the greater of the middle's
//!   [`geom::Curve3::clearance`] less the piece's reach, and the least `|H|`;
//! - **on** (`curve_pair_window_on`): an upper bound inside the zero band —
//!   the middle's foot gap plus the reach, `√(|H|² + (L·|F|)²)` at most,
//!   or the foot gap carried along `b` (below);
//! - **(ii) receding** (`curve_pair_window_recedes`): `d` strictly growing
//!   away from the ring.
//!
//! An arc bounded by no ring must read off throughout, past the band's
//! escalation rather than its zero: it answers to no found meeting, so a
//! point within the gap there is a near-miss the band leaves undecided,
//! and the root goes to the polish, which escalates on it (the parent
//! module's docs). **Off there is off `b`'s span**, not only its carrier:
//! where `b`'s carrier runs on past its span beside `a`, a piece whose
//! carrier readings fail is read against the span by its carrier's point
//! walk (`curve_pair_window_off_span`), and one that does not is halved.
//! Each piece of such an arc stands alone, so the span's reading needs
//! nothing of its neighbours.
//!
//! **An arc a ring bounds** is read outward in a chain whose receding
//! link is the carrier's distance growing ("No second meeting", below).
//! A piece read off `b`'s span ends that chain: no point of it is within
//! the zero of the span, but its far end may lie within the zero of the
//! carrier, so a piece past it recedes only from a piece read off the
//! carrier, and any other piece past it must itself read off the span,
//! halved until it does (`Window::chained`).
//!
//! **(ii).** `b` is planar, so `d² = H² + D²`, `D` the distance of `a(θ)`'s
//! image `Q` in `b`'s plane to `b`. With `F` `b`'s level at `Q`
//! ([`Level`]), `s ≤ L` `b`'s semi-axes, `q*` a foot of `Q` and
//! `Q − q* = D·ν`: `F` has Hessian `2M`, `M = diag(1/A², 1/B²)`, so
//! `∇F(Q) = ∇F(q*) + 2D·Mν` with `∇F(q*)` along `±ν`, of length in
//! `[2/L, 2/s]`; hence `(D²)′ = 2D·ν·Q′`,
//! `ν·Q′ = sign(F)·(F′ − 2D·Mν·Q′)/|∇F(q*)|`, `|2D·Mν·Q′| ≤ 2D·|Q′|/s²`,
//! `|Q′|` at most `a`'s greatest speed. And `L·|F| ≥ D ≥ min(s/4, |F|·s/4)`:
//! the point of `b` along `Q`'s unit-coordinate direction lies
//! `≤ L·|√(F + 1) − 1| ≤ L·|F|` from `Q`, and the lower bound is
//! [`geom::ellipse_level_clearance`]'s, held to the coplanar ellipse
//! pair's ceiling. **Read locally**, both are loose by the aspect ratio,
//! and tighter: `F` is quadratic, so `F(Q) = ∇F(q*)·Dν + D²·νᵀMν` and
//! `|∇F(q*)| ≤ |∇F(Q)| + 2D/s²`, whence `|F| ≤ g·D + 3D²/s²` and, on `D`'s
//! smaller branch, `|F| ≥ g·D − 3D²/s²`, `g` bounding `|∇F(Q)|` over the
//! piece (it moves `2/s²` per metre of `Q`, and `Q` by the piece's reach):
//! `D ≥ 2|F|/(g + √(g² + 12|F|/s²))`, and `D ≤ 2|F|/(g + √(g² − 12|F|/s²))`
//! where `L·|F|` lies under the larger branch `(g + √(g² − 12|F|/s²))·s²/6`.
//! The larger lower bound and the smaller upper bound are read. A piece
//! recedes (for every foot) where `H` and `H′` keep the sign away from the
//! ring and either `F` and `F′` do too with `|F′| > 2D·|Q′|/s²`, or
//! `2|H|·|H′|` exceeds the level's drift `D·L·(|F′| + 2D·|Q′|/s²)`; or
//! where `F` and `F′` keep that sign and
//! `2D·(|F′| − 2D·|Q′|/s²)/|∇F(q*)|` exceeds `2|H|·|H′|`, `|∇F(q*)|` at
//! most the lesser of `2/s` and `g + 2D/s²`. `H`, `F` and their slopes
//! are bounded over a piece by Taylor's theorem about its middle, as
//! [`crate::half_angle::certified_subdivision`] bounds its harmonic, from
//! their jets read locally there ([`Level`] for `F`, the same for `H`),
//! uncharged at `f64`, with fourth-derivative bounds that need no
//! precision. Posed about `b`'s centre their harmonics carry rounding of
//! order `((|w| + R)/s)²` for `F` and the carriers' distance from the
//! origin for `H`, which at a far, thin or large pair reaches the band's
//! zero; read about the piece, each rounds as its few local terms do.

//! **On, along `b`.** Where `a` touches `b`, tangent to it, `d` stays
//! within the zero band `z` over a run about `√(2z/κ)` long, `κ` their
//! relative bend, far past the ring's `e`. The two first-order bounds
//! need pieces of reach under `z` to read it, about `1/√(zκ)` of them,
//! so at a fine band no piece reads on and the window runs out of
//! pieces. Carried along `b` instead: with `φ` the middle's foot
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

use geom::PieceStep;
use geom_core::bisect::Step;
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign};

use super::super::span::Span;
use super::{Conic, Level, level_on, up};

/// The most turns of a window or a ring a span is read over.
const MAX_TURNS: usize = 64;

/// What one window reads: the two conics, `a`'s span, `b`'s span,
/// and `a`'s height above `b`'s plane as `(c₀, c₁, s₁)`.
pub(super) struct Window<'w, T: geom_core::Real> {
    pub(super) a: &'w dyn Conic<T>,
    pub(super) span_a: (T, T),
    pub(super) b: &'w dyn Conic<T>,
    pub(super) span_b: &'w Span<'w, T>,
    pub(super) height: (T, T, T),
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
    /// Off `b`'s span, not its carrier (module docs).
    OffSpan = 3,
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

/// [`Bounds`] over the piece of half-width `w` about a middle where the
/// value and its first three derivatives are `d`, each within `err`, and
/// the fourth is at most `top` throughout.
fn taylor<T: Decide>([v, d1, d2, d3]: [T; 4], [e0, e1, e2, e3]: [T; 4], top: T, w: T) -> Bounds<T> {
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
    /// `a`'s height above `b`'s plane and its first three derivatives at
    /// `θ`, read locally as the level is ([`Level`]): on a conic
    /// `P″ = −(P − c)` and `P‴ = −P′`.
    fn height_jet(&self, theta: T) -> [T; 4] {
        let (n, centre) = (self.b.frame().axis, self.b.frame().origin);
        let (p, dp) = self.a.jet_at(theta);
        let slope = dp.dot(n);
        let bend = (self.a.frame().origin - p).dot(n);
        [(p - centre).dot(n), slope, bend, T::zero() - slope]
    }

    /// A bound on the height's fourth derivative, `(P − c)·n̂`, at every
    /// angle: its swing `√(c₁² + s₁²)`, raised past its four roundings and
    /// the twelve of `c₁` and `s₁` (`k.along`).
    fn height_fourth(&self) -> T {
        let (_, c1, s1) = self.height;
        up::<16, _>((c1.powi(2) + s1.powi(2)).sqrt())
    }

    /// **How the piece `(l, r)` reads**, `away` whether `θ` increasing
    /// leaves the found meeting, or `None` on an arc no ring bounds, where
    /// off reads past the band's escalation rather than its zero (module
    /// docs).
    fn read(
        &self,
        (l, r): (T, T),
        away: Option<bool>,
        level: &Level<'_, T>,
        band: Band,
    ) -> Option<Read> {
        let two = T::from_f64(2.0);
        let (m, w) = ((l + r) / two, (r - l) / two);
        let speed = self.a.speed_hi();
        let reach = speed * w;
        let q = self.a.point_at(m);
        let lower = self.span_b.carrier().clearance(q).unwrap_or_else(T::zero) - reach;
        let row = "curve_pair_window_recedes";
        // The height and the level are read locally at the middle,
        // uncharged: their few terms round far below the band, and an
        // Interval reading encloses them.
        let z0 = T::zero();
        let h = taylor(self.height_jet(m), [z0; 4], self.height_fourth(), w);
        let f = taylor(level.jet_at(m), [z0; 4], level.fourth(), w);
        let (big, small) = (self.b.speed_hi(), self.b.speed_lo());
        let height_recedes = positive(row, h.lo)
            && positive(row, h.slope_lo)
            && same_way(row, h.at, h.slope_at, away.unwrap_or(true));
        // `D` from the level, read locally (module docs, (ii)).
        let s2 = small.powi(2);
        let (six, twelve) = (T::from_f64(6.0), T::from_f64(12.0));
        let grad = level.gradient_at(m);
        let (g_lo, g_hi) = (grad - two * reach / s2, grad + two * reach / s2);
        let f_lo = f.lo.max(T::zero());
        let four = T::from_f64(4.0);
        let d_lo = (two * f_lo / (g_hi + (g_hi.powi(2) + twelve * f_lo / s2).sqrt()))
            .max((small / four).min(f_lo * small / four));
        let disc = g_lo.powi(2) - twelve * f.hi / s2;
        let d_hi = if positive(row, g_lo)
            && positive(row, disc)
            && positive(row, (g_lo + disc.sqrt()) * s2 / six - big * f.hi)
        {
            (two * f.hi / (g_lo + disc.sqrt())).min(big * f.hi)
        } else {
            big * f.hi
        };
        // `|(D²)′| ≤ D·L·(|F′| + 2D·|Q′|/s²)`.
        let level_drift = d_hi * big * (f.slope_hi + two * d_hi * speed / s2);
        let level_negligible = positive(row, two * h.lo * h.slope_lo - level_drift);
        let level_recedes = positive(row, f.lo)
            && positive(row, f.slope_lo)
            && same_way(row, f.at, f.slope_at, away.unwrap_or(true))
            && positive(row, f.slope_lo - two * d_hi * speed / s2);
        // `(D²)′ ≥ 2D·(|F′| − 2D·|Q′|/s²)/|∇F(q*)|`.
        let level_way =
            positive(row, f.lo) && same_way(row, f.at, f.slope_at, away.unwrap_or(true));
        let foot_grad = (two / small).min(g_hi + two * d_hi / s2);
        let level_growth = two * d_lo * (f.slope_lo - two * d_hi * speed / s2) / foot_grad;
        let level_dominates = level_way
            && positive(row, f.slope_lo)
            && positive(row, level_growth - two * h.hi * h.slope_hi);
        let z = match away {
            Some(_) => T::from_f64(band.zero()),
            None => T::from_f64(band.escalate()),
        };
        if positive("curve_pair_window_off", h.lo.max(lower) - z) {
            return Some(Read::Off);
        }
        if away.is_none() {
            // Only off certifies an arc no ring bounds: a piece that does
            // not read off is halved.
            return self.off_span(q, reach, z, band).then_some(Read::Off);
        }
        if (height_recedes && (level_recedes || level_negligible)) || level_dominates {
            return Some(Read::Recedes);
        }
        let foot = self.span_b.carrier().foot(q, self.foot_seed(q));
        let gap = foot.gap() + reach;
        let close = (h.hi.powi(2) + d_hi.powi(2)).sqrt();
        let on = |x: T| decide("curve_pair_window_on", Margin::of(x), band) == Ok(Sign::Zero);
        if on(close) || on(gap) || on(self.along_b(m, w, &foot)) {
            return Some(Read::On);
        }
        self.off_span(q, reach, z, band).then_some(Read::OffSpan)
    }

    /// **(i) off `b`'s span**, on an arc no ring bounds (module docs):
    /// whether every point within `reach` of `q` lies past `z` from `b`'s
    /// span, by its carrier's point walk ([`geom::Curve3::point_walk`]):
    /// each piece of the span settles where its middle's distance from
    /// `q`, less its own reach and `reach`, reads past `z`. `false` once a
    /// piece's reach is within `z`, or past the walk's budget.
    fn off_span(&self, q: Point3<T>, reach: T, z: T, band: Band) -> bool {
        let row = "curve_pair_window_off_span";
        let span = self.span_b;
        let walked = span
            .carrier()
            .point_walk(q, span.params(), span.speeds().1, |p| {
                if positive(row, p.gap - p.reach - reach - z) {
                    return PieceStep::Settled;
                }
                if !positive(row, p.reach - T::from_f64(band.zero())) {
                    return PieceStep::Stop(());
                }
                PieceStep::Halve
            });
        matches!(walked, Ok(None))
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
        let (_, va) = self.a.jet_at(m);
        let (_, vb) = self.span_b.carrier().ders1(foot.seed());
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
        let (x, y) = self.b.unit_in_plane(q);
        geom::Azimuth::of(y, x).principal_as(super::row::BRANCH, Band::least())
    }

    /// **Whether a ringed arc's pieces, in order up `θ`, hold no meeting
    /// past the ring** (module docs), read outward from it (`away` up
    /// `θ`, or down it). Up to the first piece read off `b`'s span, the
    /// carrier's chain holds them. Past it, the chain restarts at each
    /// piece read off the carrier: a piece recedes only from one, and any
    /// other piece must itself read off the span.
    fn chained(&self, reads: &[((T, T), Read)], away: bool, band: Band) -> bool {
        let outward: Vec<_> = if away {
            reads.iter().collect()
        } else {
            reads.iter().rev().collect()
        };
        let Some(first) = outward.iter().position(|r| r.1 == Read::OffSpan) else {
            return true;
        };
        let mut from_off = false;
        for &&((l, r), read) in &outward[first..] {
            from_off = match read {
                Read::Off => true,
                Read::Recedes if from_off => true,
                Read::OffSpan => false,
                Read::On | Read::Recedes => {
                    if !self.piece_off_span((l, r), band) {
                        return false;
                    }
                    false
                }
            };
        }
        true
    }

    /// Whether the piece `(l, r)` reads off `b`'s span past the band's
    /// zero, halved until each part does or one is narrower than the band.
    fn piece_off_span(&self, (l, r): (T, T), band: Band) -> bool {
        let (two, z) = (T::from_f64(2.0), T::from_f64(band.zero()));
        let speed = self.a.speed_hi();
        let walked = geom_core::bisect::bisect([(l, r)], |(pl, pr)| {
            let mid = (pl + pr) / two;
            if self.off_span(self.a.point_at(mid), speed * (pr - pl) / two, z, band) {
                return Step::Settled;
            }
            if !positive("curve_pair_window_width", speed * (pr - pl) - z) {
                return Step::Stop(());
            }
            Step::Halve([(pl, mid), (mid, pr)])
        });
        matches!(walked, Ok(None))
    }

    /// **The pieces of `(l, r)`, each read**, halved until each reads or
    /// one is narrower than the band; `None` past the budget or at a piece
    /// that cannot read.
    fn pieces(
        &self,
        (l, r): (T, T),
        away: Option<bool>,
        level: &Level<'_, T>,
        band: Band,
    ) -> Option<Vec<((T, T), Read)>> {
        let speed = self.a.speed_hi();
        let mut out = Vec::new();
        let walked = geom_core::bisect::bisect([(l, r)], |(pl, pr)| {
            if let Some(read) = self.read((pl, pr), away, level, band) {
                out.push(((pl, pr), read));
                return Step::Settled;
            }
            if !positive(
                "curve_pair_window_width",
                speed * (pr - pl) - T::from_f64(band.zero()),
            ) {
                return Step::Stop(());
            }
            let mid = (pl + pr) / T::from_f64(2.0);
            Step::Halve([(pl, mid), (mid, pr)])
        });
        matches!(walked, Ok(None)).then_some(out)
    }

    /// **The half-width of `j`'s ring on `a`, in `a`'s parameter**: the arc
    /// about `ta` every point of which lies certainly within the band's
    /// escalation `e` of `j`'s point, `(e − g)/(2·speed)`, `g` the gap from
    /// `a(ta)` to it and `speed` `a`'s greatest. `None` where `g` leaves
    /// no such arc.
    fn ring(&self, j: &Joint<T>, ta: T, band: Band) -> Option<T> {
        let gap = (self.a.point_at(ta) - j.point).norm();
        let e = T::from_f64(band.escalate());
        let radius = (e - gap) / (T::from_f64(2.0) * self.a.speed_hi());
        positive("curve_pair_window_ring", radius).then_some(radius)
    }

    /// **Whether the window `(lo, hi)` holds no meeting but the found
    /// `joints`**, none among them included (module docs). `false` where a
    /// reading cannot certify it: the root is then polished as with no
    /// meeting found.
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
                let Some(reads) = self.pieces(arc, away, &level, band) else {
                    return false;
                };
                let holds = match away {
                    None => reads.iter().all(|&(_, x)| x == Read::Off),
                    Some(up) => self.chained(&reads, up, band),
                };
                if !holds {
                    return false;
                }
            }
        }
        true
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
        let (ka, kb) = (a.conic().unwrap(), b.conic().unwrap());
        let w = Window {
            a: &ka,
            span_a,
            b: &kb,
            span_b: &crate::curve_pair::Span::new(
                b,
                (-core::f64::consts::PI, core::f64::consts::PI),
            ),
            height: super::super::height(&ka, kb.frame().origin, kb.frame().axis),
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
        let b = Curve3::circle(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let u = Vec3::new(psi.cos(), psi.sin(), 0.0);
        let v = Vec3::new(0.0, 0.0, 1.0).cross(u);
        let c = Point3::new(1.0, 0.0, 0.0) - u;
        let root = 5e-6_f64;
        let k = u * root.cos() + v * root.sin();
        let pivot = c + k;
        let tilt = 1e-4;
        let a = Curve3::circle(
            pivot + turn(c - pivot, k, tilt),
            turn(u.cross(v), k, tilt),
            1.0,
            turn(u, k, tilt),
        );
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
        let a = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(
                Point3::new(x, -5.0, 95.0),
                Vec3::new(h, h, 0.0),
                Vec3::new(-h, h, 0.0),
            ),
            7.071_067_811_865_475_5,
            5.0,
        ));
        let b = Curve3::circle(
            Point3::new(x, 0.0, 95.0),
            Vec3::new(0.0, 1.0, 0.0),
            5.0,
            Vec3::new(0.0, 0.0, 1.0),
        );
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
        let a = Curve3::circle(
            Point3::origin(),
            Vec3::new(0.0, -t.sin(), t.cos()),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let cb = 0.5 + 1e-4;
        let b = Curve3::circle(
            Point3::new(cb, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            0.5,
            Vec3::new(1.0, 0.0, 0.0),
        );
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
        let a = Curve3::circle(
            Point3::origin(),
            Vec3::new(0.0, -t.sin(), t.cos()),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let cb = 0.5 + 1e-4;
        let b = Curve3::circle(
            Point3::new(cb, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            0.5,
            Vec3::new(1.0, 0.0, 0.0),
        );
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

    /// **An arc no ring bounds, in a window that holds a ring, reads off
    /// only past the escalation.** The unit circle's window `±2·10⁻⁶`
    /// about its span's seam. A circle in the plane of the chord between
    /// `θ = ±10⁻⁶` meets it at `10⁻⁶`, the found meeting, whose ring bounds
    /// the part past the seam's start; near `−10⁻⁶`, in the part before the
    /// seam's end, which no ring bounds, it passes the first at the least
    /// gap the closed-form distance to it reads, sampled densely. Within the zero
    /// band the window is not the joint's (a second meeting), and past the
    /// escalation it is. In the band's gap the arc's pieces all read off
    /// past the zero, as a ring-bounded arc's would, so the window read the
    /// pair apart there; now it does not certify, and the root goes to the
    /// polish. Red where an arc no ring bounds reads off past the zero.
    #[test]
    fn a_free_arc_in_the_gap_beside_a_ring_is_not_the_joints() {
        let (tau, w) = (core::f64::consts::TAU, 2e-6_f64);
        let (z, e) = (band().zero(), band().escalate());
        let (c, s) = ((w / 2.0).cos(), (w / 2.0).sin());
        let a = Curve3::circle(Point3::origin(), Vec3::unit_z(), 1.0, Vec3::unit_x());
        let mut gap_seen = 0;
        let mut bad = Vec::new();
        for pass in [5e-10, 8e-9, 3e-8, 1e-7] {
            let r = 2.0 * s * s / pass;
            let centre = Point3::new(c, s, r);
            let b = Curve3::circle(centre, Vec3::unit_x(), r, Vec3::new(0.0, 0.0, -1.0));
            let gap = (0..=20_000)
                .map(|i| {
                    let q = a.eval(-w + w * f64::from(i) / 20_000.0) - centre;
                    let h = q.x;
                    let rho = (q - Vec3::unit_x() * h).norm();
                    (h * h + (rho - r).powi(2)).sqrt()
                })
                .fold(f64::INFINITY, f64::min);
            let got = holds((&a, (0.0, tau)), &b, (-w, w), w / 2.0);
            let label = format!("least gap {gap:e}");
            if gap <= z || gap > e {
                if got != (gap > e) {
                    bad.push(format!("{label}: holds {got}"));
                }
                continue;
            }
            gap_seen += 1;
            let (ka, kb) = (a.conic().unwrap(), b.conic().unwrap());
            let window = Window {
                a: &ka,
                span_a: (0.0, tau),
                b: &kb,
                span_b: &crate::curve_pair::Span::new(
                    &b,
                    (-core::f64::consts::PI, core::f64::consts::PI),
                ),
                height: super::super::height(&ka, kb.frame().origin, kb.frame().axis),
            };
            let level = super::super::level_on(&ka, &kb);
            let past_zero = window
                .pieces((tau - w, tau), Some(true), &level, band())
                .is_some_and(|reads| reads.iter().all(|&(_, x)| x == super::Read::Off));
            if !past_zero || got {
                bad.push(format!(
                    "{label}: off past the zero {past_zero}, holds {got}"
                ));
            }
        }
        assert_eq!(gap_seen, 2, "two passes in the band's gap");
        assert!(bad.is_empty(), "{}", bad.join("\n"));
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
        let a = Curve3::circle(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            2.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let b = Curve3::circle(
            Point3::new(5.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            3.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let (ka, kb) = (a.conic().unwrap(), b.conic().unwrap());
        let w = Window {
            a: &ka,
            span_a: (-1.0, 1.0),
            b: &kb,
            span_b: &crate::curve_pair::Span::new(
                &b,
                (-core::f64::consts::PI, core::f64::consts::PI),
            ),
            height: super::super::height(&ka, kb.frame().origin, kb.frame().axis),
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

    /// `p`'s jet at `m` and `Σ k⁴·|c_k|`, read over `±w` by
    /// [`super::taylor`].
    fn bounds(p: &TrigPoly<f64>, m: f64, w: f64) -> super::Bounds<f64> {
        let mut d = [p.cos[0], 0.0, 0.0, 0.0];
        let mut top = 0.0;
        for k in 1..=usize::from(p.degree) {
            let kf = k as f64;
            let (sn, cs) = (kf * m).sin_cos();
            let (even, odd) = (p.cos[k] * cs + p.sin[k] * sn, p.sin[k] * cs - p.cos[k] * sn);
            d[0] += even;
            d[1] += kf * odd;
            d[2] -= kf.powi(2) * even;
            d[3] -= kf.powi(3) * odd;
            top += kf.powi(4) * p.cos[k].hypot(p.sin[k]);
        }
        super::taylor(d, [0.0; 4], top, w)
    }

    /// **A jet's bounds over a piece hold at its extremes.** Three
    /// polynomials of degree two, one of them flat to third order at `0`,
    /// and one of degree one, over pieces of half-width `10⁻³` to `1.5`
    /// about 64 middles round the turn, read by [`super::taylor`] from
    /// their jet and `Σ k⁴·|c_k|`: the lower and upper bounds on `|p|`
    /// and `|p′|` enclose the least and greatest sampled. Red where the
    /// Taylor remainder drops its fourth-order term.
    #[test]
    fn a_jets_bounds_over_a_piece_hold_at_its_extremes() {
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
                    let b = bounds(p, m, w);
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
        Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(
                Point3::new(0.1, 0.0, 0.0) - u * big,
                u.cross(Vec3::new(0.0, 1.0, 0.0)),
                u,
            ),
            big,
            0.1,
        ))
    }

    /// A circle of `radius` in `z = 0` through `J = (0.1, 0, 0)`, tangent
    /// to `ŷ` there, its centre on the `x` axis, `J` at its angle `0`.
    fn rim_circle(radius: f64) -> Curve3<f64> {
        Curve3::circle(
            Point3::new(0.1 - radius, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            radius,
            Vec3::new(1.0, 0.0, 0.0),
        )
    }

    /// `a`'s window `(-reach, reach)` about `J` at `θ = 0` against `b` at
    /// the fine band: whether it holds `J` alone, and the pieces the arc
    /// past `J`'s ring reads in, outward.
    fn touch(a: &Curve3<f64>, b: &Curve3<f64>, reach: f64) -> (bool, Option<usize>) {
        let (ka, kb) = (a.conic().unwrap(), b.conic().unwrap());
        let w = Window {
            a: &ka,
            span_a: (-0.5, 0.5),
            b: &kb,
            span_b: &crate::curve_pair::Span::new(
                b,
                (-core::f64::consts::PI, core::f64::consts::PI),
            ),
            height: super::super::height(&ka, kb.frame().origin, kb.frame().axis),
        };
        let j = Joint {
            point: a.eval(0.0),
            ta: 0.0,
        };
        let ring = w.ring(&j, 0.0, fine()).unwrap();
        let level = super::level_on(&ka, &kb);
        let past = w
            .pieces((ring, reach), Some(true), &level, fine())
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
        let (ka, kb) = (a.conic().unwrap(), b.conic().unwrap());
        let w = Window {
            a: &ka,
            span_a: (-0.5, 0.5),
            b: &kb,
            span_b: &crate::curve_pair::Span::new(
                &b,
                (-core::f64::consts::PI, core::f64::consts::PI),
            ),
            height: super::super::height(&ka, kb.frame().origin, kb.frame().axis),
        };
        let foot = b.foot(a.eval(5e-6), 0.0);
        let along = w.along_b(5e-6, 5e-6, &foot);
        assert!(
            along >= far,
            "the reading along b, {along:e}, is under the far end's distance {far:e}"
        );
        let level = super::level_on(&ka, &kb);
        assert_ne!(
            w.read((0.0, 1e-5), Some(true), &level, fine()),
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
        let b = Curve3::circle(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let a = Curve3::circle(
            Point3::new(1.0, 0.0, 1.0),
            Vec3::new(0.0, -1.0, 0.0),
            1.0,
            Vec3::new(-400f64.sin(), 0.0, -400f64.cos()),
        );
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
        let (ka, kb) = (a.conic().unwrap(), b.conic().unwrap());
        let w = Window {
            a: &ka,
            span_a: (399.0, 401.0),
            b: &kb,
            span_b: &crate::curve_pair::Span::new(
                &b,
                (-core::f64::consts::PI, core::f64::consts::PI),
            ),
            height: super::super::height(&ka, kb.frame().origin, kb.frame().axis),
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
