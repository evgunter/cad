//! CERT-N1 R1 reviewer probe — C24's ns table, re-taken.
//!
//! The retired `deriv_in_span` WAS `ders_in_span(..).1`, so at the head
//! both spellings are still callable and the comparison needs one
//! release build: `deriv_in_span` (order-1) against `ders_in_span().1`
//! (the retired order-2 pass), plus `eval_in_span` as the floor.
//! Runs only under `CAD_R1_BENCH=1`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::KnotVector;
use std::hint::black_box;
use std::time::Instant;

fn curve(p: usize, interior: usize) -> NurbsCurve3<f64> {
    let mut k = vec![0.0; p + 1];
    for i in 1..=interior {
        k.push(i as f64 / (interior + 1) as f64);
    }
    k.extend(std::iter::repeat_n(1.0, p + 1));
    let kv = KnotVector::clamped(k, p).unwrap();
    let n = kv.control_count();
    let control: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            let t = i as f64;
            Point3::new(t * 0.31, (t * 1.1).sin(), (t * 0.7).cos() * 2.0)
        })
        .collect();
    let weights: Vec<f64> = (0..n).map(|i| 1.0 + 0.25 * (i % 5) as f64).collect();
    NurbsCurve3::new(kv, control, weights).unwrap()
}

#[test]
fn n1r1_c24_meter() {
    if std::env::var("CAD_R1_BENCH").is_err() {
        return;
    }
    const REPS: usize = 200_000;
    println!("degree/interior  eval_in_span  deriv_in_span(order1)  ders_in_span().1(retired)");
    for (p, interior) in [(2usize, 3usize), (3, 8), (5, 8), (7, 8)] {
        let c = curve(p, interior);
        let span = c.knots().span_at(0.37);
        // warm
        for _ in 0..10_000 {
            black_box(c.deriv_in_span(span, black_box(0.37)));
            black_box(c.ders_in_span(span, black_box(0.37)));
            black_box(c.eval_in_span(span, black_box(0.37)));
        }
        let t0 = Instant::now();
        for i in 0..REPS {
            black_box(c.eval_in_span(span, black_box(0.37 + i as f64 * 1e-12)));
        }
        let ev = t0.elapsed().as_nanos() as f64 / REPS as f64;
        let t1 = Instant::now();
        for i in 0..REPS {
            black_box(c.deriv_in_span(span, black_box(0.37 + i as f64 * 1e-12)));
        }
        let d1 = t1.elapsed().as_nanos() as f64 / REPS as f64;
        let t2 = Instant::now();
        for i in 0..REPS {
            black_box(c.ders_in_span(span, black_box(0.37 + i as f64 * 1e-12)).1);
        }
        let d2 = t2.elapsed().as_nanos() as f64 / REPS as f64;
        println!("p={p}/{interior}  {ev:.0} ns   {d1:.0} ns   {d2:.0} ns");
    }
}
