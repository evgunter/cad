//! Degree elevation agrees with its source to rounding at every knot
//! spacing, close pairs included.
//!
//! Elevation recomposes by exact removals, each free to leave any one
//! of its equations unused; which one decides the rounding
//! (`geom_core::spline::algebra::elevate_plan`). Piegl–Tiller's
//! midpoint split divided by about `g/span` beside a breakpoint a gap
//! `g` from another, and each pass handed the next its error: one
//! elevation from degree 3–5 disagreed by about `1e-15·span/g` of the
//! coordinates, from degree 6–7 by about its square, from degree 8–9 by
//! about its cube — a pair `1e-5` apart at degree 8 moved the curve by
//! the size of its coordinates, and some curves refused with
//! `WeightCollapse` instead. Uniform knots were never affected.
//!
//! Every row reads the sup of `|C(t) − E(t)|` over nine points per
//! distinct span (the close pair's own span included) against
//! `1e-12` of the largest control coordinate. Measured after the fix,
//! the worst close-pair row reads about `4e-14` of it.

#![allow(clippy::unwrap_used, clippy::expect_used)]

test_utils::gated_to!["crates/geom/src/curves/", "crates/geom-core/src/spline/"];

use geom::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::KnotVector;
use test_utils::fuzz;

/// A clamped curve of degree `p` on `[0, 1]` with simple interior
/// knots `mids`, its control points and weights from `at(i)`.
fn curve(p: usize, mids: &[f64], at: impl Fn(usize) -> (Point3<f64>, f64)) -> NurbsCurve3<f64> {
    let mut ks = vec![0.0; p + 1];
    ks.extend_from_slice(mids);
    ks.extend(std::iter::repeat_n(1.0, p + 1));
    let kv = KnotVector::clamped(ks, p).unwrap();
    let (control, weights) = (0..kv.control_count()).map(at).unzip();
    NurbsCurve3::new(kv, control, weights).unwrap()
}

/// A drawn curve: coordinates in ±10, weights in `[0.2, 5]`.
fn drawn(rng: &mut fuzz::Rng, p: usize, mids: &[f64]) -> NurbsCurve3<f64> {
    let draws: Vec<(Point3<f64>, f64)> = (0..mids.len() + p + 1)
        .map(|_| {
            let c = Point3::new(
                rng.range(-10.0, 10.0),
                rng.range(-10.0, 10.0),
                rng.range(-10.0, 10.0),
            );
            (c, rng.range(0.2, 5.0))
        })
        .collect();
    curve(p, mids, |i| draws[i])
}

/// The disagreement of `e` with `c`, as a fraction of `c`'s largest
/// control coordinate.
fn disagreement(c: &NurbsCurve3<f64>, e: &NurbsCurve3<f64>) -> f64 {
    let mut ks = c.knots_f64_interim().knots().to_vec();
    ks.dedup();
    let sup = ks
        .windows(2)
        .flat_map(|w| (0..=8).map(move |i| w[0] + (w[1] - w[0]) * f64::from(i) / 8.0))
        .map(|t| (c.eval(t) - e.eval(t)).norm_inf())
        .fold(0.0, f64::max);
    let scale = c
        .control()
        .iter()
        .map(|q| q.x.abs().max(q.y.abs()).max(q.z.abs()))
        .fold(0.0, f64::max);
    sup / scale
}

const AGREE: f64 = 1e-12;

/// Every degree to 10 against a close pair `g` apart, `g` down to
/// `1e-9`, and against ten uniform knots. Red on the midpoint split
/// from degree 3, at a pair `1e-5` apart.
#[test]
fn one_elevation_agrees_at_every_degree_and_close_pair() {
    let mut rng = fuzz::start("elevation_conditioning::table");
    let uniform: Vec<f64> = (1..=10).map(|i| f64::from(i) / 11.0).collect();
    for p in 1..=10 {
        for g in [1e-3, 1e-5, 1e-7, 1e-9, f64::NAN] {
            for _ in 0..fuzz::scaled(2) {
                let mids = if g.is_nan() {
                    uniform.clone()
                } else {
                    let a = rng.range(0.2, 0.7);
                    vec![a, a + g]
                };
                let c = drawn(&mut rng, p, &mids);
                let d = disagreement(&c, &c.elevate_degree(1).unwrap());
                assert!(d <= AGREE, "p={p} g={g:e}: {d:e} — {}", fuzz::replay());
            }
        }
    }
}

