//! **A circle on a sphere chart**: how far the circle is from the
//! sphere, its chart image, and a certified bound on how far that image
//! strays from the circle — the general-circle lane's three objects, in
//! one home.
//!
//! # The circle against the sphere
//!
//! For the carrier `C(t) = c + a·cos t + b·sin t` and the sphere of
//! centre `s` and radius `R`, with `d = c − s`,
//!
//! ```text
//! |C(t) − s|² − R² = k₀ + k₁·cos t + k₂·sin t + k₃·cos 2t + k₄·sin 2t
//! k₀ = |d|² + (|a|² + |b|²)/2 − R²
//! k₁ = 2·d·a     k₂ = 2·d·b     k₃ = (|a|² − |b|²)/2     k₄ = a·b
//! ```
//!
//! exactly, for any `a`, `b` (the second harmonics vanish for an
//! orthonormal frame) — [`off_sphere_coefficients`]. Since
//! `| |p − s| − R | = | |p − s|² − R² | / (|p − s| + R) ≤ | |p − s|² − R² | / R`,
//! each coefficient over the lever `R` is a sound metre reading, inside
//! the sphere and out, and `Σ|kᵢ| / R` bounds the circle's distance from
//! the sphere over the whole circle ([`off_sphere_sup`]). Two readers:
//! the chart door's incidence trilean (`pcurve_sphere_chart_incident`)
//! and the certificate below.
//!
//! # The image, and the bound on it
//!
//! A general circle's chart image `g(t) = ψ(C(t))` — the chart
//! coordinates of the circle's radial projection onto the sphere — is
//! transcendental. The image stored is its **piecewise Hermite
//! interpolant** — quintic, matching `g`, `g'` and `g''` at both ends of
//! every span — on the carrier's own parameter, as a clamped quintic
//! whose interior knots have multiplicity five, so each span's six
//! controls are its Bézier controls ([`hermite_image`]). Spans are refined by TRISECTION, so no
//! node is a dyadic fraction of the arc and no certification sample is
//! a node: the schedule residual reads the image where it was NOT made
//! to agree.
//!
//! The certificate ([`span_bound`]) bounds, on every span of ANY image
//! in that form, `sup |S(P(t)) − C(t)|` by
//!
//! ```text
//! |S(P) − C| ≤ |S(P) − S(g)| + |S(g) − C|
//!            ≤ R·(|Pᵤ − gᵤ| + |Pᵥ − gᵥ|) + off_sphere_sup
//! |P − g|    ≤ max_k |P_k − G_k| + h⁶/46080 · sup|g⁽⁶⁾|      (per channel)
//! ```
//!
//! — the chart map's derivatives are `R·cos v` in the azimuth and `R`
//! in the polar angle (and `|cos v|` along the chart segment from `g` to
//! `P` is at most `ρ/|q| + |Pᵥ − gᵥ|`); the image's span is a quintic,
//! so `P` minus the Hermite interpolant `H` of `g` is the Bézier of the
//! control differences, bounded by their largest (partition of unity);
//! and `|H − g| ≤ (h/2)⁶/6!·sup|g⁽⁶⁾|` is the two-point quintic Hermite
//! remainder.
//!
//! **The sixth derivative, without a transcendental of `t` in the
//! bound.** In the sphere's chart frame (units of `R`) the circle is
//! `x, y, z`, each `X₀ + X_c·cos t + X_s·sin t`. The azimuth is
//! `u = arg w` with `w = x + i·y`, and the polar angle is
//! `v = arg(ρ + i·z)` with `ρ = √(w·w*)` (`w*` the conjugate-coefficient
//! twin), both the imaginary part of a logarithm of a function
//! holomorphic in `t`. On the disc `|ζ − t_c| ≤ λ` about the span's
//! centre, `|cos ζ − cos t_c|` and `|sin ζ − sin t_c|` are at most
//! `e^λ − 1 ≤ 1.3·λ`, and `|sin ζ|, |cos ζ| ≤ e^λ ≤ 1.65` (`λ ≤ ½`), so
//! with `K = |X_c + iY_c| + |X_s + iY_s|` and `Z₁ = |Z_c| + |Z_s|`:
//!
//! ```text
//! |w|, |w*| ≥ m = ρ_c − 1.3·K·λ          |w'| ≤ 1.65·K
//! |ρ'| ≤ 1.65·K·W/m  (W = ρ_c + 1.3·K·λ)   |z'| ≤ 1.65·Z₁
//! |ρ + iz| ≥ q_c − λ·(|ρ'| + 1.3·Z₁)
//! ```
//!
//! and Cauchy's estimate on a disc of radius `δ = λ − h/2` about any
//! point of the span bounds `|(log f)⁽⁶⁾| ≤ 5!·sup|f'/f| / δ⁵`. The radii
//! are chosen so that `m ≥ ρ_c/2` and `|ρ + iz| ≥ q_c/2`, so the bound is
//! finite exactly when the span is short against its distance from the
//! chart's poles (`ρ_c`, the circle's distance from the polar axis). An
//! arc that runs into a pole has no span short enough, and refuses.
//!
//! The bound is plain lane arithmetic at `T` — `±`, `×`, `÷`, `√` on
//! the circle's and sphere's data, and the jets at the span's ends,
//! which is the same evaluation the schedule residual makes — so at the
//! interval scalar it encloses its own value.

