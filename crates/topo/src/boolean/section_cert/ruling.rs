//! **The ruling charts**: a cone against a cylinder or another cone in
//! any pose, read along the lines that rule one carrier.
//!
//! # One chart
//!
//! A family of lines `L(θ) = B(θ) + t·v(θ)`, `θ` round the turn, `v`
//! unit, rules a carrier: a cone's generator lines through its apex
//! (`B` the apex, `v = cos α·â + sin α·r̂(θ)`; `t > 0` is the `v > 0`
//! nappe at azimuth `θ`, `t < 0` the mirror nappe at `θ + π`), or a
//! cylinder's rulings (`B` round the wall at the radius, `v` its axis).
//! Each chart point `(θ, t)` is one point of the carrier, the apex
//! (`t = 0` on a cone) aside. Against the partner's quadric form `q`,
//! `q(L(θ, t)) = p₂t² + p₁t + p₀`, and in both charts one of `B`, `v`
//! is constant, so `p₂`, `p₁`, `p₀` and the discriminant
//! `E = p₁² − 4p₂p₀` are trigonometric polynomials of degree two:
//! at most four roots round the turn, which [`certified_subdivision`]
//! isolates.
//!
//! The section on the chart is `{(θ, t) : q = 0}`. With `p₀` never zero
//! where the line meets the apex (the caller decides the apex off the
//! partner), the projective roots of the quadratic, labelled by the
//! sign `σ` of `√E` in `t_σ = (−p₁ + σ√E)/2p₂`, are continuous over every
//! arc where `E > 0`, and meet only where `E` has a simple root (a fold:
//! the curve is regular there, since `∂F/∂θ = −E′/4p₂` at the double
//! root). So:
//!
//! - each maximal arc of `E > 0` is ONE component, its two branches
//!   joined at the arc's ends, and it spans a proper arc of the turn:
//!   null on this carrier;
//! - `E > 0` round the whole turn gives the TWO branches `σ = ±`, each
//!   a graph over the turn: essential on this carrier;
//! - a branch runs to infinity exactly where `p₂ = 0` (a line parallel to
//!   an asymptotic direction of the partner), and there `E = p₁² > 0`,
//!   so it is the branch `σ = −sign(p₁)`; a component that does so `k`
//!   times is `k` unbounded affine components, and one that never does
//!   is bounded.
//!
//! So every bounded component a chart reads is null on its carrier, or
//! every one is essential: the classes are uniform per chart, and two
//! charts, one ruling each carrier, give each component both flags with
//! no matching between them. Their counts must agree.
//!
//! # The residuals, in metres
//!
//! The subdivision decides on a residual in metres sharing `E`'s sign:
//! how far the line passes inside the partner, to first order.
//!
//! - **Against a cylinder** `(O, d, r)`: `r − δ`, `δ = |u·(v × d)|/|v × d|`
//!   the line's distance from the axis (`u = B − O`), exactly, since
//!   `E = 4|v × d|²(r² − δ²)`.
//! - **Against a cone** `(A, â, α)`: the quadratic's extreme value
//!   `−E/4p₂` over the gradient `2 sin α |â·(X − A)|` of the form at the
//!   extreme point, which is `E / (8 sin α cos²α |â·(F − A)|)` with `F` the
//!   apex's foot on the line (`2p₂ â·(X − A) = −2cos²α â·(F − A)`). It
//!   stays finite where `p₂ = 0`. Its denominator is floored at
//!   `8 sin α cos²α λ`, `λ` a millionth of the chart's lever, which only
//!   shrinks the residual (the conservative direction): it vanishes
//!   where the extreme point lies in the apex's plane, far from any
//!   tangency.
//!
//! `p₂`'s roots (cone against cone) are decided on `p₂` times the
//! chart's lever. Every order between roots is decided as arc length
//! at the chart's speed, and a pair the band cannot order is R-tan.

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use super::{sign, square_to};
use crate::boolean::circle_roots::{
    CircleRoots, SubdivisionFrame, SubdivisionRows, TrigPoly, certified_subdivision,
    rounding_charge,
};

