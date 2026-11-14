//! **A chart image's speed along each chart axis**
//! ([`Pcurve::speed_bound`](super::Pcurve::speed_bound)): `(s_u, s_v)` with `|u′(t)| ≤ s_u` and
//! `|v′(t)| ≤ s_v` over a carrier span, in chart units per unit of the
//! carrier's own parameter. It is the image's third fact beside its
//! value ([`Pcurve::eval`](super::Pcurve::eval)) and its box ([`Pcurve::chart_box`](super::Pcurve::chart_box)); a
//! mesher's chord pass reads it to keep every boundary chord's chart step
//! within a lattice step (`n ≥ s·Δt/h` per axis).
//!
//! Every bound is a supremum over the span or over the image's whole
//! period, never a sample. Plain `f64`, as the chord counts it feeds:
//! each is conservative by its own slack, and its rounding sits far
//! below the `ceil` of the count it sizes.
//!
//! # Per form
//!
//! - **Harmonic** `P = p0 + pa·cos t + pb·sin t + pl·t`: per axis
//!   `|−pa·sin t + pb·cos t + pl| ≤ √(pa² + pb²) + |pl|`, attained where
//!   the sinusoid peaks with the linear term's sign (exact when `pl` or
//!   the sinusoid vanishes: every rim and meridian).
//! - **Iso line** `P = p0 + pl·t`: `|pl|` per axis, exact.
//! - **Arc rim**: `|pd|` per axis times the chart parameter's rate
//!   ([`iso_arc_rate`]), attained at a sub-arc end.
//! - **General** spline image: the hull of its differenced control net
//!   ([`general_speeds`]), a convexity fact.
//! - **Fitted**: no bound is wired; it refuses ([`NoSpeedBound::Fitted`]).
//! - **Spiric** ([`spiric_speeds`]), **focal section**
//!   ([`focal_section_speeds`]) and **projected** ([`projected_speeds`])
//!   images: closed forms derived where they are defined.
//!
//! The rows that pin each bound against the sampled image are
//! `speed_rows`.

use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::{KnotVector, SplineCoeffs};
use geom_core::{Bounds, Vec2, Vec3};

use super::projected::{FramedCarrier, ProjectedChart, ProjectedImage};
use super::{Channel, FocalImage, SpiricImage, harmonic_form_of, iso_arc_skew, iso_arc_spans};

/// Why an image has no certified per-axis speed bound
/// ([`Pcurve::speed_bound`](super::Pcurve::speed_bound)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoSpeedBound {
    /// A fitted image: no certified speed bound is wired for it.
    Fitted,
    /// A general image whose weights are not one constant: its
    /// derivative is a quotient, not the differenced net.
    RationalGeneral,
    /// A general image read past its own knot domain, where its
    /// evaluator extends the end span's polynomial and no hull bounds it.
    OutsideGeneralDomain,
    /// A spline net whose derivative bound does not certify.
    NetUnbounded,
    /// An arc rim whose breaks hold no sub-arc.
    NoSubArc,
    /// The image's own parameters lie outside the regime its closed
    /// form holds in: a spiric past the two-oval regime (`|d| ≥ R − r`),
    /// a focal section with `|β| ≥ 1`.
    OffRegime,
    /// A projected image whose distance from its chart's singular set
    /// (a sphere's or a torus's axis, a cone's, a torus's core circle)
    /// has no positive lower bound over the span: the chart's inverse is
    /// not Lipschitz there.
    ThroughSingularSet,
}

/// The per-axis speed bounds of a harmonic image's two channels: each
/// axis' `√(pa² + pb²) + |pl|`.
pub(super) fn harmonic_speeds(u: Channel<f64>, v: Channel<f64>) -> (f64, f64) {
    let (_, pa, pb, pl) = harmonic_form_of(u, v);
    (pa.x.hypot(pb.x) + pl.x.abs(), pa.y.hypot(pb.y) + pl.y.abs())
}

