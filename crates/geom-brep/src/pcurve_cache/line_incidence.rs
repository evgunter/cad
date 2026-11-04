//! **Where a line's distance from an analytic chart can peak over its
//! span**: the read `incident` takes of a line is the chart's exact
//! distance ([`super::chart_distance`]) at the span's two ends and at
//! the interior parameters this module names, and nowhere else. Its
//! largest value is the distance's supremum over the span, so a line
//! off the chart anywhere in the span is shown off, however narrow the
//! stretch it leaves the chart over.
//!
//! Along `p(t) = o + t·u`, with the chart's anchor `c`, its unit axis
//! `a`, `q = o − c`, the axial height `h(t) = q·a + t·u·a` and the
//! radial distance `ρ(t) = |q⊥ + t·u⊥|` (`⊥` the part off the axis),
//! `ρ` is the norm of an affine map, so convex, and kinked only where
//! the line meets the axis. Per chart:
//!
//! - **plane**: the distance is `|affine|`, largest at an end;
//! - **sphere**: `|‖p − c‖ − R|`, with `‖p − c‖` convex: its largest
//!   value is at an end, its least at the foot of the centre;
//! - **cylinder**: `|ρ − R|`, the same with `ρ`'s least at the foot of
//!   the axis, `t*`;
//! - **cone**: `|f|`, `f = ρ cos α − |h| sin α` (the double cone's exact
//!   distance). On each side of the apex plane (`h` of one sign) `f` is
//!   convex, so its largest value is at an end of the side — a span end
//!   or `t_h`, where the line crosses the apex plane, the apex's own
//!   kink — and its least at the side's stationary point,
//!   `v²·τ = k·ρ` with `τ = t − t*`, `k = σ·(u·a)·tan α`,
//!   `v = |u⊥|`, `d = ρ(t*)`: `τ = k·d / (v·√(v² − k²))`, where
//!   `v² > k²` (otherwise `f` is monotone on the side);
//! - **torus and spindle**: `|q − r|`, `q² = (ρ − R)² + h² =
//!   |p − c|² + R² − 2Rρ`, smooth but at `t*`. Its stationary points
//!   are the roots of `s = Q'ρ − R·P'` (`Q = |p − c|² + R²`,
//!   `P = ρ²`), which are among the real roots of the quartic
//!   `Q'²·P − R²·P'²`: found by isolation on the quartic, then each
//!   refined on `s` itself where `s` changes sign about it.
//!
//! Every extremum of the distance is at one of these parameters, so the
//! read is the supremum, not a sample of it.
//!
//! The parameters are computed in `f64` from the middles of the data's
//! brackets and returned as fractions of the span in `(0, 1)`: a place
//! to look, which the read evaluates in the scalar's own arithmetic and
//! which is a point of the span at every scalar. Their own rounding moves
//! a read stationary point by a few ulps of the span, which moves its
//! value to second order; the cone's `t_h` is a kink, where it moves the
//! value by the line's speed times those ulps, below the read's own
//! rounding charge.

use geom::Surface;
use geom_core::{Point3, SpanLocate, Vec3};

/// The middle of a value's bracket; NaN for the poison.
fn middle<T: SpanLocate>(x: T) -> f64 {
    x.structure_bracket()
        .map_or(f64::NAN, |(lo, hi)| lo + (hi - lo) * 0.5)
}

fn mid3<T: SpanLocate>(v: Vec3<T>) -> Vec3<f64> {
    Vec3::new(middle(v.x), middle(v.y), middle(v.z))
}

fn mid_point<T: SpanLocate>(p: Point3<T>) -> Vec3<f64> {
    mid3(p - Point3::origin())
}

