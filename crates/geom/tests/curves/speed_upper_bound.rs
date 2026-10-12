//! **The rational speed bound** (`NurbsCurve3::speed_upper_bound`, the
//! pair-term bound of `geom_core::spline::rational_speed_sup`)
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

/// **The pin at `j ∉ {i, i + 1}`.** On the quadratic through `(0, 0)`,
/// `(1, 0)`, `(2, 0)` with weights `4, 1, 1`, the largest pair term is
/// pair `0` read at `P₂`:
/// `‖1·(P₁ − P₀) − (1 − 4)·(P₂ − P₀)‖ = 7`, scale `2`, least weight `1`,
/// so the bound is `14` exactly. Dropping the `−Δw·(P_j − Pᵢ)` term
/// reads `2`, and reading `j` over the pair's own points only reads
/// `8` (pair `0` at `P₁`, `w₀·‖P₁ − P₀‖ = 4`).
#[test]
fn the_bound_reads_its_largest_term_at_a_point_outside_the_pair() {
    let c = curve(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        &[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)],
        vec![4.0, 1.0, 1.0],
    );
    assert_eq!(c.speed_upper_bound().get(), 14.0);
    check("collinear quadratic, weights 4 1 1", &c);
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

/// **A refinement that rounds a weight to zero reads no finite low
/// bound.** On a net of the least subnormal weight, a midpoint
/// insertion forms `½·w + ½·w`, and each half rounds to zero: the
/// refined curve holds a zero weight the constructor would refuse.
/// The bound divides by that zero least weight, so it reads `+∞` or
/// NaN, never a number below the true speed.
/// The insertion side: `work/nurbs/knot-insertion-can-round-a-weight-to-zero.md`.
#[test]
fn a_weight_collapsed_by_refinement_reads_no_finite_bound() {
    let w = f64::from_bits(1);
    let c = curve(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        &[(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)],
        vec![w, w, w],
    );
    let refined = c.insert_knot(0.5, 1).expect("the insertion is admitted");
    assert!(
        refined.weight_channel().contains(&0.0),
        "the midpoint insertion kept every weight positive: {:?}",
        refined.weight_channel()
    );
    let m = refined.speed_upper_bound().get();
    assert!(!m.is_finite(), "a collapsed weight read a finite bound {m}");
}

/// **The pair's own points do not suffice.** On this quadratic
/// (`P₀ = P₁`, weights over six decades) the speed peaks near
/// `658.6` while every pair term read at the pair's own two points
/// stays below `4.9`: reading `j` over the pair only would be unsound
/// by `136×`. `j` runs over every active point, and the row checks the
/// sampled sup against the bound and against that pair-only reading.
/// (Part 1's search for such a carrier drew weights over too narrow a
/// range to find one.)
#[test]
fn reading_the_pair_terms_at_the_pairs_own_points_is_unsound() {
    let (p0, p2) = ((-0.76958, 0.67766), (-0.21312, -0.94362));
    let w = [563.52, 1.1505e-3, 1.6169e-3];
    let c = curve(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        &[p0, p0, p2],
        w.to_vec(),
    );
    let pts = [p0, p0, p2];
    let least = w.iter().copied().fold(f64::INFINITY, f64::min);
    let mut pair_only = 0.0f64;
    for i in 0..2 {
        for j in [i, i + 1] {
            let v: [f64; 2] = core::array::from_fn(|k| {
                let at = |p: (f64, f64)| if k == 0 { p.0 } else { p.1 };
                w[i + 1] * (at(pts[i + 1]) - at(pts[i]))
                    - (w[i + 1] - w[i]) * (at(pts[j]) - at(pts[i]))
            });
            pair_only = pair_only.max(2.0 * v[0].hypot(v[1]) / least);
        }
    }
    let floor = sampled_sup_floor(&c, 4000);
    assert!(
        floor > 10.0 * pair_only,
        "sampled sup {floor} vs the pair-only reading {pair_only}"
    );
    check("pair-only counterexample", &c);
}
