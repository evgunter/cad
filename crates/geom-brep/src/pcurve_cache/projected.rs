//! **The projected image** ([`Pcurve::Projected`], C4): on an analytic
//! chart, the image of a carrier with no closed-form image (a spline
//! carrier; a circle on a sphere that is neither a parallel nor a
//! meridian) is the chart's own inverse applied to the carrier,
//! `P(t) = ψ(C(t))`. It is exact, not a fit.
//!
//! # What the row stores
//!
//! The carrier written in the chart's frame (`FramedCarrier`: its
//! control net, or a circle's centre and two radius vectors), the
//! chart's own scalars the inverse reads, a partition of the parameter
//! into pieces that each stay in one `atan2` sector (a per-piece branch
//! centre `θ_k = m_k·π/2`, a whole number of quarter turns, and on a
//! torus a second one `φ_k` for the tube angle),
//! and the deck map `(u, v) ↦ (u_off + u, v_off + v_sign·v)` that
//! places the image on the branch its loop walk chose.
//!
//! # The channels
//!
//! On chart-frame coordinates `(x, y, z)` (origin at the chart's origin,
//! apex or centre; `z` along the axis), with `ρ = √(x² + y²)` and the
//! azimuth `α(x, y) = θ_k + atan2(R(−θ_k)·(x, y))` read on the branch
//! nearest the piece's centre:
//!
//! | chart | `u` | `v` |
//! |---|---|---|
//! | plane | `x` | `y` |
//! | cylinder | `α(x, y)` | `z` |
//! | cone | `α(σx, σy)` | `z·cos α + σ·ρ·sin α` |
//! | sphere | `α(x, y)` | `atan2(z, ρ)` |
//! | torus | `α(x, y)` | `φ_k + atan2(R(−φ_k)·(ρ − R, z))` |
//!
//! `σ = ±1` is the cone's nappe. The cone's `v` is the slant of the
//! perpendicular foot of the point on the generator through its
//! azimuth, not its height (which is off by `1/cos α`).
//!
//! **`S(ψ(ξ))` is the chart's nearest point to `ξ`** on the plane, the
//! cylinder, the sphere and the torus, and on the cone the foot on the
//! generator line through the point's azimuth — the nearest point of
//! the nappe `σ` names wherever `σ·z ≥ 0`. Per chart, in the canonical
//! frame (`N = S∘ψ`):
//!
//! - plane: `N(ξ) = (x, y, 0)`, the orthogonal projection;
//! - cylinder: `N(ξ) = (R·(x, y)/ρ, z)`; `|ξ − N(ξ)| = |ρ − R|`;
//! - cone: `N(ξ) = s·(σ·sin α·r̂, cos α)` with `s = σρ·sin α + z·cos α`
//!   the foot's slant on the line through the apex in direction
//!   `(σ sin α r̂, cos α)`; `|ξ − N(ξ)| = |ρ·cos α − σz·sin α|`;
//! - sphere: `N(ξ) = r·ξ/|ξ|`; `|ξ − N(ξ)| = ||ξ| − r|`;
//! - torus: `N(ξ) = R·r̂ + r·unit(ρ − R, z)` in the meridian half-plane;
//!   `|ξ − N(ξ)| = |√((ρ − R)² + z²) − r|`.
//!
//! The rows `the_projection_is_the_nearest_point_on_every_chart` (unit
//! tests below) measure each identity on random points.
//!
//! # The certificate (the envelope alone certifies)
//!
//! With `Ŝ` the chart's orthonormal twin and `ξ̂` the carrier re-derived
//! in its frame (so `C = Ŝ`'s origin `+ F̂·ξ̂` exactly),
//!
//! ```text
//! |S(P) − C| ≤ |S(P) − Ŝ(P)| + |N(ξ) − N(ξ̂)| + |N(ξ̂) − ξ̂|
//!                  frame          fidelity        incidence
//! ```
//!
//! plus the deck map's distance from a deck transformation of the chart
//! (whole periods, the sphere's twin) and the stored chart scalars'
//! distance from the chart's, both fidelity. **Incidence** is the
//! carrier's distance from the chart: the chart's implicit form `f`
//! composed along `ξ̂` (`geom_core::spline::compose::canonical_composite`
//! for a net, `sphere_circle::off_sphere_sup` for a circle), converted to
//! metres per knot span of the twin net, refined to `INCIDENCE_CUTS`
//! parts a span. The conversion divides `|f|` by a lower bound on
//! `|∇f|` along the segment from the point to its nearest point on the
//! chart, so each is tight to first order: `|f|` on the plane;
//! `D = |f| / (R + max(ρ_min, R − |f|/R))` on the cylinder and the same
//! with `r` on the sphere; `|f| / lever` on the cone, with
//! `lever = ρ_min·cos α + (σz)_min·sin α` (the twin's `half_angle`),
//! which must decide positive or the span refuses
//! (`SectorRefused { channel: Lever }`); and on the torus
//! `|f| / ((r + max(0, r − D₁))·((ρ_min + R)² − r²))` with
//! `D₁ = |f| / (r·((ρ_min + R)² − r²))`. `ρ_min` and `(σz)_min` are
//! read off the span's refined parts (`part_floors`: each part's box
//! nearest point maxed with its chord-support bound). The composite
//! takes no root; `ρ_min` takes one square root of a scalar bound, in
//! the scalar's outward-rounded arithmetic. **Fidelity** is the
//! stored net's distance from `ξ̂` (`max |qᵢ − q̂ᵢ|`, the partition of
//! unity) through `N`'s Lipschitz bound on the segment between them:
//! `max(1, R/ρ)` on the cylinder, `r/|ξ|` on the sphere,
//! `1 + |s|·sin α/ρ` on the cone, `(R + r)/ρ + r/m` on the torus (`m`
//! the distance to the tube's core), each with its lower bound read off
//! the incidence term (`|ξ̂| ≥ r − incidence`, and so on) less the
//! fidelity itself.
//!
//! **The sector condition**, decided per piece: the piece's radial hull
//! (its Bézier controls rotated by `−θ_k`, or a circle's harmonic box)
//! lies in the open half-plane `x′ > 0`, and on a torus the same for
//! `(ρ − R, z)` rotated by `−φ_k`; consecutive branch centres are at most
//! a quarter turn apart, so the image is one continuous branch. The
//! centres are quarter turns so the rotation is exact: a bracketed
//! scalar's image carries no width from them. A
//! piece at or within the band of the singular set (a sphere's pole, the
//! cone's axis, the torus's core) refuses typed
//! ([`PcurveCertifyError::SectorRefused`]). Sector, branch and incidence
//! are decided over the pieces and spans the edge's interval `[t0, t1]`
//! overlaps; a piece wholly outside it is not the edge's. The azimuth
//! period gate reads the image's sweep on a cover finer than its pieces
//! (`SWEEP_CUTS` a piece, graded toward both ends by `SWEEP_GRADE`
//! halvings), so a whole turn certifies and a sweep past one refuses.
//!
//! A net's hull terms are certification arithmetic (C9), so they run
//! through the fitted door ([`crate::FittedLane`]); a circle's are
//! closed forms of its coefficients and certify at every `Decide`
//! scalar.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_core::predicate::Band;
use geom_core::spline::{KnotVector, SpanLocate};
use geom_core::{Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Vec2, Vec3};

use crate::offset::Nappe;

use super::{
    ChartWindow, EnvelopeTerm, EnvelopeTerms, PcurveCertifyError, PcurveCheck, PcurveKind, decide,
    harmonic_span_box,
};

/// `σ = ±1` of a cone's nappe (module docs).
fn nappe_sign(nappe: Nappe) -> f64 {
    match nappe {
        Nappe::Opening => 1.0,
        Nappe::Mirror => -1.0,
    }
}

/// The chart a projected image's inverse is written for, with the
/// chart scalars the inverse reads (copied, so the image evaluates from
/// `t` alone; check 4 meters their distance from the chart's).
#[derive(Clone, Copy, Debug)]
pub enum ProjectedChart<T: Real> {
    /// `u = x`, `v = y`.
    Plane,
    /// `u = α`, `v = z`.
    Cylinder,
    /// `u = α(σx, σy)`, `v = z·cos α + σ·ρ·sin α`.
    Cone {
        /// `sin α` of the chart's half-angle.
        sin: T,
        /// `cos α` of the chart's half-angle.
        cos: T,
        /// The nappe `σ`.
        nappe: Nappe,
    },
    /// `u = α`, `v = atan2(z, ρ)`.
    Sphere,
    /// `u = α`, `v = φ + atan2(R(−φ)·(ρ − R, z))`.
    Torus {
        /// The major radius `R`.
        major: T,
    },
}

impl<T: Real> ProjectedChart<T> {
    fn kind(&self) -> geom::SurfaceKind {
        match self {
            Self::Plane => geom::SurfaceKind::Plane,
            Self::Cylinder => geom::SurfaceKind::Cylinder,
            Self::Cone { .. } => geom::SurfaceKind::Cone,
            Self::Sphere => geom::SurfaceKind::Sphere,
            Self::Torus { .. } => geom::SurfaceKind::Torus,
        }
    }

