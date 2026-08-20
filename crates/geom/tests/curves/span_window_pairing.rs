//! The span evaluators must read **exactly** the control points their
//! `Span`'s window names — no more (which would make the result depend
//! on a control point the span cannot see) and no fewer (which is what
//! a truncating `zip` against a mis-sized basis row would do).
//!
//! Tested behaviourally rather than argued: at a span's midpoint every
//! one of the `p + 1` nonvanishing basis functions is strictly
//! positive, so perturbing a control point moves the result **iff**
//! that point is in the window. A dropped trailing term and a
//! borrowed extra term both show up here as an exact equality that
//! should have been an inequality, or the reverse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn curve(knots: Vec<f64>, degree: usize) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(knots, degree).expect("valid knot vector");
    let n = kv.control_count();
    #[allow(clippy::cast_precision_loss)]
    let control: Vec<Point3<f64>> = (0..n)
        .map(|i| Point3::new(i as f64, (i * i % 7) as f64, (i % 3) as f64))
        .collect();
    #[allow(clippy::cast_precision_loss)]
    let weights: Vec<f64> = (0..n).map(|i| 1.0 + i as f64 * 0.25).collect();
    NurbsCurve3::new(kv, control, weights).expect("valid curve")
}

/// `curve` with control point `moved` displaced.
fn perturbed(base: &NurbsCurve3<f64>, moved: usize) -> NurbsCurve3<f64> {
    let mut control = base.control().to_vec();
    control[moved] = control[moved] + Vec3::new(1.0, -2.0, 3.0);
    NurbsCurve3::new(base.knots().clone(), control, base.weights().to_vec())
        .expect("perturbation keeps the invariants")
}

/// Point/vector types carry no `PartialEq` (Q1: `Real` is
/// comparison-free), so "moved" is decided on the coordinate bits.
fn pbits(p: Point3<f64>) -> (u64, u64, u64) {
    (p.x.to_bits(), p.y.to_bits(), p.z.to_bits())
}

fn vbits(v: Vec3<f64>) -> (u64, u64, u64) {
    (v.x.to_bits(), v.y.to_bits(), v.z.to_bits())
}

fn check(knots: Vec<f64>, degree: usize) {
    let base = curve(knots, degree);
    let kv = base.knots().clone();
    for index in kv.first_span()..=kv.last_span() {
        let Some(span) = kv.span(index) else { continue };
        let t = 0.5 * (kv.knots()[index] + kv.knots()[index + 1]);
        let (p0, d0, dd0) = base.ders_in_span(span, t);
        let (p0, d0, dd0) = (pbits(p0), vbits(d0), vbits(dd0));
        let e0 = pbits(base.eval_in_span(span, t));
        for moved in 0..kv.control_count() {
            let moved_curve = perturbed(&base, moved);
            let (p1, d1, dd1) = moved_curve.ders_in_span(span, t);
            let (p1, d1, dd1) = (pbits(p1), vbits(d1), vbits(dd1));
            let e1 = pbits(moved_curve.eval_in_span(span, t));
            let in_window = span.window().contains(&moved);
            assert_eq!(
                e1 != e0,
                in_window,
                "eval_in_span at span {index}, t {t}: control {moved} \
                 {} the window {:?} but {} the result",
                if in_window { "is in" } else { "is outside" },
                span.window(),
                if in_window { "did not move" } else { "moved" },
            );
            assert_eq!(
                p1 != p0,
                in_window,
                "ders_in_span point at span {index}, control {moved}"
            );
            assert_eq!(
                d1 != d0,
                in_window,
                "ders_in_span first derivative at span {index}, control {moved}"
            );
            assert_eq!(
                dd1 != dd0,
                in_window,
                "ders_in_span second derivative at span {index}, control {moved}"
            );
        }
    }
}

#[test]
fn uniform_cubic_reads_exactly_its_window() {
    check(
        vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0],
        3,
    );
}

