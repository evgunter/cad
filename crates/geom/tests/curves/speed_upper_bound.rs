//! **The rational speed bound** (`NurbsCurve3::speed_upper_bound`, the
//! pair-term bound of `geom_core::spline::rational_span_speed_sup`)
//! dominates the true speed, never reads above the homogeneous recipe
//! it replaced, and is the true sup where the pair terms attain it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom/src/curves/", "crates/geom-core/src/spline/",];

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real};
use test_utils::fuzz;

fn curve(knots: Vec<f64>, p: usize, pts: &[(f64, f64)], w: Vec<f64>) -> NurbsCurve3<f64> {
    NurbsCurve3::new(
        KnotVector::clamped(knots, p).unwrap(),
        pts.iter().map(|&(x, y)| Point3::new(x, y, 0.0)).collect(),
        w,
    )
    .unwrap()
}

/// The least certified LOWER end of `‖C′‖`'s largest sample: an
/// enclosure of `‖C′(t)‖` at `n + 1` parameters, the greatest lower
/// end — at most the true sup, so a bound below it is unsound with
/// certainty.
fn sampled_sup_floor(c: &NurbsCurve3<f64>, n: usize) -> f64 {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    let (lo, hi) = c.domain();
    #[allow(clippy::cast_precision_loss)]
    (0..=n)
        .map(|i| {
            let t = lo + (hi - lo) * i as f64 / n as f64;
            ci.deriv(Interval::from_f64(t)).norm().lo()
        })
        .fold(0.0, f64::max)
}

/// The homogeneous recipe the pair-term bound replaced, about the first
/// control point `c`: `(max ‖A′ᵢ‖ + d·max |w′ᵢ|)/min w`, `A = w·(P − c)`,
/// `d` the net's reach from `c`, over the coefficients of nonempty
/// spans.
fn recipe_a(c: &NurbsCurve3<f64>) -> f64 {
    let (ctl, w, k, p) = (c.control(), c.weights(), c.knots().knots(), c.degree());
    let o = ctl[0];
    let reach = ctl.iter().map(|q| (*q - o).norm()).fold(0.0, f64::max);
    let least = w.iter().copied().fold(f64::INFINITY, f64::min);
    let (mut a, mut b) = (0.0f64, 0.0f64);
    for i in 0..ctl.len() - 1 {
        let du = k[i + p + 1] - k[i + 1];
        if du == 0.0 {
            continue;
        }
        #[allow(clippy::cast_precision_loss)]
        let s = p as f64 / du;
        a = a.max(((ctl[i + 1] - o) * w[i + 1] - (ctl[i] - o) * w[i]).norm() * s);
        b = b.max((w[i + 1] - w[i]).abs() * s);
    }
    (a + reach * b) / least
}

/// One row: the `Interval` door's upper end is at least the sampled
/// sup's certified floor, the `f64` door agrees with it to rounding,
/// and the bound is at most recipe A. Returns `bound / sampled`.
fn check(name: &str, c: &NurbsCurve3<f64>) -> f64 {
    let (m, mi) = (
        c.speed_upper_bound().get(),
        c.map_scalar(<Interval as Real>::from_f64)
            .speed_upper_bound()
            .get(),
    );
    let floor = sampled_sup_floor(c, 4000);
    let a = recipe_a(c);
    assert!(
        mi.hi() >= floor && mi.lo() <= m && m <= mi.hi(),
        "{name}: bound {m} / [{}, {}] under the sampled sup {floor}",
        mi.lo(),
        mi.hi()
    );
    assert!(
        m <= a * (1.0 + 1e-12),
        "{name}: the pair-term bound {m} above recipe A's {a}"
    );
    m / floor
}

/// The named carriers: a quarter arc, a full circle, the weight-40
/// parabola, and an arc that the weights carry rather than the net.
#[test]
fn the_bound_covers_the_named_carriers_and_never_reads_above_recipe_a() {
    let h = 0.5f64.sqrt();
    check(
        "quarter arc",
        &curve(
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            2,
            &[(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            vec![1.0, h, 1.0],
        ),
    );
    check(
        "full circle",
        &curve(
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
    );
    check(
        "strongly rational arc",
        &curve(
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            2,
            &[(0.0, 0.0), (0.0, 0.1), (-0.6, 1.7)],
            vec![1.0, 4.0, 0.57],
        ),
    );
}

/// **The pin: the bound is the true sup on the weight-40 parabola.** Its
/// speed peaks at the ends, `C′(0) = 2·40·(P₁ − P₀)`, and the pair term
/// `i = 0, j = 0` is exactly that vector: `80√5`, which the bound reads
/// to rounding and the sampled sup meets.
#[test]
fn the_bound_is_the_true_sup_on_the_weight_forty_parabola() {
    let c = curve(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        &[(-1.0, 0.0), (0.0, 2.0), (1.0, 0.0)],
        vec![1.0, 40.0, 1.0],
    );
    let exact = 80.0 * 5.0f64.sqrt();
    let m = c.speed_upper_bound().get();
    assert!(
        (m - exact).abs() <= 1e-12 * exact,
        "bound {m}, exact sup {exact}"
    );
    let ratio = check("weight-40 parabola", &c);
    assert!(ratio <= 1.0 + 1e-9, "bound / sampled sup = {ratio}");
}

/// **Soundness fuzz** (counterexample search): random rational nets,
/// degree 1–4, weights over three decades, random interior knots
/// (some repeated): every row of [`check`].
#[test]
fn the_bound_covers_random_rational_nets() {
    let mut rng = fuzz::start("speed_upper_bound::random");
    let cases = fuzz::scaled(40);
    for case in 0..cases {
        let p = 1 + case % 4;
        let n = p + 1 + rng.below(6);
        let mut knots = vec![0.0; p + 1];
        let mut u = 0.0;
        while knots.len() < n {
            u += rng.range(0.1, 1.0);
            for _ in 0..(1 + rng.below(p)).min(n - knots.len()) {
                knots.push(u);
            }
        }
        u += rng.range(0.1, 1.0);
        knots.extend(std::iter::repeat_n(u, p + 1));
        let pts: Vec<(f64, f64)> = (0..n)
            .map(|_| (rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)))
            .collect();
        let w: Vec<f64> = (0..n).map(|_| 10f64.powf(rng.range(-1.5, 1.5))).collect();
        let Ok(kv) = KnotVector::clamped(knots, p) else {
            continue;
        };
        let c = NurbsCurve3::new(
            kv,
            pts.iter().map(|&(x, y)| Point3::new(x, y, 0.0)).collect(),
            w,
        )
        .unwrap();
        check(
            &format!("case {case} (deg {p}, {n} pts) — {}", fuzz::replay()),
            &c,
        );
    }
}
