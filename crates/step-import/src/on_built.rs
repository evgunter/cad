//! **The printed carrier on its built one** (`docs/DESIGN.md` `[import]`
//! step 2): the importer verifies that the curve the file printed lies
//! within ε_in of the edge's description, then rebuilds from the
//! description. The build ([`crate::build`]) constructs each edge's
//! carrier from its description; this module answers the verification as
//! one number, a CERTIFIED upper bound in metres on the distance of every
//! point of the printed carrier over its trim from the built carrier's
//! locus, read at the file's band by the one decision
//! `step_edge_on_description`.
//!
//! The certificate reads the locus alone: what the printed carrier is
//! evidence of, never its parameter or its kind's choice of arm. Each arm
//! is a whole-trim inequality, no sampling:
//!
//! - **any printed carrier on a built line**: the far end of the
//!   distance range from the line of the printed carrier's cover over its
//!   trim ([`geom::Curve3::cover_over`] at the certification scalar: a
//!   spline's pieces' nets over the window, a line's segment, a conic's
//!   arc), which holds every point of the trim. The distance from a line
//!   is convex, so a net's farthest vertex bounds its hull;
//! - **a printed spline on a built circle**: the ring composites of the
//!   curve recognizer ([`crate::recognize_curve`], INV-C1 and INV-C2)
//!   built over the printed spline and cut at its trim's ends
//!   ([`crate::recognize_curve::circle_locus_within`]), so the sup is
//!   over exactly the spans the edge uses. The built circle's frame is
//!   read as exact, as the recognizer reads its own estimate
//!   (`docs/DESIGN.md` `[simple.conditioned]`);
//! - **a printed spline on the identical spline** (a wall's column the
//!   file repeats bit for bit): zero;
//! - **a printed circle's arc on a built circle**: the extremes over the
//!   trim of two sinusoids ([`arc_on_circle`]);
//! - **any other printed conic on a built conic**: each
//!   carrier is `c + cos θ·A + sin θ·B` for a centre `c` and conjugate
//!   semi-diameters `A`, `B`, and `θ ↦ Q(θ)` names a point of the built
//!   conic for every θ under each of its own reparameterizations chosen
//!   below. So `|P(θ) − Q(θ)| ≤ |c₁ − c₂| + |A₁ − A₂| + |B₁ − B₂|` bounds
//!   the distance of every printed point from the built conic, over the
//!   whole printed conic and so over its trim. The built circle's `A₂`
//!   is the printed seam direction projected into its plane, at the
//!   built radius; a built ellipse's is the best of its eight principal
//!   pairings `(±a·u, ±b·v)`, `(±b·v, ±a·u)`. The frames' `f64` closed
//!   forms are read as exact; the sums are outward.
//!
//! Every other pair (a printed spline on a built spline that is not its
//! twin bit for bit, a spline on an ellipse or a spiric) has no
//! certificate here, and [`printed_on`]
//! answers `None`: the build then reads the printed carrier's ends and
//! middle (`work/exch/the-printed-on-built-certificate-has-no-arm-for-a-built-spline.md`).

// The `!(a > b)` forms are deliberate, NaN-catching negations: a NaN
// quantity refuses, where the positive form would accept it.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

use geom::{Curve3, NurbsCurve3};
use geom_core::interval::certification::Certification as _;
use geom_core::interval::{cross, max_bound, norm_sup};
use geom_core::{Bounds, Interval, Point3, Vec3};

use crate::entities::EdgeSpec;
use crate::recognize_curve::circle_locus_within;