use geom::{Curve3, NurbsCurve2, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Real, Vec3};

/// The coefficients `[k₀, k₁, k₂, k₃, k₄]` of `|C(t) − s|² − R²` for
/// `C(t) = s + d + a·cos t + b·sin t` (module docs) — the one home of
/// that expansion.
pub(crate) fn off_sphere_coefficients<T: Real>(
    d: Vec3<T>,
    a: Vec3<T>,
    b: Vec3<T>,
    radius: T,
) -> [T; 5] {
    let two = T::from_f64(2.0);
    let (aa, bb) = (a.dot(a), b.dot(b));
    [
        d.dot(d) + (aa + bb) / two - radius.powi(2),
        two * d.dot(a),
        two * d.dot(b),
        (aa - bb) / two,
        a.dot(b),
    ]
}

/// `Σ|kᵢ| / R`: a bound in metres on `| |C(t) − s| − R |` over the
/// whole circle (module docs).
pub(crate) fn off_sphere_sup<T: Real>(coefficients: [T; 5], radius: T) -> T {
    coefficients
        .into_iter()
        .fold(T::zero(), |sum, k| sum + k.abs())
        / radius
}

/// The circle in the sphere's chart frame, in units of the sphere's
/// radius: each of `x, y, z` as `[X₀, X_c, X_s]` (module docs).
#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartFrame<T: Real> {
    x: [T; 3],
    y: [T; 3],
    z: [T; 3],
    /// The sphere's radius, metres.
    radius: T,
    /// `|X_c + iY_c| + |X_s + iY_s|`.
    k: T,
    /// `|Z_c| + |Z_s|`.
    z1: T,
    /// [`off_sphere_sup`] of the circle, metres.
    off_sphere: T,
}

/// The chart image's value and first two derivatives at one parameter,
/// with the frame's `x, y` there (for the azimuth gap, [`azimuth_gap`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Jet<T: Real> {
    x: T,
    y: T,
    /// `atan2(y, x)` — the principal azimuth.
    u: T,
    v: T,
    du: T,
    dv: T,
    ddu: T,
    ddv: T,
}

/// `Im(n/d)` and `Re(n/d)` for complex `n = (nr, ni)`, `d = (dr, di)`.
fn complex_div<T: Real>((nr, ni): (T, T), (dr, di): (T, T)) -> (T, T) {
    let den = dr.powi(2) + di.powi(2);
    ((nr * dr + ni * di) / den, (ni * dr - nr * di) / den)
}

/// `Im(F″/F − (F′/F)²)` and `Im(F′/F)`: the first two derivatives of
/// `arg F` along `t`.
fn arg_derivatives<T: Real>(f: (T, T), df: (T, T), ddf: (T, T)) -> (T, T) {
    let (qr, qi) = complex_div(df, f);
    let (_, ri) = complex_div(ddf, f);
    let two = T::from_f64(2.0);
    (qi, ri - two * qr * qi)
}

impl<T: Real> ChartFrame<T> {
    /// The frame of a `Curve3::Circle` against a `Surface::Sphere`;
    /// `None` for any other pair.
    pub(crate) fn of(carrier: &Curve3<T>, surface: &Surface<T>) -> Option<Self> {
        let (
            &Curve3::Circle {
                center,
                axis,
                radius: r,
                u_ref,
            },
            &Surface::Sphere {
                center: s_center,
                radius,
                axis: s_axis,
                u_ref: s_u_ref,
            },
        ) = (carrier, surface)
        else {
            return None;
        };
        let d = center - s_center;
        let (a, b) = (u_ref * r, axis.cross(u_ref) * r);
        let s_v = s_axis.cross(s_u_ref);
        let channel = |e: Vec3<T>| [d.dot(e) / radius, a.dot(e) / radius, b.dot(e) / radius];
        let (x, y, z) = (channel(s_u_ref), channel(s_v), channel(s_axis));
        let k = (x[1].powi(2) + y[1].powi(2)).sqrt() + (x[2].powi(2) + y[2].powi(2)).sqrt();
        let z1 = z[1].abs() + z[2].abs();
        let off_sphere = off_sphere_sup(off_sphere_coefficients(d, a, b, radius), radius);
        Some(Self {
            x,
            y,
            z,
            radius,
            k,
            z1,
            off_sphere,
        })
    }

