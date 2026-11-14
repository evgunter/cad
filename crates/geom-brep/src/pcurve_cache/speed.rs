//! **A chart image's speed along each chart axis**
//! ([`Pcurve::speed_bound`](super::Pcurve::speed_bound)): `(s_u, s_v)` with `|u′(t)| ≤ s_u` and
//! `|v′(t)| ≤ s_v` over a carrier span, in chart units per unit of the
//! carrier's own parameter. It is the image's third fact beside its
//! value ([`Pcurve::eval`](super::Pcurve::eval)) and its box ([`Pcurve::chart_box`](super::Pcurve::chart_box)); a
//! mesher's chord pass reads it to keep every boundary chord's chart step
//! within a lattice step (`n ≥ s·Δt/h` per axis).
//!
//! Every bound is a supremum over the span or over the image's whole
//! period, never a sample. The spiric, focal-section and projected bounds
//! and the general image's hull are read in outward-rounded interval
//! arithmetic, so each dominates in floats as well as in reals. The
//! harmonic, iso-line and arc-rim bounds are their closed forms in plain
//! `f64`: each is attained, so its rounding can leave it an ulp either
//! side of the true sup, a relative `ε` of the count it sizes.
//!
//! A span is read forward: `t₀ ≤ t₁`, both finite, or the bound refuses
//! ([`NoSpeedBound::NotASpan`]).
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
    /// A spline image or carrier read past its own knot domain, where
    /// its evaluator extends the end span's polynomial and no hull bounds
    /// it.
    OutsideDomain,
    /// The span is not one: reversed, or not finite.
    NotASpan,
    /// A closed-form image whose coefficients give no finite bound.
    NonFinite,
    /// A spline net whose derivative bound does not certify.
    NetUnbounded,
    /// An arc rim whose breaks hold no sub-arc.
    NoSubArc,
    /// The image's own parameters lie outside the regime its closed
    /// form holds in: a spiric off a ring torus's two-oval regime
    /// (`0 < r < R`, `|d| < R − r`), a focal section with `|β| ≥ 1`.
    OffRegime,
    /// A projected image whose lever arm (its distance from the chart's
    /// singular set: a cylinder's, cone's, sphere's or torus's axis, a
    /// torus's core circle) reads no positive lower bound over the span.
    /// The bound read is the closed form's for a conic or a line, and for
    /// a net its control box's over the whole domain, which can reach the
    /// singular set where the curve does not.
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
        return Err(NoSpeedBound::OutsideDomain);
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

/// The pieces of a period [`spiric_breaks`] cuts a span into.
const SPIRIC_PIECES: u32 = 64;

/// The steps of `√2` [`spiric_breaks`] grades toward each `t = π + 2kπ`.
const SPIRIC_GRADE: i32 = 60;

/// The breaks of the pieces [`spiric_speeds`] reads over `[t0, t1]`: a
/// period's [`SPIRIC_PIECES`] equal pieces, and toward each
/// `t = π + 2kπ` in the span pieces shrinking by `√2`, [`SPIRIC_GRADE`]
/// times. There
/// `ρ` is least, and the image's rates peak within `√(2(R − r)/r)` of it,
/// which on a fat tube is far narrower than an equal piece: a piece read
/// whole pairs its least `ρ` with its widest `sin t`.
fn spiric_breaks((t0, t1): (f64, f64)) -> Vec<f64> {
    use core::f64::consts::{PI, TAU};
    let step = TAU / f64::from(SPIRIC_PIECES);
    let mut breaks = vec![t0, t1];
    let first = (t0 / step).floor();
    let mut k = first;
    while k * step < t1 {
        breaks.push(k * step);
        k += 1.0;
    }
    let mut m = ((t0 - PI) / TAU).floor();
    while m.mul_add(TAU, PI) - step < t1 {
        let centre = m.mul_add(TAU, PI);
        breaks.push(centre);
        for g in 1..=SPIRIC_GRADE {
            let reach = step * core::f64::consts::FRAC_1_SQRT_2.powi(g);
            breaks.extend([centre - reach, centre + reach]);
        }
        m += 1.0;
    }
    breaks.retain(|&b| b >= t0 && b <= t1);
    breaks.sort_by(f64::total_cmp);
    breaks.dedup();
    breaks
}

