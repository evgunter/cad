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

    // Exactly one of the three conditions separates this pairing: the
    // degrees agree, the index does not fit. (`short` has one span, so
    // emptiness has nothing to say either.)
    assert_eq!(span.degree(), short.degree());
    assert!(span.index() > short.last_span());

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

    // LAST, deliberately. This one is an assertion about the guard, so
    // deleting the compare reds it tautologically and it is evidence of
    // nothing downstream. The four calls above are this row's evidence:
    // they are what panicked before the guard existed, and they are
    // what must red if it stops working.
    assert!(!curve.knots().admits(span));
}

/// A span of a **lower degree** whose index is in range for the curve
/// AND names a nonempty span of it — so the other two conditions both
/// pass and only the degree compare can refuse.
///
/// Getting that combination right took a correction worth recording:
/// the obvious choice, a degree-1 span at index 1 against a cubic
/// Bezier, is *also* an empty span of that cubic (`knots[1] == knots[2]`
/// in the leading clamp), so it was separated by two conditions at once
/// and was never degree-compare-only evidence. A clamped Bezier's only
/// nonempty span is its last, which is what forces the shape below.
///
/// Without the degree compare, `base = index − degree` is larger than
/// the window this curve's basis row indexes from, and
/// `control[base + p]` runs one past the end.
#[test]
fn a_lower_degree_span_is_refused_although_its_index_and_span_are_valid() {
    // Degree 2, four control points: span 3 is [0.5, 1), nonempty.
    let quad = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).expect("valid");
    let span = quad.span(3).expect("span 3 is valid for the quadratic");

    // Degree 3 Bezier: four control points, last span 3, nonempty.
    let cubic =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).expect("valid");
    assert_eq!(quad.control_count(), cubic.control_count());
    assert!(
        span.index() <= cubic.last_span(),
        "only evidence about the degree compare if the index compare passes"
    );
    assert!(
        cubic.span_is_nonempty(span.index()),
        "only evidence about the degree compare if the emptiness compare passes"
    );
    assert_ne!(span.degree(), cubic.degree());

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

    // LAST, for the reason given in the row above: this is an assertion
    // about the guard, not about what the guard prevents.
    assert!(!curve.knots().admits(span));
}