    fn sigma(&self) -> T {
        match self {
            Self::Cone { nappe, .. } => T::from_f64(nappe_sign(*nappe)),
            _ => T::one(),
        }
    }
}

/// The carrier written in the chart's frame (module docs): chart-frame
/// coordinates, origin at the chart's origin, apex or centre.
#[derive(Clone, Debug)]
pub enum FramedCarrier<T: Real> {
    /// A spline carrier's net, with the carrier's own knots and weights.
    /// Its pieces are read on the carrier's own parameter.
    Net(Arc<NurbsCurve3<T>>),
    /// A circle `centre + a·cos t + b·sin t`. Its pieces are read on
    /// `s = (t − origin)/span`, the uniform partition of the interval it
    /// was derived over.
    Circle {
        /// The centre.
        centre: Vec3<T>,
        /// The `cos t` coefficient.
        a: Vec3<T>,
        /// The `sin t` coefficient.
        b: Vec3<T>,
        /// The carrier parameter at `s = 0`.
        origin: T,
        /// The parameter length at `s = 1`.
        span: T,
    },
}

impl<T: SpanLocate> FramedCarrier<T> {
    /// The chart-frame point at the carrier parameter `t`.
    #[must_use]
    pub fn eval(&self, t: T) -> Vec3<T> {
        match self {
            Self::Net(net) => net.eval(t) - Point3::origin(),
            Self::Circle { centre, a, b, .. } => {
                let (s, c) = t.sin_cos();
                *centre + *a * c + *b * s
            }
        }
    }
}

impl<T: Real> FramedCarrier<T> {
    /// The carrier parameters of the piece parameters `a`, `b`.
    fn piece_span(&self, a: f64, b: f64) -> (T, T) {
        match self {
            Self::Net(_) => (T::from_f64(a), T::from_f64(b)),
            Self::Circle { origin, span, .. } => (
                *origin + *span * T::from_f64(a),
                *origin + *span * T::from_f64(b),
            ),
        }
    }

    /// The piece parameter of `t`.
    fn piece_param(&self, t: T) -> T {
        match self {
            Self::Net(_) => t,
            Self::Circle { origin, span, .. } => (t - *origin) / *span,
        }
    }
}

/// A projected chart image (module docs). Built by [`super::chart_pcurve`]
/// and its interval-aware sibling [`super::chart_pcurve_over`]; carried
/// across a chart translation or reflection by [`Pcurve::map_affine`].
#[derive(Clone, Debug)]
pub struct ProjectedImage<T: Real> {
    pub(super) chart: ProjectedChart<T>,
    pub(super) carrier: FramedCarrier<T>,
    /// The piece breaks on the piece parameter: a clamped degree-1 knot
    /// vector, one span per piece.
    pub(super) breaks: KnotVector,
    /// `θ_k = m_k·π/2`, one quarter-turn index per piece.
    pub(super) azimuth: Vec<i32>,
    /// `φ_k = n_k·π/2`, one per piece on a torus chart; empty elsewhere.
    pub(super) tube: Vec<i32>,
    pub(super) u_off: T,
    pub(super) v_off: T,
    pub(super) v_sign: T,
}

use super::Pcurve;

impl<T: Real> ProjectedImage<T> {
    /// The chart the inverse is written for.
    #[must_use]
    pub fn chart(&self) -> &ProjectedChart<T> {
        &self.chart
    }

    /// The carrier in the chart's frame.
    #[must_use]
    pub fn carrier(&self) -> &FramedCarrier<T> {
        &self.carrier
    }

    /// The piece breaks.
    #[must_use]
    pub fn breaks(&self) -> &KnotVector {
        &self.breaks
    }

    /// The per-piece branch centres `θ_k`, as quarter turns.
    #[must_use]
    pub fn azimuth(&self) -> &[i32] {
        &self.azimuth
    }

    /// The per-piece tube branch centres `φ_k` (torus only), as quarter
    /// turns.
    #[must_use]
    pub fn tube(&self) -> &[i32] {
        &self.tube
    }

    /// The deck map `(u_off, v_off, v_sign)`.
    #[must_use]
    pub fn deck(&self) -> (T, T, T) {
        (self.u_off, self.v_off, self.v_sign)
    }

    pub(crate) fn pieces(&self) -> usize {
        self.breaks.control_count().saturating_sub(1)
    }

    /// The image under the affine chart map `point`/`vector`, whose
    /// linear part is `diag(1, ±1)` (every map a chart row takes: the
    /// branch translations, the sphere's twin and the plane's `v`
    /// reflection) — [`Pcurve::map_affine`]'s arm.
    pub(super) fn map_affine(
        &self,
        point: impl Fn(Point2<T>) -> Point2<T>,
        vector: impl Fn(Vec2<T>) -> Vec2<T>,
    ) -> Self {
        let p = point(Point2::new(self.u_off, self.v_off));
        Self {
            u_off: p.x,
            v_off: p.y,
            v_sign: vector(Vec2::new(T::zero(), self.v_sign)).y,
            ..self.clone()
        }
    }

    /// The image's channels at a chart-frame point on piece `k`, before
    /// the deck map.
    fn raw(&self, p: Vec3<T>, k: usize) -> Point2<T> {
        if matches!(self.chart, ProjectedChart::Plane) {
            return Point2::new(p.x, p.y);
        }
        let sigma = self.chart.sigma();
        let theta = self.azimuth.get(k).copied().unwrap_or(0);
        let u = branch_azimuth(theta, p.x * sigma, p.y * sigma);
        let rho = (p.x.powi(2) + p.y.powi(2)).sqrt();
        let v = match self.chart {
            ProjectedChart::Plane => unreachable!("answered above"),
            ProjectedChart::Cylinder => p.z,
            ProjectedChart::Cone { sin, cos, .. } => p.z * cos + sigma * rho * sin,
            ProjectedChart::Sphere => p.z.atan2(rho),
            ProjectedChart::Torus { major } => {
                let phi = self.tube.get(k).copied().unwrap_or(0);
                branch_azimuth(phi, rho - major, p.z)
            }
        };
        Point2::new(u, v)
    }

    fn place(&self, raw: Point2<T>) -> Point2<T> {
        Point2::new(self.u_off + raw.x, self.v_off + self.v_sign * raw.y)
    }

    fn place_box(&self, w: ChartWindow<T>) -> ChartWindow<T> {
        let (a, b) = (self.v_sign * w.v_min, self.v_sign * w.v_max);
        ChartWindow {
            u_min: self.u_off + w.u_min,
            u_max: self.u_off + w.u_max,
            v_min: self.v_off + a.min(b),
            v_max: self.v_off + a.max(b),
        }
    }
}

/// `R(−m·π/2)·(x, y)`: a quarter-turn rotation, exact (a swap and
/// sign flips).
fn quarter<T: Real>(m: i32, x: T, y: T) -> (T, T) {
    match m.rem_euclid(4) {
        0 => (x, y),
        1 => (y, T::zero() - x),
        2 => (T::zero() - x, T::zero() - y),
        _ => (T::zero() - y, x),
    }
}

/// `m·π/2`.
fn quarter_angle<T: Real>(m: i32) -> T {
    T::pi() * T::from_f64(0.5 * f64::from(m))
}

/// `θ + atan2(R(−θ)·(x, y))` with `θ = m·π/2`: the azimuth of `(x, y)`
/// on the branch within half a period of `θ`. The rotation is exact, so
/// the branch centre adds no width to the image at a bracketed scalar.
fn branch_azimuth<T: Real>(m: i32, x: T, y: T) -> T {
    let (xr, yr) = quarter(m, x, y);
    quarter_angle::<T>(m) + yr.atan2(xr)
}

/// A chart-frame box of one piece: `x′`, `y′` the radial part rotated by
/// `−θ_k` (and flipped through the cone's nappe), `z` the axial part.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FrameBox<T> {
    pub(crate) x: (T, T),
    pub(crate) y: (T, T),
    pub(crate) z: (T, T),
}

/// `[lo, hi]·k` as a range.
fn scale<T: Real>((lo, hi): (T, T), k: T) -> (T, T) {
    let (a, b) = (lo * k, hi * k);
    (a.min(b), a.max(b))
}

fn add<T: Real>((a, b): (T, T), (c, d): (T, T)) -> (T, T) {
    (a + c, b + d)
}

/// The range of `atan2(y, x)` over a box: its corners' extremes, which
/// are exact for a box clear of the cut; a box reaching `x ≤ 0` answers
/// the whole period, through the selection rather than a branch.
fn atan2_range<T: Real>(x: (T, T), y: (T, T)) -> (T, T) {
    let corners = [
        y.0.atan2(x.0),
        y.0.atan2(x.1),
        y.1.atan2(x.0),
        y.1.atan2(x.1),
    ];
    let lo = corners[1..].iter().fold(corners[0], |m, &c| m.min(c));
    let hi = corners[1..].iter().fold(corners[0], |m, &c| m.max(c));
    (
        x.0.select_le_zero(T::zero() - T::pi(), lo),
        x.0.select_le_zero(T::pi(), hi),
    )
}

