//! **Adopted adversarial-review scratch suite for the hull primitives**
//! (M5 PR 2 review). Recreated faithfully from the review's description.
//!
//! Two attacks the implementer's own suite did not run:
//!
//! 1. **Near-collapse weights.** The convex-hull property for a rational
//!    spline needs only `w > 0`, but at `w ∈ {1e-300, 1e+300}` the
//!    rational basis is numerically a step function: one control value
//!    dominates almost everywhere and the transition happens inside a
//!    single span. If the positive-weight argument were subtly wrong,
//!    this is where a sample would escape the hull.
//! 2. **Exact derivative coefficients.** With dyadic knots (multiples of
//!    `1/64`) and dyadic coefficients (multiples of `1/1024`), the true
//!    derivative coefficient `p·(c_{i+1} − c_i)/(u_{i+p+1} − u_{i+1})` is
//!    an exact rational whose numerator and denominator are small
//!    integers once scaled — so containment is checkable by **`i128`
//!    cross-multiplication** against exact truth, with no floating-point
//!    re-evaluation anywhere. The suite also pins the enclosure's
//!    **width at ≤ 16 relative ulps**: a sound-but-slack derivative
//!    bound would pass containment and fail here.

test_utils::gated_to![
    "crates/geom-core/src/spline/",
    "crates/geom-core/src/ring_interval.rs",
];

use geom_core::spline::{KnotVector, basis, hull};
use test_utils::fuzz;

/// A clamped knot vector on `[0, 1]` whose interior knots are multiples
/// of `1/64` — dyadic, so every knot difference is exact in `f64` and
/// exactly an integer once scaled by 64.
fn dyadic_kv(rng: &mut fuzz::Rng, degree: usize, interior: usize) -> Option<KnotVector> {
    let mut inner: Vec<i64> = Vec::new();
    for _ in 0..interior {
        inner.push(1 + rng.below(63) as i64);
    }
    inner.sort_unstable();
    inner.dedup();
    let mut knots = vec![0.0; degree + 1];
    knots.extend(inner.iter().map(|k| *k as f64 / 64.0));
    knots.extend(std::iter::repeat_n(1.0, degree + 1));
    KnotVector::clamped(knots, degree).ok()
}

/// Coefficients that are multiples of `1/1024` — exact in `f64`, exact
/// integers once scaled by 1024.
fn dyadic_coeffs(rng: &mut fuzz::Rng, n: usize) -> (Vec<f64>, Vec<i64>) {
    let ints: Vec<i64> = (0..n).map(|_| rng.below(4096) as i64 - 2048).collect();
    let vals = ints.iter().map(|k| *k as f64 / 1024.0).collect();
    (vals, ints)
}

/// Rational spline value at `t` — the sampling oracle, independent of
/// the hull machinery.
fn eval_rational(kv: &KnotVector, coeffs: &[f64], weights: &[f64], t: f64) -> f64 {
    let span = kv.span_at(t);
    let nvals = basis::basis_funs(kv, span, t);
    let first = span.first_control();
    let (mut num, mut den) = (0.0, 0.0);
    for (j, nj) in nvals.iter().enumerate() {
        let w = nj * weights[first + j];
        num += w * coeffs[first + j];
        den += w;
    }
    num / den
}