/// An arc rim's per-axis bounds: `|pd|` per axis times [`iso_arc_rate`].
pub(super) fn iso_arc_speeds(
    pd: Vec2<f64>,
    angle: f64,
    breaks: &KnotVector,
    ends: &[f64],
) -> Result<(f64, f64), NoSpeedBound> {
    let spans = iso_arc_spans(breaks);
    if spans == 0 {
        return Err(NoSpeedBound::NoSubArc);
    }
    let rate = iso_arc_rate(angle, spans, ends);
    Ok((pd.x.abs() * rate, pd.y.abs() * rate))
}

/// **An arc rim's `sup |dg/dt|`**, a closed form over the whole span
/// with no sampling. With `s = ½ + tan(φ/2)/(2·tan(h/4))` and
/// `g = (k + s)/m`, `dg/dt = sec²(φ/2) / (4·m·tan(h/4))`, maximal at the
/// sub-arc ends (`|φ| = h/2`), where it is `1/(2·m·sin(h/2))`. A row at
/// unequal end weights (`ends`) moves the chart parameter through
/// `σ = λs/(λs + 1 − s)` on each sub-arc, whose `dσ/ds` lies between
/// `λ` and `1/λ`, so the sup is scaled by the largest of those.
pub(super) fn iso_arc_rate(angle: f64, spans: usize, ends: &[f64]) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let m = spans as f64;
    let skew = ends
        .windows(2)
        .map(|e| {
            let lambda = iso_arc_skew(e[0], e[1]);
            lambda.max(1.0 / lambda)
        })
        .fold(1.0_f64, f64::max);
    skew / (2.0 * m * (angle / (2.0 * m)).sin())
}

/// The per-axis speed sups `(s_u, s_v)` of a **general curve-in-UV**
/// image over `params`.
///
/// **The fact.** For `P(t) = Σ_j N_{j,p}(t)·P_j` on a clamped vector,
/// `P′(t) = Σ_j N_{j,p−1}(t)·Q_j` with
/// `Q_j = p·(P_{j+1} − P_j)/(u_{j+p+1} − u_{j+1})`, and the `N_{j,p−1}`
/// are a nonnegative partition of unity, so every value of `u′` lies in
/// the hull of the `Q^u_j`: `max_j |Q^u_j|` is a certified `sup|u′|`,
/// and likewise in `v`. The differencing and the hull are
/// [`SplineCoeffs::derivative_domain_hull`]'s.
///
/// **A sup, not an attained maximum.** It is attained where the winning
/// coefficient's basis function reaches 1 (a clamped end, or an interior
/// knot of a degree-`p` image); on an interior coefficient of a higher
/// degree it is a genuine over-estimate.
///
/// **The domain.** The hull is over the image's whole knot domain, and
/// `params` must lie inside it: past the domain `eval`/`deriv` extend
/// the end span's polynomial, which no hull of the net bounds. A proper
/// sub-range is admitted and over-bounded.
///
/// **Constant weights only.** A constant weight vector cancels out of
/// the rational basis, so such an image is its polynomial wrap; a
/// varying one's derivative is a quotient and refuses.
///
/// A degree-0 image and an interior multiplicity above the degree are
/// refused by [`KnotVector::clamped`] at construction. Multiplicity `p`
/// (a C⁰ kink) is admitted: the bound is a Lipschitz statement.
pub(super) fn general_speeds(
    image: &geom::NurbsCurve2<f64>,
    (t0, t1): (f64, f64),
) -> Result<(f64, f64), NoSpeedBound> {
    if image.constant_weight().is_none() {
        return Err(NoSpeedBound::RationalGeneral);
    }
    let kv = image.knots_f64_interim();
    let (d0, d1) = kv.domain();
    if !(t0 >= d0 && t1 <= d1) {
        return Err(NoSpeedBound::OutsideGeneralDomain);
    }
    let mut speeds = [f64::NAN; 2];
    for (axis, s) in speeds.iter_mut().enumerate() {
        let coeffs: Vec<Interval> = image
            .control()
            .iter()
            .map(|pt| Interval::point(if axis == 0 { pt.x } else { pt.y }))
            .collect();
        // `mag` is NaN on a refusal, which leaves as the refusal below.
        *s = kv
            .with_coeffs(&coeffs)
            .map_or_else(Interval::refused, SplineCoeffs::derivative_domain_hull)
            .mag()
            .next_up();
    }
    let [su, sv] = speeds;
    if su.is_finite() && sv.is_finite() {
        Ok((su, sv))
    } else {
        Err(NoSpeedBound::NetUnbounded)
    }
}

