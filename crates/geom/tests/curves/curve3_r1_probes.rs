//! R1 review-lane probes for CURVE3-JET: adversarial inputs for the
//! whole-curve order-1 jet doors, beyond the corpus the unit's rows
//! carry. Ordinary tests; they assert bit identity between `ders1`
//! and the `eval`/`deriv` pair it replaces.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3};
use geom_core::spline::{KnotVector, SpanLocate};
use geom_core::{Dual64, Point2, Point3, Real, Vec3};

fn kv(k: &[f64], p: usize) -> KnotVector {
    KnotVector::clamped(k.to_vec(), p).unwrap()
}

fn curve3(k: &KnotVector, weights: Vec<f64>) -> NurbsCurve3<f64> {
    let n = k.control_count();
    assert_eq!(n, weights.len());
    let control: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            Point3::new(
                i as f64 * 0.73 - 1.1,
                ((i * 3) % 7) as f64 * 0.41,
                ((i * 5) % 3) as f64 * -0.62,
            )
        })
        .collect();
    NurbsCurve3::new(k.clone(), control, weights).unwrap()
}

fn lift3<T: Real>(c: &NurbsCurve3<f64>) -> NurbsCurve3<T> {
    NurbsCurve3::new(
        c.knots().clone(),
        c.control().iter().map(|p| p.map(T::from_f64)).collect(),
        c.weights().to_vec(),
    )
    .unwrap()
}

fn lift2<T: Real>(c: &NurbsCurve3<f64>) -> NurbsCurve2<T> {
    NurbsCurve2::new(
        c.knots().clone(),
        c.control()
            .iter()
            .map(|p| Point2::new(T::from_f64(p.x), T::from_f64(p.z)))
            .collect(),
        c.weights().to_vec(),
    )
    .unwrap()
}

fn same3<T: Real + SpanLocate>(c: &NurbsCurve3<T>, t: T, bits: fn(T) -> Vec<u64>, what: &str) {
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
        assert_eq!(bits(a), bits(b), "{what}: {n}");
    }
}

fn same2<T: Real + SpanLocate>(c: &NurbsCurve2<T>, t: T, bits: fn(T) -> Vec<u64>, what: &str) {
    let (p, d) = c.ders1(t);
    let (q, e) = (c.eval(t), c.deriv(t));
    for (n, a, b) in [
        ("x", p.x, q.x),
        ("y", p.y, q.y),
        ("dx", d.x, e.x),
        ("dy", d.y, e.y),
    ] {
        assert_eq!(bits(a), bits(b), "{what}: {n}");
    }
}

fn same_enum(c: &Curve3<f64>, t: f64, what: &str) {
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
        assert_eq!(a.to_bits(), b.to_bits(), "{what}: {n} {a} vs {b}");
    }
}

fn bf64(x: f64) -> Vec<u64> {
    vec![x.to_bits()]
}
fn bdual(x: Dual64) -> Vec<u64> {
    vec![x.value.to_bits(), x.deriv.to_bits()]
}

/// The corpus the unit's `_ext` row does not carry: single-span
/// (Bezier) curves at four degrees, near-zero weights, a two-span
/// curve whose spans differ in width by six orders of magnitude, and
/// a knot vector with an interior multiplicity equal to the degree.
fn adversarial() -> Vec<(&'static str, NurbsCurve3<f64>)> {
    let mut out = Vec::new();
    for p in [1usize, 2, 3, 5] {
        let mut k = vec![0.0; p + 1];
        k.extend(std::iter::repeat_n(1.0, p + 1));
        let kvp = kv(&k, p);
        let n = kvp.control_count();
        let (plain, tiny) = match p {
            1 => ("bezier1", "bezier1_tinyw"),
            2 => ("bezier2", "bezier2_tinyw"),
            3 => ("bezier3", "bezier3_tinyw"),
            _ => ("bezier5", "bezier5_tinyw"),
        };
        out.push((plain, curve3(&kvp, vec![1.0; n])));
        let mut w = vec![1.0; n];
        w[n / 2] = 1e-300;
        out.push((tiny, curve3(&kvp, w)));
    }
    // Two spans of wildly unequal width.
    let k = kv(&[0.0, 0.0, 0.0, 1e-6, 5.0, 5.0, 5.0], 2);
    let n = k.control_count();
    out.push(("lopsided", curve3(&k, vec![1.0; n])));
    out.push(("lopsided_rat", curve3(&k, vec![0.3, 1e-9, 7.0, 2.0])));
    // Interior multiplicity == degree (C0), degree 3.
    let k = kv(
        &[0.0, 0.0, 0.0, 0.0, 0.25, 0.25, 0.25, 1.0, 1.0, 1.0, 1.0],
        3,
    );
    let n = k.control_count();
    out.push(("c0_deg3", curve3(&k, vec![1.0; n])));
    let w: Vec<f64> = (0..n).map(|i| 0.2 + 1.7 * (i % 3) as f64).collect();
    out.push(("c0_deg3_rat", curve3(&k, w)));
    // Degree 1, many spans, unequal widths.
    let k = kv(&[0.0, 0.0, 1e-3, 0.4, 0.400_000_1, 2.0, 2.0], 1);
    let n = k.control_count();
    out.push(("deg1_uneven", curve3(&k, vec![1.0; n])));
    out
}

