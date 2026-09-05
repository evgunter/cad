//! **The type-level claims of `SplineCoeffs` / `RationalCoeffs` and
//! their windows, as behaviour** — adopted from the dual review's type
//! probe. `Debug` prints addresses and never a coefficient, weight or
//! knot value; equality is address equality (a bit-equal copy of the
//! array is a different pair, a bit-equal vector at another address is
//! a different pair); the minimal degree-1 pair has exactly one
//! derivative coefficient and no degree's minimal vector underflows
//! `derivative_coeffs`'s `len − 1`; `span_at` on the totality cases
//! mints the window `KnotVector::span_at` names.
//!
//! What is NOT here, because it has no spelling: a rational claim on a
//! pair minted without weights, and a nonrational bound on a pair
//! minted with them — rows (d) and (e) on the types in
//! `spline/hull.rs`, `compile_fail` doctests with twins.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::RingInterval;
use geom_core::spline::KnotVector;

fn cubic() -> KnotVector {
    KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0],
        3,
    )
    .unwrap()
}

#[test]
fn debug_prints_addresses_and_no_value() {
    let k = cubic();
    let n = k.control_count();
    let c: Vec<f64> = (0..n).map(|_| 123_456.789).collect();
    let w: Vec<f64> = (0..n).map(|_| 987_654.321).collect();
    let pair = k.with_coeffs(&c).unwrap();
    let s = format!("{pair:?}");
    assert!(s.starts_with("SplineCoeffs"), "{s}");
    assert!(!s.contains("123456"), "{s}");
    assert!(s.contains("0x"), "addresses are hex: {s}");
    assert!(s.contains("len: 7"), "{s}");
    let win = pair.span_at(0.3);
    let s = format!("{win:?}");
    assert!(s.starts_with("CoeffWindow"), "{s}");
    assert!(!s.contains("123456"), "{s}");
    // And a knot value must not leak either.
    assert!(!s.contains("0.25"), "{s}");
    let rational = k.with_rational_coeffs(&c, &w).unwrap();
    let s = format!("{rational:?}");
    assert!(s.starts_with("RationalCoeffs"), "{s}");
    assert!(!s.contains("123456") && !s.contains("987654"), "{s}");
    assert!(s.contains("len: 7"), "{s}");
    let s = format!("{:?}", rational.span_at(0.3));
    assert!(s.starts_with("RationalWindow"), "{s}");
    assert!(
        !s.contains("123456") && !s.contains("987654") && !s.contains("0.25"),
        "{s}"
    );
}

#[test]
fn equality_is_address_equality() {
    let k = cubic();
    let n = k.control_count();
    #[allow(clippy::cast_precision_loss)]
    let c: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let c2 = c.clone();
    let w = vec![1.0; n];
    let w2 = w.clone();
    let a = k.with_coeffs(&c).unwrap();
    let b = k.with_coeffs(&c).unwrap();
    let other_array = k.with_coeffs(&c2).unwrap();
    assert_eq!(a, b);
    assert_ne!(
        a, other_array,
        "bit-equal arrays at another address are another pair"
    );
    assert_eq!(a.span_at(0.3), b.span_at(0.3));
    assert_ne!(a.span_at(0.3), a.span_at(0.7));
    assert_ne!(a.span_at(0.3), other_array.span_at(0.3));
    let k2 = cubic();
    let on_k2 = k2.with_coeffs(&c).unwrap();
    assert_ne!(
        a, on_k2,
        "a bit-equal vector at another address is another pair"
    );
    // The rational pair reads its weights' address too.
    let r = k.with_rational_coeffs(&c, &w).unwrap();
    assert_eq!(r, k.with_rational_coeffs(&c, &w).unwrap());
    assert_ne!(
        r,
        k.with_rational_coeffs(&c, &w2).unwrap(),
        "bit-equal weights elsewhere"
    );
    assert_ne!(r, k.with_rational_coeffs(&c2, &w).unwrap());
    assert_ne!(r, k2.with_rational_coeffs(&c, &w).unwrap());
    assert_eq!(r.span_at(0.3), r.span_at(0.3));
    assert_ne!(r.span_at(0.3), r.span_at(0.7));
}

#[test]
fn the_minimal_degree_one_pair_has_one_derivative_coefficient() {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    assert_eq!(k.control_count(), 2);
    let c = [2.0f64, 5.0];
    let pair = k.with_coeffs(&c).unwrap();
    let q = pair.derivative_coeffs();
    assert_eq!(q.len(), 1);
    // The ring widens every op by an ulp, so containment, not equality.
    let encloses = |r: RingInterval| r.lo() <= 3.0 && 3.0 <= r.hi() && r.hi() - r.lo() < 1e-12;
    assert!(encloses(q[0]), "{:?}", (q[0].lo(), q[0].hi()));
    assert!(encloses(pair.derivative_domain_hull()));
    let win = pair.span(1).unwrap();
    assert!(encloses(win.derivative_hull()));
    assert_eq!(win.window(), 0..=1);
    // Every constructor holds `control_count() >= degree + 1 >= 2`, so
    // the `len − 1` in `derivative_coeffs` cannot underflow: exhaustively
    // over the smallest vector of each degree up to 6, by both mints.
    for p in 1..=6 {
        let mut knots = vec![0.0; p + 1];
        knots.extend(vec![1.0; p + 1]);
        let k = KnotVector::clamped(knots, p).unwrap();
        assert_eq!(k.control_count(), p + 1);
        let c = vec![RingInterval::point(1.0); p + 1];
        assert_eq!(k.with_coeffs(&c).unwrap().derivative_coeffs().len(), p);
        assert_eq!(
            k.difference_coeffs(&c).len(),
            p,
            "the one-home helper agrees"
        );
        let short = k.difference_coeffs(&c[..p]);
        assert_eq!(
            short.len(),
            1,
            "a refused mint answers one poison entry, never zero"
        );
        assert!(short[0].is_poison());
    }
}

#[test]
fn span_at_is_total_exactly_as_the_vectors_is() {
    let k = cubic();
    let n = k.control_count();
    #[allow(clippy::cast_precision_loss)]
    let c: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let w = vec![1.0; n];
    let pair = k.with_coeffs(&c).unwrap();
    let rational = k.with_rational_coeffs(&c, &w).unwrap();
    for t in [
        f64::NAN,
        -1.0,
        0.0,
        0.5,
        1.0,
        2.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert_eq!(pair.span_at(t).span(), k.span_at(t), "t = {t}");
        // The rational window has no span accessor to read; its mint
        // agrees with the indexed one at the span the vector names.
        assert_eq!(
            rational.span_at(t),
            rational.span(k.span_at(t).index()).unwrap(),
            "t = {t}"
        );
    }
}
