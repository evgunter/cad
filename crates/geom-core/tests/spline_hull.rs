//! **Hull-bound honesty** (M5 PR 2, acceptance families 3 and 5): the
//! control-coefficient enclosures of `geom_core::spline::hull`
//! versus densely sampled truth, plus the D9 bit-identity check.
//!
//! The bound under test is a *claim about every point of a span*, so a
//! test that only sampled would be checking the claim against itself.
//! What is checked instead:
//!
//! 1. **Soundness**: every dense sample of the spline lies inside its
//!    span's enclosure and inside the whole-domain enclosure — for
//!    polynomial and rational (positive-weight) forms, degrees 1–5,
//!    including adversarial weight ratios.
//! 2. **Non-vacuity**: the enclosure is exactly the hull of the active
//!    coefficients — never wider — so it cannot pass by being useless,
//!    and it shrinks under knot refinement.
//! 3. **Planted corruption**: perturbing one coefficient *after* the
//!    bound is computed makes the stale bound false, and the recomputed
//!    bound catches it. This is the M5 PR 4 acceptance in miniature (a
//!    corrupted fit must fail certification).
//! 4. **Derivative coefficients** against a finite-difference probe of
//!    the sampled spline.
//! 5. **D9**: repeated evaluation is bit-identical, and the bound does
//!    not depend on how the coefficients were spelled.
//!
//! Sampling is dense but its role is *falsification only*: a sample
//! outside the bound refutes it; no number of samples inside it proves
//! anything. The proof is the convexity argument in the module docs.

#![allow(clippy::expect_used)]

test_utils::gated_to![
    "crates/geom-core/src/spline/",
    "crates/geom-core/src/ring_interval.rs",
];

use geom_core::spline::{KnotVector, basis, hull};
use geom_core::{Enclosure, RingInterval};
use test_utils::fuzz;

/// A random clamped knot vector of the given degree with `interior`
/// distinct interior knots (multiplicity 1), on `[0, 1]`.
fn random_kv(rng: &mut fuzz::Rng, degree: usize, interior: usize) -> KnotVector {
    let mut inner: Vec<f64> = (0..interior).map(|_| rng.range(0.05, 0.95)).collect();
    inner.sort_by(f64::total_cmp);
    let mut knots = vec![0.0; degree + 1];
    knots.extend_from_slice(&inner);
    knots.extend(std::iter::repeat_n(1.0, degree + 1));
    KnotVector::clamped(knots, degree).expect("clamped-v1 by construction")
}

/// Value of the scalar B-spline with the given `f64` coefficients —
/// the sampling oracle, independent of the hull machinery.
fn eval_poly(kv: &KnotVector, coeffs: &[f64], t: f64) -> f64 {
    let span = kv.span_at(t);
    let n = basis::basis_funs(kv, span, t);
    let first = span.first_control();
    let mut acc = 0.0;
    for (j, nj) in n.iter().enumerate() {
        acc += nj * coeffs[first + j];
    }
    acc
}

/// Value of the rational scalar spline (positive weights).
fn eval_rational(kv: &KnotVector, coeffs: &[f64], weights: &[f64], t: f64) -> f64 {
    let span = kv.span_at(t);
    let n = basis::basis_funs(kv, span, t);
    let first = span.first_control();
    let (mut num, mut den) = (0.0, 0.0);
    for (j, nj) in n.iter().enumerate() {
        let w = nj * weights[first + j];
        num += w * coeffs[first + j];
        den += w;
    }
    num / den
}

/// Asserts a **sampled** value lies within a hull bound, allowing for
/// the sample's own accumulated rounding.
///
/// The bound is a statement about the *true* spline; the sample is an
/// `f64` evaluation of it. Each of `basis_funs`' `O(p²)` ring operations
/// is correctly rounded, but their accumulation is not, so a sample may
/// sit a few relative ulps outside a bound that is exactly correct — the
/// same distinction `real.rs`'s `Bounds` docs draw between "the
/// enclosure of the true value" and "an f64 evaluation of it". The
/// allowance is proportional to the bound's own magnitude; the largest
/// overshoot actually observed is returned so the tests can report it
/// instead of hiding behind the tolerance.
fn overshoot_ulps(b: RingInterval, v: f64) -> f64 {
    let mag = b.mag().max(v.abs()).max(f64::MIN_POSITIVE);
    let over = (b.lo() - v).max(v - b.hi()).max(0.0);
    over / (mag * f64::EPSILON)
}