/// A vector `x₀ + x₁ cos θ + x₂ sin θ`.
#[derive(Clone, Copy)]
pub(super) struct Swing<T: Real>(pub(super) [Vec3<T>; 3]);

impl<T: Real> Swing<T> {
    fn at(&self, theta: T) -> Vec3<T> {
        let (sn, cs) = theta.sin_cos();
        let [x0, x1, x2] = self.0;
        x0 + x1 * cs + x2 * sn
    }

    /// `self · o`: two first harmonics make at most a second.
    fn dot(&self, o: &Self) -> Wave<T> {
        let half = T::from_f64(0.5);
        let ([x0, x1, x2], [y0, y1, y2]) = (self.0, o.0);
        Wave {
            k: x0.dot(y0) + (x1.dot(y1) + x2.dot(y2)) * half,
            c1: x0.dot(y1) + x1.dot(y0),
            s1: x0.dot(y2) + x2.dot(y0),
            c2: (x1.dot(y1) - x2.dot(y2)) * half,
            s2: (x1.dot(y2) + x2.dot(y1)) * half,
        }
    }

    /// `self · n`, `n` fixed.
    fn along(&self, n: Vec3<T>) -> Wave<T> {
        let [x0, x1, x2] = self.0;
        Wave::first(x0.dot(n), x1.dot(n), x2.dot(n))
    }

    /// A ceiling on `|self(θ)|`.
    fn bound(&self) -> T {
        let [x0, x1, x2] = self.0;
        x0.norm() + x1.norm() + x2.norm()
    }
}

/// A scalar `k + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ`.
#[derive(Clone, Copy)]
struct Wave<T: Real> {
    k: T,
    c1: T,
    s1: T,
    c2: T,
    s2: T,
}

impl<T: Real> Wave<T> {
    fn first(k: T, c1: T, s1: T) -> Self {
        let z = T::zero();
        Self {
            k,
            c1,
            s1,
            c2: z,
            s2: z,
        }
    }

    fn plus(self, o: Self) -> Self {
        Self {
            k: self.k + o.k,
            c1: self.c1 + o.c1,
            s1: self.s1 + o.s1,
            c2: self.c2 + o.c2,
            s2: self.s2 + o.s2,
        }
    }

    fn scaled(self, x: T) -> Self {
        Self {
            k: self.k * x,
            c1: self.c1 * x,
            s1: self.s1 * x,
            c2: self.c2 * x,
            s2: self.s2 * x,
        }
    }

    /// `self · o` where the product stays of degree two: one factor
    /// constant, or both first harmonics. The harmonics past the second
    /// that a general product would carry are those factors' zero
    /// terms, and are not formed.
    fn times(self, o: Self) -> Self {
        let half = T::from_f64(0.5);
        Self {
            k: self.k * o.k + (self.c1 * o.c1 + self.s1 * o.s1) * half,
            c1: self.k * o.c1 + o.k * self.c1,
            s1: self.k * o.s1 + o.k * self.s1,
            c2: self.k * o.c2 + o.k * self.c2 + (self.c1 * o.c1 - self.s1 * o.s1) * half,
            s2: self.k * o.s2 + o.k * self.s2 + (self.c1 * o.s1 + self.s1 * o.c1) * half,
        }
    }

    fn poly(self) -> TrigPoly<T> {
        TrigPoly::second(self.k, self.c1, self.s1, self.c2, self.s2)
    }
}

/// The partner's carrier as a quadric form
/// `q(X) = α|X − P|² + β(n·(X − P))² + γ`: only its zero set, and the
/// sign of its discriminant along a line, are read.
#[derive(Clone, Copy)]
pub(super) enum Quadric<T: Real> {
    /// The wall about the axis through `o` along the unit `d`.
    Cylinder { o: Point3<T>, d: Vec3<T>, r: T },
    /// The double cone at `apex` about the unit `a`, `sc = (sin α, cos α)`.
    Cone {
        apex: Point3<T>,
        a: Vec3<T>,
        sc: (T, T),
    },
}

