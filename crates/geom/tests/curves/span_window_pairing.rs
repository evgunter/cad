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
        let Some(span) = base.span(index) else {
            continue;
        };
        let t = 0.5 * (kv.knots()[index] + kv.knots()[index + 1]);
        let (p0, d0, dd0) = span.ders_in_span(t);
        let (p0, d0, dd0) = (pbits(p0), vbits(d0), vbits(dd0));
        let e0 = pbits(span.eval_in_span(t));
        for moved in 0..kv.control_count() {
            let moved_curve = perturbed(&base, moved);
            // The window is minted afresh on the perturbed curve: it
            // borrows the curve it evaluates, so "the same window on a
            // different control polygon" has no spelling. Its SHAPE is
            // a fact about the knot vector alone, unchanged here, so
            // the two windows name the same indices.
            let moved_span = moved_curve
                .span(index)
                .expect("the perturbation keeps the knot vector");
            let (p1, d1, dd1) = moved_span.ders_in_span(t);
            let (p1, d1, dd1) = (pbits(p1), vbits(d1), vbits(dd1));
            let e1 = pbits(moved_span.eval_in_span(t));
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
// The pairing at the curve doors, and the one left open.
//
// Evaluation restricted to a span lives on `CurveWindow{2,3}`, which
// borrows the curve; the only mints are `NurbsCurve::span` and
// `::span_at`, both taking `&self`. So a window evaluates the curve it
// names and there is no door anywhere that takes a curve beside a span
// of some other curve's knots. That claim is about what COMPILES and
// is pinned where such claims belong — `compile_fail` doctests on the
// `curves::nurbs` module — not here, because a row here would have to
// write the expression it says cannot be written.
//
// What is left open, one level down, is the coefficients: `geom-core`'s
// FREE hull functions take a `&[E]` beside a `Span` and relate them by
// LENGTH alone. The rows below drive exactly that, at the doors where
// it is still reachable.

/// **The residue, at the only doors that still have it.**
/// `hull::span_hull` and its three siblings take loose coefficients
/// beside a span and check `coeffs.len() == kv.control_count()` and
/// nothing else, so a same-length array from another curve is answered
/// — finitely, and wrongly.
///
/// The curve doors are *not* in this row's scope and cannot be: they
/// read their coefficients from the curve the window borrows.
/// (`work/props/coefficients-carry-their-knot-vector.md`.)
#[test]
fn the_free_hull_doors_relate_coefficients_by_length_alone() {
    use geom_core::spline::hull;

    let mine = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3)
        .expect("valid");
    let n = mine.control_count();
    let span = mine.span(mine.first_span()).expect("nonempty");

    #[allow(clippy::cast_precision_loss)]
    let ours: Vec<f64> = (0..n).map(|i| (i % 5) as f64 * 0.5 - 1.0).collect();
    let mut theirs = ours.clone();
    theirs[0] = 1.0e6;

    let right = hull::span_hull(&ours, span);
    let wrong = hull::span_hull(&theirs, span);
    assert!(!right.is_poison() && !wrong.is_poison());
    assert_ne!(
        (right.lo(), right.hi()),
        (wrong.lo(), wrong.hi()),
        "the two arrays must disagree on this span, or the row proves nothing"
    );
    assert!(
        hull::sup_norm_bound_span(&theirs, span).is_finite(),
        "the honesty limb certifies a curve whose coefficients it never saw"
    );
    // A wrong LENGTH is still refused, which is what keeps the window
    // in range at every door here.
    assert!(hull::span_hull(&ours[..n - 1], span).is_poison());
    assert!(hull::derivative_span_hull(&ours[..n - 1], span).is_poison());
}

/// The other direction, which is how a guard fails when it cannot
/// fail: **every** window a curve mints evaluates finitely on that
/// curve, at every degree and every span the family produces, and the
/// window's answer is the whole-curve door's at the same parameter.
///
/// `span_families` is the spread; a family that stopped producing
/// spans would leave this row green over nothing, so the count is
/// floored.
#[test]
fn every_window_a_curve_mints_evaluates_that_curve() {
    let mut checked = 0usize;
    for (knots, p) in span_families() {
        let kv = KnotVector::clamped(knots, p).expect("valid");
        let c = curve(kv.knots().to_vec(), kv.degree());
        for index in kv.first_span()..=kv.last_span() {
            let Some(win) = c.span(index) else { continue };
            assert!(core::ptr::eq(win.curve(), &c));
            assert_eq!(win.index(), index);
            let t = 0.5 * (kv.knots()[index] + kv.knots()[index + 1]);
            let p0 = win.eval_in_span(t);
            assert!(
                p0.x.is_finite() && p0.y.is_finite() && p0.z.is_finite(),
                "degree {p}, span {index}: {p0:?}"
            );
            // The whole-curve door locates this very span at `t`.
            assert_eq!(pbits(p0), pbits(c.eval(t)));
            assert_eq!(vbits(win.deriv_in_span(t)), vbits(c.deriv(t)));
            checked += 1;
        }
        // The located mint agrees with the indexed one.
        let (lo, hi) = kv.domain();
        for t in [lo, hi, f64::NAN, lo - 1.0, hi + 1.0, 0.5 * (lo + hi)] {
            let w = c.span_at(t);
            assert!(core::ptr::eq(w.curve(), &c));
            assert_eq!(w.index(), kv.find_span(t));
            checked += 1;
        }
    }
    assert!(
        checked >= 40,
        "the family stopped covering spans: {checked}"
    );
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
