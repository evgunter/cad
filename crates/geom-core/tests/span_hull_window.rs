//! A `CoeffWindow`'s coefficient window is **exactly** its `Span`'s
//! window, and that window is exactly the set of coefficients the basis
//! reads.
//!
//! `CoeffWindow::hull` reads the window the `Span` computed once at
//! construction, so the claim is one ACROSS two modules — `hull`
//! trusting `knots` — rather than one function's internal consistency.
//! This suite is what makes that crossing behavioural rather than
//! argued.
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

use geom_core::spline::KnotVector;
use geom_core::spline::basis::basis_funs;

use crate::span_fixtures;
use span_fixtures::vectors;

/// Far outside the base coefficients' `[0, 1)` range, in both
/// directions, so the perturbed coefficient is the strict extremum of
/// any window containing it.
const HIGH: f64 = 1.0e6;
const LOW: f64 = -1.0e6;

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
    for (j, nj) in basis_funs::<f64>(span, t).iter().enumerate() {
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
            let hull0 = k.coeffs(&base).unwrap().span(index).unwrap().hull();
            assert!(!hull0.is_poison(), "{name}: span {index} must bound");
            let mid = 0.5 * (k.knots()[index] + k.knots()[index + 1]);
            let value0 = eval(&k, &base, mid);
            for moved in 0..n {
                let in_window = span.window().contains(&moved);
                for outlier in [HIGH, LOW] {
                    let mut coeffs = base.clone();
                    coeffs[moved] = outlier;
                    let win = k.coeffs(&coeffs).unwrap().span(index).unwrap();
                    assert_eq!(win.span(), span, "the pair mints the vector's own span");
                    let h = win.hull();
                    // The hull moves iff the coefficient is in the
                    // window; when it does, the outlier IS the new
                    // extremum, which is the tightness claim too.
                    let reached = if outlier > 0.0 {
                        h.hi() == outlier
                    } else {
                        h.lo() == outlier
                    };
                    assert_eq!(
                        reached,
                        in_window,
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
                let h = k
                    .coeffs_rational(&coeffs, &weights)
                    .unwrap()
                    .span(index)
                    .unwrap()
                    .hull_rational();
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
            let d0 = k
                .coeffs(&base)
                .unwrap()
                .span(index)
                .unwrap()
                .derivative_hull();
            assert!(!d0.is_poison(), "{name}: span {index} derivative bound");
            for moved in 0..n {
                // `Q_i` mixes `c_i` and `c_{i+1}`, so a coefficient is
                // read by the hulled range `[s − p, s − 1]` when it or
                // its predecessor lies in it: `[s − p, s]` — the full
                // window again, minus nothing at the low end.
                let touches = span.window().contains(&moved);
                let mut coeffs = base.clone();
                coeffs[moved] = HIGH;
                let d = k
                    .coeffs(&coeffs)
                    .unwrap()
                    .span(index)
                    .unwrap()
                    .derivative_hull();
                let moved_bound =
                    d.lo().to_bits() != d0.lo().to_bits() || d.hi().to_bits() != d0.hi().to_bits();
                assert_eq!(
                    moved_bound,
                    touches,
                    "{name}: span {index} derivative, coefficient {moved} vs window {:?}",
                    span.window(),
                );
            }
        }
    }
}

// ---------------------------------------------------------------------
// The pairing at `geom-core`'s own doors.
//
// `basis_funs`, `ders_basis_funs` and every span-restricted `hull`
// door read through a borrow and take no second structure: a `Span`
// borrows the vector it is a proof about, and a `CoeffWindow` is a
// pair of coefficients-with-their-vector beside a span of THAT
// vector. The rows that used to drive a foreign span or a foreign
// coefficient array through these doors have no argument to write;
// the claims they made are `compile_fail` claims now, on `Span` in
// `spline/knots.rs` and on `SplineCoeffs` in `spline/hull.rs`.
//
// What the rows below drive is the relation that survives as
// behaviour: the mint's count refusal, and that every window a pair
// mints answers what the whole-domain door hulls over that window.

/// **Why [`KnotVector::span`]'s index compare has no behavioural
/// evidence, pinned as the fact it rests on.**
///
/// On a clamped-v1 vector, `span_is_nonempty(i)` *implies* `first_span()
/// <= i <= last_span()`: the leading run `knots[0..=degree]` is
/// constant, so every `i < degree` has `knots[i] == knots[i+1]`, and the
/// trailing run `knots[len−degree−1..]` is constant, so every
/// `i > len−degree−2` does too. The emptiness compare therefore already
/// refuses every index the index compare would, and deleting the index
/// compare reds no behavioural row — which is exactly the shape a guard
/// takes when it cannot fail.
///
/// It is kept anyway, and this row is the price of keeping it. The
/// index compare is the bound that does **not** depend on the
/// end-multiplicity invariant, and clamped-v1 is a designed choice with
/// a documented absence beside it (`spline/mod.rs`: periodic and
/// unclamped forms are "a designed absence until a consumer exists").
/// Admit one and the implication above stops holding, silently, while
/// the emptiness compare goes on looking sufficient. So the redundancy
/// is **checked here rather than assumed**: if it ever stops holding,
/// this row reds and the index compare gets its evidence back.
///
/// The compares live in `KnotVector::span`, which is where a span is
/// minted and therefore the only place either can now fail.
#[test]
fn nonemptiness_implies_the_index_bound_under_clamped_v1() {
    let mut nonempty = 0usize;
    for (name, k) in spread() {
        // Past the end as well as inside it: `span_is_nonempty` is
        // total on `usize` and its own bound is part of the claim.
        for i in 0..k.knots().len() + 4 {
            if k.span_is_nonempty(i) {
                nonempty += 1;
                assert!(
                    i >= k.first_span() && i <= k.last_span(),
                    "{name}: span {i} is nonempty but outside [{}, {}] — the \
                     implication the index compare's redundancy rests on has \
                     stopped holding, so that compare is load-bearing again",
                    k.first_span(),
                    k.last_span()
                );
            }
        }
    }
    assert!(nonempty > 0, "the spread produced no nonempty spans");
}

/// **The mint's count refusal, kept as one behavioural row.** A
/// coefficient array that is not `control_count()` long is refused at
/// [`KnotVector::coeffs`] and [`KnotVector::coeffs_rational`] — the
/// bound that keeps every window inside the array, and the only
/// relation a length can state. It is the one check the free doors
/// used to carry per call, done once here instead.
#[test]
fn the_mint_refuses_exactly_the_wrong_length() {
    for (name, k) in spread() {
        let n = k.control_count();
        let ones = vec![1.0; n];
        for len in [n - 1, n, n + 1] {
            let coeffs = base_coeffs(len);
            assert_eq!(
                k.coeffs(&coeffs).is_some(),
                len == n,
                "{name}: {len} coefficients against control_count {n}"
            );
            assert_eq!(
                k.coeffs_rational(&coeffs, &ones).is_some(),
                len == n,
                "{name}: {len} coefficients, {n} weights"
            );
            assert_eq!(
                k.coeffs_rational(&base_coeffs(n), &vec![1.0; len])
                    .is_some(),
                len == n,
                "{name}: {n} coefficients, {len} weights"
            );
        }
    }
}

/// **Every window a pair mints answers the whole-domain door hulled
/// over that window**, at every vector in the spread and every span:
/// `win.hull()` is the plain hull of `coeffs[win.window()]`, the
/// domain door is the hull of every window's answer, and the sup-norm
/// readings are those hulls' magnitudes — so a window's bound is
/// never a claim the whole-domain door would not make.
#[test]
fn every_window_answers_what_the_domain_door_hulls_over_it() {
    let mut windows = 0usize;
    for (name, k) in spread() {
        let n = k.control_count();
        let mut coeffs = base_coeffs(n);
        // One outlier so the per-window hulls differ from each other.
        coeffs[n / 2] = HIGH;
        let pair = k.coeffs(&coeffs).unwrap();
        let mut acc: Option<(f64, f64)> = None;
        for index in k.first_span()..=k.last_span() {
            let Some(win) = pair.span(index) else {
                assert!(
                    k.span(index).is_none(),
                    "{name}: the pair refuses only what its vector refuses"
                );
                continue;
            };
            assert_eq!(win.coeffs(), pair);
            assert_eq!(win.span(), k.span(index).unwrap());
            let (lo, hi) = coeffs[win.window()]
                .iter()
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), c| {
                    (lo.min(*c), hi.max(*c))
                });
            let h = win.hull();
            assert_eq!(
                (h.lo(), h.hi()),
                (lo, hi),
                "{name}: span {index} window {:?}",
                win.window()
            );
            assert_eq!(win.sup_norm_bound().to_bits(), h.mag().to_bits());
            acc = Some(acc.map_or((lo, hi), |(a, b)| (a.min(lo), b.max(hi))));
            windows += 1;
        }
        let d = pair.domain_hull();
        assert_eq!((d.lo(), d.hi()), acc.unwrap(), "{name}: the domain door");
        assert_eq!(pair.sup_norm_bound().to_bits(), d.mag().to_bits());
        // `span_at` mints the same window `span` does, at the midpoint.
        for index in k.first_span()..=k.last_span() {
            let Some(win) = pair.span(index) else {
                continue;
            };
            let mid = 0.5 * (k.knots()[index] + k.knots()[index + 1]);
            assert_eq!(pair.span_at(mid), win, "{name}: span_at({mid})");
        }
    }
    assert!(windows > 0, "the spread produced no windows");
}

/// [`vectors()`] plus two vectors that are the **same degree as an
/// existing one and a different length**, which is the only shape the
/// index compare can separate on its own.
fn same_degree_different_length() -> Vec<(&'static str, KnotVector)> {
    vec![
        (
            "degree 1, shorter",
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("valid"),
        ),
        (
            "cubic, longer",
            KnotVector::clamped(
                vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0],
                3,
            )
            .expect("valid"),
        ),
    ]
}

/// The pairing sweep's knot-vector spread.
fn spread() -> Vec<(&'static str, KnotVector)> {
    let mut v = vectors();
    v.extend(same_degree_different_length());
    v
}
