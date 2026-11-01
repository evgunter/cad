//! Degree elevation agrees with its source to rounding at every knot
//! spacing.
//!
//! Elevation recomposes by exact removals, each free to leave any one
//! of its equations unused; which one decides the rounding
//! (`geom_core::spline::algebra::elevate_plan`). Piegl–Tiller's
//! midpoint split divides by about `g/span` beside a breakpoint whose
//! gap `g` is small next to its neighbours, and each pass hands the next
//! its error, so it amplifies rounding by about `(span/g)^k`, `k` growing
//! with degree: about 1 from degree 3 to 5, 2 at 6–7, 3 at 8–9. Close
//! pairs, graded spacing and, at degree 9, uniform spacing all reach it.
//! The rows pin agreement where the midpoint is far from it, and pin
//! that the split keeps the midpoint where it is not.
//!
//! Every row reads the sup of `|C(t) − E(t)|` over nine points per
//! distinct span (the close pair's own span included) against a
//! fraction of the largest control coordinate: `1e-12` for a close pair
//! (the worst reads about `4e-14`), `1e-11` for gently graded knots at
//! degree 10 (about `8e-13`), `1e-13` for steeply graded knots (about
//! `2e-14`).

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
/// `1e-9`, and against ten uniform knots. The midpoint split reads past
/// the cap from degree 3, at a pair `1e-5` apart.
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

/// Degree 8 with a pair `1e-5` apart, on pinned data: the midpoint split
/// moves this curve by 0.19 of its largest coordinate.
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
/// step starts at degree 6, where the midpoint split amplifies by about
/// `(span/g)²` and moves this curve by 0.076 of its largest coordinate.
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

/// Degrees 8 and 9 with a pair `1e-6` apart and drawn weights. Every
/// curve elevates, and agrees. The midpoint split's amplified rounding
/// writes a weight out of the positive regime on about three curves in
/// four (`WeightCollapse`), and moves the rest by hundreds of their
/// coordinates.
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

/// Fifty uniform knots, a catastrophe guard. The midpoint split is at
/// rounding there at degrees 6–8 and 10 (worst of 40 curves at degree
/// 10: about `1.1e-13` of scale) but not at 9 (about `1e-8`). A split
/// forward while `α ≥ ½`, or chosen per pass without the error its
/// inputs carry, refuses every degree-8 curve with `WeightCollapse`.
#[test]
fn uniform_knots_keep_the_midpoint_splits_rounding() {
    const UNIFORM: f64 = 5e-13;
    let mut rng = fuzz::start("elevation_conditioning::uniform");
    let mids: Vec<f64> = (1..=50).map(|i| f64::from(i) / 51.0).collect();
    for p in [6, 8, 9, 10] {
        for _ in 0..fuzz::scaled(4) {
            let c = drawn(&mut rng, p, &mids);
            let d = disagreement(&c, &c.elevate_degree(1).unwrap());
            assert!(d <= UNIFORM, "p={p}: {d:e} — {}", fuzz::replay());
        }
    }
}

/// Geometric ×1.15 knot spacing at degree 10, no close pair: each gap
/// is 15% wider than the last, and the midpoint split reads about 0.15
/// on ±10 coordinates with 40 knots.
#[test]
fn graded_knots_at_degree_ten_elevate_to_rounding() {
    const GRADED: f64 = 1e-11;
    let mut rng = fuzz::start("elevation_conditioning::graded");
    let total: f64 = (0..=40).map(|i| 1.15f64.powi(i)).sum();
    let mids: Vec<f64> = (0..40)
        .scan(0.0, |at, i| {
            *at += 1.15f64.powi(i) / total;
            Some(*at)
        })
        .collect();
    for _ in 0..fuzz::scaled(4) {
        let c = drawn(&mut rng, 10, &mids);
        let d = disagreement(&c, &c.elevate_degree(1).unwrap());
        assert!(d <= GRADED, "{d:e} — {}", fuzz::replay());
    }
}

/// Geometric ×2 knot spacing at degree 10: each gap twice the last, so
/// every pass has a split far better than the midpoint, which refuses
/// these curves. The margin must leave the midpoint here. Over 30
/// drawn curves, relative to scale: worst about `2e-14` at the margin of
/// 2; median `3e-13` (worst `9e-13`) at 4; median `2e-12` at 8. The cap
/// pins the margin from above.
#[test]
fn steeply_graded_knots_leave_the_midpoint() {
    const STEEP: f64 = 1e-13;
    let mut rng = fuzz::start("elevation_conditioning::steep");
    let total: f64 = (0..=40).map(|i| 2f64.powi(i)).sum();
    let mids: Vec<f64> = (0..40)
        .scan(0.0, |at, i| {
            *at += 2f64.powi(i) / total;
            Some(*at)
        })
        .collect();
    for _ in 0..fuzz::scaled(8) {
        let c = drawn(&mut rng, 10, &mids);
        let d = disagreement(&c, &c.elevate_degree(1).unwrap());
        assert!(d <= STEEP, "{d:e} — {}", fuzz::replay());
    }
}

