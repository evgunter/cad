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
//! # The clearance, in metres
//!
//! The subdivision decides on a residual sharing `E`'s sign that is a
//! LOWER bound on the line's **clearance**: how far the line must be
//! translated before it stops (`E > 0`) or starts (`E < 0`) crossing
//! the partner. It walks `E`'s harmonics through `D`, a ceiling on
//! `|E|` per metre of that residual round the turn, with `U ≥ |B(θ) − P|`
//! (`P` the partner's apex, or a point of its axis).
//!
//! - **Against a cylinder** `(P, d, r)` the residual is the clearance
//!   itself, `r − δ`, `δ ≤ U` the line's distance from the axis;
//!   `E = 4|v × d|²(r − δ)(r + δ)` with `|v × d| ≤ 1`, so `D = 4(r + U)`.
//! - **Against a cone** `(P, â, α)` it is `E/16d`, `d` the line's
//!   distance from the apex, so `D = 16U`. The form
//!   `q = (â·w)² − cos²α |w|²` (`w = X − P`) is `−e·(sin α|h| + cos α ρ)`,
//!   `e` the point's signed distance from the double cone and
//!   `sin α|h| + cos α ρ ≤ |w|`, so a point is at least `|q|/|w|` off
//!   it. Measure `t` from the line's point nearest the apex: `|w| ≤ d + |t|`,
//!   and `|p₂| ≤ 1`, `|p₁| ≤ 2d`, `|p₀| ≤ d²` (the form's matrix has norm
//!   at most one), so `|t*| ≤ d/|p₂|` at the quadratic's extreme
//!   `t* = −p₁/2p₂`, where `|q| = |E|/4|p₂|`. Where `E < 0`, `q` keeps one
//!   sign and `|q| = |p₂|(t − t*)² + |E|/4|p₂|` along the line: within
//!   `d + |t*|` of `t*` the ratio is at least `|E|/8d(1 + |p₂|)`, past it
//!   at least `√|E|/2`, and `|E| ≤ 8d²` makes both at least `|E|/16d`.
//!   Where `E > 0` the extreme point lies on the far side of the cone
//!   from the line's ends (`q` there has the sign opposite `p₂`'s, which
//!   a translation keeps), at least `|E|/4d(1 + |p₂|) ≥ E/16d` off it
//!   (`p₂ = 0`: a translation moves `p₁ = 2v·Mw` by at most twice its
//!   length, and `|p₁|/2 ≥ p₁²/16d`). `d` is at least the apex's distance
//!   from the chart's carrier, which the caller decides off the band.
//!
//! Through `d` rather than a constant, the bound keeps the clearance's
//! order near the apex, where `E` vanishes to second order. It falls
//! short of the clearance by the partner's conditioning (`sin 2α` and
//! `|p₂|`), never past it: a decision on it is never more confident
//! than the geometry.
//!
//! The residual is evaluated at each line from the same form, with a
//! running bound on its rounding ([`Rounded`]), and every sign decided
//! on it is decided on its magnitude less that bound: a side, the ON
//! test at a located root, the arcs' midpoints, the full turn's eight
//! angles, the asymptotes' branches. Each walk carries a [`RootSlack`]
//! meter, so a located root is within the band's zero of the true one,
//! and an order between two roots is decided on their gap less both
//! slacks. A reading that refuses does so under the chart's
//! **precision** row where the representation cannot resolve the band:
//! where its resolution (the largest of the harmonics' noise and the
//! clearance's running bound round the turn, in metres)
//! is not inside the band's zero, or where the same reading with its
//! rounding uncharged (no harmonic noise, the residual's value, no slack
//! meter or slacks) answers. Otherwise it refuses under the row that
//! refused.
//!
//! `p₂`'s roots (cone against cone) are decided on `p₂` times the
//! chart's lever. Every order between roots is decided as arc length
//! at the chart's reach speed, and a pair the band cannot order is
//! R-tan.

use geom_core::running::{self, Rounded};
use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use super::sign;
use crate::boolean::circle_roots::{
    CircleRoots, RootSlack, SubdivisionFrame, SubdivisionRows, TrigPoly, certified_subdivision,
    rounding_charge,
};

