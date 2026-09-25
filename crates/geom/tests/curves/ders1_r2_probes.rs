//! Review probes for the whole-curve order-1 jet `ders1`: adversarial
//! parameters and fixtures the shipped differential rows do not draw —
//! an `Interval` overlapping exactly two spans of unequal widths, a
//! repeated interior knot, degree 1, a single-span (Bézier) curve, a
//! rational curve with a near-zero weight, a `Dual64` whose derivative
//! channel is not 1, the `Curve3::Nurbs` arm at `Dual64` and `Interval`,
//! and the analytic arms at `Dual64` and `Interval`. Each half of the
//! jet is compared to its own evaluator by bits.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3};
use geom_core::spline::{KnotVector, SpanLocate};
use geom_core::{Bounds, Interval};
use geom_core::{Dual64, Point2, Point3, Real, Vec3};

trait Bits {
    fn bits(self) -> Vec<u64>;
}
impl Bits for f64 {
    fn bits(self) -> Vec<u64> {
        vec![self.to_bits()]
    }
}
impl Bits for Dual64 {
    fn bits(self) -> Vec<u64> {
        vec![self.value.to_bits(), self.deriv.to_bits()]
    }
}
impl Bits for Interval {
    fn bits(self) -> Vec<u64> {
        vec![self.lo().to_bits(), self.hi().to_bits()]
    }
}

fn check3<T: SpanLocate + Bits>(c: &Curve3<T>, t: T, what: &str) {
    let (p, d) = c.ders1(t);
    let (q, e) = (c.eval(t), c.deriv(t));
    for (n, a, b) in [
        ("x", p.x, q.x),
        ("y", p.y, q.y),
        ("z", p.z, q.z),
        ("dx", d.x, e.x),
        ("dy", d.y, e.y),
        ("dz", d.z, e.z),
    ] {
        assert_eq!(
            a.bits(),
            b.bits(),
            "{what}: {n} differs between ders1 and its evaluator"
        );
    }
}

fn check2<T: SpanLocate + Bits>(c: &NurbsCurve2<T>, t: T, what: &str) {
    let (p, d) = c.ders1(t);
    let (q, e) = (c.eval(t), c.deriv(t));
    for (n, a, b) in [
        ("x", p.x, q.x),
        ("y", p.y, q.y),
        ("dx", d.x, e.x),
        ("dy", d.y, e.y),
    ] {
        assert_eq!(
            a.bits(),
            b.bits(),
            "{what}: {n} differs between ders1 and its evaluator"
        );
    }
}

fn kv(k: &[f64], p: usize) -> KnotVector {
    KnotVector::clamped(k.to_vec(), p).unwrap()
}

fn control(n: usize) -> Vec<Point3<f64>> {
    (0..n)
        .map(|i| {
            let x = i as f64;
            Point3::new(
                x * 0.37 - 1.1,
                (x * 1.3).sin() * 0.9,
                (x * 0.6).cos() * -0.7,
            )
        })
        .collect()
}

fn lift3<T: Real>(k: &KnotVector, w: &[f64]) -> Curve3<T> {
    let n = k.control_count();
    let ctl: Vec<Point3<T>> = control(n)
        .iter()
        .map(|p| Point3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z)))
        .collect();
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(k.clone(), ctl, w.to_vec()).unwrap(),
    ))
}

fn lift2<T: Real>(k: &KnotVector, w: &[f64]) -> NurbsCurve2<T> {
    let n = k.control_count();
    let ctl: Vec<Point2<T>> = control(n)
        .iter()
        .map(|p| Point2::new(T::from_f64(p.x), T::from_f64(p.y)))
        .collect();
    NurbsCurve2::new(k.clone(), ctl, w.to_vec()).unwrap()
}