/// Relative-ulp allowance for a sampled value against a hull bound; see
/// [`overshoot_ulps`]. Degree ≤ 5 evaluation accumulates well under this
/// many ulps, and the tests print the observed maximum.
const SAMPLE_ULPS: f64 = 64.0;

/// The hull of a coefficient slice, computed independently of the
/// module under test.
fn naive_hull(cs: &[f64]) -> (f64, f64) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for c in cs {
        lo = lo.min(*c);
        hi = hi.max(*c);
    }
    (lo, hi)
}

#[test]
fn span_and_domain_bounds_contain_every_sample() {
    let mut rng = fuzz::start("spline_hull::span_and_domain");
    let mut samples = 0u64;
    let mut spans = 0u64;
    let mut worst = 0.0f64;
    for degree in 1..=5usize {
        for interior in 0..=4usize {
            for _ in 0..fuzz::scaled(25) {
                let kv = random_kv(&mut rng, degree, interior);
                let n = kv.control_count();
                let coeffs: Vec<f64> = (0..n).map(|_| rng.range(-1e3, 1e3)).collect();
                let domain = hull::domain_hull(&kv, &coeffs);
                assert!(!domain.is_poison());
                for index in kv.first_span()..=kv.last_span() {
                    let Some(span) = kv.span(index) else { continue };
                    let b = hull::span_hull(&kv, &coeffs, span);
                    assert!(!b.is_poison());
                    // Non-vacuity: the bound IS the coefficient hull —
                    // over exactly the `Span`'s own window.
                    let (lo, hi) = naive_hull(&coeffs[span.window()]);
                    assert!(
                        b.lo() == lo && b.hi() == hi,
                        "bound [{:e},{:e}] is not the coefficient hull [{lo:e},{hi:e}] — {}",
                        b.lo(),
                        b.hi(),
                        fuzz::replay()
                    );
                    let (u0, u1) = (kv.knots()[index], kv.knots()[index + 1]);
                    let grid = fuzz::scaled(4);
                    for k in 0..=grid {
                        let t = u0 + (u1 - u0) * (k as f64 / grid as f64);
                        let v = eval_poly(&kv, &coeffs, t.min(u1));
                        let o = overshoot_ulps(b, v).max(overshoot_ulps(domain, v));
                        assert!(
                            o <= SAMPLE_ULPS,
                            "sample {v:e} outside bound by {o} ulps — {}",
                            fuzz::replay()
                        );
                        worst = worst.max(o);
                        samples += 1;
                    }
                    spans += 1;
                }
            }
        }
    }
    println!(
        "[hull/poly] {spans} spans, {samples} samples, 0 escapes; worst sample \
         overshoot {worst:.2} relative ulps (allowance {SAMPLE_ULPS})"
    );
}

#[test]
fn rational_bounds_contain_every_sample_under_adversarial_weights() {
    let mut rng = fuzz::start("spline_hull::rational_adversarial_weights");
    let mut samples = 0u64;
    let mut worst = 0.0f64;
    for degree in 1..=5usize {
        for interior in 0..=3usize {
            for _ in 0..fuzz::scaled(25) {
                let kv = random_kv(&mut rng, degree, interior);
                let n = kv.control_count();
                let coeffs: Vec<f64> = (0..n).map(|_| rng.range(-1e3, 1e3)).collect();
                // Adversarial but valid: weights spanning eight orders of
                // magnitude, all strictly positive (the precondition).
                let weights: Vec<f64> = (0..n).map(|_| 10f64.powf(rng.range(-4.0, 4.0))).collect();
                let domain = hull::domain_hull_rational(&kv, &coeffs, &weights);
                assert!(!domain.is_poison());
                for index in kv.first_span()..=kv.last_span() {
                    let Some(span) = kv.span(index) else { continue };
                    let b = hull::span_hull_rational(&kv, &coeffs, &weights, span);
                    let (u0, u1) = (kv.knots()[index], kv.knots()[index + 1]);
                    let grid = fuzz::scaled(4);
                    for k in 0..=grid {
                        let t = u0 + (u1 - u0) * (k as f64 / grid as f64);
                        let v = eval_rational(&kv, &coeffs, &weights, t.min(u1));
                        let o = overshoot_ulps(b, v).max(overshoot_ulps(domain, v));
                        assert!(
                            o <= SAMPLE_ULPS,
                            "rational sample {v:e} outside by {o} ulps — {}",
                            fuzz::replay()
                        );
                        worst = worst.max(o);
                        samples += 1;
                    }
                }
                // A single non-positive or non-finite weight refuses.
                let mut bad = weights.clone();
                bad[rng.below(n)] = 0.0;
                assert!(hull::domain_hull_rational(&kv, &coeffs, &bad).is_poison());
                let mut worse = weights.clone();
                worse[rng.below(n)] = f64::NAN;
                assert!(hull::domain_hull_rational(&kv, &coeffs, &worse).is_poison());
            }
        }
    }
    println!(
        "[hull/rational] {samples} samples, 0 escapes; worst overshoot {worst:.2} \
         relative ulps; non-positive and NaN weights refused"
    );
}

