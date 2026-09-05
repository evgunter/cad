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
// One level down, the coefficients take the same shape: `geom-core`'s
// hull doors read a `SplineCoeffs` — an array with the vector it was
// minted against — through a `CoeffWindow` the pair minted, so a span
// of another vector beside a coefficient array has no spelling either.
// The three equal-count shapes the retired guard refused are
// `compile_fail` doctests on `SplineCoeffs` in `spline/hull.rs`, each
// with a legal twin; the row below is the curve-side half.

/// **A curve's own coordinate channels mint against its own vector, and
/// the windows they mint are the curve's windows.** `ring_coords()` and
/// `knots()` are read from one curve, so every channel mints (`Some`),
/// every window the pair mints selects the span the curve mints at that
/// index and refuses the indices the curve refuses, and the window's
/// hull is the channel's plain hull over that span's window — which is
/// what the SSI box chains and the chart tubes bank. A channel of a
/// curve of another length is refused at the mint, which is the only
/// relation a length can state.
#[test]
fn a_curves_channels_mint_against_its_own_vector() {
    let mut windows = 0usize;
    for (knots, degree) in span_families() {
        let c = curve(knots, degree);
        let k = c.knots();
        let coords = c.ring_coords();
        for (ch, coeffs) in coords.iter().enumerate() {
            let pair = k
                .with_coeffs(coeffs)
                .expect("a curve's channel is its own vector's length");
            assert_eq!(pair.knots() as *const _, k as *const _);
            for index in 0..k.knots().len() + 2 {
                match (pair.span(index), c.span(index)) {
                    (Some(win), Some(cw)) => {
                        assert_eq!(
                            win.span(),
                            cw.span(),
                            "degree {degree} channel {ch} index {index}"
                        );
                        let h = win.hull();
                        let (lo, hi) = coeffs[win.window()]
                            .iter()
                            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), r| {
                                (lo.min(r.lo()), hi.max(r.hi()))
                            });
                        assert_eq!((h.lo(), h.hi()), (lo, hi));
                        windows += 1;
                    }
                    (None, None) => {}
                    (a, b) => panic!(
                        "degree {degree} channel {ch} index {index}: the pair {} where the curve {}",
                        if a.is_some() { "mints" } else { "refuses" },
                        if b.is_some() { "mints" } else { "refuses" },
                    ),
                }
            }
        }
    }
    assert!(windows >= 20, "the families produced {windows} windows");

    // Another curve's channel, of another length: refused where the
    // retired relation would have been wrong instead.
    let mine = curve(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3);
    let longer = curve(
        vec![0.0, 0.0, 0.0, 0.0, 0.2, 0.4, 0.6, 1.0, 1.0, 1.0, 1.0],
        3,
    );
    let theirs = longer.ring_coords();
    assert!(mine.knots().with_coeffs(&theirs[0]).is_none());
    assert!(longer.knots().with_coeffs(&theirs[0]).is_some());
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