/// A box rotated by `−m·π/2`, exactly.
fn rotate_box<T: Real>(m: i32, x: (T, T), y: (T, T)) -> ((T, T), (T, T)) {
    let neg = |(lo, hi): (T, T)| (T::zero() - hi, T::zero() - lo);
    match m.rem_euclid(4) {
        0 => (x, y),
        1 => (y, neg(x)),
        2 => (neg(x), neg(y)),
        _ => (neg(y), x),
    }
}

/// The control points and weights of a net's restriction to `[a, b]`,
/// a sub-interval of one of its knot spans: the blossom
/// `B(a^{p−m}, b^m)`, `m = 0…p`, in homogeneous coordinates
/// (de Boor's recursion with the arguments taken in turn). Each control
/// is a convex combination of the span's, so the weights stay positive.
/// The recursion's ratios are formed in the scalar, so at an enclosure
/// scalar the controls enclose the true restriction's.
pub(crate) fn piece_controls<T: Real>(net: &NurbsCurve3<T>, a: f64, b: f64) -> Vec<(Vec3<T>, T)> {
    let kv = net.knots();
    let p = kv.degree();
    let span = kv.span_at(0.5 * (a + b));
    let j = span.index();
    let knots = kv.knots();
    let base: Vec<(Vec3<T>, T)> = span
        .window()
        .map(|i| {
            let w = T::from_f64(net.weights()[i]);
            ((net.control()[i] - Point3::origin()) * w, w)
        })
        .collect();
    (0..=p)
        .map(|m| {
            let mut d = base.clone();
            for r in 1..=p {
                let u = T::from_f64(if r <= p - m { a } else { b });
                for i in (r..=p).rev() {
                    let lo = T::from_f64(knots[j - p + i]);
                    let hi = T::from_f64(knots[j + 1 + i - r]);
                    let alpha = (u - lo) / (hi - lo);
                    let beta = T::one() - alpha;
                    d[i] = (
                        d[i - 1].0 * beta + d[i].0 * alpha,
                        d[i - 1].1 * beta + d[i].1 * alpha,
                    );
                }
            }
            let (hp, w) = d[p];
            (hp / w, w)
        })
        .collect()
}

impl<T: SpanLocate> ProjectedImage<T> {
    /// [`Pcurve::eval`]'s arm.
    pub(super) fn eval(&self, t: T) -> Point2<T> {
        let p = self.carrier.eval(t);
        let set = self.carrier.piece_param(t).locate_spans(&self.breaks);
        let mut acc: Option<Point2<T>> = None;
        for span in set.first.index()..=set.last.index() {
            let raw = self.raw(p, span - 1);
            acc = Some(match acc {
                None => raw,
                Some(q) => Point2::new(q.x.enclosure_hull(raw.x), q.y.enclosure_hull(raw.y)),
            });
        }
        self.place(acc.unwrap_or_else(|| self.raw(p, 0)))
    }

    /// The pieces `[t0, t1]` overlaps.
    pub(crate) fn overlapped(&self, t0: T, t1: T) -> core::ops::RangeInclusive<usize> {
        let a = self.carrier.piece_param(t0).locate_spans(&self.breaks);
        let b = self.carrier.piece_param(t1).locate_spans(&self.breaks);
        let lo = a.first.index().min(b.first.index()) - 1;
        let hi = a.last.index().max(b.last.index()) - 1;
        lo..=hi
    }

    /// Piece `k`'s chart-frame box, its end pieces stretched to `t0`
    /// and `t1` (a circle's), so the box covers what `[t0, t1]` reads of
    /// the piece.
    pub(crate) fn frame_box(&self, k: usize, t0: T, t1: T) -> FrameBox<T> {
        let knots = self.breaks.knots();
        self.frame_box_on(k, knots[k + 1], knots[k + 2], t0, t1)
    }

    /// The chart-frame box of `[s0, s1]`, a sub-interval of piece `k`
    /// on the piece parameter, read on that piece's branch. A circle's
    /// sub-interval at the partition's first or last break is stretched
    /// to `t0` or `t1`, as [`Self::frame_box`] stretches an end piece.
    fn frame_box_on(&self, k: usize, s0: f64, s1: f64, t0: T, t1: T) -> FrameBox<T> {
        let theta = self.azimuth.get(k).copied().unwrap_or(0);
        let sigma = self.chart.sigma();
        let knots = self.breaks.knots();
        match &self.carrier {
            FramedCarrier::Net(net) => {
                let controls = piece_controls(net, s0, s1);
                let first = controls[0].0;
                let mut x = (first.x, first.x);
                let mut y = (first.y, first.y);
                let mut z = (first.z, first.z);
                for (c, _) in &controls[1..] {
                    x = (x.0.min(c.x), x.1.max(c.x));
                    y = (y.0.min(c.y), y.1.max(c.y));
                    z = (z.0.min(c.z), z.1.max(c.z));
                }
                let (x, y) = if matches!(self.chart, ProjectedChart::Plane) {
                    (x, y)
                } else {
                    rotate_box(theta, scale(x, sigma), scale(y, sigma))
                };
                FrameBox { x, y, z }
            }
            FramedCarrier::Circle {
                centre,
                a,
                b,
                origin,
                span,
            } => {
                let mut ta = *origin + *span * T::from_f64(s0);
                let mut tb = *origin + *span * T::from_f64(s1);
                if s0 <= knots[0] {
                    ta = ta.min(t0);
                }
                if s1 >= knots[knots.len() - 1] {
                    tb = tb.max(t1);
                }
                let rot = |v: Vec3<T>| {
                    let (x, y) = quarter(theta, v.x * sigma, v.y * sigma);
                    Vec2::new(x, y)
                };
                let zero = Vec2::new(T::zero(), T::zero());
                let radial = harmonic_span_box(
                    Point2::origin() + rot(*centre),
                    rot(*a),
                    rot(*b),
                    zero,
                    ta,
                    tb,
                );
                let axial = harmonic_span_box(
                    Point2::new(centre.z, T::zero()),
                    Vec2::new(a.z, T::zero()),
                    Vec2::new(b.z, T::zero()),
                    zero,
                    ta,
                    tb,
                );
                FrameBox {
                    x: (radial.u_min, radial.u_max),
                    y: (radial.v_min, radial.v_max),
                    z: (axial.u_min, axial.u_max),
                }
            }
        }
    }

    /// `ρ`'s range over a box: its nearest point's distance from the
    /// axis below, its farthest corner's above.
    pub(crate) fn rho_range(b: &FrameBox<T>) -> (T, T) {
        let sq = |(lo, hi): (T, T)| lo.powi(2).max(hi.powi(2));
        let gap = |(lo, hi): (T, T)| lo.max(T::zero() - hi).max(T::zero());
        let (gx, gy) = (gap(b.x), gap(b.y));
        ((gx.powi(2) + gy.powi(2)).sqrt(), (sq(b.x) + sq(b.y)).sqrt())
    }

    /// The tube channel's rotated box `(ρ − R, z)` by `−φ_k`.
    pub(crate) fn tube_box(&self, k: usize, b: &FrameBox<T>, major: T) -> ((T, T), (T, T)) {
        let phi = self.tube.get(k).copied().unwrap_or(0);
        let rho = Self::rho_range(b);
        rotate_box(phi, (rho.0 - major, rho.1 - major), b.z)
    }

    /// The channel ranges over one box of piece `k`, before the deck
    /// map.
    fn window(&self, k: usize, b: &FrameBox<T>) -> ChartWindow<T> {
        let theta = quarter_angle::<T>(self.azimuth.get(k).copied().unwrap_or(0));
        let (u, v) = match self.chart {
            ProjectedChart::Plane => (b.x, b.y),
            ref chart => {
                let u = atan2_range(b.x, b.y);
                let u = (theta + u.0, theta + u.1);
                let rho = Self::rho_range(b);
                let v = match *chart {
                    ProjectedChart::Plane => unreachable!("answered above"),
                    ProjectedChart::Cylinder => b.z,
                    ProjectedChart::Cone { sin, cos, .. } => {
                        add(scale(b.z, cos), scale(scale(rho, self.chart.sigma()), sin))
                    }
                    ProjectedChart::Sphere => {
                        let c = [
                            b.z.0.atan2(rho.0),
                            b.z.0.atan2(rho.1),
                            b.z.1.atan2(rho.0),
                            b.z.1.atan2(rho.1),
                        ];
                        (
                            c[1..].iter().fold(c[0], |m, &x| m.min(x)),
                            c[1..].iter().fold(c[0], |m, &x| m.max(x)),
                        )
                    }
                    ProjectedChart::Torus { major } => {
                        let phi = quarter_angle::<T>(self.tube.get(k).copied().unwrap_or(0));
                        let (tx, ty) = self.tube_box(k, b, major);
                        let r = atan2_range(tx, ty);
                        (phi + r.0, phi + r.1)
                    }
                };
                (u, v)
            }
        };
        ChartWindow {
            u_min: u.0,
            u_max: u.1,
            v_min: v.0,
            v_max: v.1,
        }
    }

