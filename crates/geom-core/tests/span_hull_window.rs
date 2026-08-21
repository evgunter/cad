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

mod span_fixtures;
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
// The pairing refusal at `geom-core`'s own doors.
//
// `basis_funs`, `ders_basis_funs` and every span-restricted `hull`
// entry point are `pub` and take a `(kv, span)` pair that nothing tied
// together. A span of a different vector used to index past the knot
// array (or underflow below it) and panic; each door now asks
// `KnotVector::admits` and poisons instead.
//
// The residue these rows must NOT be read as excluding: a span from a
// different vector of the same degree and control count, nonempty at
// that index in both, is admitted, and the bound is then computed over
// the wrong window — wrong rather than refused. `admits` relates a span
// to a vector's shape only, never to its knot values.

/// Every `(source, target)` pair of the spread, at every span: nothing
/// panics, and a pair the target refuses yields **poison** from the
/// hull doors and an all-NaN basis row.
///
/// The outcome classes are counted **separately** and each floored —
/// admitted, and one per compare `admits` makes — because a class that
/// reaches zero means the spread stopped exercising that compare, and
/// the row would then be green over a guard part of which it never
/// drove. Classified by the FIRST condition to fail, the only partition
/// that attributes a refusal to one compare.
///
/// That is not hypothetical: the shared [`vectors()`] fixture has five
/// vectors of four distinct degrees and its two cubics are the same
/// length, so on that list alone the index compare never separates
/// anything. [`same_degree_different_length`] is what supplies it.
#[test]
fn a_foreign_span_poisons_every_span_door_and_panics_at_none() {
    let (mut admitted, mut by_degree, mut by_index, mut by_empty) =
        (0usize, 0usize, 0usize, 0usize);
    for (source_name, source) in spread() {
        for index in source.first_span()..=source.last_span() {
            let Some(span) = source.span(index) else {
                continue;
            };
            for (target_name, target) in spread() {
                let coeffs = base_coeffs(target.control_count());
                let weights = vec![1.0; target.control_count()];
                let h = hull::span_hull(&target, &coeffs, span);
                let r = hull::span_hull_rational(&target, &coeffs, &weights, span);
                let d = hull::derivative_span_hull(&target, &coeffs, span);
                let s = hull::sup_norm_bound_span(&target, &coeffs, span);
                let row = basis_funs::<f64>(&target, span, 0.5);
                let ders = geom_core::spline::basis::ders_basis_funs::<f64>(&target, span, 0.5, 2);
                assert_eq!(row.len(), target.degree() + 1);
                assert_eq!(ders.len(), 3);
                // The contract, written independently of the guard:
                // what MUST be refused. Asserting behaviour against
                // this rather than against `admits` is what makes the
                // row red when a compare is deleted — gating on
                // `admits` would simply reclassify and assert nothing.
                let where_ = format!("{source_name} span {index} against {target_name}");
                let refuse = span.degree() != target.degree()
                    || span.index() > target.last_span()
                    || !target.span_is_nonempty(span.index());
                assert_eq!(
                    target.admits(span),
                    !refuse,
                    "{where_}: `admits` disagrees with the span contract"
                );
                if !refuse {
                    admitted += 1;
                } else {
                    if span.degree() != target.degree() {
                        by_degree += 1;
                    } else if span.index() > target.last_span() {
                        by_index += 1;
                    } else {
                        by_empty += 1;
                    }
                    assert!(h.is_poison(), "{where_}: span_hull answered");
                    assert!(r.is_poison(), "{where_}: span_hull_rational answered");
                    assert!(d.is_poison(), "{where_}: derivative_span_hull answered");
                    assert!(s.is_nan(), "{where_}: sup_norm_bound_span answered {s}");
                    assert!(
                        row.iter().all(|n| n.is_nan()),
                        "{where_}: basis_funs answered"
                    );
                    assert!(
                        ders.iter().flatten().all(|n| n.is_nan()),
                        "{where_}: ders_basis_funs answered"
                    );
                }
            }
        }
    }
    for (class, n) in [
        ("admitted", admitted),
        ("refused on the degree compare", by_degree),
        ("refused on the index compare", by_index),
        ("refused on the emptiness compare", by_empty),
    ] {
        assert!(n > 0, "the spread no longer produces any pairing {class}");
    }
}

/// **Why the index compare has no behavioural evidence, pinned as the
/// fact it rests on.**
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