/// The fixtures: (name, knots, weights), every one a shape the shipped
/// corpus does not carry.
fn fixtures() -> Vec<(&'static str, KnotVector, Vec<f64>)> {
    let mut out = Vec::new();
    // Degree 1, two spans of unequal widths (0.2 and 0.8).
    let k = kv(&[0.0, 0.0, 0.2, 1.0, 1.0], 1);
    let n = k.control_count();
    out.push(("deg1_unequal", k, vec![1.0; n]));
    // Degree 2, a repeated interior knot (C0) at 0.3, unequal spans.
    let k = kv(&[0.0, 0.0, 0.0, 0.3, 0.3, 1.0, 1.0, 1.0], 2);
    let n = k.control_count();
    out.push(("deg2_double_knot", k.clone(), vec![1.0; n]));
    out.push((
        "deg2_double_knot_rational",
        k,
        vec![0.7, 2.5, 0.05, 1.3, 0.9],
    ));
    // Single span (Bézier), degree 3.
    let k = kv(&[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3);
    out.push(("bezier3", k.clone(), vec![1.0; 4]));
    // Rational with a near-zero weight, single span.
    out.push(("bezier3_tiny_weight", k, vec![1.0, 1e-300, 3.0, 1e-8]));
    // Degree 3, seven interior knots at irregular places, rational.
    let k = kv(
        &[
            0.0, 0.0, 0.0, 0.0, 0.05, 0.1, 0.35, 0.36, 0.7, 0.9, 0.99, 1.0, 1.0, 1.0, 1.0,
        ],
        3,
    );
    let n = k.control_count();
    let w: Vec<f64> = (0..n).map(|i| 0.3 + ((i * 7) % 6) as f64 * 0.55).collect();
    out.push(("deg3_irregular_rational", k, w));
    // Degree 5, one interior knot of multiplicity 4.
    let k = kv(
        &[
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ],
        5,
    );
    let n = k.control_count();
    out.push(("deg5_mult4", k, vec![1.0; n]));
    out
}

/// Every knot, every span midpoint, both ends, and a few points that
/// are not special to any fixture.
fn point_params(k: &KnotVector) -> Vec<f64> {
    let mut ts: Vec<f64> = k.knots().to_vec();
    for i in k.first_span()..=k.last_span() {
        ts.push(0.5 * (k.knots()[i] + k.knots()[i + 1]));
    }
    let (lo, hi) = k.domain();
    for f in [
        0.001,
        0.123_456_789,
        0.299_999_999,
        0.300_000_001,
        0.5,
        0.777,
        0.999,
    ] {
        ts.push((hi - lo).mul_add(f, lo));
    }
    ts.sort_by(f64::total_cmp);
    ts.dedup();
    ts
}

#[test]
fn r2_nurbs_arm_point_scalars_bit_for_bit() {
    let mut checked = 0usize;
    for (name, k, w) in fixtures() {
        let c: Curve3<f64> = lift3(&k, &w);
        let cd: Curve3<Dual64> = lift3(&k, &w);
        let c2: NurbsCurve2<f64> = lift2(&k, &w);
        let cd2: NurbsCurve2<Dual64> = lift2(&k, &w);
        for t in point_params(&k) {
            check3(&c, t, &format!("{name} f64 @{t}"));
            check3(&cd, Dual64::variable(t), &format!("{name} dual @{t}"));
            // A derivative channel that is not 1: the jet's Dual
            // tangent channel must still be `deriv`'s.
            check3(
                &cd,
                Dual64::new(t, -3.7),
                &format!("{name} dual(-3.7) @{t}"),
            );
            check3(&cd, Dual64::new(t, 0.0), &format!("{name} dual(0) @{t}"));
            check2(&c2, t, &format!("{name} 2d f64 @{t}"));
            check2(&cd2, Dual64::variable(t), &format!("{name} 2d dual @{t}"));
            checked += 6;
        }
    }
    assert!(checked >= 200, "{checked}");
}

#[test]
fn r2_nurbs_arm_interval_hulls_bit_for_bit() {
    let mut checked = 0usize;
    for (name, k, w) in fixtures() {
        let ci: Curve3<Interval> = lift3(&k, &w);
        let ci2: NurbsCurve2<Interval> = lift2(&k, &w);
        let (lo, hi) = k.domain();
        let knots = k.knots();
        let mut ivs: Vec<Interval> = Vec::new();
        // Every pair of consecutive distinct knots: an interval that
        // overlaps exactly those two spans, off-centre on the knot.
        let mut distinct: Vec<f64> = knots.to_vec();
        distinct.dedup();
        for win in distinct.windows(3) {
            let (a, b, c) = (win[0], win[1], win[2]);
            ivs.push(Interval::from_bounds(a + 0.75 * (b - a), b + 0.1 * (c - b)));
            ivs.push(Interval::from_bounds(a + 0.1 * (b - a), b + 0.9 * (c - b)));
        }
        // Three-span windows, every knot value degenerate, the whole
        // domain, each end pinned, and an interval strictly inside
        // one span.
        for win in distinct.windows(4) {
            ivs.push(Interval::from_bounds(
                win[0] + 0.5 * (win[1] - win[0]),
                win[2] + 0.5 * (win[3] - win[2]),
            ));
        }
        for &t in knots {
            ivs.push(Interval::from_bounds(t, t));
        }
        ivs.push(Interval::from_bounds(lo, hi));
        ivs.push(Interval::from_bounds(lo, lo + 0.5 * (hi - lo)));
        ivs.push(Interval::from_bounds(lo + 0.5 * (hi - lo), hi));
        ivs.push(Interval::from_bounds(
            lo + 0.01 * (hi - lo),
            lo + 0.011 * (hi - lo),
        ));
        for iv in ivs {
            let what = format!("{name} iv [{}, {}]", iv.lo(), iv.hi());
            check3(&ci, iv, &what);
            check2(&ci2, iv, &format!("{what} 2d"));
            checked += 2;
        }
    }
    assert!(checked >= 80, "{checked}");
}

fn tilted_circle<T: Real>() -> Curve3<T> {
    let f = T::from_f64;
    Curve3::Circle {
        center: Point3::new(f(-0.5), f(4.0), f(1.25)),
        axis: Vec3::new(f(2.0 / 3.0), f(2.0 / 3.0), f(1.0 / 3.0)),
        radius: f(2.5),
        u_ref: Vec3::new(f(1.0 / 3.0), f(-2.0 / 3.0), f(2.0 / 3.0)),
    }
}

fn tilted_ellipse<T: Real>() -> Curve3<T> {
    let f = T::from_f64;
    Curve3::Ellipse {
        center: Point3::new(f(-0.5), f(4.0), f(1.25)),
        axis: Vec3::new(f(2.0 / 3.0), f(2.0 / 3.0), f(1.0 / 3.0)),
        major: f(2.5),
        minor: f(1.0),
        u_ref: Vec3::new(f(1.0 / 3.0), f(-2.0 / 3.0), f(2.0 / 3.0)),
    }
}

fn line<T: Real>() -> Curve3<T> {
    let f = T::from_f64;
    Curve3::Line {
        origin: Point3::new(f(1.0), f(-2.0), f(0.5)),
        dir: Vec3::new(f(0.3), f(-0.4), f(1.2)),
    }
}

const ANGLES: [f64; 12] = [
    -1e6,
    -7.3,
    -1.0,
    0.0,
    1e-9,
    0.37,
    1.0,
    std::f64::consts::FRAC_PI_2,
    2.9,
    std::f64::consts::PI,
    std::f64::consts::TAU,
    41.5,
];

/// The analytic arms at `Dual64` (both channels) — the shipped row
/// draws only `f64`.
#[test]
fn r2_analytic_arms_dual_bit_for_bit() {
    for (kind, c) in [
        ("line", line::<Dual64>()),
        ("circle", tilted_circle::<Dual64>()),
        ("ellipse", tilted_ellipse::<Dual64>()),
    ] {
        for t in ANGLES {
            check3(&c, Dual64::variable(t), &format!("{kind} dual @{t}"));
            check3(&c, Dual64::new(t, 2.5), &format!("{kind} dual(2.5) @{t}"));
        }
    }
}

/// The analytic arms at `Interval` (both bounds), on wide and
/// degenerate intervals.
#[test]
fn r2_analytic_arms_interval_bit_for_bit() {
    for (kind, c) in [
        ("line", line::<Interval>()),
        ("circle", tilted_circle::<Interval>()),
        ("ellipse", tilted_ellipse::<Interval>()),
    ] {
        for t in ANGLES {
            check3(
                &c,
                Interval::from_bounds(t, t),
                &format!("{kind} iv[{t},{t}]"),
            );
            check3(
                &c,
                Interval::from_bounds(t - 0.3, t + 0.2),
                &format!("{kind} iv around {t}"),
            );
            check3(
                &c,
                Interval::from_bounds(t, t + 7.0),
                &format!("{kind} iv wide {t}"),
            );
        }
    }
}

/// Parameters outside the domain (the span's polynomial extension) and
/// a NaN parameter: whatever the pair answers, the jet answers the
/// same bits.
#[test]
fn r2_nurbs_arm_out_of_domain_and_nan() {
    for (name, k, w) in fixtures() {
        let c: Curve3<f64> = lift3(&k, &w);
        let (lo, hi) = k.domain();
        for t in [lo - 0.5, hi + 0.5, f64::NAN, f64::INFINITY, -f64::INFINITY] {
            check3(&c, t, &format!("{name} f64 @{t}"));
        }
        let cd: Curve3<Dual64> = lift3(&k, &w);
        for t in [lo - 0.5, hi + 0.5, f64::NAN] {
            check3(&cd, Dual64::variable(t), &format!("{name} dual @{t}"));
        }
    }
}