    /// The hull of `windows`, placed by the deck map; an empty list
    /// answers an inverted window.
    fn placed_hull(&self, windows: impl Iterator<Item = ChartWindow<T>>) -> ChartWindow<T> {
        let w = windows
            .reduce(|o, w| ChartWindow {
                u_min: o.u_min.min(w.u_min),
                u_max: o.u_max.max(w.u_max),
                v_min: o.v_min.min(w.v_min),
                v_max: o.v_max.max(w.v_max),
            })
            .unwrap_or(ChartWindow {
                u_min: T::one(),
                u_max: -T::one(),
                v_min: T::one(),
                v_max: -T::one(),
            });
        self.place_box(w)
    }

    /// [`Pcurve::chart_box`]'s arm: per overlapped piece, the channel
    /// ranges over that piece's box, hulled. Restriction-monotone: a
    /// sub-span overlaps a subset of the pieces, and an end piece is
    /// stretched only to an end it does not already contain.
    pub(super) fn chart_box(&self, t0: T, t1: T) -> ChartWindow<T> {
        self.placed_hull(
            self.overlapped(t0, t1)
                .map(|k| self.window(k, &self.frame_box(k, t0, t1))),
        )
    }

    /// The channel ranges over `[t0, t1]` read on a cover finer than the
    /// pieces: each piece cut into [`SWEEP_CUTS`] equal parts, and the
    /// parts `[t0, t1]` overlaps boxed on their piece's branch. The
    /// period gate's reading (check 2): a piece's box is as wide as its
    /// sector allows, so the hull of piece boxes over-reads a long sweep
    /// by up to a sector at each end, while a short part's box is tight
    /// to the part's own sweep.
    pub(super) fn sweep_box(&self, t0: T, t1: T) -> ChartWindow<T> {
        let knots = self.breaks.knots();
        let pieces = self.pieces();
        let mut fine = Vec::with_capacity(pieces * SWEEP_CUTS + 2 * usize::from(SWEEP_GRADE) + 1);
        fine.push(knots[1]);
        for k in 0..pieces {
            let (a, b) = (knots[k + 1], knots[k + 2]);
            #[allow(clippy::cast_precision_loss)]
            let step = (b - a) / SWEEP_CUTS as f64;
            // The partition's two ends are graded geometrically, so the
            // parts there are short enough that their boxes read the
            // sweep's ends tightly.
            if k == 0 {
                fine.extend(
                    (1..=SWEEP_GRADE)
                        .rev()
                        .map(|j| a + step * 0.5f64.powi(i32::from(j))),
                );
            }
            for c in 1..SWEEP_CUTS {
                #[allow(clippy::cast_precision_loss)]
                fine.push(a + step * c as f64);
            }
            if k + 1 == pieces {
                fine.extend((1..=SWEEP_GRADE).map(|j| b - step * 0.5f64.powi(i32::from(j))));
            }
            fine.push(b);
        }
        fine.dedup();
        let fine = breaks_vector(&fine);
        let (lo, hi) = (
            self.carrier.piece_param(t0).locate_spans(&fine),
            self.carrier.piece_param(t1).locate_spans(&fine),
        );
        let first = lo.first.index().min(hi.first.index()) - 1;
        let last = lo.last.index().max(hi.last.index()) - 1;
        let cuts = fine.knots();
        self.placed_hull((first..=last).map(|j| {
            let (a, b) = (cuts[j + 1], cuts[j + 2]);
            let k = self.breaks.span_at(0.5 * (a + b)).index() - 1;
            self.window(k, &self.frame_box_on(k, a, b, t0, t1))
        }))
    }
}

/// How many equal parts the fitted door cuts each knot span of a net
/// into before composing the chart's implicit form along it
/// (`pcurve_cache::projected_hull_lane`): the incidence is read per part.
pub(crate) const INCIDENCE_CUTS: usize = 8;

/// How many equal parts [`ProjectedImage::sweep_box`] cuts a piece into.
const SWEEP_CUTS: usize = 16;

/// How many halvings [`ProjectedImage::sweep_box`] grades the
/// partition's first and last part by.
const SWEEP_GRADE: u8 = 40;

// ---------------------------------------------------------------------
// Derivation
// ---------------------------------------------------------------------

/// The deepest halving of a top-level piece the derivation tries before
/// refusing: a piece `2⁻²⁰` of its span long, which a carrier the band
/// clears of the singular set does not need.
const MAX_REFINE: u32 = 20;

/// The chart's frame: origin and `(e₁, e₂, e₃)` (`u_ref`, `axis × u_ref`,
/// `axis`; a plane's `normal` for the axis). `None` off an analytic chart.
pub(crate) fn chart_frame<T: Real>(surface: &Surface<T>) -> Option<(Point3<T>, [Vec3<T>; 3])> {
    let frame = |o: Point3<T>, axis: Vec3<T>, u_ref: Vec3<T>| (o, [u_ref, axis.cross(u_ref), axis]);
    Some(match *surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => frame(origin, normal, u_ref),
        Surface::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        } => frame(origin, axis, u_ref),
        Surface::Cone {
            apex, axis, u_ref, ..
        } => frame(apex, axis, u_ref),
        Surface::Sphere {
            center,
            axis,
            u_ref,
            ..
        }
        | Surface::Torus {
            center,
            axis,
            u_ref,
            ..
        } => frame(center, axis, u_ref),
        Surface::Nurbs(_) | Surface::Approx(_) => return None,
    })
}

fn to_frame<T: Real>(v: Vec3<T>, e: &[Vec3<T>; 3]) -> Vec3<T> {
    Vec3::new(v.dot(e[0]), v.dot(e[1]), v.dot(e[2]))
}

/// A net's control points written in a chart frame.
pub(crate) fn framed_net<T: Real>(
    net: &NurbsCurve3<T>,
    origin: Point3<T>,
    e: &[Vec3<T>; 3],
) -> NurbsCurve3<T> {
    net.map_points(|p| Point3::origin() + to_frame(p - origin, e))
}

fn sector_refused(piece: usize, channel: SectorChannel) -> PcurveCertifyError {
    PcurveCertifyError::SectorRefused {
        piece: u32::try_from(piece).unwrap_or(u32::MAX),
        channel,
    }
}

/// Which `atan2` channel a sector refusal names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SectorChannel {
    /// The azimuth: the radial hull reaches the axis, or leaves the
    /// half-plane its branch centre names.
    Azimuth,
    /// A torus's tube angle: the hull reaches the tube's core.
    Tube,
    /// Two consecutive branch centres half a period apart or more.
    Branch,
    /// A cone's metre lever `ρ·cos α + σz·sin α`: the piece reaches the
    /// apex or the other nappe.
    Lever,
}

impl SectorChannel {
    /// What failed, in a clause.
    #[must_use]
    pub fn describe(self) -> &'static str {
        match self {
            Self::Azimuth => {
                "its radial hull reaches the chart's axis (a sphere's pole, the cone's apex) or \
                 leaves the half-plane of its branch"
            }
            Self::Tube => "its hull reaches the torus's tube core",
            Self::Branch => "two consecutive branch centres are half a period apart or more",
            Self::Lever => "its cone lever is not positive: it reaches the apex or the other nappe",
        }
    }
}

/// Whether piece `k`'s sector holds, decided at `T` on its box.
fn piece_sector<T: Decide>(
    image: &ProjectedImage<T>,
    k: usize,
    t0: T,
    t1: T,
    band: Band,
) -> Result<Option<SectorChannel>, Indeterminate> {
    if matches!(image.chart, ProjectedChart::Plane) {
        return Ok(None);
    }
    let b = image.frame_box(k, t0, t1);
    if !positive("pcurve_projected_sector", b.x.0, band)? {
        return Ok(Some(SectorChannel::Azimuth));
    }
    if let ProjectedChart::Torus { major } = image.chart {
        let (tx, _) = image.tube_box(k, &b, major);
        if !positive("pcurve_projected_sector", tx.0, band)? {
            return Ok(Some(SectorChannel::Tube));
        }
    }
    Ok(None)
}

/// Chooses piece `k`'s branch centres: the first quarter turn whose
/// open half-plane holds the piece's hull, and on a torus the same for
/// the tube channel. A SELECTION of structure, read off the candidates
/// with nothing recorded; the chosen pair is then decided as a verdict
/// ([`piece_sector`]).
fn choose_quarters<T: Decide>(image: &mut ProjectedImage<T>, k: usize, t0: T, t1: T, band: Band) {
    let clear = |x: T| {
        geom_core::k_stats::detached(|| positive("pcurve_projected_sector", x, band))
            .0
            .unwrap_or(false)
    };
    for m in 0..4 {
        image.azimuth[k] = m;
        let b = image.frame_box(k, t0, t1);
        if clear(b.x.0) {
            break;
        }
    }
    if let ProjectedChart::Torus { major } = image.chart {
        let b = image.frame_box(k, t0, t1);
        for n in 0..4 {
            image.tube[k] = n;
            if clear(image.tube_box(k, &b, major).0.0) {
                break;
            }
        }
    }
}