/// A vector `x₀ + x₁ cos θ + x₂ sin θ`; `turning` false, a constant
/// `x₀` (`x₁ = x₂ = 0`).
#[derive(Clone, Copy)]
pub(super) struct Swing<T: Real> {
    x: [Vec3<T>; 3],
    turning: bool,
}

impl<T: Real> Swing<T> {
    pub(super) fn fixed(x0: Vec3<T>) -> Self {
        let zero = Vec3::new(T::zero(), T::zero(), T::zero());
        Self {
            x: [x0, zero, zero],
            turning: false,
        }
    }

    pub(super) fn round(x: [Vec3<T>; 3]) -> Self {
        Self { x, turning: true }
    }

    fn at(&self, theta: T) -> Vec3<T> {
        let (sn, cs) = theta.sin_cos();
        let [x0, x1, x2] = self.x;
        x0 + x1 * cs + x2 * sn
    }

    /// A ceiling on `|self(θ)|`.
    fn bound(&self) -> T {
        let [x0, x1, x2] = self.x;
        x0.norm() + x1.norm() + x2.norm()
    }

    fn degree(&self) -> u8 {
        u8::from(self.turning)
    }
}

/// A scalar `k + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ`, of
/// `degree` as formed (a harmonic past it is zero).
#[derive(Clone, Copy)]
struct Wave<T: Real> {
    k: T,
    c1: T,
    s1: T,
    c2: T,
    s2: T,
    degree: u8,
}

impl<T: Real> Wave<T> {
    fn first(k: T, c1: T, s1: T, degree: u8) -> Self {
        let z = T::zero();
        Self {
            k,
            c1,
            s1,
            c2: z,
            s2: z,
            degree,
        }
    }

    fn poly(self) -> TrigPoly<T> {
        TrigPoly::second(self.k, self.c1, self.s1, self.c2, self.s2)
    }
}

/// The arithmetic a line's coefficients are formed in: a scalar at one
/// line, carried with its running bound, or a harmonic round the turn.
trait Ring<T: Real>: Copy {
    fn of(x: Rounded<T>) -> Self;
    fn plus(self, o: Self) -> Self;
    fn times(self, o: Self) -> Self;
}

impl<T: Real> Ring<T> for Rounded<T> {
    fn of(x: Rounded<T>) -> Self {
        x
    }

    fn plus(self, o: Self) -> Self {
        self + o
    }

    fn times(self, o: Self) -> Self {
        self * o
    }
}

impl<T: Real> Ring<T> for Wave<T> {
    fn of(x: Rounded<T>) -> Self {
        Self::first(x.value, T::zero(), T::zero(), 0)
    }

    fn plus(self, o: Self) -> Self {
        Self {
            k: self.k + o.k,
            c1: self.c1 + o.c1,
            s1: self.s1 + o.s1,
            c2: self.c2 + o.c2,
            s2: self.s2 + o.s2,
            degree: self.degree.max(o.degree),
        }
    }

    /// The product where it stays of degree two: one factor constant,
    /// or both first harmonics. Every product a chart forms is one (one
    /// of `B`, `v` is constant, so `p₁` is a first harmonic and one of
    /// `p₂`, `p₀` a constant), and the harmonics past the second a
    /// general product would carry are not formed.
    fn times(self, o: Self) -> Self {
        debug_assert!(
            self.degree + o.degree <= 2,
            "a product of harmonics past degree two"
        );
        let half = T::from_f64(0.5);
        Self {
            k: self.k * o.k + (self.c1 * o.c1 + self.s1 * o.s1) * half,
            c1: self.k * o.c1 + o.k * self.c1,
            s1: self.k * o.s1 + o.k * self.s1,
            c2: self.k * o.c2 + o.k * self.c2 + (self.c1 * o.c1 - self.s1 * o.s1) * half,
            s2: self.k * o.s2 + o.k * self.s2 + (self.c1 * o.s1 + self.s1 * o.c1) * half,
            degree: self.degree + o.degree,
        }
    }
}

/// A vector in a [`Ring`]'s arithmetic.
trait Vector<T: Real> {
    type S: Ring<T>;
    fn dot(&self, o: &Self) -> Self::S;
    fn along(&self, n: Vec3<T>) -> Self::S;
}