fn probe_params(c: &NurbsCurve3<f64>) -> Vec<f64> {
    let knots = c.knots().knots();
    let mut ts: Vec<f64> = knots.to_vec();
    ts.extend(knots.windows(2).filter(|w| w[1] > w[0]).flat_map(|w| {
        let (a, b) = (w[0], w[1]);
        [
            0.5 * (a + b),
            (b - a).mul_add(1e-12, a),
            (b - a).mul_add(-1e-12, b),
            (b - a).mul_add(0.999_999, a),
        ]
    }));
    ts.sort_by(f64::total_cmp);
    ts.dedup();
    ts
}

#[test]
fn probe_nurbs_ders1_is_the_pair_on_adversarial_curves_f64_and_dual() {
    let mut checked = 0usize;
    for (name, c) in adversarial() {
        let cd: NurbsCurve3<Dual64> = lift3(&c);
        let c2: NurbsCurve2<f64> = lift2(&c);
        let cd2: NurbsCurve2<Dual64> = lift2(&c);
        let enum_c = Curve3::Nurbs(Arc::new(c.clone()));
        for t in probe_params(&c) {
            same3(&c, t, bf64, &format!("{name} f64 @{t}"));
            same3(
                &cd,
                Dual64::variable(t),
                bdual,
                &format!("{name} dual @{t}"),
            );
            same2(&c2, t, bf64, &format!("{name} 2d @{t}"));
            same2(
                &cd2,
                Dual64::variable(t),
                bdual,
                &format!("{name} 2d dual @{t}"),
            );
            same_enum(&enum_c, t, &format!("{name} enum @{t}"));
            checked += 5;
        }
        // Out-of-domain parameters: the locator decides where these
        // land, and both doors must land them the same way.
        let (lo, hi) = c.knots().domain();
        for t in [lo - 1.0, hi + 1.0, lo - 1e-14, hi + 1e-14] {
            same3(&c, t, bf64, &format!("{name} oob @{t}"));
            same_enum(&enum_c, t, &format!("{name} enum oob @{t}"));
            checked += 2;
        }
    }
    assert!(checked > 400, "probe corpus shrank: {checked}");
}

#[test]
fn probe_nurbs_ders1_is_the_pair_on_adversarial_curves_interval() {
    use geom_core::{Bounds, Interval};
    let mut checked = 0usize;
    for (name, c) in adversarial() {
        let ci: NurbsCurve3<Interval> = lift3(&c);
        let ci2: NurbsCurve2<Interval> = lift2(&c);
        let (lo, hi) = c.knots().domain();
        let span = hi - lo;
        let mut ivs: Vec<Interval> = Vec::new();
        for t in probe_params(&c) {
            // Degenerate, a hair either side of the knot, two
            // asymmetric straddles, and one that eats the domain.
            ivs.push(Interval::from_bounds(t, t));
            ivs.push(Interval::from_bounds(
                span.mul_add(-1e-13, t),
                span.mul_add(1e-13, t),
            ));
            ivs.push(Interval::from_bounds(
                span.mul_add(-0.3, t),
                span.mul_add(0.02, t),
            ));
            ivs.push(Interval::from_bounds(
                span.mul_add(-0.02, t),
                span.mul_add(0.3, t),
            ));
        }
        ivs.push(Interval::from_bounds(lo, hi));
        ivs.push(Interval::from_bounds(lo - span, hi + span));
        for iv in ivs {
            let what = format!("{name} iv [{}, {}]", iv.lo(), iv.hi());
            same3(&ci, iv, |x| vec![x.lo().to_bits(), x.hi().to_bits()], &what);
            same2(
                &ci2,
                iv,
                |x| vec![x.lo().to_bits(), x.hi().to_bits()],
                &format!("{what} 2d"),
            );
            checked += 2;
        }
    }
    assert!(checked > 400, "probe corpus shrank: {checked}");
}