/// Unwraps quarter-turn indices onto one branch: each within a quarter
/// turn of the one before where the two pieces' sectors allow it.
fn unwrap_quarters(q: &mut [i32]) {
    for k in 1..q.len() {
        let d = (q[k] - q[k - 1]).rem_euclid(4);
        q[k] = q[k - 1] + if d > 2 { d - 4 } else { d };
    }
}

fn breaks_vector(breaks: &[f64]) -> KnotVector {
    let mut knots = Vec::with_capacity(breaks.len() + 2);
    knots.push(breaks[0]);
    knots.extend_from_slice(breaks);
    knots.push(breaks[breaks.len() - 1]);
    KnotVector::clamped(knots, 1)
        .unwrap_or_else(|e| unreachable!("projected breaks are increasing by construction: {e}"))
}

/// The chart a projected image of `framed` on `surface` is written
/// for: the chart's own scalars, and on a cone the nappe the carrier's
/// middle sits on (a carrier on the cone stays on one nappe, and the
/// lever reads every span of it).
///
/// # Errors
///
/// [`PcurveCertifyError::SectorRefused`] ([`SectorChannel::Lever`]) for a
/// cone carrier whose middle is at the apex's height;
/// [`PcurveCertifyError::Escalated`] when that is undecided.
fn chart_of<T: Decide>(
    surface: &Surface<T>,
    framed: &FramedCarrier<T>,
    band: Band,
) -> Result<ProjectedChart<T>, PcurveCertifyError> {
    Ok(match *surface {
        Surface::Plane { .. } => ProjectedChart::Plane,
        Surface::Cylinder { .. } => ProjectedChart::Cylinder,
        Surface::Sphere { .. } => ProjectedChart::Sphere,
        Surface::Torus { major_radius, .. } => ProjectedChart::Torus {
            major: major_radius,
        },
        Surface::Cone { half_angle, .. } => {
            let (sin, cos) = half_angle.sin_cos();
            let mid = match framed {
                FramedCarrier::Net(net) => {
                    let (d0, d1) = net.domain();
                    net.eval(T::from_f64(0.5 * (d0 + d1))).z
                }
                FramedCarrier::Circle { origin, span, .. } => {
                    framed.eval(*origin + *span * T::from_f64(0.5)).z
                }
            };
            let nappe =
                match decide("pcurve_projected_nappe", Margin::of(mid), band).map_err(|cause| {
                    PcurveCertifyError::Escalated {
                        check: PcurveCheck::Sector,
                        sample: 0,
                        cause,
                    }
                })? {
                    Sign::Positive => Nappe::Opening,
                    Sign::Negative => Nappe::Mirror,
                    Sign::Zero => return Err(sector_refused(0, SectorChannel::Lever)),
                };
            ProjectedChart::Cone { sin, cos, nappe }
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            return Err(PcurveCertifyError::UnsupportedChart {
                chart: surface.kind(),
            });
        }
    })
}

/// The projected image of `carrier` on `surface` (module docs), over the
/// carrier's knot domain for a net and over `span` (or one turn from
/// `0`) for a circle.
///
/// # Errors
///
/// [`PcurveCertifyError::SectorRefused`] when no partition within the
/// refinement cap puts every piece in one sector (a carrier through or
/// within the band of a pole, the cone's apex or the torus's core);
/// [`PcurveCertifyError::Escalated`] ([`PcurveCheck::Sector`]) when the
/// nappe or a sector stays in the band at the cap.
pub(crate) fn project<T: Decide>(
    carrier: &Curve3<T>,
    span: Option<(T, T)>,
    surface: &Surface<T>,
    band: Band,
) -> Result<ProjectedImage<T>, PcurveCertifyError> {
    let Some((origin, e)) = chart_frame(surface) else {
        return Err(PcurveCertifyError::UnsupportedChart {
            chart: surface.kind(),
        });
    };
    let framed = match carrier {
        Curve3::Nurbs(net) => FramedCarrier::Net(Arc::new(framed_net(net, origin, &e))),
        Curve3::Circle { .. } => {
            let Some(form) = super::carrier_harmonic(carrier) else {
                unreachable!("a circle has a harmonic form")
            };
            let (t0, t1) = span.unwrap_or((T::zero(), T::tau()));
            FramedCarrier::Circle {
                centre: to_frame(form.c - origin, &e),
                a: to_frame(form.a, &e),
                b: to_frame(form.b, &e),
                origin: t0,
                span: t1 - t0,
            }
        }
        Curve3::Line { .. } | Curve3::Ellipse { .. } | Curve3::Spiric { .. } => {
            return Err(PcurveCertifyError::ImageMismatch {
                image: PcurveKind::Projected,
                why: "a line, ellipse or spiric has a closed-form image, never a projected one",
            });
        }
    };
    let escalated = |cause| PcurveCertifyError::Escalated {
        check: PcurveCheck::Sector,
        sample: 0,
        cause,
    };
    let chart = chart_of(surface, &framed, band)?;
    let mut image = ProjectedImage {
        chart,
        carrier: framed,
        breaks: breaks_vector(&[0.0, 1.0]),
        azimuth: vec![0],
        tube: Vec::new(),
        u_off: T::zero(),
        v_off: T::zero(),
        v_sign: T::one(),
    };
    // The top-level pieces: a net's knot spans (a piece's hull is one
    // span's Bézier restriction), a circle's one interval.
    let tops: Vec<f64> = match &image.carrier {
        FramedCarrier::Net(net) => net.knots().knot_runs().map(|(k, _)| k).collect(),
        FramedCarrier::Circle { .. } => vec![0.0, 1.0],
    };
    if matches!(image.chart, ProjectedChart::Plane) {
        image.breaks = breaks_vector(&[tops[0], tops[tops.len() - 1]]);
        return Ok(image);
    }
    // A net's edge reads only `[t0, t1]` of it: a piece wholly outside
    // is kept for the partition and needs no sector (check 4 decides
    // the pieces the edge overlaps).
    let edge = match image.carrier {
        FramedCarrier::Net(_) => span,
        FramedCarrier::Circle { .. } => None,
    };
    let mut breaks = vec![tops[0]];
    let mut azimuth = Vec::new();
    let mut tube = Vec::new();
    for w in tops.windows(2) {
        refine(
            &mut image,
            (w[0], w[1]),
            edge,
            0,
            band,
            &mut breaks,
            &mut azimuth,
            &mut tube,
        )
        .map_err(|e| match e {
            Refine::Refused(channel) => sector_refused(azimuth.len(), channel),
            Refine::Escalated(cause) => escalated(cause),
        })?;
    }
    unwrap_quarters(&mut azimuth);
    unwrap_quarters(&mut tube);
    image.breaks = breaks_vector(&breaks);
    image.azimuth = azimuth;
    image.tube = tube;
    Ok(image)
}

enum Refine {
    Refused(SectorChannel),
    Escalated(Indeterminate),
}

/// Whether the piece `[a, b]` lies wholly outside the edge interval
/// `edge`, by more than the band: a SELECTION of the partition, read
/// with nothing recorded, an undecided answer reading as inside.
fn outside<T: Decide>(a: f64, b: f64, edge: Option<(T, T)>, band: Band) -> bool {
    let Some((t0, t1)) = edge else {
        return false;
    };
    let clear = |x: T| {
        geom_core::k_stats::detached(|| positive("pcurve_projected_outside", x, band))
            .0
            .unwrap_or(false)
    };
    clear(t0 - T::from_f64(b)) || clear(T::from_f64(a) - t1)
}

/// Halves `[a, b]` (on the piece parameter) until its piece holds its
/// sector, appending the pieces in order. A piece wholly outside the
/// edge interval `edge` is appended as found.
#[allow(clippy::too_many_arguments)]
fn refine<T: Decide>(
    image: &mut ProjectedImage<T>,
    (a, b): (f64, f64),
    edge: Option<(T, T)>,
    depth: u32,
    band: Band,
    breaks: &mut Vec<f64>,
    azimuth: &mut Vec<i32>,
    tube: &mut Vec<i32>,
) -> Result<(), Refine> {
    image.breaks = breaks_vector(&[a, b]);
    image.azimuth = vec![0];
    image.tube = if matches!(image.chart, ProjectedChart::Torus { .. }) {
        vec![0]
    } else {
        Vec::new()
    };
    let (ta, tb) = image.carrier.piece_span(a, b);
    choose_quarters(image, 0, ta, tb, band);
    let verdict = if outside(a, b, edge, band) {
        Ok(None)
    } else {
        piece_sector(image, 0, ta, tb, band)
    };
    match verdict {
        Ok(None) => {
            breaks.push(b);
            azimuth.push(image.azimuth[0]);
            if let Some(&phi) = image.tube.first() {
                tube.push(phi);
            }
            Ok(())
        }
        failed if depth >= MAX_REFINE => Err(match failed {
            Ok(Some(channel)) => Refine::Refused(channel),
            Err(cause) => Refine::Escalated(cause),
            Ok(None) => unreachable!("matched above"),
        }),
        _ => {
            let m = 0.5 * (a + b);
            refine(image, (a, m), edge, depth + 1, band, breaks, azimuth, tube)?;
            refine(image, (m, b), edge, depth + 1, band, breaks, azimuth, tube)
        }
    }
}