    /// [`off_sphere_sup`] of the circle, metres.
    pub(crate) fn off_sphere(&self) -> T {
        self.off_sphere
    }

    /// `(value, first, second derivative)` of one channel at `t`.
    fn channel_at(e: [T; 3], s: T, c: T) -> (T, T, T) {
        (
            e[0] + e[1] * c + e[2] * s,
            e[2] * c - e[1] * s,
            T::zero() - e[1] * c - e[2] * s,
        )
    }

    /// The image's value and first two derivatives at `t`:
    /// `u = arg w`, `v = arg(ρ + iz)` (module docs).
    pub(crate) fn jet(&self, t: T) -> Jet<T> {
        let (s, c) = t.sin_cos();
        let (x, dx, ddx) = Self::channel_at(self.x, s, c);
        let (y, dy, ddy) = Self::channel_at(self.y, s, c);
        let (z, dz, ddz) = Self::channel_at(self.z, s, c);
        let rho = (x.powi(2) + y.powi(2)).sqrt();
        let drho = (x * dx + y * dy) / rho;
        let ddrho = (dx.powi(2) + dy.powi(2) + x * ddx + y * ddy - drho.powi(2)) / rho;
        let (du, ddu) = arg_derivatives((x, y), (dx, dy), (ddx, ddy));
        let (dv, ddv) = arg_derivatives((rho, z), (drho, dz), (ddrho, ddz));
        Jet {
            x,
            y,
            u: y.atan2(x),
            v: z.atan2(rho),
            du,
            dv,
            ddu,
            ddv,
        }
    }
}

/// `P_u − g_u` reduced into one period about zero, read in the frame
/// rotated to `P_u` — so the arc tangent is taken near zero, where no
/// branch cut sits, at every scalar.
fn azimuth_gap<T: Real>(p_u: T, jet: &Jet<T>) -> T {
    let (s, c) = p_u.sin_cos();
    T::zero() - (jet.y * c - jet.x * s).atan2(jet.x * c + jet.y * s)
}

/// The quintic Hermite Bézier controls of one channel on a span of
/// length `h` from its end data `(value, d, dd)` (module docs).
fn hermite_controls<T: Real>(h: T, a: (T, T, T), b: (T, T, T)) -> [T; 6] {
    let f = T::from_f64;
    let (h5, h2) = (h / f(5.0), h.powi(2) / f(20.0));
    [
        a.0,
        a.0 + h5 * a.1,
        a.0 + f(2.0) * h5 * a.1 + h2 * a.2,
        b.0 - f(2.0) * h5 * b.1 + h2 * b.2,
        b.0 - h5 * b.1,
        b.0,
    ]
}

/// What [`span_bound`] says about one span.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SpanBound<T: Real> {
    /// The bound on `|S(P(t)) − S(g(t))|` over the span, metres. Valid
    /// only when [`Self::radius`] is positive.
    pub(crate) metres: T,
    /// The smaller of the two Cauchy radii: positive exactly when the
    /// span is short enough against its distance from the poles for
    /// [`Self::metres`] to be a bound at all.
    pub(crate) radius: T,
}

