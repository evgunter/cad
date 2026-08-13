//! `span_hull`'s coefficient window is **exactly** the `Span`'s window,
//! and that window is exactly the set of coefficients the basis reads.
//!
//! Before the fold, `hull::span_indices` re-derived `(span − p, span)`
//! behind its own copy of the range guard. It now returns the window the
//! `Span` computed once at construction, so the two claims that used to
//! be one function's internal consistency became a claim ACROSS two
//! modules — `hull` trusting `knots`. This suite is what makes that
//! crossing behavioural rather than argued.
//!
//! The discriminating move is a coefficient driven far outside the
//! others' range. A hull is a min/max, so nudging an interior
//! coefficient need not move it; making one coefficient the strict
//! extremum must move the hull **iff** it is in the window. The same
//! perturbation is then evaluated: at the span's midpoint every one of
//! the `p + 1` nonvanishing basis functions is strictly positive, so the
//! sampled value must move under exactly the same condition. Requiring
//! the two to agree is what pins hull-window == basis-window; an
//! off-by-one in either direction (`span..=span + p`, `span − p..span`)
//! fails one side or the other at the ends of the coefficient array.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::basis::basis_funs;
use geom_core::spline::{KnotVector, hull};

/// Far outside the base coefficients' `[0, 1)` range, in both
/// directions, so the perturbed coefficient is the strict extremum of
/// any window containing it.
const HIGH: f64 = 1.0e6;
const LOW: f64 = -1.0e6;

fn kv(knots: Vec<f64>, degree: usize) -> KnotVector {
    KnotVector::clamped(knots, degree).expect("valid knot vector")
}

fn vectors() -> Vec<(&'static str, KnotVector)> {
    vec![
        (
            "uniform cubic",
            kv(
                vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0],
                3,
            ),
        ),
        (
            "cubic, interior multiplicity 2",
            kv(
                vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.7, 2.0, 2.0, 2.0, 2.0],
                3,
            ),
        ),
        ("degree 1", kv(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1)),
        (
            "degree 5",
            kv(
                vec![0.0; 6]
                    .into_iter()
                    .chain([0.4, 0.9])
                    .chain(vec![1.0; 6])
                    .collect(),
                5,
            ),
        ),
        (
            "quadratic, interior multiplicity 2, nonuniform",
            kv(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.25, 3.0, 3.0, 3.0], 2),
        ),
    ]
}

/// Distinct, bounded base coefficients — nothing near `HIGH`/`LOW`.
fn base_coeffs(n: usize) -> Vec<f64> {
    #[allow(clippy::cast_precision_loss)]
    (0..n).map(|i| (i % 7) as f64 / 8.0).collect()
}

/// The scalar spline's value at `t`, indexed off the window base — the
/// sampling side of the pairing.
fn eval(k: &KnotVector, coeffs: &[f64], t: f64) -> f64 {
    let span = k.span_at(t);
    let first = span.first_control();
    let mut acc = 0.0;
    for (j, nj) in basis_funs::<f64>(k, span, t).iter().enumerate() {
        acc += nj * coeffs[first + j];
    }
    acc
}

#[test]
fn the_hull_window_is_the_spans_window_and_the_basis_reads_the_same_one() {
    for (name, k) in vectors() {
        let n = k.control_count();
        for index in k.first_span()..=k.last_span() {
            let Some(span) = k.span(index) else { continue };
            let base = base_coeffs(n);
            let hull0 = hull::span_hull(&k, &base, span);
            assert!(!hull0.is_poison(), "{name}: span {index} must bound");
            let mid = 0.5 * (k.knots()[index] + k.knots()[index + 1]);
            let value0 = eval(&k, &base, mid);
            for moved in 0..n {
                let in_window = span.window().contains(&moved);
                for outlier in [HIGH, LOW] {
                    let mut coeffs = base.clone();
                    coeffs[moved] = outlier;
                    let h = hull::span_hull(&k, &coeffs, span);
                    // The hull moves iff the coefficient is in the
                    // window; when it does, the outlier IS the new
                    // extremum, which is the tightness claim too.
                    let reached = if outlier > 0.0 {
                        h.hi() == outlier
                    } else {
                        h.lo() == outlier
                    };
                    assert_eq!(
                        reached, in_window,
                        "{name}: span {index} window {:?}, coefficient {moved} \
                         set to {outlier:e}: hull [{:e}, {:e}] vs base [{:e}, {:e}]",
                        span.window(),
                        h.lo(),
                        h.hi(),
                        hull0.lo(),
                        hull0.hi(),
                    );
                    // Unchanged coefficients must leave the bound
                    // untouched — bit-for-bit, not just "close".
                    if !in_window {
                        assert_eq!(h.lo().to_bits(), hull0.lo().to_bits());
                        assert_eq!(h.hi().to_bits(), hull0.hi().to_bits());
                    }
                    // The evaluation side: same window, decided by the
                    // basis rather than by the hull's index arithmetic.
                    let value = eval(&k, &coeffs, mid);
                    assert_eq!(
                        value.to_bits() != value0.to_bits(),
                        in_window,
                        "{name}: span {index} midpoint {mid}: coefficient {moved} \
                         {} the window but {} the value",
                        if in_window { "is in" } else { "is outside" },
                        if in_window { "did not move" } else { "moved" },
                    );
                }
            }
        }
    }
}

/// The rational entry point takes the same window: its weights are read
/// over the window it hulls, so a non-positive weight refuses **iff** it
/// is inside. A window shifted by one would refuse for the wrong spans.
#[test]
fn the_rational_window_refuses_on_exactly_its_own_weights() {
    for (name, k) in vectors() {
        let n = k.control_count();
        let coeffs = base_coeffs(n);
        for index in k.first_span()..=k.last_span() {
            let Some(span) = k.span(index) else { continue };
            for bad in 0..n {
                let mut weights = vec![1.0; n];
                weights[bad] = 0.0;
                let h = hull::span_hull_rational(&k, &coeffs, &weights, span);
                assert_eq!(
                    h.is_poison(),
                    span.window().contains(&bad),
                    "{name}: span {index} window {:?}, zero weight at {bad}",
                    span.window(),
                );
            }
        }
    }
}

/// The derivative form hulls the window minus its top end — `p` entries,
/// never zero, because degree 0 is not constructible. The `last == 0`
/// refusal the fold deleted was unreachable for exactly that reason;
/// this is the test that says so, over the smallest legal degree.
#[test]
fn the_derivative_window_is_the_window_minus_its_top() {
    for (name, k) in vectors() {
        let n = k.control_count();
        for index in k.first_span()..=k.last_span() {
            let Some(span) = k.span(index) else { continue };
            let base = base_coeffs(n);
            let d0 = hull::derivative_span_hull(&k, &base, span);
            assert!(!d0.is_poison(), "{name}: span {index} derivative bound");
            for moved in 0..n {
                // `Q_i` mixes `c_i` and `c_{i+1}`, so a coefficient is
                // read by the hulled range `[s − p, s − 1]` when it or
                // its predecessor lies in it: `[s − p, s]` — the full
                // window again, minus nothing at the low end.
                let touches = span.window().contains(&moved);
                let mut coeffs = base.clone();
                coeffs[moved] = HIGH;
                let d = hull::derivative_span_hull(&k, &coeffs, span);
                let moved_bound =
                    d.lo().to_bits() != d0.lo().to_bits() || d.hi().to_bits() != d0.hi().to_bits();
                assert_eq!(
                    moved_bound, touches,
                    "{name}: span {index} derivative, coefficient {moved} vs window {:?}",
                    span.window(),
                );
            }
        }
    }
}