impl<T: Real> Pcurve<T> {
    /// The projected image, if this is one.
    #[must_use]
    pub fn projected(&self) -> Option<&ProjectedImage<T>> {
        match self {
            Pcurve::Projected(image) => Some(image),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------
// The certificate
// ---------------------------------------------------------------------

/// What the fitted door derives for a net's projected row, in
/// certification arithmetic: every piece's sector hull and every knot
/// span's composite bound.
#[derive(Clone, Debug)]
pub(crate) struct ProjectedHull {
    /// Per piece of the stored image, in order: the lower bound of
    /// `x′`, and of the tube's `X′` (`+∞` off a torus).
    pub(crate) pieces: Vec<PieceHull>,
    /// Per part of the re-derived twin net (each knot span cut into
    /// [`INCIDENCE_CUTS`]): its parameter range, the bound of `|f|`, and
    /// the floors its metre conversion reads.
    pub(crate) spans: Vec<SpanHull>,
}

/// One piece of a [`ProjectedHull`].
#[derive(Clone, Copy, Debug)]
pub(crate) struct PieceHull {
    pub(crate) range: (f64, f64),
    pub(crate) x_lo: f64,
    pub(crate) tube_lo: f64,
}

/// One part of a [`ProjectedHull`]'s twin net.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SpanHull {
    pub(crate) range: (f64, f64),
    /// The upper bound of `|f|` over the part (NaN where the composite
    /// refuses it).
    pub(crate) f_sup: f64,
    /// The lower bound of `ρ` over the part's points.
    pub(crate) rho_lo: f64,
    /// The range of `z` over the part's points.
    pub(crate) z: (f64, f64),
}

/// The floors of a Bézier part's points, read off its controls `q`:
/// the lower bound of `ρ` (the larger of the box's nearest point to the
/// axis and the controls' support along their chord, `ρ ≥ p·n̂`), and
/// the range of `z`.
pub(crate) fn part_floors<T: SpanLocate>(q: &[(Vec3<T>, T)]) -> (T, (T, T)) {
    let first = q[0].0;
    let mut b = FrameBox {
        x: (first.x, first.x),
        y: (first.y, first.y),
        z: (first.z, first.z),
    };
    for (c, _) in &q[1..] {
        b.x = (b.x.0.min(c.x), b.x.1.max(c.x));
        b.y = (b.y.0.min(c.y), b.y.1.max(c.y));
        b.z = (b.z.0.min(c.z), b.z.1.max(c.z));
    }
    let last = q[q.len() - 1].0;
    let (mx, my) = (first.x + last.x, first.y + last.y);
    let norm = (mx.powi(2) + my.powi(2)).sqrt();
    let support = q
        .iter()
        .map(|(c, _)| (c.x * mx + c.y * my) / norm)
        .reduce(|m, x| m.min(x))
        .unwrap_or_else(T::zero);
    let boxed = ProjectedImage::<T>::rho_range(&b).0;
    (boxed.max(support).max(T::zero()), b.z)
}

/// The one decision every condition of this module reads: whether
/// `value` decides positive. Not positive (zero or negative) is
/// `Ok(false)`; the band's undecided answer escalates.
fn positive<T: Decide>(name: &'static str, value: T, band: Band) -> Result<bool, Indeterminate> {
    Ok(decide(name, Margin::of(value), band)? == Sign::Positive)
}

/// [`positive`] as a check-4 verdict: `value` when it decides positive,
/// `refused` when it does not, an escalation at `check` (its sample the
/// piece or span `at`) when the band cannot tell.
fn require_positive<T: Decide>(
    name: &'static str,
    value: T,
    band: Band,
    check: PcurveCheck,
    at: usize,
    refused: PcurveCertifyError,
) -> Result<T, PcurveCertifyError> {
    match positive(name, value, band) {
        Ok(true) => Ok(value),
        Ok(false) => Err(refused),
        Err(cause) => Err(PcurveCertifyError::Escalated {
            check,
            sample: u32::try_from(at).unwrap_or(u32::MAX),
            cause,
        }),
    }
}

/// A piece's sector condition on `channel`: `value` (the hull's lower
/// bound of `x′`, or of the tube's `X′`) decides positive.
fn holds<T: Decide>(
    value: T,
    band: Band,
    piece: usize,
    channel: SectorChannel,
) -> Result<(), PcurveCertifyError> {
    require_positive(
        "pcurve_projected_sector",
        value,
        band,
        PcurveCheck::Sector,
        piece,
        sector_refused(piece, channel),
    )
    .map(drop)
}

/// The distance of an angle from the nearest point of `shift + τℤ`.
fn off_period<T: Real>(angle: T, shift: T) -> T {
    (angle - shift).reduce_periodic_centred(T::tau()).abs()
}

/// The incidence of a net's re-derived twin over the parts of it the
/// pieces `met` overlap: per part, the canonical composite's bound `|f|`
/// converted to metres (module docs) on the floors of the twin's own
/// points, and the smallest radial floor of the stored carrier, `ρ_min`
/// of the twin less `d`, the stored net's distance from it (what the
/// Lipschitz bounds on the segment between the two read).
///
/// The conversions, with `D` the first bound below and the factor each
/// divides by a lower bound on the exact denominator of the distance:
///
/// - plane: `|f|` (metres already);
/// - cylinder: `|ρ − R| = |f| / (ρ + R)`, with `ρ ≥ max(ρ_min, R − D)`,
///   `D = |f|/R`;
/// - sphere: `||ξ| − r| = |f| / (|ξ| + r)`, with
///   `|ξ| ≥ max(ρ_min, r − D)`, `D = |f|/r`;
/// - cone: `|ρ cos α − σz sin α| = |f| / (ρ cos α + σz sin α)`, the
///   lever `ρ_min cos α + (σz)_min sin α` (the twin's angle, the stored
///   nappe) decided positive;
/// - torus: `|m − r| = |f| / ((m + r)·((ρ + R)² + z² − r²))`, `m` the
///   distance to the tube's core, with `m ≥ max(0, r − D)` and the
///   second factor `≥ (ρ_min + R)² − r²`, `D = |f| / (r·((ρ_min + R)² − r²))`.
///
/// # Errors
///
/// [`PcurveCertifyError::ImageMismatch`] when no part meets a met piece,
/// or one that does has no certified composite bound; the cone's lever
/// refuses [`SectorChannel::Lever`] at the part.
pub(super) fn net_incidence<T: Decide>(
    hull: &ProjectedHull,
    met: &[usize],
    chart: &ProjectedChart<T>,
    twin: &Surface<T>,
    d: T,
    band: Band,
) -> Result<(T, T), PcurveCertifyError> {
    let refuse = |why| PcurveCertifyError::ImageMismatch {
        image: PcurveKind::Projected,
        why,
    };
    let mut incidence = T::zero();
    let mut rho_floor: Option<T> = None;
    for (j, span) in hull.spans.iter().enumerate() {
        let meets = met
            .iter()
            .map(|&k| &hull.pieces[k])
            .any(|p| p.range.0 < span.range.1 && p.range.1 > span.range.0);
        if !meets {
            continue;
        }
        if !span.f_sup.is_finite() {
            return Err(refuse(
                "a knot span's canonical composite bound is not certified",
            ));
        }
        let rho = T::from_f64(span.rho_lo);
        let floor = (rho - d).max(T::zero());
        rho_floor = Some(rho_floor.map_or(floor, |m| m.min(floor)));
        let f = T::from_f64(span.f_sup);
        let disp = match *twin {
            Surface::Plane { .. } => f,
            Surface::Cylinder { radius, .. } => f / (radius + rho.max(radius - f / radius)),
            Surface::Sphere { radius, .. } => f / (radius + rho.max(radius - f / radius)),
            Surface::Cone { half_angle, .. } => {
                let ProjectedChart::Cone { nappe, .. } = *chart else {
                    unreachable!("the chart kinds match")
                };
                let z = if nappe == Nappe::Opening {
                    T::from_f64(span.z.0)
                } else {
                    T::zero() - T::from_f64(span.z.1)
                };
                let (sin, cos) = half_angle.sin_cos();
                let lever = require_positive(
                    "pcurve_projected_lever",
                    rho * cos + z * sin,
                    band,
                    PcurveCheck::Sector,
                    j,
                    sector_refused(j, SectorChannel::Lever),
                )?;
                f / lever
            }
            Surface::Torus {
                major_radius,
                minor_radius,
                ..
            } => {
                let outer = (rho + major_radius).powi(2) - minor_radius.powi(2);
                let first = f / (minor_radius * outer);
                f / ((minor_radius + (minor_radius - first).max(T::zero())) * outer)
            }
            Surface::Nurbs(_) | Surface::Approx(_) => {
                unreachable!("a projected chart is analytic")
            }
        };
        incidence = incidence.max(disp);
    }
    let Some(rho_floor) = rho_floor else {
        return Err(refuse("no knot span of the net is met by a piece"));
    };
    Ok((incidence, rho_floor))
}

/// The stored net's distance from the twin's, control point for control
/// point: a bound on their distance at every parameter (the partition
/// of unity over one knot vector and one weight vector).
fn net_fidelity<T: Real>(net: &NurbsCurve3<T>, twin_net: &NurbsCurve3<T>) -> T {
    net.control()
        .iter()
        .zip(twin_net.control())
        .fold(T::zero(), |m, (q, p)| m.max(q.distance(*p)))
}

/// **A spline carrier's distance from an analytic surface**, certified
/// over its whole knot domain: limb 2 of C2 for an analytic operand of
/// a rung-3 edge ([`crate::analytic_rung3`]). The net is written in the
/// frame of the surface's orthonormal twin, which is the same point set
/// (orthonormalising a frame moves no point of the implicit surface),
/// and its incidence is [`net_incidence`] over one piece per knot span;
/// no branch is chosen, because a distance reads no angle.
///
/// # Errors
///
/// As [`net_incidence`]; [`PcurveCertifyError::UnsupportedChart`] off
/// an analytic chart; the fitted door's refusal of the composite.
pub(crate) fn net_offset_sup<T: Decide>(
    net: &NurbsCurve3<T>,
    surface: &Surface<T>,
    band: Band,
    lane: crate::FittedLane<T>,
) -> Result<T, PcurveCertifyError> {
    let Some((origin, e)) = chart_frame(surface) else {
        return Err(PcurveCertifyError::UnsupportedChart {
            chart: surface.kind(),
        });
    };
    let framed = FramedCarrier::Net(Arc::new(framed_net(net, origin, &e)));
    let chart = chart_of(surface, &framed, band)?;
    let runs: Vec<f64> = net.knots().knot_runs().map(|(k, _)| k).collect();
    let pieces = runs.len() - 1;
    let image = ProjectedImage {
        chart,
        carrier: framed,
        breaks: breaks_vector(&runs),
        azimuth: vec![0; pieces],
        tube: if matches!(chart, ProjectedChart::Torus { .. }) {
            vec![0; pieces]
        } else {
            Vec::new()
        },
        u_off: T::zero(),
        v_off: T::zero(),
        v_sign: T::one(),
    };
    let twin = super::orthonormal_chart(surface);
    let Some((o, te)) = chart_frame(&twin) else {
        unreachable!("an analytic chart has a frame")
    };
    let twin_net = framed_net(net, o, &te);
    let FramedCarrier::Net(stored) = &image.carrier else {
        unreachable!("built from the net above")
    };
    let d = net_fidelity(stored, &twin_net);
    let hull = lane.projected_hull(&image, &twin_net, &twin)?;
    let met: Vec<usize> = (0..pieces).collect();
    net_incidence(&hull, &met, &image.chart, &twin, d, band).map(|(incidence, _)| incidence)
}

/// Check 4 of a projected row (module docs): the envelope's terms, with
/// the sector condition decided first, each over the pieces `[t0, t1]`
/// overlaps. `boxed` is the row's chart box over `[t0, t1]`.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(super) fn projected_envelope<T: Decide>(
    image: &ProjectedImage<T>,
    t0: T,
    t1: T,
    boxed: &ChartWindow<T>,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
    lane: Option<crate::FittedLane<T>>,
) -> Result<EnvelopeTerms<T>, PcurveCertifyError> {
    let mut terms = EnvelopeTerms::new();
    let refuse = |why| PcurveCertifyError::ImageMismatch {
        image: PcurveKind::Projected,
        why,
    };
    // ---- The plane: the mapped net's difference, exactly. ----
    if let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = *surface
    {
        let (FramedCarrier::Net(net), Curve3::Nurbs(c)) = (&image.carrier, carrier) else {
            return Err(refuse(
                "a plane's projected image is a spline carrier's net",
            ));
        };
        // `S(P(t)) − C(t)` is affine in the image's net and linear in
        // the carrier's, over one knot vector and one weight vector, so
        // it is `Σ Rᵢ(t)·Δᵢ` and its sup is at most `max |Δᵢ|`.
        let v_ref = normal.cross(u_ref);
        let sup = net
            .control()
            .iter()
            .zip(c.control())
            .fold(T::zero(), |m, (q, p)| {
                let mapped = origin
                    + u_ref * (image.u_off + q.x)
                    + v_ref * (image.v_off + image.v_sign * q.y);
                m.max(mapped.distance(*p))
            });
        terms.add(EnvelopeTerm::Incidence, sup);
        return Ok(terms);
    }
    let twin = super::orthonormal_chart(surface);
    let Some((o, e)) = chart_frame(&twin) else {
        unreachable!("an analytic chart has a frame")
    };
    terms.add(
        EnvelopeTerm::Frame,
        super::frame_defect(surface, boxed.v_reach()),
    );
    let reach = |x: T| {
        require_positive(
            "pcurve_projected_reach",
            x,
            band,
            PcurveCheck::EnvelopeTerm(EnvelopeTerm::Fidelity),
            0,
            PcurveCertifyError::ResidualExceeded {
                check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::Fidelity),
                sample: 0,
            },
        )
    };
    let met: Vec<usize> = image.overlapped(t0, t1).collect();
    // ---- Incidence, raw fidelity and the radial floor. ----
    let (incidence, d, rho_floor) = match (&image.carrier, carrier) {
        (FramedCarrier::Net(net), Curve3::Nurbs(c)) => {
            let twin_net = framed_net(c, o, &e);
            let d = net_fidelity(net, &twin_net);
            let lane = lane.ok_or(PcurveCertifyError::FittedLaneUnsupported { scalar: T::NAME })?;
            let hull = lane.projected_hull(image, &twin_net, &twin)?;
            for &k in &met {
                let piece = &hull.pieces[k];
                holds(T::from_f64(piece.x_lo), band, k, SectorChannel::Azimuth)?;
                if matches!(image.chart, ProjectedChart::Torus { .. }) {
                    holds(T::from_f64(piece.tube_lo), band, k, SectorChannel::Tube)?;
                }
            }
            let (incidence, rho_floor) = net_incidence(&hull, &met, &image.chart, &twin, d, band)?;
            (incidence, d, rho_floor)
        }
        (FramedCarrier::Circle { centre, a, b, .. }, Curve3::Circle { .. }) => {
            let Some(form) = super::carrier_harmonic(carrier) else {
                unreachable!("a circle has a harmonic form")
            };
            let Surface::Sphere { center, radius, .. } = *surface else {
                return Err(refuse("a circle's projected image is written for a sphere"));
            };
            let d = (*centre - to_frame(form.c - o, &e)).norm()
                + (*a - to_frame(form.a, &e)).norm()
                + (*b - to_frame(form.b, &e)).norm();
            for &k in &met {
                let b = image.frame_box(k, t0, t1);
                holds(b.x.0, band, k, SectorChannel::Azimuth)?;
            }
            let incidence = crate::sphere_circle::off_sphere_sup(
                crate::sphere_circle::off_sphere_coefficients(
                    form.c - center,
                    form.a,
                    form.b,
                    radius,
                ),
                radius,
            );
            (incidence, d, T::zero())
        }
        _ => return Err(refuse("the stored carrier is not the edge's carrier kind")),
    };
    let (u_arm, v_arm) = match *surface {
        Surface::Cylinder { radius, .. } => (radius, T::one()),
        Surface::Cone { half_angle, .. } => (boxed.v_reach() * half_angle.sin().abs(), T::one()),
        Surface::Sphere { radius, .. } => (radius, radius),
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => (major_radius + minor_radius, minor_radius),
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            unreachable!("answered above")
        }
    };
    // Consecutive branch centres on one branch: each piece holds its
    // sector at the break it shares with the next, so their centres are
    // at most a quarter turn apart on one branch, and a whole period or
    // more apart on two.
    for w in met.windows(2) {
        let (i, j) = (w[0], w[1]);
        for angles in [&image.azimuth, &image.tube] {
            if let (Some(&x), Some(&y)) = (angles.get(i), angles.get(j))
                && (y - x).abs() > 1
            {
                return Err(sector_refused(j, SectorChannel::Branch));
            }
        }
    }
    terms.add(EnvelopeTerm::Incidence, incidence);
    // ---- Fidelity: the stored carrier through N's Lipschitz bound. ----
    let lipschitz = match *surface {
        Surface::Cylinder { radius, .. } => (radius / reach(radius - incidence - d)?).max(T::one()),
        Surface::Sphere { radius, .. } => radius / reach(radius - incidence - d)?,
        Surface::Cone { half_angle, .. } => {
            T::one() + (boxed.v_reach() + d) * half_angle.sin().abs() / reach(rho_floor)?
        }
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => {
            (major_radius + minor_radius) / reach(major_radius - minor_radius - incidence - d)?
                + minor_radius / reach(minor_radius - incidence - d)?
        }
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            unreachable!("answered above")
        }
    };
    terms.add(EnvelopeTerm::Fidelity, lipschitz * d);
    // ---- The deck map and the chart scalars. ----
    let twin_deck = match decide("pcurve_projected_v_sign", Margin::of(image.v_sign), band) {
        Ok(Sign::Positive) => false,
        Ok(Sign::Negative) if matches!(surface, Surface::Sphere { .. }) => true,
        Ok(_) => {
            return Err(refuse(
                "a reflected image on a curved chart is no deck map of it (only the sphere's twin is)",
            ));
        }
        Err(cause) => {
            return Err(PcurveCertifyError::Escalated {
                check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityV),
                sample: 0,
                cause,
            });
        }
    };
    let (shift, sign) = if twin_deck {
        (T::pi(), T::zero() - T::one())
    } else {
        (T::zero(), T::one())
    };
    terms.add(
        EnvelopeTerm::FidelityU,
        off_period(image.u_off, shift) * u_arm,
    );
    let v_reach = boxed.v_reach();
    let v_off = match *surface {
        Surface::Sphere { .. } | Surface::Torus { .. } => off_period(image.v_off, shift),
        _ => image.v_off.abs(),
    };
    terms.add(
        EnvelopeTerm::FidelityV,
        (v_off + (image.v_sign - sign).abs() * v_reach) * v_arm,
    );
    match (image.chart, &twin) {
        (ProjectedChart::Cone { sin, cos, .. }, Surface::Cone { half_angle, .. }) => {
            let (s, c) = half_angle.sin_cos();
            let drift = (sin - s).abs() + (cos - c).abs();
            terms.add(EnvelopeTerm::FidelityV, drift * (v_reach + incidence + d));
        }
        (
            ProjectedChart::Torus { major },
            Surface::Torus {
                major_radius,
                minor_radius,
                ..
            },
        ) => {
            let core = reach(*minor_radius - incidence - d)?;
            terms.add(
                EnvelopeTerm::FidelityV,
                *minor_radius * (major - *major_radius).abs() / core,
            );
        }
        _ => {}
    }
    Ok(terms)
}

