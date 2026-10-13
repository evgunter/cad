//! **The rational second-derivative bound** (`NurbsCurve3::accel_upper_bound`,
//! the second-order pair-term bound of `geom_core::spline::rational_accel_sup`)
//! dominates the sampled `‖C″‖`, is the true sup where every term of its
//! derivation is attained, and reads only the span's own points.

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

/// **The bound covers the named carriers.** A quarter arc, a full
/// circle, the weight-40 parabola, a strongly rational arc and a
/// rational cubic with interior knots: on each, the certified bound is
/// at least the sampled sup of `‖C″‖`. The `f64` door agrees with the
/// `Interval` one to rounding.
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
            "full circle",
            curve(
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
}

/// **On one rational segment the bound is the true sup.** At degree 1
/// there is no second difference, and `C″ = −2·W′·C′/W` exactly: with
/// weights `(1, a)` on the unit segment, `sup‖C″‖ = 2a·|a − 1|/w_min³`.
/// The bound reads `2·|a − 1|·S₁/w_min` with `S₁ = max(a, 1)/w_min`,
/// which is that number for both `a < 1` and `a > 1`. So halving the
/// `2·W′·C′` term, dropping it, or reading `S₁` without its own division
/// by the least weight reads under the sup.
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

/// **At unit weights the bound is the second-difference bound, exactly.**
/// A cubic Bézier's `C″` is linear between `6·Δ²P₀` and `6·Δ²P₁`, so its
/// sup is the larger of those norms, which is what the bound reads
/// (`k²ⱼ = 2`, `kᵢ = 3`). A wrong second-difference scale reads off it,
/// and dropping the second-difference term reads zero.
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
/// first span's bound bit-identical, while the domain's bound grows.
#[test]
fn a_far_point_active_on_the_last_span_only_does_not_reach_the_first() {
    let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
    let near = curve(
        knots.clone(),
        2,
        &[(0.0, 0.0), (1.0, 0.5), (2.0, 0.0), (3.0, 1.0), (4.0, 0.0)],
        vec![1.0, 2.0, 1.0, 1.5, 1.0],
    );
    let far = curve(
        knots,
        2,
        &[
            (0.0, 0.0),
            (1.0, 0.5),
            (2.0, 0.0),
            (3.0, 1.0),
            (400.0, 90.0),
        ],
        vec![1.0, 2.0, 1.0, 1.5, 100.0],
    );
    let first = |c: &NurbsCurve3<f64>| {
        let pts: Vec<[f64; 3]> = c.control().iter().map(|p| [p.x, p.y, p.z]).collect();
        let s = c.knots().first_span();
        geom_core::spline::rational_accel_sup(
            &geom_core::spline::Knots::from(c.knots().clone()),
            &pts,
            c.weights(),
            s..=s,
        )
    };
    assert_eq!(
        first(&near).to_bits(),
        first(&far).to_bits(),
        "the first span read a point active on the last span only"
    );
    assert!(
        bound(&far) > 10.0 * bound(&near),
        "the far point is not far"
    );
}