/// The per-axis bounds of a **spiric** image (`R`, `r`, `d` the
/// carrier's; `f(t) = √(ρ² − d²)`, `ρ = R + r·cos t`), over the whole
/// period, in the two-oval regime `|d| < R − r` where
/// `f_min = √((R − r)² − d²) > 0`.
///
/// With `f′ = −r·sin t·ρ/f`:
///
/// - **Wall** `u = u0 + σμ·atan2(f, d)`, `v = v0 + σ·t`:
///   `u′ = σμ·d·f′/(f² + d²) = −σμ·d·r·sin t/(ρ·f)` (since
///   `f² + d² = ρ²`), bounded with `sin t` and `f` paired
///   ([`wall_azimuth_rate`]); `|v′| = |σ|`, exact.
/// - **Cap** `P = p0 + pm·f + pa·sin t`: `P′ = pm·f′ + pa·cos t`, so per
///   axis `|pm|·sup|f′| + |pa|` ([`radial_rate`]); the two terms peak
///   apart, so it is not attained.
pub(super) fn spiric_speeds(
    (major, minor, offset): (f64, f64, f64),
    image: &SpiricImage<f64>,
) -> Result<(f64, f64), NoSpeedBound> {
    let (f_min, _) = geom::spiric_f_range(major, minor, offset);
    if f_min.is_nan() || f_min <= 0.0 {
        return Err(NoSpeedBound::OffRegime);
    }
    Ok(match *image {
        SpiricImage::Wall { sense, torus, .. } => (
            (sense * torus.mu::<f64>()).abs() * wall_azimuth_rate((major, minor, offset)),
            sense.abs(),
        ),
        SpiricImage::Cap { pm, pa, .. } => {
            let s = radial_rate((major, minor, offset));
            (pm.x.abs() * s + pa.x.abs(), pm.y.abs() * s + pa.y.abs())
        }
    })
}

/// The pieces of `x = 1 + cos t ∈ [0, 2]` [`paired_sup`] reads.
const WALL_PIECES: u32 = 32;

/// **`sup sin²t/f² · w(ρ)` over a period, `sin t` and `f` paired**, for a
/// weight `w` positive and monotone in `ρ = R + r·cos t`, `f² = ρ² − d²`.
///
/// With `x = 1 + cos t`, `sin²t = x·(2 − x)` and
/// `f² = (ρ − |d|)(ρ + |d|)` with `ρ − |d| = δ + r·x`,
/// `δ = R − r − |d| > 0` in the two-oval regime, so
///
/// ```text
/// sin²t/f² = (2 − x)·φ(x) / (ρ + |d|),   φ(x) = x/(δ + r·x)
/// ```
///
/// and `φ` is increasing. On each of [`WALL_PIECES`] equal pieces
/// `[x_a, x_b]` the bound is `(2 − x_a)·φ(x_b)/(ρ_a + |d|)` times `w`'s
/// sup over `[ρ_a, ρ_b]` (`ρ_a = R − r + r·x_a` the piece's least `ρ`).
/// Pairing `sin t` with `f` is what holds near a tangent cut, where both
/// vanish at `t = π` (`props::spiric`'s `piece_speed` reads the
/// carrier's speed the same way); read apart, the bound grows without
/// limit there.
fn paired_sup((major, minor, offset): (f64, f64, f64), w: impl Fn(f64, f64) -> f64) -> f64 {
    let d = offset.abs();
    let delta = major - minor - d;
    let step = 2.0 / f64::from(WALL_PIECES);
    (0..WALL_PIECES)
        .map(|k| {
            let (xa, xb) = (f64::from(k) * step, f64::from(k + 1) * step);
            let phi = xb / minor.mul_add(xb, delta);
            let (rho_a, rho_b) = (
                minor.mul_add(xa, major - minor),
                minor.mul_add(xb, major - minor),
            );
            (2.0 - xa) * phi / (rho_a + d) * w(rho_a, rho_b)
        })
        .fold(0.0, f64::max)
}

