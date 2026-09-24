//! The NURBS fixture the curve meters time: a clamped curve of degree
//! `p` with `interior` uniformly spaced interior knots, rational with
//! a five-cycle of weights, its control polygon a slow helix-ish
//! walk. One home, so the meters that compare a pair of doors against
//! one door time the same curve — a second copy of this fixture would
//! be a second curve two ns tables silently disagree on.

#![allow(dead_code, unreachable_pub, clippy::unwrap_used)]

use geom::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::KnotVector;

/// The degree/interior-knot pairs every curve meter reports at.
pub const DEGREES: [(usize, usize); 4] = [(2, 3), (3, 8), (5, 8), (7, 8)];

pub fn curve(p: usize, interior: usize) -> NurbsCurve3<f64> {
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