impl<T: Real> Vector<T> for [Rounded<T>; 3] {
    type S = Rounded<T>;

    fn dot(&self, o: &Self) -> Rounded<T> {
        running::dot(*self, *o)
    }

    fn along(&self, n: Vec3<T>) -> Rounded<T> {
        running::dot(*self, running::exact_vec(n))
    }
}

impl<T: Real> Vector<T> for Swing<T> {
    type S = Wave<T>;

    /// Two first harmonics make at most a second.
    fn dot(&self, o: &Self) -> Wave<T> {
        let half = T::from_f64(0.5);
        let ([x0, x1, x2], [y0, y1, y2]) = (self.x, o.x);
        Wave {
            k: x0.dot(y0) + (x1.dot(y1) + x2.dot(y2)) * half,
            c1: x0.dot(y1) + x1.dot(y0),
            s1: x0.dot(y2) + x2.dot(y0),
            c2: (x1.dot(y1) - x2.dot(y2)) * half,
            s2: (x1.dot(y2) + x2.dot(y1)) * half,
            degree: self.degree() + o.degree(),
        }
    }

    fn along(&self, n: Vec3<T>) -> Wave<T> {
        let [x0, x1, x2] = self.x;
        Wave::first(x0.dot(n), x1.dot(n), x2.dot(n), self.degree())
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

/// The pad a ceiling formed in a few correctly rounded operations is
/// multiplied by, so that the `f64` value bounds the true one.
const CEILING_PAD: f64 = 1.0 + 64.0 * geom_core::UNIT_ROUNDOFF;

impl<T: Real> Quadric<T> {
    /// `P`.
    fn at(&self) -> Point3<T> {
        match *self {
            Self::Cylinder { o, .. } => o,
            Self::Cone { apex, .. } => apex,
        }
    }

    /// `(p₂, p₁, p₀)` of the lines `P + u + t·v`, `v` unit, in any
    /// [`Ring`]: the one spelling of the form.
    fn coefficients<V: Vector<T>>(&self, u: &V, v: &V) -> [V::S; 3] {
        let exact = Rounded::exact;
        let (alpha, beta, n, gamma) = match *self {
            Self::Cylinder { d, r, .. } => (
                exact(T::one()),
                exact(T::zero() - T::one()),
                d,
                exact(T::zero()) - exact(r).square(),
            ),
            Self::Cone { a, sc, .. } => (
                exact(T::zero()) - exact(sc.1).square(),
                exact(T::one()),
                a,
                exact(T::zero()),
            ),
        };
        let (alpha, beta, gamma) = (V::S::of(alpha), V::S::of(beta), V::S::of(gamma));
        let two = V::S::of(exact(T::from_f64(2.0)));
        let (nu, nv) = (u.along(n), v.along(n));
        let p2 = alpha.times(v.dot(v)).plus(beta.times(nv.times(nv)));
        let p1 = two.times(alpha.times(u.dot(v)).plus(beta.times(nu.times(nv))));
        let p0 = alpha
            .times(u.dot(u))
            .plus(beta.times(nu.times(nu)))
            .plus(gamma);
        [p2, p1, p0]
    }

    /// The line `P + u + t·w`'s clearance bound (module docs), from its
    /// discriminant `e`, with its running bound: `r − δ` against a
    /// cylinder, `E/16d` against a cone. `d` is at least the apex's
    /// distance from the chart's carrier, which the caller decides off
    /// the band's zero, so the division is well conditioned.
    fn clearance(&self, u: [Rounded<T>; 3], w: [Rounded<T>; 3], e: Rounded<T>) -> Rounded<T> {
        match *self {
            Self::Cylinder { d, r, .. } => {
                let m = running::cross(w, running::exact_vec(d));
                let reach = running::dot(u, m);
                let reach = Rounded {
                    value: reach.value.abs(),
                    ..reach
                };
                Rounded::exact(r) - quotient(reach, running::dot(m, m).sqrt())
            }
            Self::Cone { .. } => {
                let along = running::dot(u, w);
                let perp = [0, 1, 2].map(|i| u[i] - w[i] * along);
                let apart = running::dot(perp, perp).sqrt();
                quotient(e, apart * Rounded::exact(T::from_f64(16.0)))
            }
        }
    }

    /// `D`, a ceiling on `|E|` per metre of the clearance bound (module
    /// docs), for lines whose base stands within `big_u` of `P`.
    fn per_metre(&self, big_u: T) -> T {
        let d = match *self {
            Self::Cylinder { r, .. } => T::from_f64(4.0) * (r + big_u),
            Self::Cone { .. } => T::from_f64(16.0) * big_u,
        };
        d * T::from_f64(CEILING_PAD)
    }
}

/// `n / d` with its running bound, `d`'s magnitude past its own bound.
fn quotient<T: Real>(n: Rounded<T>, d: Rounded<T>) -> Rounded<T> {
    let value = n.value / d.value;
    let room = d.value.abs() - d.error;
    Rounded {
        value,
        error: (n.error + value.abs() * d.error) / room
            + T::from_f64(geom_core::UNIT_ROUNDOFF) * value.abs(),
    }
}

/// `E = p₁² − 4p₂p₀`.
fn discriminant<T: Real, S: Ring<T>>([p2, p1, p0]: [S; 3]) -> S {
    let minus_four = S::of(Rounded::exact(T::from_f64(-4.0)));
    p1.times(p1).plus(minus_four.times(p2.times(p0)))
}

/// The value with its rounding bound taken off its magnitude, so that a
/// sign decided on it is the exact value's; `charged` false, the value.
fn shrunk<T: Real>(x: Rounded<T>, charged: bool) -> T {
    if !charged {
        return x.value;
    }
    let mag = (x.value.abs() - x.error).max(T::zero());
    x.value.select_le_zero(T::zero() - mag, mag)
}

/// A family of lines `B(θ) + t·v(θ)` ruling a carrier, one of `B`, `v`
/// constant, `v` unit.
pub(super) struct Chart<T: Real> {
    pub(super) base: Point3<T>,
    pub(super) offset: Swing<T>,
    pub(super) dir: Swing<T>,
    /// A ceiling on `|∂L/∂θ|` (metres per radian) over the chart's
    /// points within the pair's reach: the arc length every angle the
    /// chart decides is read in. A cone's generators turn at
    /// `|t| sin α`, unbounded along the line, so this is the speed at
    /// the reach's farthest distance from the apex; the wall's rulings
    /// turn at its radius everywhere.
    pub(super) reach_speed: T,
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
    /// A refusal the representation, not the band, makes: the reading's
    /// resolution is not inside the band's zero, or the reading answers
    /// with its rounding uncharged (module docs).
    pub(super) precision: &'static str,
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

/// One root walk of a chart: `f`'s harmonics, the residual (metres,
/// with its running bound) sharing its sign, a ceiling on `|f|` per
/// metre of that residual, and the harmonics' rounding (in `f`'s units).
struct Walk<'a, T: Real> {
    f: TrigPoly<T>,
    residual: &'a dyn Fn(T) -> Rounded<T>,
    per_metre: T,
    noise: T,
    row: &'static str,
}

/// The walk's certified roots, in increasing order on `[−π, π]`, each
/// within the band's zero of the true one (the [`RootSlack`] meter);
/// `charged` false, the walk with its rounding uncharged (no harmonic
/// noise, the residual's value, no slack meter).
fn roots<T: Decide>(
    walk: &Walk<'_, T>,
    speed: T,
    rows: &ChartRows,
    charged: bool,
    band: Band,
) -> Result<Vec<T>, &'static str> {
    let pi = T::pi();
    let frame = SubdivisionFrame {
        t0: T::zero() - pi,
        t1: pi,
        speed_hi: speed,
        noise: if charged { walk.noise } else { T::zero() },
        f_per_metre: walk.per_metre,
        f_per_metre_hi: walk.per_metre,
        residual_reach: None,
    };
    let slack = RootSlack {
        row: walk.row,
        residual: walk.residual,
        f_per_metre_hi: walk.per_metre,
    };
    let residual = |theta: T| shrunk((walk.residual)(theta), charged);
    let found = match certified_subdivision(
        &walk.f,
        &residual,
        &frame,
        &rows.walk,
        charged.then_some(&slack),
        band,
    ) {
        Ok(CircleRoots::Miss) => Vec::new(),
        Ok(CircleRoots::Certified { count, thetas }) => thetas[..count].to_vec(),
        _ => return Err(walk.row),
    };
    let slacks = slacks(charged, band);
    let mut out: Vec<T> = Vec::with_capacity(found.len());
    for theta in found {
        let mut at = out.len();
        for (i, &o) in out.iter().enumerate() {
            if !before(o, theta, speed, slacks, walk.row, band)? {
                at = i;
                break;
            }
        }
        out.insert(at, theta);
    }
    Ok(out)
}

/// Two roots' slacks together (metres): each within the band's zero
/// where the walk is `charged`.
fn slacks<T: Real>(charged: bool, band: Band) -> T {
    T::from_f64(if charged { 2.0 * band.zero() } else { 0.0 })
}

/// Whether the root `x` comes before the root `y`, decided as arc length
/// at `speed`, less both roots' `slacks`.
fn before<T: Decide>(
    x: T,
    y: T,
    speed: T,
    slacks: T,
    row: &'static str,
    band: Band,
) -> Result<bool, &'static str> {
    let gap = (y - x) * speed;
    let mag = (gap.abs() - slacks).max(T::zero());
    match sign(
        row,
        Margin::of(gap.select_le_zero(T::zero() - mag, mag)),
        band,
    ) {
        Some(Sign::Positive) => Ok(true),
        Some(Sign::Negative) => Ok(false),
        _ => Err(row),
    }
}