/// **The two `pub` predicates on one `KnotVector` must not disagree
/// about one index**, and the exit where disagreeing is worst.
///
/// `KnotVector::span(i)` returns `None` for an index that names an
/// empty span; before the emptiness compare, `admits` accepted the same
/// index when the `Span` came from another vector of the same shape.
/// Two public predicates on the same type, contradicting each other
/// about the same index, through public constructors in safe Rust.
///
/// What made it worth a compare rather than a paragraph is the exit:
/// this module reads **no knots** on the hull path, so nothing poisons
/// by arithmetic — `span_hull` returned a perfectly finite bound over a
/// window the basis never reads, and `sup_norm_bound_span(..) <= eps`
/// is the C2.2 honesty limb. A finite wrong bound there **certifies a
/// span it never bounded**, which is a worse failure than the panic
/// this whole change is about.
#[test]
fn an_empty_span_of_this_vector_is_refused_however_it_was_minted() {
    // Two cubics, same length, same control count. `0.5` is doubled in
    // `mult`, so span 5 is empty there and nonempty in `uni`.
    let uni = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3)
        .expect("valid");
    let mult = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3)
        .expect("valid");
    assert_eq!(uni.degree(), mult.degree());
    assert_eq!(uni.control_count(), mult.control_count());
    assert_eq!(uni.last_span(), mult.last_span());

    // Derived, not hard-coded: the index `mult` refuses and `uni` does
    // not. Writing the number in by hand got it wrong once already, and
    // a wrong index here makes the row assert nothing.
    let empty = (mult.first_span()..=mult.last_span())
        .find(|i| mult.span(*i).is_none() && uni.span(*i).is_some())
        .expect("the two cubics must disagree about some index");
    let span = uni.span(empty).expect("nonempty in the uniform cubic");
    // The other two compares have nothing to say about this pairing.
    assert_eq!(span.degree(), mult.degree());
    assert!(span.index() <= mult.last_span());

    let coeffs = base_coeffs(mult.control_count());
    let weights = vec![1.0; mult.control_count()];
    assert!(hull::span_hull(&mult, &coeffs, span).is_poison());
    assert!(hull::span_hull_rational(&mult, &coeffs, &weights, span).is_poison());
    assert!(hull::derivative_span_hull(&mult, &coeffs, span).is_poison());
    // The honesty limb: NaN fails `<= eps` under every direction, so a
    // span that was never bounded can no longer be certified.
    let bound = hull::sup_norm_bound_span(&mult, &coeffs, span);
    assert!(
        bound.is_nan(),
        "sup_norm_bound_span certified an empty span with a finite bound: {bound}"
    );
    assert!(!(bound <= 1.0e9), "a poisoned bound must fail `<= eps`");

    // And the basis row, which divides by the zero knot gap. Without
    // the compare this is `[-inf, NaN, NaN, inf]`, so `all(is_nan)` is
    // the discriminating assertion, not a restatement of the guard.
    // Off the doubled knot value: at `t == knots[empty]` the recursion
    // is all `0 * inf` and comes out all-NaN with or without the
    // compare, which would satisfy this assertion by accident.
    let row = basis_funs::<f64>(&mult, span, 0.3);
    assert!(
        row.iter().all(|n| n.is_nan()),
        "an empty span's basis row is not all-poison: {row:?}"
    );

    // LAST, deliberately: an assertion ABOUT the guard, tautologically
    // red if a compare is deleted and therefore evidence of nothing
    // downstream. The bounds and the row above are this row's evidence.
    // It is kept because the disagreement it names — two public
    // predicates on one `KnotVector` contradicting each other about one
    // index — is the finding, and it belongs where it was found.
    assert!(
        !mult.admits(span),
        "`span({empty})` refuses and `admits` accepts — two public \
         predicates on one KnotVector disagreeing about one index"
    );

    // The mirror: `mult`'s own span 4 is nonempty and `uni` admits it —
    // the residue, still open, stated as an executed fact rather than
    // as prose.
    let shared = (mult.first_span()..=mult.last_span())
        .find(|i| mult.span(*i).is_some() && uni.span(*i).is_some())
        .expect("the two cubics must share some nonempty index");
    assert!(
        uni.admits(mult.span(shared).expect("nonempty")),
        "the residue this row must not be read as closing"
    );
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