/// **The interior parameters of `[t0, t1]` where the distance of the
/// line `origin + dir·t` from `surface` can peak** (module docs), as
/// fractions of the span strictly inside `(0, 1)`. The ends are the
/// caller's. Empty on a plane, on a span of no length, and where the
/// data's middles are not finite (the read at the ends then carries
/// the poison).
pub(super) fn critical_fractions<T: SpanLocate>(
    line: &geom::Line<T>,
    surface: &Surface<T>,
    (t0, t1): (T, T),
) -> Vec<f64> {
    let (s0, s1) = (middle(t0), middle(t1));
    let (o, u) = (mid_point(line.origin), mid3(line.dir));
    let length = s1 - s0;
    if !(length > 0.0 && length.is_finite()) {
        return Vec::new();
    }
    let ts = match *surface {
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => Vec::new(),
        Surface::Sphere { center, .. } => {
            let q = o - mid_point(center);
            vec![-q.dot(u) / u.dot(u)]
        }
        Surface::Cylinder { origin, axis, .. } => {
            let axial = Axial::of(o - mid_point(origin), u, mid3(axis));
            vec![axial.foot()]
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => Axial::of(o - mid_point(apex), u, mid3(axis)).cone(middle(half_angle)),
        Surface::Torus {
            center,
            axis,
            major_radius,
            ..
        }
        | Surface::Spindle {
            center,
            axis,
            major_radius,
            ..
        } => Axial::of(o - mid_point(center), u, mid3(axis)).torus(middle(major_radius), (s0, s1)),
    };
    let mut fractions: Vec<f64> = ts
        .into_iter()
        .map(|t| (t - s0) / length)
        .filter(|l| *l > 0.0 && *l < 1.0)
        .collect();
    fractions.sort_by(f64::total_cmp);
    fractions.dedup();
    fractions
}

/// The line in the chart's axial frame: `q = o − c` and `u`, each split
/// along the unit axis `a` and off it.
struct Axial {
    q: Vec3<f64>,
    u: Vec3<f64>,
    h0: f64,
    w: f64,
    q_off: Vec3<f64>,
    u_off: Vec3<f64>,
}

impl Axial {
    fn of(q: Vec3<f64>, u: Vec3<f64>, axis: Vec3<f64>) -> Self {
        let a = axis / axis.norm();
        let (h0, w) = (q.dot(a), u.dot(a));
        Self {
            q,
            u,
            h0,
            w,
            q_off: q - a * h0,
            u_off: u - a * w,
        }
    }

    /// `t*`, the line's nearest approach to the axis: `ρ`'s least value
    /// (NaN for a line along the axis, whose `ρ` is constant).
    fn foot(&self) -> f64 {
        -self.q_off.dot(self.u_off) / self.u_off.dot(self.u_off)
    }

    /// The cone's interior candidates (module docs): `t*`, `t_h`, and
    /// each side's stationary point.
    fn cone(&self, half_angle: f64) -> Vec<f64> {
        let star = self.foot();
        let v2 = self.u_off.dot(self.u_off);
        let d = (self.q_off + self.u_off * star).norm();
        let mut ts = vec![star, -self.h0 / self.w];
        for sigma in [1.0, -1.0] {
            let k = sigma * self.w * half_angle.tan();
            if v2 > k.powi(2) {
                ts.push(star + k * d / (v2.sqrt() * (v2 - k.powi(2)).sqrt()));
            }
        }
        ts
    }

    /// The torus's interior candidates (module docs): `t*` and the
    /// stationary points of `q²`, in `(s0, s1)`.
    fn torus(&self, big: f64, (s0, s1): (f64, f64)) -> Vec<f64> {
        let star = self.foot();
        // Polynomials in `y = (t − t*) / scale`, about the line's nearest
        // approach to the axis, so `P`'s coefficients do not cancel there
        // and the quartic's are of the span's size.
        let centre = if star.is_finite() { star } else { 0.0 };
        let scale = (s0 - centre).abs().max((s1 - centre).abs());
        if !(scale > 0.0 && scale.is_finite() && big.is_finite()) {
            return vec![star];
        }
        let (q, q_off) = (self.q + self.u * centre, self.q_off + self.u_off * centre);
        let (u, uo) = (self.u * scale, self.u_off * scale);
        // P = |q⊥ + y·u⊥|², P' and Q' in y (their common factor 2
        // dropped: it cancels from `Q'²P − R²P'²`).
        let p = [q_off.dot(q_off), 2.0 * q_off.dot(uo), uo.dot(uo)];
        let dp = [q_off.dot(uo), uo.dot(uo)];
        let dq = [q.dot(u), u.dot(u)];
        let quartic = sub(
            &mul(&mul(&dq, &dq), &p),
            &mul(&[big.powi(2)], &mul(&dp, &dp)),
        );
        let (lo, hi) = ((s0 - centre) / scale, (s1 - centre) / scale);
        let roots = roots_in(&quartic, lo, hi);
        // `s(y) = Q'·ρ − R·P'`, whose roots are the stationary points,
        // read on the vectors themselves.
        let s = |y: f64| {
            let w = q_off + uo * y;
            (q + u * y).dot(u) * w.norm() - big * w.dot(uo)
        };
        let mut marks: Vec<f64> = [lo, hi, 0.0]
            .into_iter()
            .chain(roots.iter().copied())
            .filter(|y| y.is_finite())
            .collect();
        marks.sort_by(f64::total_cmp);
        marks.dedup();
        // Every root of `s` is a root of the quartic, so each lies at a
        // quartic root as isolated, or, where the quartic's own rounding
        // moved it, between two marks across which `s` changes sign.
        let refined = marks
            .windows(2)
            .filter(|m| s(m[0]) * s(m[1]) < 0.0)
            .map(|m| bisect(&s, m[0], m[1]));
        let mut ts = vec![star];
        ts.extend(
            roots
                .iter()
                .copied()
                .chain(refined)
                .map(|y| centre + y * scale),
        );
        ts
    }
}

/// The product of two polynomials (coefficients lowest degree first).
fn mul(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] += x * y;
        }
    }
    out
}