/// A foreign span of the **same degree and the same control count**
/// whose index names an EMPTY span of this curve — so the degree and
/// index compares both pass and only the emptiness compare refuses.
///
/// The curve door's failure here is not an out-of-bounds read: the
/// window is in range, and the span's zero knot gap divides in
/// `basis_funs`, which at `f64` gives a row of `±inf` and `NaN` and a
/// point built from it. That is a wrong number rather than a panic,
/// which is exactly why it is worth a compare — `geom-core`'s
/// `sup_norm_bound_span` is the exit where a wrong number certifies.
#[test]
fn a_span_empty_in_this_curve_is_refused_although_it_is_in_range() {
    let uni = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3)
        .expect("valid");
    let mult = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3)
        .expect("valid");
    let empty = (mult.first_span()..=mult.last_span())
        .find(|i| mult.span(*i).is_none() && uni.span(*i).is_some())
        .expect("the two cubics must disagree about some index");
    let span = uni.span(empty).expect("nonempty in the uniform cubic");

    assert_eq!(span.degree(), mult.degree());
    assert!(span.index() <= mult.last_span());
    assert_eq!(uni.control_count(), mult.control_count());

    // NOT the doubled knot value itself. At `t == knots[empty]` every
    // term of the recursion is `0 * inf`, so the row comes out all-NaN
    // with or without the compare and the assertion below would be
    // satisfied by the bug it is meant to catch. Off the knot, the
    // zero denominator survives as a signed infinity.
    let t = 0.3;
    let c = curve(mult.knots().to_vec(), mult.degree());
    let p = c.eval_in_span(span, t);
    assert!(
        p.x.is_nan() && p.y.is_nan() && p.z.is_nan(),
        "an empty span must poison, not answer: {p:?}"
    );
    assert!(c.ders_in_span(span, t).0.x.is_nan());

    // The two assertions above do NOT discriminate, and saying so is
    // the point: at this door an empty span yields NaN coordinates with
    // or without the compare, because the zero knot gap divides and the
    // poison reaches the coordinates through the weight accumulator.
    // The basis row this point is built from is where the difference
    // is visible — unguarded it is `[-inf, NaN, NaN, inf]`.
    let row = geom_core::spline::basis::basis_funs::<f64>(c.knots(), span, t);
    assert!(
        row.iter().all(|n| n.is_nan()),
        "the empty span's basis row is live, not poison: {row:?}"
    );

    // LAST, for the reason given two rows above.
    assert!(!c.knots().admits(span));
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
/// admitted, and one per compare `admits` makes — because a class that
/// reaches zero means the family stopped exercising that compare, and
/// the row would be green over a guard part of which it never drove.
/// Classified by the FIRST condition that fails, which is the only
/// partition that attributes a refusal to one compare.
#[test]
fn no_cross_vector_pairing_panics_and_every_outcome_occurs() {
    let vectors: Vec<KnotVector> = span_families()
        .into_iter()
        .map(|(knots, p)| KnotVector::clamped(knots, p).expect("valid"))
        .collect();
    let (mut admitted, mut by_degree, mut by_index, mut by_empty) =
        (0usize, 0usize, 0usize, 0usize);
    for source in &vectors {
        for index in source.first_span()..=source.last_span() {
            let Some(span) = source.span(index) else {
                continue;
            };
            for target in &vectors {
                let c = curve(target.knots().to_vec(), target.degree());
                let pt = c.eval_in_span(span, 0.5);
                let (_, d1, _) = c.ders_in_span(span, 0.5);
                // The contract, written independently of the guard:
                // what MUST be refused. Behaviour is asserted against
                // THIS, not against `admits` — gating on `admits` would
                // reclassify when a compare is deleted and assert
                // nothing.
                let refuse = span.degree() != target.degree()
                    || span.index() > target.last_span()
                    || !target.span_is_nonempty(span.index());
                assert_eq!(
                    target.admits(span),
                    !refuse,
                    "`admits` disagrees with the span contract"
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
                    assert!(
                        pt.x.is_nan() && d1.x.is_nan(),
                        "a refused pairing answered instead of poisoning"
                    );
                    // The discriminating one. At the curve door an
                    // EMPTY span already yields NaN coordinates without
                    // any guard (the zero knot gap divides, and the
                    // poison reaches the coordinates through `w_acc`),
                    // so the assertion above cannot separate that class.
                    // The basis row can: unguarded it is
                    // `[-inf, NaN, .., inf]`, and `-inf` is not NaN.
                    // Off any knot value, so a zero denominator shows
                    // up as a signed infinity rather than as `0 * inf`
                    // — see the note in the empty-span row above.
                    let row = geom_core::spline::basis::basis_funs::<f64>(target, span, 0.3);
                    assert!(
                        row.iter().all(|n| n.is_nan()),
                        "a refused pairing produced a live basis row: {row:?}"
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
        assert!(n > 0, "the family no longer produces any pairing {class}");
    }
}

/// The knot vectors the two sweeps above range over: several degrees,
/// several lengths, clamped and interior-knotted.
///
/// **Two entries are here for one compare each and nothing else**, and
/// they are marked so a later tidy-up does not drop them: the
/// same-degree pairs (`1`/`1`, `2`/`2`, `3`/`3`) of *different length*
/// are the only shape the index compare can separate on its own, and
/// the multiplicity-2 cubic is the only one that gives another cubic
/// of the same length an EMPTY span to be refused at. Without them the
/// sweep above stays green with those compares deleted — which is how
/// it behaved before they were added.
fn span_families() -> Vec<(Vec<f64>, usize)> {
    vec![
        (vec![0.0, 0.0, 1.0, 1.0], 1),
        (vec![0.0, 0.0, 0.5, 1.0, 1.0], 1),
        (vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2),
        (vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2),
        (vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3),
        (vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3),
        // Same degree and same length as the row above, but `0.5` is
        // doubled: span 5 is EMPTY here and nonempty there, so each
        // admits indices the other refuses on emptiness alone.
        (vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3),
        (
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            ],
            5,
        ),
    ]
}