impl<T: Real> Quadric<T> {
    /// `(α, β, n, γ, P)`.
    fn form(&self) -> (T, T, Vec3<T>, T, Point3<T>) {
        match *self {
            Self::Cylinder { o, d, r } => {
                (T::one(), T::zero() - T::one(), d, T::zero() - r.powi(2), o)
            }
            Self::Cone { apex, a, sc } => (T::zero() - sc.1.powi(2), T::one(), a, T::zero(), apex),
        }
    }

    /// `(p₂, p₁, p₀)` of the line `B + t·v`, `v` unit, read directly.
    fn line(&self, b: Point3<T>, v: Vec3<T>) -> (T, T, T) {
        let two = T::from_f64(2.0);
        match *self {
            Self::Cylinder { o, d, r } => {
                let (m, w) = (v.cross(d), (b - o).cross(d));
                (m.dot(m), two * m.dot(w), w.dot(w) - r.powi(2))
            }
            Self::Cone { apex, a, sc } => {
                let u = b - apex;
                let (av, au, cc) = (a.dot(v), a.dot(u), sc.1.powi(2));
                (
                    av.powi(2) - cc * v.dot(v),
                    two * (av * au - cc * v.dot(u)),
                    au.powi(2) - cc * u.dot(u),
                )
            }
        }
    }

    /// The line's residual in metres, sharing `E`'s sign (module docs),
    /// and `E` itself.
    fn residual(&self, b: Point3<T>, v: Vec3<T>, floor: T) -> (T, T) {
        let (p2, p1, p0) = self.line(b, v);
        let e = p1.powi(2) - T::from_f64(4.0) * p2 * p0;
        let r = match *self {
            Self::Cylinder { o, d, r } => {
                let m = v.cross(d);
                r - (b - o).dot(m).abs() / m.norm()
            }
            Self::Cone { apex, a, sc } => {
                let (foot, _) = square_to(b - apex, v);
                let k = T::from_f64(8.0) * sc.0 * sc.1.powi(2);
                e / (k * a.dot(foot).abs().max(floor))
            }
        };
        (r, e)
    }
}

/// A family of lines `B(θ) + t·v(θ)` ruling a carrier, one of `B`, `v`
/// constant, `v` unit; `speed` (metres per radian) turns an angle into
/// the arc length its decisions are read in.
pub(super) struct Chart<T: Real> {
    pub(super) base: Point3<T>,
    pub(super) offset: Swing<T>,
    pub(super) dir: Swing<T>,
    pub(super) speed: T,
    /// The length a dimensionless margin of the chart is levered by.
    pub(super) lever: T,
}

impl<T: Real> Chart<T> {
    fn line(&self, theta: T) -> (Point3<T>, Vec3<T>) {
        (self.base + self.offset.at(theta), self.dir.at(theta))
    }
}

/// The predicate rows one chart is read under.
pub(super) struct ChartRows {
    /// A fold: `E`'s roots, their order, and the arcs between them.
    pub(super) fold: &'static str,
    /// A line parallel to the partner's asymptotic directions: `p₂`'s
    /// roots, and `p₁`'s sign at them.
    pub(super) asymptote: &'static str,
    /// The subdivision's own rows.
    pub(super) walk: SubdivisionRows,
}

/// What one chart reads of the section.
#[derive(Debug)]
pub(super) struct Reading<T: Real> {
    /// One witness per bounded component.
    pub(super) bounded: Vec<Point3<T>>,
    /// Every bounded component is essential on the chart's carrier.
    pub(super) essential: bool,
    /// The count of unbounded components.
    pub(super) unbounded: usize,
}

/// The chart's certified roots of `f` (sharing `residual`'s sign), in
/// increasing order on `[−π, π]`; the answer's refusal names `row`.
fn roots<T: Decide>(
    f: &TrigPoly<T>,
    residual: &impl Fn(T) -> T,
    frame: &SubdivisionFrame<T>,
    rows: &ChartRows,
    row: &'static str,
    band: Band,
) -> Result<Vec<T>, &'static str> {
    let found = match certified_subdivision(f, residual, frame, &rows.walk, None, band) {
        Ok(CircleRoots::Miss) => Vec::new(),
        Ok(CircleRoots::Certified { count, thetas }) => thetas[..count].to_vec(),
        _ => return Err(row),
    };
    let mut out: Vec<T> = Vec::with_capacity(found.len());
    for theta in found {
        let mut at = out.len();
        for (i, &o) in out.iter().enumerate() {
            match before(o, theta, frame.speed_hi, row, band)? {
                true => {}
                false => {
                    at = i;
                    break;
                }
            }
        }
        out.insert(at, theta);
    }
    Ok(out)
}