#[test]
fn near_collapse_weights_never_escape_the_hull() {
    // Weights straddling the whole representable range while staying
    // strictly positive and finite: the precondition holds, so the bound
    // must hold, however degenerate the basis looks numerically.
    let mut rng = fuzz::start("review_m5_pr2_scratch_hull::near_collapse_weights");
    let mut samples = 0u64;
    let mut worst = 0.0f64;
    for degree in 1..=5usize {
        for _ in 0..fuzz::scaled(38) {
            let interior = rng.below(4);
            let Some(kv) = dyadic_kv(&mut rng, degree, interior) else {
                continue;
            };
            let n = kv.control_count();
            let coeffs: Vec<f64> = (0..n).map(|_| rng.range(-100.0, 100.0)).collect();
            let weights: Vec<f64> = (0..n)
                .map(|_| {
                    if rng.next_u64() & 1 == 0 {
                        1e-300
                    } else {
                        1e300
                    }
                })
                .collect();
            let domain = hull::domain_hull_rational(&kv, &coeffs, &weights);
            assert!(
                !domain.is_poison(),
                "positive weights must not poison — {}",
                fuzz::replay()
            );
            for index in kv.first_span()..=kv.last_span() {
                let Some(span) = kv.span(index) else { continue };
                let b = hull::span_hull_rational(&kv, &coeffs, &weights, span);
                let (u0, u1) = (kv.knots()[index], kv.knots()[index + 1]);
                // Falsification grid across the span; its density is a
                // sweep count like any other, so it rides the EFFORT dial.
                let grid = fuzz::scaled(8);
                for k in 0..=grid {
                    let frac = k as f64 / grid as f64;
                    let t = (u0 + (u1 - u0) * frac).min(u1);
                    let v = eval_rational(&kv, &coeffs, &weights, t);
                    if !v.is_finite() {
                        // The 1e±300 ratio can overflow the f64 SAMPLE
                        // (num and den both leave range). The bound is
                        // unaffected — it never evaluates — and a
                        // non-finite sample falsifies nothing.
                        continue;
                    }
                    let mag = b.mag().max(v.abs()).max(f64::MIN_POSITIVE);
                    let over = (b.lo() - v).max(v - b.hi()).max(0.0) / (mag * f64::EPSILON);
                    assert!(
                        over <= 64.0,
                        "near-collapse sample {v:e} outside [{:e}, {:e}] by {over} ulps — {}",
                        b.lo(),
                        b.hi(),
                        fuzz::replay()
                    );
                    assert!(
                        domain.contains(v) || over <= 64.0,
                        "near-collapse sample {v:e} outside the domain hull — {}",
                        fuzz::replay()
                    );
                    worst = worst.max(over);
                    samples += 1;
                }
            }
        }
    }
    println!(
        "[review-scratch hull] {samples} near-collapse (1e±300) rational samples, \
         0 escapes; worst overshoot {worst:.2} relative ulps"
    );
}

#[test]
fn derivative_coefficients_are_exact_by_i128_cross_multiplication() {
    // Q_i = p·(c_{i+1} − c_i) / (u_{i+p+1} − u_{i+1}).
    // With c = C/1024 and u = U/64 (C, U integers), the true value is
    //     Q_i = p·(C_{i+1} − C_i)·64 / (1024·(U_{i+p+1} − U_{i+1}))
    // an exact rational NUM/DEN with small integer parts. Containment is
    // then `lo ≤ NUM/DEN ≤ hi`, i.e. `lo·DEN ≤ NUM` and `NUM ≤ hi·DEN`
    // — checked in i128 after scaling the bound by 2^k, with no
    // floating-point re-evaluation of the formula anywhere.
    let mut rng = fuzz::start("review_m5_pr2_scratch_hull::derivative_coefficients");
    let (mut checked, mut worst_ulps) = (0u64, 0.0f64);
    for degree in 1..=5usize {
        for _ in 0..fuzz::scaled(50) {
            let interior = rng.below(5);
            let Some(kv) = dyadic_kv(&mut rng, degree, interior) else {
                continue;
            };
            let n = kv.control_count();
            let (coeffs, cints) = dyadic_coeffs(&mut rng, n);
            let qs = hull::derivative_coeffs(&kv, &coeffs);
            assert_eq!(
                qs.len(),
                n - 1,
                "one derivative coefficient per gap — {}",
                fuzz::replay()
            );
            let u64ths: Vec<i64> = kv
                .knots()
                .iter()
                .map(|u| (u * 64.0).round() as i64)
                .collect();
            for (i, q) in qs.iter().enumerate() {
                assert!(
                    !q.is_poison(),
                    "degree {degree}, i {i}: poisoned — {}",
                    fuzz::replay()
                );
                let du = u64ths[i + degree + 1] - u64ths[i + 1];
                assert!(
                    du > 0,
                    "clamped-v1 guarantees a positive knot gap — {}",
                    fuzz::replay()
                );
                // NUM / DEN with DEN > 0.
                let num = i128::from(degree as i64) * i128::from(cints[i + 1] - cints[i]) * 64;
                let den = i128::from(du) * 1024;
                // Bound endpoints scaled to integers: both are dyadic
                // multiples of 2^-shift for a shift that covers the
                // 1/1024 and 1/64 grids plus the ring's one-ulp pads,
                // so compare via exact f64 → rational conversion.
                assert!(
                    cmp_bound_vs_ratio(q.lo(), num, den) != std::cmp::Ordering::Greater,
                    "deriv LO above truth: degree {degree} i {i} — {}",
                    fuzz::replay()
                );
                assert!(
                    cmp_bound_vs_ratio(q.hi(), num, den) != std::cmp::Ordering::Less,
                    "deriv HI below truth: degree {degree} i {i} — {}",
                    fuzz::replay()
                );
                // Width pin: a sound-but-slack bound fails here.
                //
                // The relative-ulp metric below is UNDEFINED for an
                // enclosure straddling zero (`c[i] == c[i+1]` makes the
                // true coefficient exactly 0): `q.mag()` is 0, the
                // `.max(MIN_POSITIVE)` clamp makes the denominator one
                // smallest subnormal, and "relative ulps" degenerates
                // into an absolute count of subnormal steps. So: relative
                // width where a relative width exists, an absolute cap
                // where it does not.
                //
                // The absolute cap is DERIVED from the ring's pad
                // structure, not a constant a fresh seed can outgrow
                // (#489): around zero each outward pad is one subnormal
                // step, and the op chain `(dc) * degree / (du/64)` bounds
                // the half-width by (degree·1 + 1)·(64/du) + 1 steps —
                // the subtraction pads ±1, the scale multiplies and pads,
                // the division by the knot gap (du 64ths) multiplies by
                // 64/du and pads. The assert allows 2× that width for
                // op-order and subnormal-rounding freedom; a genuinely
                // slack bound overshoots by orders of magnitude, never 2×.
                if q.lo() <= 0.0 && q.hi() >= 0.0 {
                    #[allow(clippy::cast_precision_loss)]
                    let half_steps = (degree as f64 + 1.0) * 64.0 / du as f64 + 2.0;
                    let abs_cap = 4.0 * half_steps * f64::MIN_POSITIVE * f64::EPSILON;
                    assert!(
                        q.width() <= abs_cap,
                        "derivative enclosure of an exactly-zero coefficient is \
                         {:e} wide (absolute cap {abs_cap:e}): [{:e}, {:e}] — \
                         degree {degree} i {i} num {num} den {den} du {du} \
                         c[i] {} c[i+1] {} — {}",
                        q.width(),
                        q.lo(),
                        q.hi(),
                        cints[i],
                        cints[i + 1],
                        fuzz::replay()
                    );
                    checked += 1;
                    continue;
                }
                let mag = q.mag().max(f64::MIN_POSITIVE);
                let ulps = q.width() / (mag * f64::EPSILON);
                assert!(
                    ulps <= 16.0,
                    "derivative enclosure is {ulps} relative ulps wide (cap 16): \
                     [{:e}, {:e}] — degree {degree} i {i} num {num} den {den} \
                     du {du} c[i] {} c[i+1] {} mag {mag:e} — {}",
                    q.lo(),
                    q.hi(),
                    cints[i],
                    cints[i + 1],
                    fuzz::replay()
                );
                worst_ulps = worst_ulps.max(ulps);
                checked += 1;
            }
        }
    }
    println!(
        "[review-scratch hull] {checked} derivative coefficients verified by exact \
         i128 cross-multiplication; worst enclosure width {worst_ulps:.2} relative \
         ulps (cap 16)"
    );
}