/// **`sup|atan2(f, d)′|` over a period**: `|d|·f′/ρ²` with
/// `f′ = −r·sin t·ρ/f`, so `r·|d|·√(sup sin²t/(f²·ρ²))` ([`paired_sup`]
/// with `w = 1/ρ_a²`).
fn wall_azimuth_rate(spiric: (f64, f64, f64)) -> f64 {
    let (_, minor, offset) = spiric;
    minor * offset.abs() * paired_sup(spiric, |rho_a, _| 1.0 / rho_a.powi(2)).sqrt()
}

/// **`sup|f′|` over a period**: `r·√(sup sin²t·ρ²/f²)` ([`paired_sup`]
/// with `w = ρ_b²`).
fn radial_rate(spiric: (f64, f64, f64)) -> f64 {
    let (_, minor, _) = spiric;
    minor * paired_sup(spiric, |_, rho_b| rho_b.powi(2)).sqrt()
}

/// The per-axis bounds of a **focal section** image (variant docs on
/// [`Pcurve::FocalSection`](super::Pcurve::FocalSection)), over the whole period.
///
/// The azimuth `u = u0 + σ·(E + 2·atan2(β·sin E, 1 − β·cos E))` is the
/// projected ellipse's true anomaly `ν` at eccentric anomaly `E = t − t0`:
/// differentiating, `ν′ = 1 + 2(β·cos E − β²)/(1 − 2β·cos E + β²)
/// = (1 − β²)/(1 − 2β·cos E + β²)`, largest where `β·cos E = |β|`:
/// `(1 + |β|)/(1 − |β|)`. That is attained (at the vertex nearer the
/// axis), so `|u′| ≤ |σ|·(1 + |β|)/(1 − |β|)` is exact over a period.
/// The second channel `v0 + va·cos t + vb·sin t + vl·t` is harmonic:
/// `|v′| ≤ √(va² + vb²) + |vl|`, exact on both minted instances (a cone
/// section has `vl = 0`, a Villarceau circle `va = vb = 0`).
pub(super) fn focal_section_speeds(image: &FocalImage<f64>) -> Result<(f64, f64), NoSpeedBound> {
    let beta = image.beta.abs();
    if beta.is_nan() || beta >= 1.0 {
        return Err(NoSpeedBound::OffRegime);
    }
    Ok((
        image.sense.abs() * (1.0 + beta) / (1.0 - beta),
        image.va.hypot(image.vb) + image.vl.abs(),
    ))
}

/// The per-axis bounds of a **projected** image `P = ψ(C(t))` over
/// `params`: the carrier's speed `sup|C′|` over each channel's least
/// lever arm, in the chart frame (`projected`'s module docs: `z` on the
/// axis, `ρ = √(x² + y²)`).
///
/// Each channel is a function of the chart-frame point whose gradient
/// has a closed-form norm, so `|channel′| ≤ |∇channel|·|C′|`:
///
/// | chart | `u` | `|∇u|` | `v` | `|∇v|` |
/// |---|---|---|---|---|
/// | plane | `x` | 1 | `y` | 1 |
/// | cylinder | azimuth | `1/ρ` | `z` | 1 |
/// | cone | azimuth | `1/ρ` | `z·cos α + σρ·sin α` | 1 |
/// | sphere | azimuth | `1/ρ` | `atan2(z, ρ)` | `1/|p|` |
/// | torus | azimuth | `1/ρ` | tube angle | `1/m`, `m = √((ρ − R)² + z²)` |
///
/// so the lever arms read are lower bounds on `ρ`, `|p|` and `m` over
/// the span ([`Levers`]). The deck map's `v_sign` moves no magnitude.
///
/// **How tight.** Each bound is attained only where the carrier moves
/// along its channel's gradient at the least lever; a channel the carrier
/// moves across reads the whole `|C′|` (a flat circle's height on a
/// cylinder moves at 0 against `|C′|`). The azimuth reads `|C′|` where
/// `|C′_xy|` would do. On a circle of a sphere both bounds are attained:
/// `ρ_c/(r·cos v)` where the arc turns in latitude, and `ρ_c/r`.
pub(super) fn projected_speeds(
    image: &ProjectedImage<f64>,
    params: (f64, f64),
) -> Result<(f64, f64), NoSpeedBound> {
    let carrier = image.carrier();
    let speed = carrier_speed(carrier)?;
    let levers = Levers::of(carrier, params);
    let over = |lever: f64| {
        if lever > 0.0 {
            Ok(speed / lever)
        } else {
            Err(NoSpeedBound::ThroughSingularSet)
        }
    };
    match *image.chart() {
        ProjectedChart::Plane => Ok((speed, speed)),
        ProjectedChart::Cylinder | ProjectedChart::Cone { .. } => Ok((over(levers.rho)?, speed)),
        ProjectedChart::Sphere => Ok((over(levers.rho)?, over(levers.norm)?)),
        ProjectedChart::Torus { major } => Ok((over(levers.rho)?, over(levers.tube(major))?)),
    }
}