/// Whether `x` comes before `y`, decided as arc length at `speed`.
fn before<T: Decide>(
    x: T,
    y: T,
    speed: T,
    row: &'static str,
    band: Band,
) -> Result<bool, &'static str> {
    match sign(row, Margin::of((y - x) * speed), band) {
        Some(Sign::Positive) => Ok(true),
        Some(Sign::Negative) => Ok(false),
        _ => Err(row),
    }
}

/// **The section as one chart reads it** (module docs).
///
/// # Errors
///
/// The R-tan row: a fold or asymptote the band cannot isolate or order.
#[allow(clippy::too_many_lines)] // one reading: the roots, the arcs, the branches
pub(super) fn read<T: Decide>(
    chart: &Chart<T>,
    partner: &Quadric<T>,
    rows: &ChartRows,
    band: Band,
) -> Result<Reading<T>, &'static str> {
    let (two, four) = (T::from_f64(2.0), T::from_f64(4.0));
    let (alpha, beta, n, gamma, at) = partner.form();
    let u = Swing([
        chart.base - at + chart.offset.0[0],
        chart.offset.0[1],
        chart.offset.0[2],
    ]);
    let v = chart.dir;
    let (nu, nv) = (u.along(n), v.along(n));
    let p2 = v.dot(&v).scaled(alpha).plus(nv.times(nv).scaled(beta));
    let p1 = u
        .dot(&v)
        .scaled(alpha)
        .plus(nu.times(nv).scaled(beta))
        .scaled(two);
    let p0 = u.dot(&u).scaled(alpha).plus(nu.times(nu).scaled(beta));
    let p0 = Wave {
        k: p0.k + gamma,
        ..p0
    };
    let e = p1.times(p1).plus(p2.times(p0).scaled(T::zero() - four));
    // The terms each coefficient sums, for the harmonics' rounding.
    let (big_u, big_v) = (u.bound(), v.bound());
    let kappa = alpha.abs() + beta.abs();
    let t2 = kappa * big_v.powi(2);
    let t1 = two * kappa * big_u * big_v;
    let t0 = kappa * big_u.powi(2) + gamma.abs();
    let floor = chart.lever * T::from_f64(1e-6);
    let ceiling = match *partner {
        Quadric::Cylinder { r, .. } => four * big_v.powi(2) * (r + big_u),
        Quadric::Cone { sc, .. } => T::from_f64(8.0) * sc.0 * sc.1.powi(2) * (big_u + floor),
    };
    let fold = |theta: T| {
        let (b, w) = chart.line(theta);
        partner.residual(b, w, floor).0
    };
    let pi = T::pi();
    let frame = |noise: T, f_per_metre_hi: T| SubdivisionFrame {
        t0: T::zero() - pi,
        t1: pi,
        speed_hi: chart.speed,
        noise,
        f_per_metre: T::zero(),
        f_per_metre_hi,
        residual_reach: None,
    };
    let folds = roots(
        &e.poly(),
        &fold,
        &frame(rounding_charge(t1.powi(2) + four * t2 * t0), ceiling),
        rows,
        rows.fold,
        band,
    )?;
    // The asymptotes: where `p₂` vanishes, and the branch that leaves
    // there. Against a cylinder `p₂ = |v × d|²` (the cone's chart: its
    // zero is a generator along the axis, the aperture the caller
    // decides) or constant (the wall's chart), and neither vanishes.
    let mut asymptotes: Vec<(T, Sign)> = Vec::new();
    if let Quadric::Cone { .. } = partner {
        let lean = |theta: T| {
            let (b, w) = chart.line(theta);
            partner.line(b, w).0 * chart.lever
        };
        let found = roots(
            &p2.poly(),
            &lean,
            &frame(rounding_charge(t2), T::one() / chart.lever),
            rows,
            rows.asymptote,
            band,
        )?;
        for theta in found {
            let (b, w) = chart.line(theta);
            let (_, p1_at, _) = partner.line(b, w);
            let leaving = match sign(rows.asymptote, Margin::of(p1_at / two), band) {
                Some(Sign::Positive) => Sign::Negative,
                Some(Sign::Negative) => Sign::Positive,
                _ => return Err(rows.asymptote),
            };
            if sign(rows.asymptote, Margin::of(fold(theta)), band) != Some(Sign::Positive) {
                return Err(rows.asymptote);
            }
            asymptotes.push((theta, leaving));
        }
    }
    // A point of branch `σ` at `θ` (`None`: the root nearer `B`), by the
    // root form that does not cancel: `q = −(p₁ + sign(p₁)√E)/2`, roots
    // `q/p₂` and `p₀/q`.
    let point = |theta: T, branch: Option<Sign>| {
        let (b, w) = chart.line(theta);
        let (p2, p1, p0) = partner.line(b, w);
        let root = (p1.powi(2) - four * p2 * p0).max(T::zero()).sqrt();
        let q = (p1 + p1.select_le_zero(T::zero() - root, root)) / (T::zero() - two);
        let (wide, narrow) = (q / p2, p0 / q);
        // `p₁ ≤ 0`: `q/p₂` is `σ = +`; otherwise it is `σ = −`.
        let t = match branch {
            Some(Sign::Positive) => p1.select_le_zero(wide, narrow),
            Some(_) => p1.select_le_zero(narrow, wide),
            None => narrow,
        };
        b + w * t
    };
    let mut out = Reading {
        bounded: Vec::new(),
        essential: false,
        unbounded: 0,
    };
    if folds.is_empty() {
        let quarter = T::from_f64(core::f64::consts::FRAC_PI_4);
        let side = (0..8u8)
            .map(|k| quarter * T::from_f64(f64::from(k)))
            .find_map(
                |theta| match sign(rows.fold, Margin::of(fold(theta)), band) {
                    Some(s @ (Sign::Positive | Sign::Negative)) => Some(s),
                    _ => None,
                },
            )
            .ok_or(rows.fold)?;
        if side == Sign::Negative {
            return Ok(out);
        }
        out.essential = true;
        for branch in [Sign::Positive, Sign::Negative] {
            match asymptotes.iter().filter(|(_, s)| *s == branch).count() {
                0 => out.bounded.push(point(T::zero(), Some(branch))),
                k => out.unbounded += k,
            }
        }
        return Ok(out);
    }
    // The arcs between consecutive folds, the last wrapping round; their
    // signs alternate, and each positive one is a component.
    let count = folds.len();
    let mut placed = 0usize;
    let mut positive = 0usize;
    for i in 0..count {
        let lo = folds[i];
        let hi = if i + 1 < count {
            folds[i + 1]
        } else {
            folds[0] + T::tau()
        };
        let mid = (lo + hi) / two;
        match sign(rows.fold, Margin::of(fold(mid)), band) {
            Some(Sign::Positive) => {}
            Some(Sign::Negative) => continue,
            _ => return Err(rows.fold),
        }
        positive += 1;
        let mut hits = 0usize;
        for &(theta, _) in &asymptotes {
            // The wrapping arc holds what lies past the last fold or
            // short of the first.
            let inside = if i + 1 < count {
                before(lo, theta, chart.speed, rows.asymptote, band)?
                    && before(theta, hi, chart.speed, rows.asymptote, band)?
            } else {
                before(lo, theta, chart.speed, rows.asymptote, band)?
                    || before(theta, folds[0], chart.speed, rows.asymptote, band)?
            };
            hits += usize::from(inside);
        }
        placed += hits;
        match hits {
            0 => out.bounded.push(point(mid, None)),
            k => out.unbounded += k,
        }
    }
    if 2 * positive != count || placed != asymptotes.len() {
        return Err(rows.fold);
    }
    Ok(out)
}