/// The bound on `|S(P(t)) − S(g(t))|` over the span `[a, b]` of an
/// image whose Bézier controls there are `p` (module docs). Both chart
/// representations of `g` are offered — the principal one and the
/// sphere's involution twin `(u + π, π − v)` — and the smaller bound is
/// kept: either is a bound on the same 3-D distance.
pub(crate) fn span_bound<T: Real>(
    frame: &ChartFrame<T>,
    a: T,
    b: T,
    p: &[Point2<T>; 6],
) -> SpanBound<T> {
    let f = T::from_f64;
    let half = f(0.5);
    let (k, z1) = (frame.k, frame.z1);
    let h = b - a;
    let tc = (a + b) * half;
    let jc = frame.jet(tc);
    let rho_c = (jc.x.powi(2) + jc.y.powi(2)).sqrt();
    let (sc, cc) = tc.sin_cos();
    let zc = frame.z[0] + frame.z[1] * cc + frame.z[2] * sc;
    let q_c = (rho_c.powi(2) + zc.powi(2)).sqrt();
    // Cauchy on `(log f)′`: `|(log f)⁽⁶⁾| ≤ 5!·sup|f′/f| / δ⁵`.
    let sixth = |sup: T, delta: T| f(120.0) * sup / delta.powi(5);
    // The azimuth: `log w` on the disc of radius `lam_u`.
    let lam_u = f(0.5).min(rho_c / (f(2.6) * k));
    let m = rho_c - f(1.3) * k * lam_u;
    let w_max = rho_c + f(1.3) * k * lam_u;
    let delta_u = lam_u - h * half;
    let g6u = sixth(f(1.65) * k / m, delta_u);
    // The polar angle: `log(ρ + iz)` on the disc of radius `lam_v`.
    let slope = f(1.65) * k * w_max / m;
    let lam_v = lam_u.min(q_c / (f(2.0) * (slope + f(1.3) * z1)));
    let g_min = q_c - lam_v * (slope + f(1.3) * z1);
    let delta_v = lam_v - h * half;
    let g6v = sixth((slope + f(1.65) * z1) / g_min, delta_v);
    // The quintic Hermite remainder: `(h/2)⁶/6!`.
    let remainder = h.powi(6) / f(46080.0);
    // `|cos v|` along the chart segment from `g` to `P`: at most
    // `ρ/|q| + |Δv|`, with `ρ ≤ ρ_c + K·h/2` on the span and
    // `|q| ≥ 1 − off_sphere/R` everywhere on the circle. Where that
    // denominator is not positive the envelope already carries an
    // off-sphere term past any band.
    let q_lo = f(1.0) - frame.off_sphere / frame.radius;
    let rho_hi = rho_c + k * h * half;
    let (ja, jb) = (frame.jet(a), frame.jet(b));
    let tau = T::tau();
    let channel_gap = |twin: bool| {
        let (shift, flip) = if twin {
            (T::pi(), f(-1.0))
        } else {
            (T::zero(), f(1.0))
        };
        let v_of = |j: &Jet<T>| if twin { T::pi() - j.v } else { j.v };
        let du0 = azimuth_gap(p[0].x - shift, &ja);
        let du5 = azimuth_gap(p[5].x - shift, &jb);
        let gu = hermite_controls(
            h,
            (p[0].x - du0, ja.du, ja.ddu),
            (p[5].x - du5, jb.du, jb.ddu),
        );
        let dv0 = (p[0].y - v_of(&ja)).reduce_periodic_centred(tau);
        let dv5 = (p[5].y - v_of(&jb)).reduce_periodic_centred(tau);
        let gv = hermite_controls(
            h,
            (p[0].y - dv0, flip * ja.dv, flip * ja.ddv),
            (p[5].y - dv5, flip * jb.dv, flip * jb.ddv),
        );
        let (mut eu, mut ev) = (T::zero(), T::zero());
        for i in 0..6 {
            eu = eu.max((p[i].x - gu[i]).abs());
            ev = ev.max((p[i].y - gv[i]).abs());
        }
        let eu = eu + remainder * g6u;
        let ev = ev + remainder * g6v;
        let cos_max = f(1.0).min(rho_hi / q_lo + ev);
        frame.radius * (cos_max * eu + ev)
    };
    SpanBound {
        metres: channel_gap(false).min(channel_gap(true)),
        radius: delta_u.min(delta_v),
    }
}

/// The deepest trisection [`hermite_image`] makes of the arc.
const MAX_DEPTH: u32 = 12;

/// The most spans [`hermite_image`] will make.
const MAX_SPANS: usize = 1 << 16;

/// Why [`hermite_image`] made no image.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ImageRefusal {
    /// No span short enough exists within the refinement's caps: the arc
    /// runs into, or within the band's reach of, a pole of the chart.
    NearPole {
        /// The spans the refinement had made when it stopped.
        spans: usize,
    },
    /// The structure would not build (a degenerate interval).
    Structure,
}

/// The Hermite controls of the image on `[a, b]` from the jets there
/// (`jb`'s azimuth continued from `ja`'s).
fn span_controls(a: f64, ja: &Jet<f64>, b: f64, jb: &Jet<f64>) -> [Point2<f64>; 6] {
    let gu = hermite_controls(b - a, (ja.u, ja.du, ja.ddu), (jb.u, jb.du, jb.ddu));
    let gv = hermite_controls(b - a, (ja.v, ja.dv, ja.ddv), (jb.v, jb.dv, jb.ddv));
    core::array::from_fn(|i| Point2::new(gu[i], gv[i]))
}

