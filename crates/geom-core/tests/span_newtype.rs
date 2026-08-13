//! The `Span` newtype's invariants, and the one arithmetic property of
//! `basis_funs` that a plausible-looking simplification would break.
//!
//! `Span`'s whole value is that an invalid span index is not
//! representable, so what needs testing is the two constructors: that
//! the checked one refuses empty spans, that the total one never
//! produces one (its `debug_assert` is the fail-loud guard; this is the
//! test that says it never has to fire), and that the window it carries
//! really is `[index − degree, index]`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::basis::basis_funs;
use geom_core::spline::{KnotVector, Span};

fn kv(knots: Vec<f64>, degree: usize) -> KnotVector {
    KnotVector::clamped(knots, degree).expect("valid knot vector")
}

/// Uniform cubic, an interior-multiplicity-2 cubic (which has an empty
/// span), a degree-1 vector, and a degree-5 vector.
fn vectors() -> Vec<KnotVector> {
    vec![
        kv(
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0],
            3,
        ),
        kv(
            vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.7, 2.0, 2.0, 2.0, 2.0],
            3,
        ),
        kv(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1),
        kv(
            vec![0.0; 6]
                .into_iter()
                .chain([0.4, 0.9])
                .chain(vec![1.0; 6])
                .collect(),
            5,
        ),
    ]
}

#[test]
fn the_window_is_the_control_window() {
    for k in vectors() {
        for index in k.first_span()..=k.last_span() {
            let Some(span) = k.span(index) else { continue };
            assert_eq!(span.index(), index);
            assert_eq!(span.degree(), k.degree());
            assert_eq!(span.first_control(), index - k.degree());
            assert_eq!(*span.window().start(), index - k.degree());
            assert_eq!(*span.window().end(), index);
            assert_eq!(span.window().count(), k.degree() + 1);
            // The window is inside the control array — the fact every
            // evaluator's indexing rests on.
            assert!(*span.window().end() < k.control_count());
        }
    }
}

#[test]
fn the_checked_constructor_refuses_empty_and_out_of_range_indices() {
    let m = kv(
        vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.7, 2.0, 2.0, 2.0, 2.0],
        3,
    );
    let empty = (m.first_span()..=m.last_span())
        .find(|i| !m.span_is_nonempty(*i))
        .expect("this vector has an empty span");
    assert!(m.span(empty).is_none(), "empty span must not construct");
    assert!(m.span(m.first_span() - 1).is_none(), "below first_span");
    assert!(m.span(m.last_span() + 1).is_none(), "above last_span");
    assert!(m.span(usize::MAX).is_none(), "no overflow, just None");
}

/// `span_at` is total on all of `f64` and its postcondition — the span
/// it names is nonempty — is what the basis denominators rest on. In a
/// release build the `debug_assert` is gone, so an empty span would
/// divide by a zero knot difference and poison silently. Pin it.
#[test]
fn span_at_is_total_and_never_lands_on_an_empty_span() {
    for k in vectors() {
        let (lo, hi) = k.domain();
        let mut params: Vec<f64> = k.knots().to_vec();
        params.extend([
            lo,
            hi,
            -0.0,
            0.0,
            lo - 1.0,
            hi + 1.0,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MIN,
            f64::MAX,
        ]);
        for step in 0..=100 {
            params.push(lo + (hi - lo) * (f64::from(step) / 100.0));
        }
        for t in params {
            let span: Span = k.span_at(t);
            assert!(
                k.span_is_nonempty(span.index()),
                "span_at({t}) landed on empty span {}",
                span.index()
            );
            assert!(span.index() >= k.first_span() && span.index() <= k.last_span());
            // The total constructor and the checked one agree, and both
            // agree with `find_span` — one located index, three names.
            assert_eq!(span.index(), k.find_span(t));
            assert_eq!(k.span(span.index()), Some(span));
        }
    }
}

/// The high end of `basis_funs`' shift-and-add is `n[j] = saved`, with
/// **no addition**. Writing `saved + 0` instead — which looks like a
/// harmless uniformity — flips `-0.0` to `+0.0`, and D9 makes that a
/// re-baseline rather than a refactor. The discriminating input is
/// `t = -0.0`; verified red-then-green against that mutation.
#[test]
fn basis_funs_keeps_the_negative_zero_at_the_high_end() {
    let k = kv(
        vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0],
        3,
    );
    let row = basis_funs::<f64>(&k, 3, -0.0);
    let last = row.last().expect("degree + 1 entries");
    assert!(
        *last == 0.0 && last.is_sign_negative(),
        "the high end must stay a NEGATIVE zero, got {last} ({:#x})",
        last.to_bits()
    );
}