#[test]
fn interior_multiplicity_reads_exactly_its_window() {
    check(
        vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.7, 2.0, 2.0, 2.0, 2.0],
        3,
    );
}

#[test]
fn degree_one_and_degree_five_read_exactly_their_windows() {
    check(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1);
    check(
        vec![0.0; 6]
            .into_iter()
            .chain([0.4, 0.9])
            .chain(vec![1.0; 6])
            .collect(),
        5,
    );
}

// ---------------------------------------------------------------------
// S14(a), half two: the pairing refusal, and the panic it replaces.
//
// A `Span` carries "in range and nonempty" for the vector it was drawn
// from, and no borrow of that vector — so a span of a DIFFERENT
// `KnotVector` is a representable input at every door that takes one.
// Handed to a curve built on a shorter vector, it used to index past
// the end of that curve's control array and panic, through public
// constructors only, in safe Rust, with no kernel bug in the trace:
// a violation of D9's "the kernel never panics on any input".
//
// `KnotVector::admits` is the refusal — two integer compares — and
// these rows are its evidence. Two of them are the two compares: the
// first fails only the index bound, the second only the degree
// agreement, so deleting either compare turns exactly one of them red.
// The third and fourth are the other direction, which is the failure
// mode a guard has when it cannot fail: that nothing correctly paired
// is refused, and that the refusal is actually reached.
//
// What is NOT closed, and no row here should be read as claiming it:
// two vectors of equal degree and equal control count but different
// interior knots admit each other's spans, and evaluation is then a
// wrong answer rather than a refusal. `admits` relates a span to a
// vector's SHAPE, never to its knot values.

/// A span drawn from a **longer** vector of the same degree names a
/// control point the shorter curve does not have. This is the row that
/// panicked: window base 1, basis row 3 long, `control[3]` of a
/// 3-element array. It must now poison.
#[test]
fn a_span_from_a_longer_vector_is_refused_by_a_shorter_curve() {
    // Degree 2, three control points, one span (index 2).
    let short = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("valid");
    // Degree 2, four control points, two spans (indices 2 and 3).
    let long = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).expect("valid");
    assert_eq!(short.degree(), long.degree());
    assert_eq!((short.control_count(), long.control_count()), (3, 4));

    // Public, checked constructor: `span` returns `Some` because span 3
    // IS in range and nonempty — for `long`.
    let span = long.span(3).expect("span 3 is valid for `long`");

    // Exactly one of the two compares separates this pairing: the
    // degrees agree, the index does not fit.
    assert_eq!(span.degree(), short.degree());
    assert!(span.index() > short.last_span());
    assert!(!short.admits(span));

    let curve = NurbsCurve3::<f64>::new(
        short,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
        ],
        vec![1.0, 1.0, 1.0],
    )
    .expect("valid curve");

    let p = curve.eval_in_span(span, 0.75);
    assert!(
        p.x.is_nan() && p.y.is_nan() && p.z.is_nan(),
        "a refused span must poison, not answer: {p:?}"
    );
    let (c, d1, d2) = curve.ders_in_span(span, 0.75);
    assert!(c.x.is_nan() && d1.x.is_nan() && d2.x.is_nan());
    assert!(curve.deriv_in_span(span, 0.75).x.is_nan());
    assert!(curve.deriv2_in_span(span, 0.75).x.is_nan());
}