/// The roots one chart's reading stands on: `E`'s in increasing order
/// on `[−π, π]`, and each asymptote with the branch that leaves there.
struct Roots<T> {
    folds: Vec<T>,
    asymptotes: Vec<(T, Sign)>,
}

/// **The section as one chart reads it** (module docs).
///
/// # Errors
///
/// The R-tan row: a fold or asymptote the band cannot isolate or order;
/// the chart's precision floor where the reading's resolution is not
/// inside the band's zero or the reading with its rounding uncharged
/// answers (module docs).
pub(super) fn read<T: Decide>(
    chart: &Chart<T>,
    partner: &Quadric<T>,
    rows: &ChartRows,
    band: Band,
) -> Result<Reading<T>, &'static str> {
    reading(chart, partner, rows, true, band).map_err(|(row, resolution)| {
        let resolved = matches!(
            sign(rows.precision, Margin::of(resolution), band),
            Some(Sign::Zero)
        );
        if !resolved || reading(chart, partner, rows, false, band).is_ok() {
            rows.precision
        } else {
            row
        }
    })
}

/// [`read`], its rounding `charged` or not; a refusal carries the
/// reading's resolution (metres): the largest of the discriminant's
/// harmonic noise, `p₂`'s levered (against a cone), and the
/// clearance's running bound at eight angles.
fn reading<T: Decide>(
    chart: &Chart<T>,
    partner: &Quadric<T>,
    rows: &ChartRows,
    charged: bool,
    band: Band,
) -> Result<Reading<T>, (&'static str, T)> {
    let at = partner.at();
    let [x0, x1, x2] = chart.offset.x;
    let u = Swing {
        x: [chart.base - at + x0, x1, x2],
        turning: chart.offset.turning,
    };
    let v = chart.dir;
    let [p2, p1, p0] = partner.coefficients(&u, &v);
    let e = discriminant([p2, p1, p0]);
    // One line as `P + u + t·w`, and its coefficients from the same
    // form, with their running bounds.
    let line_at = |theta: T| {
        let (b, w) = chart.line(theta);
        let (b, p) = (
            running::exact_vec(b - Point3::origin()),
            running::exact_vec(at - Point3::origin()),
        );
        ([0, 1, 2].map(|i| b[i] - p[i]), running::exact_vec(w))
    };
    let at_line = |theta: T| {
        let (u, w) = line_at(theta);
        partner.coefficients(&u, &w)
    };
    // The terms each harmonic sums, for their rounding.
    let (two, four) = (T::from_f64(2.0), T::from_f64(4.0));
    let (big_u, big_v) = (u.bound(), v.bound());
    let kappa = match *partner {
        Quadric::Cylinder { .. } => two,
        Quadric::Cone { sc, .. } => T::one() + sc.1.powi(2),
    };
    let t2 = kappa * big_v.powi(2);
    let t1 = two * kappa * big_u * big_v;
    let t0 = match *partner {
        Quadric::Cylinder { r, .. } => kappa * big_u.powi(2) + r.powi(2),
        Quadric::Cone { .. } => kappa * big_u.powi(2),
    };
    let per_metre = partner.per_metre(big_u);
    let clearance = |theta: T| {
        let (u, w) = line_at(theta);
        partner.clearance(u, w, discriminant(partner.coefficients(&u, &w)))
    };
    let noise = rounding_charge(t1.powi(2) + four * t2 * t0);
    let quarter = T::from_f64(core::f64::consts::FRAC_PI_4);
    let lean_noise = match *partner {
        Quadric::Cylinder { .. } => T::zero(),
        Quadric::Cone { .. } => rounding_charge(t2) * chart.lever,
    };
    let resolution = (0..8u8).fold((noise / per_metre).max(lean_noise), |acc, k| {
        acc.max(clearance(quarter * T::from_f64(f64::from(k))).error)
    });
    let refused = |row: &'static str| (row, resolution);
    let folds = roots(
        &Walk {
            f: e.poly(),
            residual: &clearance,
            per_metre,
            noise,
            row: rows.fold,
        },
        chart.reach_speed,
        rows,
        charged,
        band,
    )
    .map_err(refused)?;
    // The asymptotes: where `p₂` vanishes, and the branch that leaves
    // there. Against a cylinder `p₂ = |v × d|²` (the cone's chart: its
    // zero is a generator along the axis, the aperture the caller
    // decides) or constant (the wall's chart), and neither vanishes.
    let mut asymptotes: Vec<(T, Sign)> = Vec::new();
    if let Quadric::Cone { .. } = partner {
        let lever = Rounded::exact(chart.lever);
        let lean = |theta: T| at_line(theta)[0] * lever;
        let found = roots(
            &Walk {
                f: p2.poly(),
                residual: &lean,
                per_metre: T::one() / chart.lever,
                noise: rounding_charge(t2),
                row: rows.asymptote,
            },
            chart.reach_speed,
            rows,
            charged,
            band,
        )
        .map_err(refused)?;
        for theta in found {
            let p1_at = at_line(theta)[1];
            let half = p1_at.div_exact(two);
            let leaving = match sign(rows.asymptote, Margin::of(shrunk(half, charged)), band) {
                Some(Sign::Positive) => Sign::Negative,
                Some(Sign::Negative) => Sign::Positive,
                _ => return Err(refused(rows.asymptote)),
            };
            asymptotes.push((theta, leaving));
        }
    }
    // A point of branch `σ` at `θ` (`None`: the root nearer `B`), by the
    // root form that does not cancel: `q = −(p₁ + sign(p₁)√E)/2`, roots
    // `q/p₂` and `p₀/q`.
    let point = |theta: T, branch: Option<Sign>| {
        let (b, w) = chart.line(theta);
        let [p2, p1, p0] = at_line(theta).map(|x| x.value);
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
    let side = |theta: T| {
        sign(
            rows.fold,
            Margin::of(shrunk(clearance(theta), charged)),
            band,
        )
    };
    assemble(
        &Roots { folds, asymptotes },
        &side,
        &point,
        (chart.reach_speed, slacks(charged, band)),
        rows,
        band,
    )
    .map_err(refused)
}

/// The components on one chart from its roots (module docs): `side` is
/// `E`'s sign at an angle, `point` a point of a branch there, and every
/// order is decided at `speed` less two roots' `slacks`.
///
/// Two checks guard what the roots promise by construction, refusing
/// under the fold row when broken: a positive arc count other than half
/// the folds (`E`'s signs alternate across simple roots), and an
/// asymptote on no positive arc (`E = p₁² > 0` there, `p₁`'s sign
/// decided past the band). [`read`]'s certified walks do not break them;
/// they are defence in depth against a walk that loses a root, and
/// `assemble_refuses_inconsistent_roots` plants each.
fn assemble<T: Decide>(
    roots: &Roots<T>,
    side: &dyn Fn(T) -> Option<Sign>,
    point: &dyn Fn(T, Option<Sign>) -> Point3<T>,
    (speed, slacks): (T, T),
    rows: &ChartRows,
    band: Band,
) -> Result<Reading<T>, &'static str> {
    let Roots { folds, asymptotes } = roots;
    let mut out = Reading {
        bounded: Vec::new(),
        essential: false,
        unbounded: 0,
    };
    if folds.is_empty() {
        let quarter = T::from_f64(core::f64::consts::FRAC_PI_4);
        let turn = (0..8u8)
            .map(|k| quarter * T::from_f64(f64::from(k)))
            .find_map(|theta| match side(theta) {
                Some(s @ (Sign::Positive | Sign::Negative)) => Some(s),
                _ => None,
            })
            .ok_or(rows.fold)?;
        if turn == Sign::Negative {
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
    let two = T::from_f64(2.0);
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
        match side(mid) {
            Some(Sign::Positive) => {}
            Some(Sign::Negative) => continue,
            _ => return Err(rows.fold),
        }
        positive += 1;
        let mut hits = 0usize;
        for &(theta, _) in asymptotes {
            // The wrapping arc holds what lies past the last fold or
            // short of the first.
            let inside = if i + 1 < count {
                before(lo, theta, speed, slacks, rows.asymptote, band)?
                    && before(theta, hi, speed, slacks, rows.asymptote, band)?
            } else {
                before(lo, theta, speed, slacks, rows.asymptote, band)?
                    || before(theta, folds[0], speed, slacks, rows.asymptote, band)?
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

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]
mod tests {
    //! The checks [`assemble`] guards, each planted: the walks of
    //! [`read`] never hand it roots that break them.

    use super::*;
    use geom_core::Tol;

    const ROWS: ChartRows = ChartRows {
        fold: "fold",
        asymptote: "asymptote",
        precision: "precision",
        walk: SubdivisionRows {
            clear: "clear",
            monotone: "monotone",
            side: "side",
            width: "width",
        },
    };

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// `E`'s sign as a cosine's: positive within a quarter turn of `0`.
    fn cosine(theta: f64) -> Option<Sign> {
        Some(if theta.cos() > 0.0 {
            Sign::Positive
        } else {
            Sign::Negative
        })
    }

    fn origin(_: f64, _: Option<Sign>) -> Point3<f64> {
        Point3::origin()
    }

    fn assembled(folds: &[f64], asymptotes: &[(f64, Sign)]) -> Result<usize, &'static str> {
        let roots = Roots {
            folds: folds.to_vec(),
            asymptotes: asymptotes.to_vec(),
        };
        assemble(&roots, &cosine, &origin, (1.0, 0.0), &ROWS, band()).map(|r| r.bounded.len())
    }

    #[test]
    fn assemble_refuses_inconsistent_roots() {
        use core::f64::consts::FRAC_PI_2;
        // The consistent reading the plants break: one positive arc.
        assert_eq!(
            assembled(&[-FRAC_PI_2, FRAC_PI_2], &[]),
            Ok(1),
            "consistent"
        );
        // An asymptote on the arc read negative.
        assert_eq!(
            assembled(&[-FRAC_PI_2, FRAC_PI_2], &[(3.0, Sign::Positive)]),
            Err("fold"),
            "an asymptote off a positive arc"
        );
        // Three folds: two positive arcs, not half of three.
        assert_eq!(
            assembled(&[-FRAC_PI_2, 0.5, FRAC_PI_2], &[]),
            Err("fold"),
            "an odd fold count"
        );
        // No fold and no angle of the turn read off the band.
        let roots = Roots {
            folds: Vec::new(),
            asymptotes: Vec::new(),
        };
        assert_eq!(
            assemble(&roots, &|_| None, &origin, (1.0, 0.0), &ROWS, band())
                .map(|r| r.bounded.len()),
            Err("fold"),
            "an undecided turn"
        );
    }
}
