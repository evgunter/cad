//! **The rational second-derivative bound** (`NurbsCurve3::accel_upper_bound`,
//! the second-order quotient bound of `geom_core::spline::accel_pieces`)
//! dominates the sampled `‖C″‖`, sits at the true sup where the
//! statement is tight, and reads only the span's own points.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom/src/curves/", "crates/geom-core/src/spline/",];

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real};

fn curve(knots: Vec<f64>, p: usize, pts: &[(f64, f64)], w: Vec<f64>) -> NurbsCurve3<f64> {
    NurbsCurve3::new(
        KnotVector::clamped(knots, p).unwrap(),
        pts.iter().map(|&(x, y)| Point3::new(x, y, 0.0)).collect(),
        w,
    )
    .unwrap()
}

/// The `Interval` door's upper end: the certified bound.
fn bound(c: &NurbsCurve3<f64>) -> f64 {
    let b = c
        .map_scalar(<Interval as Real>::from_f64)
        .accel_upper_bound();
    assert!(b.is_certified(), "the bound refused: {b:?}");
    b.hi()
}

/// The greatest certified LOWER end of `‖C″(t)‖` over `n + 1` parameters:
/// at most the true sup, so a bound below it is unsound with certainty.
fn sampled_floor(c: &NurbsCurve3<f64>, n: usize) -> f64 {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    let (lo, hi) = c.domain();
    #[allow(clippy::cast_precision_loss)]
    (0..=n)
        .map(|i| {
            let t = (lo + (hi - lo) * i as f64 / n as f64).clamp(lo, hi);
            ci.deriv2(Interval::from_f64(t)).norm().lo()
        })
        .fold(0.0, f64::max)
}

