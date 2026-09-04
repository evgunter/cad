//! The `Span` newtype's invariants.
//!
//! `Span`'s whole value is that an invalid span index is not
//! representable, so what needs testing is the two constructors: that
//! the checked one refuses empty spans, that the total one never
//! produces one (its `debug_assert` is the fail-loud guard; this is the
//! test that says it never has to fire), and that the window it carries
//! really is `[index − degree, index]`.
//!
//! The knot-vector spread is [`span_fixtures::vectors()`], shared with
//! the other two `Span` suites.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::Span;

use crate::span_fixtures;
use span_fixtures::{multiplicity_2_cubic, vectors};

#[test]
fn the_window_is_the_control_window() {
    for (name, k) in vectors() {
        for index in k.first_span()..=k.last_span() {
            let Some(span) = k.span(index) else { continue };
            assert_eq!(span.index(), index, "{name}");
            assert_eq!(span.degree(), k.degree(), "{name}");
            assert_eq!(span.first_control(), index - k.degree(), "{name}");
            assert_eq!(*span.window().start(), index - k.degree(), "{name}");
            assert_eq!(*span.window().end(), index, "{name}");
            assert_eq!(span.window().count(), k.degree() + 1, "{name}");
            // The window is inside the control array — the fact every
            // evaluator's indexing rests on.
            assert!(*span.window().end() < k.control_count(), "{name}");
        }
    }
}

#[test]
fn the_checked_constructor_refuses_empty_and_out_of_range_indices() {
    let m = multiplicity_2_cubic();
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
    for (name, k) in vectors() {
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
                "{name}: span_at({t}) landed on empty span {}",
                span.index()
            );
            assert!(
                span.index() >= k.first_span() && span.index() <= k.last_span(),
                "{name}: span_at({t}) left the span range"
            );
            // The total constructor and the checked one agree, and both
            // agree with `find_span` — one located index, three names.
            assert_eq!(span.index(), k.find_span(t));
            assert_eq!(k.span(span.index()), Some(span));
        }
    }
}