fn sub(a: &[f64], b: &[f64]) -> Vec<f64> {
    (0..a.len().max(b.len()))
        .map(|k| a.get(k).copied().unwrap_or(0.0) - b.get(k).copied().unwrap_or(0.0))
        .collect()
}

/// The real roots of the polynomial `c` in `[lo, hi]`, isolated between
/// the roots of its derivative (each monotone stretch holds at most one)
/// and bisected.
fn roots_in(c: &[f64], lo: f64, hi: f64) -> Vec<f64> {
    let degree = c.iter().rposition(|k| *k != 0.0).unwrap_or(0);
    let c = &c[..=degree];
    if degree == 0 {
        return Vec::new();
    }
    let f = |y: f64| c.iter().rev().fold(0.0, |acc, k| acc * y + k);
    if degree == 1 {
        let y = -c[0] / c[1];
        return if (lo..=hi).contains(&y) {
            vec![y]
        } else {
            Vec::new()
        };
    }
    let derivative: Vec<f64> = c
        .iter()
        .enumerate()
        .skip(1)
        .map(|(k, x)| x * k as f64)
        .collect();
    let mut marks = vec![lo];
    marks.extend(roots_in(&derivative, lo, hi));
    marks.push(hi);
    let mut out = Vec::new();
    for pair in marks.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let (fa, fb) = (f(a), f(b));
        if fa == 0.0 {
            out.push(a);
        } else if fa * fb < 0.0 {
            out.push(bisect(&f, a, b));
        }
    }
    if f(hi) == 0.0 {
        out.push(hi);
    }
    out
}

/// A root of `f` in `[a, b]`, across whose ends `f` changes sign, to the
/// last bit `f64` can place.
fn bisect(f: &impl Fn(f64) -> f64, mut a: f64, mut b: f64) -> f64 {
    let negative_at_a = f(a) < 0.0;
    loop {
        let m = a + (b - a) * 0.5;
        if m <= a || m >= b {
            return m;
        }
        if (f(m) < 0.0) == negative_at_a {
            a = m;
        } else {
            b = m;
        }
    }
}