/// **The certified sup of the printed carrier's distance from `built`
/// over the printed trim** (module docs); `None` where no arm serves the
/// pair, or the arm certifies no finite bound.
pub(crate) fn printed_on(spec: &EdgeSpec, built: &Curve3<f64>) -> Option<f64> {
    let trim = (spec.t0, spec.t1);
    let line = match built {
        Curve3::Line(line) => Some(*line),
        _ => None,
    };
    if let Some(line) = line {
        return printed_off_line(spec, line);
    }
    let bound = match (&spec.carrier, built) {
        (Curve3::Nurbs(printed), Curve3::Circle(circle)) => circle_locus_within(
            printed,
            (circle.frame.origin, circle.frame.axis, circle.radius()),
            trim,
        ),
        (Curve3::Nurbs(printed), Curve3::Nurbs(built)) => {
            if bitwise_twin(printed, built) {
                0.0
            } else {
                return None;
            }
        }
        (Curve3::Circle(printed), Curve3::Circle(built)) => arc_on_circle(printed, trim, built),
        (printed, built) => {
            let (Some(printed), Some(built)) = (Conic::of(printed), Conic::of(built)) else {
                return None;
            };
            built.bound_from(&printed)?
        }
    };
    bound.is_finite().then_some(bound)
}

/// Whether two splines are one bit for bit: knots, controls and weights.
pub(crate) fn bitwise_twin(a: &NurbsCurve3<f64>, b: &NurbsCurve3<f64>) -> bool {
    let bits3 = |p: &Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    let (ka, kb) = (a.knots_f64_interim(), b.knots_f64_interim());
    ka.degree() == kb.degree()
        && ka.knots().len() == kb.knots().len()
        && ka
            .knots()
            .iter()
            .zip(kb.knots())
            .all(|(x, y)| x.to_bits() == y.to_bits())
        && a.control().len() == b.control().len()
        && a.control()
            .iter()
            .zip(b.control())
            .all(|(x, y)| bits3(x) == bits3(y))
        && a.weights().len() == b.weights().len()
        && a.weights()
            .iter()
            .zip(b.weights())
            .all(|(x, y)| x.to_bits() == y.to_bits())
}

/// **The printed carrier over its trim, read where its cover holds it**:
/// the carrier at the certification scalar, exact, and its cover over
/// the trim ([`Curve3::cover_over`]: a spline's pieces' nets over the
/// window, a line's segment, a conic's arc), which holds every point the
/// trim does. `None` where no cover is read.
fn trim_cover(spec: &EdgeSpec) -> Option<geom::cover::Cover<Interval>> {
    let carrier = spec.carrier.map_scalar(Interval::point);
    let window = geom::cover::ParamWindow::new(Interval::point(spec.t0), Interval::point(spec.t1));
    carrier.cover_over(window).ok()
}

/// A line at the certification scalar, exact.
fn exact_line(origin: Point3<f64>, dir: Vec3<f64>) -> geom::Line<Interval> {
    let [ox, oy, oz] = pt(origin);
    let [dx, dy, dz] = vc(dir);
    geom::Line {
        origin: Point3::new(ox, oy, oz),
        dir: Vec3::new(dx, dy, dz),
    }
}

/// **The printed carrier's farthest from a built line over its trim**:
/// the far end of its trim cover's distance range from the line
/// ([`geom::cover::Cover::distance_range`]), read outward. Every printed
/// point lies in the cover, so none stands farther. `None` where no
/// bound certifies.
fn printed_off_line(spec: &EdgeSpec, line: geom::Line<f64>) -> Option<f64> {
    let site = geom::cover::Site::Line(exact_line(line.origin, line.dir));
    let (_, far) = trim_cover(spec)?.distance_range(&site);
    let bound = far.hi();
    bound.is_finite().then_some(bound)
}

