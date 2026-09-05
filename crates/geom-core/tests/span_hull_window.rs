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
            let hull0 = hull::span_hull(&base, span);
            assert!(!hull0.is_poison(), "{name}: span {index} must bound");
            let mid = 0.5 * (k.knots()[index] + k.knots()[index + 1]);
            let value0 = eval(&k, &base, mid);
            for moved in 0..n {
                let in_window = span.window().contains(&moved);
                for outlier in [HIGH, LOW] {
                    let mut coeffs = base.clone();
                    coeffs[moved] = outlier;
                    let h = hull::span_hull(&coeffs, span);
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
                let h = hull::span_hull_rational(&coeffs, &weights, span);
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
            let d0 = hull::derivative_span_hull(&base, span);
            assert!(!d0.is_poison(), "{name}: span {index} derivative bound");
            for moved in 0..n {
                // `Q_i` mixes `c_i` and `c_{i+1}`, so a coefficient is
                // read by the hulled range `[s − p, s − 1]` when it or
                // its predecessor lies in it: `[s − p, s]` — the full
                // window again, minus nothing at the low end.
                let touches = span.window().contains(&moved);
                let mut coeffs = base.clone();
                coeffs[moved] = HIGH;
                let d = hull::derivative_span_hull(&coeffs, span);
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
// The pairing at `geom-core`'s own doors, and what is left of it.
//
// `basis_funs`, `ders_basis_funs` and every span-restricted `hull`
// entry point take a `Span<'_>` and no knot vector: the span borrows
// the vector it is a proof about and the door reads its knots from
// that borrow, so there is no second vector for one to disagree with
// and nothing here to refuse. The rows that used to drive a foreign
// span through these doors have no argument to write; the claim they
// made is a `compile_fail` claim now, and it lives on `Span` in
// `spline/knots.rs` as doctests.
//
// What is NOT closed, and is what the rows below drive: the
// coefficients. `span_hull` relates `coeffs` to the span's vector by
// LENGTH alone, so a same-length array from another curve passes and
// the bound is computed over the wrong data — wrong rather than
// refused.

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

/// **The one pairing the borrow does not close, as an executed fact.**
///
/// A [`Span`] carries its knot vector, so `span_hull`'s knots and its
/// span always agree. Its `coeffs` do not travel with either: they are
/// related to the span's vector by `coeffs.len() ==
/// kv.control_count()` and by nothing else, so a same-length array
/// from a **different** curve passes the count and the bound is
/// computed over the wrong data.
///
/// This row constructs that case and asserts the bound is *finite and
/// wrong* rather than poison — it is the residue stated as behaviour,
/// so that closing it later reds here instead of going unnoticed.
/// (`work/props/coefficients-carry-their-knot-vector.md`.)
#[test]
fn the_coefficient_pairing_is_still_length_only() {
    // Two cubics of the same length and the same control count, so
    // either curve's coefficient array passes the other's count.
    let uni = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3)
        .expect("valid");
    let other = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0], 3)
        .expect("valid");
    assert_eq!(uni.control_count(), other.control_count());

    let n = uni.control_count();
    let mine = base_coeffs(n);
    // The other curve's coefficients: same length, different values,
    // with one driven far out so the two hulls cannot coincide.
    let mut theirs = base_coeffs(n);
    theirs[0] = HIGH;

    let span = uni.span(uni.first_span()).expect("nonempty");
    let right = hull::span_hull(&mine, span);
    let wrong = hull::span_hull(&theirs, span);
    assert!(!right.is_poison(), "the correct pairing must answer");
    assert!(
        !wrong.is_poison(),
        "the length-only relation still admits a foreign coefficient array"
    );
    assert_ne!(
        (right.lo(), right.hi()),
        (wrong.lo(), wrong.hi()),
        "the foreign array must give a DIFFERENT bound — otherwise this \
         row would pass without exercising the residue"
    );
    // And the honesty limb answers a finite number on it, which is the
    // shape of the residue that matters: `sup_norm_bound_span(..) <= eps`
    // can certify a curve whose coefficients were never bounded.
    let bound = hull::sup_norm_bound_span(&theirs, span);
    assert!(
        bound.is_finite(),
        "the residue is a wrong answer, not a refusal: {bound}"
    );

    // The counts still refuse, which is what keeps the indexing in
    // range: a coefficient array of the wrong LENGTH is poison.
    let short = base_coeffs(n - 1);
    assert!(hull::span_hull(&short, span).is_poison());
    assert!(hull::derivative_span_hull(&short, span).is_poison());
    let weights = vec![1.0; n - 1];
    assert!(hull::span_hull_rational(&short, &weights, span).is_poison());
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