/// Totality and poison for the jet door in one place — the same census
/// `curves.rs`'s `poison_parameter_poisons_the_point`,
/// `extreme_parameters_do_not_panic` and
/// `nurbs_placeholder_evaluates_to_poison` rows keep beside the doors
/// they enumerate — with both halves read at every poison input.
#[test]
fn probe_ders1_is_total_and_poisons_like_its_evaluators() {
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let u_ref = Vec3::new(1.0, 0.0, 0.0);
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis,
        radius: 2.0,
        u_ref,
    };
    let (p, d) = circle.ders1(f64::NAN);
    assert!(p.x.is_nan() && p.y.is_nan() && p.z.is_nan(), "point {p:?}");
    assert!(
        d.x.is_nan() && d.y.is_nan() && d.z.is_nan(),
        "tangent {d:?}"
    );
    for t in [f64::INFINITY, f64::NEG_INFINITY, 1e300, -1e300, f64::MAX] {
        let _ = circle.ders1(t);
    }
    let (p, d) = circle.ders1(f64::INFINITY);
    assert!(p.x.is_nan() && p.y.is_nan() && p.z.is_nan(), "point {p:?}");
    assert!(
        d.x.is_nan() && d.y.is_nan() && d.z.is_nan(),
        "tangent {d:?}"
    );

    let line = Curve3::Line {
        origin: Point3::origin(),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    // The line's tangent is data: a poison parameter does not reach it.
    assert!((line.ders1(f64::NAN).1.x - 1.0).abs() < f64::EPSILON);

    let n: Curve3<f64> = Curve3::nurbs_placeholder();
    let (p, d) = n.ders1(0.5);
    assert!(p.x.is_nan() && p.y.is_nan() && p.z.is_nan(), "point {p:?}");
    assert!(
        d.x.is_nan() && d.y.is_nan() && d.z.is_nan(),
        "tangent {d:?}"
    );
}

/// The enum's analytic arms at parameters the unit's row does not
/// reach: very large and very small magnitudes, exact multiples of
/// the trig period, subnormals, and negative zero.
#[test]
fn probe_analytic_arms_at_extreme_parameters() {
    let axis = Vec3::new(0.31, -0.72, 0.61).normalize();
    let u_ref = {
        let raw = Vec3::new(0.83, 0.19, -0.52);
        (raw - axis * raw.dot(axis)).normalize()
    };
    let center = Point3::new(-1.7, 0.4, 2.2);
    let curves = [
        (
            "line",
            Curve3::Line {
                origin: center,
                dir: Vec3::new(0.3, -0.4, 1.2),
            },
        ),
        (
            "circle",
            Curve3::Circle {
                center,
                axis,
                radius: 3.25,
                u_ref,
            },
        ),
        (
            "circle_tiny",
            Curve3::Circle {
                center,
                axis,
                radius: 1e-300,
                u_ref,
            },
        ),
        (
            "ellipse",
            Curve3::Ellipse {
                center,
                axis,
                major: 4.5,
                minor: 0.125,
                u_ref,
            },
        ),
        (
            "ellipse_degenerate",
            Curve3::Ellipse {
                center,
                axis,
                major: 1e12,
                minor: 1e-12,
                u_ref,
            },
        ),
    ];
    let params = [
        0.0,
        -0.0,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        1e-300,
        1e-17,
        std::f64::consts::FRAC_PI_2,
        std::f64::consts::PI,
        std::f64::consts::TAU,
        -std::f64::consts::TAU,
        1e6,
        -1e9,
        1e15,
        f64::MAX.sqrt(),
    ];
    for (name, c) in curves {
        for t in params {
            same_enum(&c, t, &format!("{name} @{t}"));
        }
    }
}