/// **The printed carrier's farthest from the segment `[a, b]` over its
/// trim**, certified: its trim cover's farthest from the segment's line
/// and how far its range along the segment runs past either end, `hypot`
/// of the two, outward; every printed point lies in the cover. `None`
/// where the segment is not certified to have a length or no bound
/// certifies.
pub(crate) fn printed_on_segment(
    spec: &EdgeSpec,
    (a, b): (Point3<f64>, Point3<f64>),
) -> Option<f64> {
    let d = sub(pt(b), pt(a));
    let length2 = geom_core::interval::norm_sq(&d);
    if !(length2.lo() > 0.0) {
        return None;
    }
    let cover = trim_cover(spec)?;
    let site = geom::cover::Site::Line(exact_line(a, b - a));
    let (_, across) = cover.distance_range(&site);
    let [dx, dy, dz] = d;
    let (lo, hi) = cover.range_along(Vec3::new(dx, dy, dz));
    let start = geom_core::interval::dot(&pt(a), &d);
    let length = length2.sqrt();
    let past = ((Interval::zero() - (lo - start)) / length)
        .hi()
        .max(((hi - start - length2) / length).hi())
        .max(0.0);
    let bound = (across.sqr() + Interval::point(past).sqr()).sqrt().hi();
    bound.is_finite().then_some(bound)
}

/// A certified upper bound on the distance of the exact point `p` from
/// the segment `[a, b]`: its distance from the segment's line and how far
/// its projection runs past either end, `hypot` of the two, outward; from
/// `a`, plus the segment's length, where the segment is not certified to
/// have one. `NaN` where it certifies none.
pub(crate) fn point_segment_sup(p: Point3<f64>, (a, b): (Point3<f64>, Point3<f64>)) -> f64 {
    let off = sub(pt(p), pt(a));
    let d = sub(pt(b), pt(a));
    let length = geom_core::interval::norm_sq(&d).sqrt();
    if !(length.lo() > 0.0) {
        return geom_core::interval::add_up(norm_sup(&off), norm_sup(&d));
    }
    let across = Interval::point(norm_sup(&cross(&off, &d))) / length;
    let along = geom_core::interval::dot(&off, &d) / length;
    let past = (Interval::zero() - along)
        .hi()
        .max((along - length).hi())
        .max(0.0);
    (across.sqr() + Interval::point(past).sqr()).sqrt().hi()
}

/// The componentwise enclosure of `p` (exact).
fn pt(p: Point3<f64>) -> [Interval; 3] {
    [p.x, p.y, p.z].map(Interval::point)
}

/// The componentwise enclosure of `v` (exact).
fn vc(v: Vec3<f64>) -> [Interval; 3] {
    [v.x, v.y, v.z].map(Interval::point)
}