/// **The bound covers the named carriers.** A quarter arc, the
/// weight-40 parabola, a strongly rational arc and a rational cubic with
/// interior knots: on each, the certified bound is at least the sampled
/// sup of `‖C″‖`. The `f64` door agrees with the `Interval` one to
/// rounding. The nine-point full circle's double knots are creases at
/// degree 2, across which its structure licenses no `C′`: it reads no
/// finite bound at either scalar.
#[test]
fn the_bound_covers_the_named_carriers() {
    let h = 0.5f64.sqrt();
    let carriers = [
        (
            "quarter arc",
            curve(
                vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
                2,
                &[(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
                vec![1.0, h, 1.0],
            ),
        ),
        (
            "weight-40 parabola",
            curve(
                vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
                2,
                &[(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)],
                vec![1.0, 40.0, 1.0],
            ),
        ),
        (
            "strongly rational arc",
            curve(
                vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
                2,
                &[(0.0, 0.0), (0.0, 0.1), (-0.6, 1.7)],
                vec![1.0, 4.0, 0.57],
            ),
        ),
        (
            "rational cubic",
            curve(
                vec![0.0, 0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0, 1.0],
                3,
                &[(0.0, 0.0), (0.5, 1.0), (1.5, 1.2), (2.0, -0.4), (3.0, 0.3)],
                vec![1.0, 2.5, 0.6, 1.8, 1.0],
            ),
        ),
    ];
    for (name, c) in carriers {
        let (b, floor) = (bound(&c), sampled_floor(&c, 4000));
        assert!(
            b >= floor,
            "{name}: bound {b} under the sampled sup {floor}"
        );
        let f = c.accel_upper_bound();
        assert!(
            (f - b).abs() <= 1e-9 * b,
            "{name}: the f64 door {f} and the Interval door {b} disagree"
        );
    }
    let circle = curve(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
        &[
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
            (-1.0, 1.0),
            (-1.0, 0.0),
            (-1.0, -1.0),
            (0.0, -1.0),
            (1.0, -1.0),
            (1.0, 0.0),
        ],
        vec![1.0, h, 1.0, h, 1.0, h, 1.0, h, 1.0],
    );
    assert!(
        circle.accel_upper_bound().is_infinite(),
        "the full circle's creases bound no |C″|"
    );
    assert!(
        !circle
            .map_scalar(<Interval as Real>::from_f64)
            .accel_upper_bound()
            .hi()
            .is_finite(),
        "the full circle's creases bound no |C″| at Interval"
    );
}

/// **On one rational segment the bound is the true sup.** With weights
/// `(1, a)` on the unit segment, `‖C″‖ = 2a·|a − 1|/W³` with
/// `W = 1 + (a − 1)t`, largest at the end of the least weight. A
/// Bernstein row interpolates its end values, so the end piece's end
/// ratio IS that value and the max ratio reads it. The row goes red
/// when the bound reads under the sup: a dropped numerator term, a
/// numerator over the wrong power of `W`, a ratio over another index's
/// `b_k`, or a numerator left at its own degree.
#[test]
fn a_single_rational_segment_reads_its_true_sup() {
    for a in [0.25, 0.7, 3.0, 40.0] {
        let c = curve(
            vec![0.0, 0.0, 1.0, 1.0],
            1,
            &[(0.0, 0.0), (1.0, 0.0)],
            vec![1.0, a],
        );
        let w_min = a.min(1.0);
        let truth = 2.0 * a * (a - 1.0).abs() / w_min.powi(3);
        let b = bound(&c);
        assert!(
            b >= truth && b <= truth * (1.0 + 1e-12),
            "a = {a}: bound {b} against the true sup {truth}"
        );
    }
}

/// **At unit weights the bound is the second derivative's sup,
/// exactly.** A cubic Bézier's `C″` is linear between `6·Δ²P₀` and
/// `6·Δ²P₁`, so its sup is the larger of those norms, and the end
/// pieces' end ratios are exactly those values. A wrong second-difference
/// scale reads off it, and a dropped `A″·W²` term reads zero.
#[test]
fn a_polynomial_cubic_reads_its_second_differences() {
    let pts = [(0.0, 0.0), (1.0, 2.0), (3.0, 1.5), (3.5, -1.0)];
    let c = curve(
        vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
        3,
        &pts,
        vec![1.0; 4],
    );
    let d2 = |j: usize| {
        let x = pts[j + 2].0 - 2.0 * pts[j + 1].0 + pts[j].0;
        let y = pts[j + 2].1 - 2.0 * pts[j + 1].1 + pts[j].1;
        6.0 * x.hypot(y)
    };
    let truth = d2(0).max(d2(1));
    let b = bound(&c);
    assert!(
        b >= truth && b <= truth * (1.0 + 1e-12),
        "bound {b} against the true sup {truth}"
    );
}

/// **A span's bound reads only the span's own points.** On a quadratic
/// with three nonempty spans the last control point and weight are
/// active on the last span alone; moving them a hundredfold leaves the
/// first span's pieces bit-identical, every numerator and weight-power
/// coefficient, while the domain's bound grows. The data is not dyadic,
/// so the coefficients carry rounding: `C″` is the same in every frame
/// in ℝ, and a frame read from a foreign point shows in the bits.
#[test]
fn a_far_point_active_on_the_last_span_only_does_not_reach_the_first() {
    let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
    let near = curve(
        knots.clone(),
        2,
        &[(0.1, 0.3), (1.1, 0.7), (2.3, 0.1), (3.1, 1.3), (4.0, 0.2)],
        vec![1.1, 2.3, 0.9, 1.7, 1.0],
    );
    let far = curve(
        knots,
        2,
        &[
            (0.1, 0.3),
            (1.1, 0.7),
            (2.3, 0.1),
            (3.1, 1.3),
            (400.3, 90.7),
        ],
        vec![1.1, 2.3, 0.9, 1.7, 100.0],
    );
    // The pieces come in ascending span order, the same count per span.
    let first = |c: &NurbsCurve3<f64>| -> Vec<u64> {
        let pts: Vec<[f64; 3]> = c.control().iter().map(|p| [p.x, p.y, p.z]).collect();
        let pieces = geom_core::spline::accel_pieces(
            &geom_core::spline::Knots::from(c.knots_f64_interim().clone()),
            &pts,
            c.weights(),
            32,
        )
        .unwrap();
        let per_span = pieces.len() / 3;
        pieces[..per_span]
            .iter()
            .flat_map(|q| {
                let numer = q.numer().iter().flatten().copied();
                numer.chain(q.denom().iter().copied()).map(f64::to_bits)
            })
            .collect()
    };
    assert_eq!(
        first(&near),
        first(&far),
        "the first span read a point active on the last span only"
    );
    assert!(
        bound(&far) > 10.0 * bound(&near),
        "the far point is not far"
    );
}

/// **The pieces are what makes the bound tight.** A quintic with
/// interior knots `0.31` and `0.62` and weights spread across `10^±1.5`
/// reads its sup within 5% of the sampled one at the curve's 32 pieces
/// per span (measured `1.004`). A schedule cut to one piece per span
/// keeps every soundness row green while the bound loosens to `50.7`×
/// the sample, so this row is the one that sees the schedule.
#[test]
fn the_pieces_read_a_spread_quintic_within_five_percent() {
    let pts = [
        (0.0, 0.0),
        (0.5, 1.0),
        (1.5, 1.2),
        (2.0, 0.0),
        (2.5, -1.0),
        (3.5, -1.2),
        (4.0, 0.0),
        (4.5, 1.0),
    ];
    let w = vec![1.0, 0.0316, 0.0316, 31.6, 31.6, 0.0316, 0.0316, 1.0];
    let c = curve(
        vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.31, 0.62, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ],
        5,
        &pts,
        w,
    );
    let (b, truth) = (bound(&c), sampled_floor(&c, 4000));
    assert!(b >= truth, "bound {b} under the sampled {truth}");
    assert!(
        b <= 1.05 * truth,
        "bound {b} is {}× the sampled sup {truth}",
        b / truth
    );
}

/// `Σ_k C(n,k) s^k (1 − s)^(n−k) · x_k`, the Bernstein value of `x` at `s`.
fn bernstein(x: &[f64], s: f64) -> f64 {
    let n = x.len() - 1;
    let mut row = x.to_vec();
    for level in 0..n {
        for k in 0..n - level {
            row[k] = row[k] * (1.0 - s) + row[k + 1] * s;
        }
    }
    row[0]
}

/// **The pieces' quotient is `C″` itself, pointwise.** On a rational
/// cubic with an interior knot, each piece's numerator and weight power
/// evaluated as Bernstein sums at the piece's local parameter, `M/W³`,
/// matches `deriv2` at `t ∈ {0, 0.13, 0.5, 0.77, 0.9999}` to `1e-8`
/// of its scale. A dropped or mis-signed numerator term is a wrong
/// function, which no soundness row against a sup need see.
#[test]
fn the_pieces_quotient_is_the_second_derivative_pointwise() {
    let c = curve(
        vec![0.0, 0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0, 1.0],
        3,
        &[(0.0, 0.0), (0.7, 1.2), (1.6, -0.4), (2.4, 0.9), (3.1, 0.1)],
        vec![1.0, 2.5, 0.6, 1.8, 1.2],
    );
    let pts: Vec<[f64; 3]> = c.control().iter().map(|p| [p.x, p.y, p.z]).collect();
    let pieces = geom_core::spline::accel_pieces(
        &geom_core::spline::Knots::from(c.knots_f64_interim().clone()),
        &pts,
        c.weights(),
        32,
    )
    .unwrap();
    let per_span = pieces.len() / 2;
    for t in [0.0, 0.13, 0.5, 0.77, 0.9999] {
        let (span, lo, hi) = if t < 0.4 {
            (0, 0.0, 0.4)
        } else {
            (1, 0.4, 1.0)
        };
        #[allow(clippy::cast_precision_loss)]
        let x = (t - lo) / (hi - lo) * per_span as f64;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let k = (x.floor() as usize).min(per_span - 1);
        #[allow(clippy::cast_precision_loss)]
        let s = x - k as f64;
        let q = &pieces[span * per_span + k];
        let w3 = bernstein(q.denom(), s);
        let truth = c.deriv2(t);
        for (axis, want) in [truth.x, truth.y, truth.z].into_iter().enumerate() {
            let m: Vec<f64> = q.numer().iter().map(|a| a[axis]).collect();
            let got = bernstein(&m, s) / w3;
            assert!(
                (got - want).abs() <= 1e-8 * (1.0 + want.abs()),
                "t = {t}, axis {axis}: the pieces read {got}, deriv2 {want}"
            );
        }
    }
}

/// **A speed floor that certifies nothing bounds no curvature.** A
/// quadratic whose hodograph passes within `4e-4` of the origin, near
/// `t ≈ 0.6`: its `f64` speed floor reads `−0.015`, below zero, and the
/// curvature there runs past `1e8`. `carrier_speed::curvature_hi` reads
/// a floor not above zero as zero, so the bound is not finite; squaring
/// the negative floor instead would read `accel / 0.015² ≈ 1e5`, three
/// orders below the curvature sampled on the curve.
#[test]
fn a_negative_speed_floor_bounds_no_curvature() {
    let n = curve(
        vec![
            0.0,
            0.0,
            0.0,
            0.333_333_333_333_333_3,
            0.666_666_666_666_666_6,
            1.0,
            1.0,
            1.0,
        ],
        2,
        &[
            (-0.319_028_147_829_698_35, -0.603_727_187_226_300_1),
            (-0.815_157_349_743_728_1, -0.900_129_476_941_211_1),
            (0.470_530_898_125_187_9, -0.132_348_527_774_435_4),
            (0.805_494_952_913_653_3, -0.223_410_013_943_190_7),
            (0.777_023_731_571_268, -0.315_957_763_176_544_2),
        ],
        vec![1.0; 5],
    );
    let c = geom::Curve3::Nurbs(std::sync::Arc::new(n.clone()));
    assert!(
        geom::carrier_speed::speed_range(&c).0.get() < 0.0,
        "the speed floor certifies nothing"
    );
    let sampled = (0..=20_000)
        .map(|i| {
            let t = f64::from(i) / 20_000.0;
            let (d1, d2) = (n.deriv(t), n.deriv2(t));
            d1.cross(d2).norm() / d1.norm().powi(3)
        })
        .fold(0.0, f64::max);
    assert!(
        sampled > 1e8,
        "the curve turns sharply near its cusp: {sampled:e}"
    );
    let bound = geom::carrier_speed::curvature_hi(&c);
    assert!(
        !bound.is_finite(),
        "a negative floor bounds no curvature: read {bound:e}, sampled {sampled:e}"
    );
}