/// The per-axis bounds of a **spiric** image (`R`, `r`, `d` the
/// carrier's; `f(t) = √(ρ² − d²)`, `ρ = R + r·cos t`) over the span, on a
/// ring torus in the two-oval regime `0 < r < R`, `|d| < R − r`.
///
/// With `f′ = −r·sin t·ρ/f`:
///
/// - **Wall** `u = u0 + σμ·atan2(f, d)`, `v = v0 + σ·t`:
///   `u′ = σμ·d·f′/(f² + d²) = −σμ·d·r·sin t/(ρ·f)` (since
///   `f² + d² = ρ²`); `|v′| = |σ|`, exact.
/// - **Cap** `P = p0 + pm·f + pa·sin t`: `P′ = pm·f′ + pa·cos t`, so per
///   axis `|pm|·sup|f′| + |pa|`; the two terms peak apart, so it is not
///   attained.
///
/// `sup|f′|` and `sup|atan2(f, d)′|` are `props::spiric`'s window rates
/// over the pieces [`spiric_breaks`] cuts (`image_rates`), which read
/// `sin t` and `f` together near a tangent cut and apart elsewhere, in
/// certified arithmetic.
pub(super) fn spiric_speeds(
    (major, minor, offset): (f64, f64, f64),
    image: &SpiricImage<f64>,
    span: (f64, f64),
) -> Result<(f64, f64), NoSpeedBound> {
    let ring = minor > 0.0 && major > minor && offset.abs() < major - minor;
    if !ring {
        return Err(NoSpeedBound::OffRegime);
    }
    let (df, du) = crate::props::spiric::image_rates((major, minor, offset), &spiric_breaks(span))
        .ok_or(NoSpeedBound::OffRegime)?;
    let i = Interval::point;
    Ok(match *image {
        SpiricImage::Wall { sense, torus, .. } => (
            (i((sense * torus.mu::<f64>()).abs()) * i(du)).hi(),
            sense.abs(),
        ),
        SpiricImage::Cap { pm, pa, .. } => (
            (i(pm.x.abs()) * i(df) + i(pa.x.abs())).hi(),
            (i(pm.y.abs()) * i(df) + i(pa.y.abs())).hi(),
        ),
    })
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
    let i = Interval::point;
    let (u, v) = (
        i(image.sense.abs()) * (i(1.0) + i(beta)) / (i(1.0) - i(beta)),
        hyp(i(image.va), i(image.vb)) + i(image.vl.abs()),
    );
    Ok((finite(u.hi())?, finite(v.hi())?))
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
    if let FramedCarrier::Net(net) = carrier {
        let (d0, d1) = net.domain();
        if !(params.0 >= d0 && params.1 <= d1) {
            return Err(NoSpeedBound::OutsideDomain);
        }
    }
    let speed = carrier_speed(carrier)?;
    let levers = Levers::of(carrier, params);
    let over = |lever: f64| {
        if lever > 0.0 {
            Ok((Interval::point(speed) / Interval::point(lever)).hi())
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
            let (aa, bb, ab) = (dot(*a, *a), dot(*b, *b), dot(*a, *b));
            let half = Interval::point(0.5);
            finite((half * (aa + bb) + hyp(half * (aa - bb), ab)).sqrt().hi())
        }
        FramedCarrier::Line { dir, .. } => finite(dot(*dir, *dir).sqrt().hi()),
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
/// chart axis (`rho`) and from the chart origin (`norm`), with an
/// enclosure of its height `z` and an upper bound on `ρ`, each holding in
/// floats. A bound that fails (a NaN) reads as `0`, never as a lever.
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
/// ([`harmonic_enclosure`]). The larger of the two
/// lower bounds on `ρ²` is read: the second is exact on a circle of a
/// sphere, the first on a circle of a cylinder. Near a sphere's pole the
/// second is a difference of two near-equal squares, so every term is
/// read in outward-rounded interval arithmetic, whose lower end holds
/// where an `f64` difference does not.
fn conic_levers(c: Vec3<f64>, a: Vec3<f64>, b: Vec3<f64>, params: (f64, f64)) -> Levers {
    let i = Interval::point;
    let quadratic = |c: Vec3<f64>, a: Vec3<f64>, b: Vec3<f64>| {
        let (aa, bb, ab) = (dot(a, a), dot(b, b), dot(a, b));
        let k = i(0.5) * (aa + bb) + dot(c, c);
        let swing = i(2.0) * hyp(dot(c, a), dot(c, b)) + hyp(i(0.5) * (aa - bb), ab);
        (k - swing, k + swing)
    };
    let flat = |v: Vec3<f64>| Vec3::new(v.x, v.y, 0.0);
    let (p_sq_min, p_sq_max) = quadratic(c, a, b);
    let (xy_sq_min, xy_sq_max) = quadratic(flat(c), flat(a), flat(b));
    let z = harmonic_enclosure(c.z, a.z, b.z, params);
    let z_sq = i(z.0.abs().max(z.1.abs())).sqr();
    let rho_sq = (p_sq_min - z_sq).lo().max(xy_sq_min.lo());
    Levers {
        rho: root_down(rho_sq),
        rho_hi: root_up(xy_sq_max.hi().min(p_sq_max.hi())),
        norm: root_down(p_sq_min.lo()),
        z,
    }
}

/// A line `point + dir·t`'s levers over `[t0, t1]`, in interval
/// arithmetic. A distance is least at the vertex of its convex quadratic
/// in `t`, where it is the line's distance from the axis or the origin,
/// `|p × d|/|d|` (a form that does not cancel); with the vertex outside
/// the span, at the nearer end. `ρ` is largest at an end.
fn line_levers(point: Vec3<f64>, dir: Vec3<f64>, (t0, t1): (f64, f64)) -> Levers {
    let i = Interval::point;
    let flat = |v: Vec3<f64>| Vec3::new(v.x, v.y, 0.0);
    let at = |p: Vec3<f64>, d: Vec3<f64>, t: f64| {
        let q = |x: f64, dx: f64| i(x) + i(dx) * i(t);
        (q(p.x, d.x).sqr() + q(p.y, d.y).sqr() + q(p.z, d.z).sqr()).sqrt()
    };
    let least = |p: Vec3<f64>, d: Vec3<f64>| {
        let ends = at(p, d, t0).lo().min(at(p, d, t1).lo());
        let dd = d.dot(d);
        let vertex = -p.dot(d) / dd;
        if dd > 0.0 && vertex > t0 && vertex < t1 {
            let cross = |x: f64, y: f64, dx: f64, dy: f64| i(x) * i(dy) - i(y) * i(dx);
            let pxd = (cross(p.y, p.z, d.y, d.z).sqr()
                + cross(p.z, p.x, d.z, d.x).sqr()
                + cross(p.x, p.y, d.x, d.y).sqr())
            .sqrt();
            (pxd / dot(d, d).sqrt()).lo().min(ends)
        } else {
            ends
        }
    };
    let z = |t: f64| i(point.z) + i(dir.z) * i(t);
    let (z0, z1) = (z(t0), z(t1));
    Levers {
        rho: least(flat(point), flat(dir)).max(0.0),
        rho_hi: at(flat(point), flat(dir), t0)
            .hi()
            .max(at(flat(point), flat(dir), t1).hi()),
        norm: least(point, dir).max(0.0),
        z: (z0.lo().min(z1.lo()), z0.hi().max(z1.hi())),
    }
}

/// A net's levers from its control points' box: a NURBS curve with
/// positive weights lies in its control hull, so in the hull's box, whose
/// nearest point to the axis or the origin bounds the curve's from below.
/// The box is the whole domain's, so the lever is too, whatever span is
/// asked: a net that runs near the axis anywhere reads that lever
/// everywhere, and a box reaching the axis refuses
/// ([`NoSpeedBound::ThroughSingularSet`]) though the curve may not.
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
    let i = Interval::point;
    let gap = |(lo, hi): (f64, f64)| i(lo.max(-hi).max(0.0));
    let far = |(lo, hi): (f64, f64)| i(lo.abs().max(hi.abs()));
    let rho = hyp(gap(x), gap(y)).lo();
    Levers {
        rho,
        rho_hi: hyp(far(x), far(y)).hi(),
        norm: hyp(i(rho), gap(z)).lo(),
        z,
    }
}

/// `u·v`, in interval arithmetic.
fn dot(u: Vec3<f64>, v: Vec3<f64>) -> Interval {
    let i = Interval::point;
    i(u.x) * i(v.x) + i(u.y) * i(v.y) + i(u.z) * i(v.z)
}

/// `√(x² + y²)`, in interval arithmetic.
fn hyp(x: Interval, y: Interval) -> Interval {
    (x.sqr() + y.sqr()).sqrt()
}

/// A finite bound, or [`NoSpeedBound::NonFinite`].
fn finite(x: f64) -> Result<f64, NoSpeedBound> {
    if x.is_finite() {
        Ok(x)
    } else {
        Err(NoSpeedBound::NonFinite)
    }
}

/// `√x` rounded down, of a lower bound on a square; `0` where the bound
/// is not positive.
fn root_down(x: f64) -> f64 {
    if x > 0.0 {
        Interval::point(x).sqrt().lo()
    } else {
        0.0
    }
}

/// `√x` rounded up, of an upper bound on a square.
fn root_up(x: f64) -> f64 {
    Interval::point(x.max(0.0)).sqrt().hi()
}

/// The range of `c + a·cos t + b·sin t` over `[t0, t1]`: the ends, and
/// `c ± √(a² + b²)` where the interval may hold the phase of either
/// extremum ([`harmonic_extremes`]).
#[must_use]
pub fn harmonic_range(c: f64, a: f64, b: f64, (t0, t1): (f64, f64)) -> (f64, f64) {
    let at = |t: f64| c + a * t.cos() + b * t.sin();
    let (e0, e1) = (at(t0), at(t1));
    let (mut lo, mut hi) = (e0.min(e1), e0.max(e1));
    let m = a.hypot(b);
    let (max, min) = harmonic_extremes(a, b, (t0, t1));
    if max {
        hi = c + m;
    }
    if min {
        lo = c - m;
    }
    (lo, hi)
}

/// [`harmonic_range`]'s reading in interval arithmetic: each end's value
/// enclosed through `props`' transcendental-free `(cos t, sin t)`, each
/// extremum's as `c ± √(a² + b²)`, so the answer encloses the range in
/// floats. Which extrema the span holds is [`harmonic_extremes`]' decision,
/// at `f64`.
fn harmonic_enclosure(c: f64, a: f64, b: f64, (t0, t1): (f64, f64)) -> (f64, f64) {
    let i = Interval::point;
    let at = |t: f64| {
        let (cos, sin) = crate::props::taylor::trig_range(t, t);
        i(c) + i(a) * cos + i(b) * sin
    };
    let (e0, e1) = (at(t0), at(t1));
    let (mut lo, mut hi) = (e0.lo().min(e1.lo()), e0.hi().max(e1.hi()));
    let m = hyp(i(a), i(b));
    let (max, min) = harmonic_extremes(a, b, (t0, t1));
    if max {
        hi = (i(c) + m).hi();
    }
    if min {
        lo = (i(c) - m).lo();
    }
    (lo, hi)
}

/// Whether `[t0, t1]` may hold the phase of `a·cos t + b·sin t`'s maximum,
/// and of its minimum ([`geom::periodic_window_may_hold`]).
fn harmonic_extremes(a: f64, b: f64, (t0, t1): (f64, f64)) -> (bool, bool) {
    use core::f64::consts::{PI, TAU};
    let tau = b.atan2(a);
    (
        geom::periodic_window_may_hold((tau, tau), (t0, t1), TAU),
        geom::periodic_window_may_hold((tau + PI, tau + PI), (t0, t1), TAU),
    )
}
