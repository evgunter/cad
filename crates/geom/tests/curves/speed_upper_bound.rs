//! **The rational speed bound** (`NurbsCurve3::speed_upper_bound`, the
//! quotient bound of `geom_core::spline::derivative` on each span's
//! dyadic pieces) dominates the true speed, never reads above the
//! whole-net homogeneous recipe ([`recipe_a`]), and is the true sup
//! where a piece's end ratio attains it.

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
/// enclosure of `‖C′(t)‖` at `n + 1` parameters in the domain, the
/// greatest lower end — at most the true sup, so a bound below it is
/// unsound with certainty.
fn sampled_sup_floor(c: &NurbsCurve3<f64>, n: usize) -> f64 {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    let (lo, hi) = c.domain();
    #[allow(clippy::cast_precision_loss)]
    (0..=n)
        .map(|i| {
            // Clamped: `lo + (hi − lo)·n/n` can round one ulp past
            // `hi`, where the last span's speed keeps growing.
            let t = (lo + (hi - lo) * i as f64 / n as f64).clamp(lo, hi);
            ci.deriv(Interval::from_f64(t)).norm().lo()
        })
        .fold(0.0, f64::max)
}

/// The whole-net homogeneous recipe, about the first
/// control point `c`: `(max ‖A′ᵢ‖ + d·max |w′ᵢ|)/min w`, `A = w·(P − c)`,
/// `d` the net's reach from `c`, over the coefficients of nonempty
/// spans.
fn recipe_a(c: &NurbsCurve3<f64>) -> f64 {
    let (ctl, w, k, p) = (
        c.control(),
        c.weights(),
        c.knots_f64_interim().knots(),
        c.degree(),
    );
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
        "{name}: the quotient bound {m} above recipe A's {a}"
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
/// speed peaks at the ends, `C′(0) = 2·40·(P₁ − P₀)`, and the first
/// piece's first coefficient ratio is exactly that vector: `80√5`, which
/// the bound reads to rounding and the sampled sup meets.
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

/// **The pieces converge on the sup.** On the quadratic through
/// `(0, 0)`, `(1, 0)`, `(2, 0)` with weights `4, 1, 1` the speed peaks
/// at `3.3191`; the bound reads every piece's largest coefficient
/// ratio and sits within `0.1%` of it. Read as each span's one Bézier
/// piece it reads `5`.
#[test]
fn the_pieces_converge_on_the_sup() {
    let c = curve(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        &[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)],
        vec![4.0, 1.0, 1.0],
    );
    let ratio = check("collinear quadratic, weights 4 1 1", &c);
    assert!(
        ratio <= 1.001,
        "bound {} / sampled sup = {ratio}",
        c.speed_upper_bound().get()
    );
}

/// **The bound is local.** A degree-2 spline over three spans whose
/// last point lies `100` out and is active on the last span only; the
/// first pair's weight step is `9`. Reading that pair against every
/// control point — a sound but global bound — meets the far point at
/// `‖10·(P₁ − P₀) − 9·(P₄ − P₀)‖ ≈ 900` per unit scale; the bound reads
/// each pair against its own span's points, and stays near the last
/// span's own `‖P₄ − P₃‖ ≈ 100`.
#[test]
fn the_bound_reads_each_pair_against_its_own_spans_points() {
    let c = curve(
        vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0],
        2,
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (2.0, 1.0), (100.0, 1.0)],
        vec![1.0, 10.0, 1.0, 1.0, 1.0],
    );
    let m = c.speed_upper_bound().get();
    // The global reading of pair 0: scale `2/(u₂ − u₁) = 2`, least
    // weight `1`.
    let global = 2.0 * (10.0f64 * 1.0 - 9.0 * 100.0).hypot(-9.0);
    assert!(
        m < global / 4.0,
        "bound {m} reads the far point through the first pair (global {global})"
    );
    check("a far point active on the last span only", &c);
}

/// **A subnormal-positive least weight reads no low bound.** The
/// bound divides by the least weight, which here is the least subnormal
/// (`f64::from_bits(1)`): it must read non-finite, or at least the
/// sampled sup, never a low number. One carrier with the subnormal in
/// the middle (the curve is nearly the chord) and one with it at both
/// ends (the speed there is enormous).
#[test]
fn a_subnormal_least_weight_reads_no_low_bound() {
    let w = f64::from_bits(1);
    for (name, weights) in [("middle", vec![1.0, w, 1.0]), ("ends", vec![w, 1.0, w])] {
        let c = curve(
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            2,
            &[(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)],
            weights,
        );
        let m = c.speed_upper_bound().get();
        let floor = sampled_sup_floor(&c, 4000);
        assert!(
            !m.is_finite() || m >= floor,
            "{name}: bound {m} below the sampled sup {floor}"
        );
    }
}

/// **Six decades of weight are covered.** On this quadratic
/// (`P₀ = P₁`, weights over six decades) the speed peaks sharply near
/// `658.6`, and the bound covers the sampled sup.
#[test]
fn six_decades_of_weight_are_covered() {
    let (p0, p2) = ((-0.76958, 0.67766), (-0.21312, -0.94362));
    let w = [563.52, 1.1505e-3, 1.6169e-3];
    let c = curve(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        &[p0, p0, p2],
        w.to_vec(),
    );
    check("six decades of weight", &c);
}
