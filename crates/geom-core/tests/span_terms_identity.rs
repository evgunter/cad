//! `span_terms` must reproduce `basis_funs` BIT-IDENTICALLY: it is the
//! same recursion with the length taken from the control window instead
//! of from `degree`, and D9 makes any drift a re-baseline, not a
//! refactor. This is the test that earns the claim.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::basis::{basis_funs, span_terms};
use geom_core::spline::{KnotVector, Span};

fn kv(knots: Vec<f64>, degree: usize) -> KnotVector {
    KnotVector::clamped(knots, degree).expect("valid knot vector")
}

/// Dummy control payload: `span_terms` is generic over it.
#[derive(Debug)]
struct Ctl(usize);

fn check(kv: &KnotVector, t: f64) {
    let n = kv.control_count();
    let control: Vec<Ctl> = (0..n).map(Ctl).collect();
    let weights: Vec<f64> = (0..n).map(|i| 1.0 + i as f64 * 0.25).collect();
    for index in kv.first_span()..=kv.last_span() {
        let Some(span) = kv.span(index) else { continue };
        let want = basis_funs::<f64>(kv, index, t);
        let got = span_terms::<f64, Ctl>(kv, span, t, &control, &weights);
        assert_eq!(got.len(), want.len(), "length at span {index}");
        for (r, (term, w)) in got.iter().zip(&want).enumerate() {
            assert_eq!(
                term.basis.to_bits(),
                w.to_bits(),
                "basis bits differ at span {index}, r {r}, t {t}: {} vs {}",
                term.basis,
                w
            );
            // And the pairing is the window, in ascending order.
            let expected_control = index - kv.degree() + r;
            assert_eq!(term.control.0, expected_control, "control pairing");
            assert_eq!(term.weight, weights[expected_control], "weight pairing");
        }
    }
}

#[test]
fn matches_basis_funs_bit_for_bit_uniform() {
    let k = kv(
        vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0],
        3,
    );
    for step in 0..=40 {
        check(&k, step as f64 * 0.1);
    }
}

#[test]
fn matches_basis_funs_bit_for_bit_nonuniform_and_multiple_knots() {
    // Interior multiplicity 2 (degree 3) — empty spans present.
    let k = kv(
        vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.7, 2.0, 2.0, 2.0, 2.0],
        3,
    );
    for step in 0..=60 {
        check(&k, step as f64 * (2.0 / 60.0));
    }
    // Exactly on the knots, including the repeated one.
    for t in [0.0, 0.3, 1.7, 2.0] {
        check(&k, t);
    }
}

#[test]
fn matches_basis_funs_bit_for_bit_degree_one_and_high_degree() {
    check(&kv(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1), 1.5);
    let hi = kv(
        vec![0.0; 6]
            .into_iter()
            .chain([0.4, 0.9])
            .chain(vec![1.0; 6])
            .collect(),
        5,
    );
    for step in 0..=20 {
        check(&hi, step as f64 * 0.05);
    }
}

#[test]
fn span_window_is_the_control_window() {
    let k = kv(
        vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0],
        3,
    );
    let span: Span = k.span_at(1.5);
    assert_eq!(span.degree(), 3);
    assert_eq!(*span.window().start(), span.index() - 3);
    assert_eq!(span.window().count(), 4);
    // Empty spans are refused by the checked constructor.
    let m = kv(
        vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.7, 2.0, 2.0, 2.0, 2.0],
        3,
    );
    let empty = (m.first_span()..=m.last_span())
        .find(|i| !m.span_is_nonempty(*i))
        .expect("this vector has an empty span");
    assert!(m.span(empty).is_none(), "empty span must not construct");
}

/// The interval lane is where a stray `+ 0` would actually bite: an
/// added exact zero cannot widen a `DInterval` (TwoSum reports it
/// exact), but the decoration is a `min` and the enclosure is what the
/// kernel decides on, so "bit-identical" has to mean BOTH endpoints of
/// BOTH bounds — checked here rather than argued.
#[cfg(feature = "interval")]
mod interval_lane {
    use geom_core::spline::KnotVector;
    use geom_core::spline::basis::{basis_funs, span_terms};
    use geom_core::{Bounds, Interval, Real};

    struct Ctl(usize);

    fn check(kv: &KnotVector, t: f64) {
        let n = kv.control_count();
        let control: Vec<Ctl> = (0..n).map(Ctl).collect();
        let weights: Vec<f64> = (0..n).map(|i| 1.0 + i as f64 * 0.25).collect();
        let ti = Interval::from_f64(t);
        for index in kv.first_span()..=kv.last_span() {
            let Some(span) = kv.span(index) else { continue };
            let want = basis_funs::<Interval>(kv, index, ti);
            let got = span_terms::<Interval, Ctl>(kv, span, ti, &control, &weights);
            assert_eq!(got.len(), want.len());
            for (r, (term, w)) in got.iter().zip(&want).enumerate() {
                assert_eq!(
                    (term.basis.lo().to_bits(), term.basis.hi().to_bits()),
                    (w.lo().to_bits(), w.hi().to_bits()),
                    "enclosure bits differ at span {index}, r {r}, t {t}"
                );
                // The pairing is the window here too, in ascending order.
                assert_eq!(
                    term.control.0,
                    index - kv.degree() + r,
                    "control pairing at span {index}, r {r}"
                );
            }
        }
    }

    #[test]
    fn matches_basis_funs_bit_for_bit_on_the_interval_lane() {
        let k = KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0],
            3,
        )
        .expect("valid");
        for step in 0..=40 {
            check(&k, step as f64 * 0.1);
        }
        let m = KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.7, 2.0, 2.0, 2.0, 2.0],
            3,
        )
        .expect("valid");
        for t in [0.0, 0.3, 1.7, 2.0, 0.15, 1.0] {
            check(&m, t);
        }
    }
}

/// The high end of the shift-and-add (`n[j] = saved`, with NO addition)
/// is load-bearing, not defensive: writing `saved + 0` instead flips
/// `-0.0` to `+0.0`. Found by scanning, pinned here so the suite has
/// teeth on that line — `t = -0.0` and the tiny negatives are the
/// discriminating inputs, and they are cheap to keep.
#[test]
fn signed_zero_survives_the_shift_and_add() {
    let k = kv(
        vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0],
        3,
    );
    for t in [-0.0, -f64::MIN_POSITIVE, -1e-320, -5e-324, -1e-300] {
        check(&k, t);
    }
    // The exact witness the scan reported, spelled out: at t = -0.0 the
    // reference produces a NEGATIVE zero in the last entry, and an
    // implementation that adds an exact zero there produces +0.0.
    let span = k.span(3).expect("span 3 is nonempty");
    let n = k.control_count();
    let control: Vec<Ctl> = (0..n).map(Ctl).collect();
    let weights = vec![1.0; n];
    let got = span_terms::<f64, Ctl>(&k, span, -0.0, &control, &weights);
    let want = basis_funs::<f64>(&k, 3, -0.0);
    assert!(
        want.iter().any(|v| v.is_sign_negative() && *v == 0.0),
        "the reference really does produce a negative zero here"
    );
    for (term, w) in got.iter().zip(&want) {
        assert_eq!(term.basis.to_bits(), w.to_bits(), "signed zero must match");
    }
}