/// `sup|C′|` of a framed carrier: a conic's largest eigenvalue of the
/// Gram matrix of `(a, b)`, a line's `|dir|`, a net's certified quotient
/// bound ([`geom::NurbsCurve3::speed_upper_bound`], rational nets
/// included).
fn carrier_speed(carrier: &FramedCarrier<f64>) -> Result<f64, NoSpeedBound> {
    match carrier {
        FramedCarrier::Conic { a, b, .. } => {
            let (aa, bb, ab) = (a.dot(*a), b.dot(*b), a.dot(*b));
            let speed_sq = 0.5 * (aa + bb) + (0.5 * (aa - bb)).hypot(ab);
            if speed_sq.is_finite() {
                Ok(speed_sq.sqrt())
            } else {
                Err(NoSpeedBound::NetUnbounded)
            }
        }
        FramedCarrier::Line { dir, .. } => Ok(dir.norm()),
        FramedCarrier::Net(net) => {
            let bound = net.map_scalar(Interval::point).speed_upper_bound().get();
            if bound.is_certified() && bound.hi().is_finite() {
                Ok(bound.hi())
            } else {
                Err(NoSpeedBound::NetUnbounded)
            }
        }
    }
}

/// Lower bounds over a span on a framed carrier's distance from the
/// chart axis (`rho`) and from the chart origin (`norm`), with the range
/// of its height `z` and an upper bound on `ρ`. A bound that fails
/// (a NaN) reads as `0`, never as a lever.
#[derive(Clone, Copy, Debug)]
struct Levers {
    rho: f64,
    rho_hi: f64,
    norm: f64,
    z: (f64, f64),
}

impl Levers {
    fn of(carrier: &FramedCarrier<f64>, params: (f64, f64)) -> Self {
        match carrier {
            FramedCarrier::Conic {
                centre: c, a, b, ..
            } => conic_levers(*c, *a, *b, params),
            FramedCarrier::Line { point, dir, .. } => line_levers(*point, *dir, params),
            FramedCarrier::Net(net) => net_levers(net.control()),
        }
    }

    /// A lower bound on the distance from a torus's core circle
    /// (radius `major` in the plane `z = 0`): the distance from `(R, 0)`
    /// to the box `[ρ_lo, ρ_hi] × [z_lo, z_hi]` of the meridian half-plane.
    fn tube(self, major: f64) -> f64 {
        let gap = |lo: f64, hi: f64, x: f64| (lo - x).max(x - hi).max(0.0);
        gap(self.rho, self.rho_hi, major).hypot(gap(self.z.0, self.z.1, 0.0))
    }
}