/// Degree 8 with a pair `1e-5` apart: the midpoint split moved this
/// curve by 0.19 of its largest coordinate.
#[test]
fn a_degree_eight_pair_a_hundred_thousandth_apart_elevates_to_rounding() {
    let c = curve(8, &[0.4, 0.4 + 1e-5], |i| {
        let t = i as f64;
        (
            Point3::new(
                10.0 * (1.3 * t).sin(),
                10.0 * (0.7 * t + 1.0).cos(),
                10.0 * (2.1 * t).sin(),
            ),
            1.0 + 0.5 * (0.9 * t).sin(),
        )
    });
    let d = disagreement(&c, &c.elevate_degree(1).unwrap());
    assert!(d <= AGREE, "{d:e}");
}

/// Two elevations from degree 5 with a pair `1e-8` apart: the second
/// step starts at degree 6, where the midpoint split amplified by
/// about `(span/g)²`, and moved this curve by 0.076 of its largest
/// coordinate.
#[test]
fn elevating_twice_past_a_close_pair_does_not_compound() {
    let c = curve(5, &[0.55, 0.55 + 1e-8], |i| {
        let t = i as f64;
        (
            Point3::new(
                10.0 * (0.8 * t).cos(),
                10.0 * (1.9 * t).sin(),
                10.0 * (0.4 * t + 2.0).sin(),
            ),
            1.0 + 0.6 * (1.4 * t).cos(),
        )
    });
    let d = disagreement(&c, &c.elevate_degree(2).unwrap());
    assert!(d <= AGREE, "{d:e}");
}

/// Degrees 8 and 9 with a pair `1e-6` apart and drawn weights: the
/// midpoint split refused about three curves in four with
/// `WeightCollapse` (its amplified rounding wrote a weight out of the
/// positive regime), and moved the rest by hundreds of their
/// coordinates. Every one elevates, and agrees.
#[test]
fn high_degree_close_pairs_elevate_without_refusing() {
    let mut rng = fuzz::start("elevation_conditioning::refusals");
    for p in [8, 9] {
        for _ in 0..fuzz::scaled(4) {
            let a = rng.range(0.2, 0.7);
            let c = drawn(&mut rng, p, &[a, a + 1e-6]);
            let e = c.elevate_degree(1);
            assert!(
                e.is_ok(),
                "p={p}: {:?} — {}",
                e.as_ref().err(),
                fuzz::replay()
            );
            let e = e.unwrap();
            let d = disagreement(&c, &e);
            assert!(d <= AGREE, "p={p}: {d:e} — {}", fuzz::replay());
        }
    }
}

/// Fifty uniform knots, where the midpoint split was already at
/// rounding: the choice of split must not move them off it. The worst
/// of 40 drawn curves on main reads about `1.1e-13` of scale at degree
/// 10 and `2.4e-15` at degree 8. Red on a split forward while
/// `α ≥ ½`, and on one chosen per pass without the error its inputs
/// carry: both refused every degree-8 curve with `WeightCollapse`.
#[test]
fn uniform_knots_keep_the_midpoint_splits_rounding() {
    const UNIFORM: f64 = 5e-13;
    let mut rng = fuzz::start("elevation_conditioning::uniform");
    let mids: Vec<f64> = (1..=50).map(|i| f64::from(i) / 51.0).collect();
    for p in [6, 8, 10] {
        for _ in 0..fuzz::scaled(4) {
            let c = drawn(&mut rng, p, &mids);
            let d = disagreement(&c, &c.elevate_degree(1).unwrap());
            assert!(d <= UNIFORM, "p={p}: {d:e} — {}", fuzz::replay());
        }
    }
}
