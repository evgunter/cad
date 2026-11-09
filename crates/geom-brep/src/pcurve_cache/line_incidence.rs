//! **Where a line's distance from an analytic chart can peak over its
//! span**: the read `incident` takes of a line is the chart's exact
//! distance ([`super::chart_distance`]) at the span's two ends and at
//! the interior parameters this module names, and nowhere else. Its
//! largest value is the distance's supremum over the span, so a line
//! off the chart anywhere in the span is shown off, however narrow the
//! stretch it leaves the chart over.
//!
//! Along `p(t) = o + t·u`, with the chart's anchor `c` and unit axis `a`
//! ([`crate::implicit::axial_radial`]), the axial height `h(t)` is
//! affine and the radial distance `ρ(t) = |w(t)|`, `w` the part of
//! `p − c` off the axis, is the norm of an affine map: convex, smooth
//! but at `t*`, the line's nearest approach to the axis, where it is
//! kinked if the line meets the axis. With `v = |u⊥|` and `d = ρ(t*)`,
//! `ρ'' = v²d²/ρ³`, largest at `t*` and falling off on either side.
//!
//! The span is cut at **marks** (its ends and the kinks below), and on
//! each piece between two marks the derivative of the function read is
//! monotone, so it has at most one root there: found by bisection where
//! it changes sign across the piece. Every extremum of the distance is a
//! mark or such a root, so the read is the supremum. Per chart:
//!
//! - **plane**: `|affine|`, largest at an end; no interior parameter;
//! - **sphere**: `|‖p − c‖ − R|`, `‖p − c‖` convex with its least value
//!   at the foot of the centre, `−q·u/u·u` (`q = o − c`);
//! - **cylinder**: `|ρ − R|`, `ρ`'s least value at `t*`;
//! - **cone**: `|f|`, `f = ρ cos α − |h| sin α` (the double cone's exact
//!   distance). Marks at `t_h`, where the line crosses the apex plane
//!   (the apex's own kink), and at `t*`. On each piece `h` keeps its
//!   sign, so `f` is convex: its largest value is at a mark, and its
//!   least at the root of `f'`;
//! - **torus and spindle**: `|q − r|`, `q² = |p − c|² + R² − 2Rρ`, read
//!   on `q²`. Marks at `t*` and, for `R > 0` with `d > 0`, at
//!   `t* ± τc`, where `(q²)'' = 2(|u|² − R·ρ'')` changes sign
//!   (`ρ(τc)³ = R·v²d²/|u|²`). `ρ''` is monotone on either side of
//!   `t*`, so `(q²)''` changes sign at most once a side and `(q²)'` is
//!   monotone between the marks; for `R ≤ 0`, `d = 0` or `v = 0`,
//!   `(q²)'' ≥ 2|u|² > 0` away from `t*`. On a line along the axis
//!   (no `t*`) the span is one piece, and its root is the centre's foot.
//!
//! The argument holds exactly for the line the data's `f64` middles
//! name, whatever its pose: a line along or near the axis, through the
//! apex, in the apex plane or through the core circle only moves the
//! marks, or drops the ones it has no use for (a closed form that is
//! `0/0` there is not finite, and is not read).
//!
//! The parameters are returned as fractions of the span in `(0, 1)`: a
//! place to look, which the read evaluates in the scalar's own
//! arithmetic and which is a point of the span at every scalar. A
//! definite Off rests on the evaluation alone; the supremum, so a Zero
//! read as within the band, rests on the placement. The placement is
//! off by the bisection's last bit or the closed form's rounding, each
//! a few ulps of the span where the line is well posed and worse where
//! its data are ill conditioned (a root of `f'` where `f'` is flat). At
//! a smooth stationary point that moves the value to second order. At a
//! kink — `t_h`, `t*` on a line through the axis, the centre's foot on
//! a line through a sphere's centre, a torus's core crossing (where
//! `q`, not `q²`, is kinked) — it moves it to first order, by the
//! line's speed times the misplacement.

use geom::Surface;
use geom_core::{Point3, SpanLocate, Vec3};

use crate::implicit::axial_radial;

/// The middle of a value's bracket; NaN for the poison.
fn middle<T: SpanLocate>(x: T) -> f64 {
    x.structure_bracket().map_or(f64::NAN, geom::bracket_mid)
}

fn mid3<T: SpanLocate>(v: Vec3<T>) -> Vec3<f64> {
    Vec3::new(middle(v.x), middle(v.y), middle(v.z))
}

fn mid_point<T: SpanLocate>(p: Point3<T>) -> Point3<f64> {
    Point3::new(middle(p.x), middle(p.y), middle(p.z))
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
    let span = (middle(t0), middle(t1));
    let (o, u) = (mid_point(line.origin), mid3(line.dir));
    let length = span.1 - span.0;
    if !(length > 0.0 && length.is_finite()) {
        return Vec::new();
    }
    let mid_line = geom::Line { origin: o, dir: u };
    let centre_foot = |c: Point3<f64>| mid_line.param(c);
    let ts = match *surface {
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => Vec::new(),
        Surface::Sphere { center, .. } => vec![centre_foot(mid_point(center))],
        Surface::Cylinder { origin, axis, .. } => Axial::of(o, u, mid_point(origin), mid3(axis))
            .star
            .into_iter()
            .collect(),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => Axial::of(o, u, mid_point(apex), mid3(axis)).cone(middle(half_angle), span),
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
        } => Axial::of(o, u, mid_point(center), mid3(axis)).torus(middle(major_radius), span),
    };
    let mut fractions: Vec<f64> = ts
        .into_iter()
        .map(|t| (t - span.0) / length)
        .filter(|l| *l > 0.0 && *l < 1.0)
        .collect();
    fractions.sort_by(f64::total_cmp);
    fractions.dedup();
    fractions
}

