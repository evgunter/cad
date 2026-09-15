//! The whole-curve order-1 jet meter: `eval` + `deriv` at one `t`
//! against `ders1` at the same `t`, on the payload door and on the
//! `Curve3::Nurbs` arm, at degrees 2/3/5/7. The pair runs two span
//! selections and two basis passes for what one order-1 pass answers;
//! this prints how much that costs. Reporting, never gating: runs only
//! under `CAD_R1_BENCH=1`, and wants a release build to mean anything.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::hint::black_box;
use std::sync::Arc;
use std::time::Instant;

use geom::{Curve3, NurbsCurve3};
use geom_core::Point3;
use geom_core::spline::KnotVector;

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

/// Nanoseconds per call of `f` over `REPS` parameters that walk the
/// domain, so the located span changes and nothing hoists.
fn ns_per<R>(reps: usize, f: impl Fn(f64) -> R) -> f64 {
    for i in 0..2_000 {
        black_box(f(black_box(0.05 + (i % 900) as f64 * 1e-3)));
    }
    let t0 = Instant::now();
    for i in 0..reps {
        black_box(f(black_box(0.05 + (i % 900) as f64 * 1e-3)));
    }
    t0.elapsed().as_nanos() as f64 / reps as f64
}

#[test]
fn ders1_meter() {
    if std::env::var("CAD_R1_BENCH").is_err() {
        return;
    }
    const REPS: usize = 200_000;
    println!(
        "degree/interior  NurbsCurve3 eval+deriv  ders1  |  Curve3::Nurbs eval+deriv  ders1"
    );
    for (p, interior) in [(2usize, 3usize), (3, 8), (5, 8), (7, 8)] {
        let n = curve(p, interior);
        let c = Curve3::Nurbs(Arc::new(n.clone()));
        let n_pair = ns_per(REPS, |t| (n.eval(t), n.deriv(t)));
        let n_jet = ns_per(REPS, |t| n.ders1(t));
        let c_pair = ns_per(REPS, |t| (c.eval(t), c.deriv(t)));
        let c_jet = ns_per(REPS, |t| c.ders1(t));
        println!(
            "p={p}/{interior}  {n_pair:.0} ns  {n_jet:.0} ns  |  {c_pair:.0} ns  {c_jet:.0} ns"
        );
    }
}