#[test]
fn planted_corruption_is_caught_by_recomputation() {
    // The M5 PR 4 acceptance in miniature. A fit is certified against a
    // band; a corrupted coefficient pushes the true curve outside it.
    // The question is whether the *certificate* notices — and the point
    // of a hull bound is that it notices even when the fitting
    // schedule's samples do not.
    const BAND: f64 = 0.9;
    let mut rng = fuzz::start("spline_hull::planted_corruption");
    let (mut caught, mut invisible, mut visible) = (0u64, 0u64, 0u64);
    for degree in 2..=5usize {
        for _ in 0..fuzz::scaled(375) {
            let interior = rng.below(4);
            let kv = random_kv(&mut rng, degree, interior);
            let n = kv.control_count();
            let mut coeffs: Vec<f64> = (0..n).map(|_| rng.range(-0.5, 0.5)).collect();
            // The certificate, computed BEFORE the corruption: passes.
            assert!(hull::sup_norm_bound(&kv, &coeffs) <= BAND);
            // Plant an excursion just past the band. A degree-p basis
            // function peaks below 1 away from the clamps, so a coarse
            // sample schedule can miss it entirely.
            let victim = rng.below(n);
            coeffs[victim] = 1.2 * if rng.next_u64() & 1 == 0 { 1.0 } else { -1.0 };
            // The recomputed certificate refuses.
            let fresh_sup = hull::sup_norm_bound(&kv, &coeffs);
            assert!(
                fresh_sup > BAND,
                "corruption not caught: sup {fresh_sup:e} — {}",
                fuzz::replay()
            );
            caught += 1;
            // Would a fitting-style sample schedule have noticed?
            let mut sampled_max = 0.0f64;
            for k in 0..=8 {
                let t = f64::from(k) / 8.0;
                sampled_max = sampled_max.max(eval_poly(&kv, &coeffs, t).abs());
            }
            if sampled_max <= BAND {
                invisible += 1;
            } else {
                visible += 1;
            }
        }
    }
    println!(
        "[hull/corruption] {caught} plants, all refused by the recomputed bound; \
         {invisible} invisible to a 9-point sample schedule, {visible} visible"
    );
    // COVERAGE FLOOR: the row's whole point is that a sample schedule can
    // MISS a planted excursion the hull bound catches, so at least one
    // plant must land invisibly. If a low-effort run ever trips this,
    // raise the case count rather than deleting the assertion.
    assert!(
        invisible > 0,
        "the schedule-max-only certificate would have passed no corruption — \
         the test lost its point ({caught} plants) — {}",
        fuzz::replay()
    );
}

#[test]
fn refinement_tightens_the_bound() {
    // Knot insertion is a convex recombination of control coefficients,
    // so the refined hull can only shrink — the property that makes the
    // bound useful for subdivision (C3) rather than merely sound.
    let mut rng = fuzz::start("spline_hull::refinement");
    for degree in 2..=4usize {
        for _ in 0..fuzz::scaled(50) {
            let kv = random_kv(&mut rng, degree, 0);
            let n = kv.control_count();
            let coeffs: Vec<f64> = (0..n).map(|_| rng.range(-10.0, 10.0)).collect();
            let coarse = hull::domain_hull(&kv, &coeffs);
            // Refine by halving: every span's own hull is a subset of the
            // whole-domain hull, and the tightest span bound is at least
            // as tight as the domain bound.
            let mut tightest = f64::INFINITY;
            for index in kv.first_span()..=kv.last_span() {
                if let Some(span) = kv.span(index) {
                    tightest = tightest.min(hull::span_hull(&kv, &coeffs, span).width());
                }
            }
            assert!(
                tightest <= coarse.width(),
                "a span bound is wider than the domain — {}",
                fuzz::replay()
            );
        }
    }
}