/// A conic `c + a·cos t + b·sin t`'s levers. `|p|²` is
/// `K + 2(c·a)·cos t + 2(c·b)·sin t + ½(|a|² − |b|²)·cos 2t + (a·b)·sin 2t`
/// with `K = |c|² + ½(|a|² + |b|²)`, bounded by `K` less (or plus) its two
/// harmonics' amplitudes; `ρ²` is the same form on the `(x, y)` parts, and
/// also `|p|² − z²` with `z`'s range over the span in closed form
/// ([`harmonic_range`]). The larger of the two lower bounds on `ρ²` is
/// read: the second is exact on a circle of a sphere, the first on a
/// circle of a cylinder.
fn conic_levers(c: Vec3<f64>, a: Vec3<f64>, b: Vec3<f64>, params: (f64, f64)) -> Levers {
    let quadratic = |c: Vec3<f64>, a: Vec3<f64>, b: Vec3<f64>| {
        let (aa, bb, ab) = (a.dot(a), b.dot(b), a.dot(b));
        let k = 0.5f64.mul_add(aa + bb, c.dot(c));
        let (first, second) = (c.dot(a).hypot(c.dot(b)), (0.5 * (aa - bb)).hypot(ab));
        (k - 2.0 * first - second, k + 2.0 * first + second)
    };
    let flat = |v: Vec3<f64>| Vec3::new(v.x, v.y, 0.0);
    let (p_sq_min, p_sq_max) = quadratic(c, a, b);
    let (xy_sq_min, xy_sq_max) = quadratic(flat(c), flat(a), flat(b));
    let z = harmonic_range(c.z, a.z, b.z, params);
    let rho_sq = (p_sq_min - z.0.abs().max(z.1.abs()).powi(2)).max(xy_sq_min);
    Levers {
        rho: root(rho_sq),
        rho_hi: xy_sq_max.min(p_sq_max).max(0.0).sqrt(),
        norm: root(p_sq_min),
        z,
    }
}

/// A line `point + dir·t`'s levers over `[t0, t1]`: each squared
/// distance is a convex quadratic in `t`, least at its vertex clamped to
/// the span, and `ρ` largest at an end.
fn line_levers(point: Vec3<f64>, dir: Vec3<f64>, (t0, t1): (f64, f64)) -> Levers {
    let flat = |v: Vec3<f64>| Vec3::new(v.x, v.y, 0.0);
    let least = |p: Vec3<f64>, d: Vec3<f64>| {
        let dd = d.dot(d);
        let t = if dd > 0.0 {
            (-p.dot(d) / dd).clamp(t0, t1)
        } else {
            t0
        };
        (p + d * t).norm()
    };
    let (z0, z1) = (point.z + dir.z * t0, point.z + dir.z * t1);
    Levers {
        rho: least(flat(point), flat(dir)),
        rho_hi: flat(point + dir * t0)
            .norm()
            .max(flat(point + dir * t1).norm()),
        norm: least(point, dir),
        z: (z0.min(z1), z0.max(z1)),
    }
}

/// A net's levers from its control points' box: a NURBS curve with
/// positive weights lies in its control hull, so in the hull's box, whose
/// nearest point to the axis or the origin bounds the curve's from below.
/// Over the whole domain, so over any span of it.
fn net_levers(control: &[geom_core::Point3<f64>]) -> Levers {
    let range = |f: fn(&geom_core::Point3<f64>) -> f64| {
        control
            .iter()
            .map(f)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), x| {
                (lo.min(x), hi.max(x))
            })
    };
    let (x, y, z) = (range(|p| p.x), range(|p| p.y), range(|p| p.z));
    let gap = |(lo, hi): (f64, f64)| lo.max(-hi).max(0.0);
    let far = |(lo, hi): (f64, f64)| lo.abs().max(hi.abs());
    let rho = gap(x).hypot(gap(y));
    Levers {
        rho,
        rho_hi: far(x).hypot(far(y)),
        norm: rho.hypot(gap(z)),
        z,
    }
}

/// `√x` of a lower bound on a square, `0` where the bound is not positive.
fn root(x: f64) -> f64 {
    if x > 0.0 { x.sqrt() } else { 0.0 }
}

/// The range of `c + a·cos t + b·sin t` over `[t0, t1]`: the ends, and
/// `c ± √(a² + b²)` where the interval may hold the phase of either
/// extremum ([`geom::periodic_window_may_hold`]).
#[must_use]
pub fn harmonic_range(c: f64, a: f64, b: f64, (t0, t1): (f64, f64)) -> (f64, f64) {
    use core::f64::consts::{PI, TAU};
    let at = |t: f64| c + a * t.cos() + b * t.sin();
    let (e0, e1) = (at(t0), at(t1));
    let (mut lo, mut hi) = (e0.min(e1), e0.max(e1));
    let (m, tau) = (a.hypot(b), b.atan2(a));
    if geom::periodic_window_may_hold((tau, tau), (t0, t1), TAU) {
        hi = c + m;
    }
    if geom::periodic_window_may_hold((tau + PI, tau + PI), (t0, t1), TAU) {
        lo = c - m;
    }
    (lo, hi)
}