/// The line in the chart's axial frame (module docs): `q = o − c` and
/// `u`, `h(t) = h0 + t·w`, and `w(t) = base + u⊥·(t − tb)` off the axis,
/// with `tb = t*` where the line is not along the axis (`star`) and `0`
/// where it is.
struct Axial {
    q: Vec3<f64>,
    u: Vec3<f64>,
    h0: f64,
    w: f64,
    u_off: Vec3<f64>,
    star: Option<f64>,
    tb: f64,
    base: Vec3<f64>,
}

impl Axial {
    fn of(o: Point3<f64>, u: Vec3<f64>, c: Point3<f64>, axis: Vec3<f64>) -> Self {
        let a = axis.normalize();
        let (h0, q_off) = axial_radial(o, c, a);
        let (w, u_off) = axial_radial(Point3::origin() + u, Point3::origin(), a);
        let star = Some(-q_off.dot(u_off) / u_off.dot(u_off)).filter(|t| t.is_finite());
        let tb = star.unwrap_or(0.0);
        Self {
            q: o - c,
            u,
            h0,
            w,
            u_off,
            star,
            tb,
            base: q_off + u_off * tb,
        }
    }

    /// `ρ(t)` and `ρ'(t)`; at a kink (`ρ = 0`) the one-sided slope on
    /// the `side` (`±1`) the caller reads from.
    fn rho(&self, t: f64, side: f64) -> (f64, f64) {
        let w = self.base + self.u_off * (t - self.tb);
        let rho = w.norm();
        let slope = if rho > 0.0 {
            w.dot(self.u_off) / rho
        } else {
            side * self.u_off.norm()
        };
        (rho, slope)
    }

    /// The cone's interior parameters (module docs).
    fn cone(&self, half_angle: f64, span: (f64, f64)) -> Vec<f64> {
        let (sin, cos) = half_angle.sin_cos();
        let t_h = Some(-self.h0 / self.w).filter(|t| t.is_finite());
        let marks: Vec<f64> = t_h.into_iter().chain(self.star).collect();
        let sigma = |t: f64| (self.h0 + self.w * t).signum();
        // On a piece `h` keeps one sign, the one at its middle.
        let roots = monotone_roots(&marks, span, |t, side, mid| {
            self.rho(t, side).1 * cos - sigma(mid) * self.w * sin
        });
        marks.into_iter().chain(roots).collect()
    }

    /// The torus's and spindle's interior parameters (module docs), on
    /// `(q²)'/2 = (q + u·t)·u − R·ρ'`.
    fn torus(&self, big: f64, span: (f64, f64)) -> Vec<f64> {
        let (uu, v) = (self.u.dot(self.u), self.u_off.norm());
        let d = self.base.norm();
        let mut marks: Vec<f64> = self.star.into_iter().collect();
        if let Some(star) = self.star
            && big > 0.0
            && d > 0.0
        {
            let rho_c = (big * v.powi(2) * d.powi(2) / uu).cbrt();
            if rho_c > d {
                let tau = (rho_c.powi(2) - d.powi(2)).sqrt() / v;
                marks.extend([star - tau, star + tau]);
            }
        }
        let roots = monotone_roots(&marks, span, |t, side, _| {
            (self.q + self.u * t).dot(self.u) - big * self.rho(t, side).1
        });
        self.star.into_iter().chain(roots).collect()
    }
}

/// The roots of `f` over `span` cut at `marks`, on each piece of which
/// `f` is monotone: one where `f` changes sign across the piece.
/// `f(t, side, mid)` reads `t` from the `side` (`±1`) of a mark it sits
/// on, on the piece whose middle is `mid`.
fn monotone_roots(
    marks: &[f64],
    (s0, s1): (f64, f64),
    f: impl Fn(f64, f64, f64) -> f64,
) -> Vec<f64> {
    let mut cuts: Vec<f64> = [s0, s1]
        .into_iter()
        .chain(marks.iter().copied().filter(|m| *m > s0 && *m < s1))
        .collect();
    cuts.sort_by(f64::total_cmp);
    cuts.dedup();
    cuts.windows(2)
        .filter_map(|piece| {
            let (a, b) = (piece[0], piece[1]);
            let mid = geom::bracket_mid((a, b));
            let g = |t: f64| f(t, if t <= a { 1.0 } else { -1.0 }, mid);
            let (ga, gb) = (g(a), g(b));
            (ga * gb < 0.0).then(|| bisect(&g, a, b, ga < 0.0))
        })
        .collect()
}

/// A root of `g` in `[a, b]`, across whose ends `g` changes sign
/// (`negative_at_a` its sign at `a`), to the last bit `f64` can place,
/// in at most 200 halvings.
fn bisect(g: &impl Fn(f64) -> f64, mut a: f64, mut b: f64, negative_at_a: bool) -> f64 {
    for _ in 0..200 {
        let m = geom::bracket_mid((a, b));
        if m <= a || m >= b {
            return m;
        }
        if (g(m) < 0.0) == negative_at_a {
            a = m;
        } else {
            b = m;
        }
    }
    geom::bracket_mid((a, b))
}