/// Fifty interior knots of three spacings, as the envelope past degree
/// 10 draws them: uniform (jittered by `1e-6` of a span), graded
/// `t^1.5`, and jittered by `±30%` of a span.
fn past_ten_knots(rng: &mut fuzz::Rng, spacing: &str) -> Vec<f64> {
    let h = 1.0 / 51.0;
    let mut v: Vec<f64> = (1..=50)
        .map(|i| {
            let i = f64::from(i);
            match spacing {
                "uniform" => (i + rng.range(-1e-6, 1e-6)) * h,
                "graded" => (i * h).powf(1.5),
                _ => (i + rng.range(-0.3, 0.3)) * h,
            }
        })
        .collect();
    v.sort_by(f64::total_cmp);
    v.dedup();
    v
}

/// The envelope at degree 11, past the rows above: fifty uniform,
/// graded and jittered knots. The midpoint split refuses every one of
/// these curves (`WeightCollapse`). Measured worst of 90 curves per
/// spacing, relative to scale: `4.4e-12`, `2.4e-12`, `8.1e-12`; the cap
/// is about 3× the worst. A margin of 1 reads alike and no row tells it
/// apart here (`uniform_knots_keep_the_midpoint_split_at_every_pass`
/// does). No bound is surfaced
/// (`work/nurbs/elevation-past-degree-ten-agrees-to-1e-11-with-no-bound`).
#[test]
fn degree_eleven_elevates_within_its_measured_envelope() {
    const ELEVEN: f64 = 2.5e-11;
    let mut rng = fuzz::start("elevation_conditioning::eleven");
    for spacing in ["uniform", "graded", "jitter"] {
        for _ in 0..fuzz::scaled(1) {
            let mids = past_ten_knots(&mut rng, spacing);
            let c = drawn(&mut rng, 11, &mids);
            let d = disagreement(&c, &c.elevate_degree(1).unwrap());
            assert!(d <= ELEVEN, "{spacing}: {d:e} — {}", fuzz::replay());
        }
    }
}

/// The envelope at degree 12, as `degree_eleven_…`. The midpoint split
/// reads `1.7e-4` to 12 of scale here, or refuses. Measured worst of 90
/// curves per spacing: `1.8e-11`, `1.5e-11`, `3.6e-11`; the cap is about
/// 3× the worst.
#[test]
fn degree_twelve_elevates_within_its_measured_envelope() {
    const TWELVE: f64 = 1.1e-10;
    let mut rng = fuzz::start("elevation_conditioning::twelve");
    for spacing in ["uniform", "graded", "jitter"] {
        for _ in 0..fuzz::scaled(1) {
            let mids = past_ten_knots(&mut rng, spacing);
            let c = drawn(&mut rng, 12, &mids);
            let d = disagreement(&c, &c.elevate_degree(1).unwrap());
            assert!(d <= TWELVE, "{spacing}: {d:e} — {}", fuzz::replay());
        }
    }
}

/// The envelope at degrees 16, 20 and 25 on clusters of 2 and 4 knots
/// `1e-9` apart. The midpoint split refuses every one of these curves.
/// Measured worst of 90 curves per shape: about `1e-13` at 16, `4e-13`
/// at 20, `5.9e-12` at 25; the cap is about 3× the worst. A margin of 8
/// reads 2–4e-11 at degree 25 and turns this row red; the degree-11 and
/// degree-12 rows do not see it.
#[test]
fn high_degree_clusters_elevate_within_their_measured_envelope() {
    const CLUSTERS: f64 = 2e-11;
    let mut rng = fuzz::start("elevation_conditioning::clusters");
    for p in [16, 20, 25] {
        for k in [2, 4] {
            for _ in 0..fuzz::scaled(4) {
                let a = rng.range(0.2, 0.6);
                let mids: Vec<f64> = (0..k).map(|i| a + 1e-9 * f64::from(i)).collect();
                let c = drawn(&mut rng, p, &mids);
                let d = disagreement(&c, &c.elevate_degree(1).unwrap());
                assert!(
                    d <= CLUSTERS,
                    "p={p}, {k} knots: {d:e} — {}",
                    fuzz::replay()
                );
            }
        }
    }
}