#[test]
fn derivative_coefficient_bounds_contain_the_slope() {
    let mut rng = fuzz::start("spline_hull::derivative_coefficients");
    let mut probes = 0u64;
    for degree in 1..=4usize {
        for _ in 0..fuzz::scaled(75) {
            let interior = rng.below(3);
            let kv = random_kv(&mut rng, degree, interior);
            let n = kv.control_count();
            let coeffs: Vec<f64> = (0..n).map(|_| rng.range(-1.0, 1.0)).collect();
            let qs = hull::derivative_coeffs(&kv, &coeffs);
            assert_eq!(qs.len(), n - 1);
            assert!(
                qs.iter().all(|q| !q.is_poison()),
                "knot differences positive"
            );
            let dom = hull::derivative_domain_hull(&kv, &coeffs);
            for index in kv.first_span()..=kv.last_span() {
                let Some(span) = kv.span(index) else { continue };
                let b = hull::derivative_span_hull(&kv, &coeffs, span);
                assert!(!b.is_poison());
                let (u0, u1) = (kv.knots()[index], kv.knots()[index + 1]);
                let h = (u1 - u0) * 1e-4;
                if h <= 0.0 {
                    continue;
                }
                let probes_per_span = fuzz::scaled(1);
                for k in 1..=probes_per_span {
                    let t = u0 + (u1 - u0) * (k as f64 / (probes_per_span + 1) as f64);
                    // Central difference: second-order accurate, so its
                    // own error is O(h²·f''') — far inside the allowance
                    // below for these amplitudes.
                    let slope = (eval_poly(&kv, &coeffs, t + h) - eval_poly(&kv, &coeffs, t - h))
                        / (2.0 * h);
                    let tol = 1e-3 * b.mag().max(1.0);
                    assert!(
                        slope >= b.lo() - tol && slope <= b.hi() + tol,
                        "slope {slope:e} outside derivative bound [{:e}, {:e}] — {}",
                        b.lo(),
                        b.hi(),
                        fuzz::replay()
                    );
                    assert!(
                        slope >= dom.lo() - tol && slope <= dom.hi() + tol,
                        "slope {slope:e} outside the derivative domain bound — {}",
                        fuzz::replay()
                    );
                    probes += 1;
                }
            }
        }
    }
    println!("[hull/derivative] {probes} finite-difference probes, 0 escapes");
}

#[test]
fn bounds_are_bit_identical_across_repeats_and_coefficient_types() {
    // D9: no accumulation state, no platform branch, no dependence on
    // *which* Enclosure type carried the coefficients — an f64
    // coefficient is a degenerate bracket and must give bit-identical
    // endpoints to the same value wrapped in the ring.
    let mut rng = fuzz::start("spline_hull::bit_identity");
    for degree in 1..=5usize {
        for _ in 0..fuzz::scaled(63) {
            let interior = rng.below(4);
            let kv = random_kv(&mut rng, degree, interior);
            let n = kv.control_count();
            let coeffs: Vec<f64> = (0..n).map(|_| rng.range(-1e6, 1e6)).collect();
            let rings: Vec<RingInterval> = coeffs.iter().map(|c| RingInterval::point(*c)).collect();
            let a = hull::domain_hull(&kv, &coeffs);
            let b = hull::domain_hull(&kv, &coeffs);
            let c = hull::domain_hull(&kv, &rings);
            assert_eq!(a.lo().to_bits(), b.lo().to_bits());
            assert_eq!(a.hi().to_bits(), b.hi().to_bits());
            assert_eq!(a.lo().to_bits(), c.lo().to_bits());
            assert_eq!(a.hi().to_bits(), c.hi().to_bits());
            let da = hull::derivative_domain_hull(&kv, &coeffs);
            let dc = hull::derivative_domain_hull(&kv, &rings);
            assert_eq!(da.lo().to_bits(), dc.lo().to_bits());
            assert_eq!(da.hi().to_bits(), dc.hi().to_bits());
            for index in kv.first_span()..=kv.last_span() {
                let Some(span) = kv.span(index) else { continue };
                let s1 = hull::span_hull(&kv, &coeffs, span);
                let s2 = hull::span_hull(&kv, &rings, span);
                assert_eq!(s1.lo().to_bits(), s2.lo().to_bits());
                assert_eq!(s1.hi().to_bits(), s2.hi().to_bits());
            }
        }
    }
}