/// **The general circle's chart image**, `f64` structure (C6): the
/// piecewise quintic Hermite interpolant of `g` over `[f0, f1]`, refined
/// by trisection until every span's [`span_bound`] is under `budget`
/// metres (module docs). The azimuth is continued along the arc, so the
/// image is one branch.
pub(crate) fn hermite_image(
    frame: &ChartFrame<f64>,
    f0: f64,
    f1: f64,
    budget: f64,
) -> Result<NurbsCurve2<f64>, ImageRefusal> {
    if !(f1 > f0 && f0.is_finite() && f1.is_finite()) {
        return Err(ImageRefusal::Structure);
    }
    let mut nodes: Vec<(f64, Jet<f64>)> = vec![(f0, frame.jet(f0))];
    let continue_u = |prev: &Jet<f64>, mut next: Jet<f64>| {
        next.u = prev.u - azimuth_gap(prev.u, &next);
        next
    };
    // Depth-first, so the nodes come out in order.
    let mut stack: Vec<(f64, f64, u32)> = vec![(f0, f1, 0)];
    while let Some((a, b, depth)) = stack.pop() {
        let (_, ja) = *nodes.last().ok_or(ImageRefusal::Structure)?;
        let jb = continue_u(&ja, frame.jet(b));
        let bound = span_bound(frame, a, b, &span_controls(a, &ja, b, &jb));
        if bound.radius > 0.0 && bound.metres <= budget {
            nodes.push((b, jb));
            continue;
        }
        if depth == MAX_DEPTH || nodes.len() + stack.len() > MAX_SPANS {
            return Err(ImageRefusal::NearPole {
                spans: nodes.len() - 1,
            });
        }
        let h3 = (b - a) / 3.0;
        let (m1, m2) = (a + h3, a + 2.0 * h3);
        stack.push((m2, b, depth + 1));
        stack.push((m1, m2, depth + 1));
        stack.push((a, m1, depth + 1));
    }
    let mut knots = vec![f0; DEGREE + 1];
    let mut points = vec![Point2::new(nodes[0].1.u, nodes[0].1.v)];
    for pair in nodes.windows(2) {
        let ((a, ja), (b, jb)) = (pair[0], pair[1]);
        points.extend_from_slice(&span_controls(a, &ja, b, &jb)[1..]);
        knots.extend([b; DEGREE]);
    }
    knots.push(f1);
    let weights = vec![1.0; points.len()];
    let kv = KnotVector::clamped(knots, DEGREE).map_err(|_| ImageRefusal::Structure)?;
    NurbsCurve2::new(kv, points, weights).map_err(|_| ImageRefusal::Structure)
}

/// The image's degree: quintic, so each span is a Bézier with six
/// controls and the image is C² (module docs).
const DEGREE: usize = 5;

/// One span of an image in [`hermite_image`]'s form: its parameter
/// interval and its six Bézier controls.
pub(crate) type HermiteSpan<T> = (f64, f64, [Point2<T>; 6]);

/// The spans of an image in [`hermite_image`]'s form — `(a, b,
/// controls)` per span — or `None` when it is not in that form: degree
/// five, unit weights, interior knots of multiplicity exactly five.
pub(crate) fn hermite_spans<T: Real>(image: &NurbsCurve2<T>) -> Option<Vec<HermiteSpan<T>>> {
    let kv = image.knots();
    let knots = kv.knots();
    let control = image.control();
    let n = knots.len();
    if kv.degree() != DEGREE
        || image.weights().iter().any(|&w| w != 1.0)
        || n < 2 * DEGREE + 2
        || !(n - 2).is_multiple_of(DEGREE)
    {
        return None;
    }
    // [a×6, t₁×5, …, b×6]: `spans` spans, break j at index 5j + 5.
    let spans = (n - 2) / DEGREE - 1;
    let at = |j: usize| knots[DEGREE * j + DEGREE];
    let well_formed = control.len() == DEGREE * spans + 1
        && knots[..=DEGREE].iter().all(|&k| k == knots[0])
        && knots[n - DEGREE - 1..].iter().all(|&k| k == knots[n - 1])
        && (1..spans).all(|j| {
            let i = DEGREE * j + DEGREE;
            knots[i + 1 - DEGREE..=i].iter().all(|&k| k == knots[i]) && knots[i + 1] > knots[i]
        })
        && (0..spans).all(|j| at(j + 1) > at(j));
    if !well_formed {
        return None;
    }
    Some(
        (0..spans)
            .map(|j| {
                let c = &control[DEGREE * j..=DEGREE * j + DEGREE];
                (at(j), at(j + 1), core::array::from_fn(|i| c[i]))
            })
            .collect(),
    )
}
