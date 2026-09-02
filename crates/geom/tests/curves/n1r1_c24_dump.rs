//! CERT-N1 R1 reviewer probe — C24 bit-identity harness.
//!
//! Dumps every evaluator component as raw bits for a corpus of curves ×
//! parameters to `$CAD_R1_DUMP`. Compiles at both `e43a9a116` (the
//! retired spelling) and the head, so the two dumps diff directly.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve2, NurbsCurve3};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3};

fn corpus3() -> Vec<(&'static str, NurbsCurve3<f64>)> {
    let mut out = Vec::new();
    // 1. polynomial cubic, uniform interior knots
    let kv = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0],
        3,
    )
    .unwrap();
    let control: Vec<Point3<f64>> = (0..7)
        .map(|i| {
            let t = i as f64;
            Point3::new(t * 0.31, (t * 1.1).sin(), (t * 0.7).cos() * 2.0)
        })
        .collect();
    out.push((
        "cubic_poly",
        NurbsCurve3::new(kv.clone(), control.clone(), vec![1.0; 7]).unwrap(),
    ));
    // 2. same net, extreme weights
    out.push((
        "cubic_extreme_w",
        NurbsCurve3::new(
            kv,
            control,
            vec![1.0, 1e9, 1e-9, 4.0, 1e7, 1e-7, 1.0],
        )
        .unwrap(),
    ));
    // 3. rational quadratic full circle
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let kc = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0],
        2,
    )
    .unwrap();
    let p = |x: f64, y: f64| Point3::new(x, y, 0.5 * x - 0.25 * y);
    out.push((
        "circle",
        NurbsCurve3::new(
            kc,
            vec![
                p(1.0, 0.0),
                p(1.0, 1.0),
                p(0.0, 1.0),
                p(-1.0, 1.0),
                p(-1.0, 0.0),
                p(-1.0, -1.0),
                p(0.0, -1.0),
                p(1.0, -1.0),
                p(1.0, 0.0),
            ],
            vec![1.0, s, 1.0, s, 1.0, s, 1.0, s, 1.0],
        )
        .unwrap(),
    ));
    // 4. degree 5, interior multiplicity p-1
    let k5 = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.4, 0.4, 0.4, 0.4, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ],
        5,
    )
    .unwrap();
    out.push((
        "deg5_mult4",
        NurbsCurve3::new(
            k5,
            (0..10)
                .map(|i| {
                    let t = i as f64;
                    Point3::new(t * 0.37 - 1.0, (t * 0.9).sin() * 3.0, t * t * 0.05)
                })
                .collect(),
            vec![1.0, 1e8, 1e-8, 3.5, 1e6, 1e-6, 2.0, 1e7, 1e-7, 1.0],
        )
        .unwrap(),
    ));
    // 5. single Bezier span, degree 2, rational
    out.push((
        "bezier_q",
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::new(2.0, 0.0, 0.0),
                Point3::new(2.0, 2.0, 0.0),
                Point3::new(0.0, 2.0, 0.0),
            ],
            vec![1.0, s, 1.0],
        )
        .unwrap(),
    ));
    out
}

#[test]
fn n1r1_c24_dump() {
    let Ok(path) = std::env::var("CAD_R1_DUMP") else {
        return;
    };
    let mut s = String::new();
    let mut n = 0usize;
    for (name, c) in corpus3() {
        let (lo, hi) = c.domain();
        for i in 0..=2000 {
            let t = lo + (hi - lo) * (i as f64) / 2000.0;
            let p = c.eval(t);
            let d = c.deriv(t);
            let d2 = c.deriv2(t);
            for v in [p.x, p.y, p.z, d.x, d.y, d.z, d2.x, d2.y, d2.z] {
                s.push_str(&format!("{name} {i} {:016x}\n", v.to_bits()));
                n += 1;
            }
        }
        // In-span doors at every span, at both ends and the middle.
        for si in 0..c.knots().knots().len() {
            let Some(span) = c.knots().span(si) else {
                continue;
            };
            for f in [0.0, 0.5, 1.0, 0.125, 0.875] {
                let t = lo + (hi - lo) * f;
                let a = c.eval_in_span(span, t);
                let b = c.deriv_in_span(span, t);
                let e = c.deriv2_in_span(span, t);
                let (j0, j1, j2) = c.ders_in_span(span, t);
                for v in [
                    a.x, a.y, a.z, b.x, b.y, b.z, e.x, e.y, e.z, j0.x, j0.y, j0.z, j1.x, j1.y,
                    j1.z, j2.x, j2.y, j2.z,
                ] {
                    s.push_str(&format!("{name} span{si} {f} {:016x}\n", v.to_bits()));
                    n += 1;
                }
            }
        }
    }
    // 2-D arm
    let c2 = NurbsCurve2::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 4.0),
            Point2::new(2.0, -1.0),
            Point2::new(3.0, 2.0),
            Point2::new(4.0, 0.0),
        ],
        vec![1.0, 1e9, 1e-9, 5.0, 1.0],
    )
    .unwrap();
    for i in 0..=2000 {
        let t = i as f64 / 2000.0;
        let p = c2.eval(t);
        let d = c2.deriv(t);
        let d2 = c2.deriv2(t);
        for v in [p.x, p.y, d.x, d.y, d2.x, d2.y] {
            s.push_str(&format!("c2 {i} {:016x}\n", v.to_bits()));
            n += 1;
        }
    }
    s.push_str(&format!("components {n}\n"));
    std::fs::write(path, s).unwrap();
}
