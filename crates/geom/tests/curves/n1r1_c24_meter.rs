//! CERT-N1 R1 reviewer probe — C24's ns table, re-taken.
//!
//! The retired `deriv_in_span` WAS `ders_in_span(..).1`, so at the head
//! both spellings are still callable and the comparison needs one
//! release build: `deriv_in_span` (order-1) against `ders_in_span().1`
//! (the retired order-2 pass), plus `eval_in_span` as the floor.
//! Runs only under `CAD_R1_BENCH=1`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::curves::meter_fixture::{DEGREES, curve};
use std::hint::black_box;
use std::time::Instant;

#[test]
fn n1r1_c24_meter() {
    if std::env::var("CAD_R1_BENCH").is_err() {
        return;
    }
    const REPS: usize = 200_000;
    println!("degree/interior  eval_in_span  deriv_in_span(order1)  ders_in_span().1(retired)");
    for (p, interior) in DEGREES {
        let c = curve(p, interior);
        let span = c.span_at(0.37);
        // warm
        for _ in 0..10_000 {
            black_box(span.deriv_in_span(black_box(0.37)));
            black_box(span.ders_in_span(black_box(0.37)));
            black_box(span.eval_in_span(black_box(0.37)));
        }
        let t0 = Instant::now();
        for i in 0..REPS {
            black_box(span.eval_in_span(black_box(0.37 + i as f64 * 1e-12)));
        }
        let ev = t0.elapsed().as_nanos() as f64 / REPS as f64;
        let t1 = Instant::now();
        for i in 0..REPS {
            black_box(span.deriv_in_span(black_box(0.37 + i as f64 * 1e-12)));
        }
        let d1 = t1.elapsed().as_nanos() as f64 / REPS as f64;
        let t2 = Instant::now();
        for i in 0..REPS {
            black_box(span.ders_in_span(black_box(0.37 + i as f64 * 1e-12)).1);
        }
        let d2 = t2.elapsed().as_nanos() as f64 / REPS as f64;
        println!("p={p}/{interior}  {ev:.0} ns   {d1:.0} ns   {d2:.0} ns");
    }
}