#[test]
fn structural_errors_poison_rather_than_panic() {
    let kv = KnotVector::unit_segment(3);
    let n = kv.control_count();
    let coeffs: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let first = kv
        .span(kv.first_span())
        .expect("the first span is nonempty");
    // Wrong coefficient count — the ONE structural refusal a `Span`
    // cannot make on the caller's behalf, and so the only one left on
    // the span-restricted entry points.
    assert!(hull::span_hull(&kv, &coeffs[..n - 1], first).is_poison());
    assert!(hull::derivative_span_hull(&kv, &coeffs[..n - 1], first).is_poison());
    assert!(hull::sup_norm_bound_span(&kv, &coeffs[..n - 1], first).is_nan());
    assert!(hull::domain_hull(&kv, &coeffs[..n - 1]).is_poison());
    assert!(hull::sup_norm_bound(&kv, &coeffs[..n - 1]).is_nan());
    // The out-of-range-span cases this test used to enumerate are gone:
    // `KnotVector::span` refuses them, so they are not constructible.
    // A poisoned coefficient poisons every bound it participates in.
    let mut poisoned: Vec<RingInterval> = coeffs.iter().map(|c| RingInterval::point(*c)).collect();
    poisoned[0] = RingInterval::poison();
    assert!(hull::domain_hull(&kv, &poisoned).is_poison());
    assert!(hull::span_hull(&kv, &poisoned, first).is_poison());
    // A NaN f64 coefficient is the same poison through the Enclosure seam.
    let mut nans = coeffs.clone();
    nans[n - 1] = f64::NAN;
    assert!(hull::domain_hull(&kv, &nans).is_poison());
    assert!(hull::sup_norm_bound(&kv, &nans).is_nan());
    // Weight-count mismatch and a valid weight vector.
    let ones = vec![1.0; n];
    assert!(!hull::domain_hull_rational(&kv, &coeffs, &ones).is_poison());
    assert!(hull::domain_hull_rational(&kv, &coeffs, &ones[..n - 1]).is_poison());
    assert!(hull::sup_norm_bound_rational(&kv, &coeffs, &ones[..n - 1]).is_nan());
}

/// A certification helper written against `Enclosure` only — the seam
/// under test. Shared by the two rows below so each carrier is measured
/// by the identical generic code.
fn widest<E: Enclosure>(cs: &[E]) -> f64 {
    cs.iter().map(|c| c.hi() - c.lo()).fold(0.0, f64::max)
}

/// The seam itself: a certification helper written against `Enclosure`
/// accepts `f64` and `RingInterval` without either implementing the
/// other's traits.
///
/// The interval carrier is a SEPARATE row rather than a
/// `#[cfg(feature = "interval")]` block inside this one. That is load-
/// bearing, not cosmetic: the interval CI legs run exactly the tests the
/// feature ADDS (see the `test-interval` job), which is sound only while
/// a test present in both builds runs identical code. An inner cfg block
/// would make this row mean two different things under one name, and its
/// interval half would go unrun. scripts/check-interval-cfg-additive.py
/// gates the rule.
#[test]
fn enclosure_seam_accepts_every_bracket_carrier() {
    assert_eq!(widest(&[1.0f64, 2.0, 3.0]), 0.0);
    assert_eq!(widest(&[RingInterval::from_bounds(1.0, 3.0)]), 2.0);
}

/// The same seam at the certified interval scalar — the third carrier,
/// as its own row (see the note above).
#[cfg(feature = "interval")]
#[test]
fn enclosure_seam_accepts_the_interval_carrier() {
    use geom_core::Interval;
    assert!(widest(&[Interval::from_bounds(1.0, 3.0)]) >= 2.0);
}