/// The four checks of a projected row, in the fixed order every lane
/// keeps: the lane and its structure; the interval and the period
/// headroom; the schedule, as the envelope's cross-check on the witness
/// lane; and the envelope, which is the whole certified statement.
///
/// `lane` is read by a net's row only, at check 4: its hull terms are
/// certification arithmetic.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(super) fn run_projected_checks<T: Decide>(
    pcurve: &Pcurve<T>,
    t0: T,
    t1: T,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
    lane: Option<crate::FittedLane<T>>,
) -> Result<super::PcurveCertificate<T>, PcurveCertifyError> {
    use super::{EnvelopeStatement, Record};
    let Pcurve::Projected(image) = pcurve else {
        unreachable!("run_projected_checks: every caller matches `Pcurve::Projected` to reach it")
    };
    let refuse = |why| PcurveCertifyError::ImageMismatch {
        image: PcurveKind::Projected,
        why,
    };
    // ---- Check 1: the lane and the stored structure. ----
    if surface.is_placeholder_chart() {
        return Err(PcurveCertifyError::PlaceholderChart);
    }
    if matches!(surface, Surface::Nurbs(_) | Surface::Approx(_)) {
        return Err(PcurveCertifyError::UnsupportedChart {
            chart: surface.kind(),
        });
    }
    if image.chart.kind() != surface.kind() {
        return Err(refuse(
            "the image's inverse is written for another chart kind",
        ));
    }
    let pieces = image.pieces();
    let curved = !matches!(image.chart, ProjectedChart::Plane);
    if curved
        && (image.azimuth.len() != pieces
            || image.tube.len()
                != if matches!(image.chart, ProjectedChart::Torus { .. }) {
                    pieces
                } else {
                    0
                })
    {
        return Err(refuse("the image's branch centres are not one per piece"));
    }
    match (&image.carrier, carrier) {
        (FramedCarrier::Net(net), Curve3::Nurbs(c)) => {
            // The structure is the carrier's, exactly (C6): one knot
            // vector and one weight vector, so the stored net and the
            // carrier's differ control point for control point.
            if net.knots() != c.knots() || net.weights() != c.weights() {
                return Err(refuse(
                    "the stored net's knots or weights are not the carrier's",
                ));
            }
            let (d0, d1) = net.domain();
            let b = image.breaks.knots();
            if b[0] > d0 || b[b.len() - 1] < d1 {
                return Err(refuse(
                    "the image's pieces do not cover the carrier's domain",
                ));
            }
            if curved {
                let kv = net.knots();
                for k in 0..pieces {
                    let (a, z) = (b[k + 1], b[k + 2]);
                    let j = kv.span_at(0.5 * (a + z)).index();
                    if a < kv.knots()[j] || z > kv.knots()[j + 1] {
                        return Err(refuse("a piece of the image straddles a knot of the net"));
                    }
                }
            }
        }
        (FramedCarrier::Circle { .. }, Curve3::Circle { .. })
            if matches!(surface, Surface::Sphere { .. }) => {}
        _ => {
            return Err(refuse(
                "the stored carrier is not the edge's carrier kind (a spline's net, or a \
                 circle on a sphere)",
            ));
        }
    }

    // ---- Check 2: the parameter interval, and the period headroom. ----
    let span_escalated = |cause| PcurveCertifyError::Escalated {
        check: PcurveCheck::ParamSpan,
        sample: 0,
        cause,
    };
    let rate = super::param_rate_gate(carrier, band).map_err(span_escalated)?;
    match decide(
        "pcurve_interval_forward",
        Margin::metered(t1 - t0, rate),
        band,
    )
    .map_err(span_escalated)?
    {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Err(PcurveCertifyError::IntervalNotForward),
    }
    let boxed = image.chart_box(t0, t1);
    if curved {
        // The period gate reads the sweep on a cover finer than the
        // pieces (`ProjectedImage::sweep_box`); the arms read the box.
        let (u_arm, _) = super::chart_arms_at(surface, &boxed)?;
        let swept = image.sweep_box(t0, t1);
        let mut gates = vec![(
            swept.u_max - swept.u_min,
            u_arm.get(),
            PcurveCheck::AzimuthPeriod,
            PcurveCertifyError::AzimuthPeriodExceeded,
        )];
        if let Surface::Torus { minor_radius, .. } = *surface {
            gates.push((
                swept.v_max - swept.v_min,
                minor_radius,
                PcurveCheck::TubePeriod,
                PcurveCertifyError::TubePeriodExceeded,
            ));
        }
        for (extent, arm, check, exceeded) in gates {
            match decide(
                "pcurve_azimuth_period",
                Margin::levered(T::tau() - extent, arm),
                band,
            )
            .map_err(|cause| PcurveCertifyError::Escalated {
                check,
                sample: 0,
                cause,
            })? {
                Sign::Positive | Sign::Zero => {}
                Sign::Negative => return Err(exceeded),
            }
        }
    }

    // ---- Check 3: the schedule, the envelope's cross-check. ----
    let mut max_residual = T::zero();
    let samples = match T::WITNESS {
        geom_core::Witness::Inexact => {
            super::schedule_residuals(
                Record::CrossCheck,
                pcurve,
                t0,
                t1,
                carrier,
                surface,
                band,
                &mut max_residual,
            )?;
            crate::certify::CERT_SAMPLES
        }
        geom_core::Witness::Exact => 0,
    };

    // ---- Check 4: the envelope, the whole certified statement. ----
    let terms = projected_envelope(image, t0, t1, &boxed, carrier, surface, band, lane)?;
    let envelope = terms.total();
    match decide("pcurve_envelope", Margin::of(envelope), band) {
        Ok(Sign::Zero) => {}
        refused => {
            let check = terms
                .first_over(band)
                .map_or(PcurveCheck::Envelope, PcurveCheck::EnvelopeTerm);
            return Err(match refused {
                Err(cause) => PcurveCertifyError::Escalated {
                    check,
                    sample: 0,
                    cause,
                },
                Ok(_) => PcurveCertifyError::ResidualExceeded { check, sample: 0 },
            });
        }
    }
    Ok(super::PcurveCertificate::closed(
        samples,
        max_residual,
        envelope,
        EnvelopeStatement::MapResidualProjected,
    ))
}
