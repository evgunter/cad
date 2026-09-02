//! CERT-N1 R2: the C24 measurement re-taken. `#[ignore]`d; run in
//! release with `--ignored --nocapture`. Degrees 2/3/5/7 with 3/8/8/8
//! interior knots, as the PR body's table.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::hint::black_box;
use std::time::Instant;

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::Point3;

fn curve(p: usize, interior: usize) -> NurbsCurve3<f64> {
    let mut knots = vec![0.0; p + 1];
    for i in 1..=interior {
        knots.push(i as f64 / (interior + 1) as f64);
    }
    knots.extend(vec![1.0; p + 1]);
    let n = knots.len() - p - 1;
    let control = (0..n).map(|i| Point3::new(i as f64, (i * i) as f64 * 0.1, -(i as f64))).collect();
    let weights = (0..n).map(|i| 1.0 + 0.3 * (i % 3) as f64).collect();
    NurbsCurve3::new(KnotVector::clamped(knots, p).unwrap(), control, weights).unwrap()
}

#[test]
#[ignore]
fn n1r2_c24_timing() {
    let iters = 200_000u32;
    for (p, interior) in [(2usize, 3usize), (3, 8), (5, 8), (7, 8)] {
        let c = curve(p, interior);
        let kv = c.knots();
        let spans: Vec<_> = (0..kv.knots().len()).filter_map(|i| kv.span(i)).collect();
        let ts: Vec<f64> = (0..64).map(|i| (i as f64 + 0.5) / 64.0).collect();
        let pick = |i: u32| {
            let t = ts[(i as usize) % ts.len()];
            let s = spans[(i as usize) % spans.len()];
            (s, t)
        };
        let time = |f: &dyn Fn(u32)| {
            for i in 0..iters / 10 {
                f(i);
            }
            let t0 = Instant::now();
            for i in 0..iters {
                f(i);
            }
            t0.elapsed().as_nanos() as f64 / iters as f64
        };
        let eval = time(&|i| {
            let (s, t) = pick(i);
            black_box(c.eval_in_span(s, black_box(t)));
        });
        let d1 = time(&|i| {
            let (s, t) = pick(i);
            black_box(c.deriv_in_span(s, black_box(t)));
        });
        let ders = time(&|i| {
            let (s, t) = pick(i);
            black_box(c.ders_in_span(s, black_box(t)));
        });
        let d2 = time(&|i| {
            let (s, t) = pick(i);
            black_box(c.deriv2_in_span(s, black_box(t)));
        });
        println!(
            "degree {p} ({interior} interior): eval_in_span {eval:.0} ns, deriv_in_span {d1:.0} ns, ders_in_span {ders:.0} ns, deriv2_in_span {d2:.0} ns"
        );
    }
}