/// `a − b`, componentwise and outward.
fn sub(a: [Interval; 3], b: [Interval; 3]) -> [Interval; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// **A printed circle's arc on a built circle**, over the arc's trim
/// `[t0, t1]` alone (module docs). Write a printed point as
/// `P(θ) = c₁ + r₁(cos θ·u + sin θ·v)`. Its height over the built plane,
/// `h = (P − c₂)·n`, and its squared distance from the built centre,
/// `S = |P − c₂|²`, are each `α + β cos θ + γ sin θ`, a sinusoid whose
/// extremes over an interval of θ lie at the interval's ends or at
/// `α ± √(β² + γ²)` where its critical angle lies inside; both critical
/// values are taken wherever the angle is within a slack of the trim, so
/// the candidates bound each quantity over the trim. The distance from
/// the built circle is `hypot(h, ρ − r₂)` with `ρ² = S − h²`, so it is
/// at most `hypot(max |h|, max(r₂ − ρ_min, ρ_max − r₂))`, with
/// `ρ_min² ≥ S_min − max h²` and `ρ_max² ≤ S_max`. The circles' frames
/// and their `f64` closed forms are read as exact; every sum is outward.
fn arc_on_circle(
    printed: &geom::Circle<f64>,
    (t0, t1): (f64, f64),
    built: &geom::Circle<f64>,
) -> f64 {
    let iv = Interval::point;
    let (c1, u, n1, r1) = (
        printed.frame.origin,
        printed.frame.u_ref,
        printed.frame.axis,
        printed.radius(),
    );
    let v = n1.cross(u);
    let v_dir = v;
    let (c2, n, r2) = (built.frame.origin, built.frame.axis, built.radius());
    let off = sub(pt(c1), pt(c2));
    let (u, v, n) = (vc(u), vc(v), vc(n));
    let dot = geom_core::interval::dot;
    let r = iv(r1);
    // h = α + β cos θ + γ sin θ; S = A + B cos θ + C sin θ.
    let (alpha, beta, gamma) = (dot(&off, &n), r * dot(&u, &n), r * dot(&v, &n));
    let big_a = geom_core::interval::norm_sq(&off) + r.sqr();
    let (big_b, big_c) = (iv(2.0) * r * dot(&off, &u), iv(2.0) * r * dot(&off, &v));
    let at = |theta: f64| {
        let (sn, cs) = theta.sin_cos();
        (iv(cs), iv(sn))
    };
    let period = geom::conic_period::<f64>().length();
    let slack = 1e-6;
    // Whether the printed point along `(cos, sin)` lies within the slack
    // of the trim, its angle read through the circle's own azimuth.
    let inside = |(cs, sn): (f64, f64)| {
        let q = c1 + printed.frame.u_ref * cs + v_dir * sn;
        let theta = geom::Conic::azimuth(printed, q).near(0.0);
        t1 - t0 >= period - slack || printed.period().above(theta, t0 - slack) <= t1 + slack
    };
    let extremes = |a: Interval, b: Interval, c: Interval| {
        let mut values = Vec::with_capacity(4);
        for t in [t0, t1] {
            let (cs, sn) = at(t);
            values.push(a + b * cs + c * sn);
        }
        let amp = (b.sqr() + c.sqr()).sqrt();
        let (bm, cm) = (
            geom::mid_param(b.lo(), b.hi()),
            geom::mid_param(c.lo(), c.hi()),
        );
        if inside((bm, cm)) {
            values.push(a + amp);
        }
        if inside((-bm, -cm)) {
            values.push(a - amp);
        }
        values
    };
    let hs = extremes(alpha, beta, gamma);
    let ss = extremes(big_a, big_b, big_c);
    let h_max = hs.iter().map(|h| h.mag()).fold(0.0_f64, max_bound);
    let s_min = ss
        .iter()
        .map(|x| x.lo())
        .fold(f64::INFINITY, geom_core::interval::min_bound);
    let s_max = ss.iter().map(|x| x.hi()).fold(0.0_f64, max_bound);
    let inner = iv(s_min) - iv(h_max).sqr();
    let rho_min = if inner.lo() > 0.0 {
        inner.sqrt().lo()
    } else {
        0.0
    };
    let rho_max = iv(s_max).sqrt().hi();
    let radial = (iv(r2) - iv(rho_min))
        .hi()
        .max((iv(rho_max) - iv(r2)).hi())
        .max(0.0);
    (iv(h_max).sqr() + iv(radial).sqr()).sqrt().hi()
}

/// **A conic as `c + cos θ·A + sin θ·B`** (module docs): its centre, its
/// two conjugate semi-diameters, and its plane normal.
struct Conic {
    center: Point3<f64>,
    a: Vec3<f64>,
    b: Vec3<f64>,
    normal: Vec3<f64>,
    /// A circle's radius, which any seam direction in its plane names;
    /// `None` for an ellipse, whose seam is a principal direction.
    radius: Option<f64>,
}

impl Conic {
    /// The circle's or ellipse's form on its own parameter; `None` for
    /// any other curve.
    fn of(curve: &Curve3<f64>) -> Option<Self> {
        match curve {
            Curve3::Circle(c) => {
                let (u, n, r) = (c.frame.u_ref, c.frame.axis, c.radius());
                Some(Self {
                    center: c.frame.origin,
                    a: u * r,
                    b: n.cross(u) * r,
                    normal: n,
                    radius: Some(r),
                })
            }
            Curve3::Ellipse(e) => {
                let (u, n) = (e.frame.u_ref, e.frame.axis);
                Some(Self {
                    center: e.frame.origin,
                    a: u * e.semi_u(),
                    b: n.cross(u) * e.semi_v(),
                    normal: n,
                    radius: None,
                })
            }
            _ => None,
        }
    }

    /// The bound of the module docs on how far `printed` stands from this
    /// (built) conic, over the least of this conic's reparameterizations
    /// read; `None` where none is read (a printed seam normal to a built
    /// circle's plane).
    fn bound_from(&self, printed: &Self) -> Option<f64> {
        let centre = norm_sup(&sub(pt(printed.center), pt(self.center)));
        let gap = |a: Vec3<f64>, b: Vec3<f64>| {
            norm_sup(&sub(vc(printed.a), vc(a))) + norm_sup(&sub(vc(printed.b), vc(b)))
        };
        let pairs: Vec<(Vec3<f64>, Vec3<f64>)> = match self.radius {
            Some(radius) => {
                let n = self.normal;
                let seam = printed.a - n * printed.a.dot(n);
                let (u, _) = seam.checked_direction().ok()?;
                let turn = if printed.normal.dot(n) < 0.0 {
                    -1.0
                } else {
                    1.0
                };
                vec![(u * radius, n.cross(u) * (radius * turn))]
            }
            None => {
                let (a, b) = (self.a, self.b);
                let mut pairs = Vec::with_capacity(8);
                for (x, y) in [(a, b), (b, a)] {
                    for sx in [1.0, -1.0] {
                        for sy in [1.0, -1.0] {
                            pairs.push((x * sx, y * sy));
                        }
                    }
                }
                pairs
            }
        };
        let least = pairs
            .into_iter()
            .map(|(a, b)| gap(a, b))
            .fold(f64::INFINITY, geom_core::interval::min_bound);
        let sum = Interval::point(centre) + Interval::point(least);
        Some(sum.hi())
    }
}

/// Each arm's bound against the TRUE distance read densely off the
/// printed carrier (closed-form distance from each built locus): sound
/// everywhere, blind to nothing the trim holds, and tight where the
/// printed carrier lies on the built one.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::spline::KnotVector;

    fn spec(carrier: Curve3<f64>, (t0, t1): (f64, f64)) -> EdgeSpec {
        EdgeSpec {
            start: 1,
            end: 2,
            carrier,
            t0,
            t1,
            reversed: false,
            curve: None,
        }
    }

    /// The farthest of 4001 points of the printed trim from `truth`.
    fn dense(spec: &EdgeSpec, truth: impl Fn(Point3<f64>) -> f64) -> f64 {
        (0..=4000)
            .map(|k| f64::from(k) / 4000.0)
            .map(|f| truth(spec.carrier.eval(spec.t0 + (spec.t1 - spec.t0) * f)))
            .fold(0.0, f64::max)
    }

    /// Distance from the circle about `c` in the plane normal to unit `n`.
    fn off_circle(p: Point3<f64>, c: Point3<f64>, n: Vec3<f64>, r: f64) -> f64 {
        let d = p - c;
        let h = d.dot(n);
        let rho = (d - n * h).norm();
        h.hypot(rho - r)
    }

    /// Two cubic spans along the `y` axis, `[0, 1]` straight and `[1, 2]`
    /// bulging `off` along `x` with a shape that vanishes at the span's
    /// ends and middle (`3s(1 − s)(1 − 2s)`).
    fn bulging_line(off: f64) -> Curve3<f64> {
        let knots = KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            3,
        )
        .unwrap();
        let y = |v: f64| Point3::new(0.0, v, 0.0);
        let control = vec![
            y(0.0),
            y(1.0 / 3.0),
            y(2.0 / 3.0),
            y(1.0),
            Point3::new(off, 4.0 / 3.0, 0.0),
            Point3::new(-off, 5.0 / 3.0, 0.0),
            y(2.0),
        ];
        Curve3::Nurbs(std::sync::Arc::new(
            NurbsCurve3::new(knots, control, vec![1.0; 7]).unwrap(),
        ))
    }

    #[test]
    fn a_spline_on_a_line_is_bounded_over_its_trim_alone() {
        let line = Curve3::line(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let truth = |p: Point3<f64>| p.x.hypot(p.z);
        let printed = bulging_line(1e-3);
        let straight = spec(printed.clone(), (0.0, 1.0));
        let whole = spec(printed, (0.0, 2.0));
        let on = printed_on(&straight, &line).unwrap();
        assert!(on <= 1e-15, "the straight span is on the line: {on:e}");
        let bound = printed_on(&whole, &line).unwrap();
        let true_sup = dense(&whole, truth);
        assert!(true_sup > 2.5e-4, "the bulge is there: {true_sup:e}");
        assert!(bound >= true_sup, "sound: {bound:e} < {true_sup:e}");
        assert!(bound <= 1e-3 * (1.0 + 1e-12), "the hull bound: {bound:e}");
    }

    #[test]
    fn a_spline_on_a_circle_is_bounded_over_its_trim_alone() {
        // Two rational quarter circles about z of radius 2, the second's
        // middle control pushed out by `push`.
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let push = 1e-4;
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
        let control = vec![
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(2.0, 2.0, 0.0),
            Point3::new(0.0, 2.0, 0.0),
            Point3::new(-2.0 - push, 2.0 + push, 0.0),
            Point3::new(-2.0, 0.0, 0.0),
        ];
        let printed = Curve3::Nurbs(std::sync::Arc::new(
            NurbsCurve3::new(knots, control, vec![1.0, w, 1.0, w, 1.0]).unwrap(),
        ));
        let (c, n) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let built = Curve3::circle(c, n, 2.0, Vec3::new(1.0, 0.0, 0.0));
        let first = spec(printed.clone(), (0.0, 1.0));
        let on = printed_on(&first, &built).unwrap();
        assert!(on <= 1e-14, "the first quarter is on the circle: {on:e}");
        let whole = spec(printed, (0.0, 2.0));
        let bound = printed_on(&whole, &built).unwrap();
        let true_sup = dense(&whole, |p| off_circle(p, c, n, 2.0));
        assert!(true_sup > push / 4.0, "the push shows: {true_sup:e}");
        assert!(bound >= true_sup, "sound: {bound:e} < {true_sup:e}");
        assert!(
            bound <= 4.0 * true_sup,
            "and near it: {bound:e} vs {true_sup:e}"
        );
    }

    #[test]
    fn a_line_on_a_line_reads_its_trim_ends() {
        let built = Curve3::line(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let tilted = Curve3::line(Point3::new(0.0, 1e-6, 0.0), Vec3::new(1.0, 1e-6, 0.0));
        let printed = spec(tilted, (0.0, 3.0));
        let bound = printed_on(&printed, &built).unwrap();
        let true_sup = dense(&printed, |p| p.y.hypot(p.z));
        assert!(bound >= true_sup, "sound: {bound:e} < {true_sup:e}");
        assert!(
            bound <= true_sup * (1.0 + 1e-9),
            "tight: {bound:e} vs {true_sup:e}"
        );
    }

    #[test]
    fn an_arc_on_a_circle_is_read_over_its_trim_not_its_whole_turn() {
        // A printed circle of radius 1 + δ tangent inside the built unit
        // circle at angle 0, tilted by `tilt` about the x axis: over a
        // short arc about the tangency it stays near; over its whole turn
        // it does not.
        let (delta, tilt) = (1e-3_f64, 1e-4_f64);
        let n = Vec3::new(0.0, -tilt.sin(), tilt.cos());
        let printed_circle = Curve3::circle(
            Point3::new(-delta, 0.0, 0.0),
            n,
            1.0 + delta,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let (c, z) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let built = Curve3::circle(c, z, 1.0, Vec3::new(1.0, 0.0, 0.0));
        let truth = |p| off_circle(p, c, z, 1.0);
        for (trim, slack) in [((-0.01, 0.01), 4.0), ((0.5, 1.0), 1.5), ((0.0, 6.0), 1.5)] {
            let arc = spec(printed_circle.clone(), trim);
            let bound = printed_on(&arc, &built).unwrap();
            let true_sup = dense(&arc, truth);
            assert!(
                bound >= true_sup,
                "{trim:?}: sound: {bound:e} < {true_sup:e}"
            );
            assert!(
                bound <= slack * true_sup + 1e-15,
                "{trim:?}: near the truth: {bound:e} vs {true_sup:e}"
            );
        }
    }

    #[test]
    fn an_ellipse_on_its_twin_with_its_axes_swapped_reads_nothing_between() {
        let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
        let (c, z) = (Point3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 1.0));
        let (x, y) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let built = Curve3::ellipse(c, z, 2.0, 1.0, x, band).unwrap();
        // The same locus, seam on the minor axis, wound the other way.
        let twin = Curve3::ellipse(c, -z, 1.0, 2.0, y, band).unwrap();
        let printed = spec(twin, (0.0, 6.0));
        let bound = printed_on(&printed, &built).unwrap();
        assert!(bound <= 1e-14, "one locus: {bound:e}");
        let fat = Curve3::ellipse(c, z, 2.0, 1.0 + 1e-5, x, band).unwrap();
        let printed = spec(fat, (0.0, 6.0));
        let bound = printed_on(&printed, &built).unwrap();
        assert!(bound >= 0.99e-5, "the fatter ellipse stands off: {bound:e}");
    }

    #[test]
    fn a_spline_on_another_spline_has_no_arm_but_its_twin() {
        let printed = bulging_line(0.0);
        let Curve3::Nurbs(n) = &printed else {
            unreachable!()
        };
        let twin = Curve3::Nurbs(std::sync::Arc::new(n.as_ref().clone()));
        assert_eq!(
            printed_on(&spec(printed.clone(), (0.0, 2.0)), &twin),
            Some(0.0)
        );
        let other = bulging_line(1e-300);
        assert_eq!(printed_on(&spec(printed, (0.0, 2.0)), &other), None);
    }

    #[test]
    fn a_point_beyond_a_segments_end_reads_its_distance_from_that_end() {
        let (a, b) = (Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
        let past = point_segment_sup(Point3::new(1.3, 0.4, 0.0), (a, b));
        assert!((past - 0.5).abs() <= 1e-15, "{past}");
        let beside = point_segment_sup(Point3::new(0.5, 0.4, 0.0), (a, b));
        assert!((beside - 0.4).abs() <= 1e-15, "{beside}");
    }
}

/// Reviewer probes (exch-d-rev2).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod rev2_probes {
    use super::*;
    use geom_core::spline::KnotVector;

    fn spec(carrier: Curve3<f64>, (t0, t1): (f64, f64)) -> EdgeSpec {
        EdgeSpec {
            start: 1,
            end: 2,
            carrier,
            t0,
            t1,
            reversed: false,
            curve: None,
        }
    }
    fn dense(spec: &EdgeSpec, truth: impl Fn(Point3<f64>) -> f64) -> f64 {
        (0..=20000)
            .map(|k| f64::from(k) / 20000.0)
            .map(|f| truth(spec.carrier.eval(spec.t0 + (spec.t1 - spec.t0) * f)))
            .fold(0.0, f64::max)
    }
    fn off_circle(p: Point3<f64>, c: Point3<f64>, n: Vec3<f64>, r: f64) -> f64 {
        let d = p - c;
        let h = d.dot(n);
        let rho = (d - n * h).norm();
        h.hypot(rho - r)
    }

    #[test]
    fn rev2_arc_trim_crossing_an_extreme_and_wrapping() {
        let tilt = 1e-3_f64;
        let n = Vec3::new(0.0, -tilt.sin(), tilt.cos());
        let printed = Curve3::circle(
            Point3::new(1e-4, 0.0, 0.0),
            n,
            1.0 + 3e-5,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let (c, z) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let built = Curve3::circle(c, z, 1.0, Vec3::new(1.0, 0.0, 0.0));
        let tp = core::f64::consts::TAU;
        for trim in [
            (1.2, 2.0),
            (6.0, 8.0),
            (tp - 0.3, tp + 0.3),
            (3.0, 3.3),
            (4.5, 4.9),
            (1.5707963, 1.5707964),
            (0.1, tp + 0.05),
        ] {
            let s = spec(printed.clone(), trim);
            let b = printed_on(&s, &built).unwrap();
            let t = dense(&s, |p| off_circle(p, c, z, 1.0));
            eprintln!("REV2 arc {trim:?}: bound {b:e} true {t:e} ratio {}", b / t);
            assert!(b >= t, "{trim:?}: unsound {b:e} < {t:e}");
        }
    }

    fn two_quarters(push: f64) -> Curve3<f64> {
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
        let control = vec![
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(2.0, 2.0, 0.0),
            Point3::new(0.0, 2.0, 0.0),
            Point3::new(-2.0 - push, 2.0 + push, 0.0),
            Point3::new(-2.0, 0.0, 0.0),
        ];
        Curve3::Nurbs(std::sync::Arc::new(
            NurbsCurve3::new(knots, control, vec![1.0, w, 1.0, w, 1.0]).unwrap(),
        ))
    }

    #[test]
    fn rev2_spline_on_circle_trim_off_the_knots() {
        let (c, n) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let built = Curve3::circle(c, n, 2.0, Vec3::new(1.0, 0.0, 0.0));
        let printed = two_quarters(1e-4);
        for trim in [(0.5, 1.5), (1.2, 1.8), (0.0, 1.5), (0.5, 2.0), (0.2, 0.8)] {
            let s = spec(printed.clone(), trim);
            let b = printed_on(&s, &built);
            let t = dense(&s, |p| off_circle(p, c, n, 2.0));
            eprintln!("REV2 spline-circle {trim:?}: bound {b:?} true {t:e}");
            let b = b.expect("an arm serves an off-knot trim");
            assert!(b >= t, "{trim:?}: unsound {b:e} < {t:e}");
        }
    }

    #[test]
    fn rev2_spline_on_line_trim_off_the_knots() {
        let line = Curve3::line(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let knots = KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            3,
        )
        .unwrap();
        let y = |v: f64| Point3::new(0.0, v, 0.0);
        let off = 1e-3;
        let control = vec![
            y(0.0),
            y(1.0 / 3.0),
            y(2.0 / 3.0),
            y(1.0),
            Point3::new(off, 4.0 / 3.0, 0.0),
            Point3::new(-off, 5.0 / 3.0, 0.0),
            y(2.0),
        ];
        let printed = Curve3::Nurbs(std::sync::Arc::new(
            NurbsCurve3::new(knots, control, vec![1.0; 7]).unwrap(),
        ));
        for trim in [(0.5, 1.5), (1.1, 1.3), (0.0, 1.0), (0.9, 1.05)] {
            let s = spec(printed.clone(), trim);
            let b = printed_on(&s, &line).unwrap();
            let t = dense(&s, |p| p.x.hypot(p.z));
            eprintln!("REV2 spline-line {trim:?}: bound {b:e} true {t:e}");
            assert!(b >= t, "{trim:?}: unsound {b:e} < {t:e}");
        }
    }

    /// An overshoot along the line: on the infinite line, past the segment.
    #[test]
    fn rev2_overshoot_along_a_line() {
        let line = Curve3::line(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(6.0, 0.0, 0.0),
            Point3::new(-5.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ];
        let printed = Curve3::Nurbs(std::sync::Arc::new(
            NurbsCurve3::new(knots, control, vec![1.0; 4]).unwrap(),
        ));
        let s = spec(printed, (0.0, 1.0));
        let b = printed_on(&s, &line).unwrap();
        let seg = printed_on_segment(&s, (Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)))
            .unwrap();
        eprintln!("REV2 overshoot: line bound {b:e}, segment bound {seg:e}");
    }
}