/// A span of a **lower degree** whose index is comfortably inside the
/// curve's span range. The index bound alone admits it; the recursion
/// then reads `u[span + 1 + r − j]` at `j = degree`, which underflows
/// below zero. Only the degree compare separates this one.
#[test]
fn a_lower_degree_span_is_refused_although_its_index_is_in_range() {
    // Degree 1, two control points, one span — index 1.
    let linear = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("valid");
    let span = linear
        .span(1)
        .expect("span 1 is valid for the linear vector");

    // Degree 3 Bézier: four control points, last span 3.
    let cubic =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).expect("valid");
    assert!(
        span.index() <= cubic.last_span(),
        "this row is only evidence about the degree compare if the index compare passes"
    );
    assert_ne!(span.degree(), cubic.degree());
    assert!(!cubic.admits(span));

    let curve = NurbsCurve3::<f64>::new(
        cubic,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 2.0, 0.0),
            Point3::new(2.0, 2.0, 0.0),
            Point3::new(3.0, 0.0, 0.0),
        ],
        vec![1.0, 1.0, 1.0, 1.0],
    )
    .expect("valid curve");

    let p = curve.eval_in_span(span, 0.5);
    assert!(p.x.is_nan() && p.y.is_nan() && p.z.is_nan(), "{p:?}");
    assert!(curve.ders_in_span(span, 0.5).0.x.is_nan());
}

/// The other direction, which is how a guard fails when it cannot fail:
/// **every** span of a vector — by index and by location — is admitted
/// by that vector, at every degree the suite builds. A guard that
/// refused a correctly paired span would red here rather than in a
/// caller three crates away.
#[test]
fn a_vector_admits_every_span_of_its_own() {
    let mut checked = 0usize;
    for (knots, p) in span_families() {
        let kv = KnotVector::clamped(knots, p).expect("valid");
        for index in kv.first_span()..=kv.last_span() {
            if let Some(span) = kv.span(index) {
                assert!(kv.admits(span), "p{p} refused its own span {index}");
                checked += 1;
            }
        }
        let (lo, hi) = kv.domain();
        for t in [lo, hi, f64::NAN, lo - 1.0, hi + 1.0, 0.5 * (lo + hi)] {
            assert!(
                kv.admits(kv.span_at(t)),
                "p{p} refused its own located span at {t}"
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 40,
        "the family stopped covering spans: {checked}"
    );
}

/// The whole cross product of the family against itself: **no pairing
/// panics**, which is the D9 claim.
///
/// The three outcome classes are counted separately and each floored —
/// admitted, refused on the degree, refused on the index — because a
/// class that reaches zero means the family stopped exercising one of
/// the two compares, and the row would be green over a guard half of
/// which it never drove.
#[test]
fn no_cross_vector_pairing_panics_and_every_outcome_occurs() {
    let vectors: Vec<KnotVector> = span_families()
        .into_iter()
        .map(|(knots, p)| KnotVector::clamped(knots, p).expect("valid"))
        .collect();
    let (mut admitted, mut by_degree, mut by_index) = (0usize, 0usize, 0usize);
    for source in &vectors {
        for index in source.first_span()..=source.last_span() {
            let Some(span) = source.span(index) else {
                continue;
            };
            for target in &vectors {
                let c = curve(target.knots().to_vec(), target.degree());
                let pt = c.eval_in_span(span, 0.5);
                let (_, d1, _) = c.ders_in_span(span, 0.5);
                if target.admits(span) {
                    admitted += 1;
                } else {
                    if span.degree() == target.degree() {
                        by_index += 1;
                    } else {
                        by_degree += 1;
                    }
                    assert!(
                        pt.x.is_nan() && d1.x.is_nan(),
                        "a refused pairing answered instead of poisoning"
                    );
                }
            }
        }
    }
    for (class, n) in [
        ("admitted", admitted),
        ("refused on the degree compare", by_degree),
        ("refused on the index compare", by_index),
    ] {
        assert!(n > 0, "the family no longer produces any pairing {class}");
    }
}

/// The knot vectors the two sweeps above range over: several degrees,
/// several lengths, clamped and interior-knotted.
fn span_families() -> Vec<(Vec<f64>, usize)> {
    vec![
        (vec![0.0, 0.0, 1.0, 1.0], 1),
        (vec![0.0, 0.0, 0.5, 1.0, 1.0], 1),
        (vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2),
        (vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2),
        (vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3),
        (vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3),
        (
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            ],
            5,
        ),
    ]
}