/// Exact ordering of an `f64` bound against the rational `num/den`
/// (`den > 0`), by `i128` cross-multiplication. The bound is itself
/// `±m·2^e`; multiply through by `den` and by `2^-e` so both sides are
/// integers, refusing (as `Equal`, i.e. "no verdict") only if the
/// products would leave `i128` — which the lane's grids never do.
fn cmp_bound_vs_ratio(x: f64, num: i128, den: i128) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    assert!(den > 0);
    if !x.is_finite() {
        return if x > 0.0 {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    }
    // x = m · 2^e exactly, with m an i128 and e ≤ 0 for this lane's
    // magnitudes; scale both sides by 2^-e.
    let bits = x.to_bits();
    let neg = bits >> 63 == 1;
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & 0xf_ffff_ffff_ffff;
    if x == 0.0 {
        return 0.cmp(&num.signum());
    }
    let (mut m, mut e) = if biased == 0 {
        (i128::from(frac), -1074)
    } else {
        (i128::from(frac | (1 << 52)), biased - 1075)
    };
    let tz = m.trailing_zeros() as i32;
    m >>= tz;
    e += tz;
    if neg {
        m = -m;
    }
    // Compare m·2^e vs num/den  ⇔  m·den·2^e vs num.
    // With e ≤ 0: m·den vs num·2^(-e); with e > 0: m·den·2^e vs num.
    let (lhs, rhs) = if e <= 0 {
        let sh = (-e) as u32;
        if sh > 100 {
            return Ordering::Equal; // out of the lane's design range
        }
        let Some(l) = m.checked_mul(den) else {
            return Ordering::Equal;
        };
        let Some(r) = num.checked_mul(1i128 << sh) else {
            return Ordering::Equal;
        };
        (l, r)
    } else {
        let sh = e as u32;
        if sh > 100 {
            return Ordering::Equal;
        }
        let Some(l) = m.checked_mul(den).and_then(|v| v.checked_mul(1i128 << sh)) else {
            return Ordering::Equal;
        };
        (l, num)
    };
    lhs.cmp(&rhs)
}
