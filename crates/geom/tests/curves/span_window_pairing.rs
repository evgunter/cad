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
// S14 reachability demonstration — evidence for an open design question,
// NOT a specification.
//
// The `Span` docs, `basis`' module docs and `hull`'s module docs all say
// the same thing three ways: a `Span` is not branded to its knot vector,
// so pairing one with the wrong vector is the caller's obligation, and
// `hull`'s docs add that breaking it "can index out of bounds and
// **panic**, which is a worse failure than the poison D4 asks for".
//
// The defence offered for that (`SMELL-SCAN-2026-08.md` §S14, Evan,
// 2026-08-18) is that it is candour about a state only a kernel bug can
// reach. The test below is the counter-evidence, executed rather than
// argued: safe Rust, public constructors only, no kernel bug anywhere in
// the trace, and D9's ratified "the kernel never panics on any input" is
// violated by a caller who wrote no `unsafe` and called no private door.
//
// It is `#[should_panic]` deliberately: pinning today's behaviour is the
// only way a test can be evidence about it. It is NOT a promise that the
// panic stays — whichever way S14 is decided this test changes with it.
// Under a guard it becomes a typed refusal or a poison; under a
// structural fix it stops compiling, which is the outcome the fix is for.
// Do not "restore" it if it goes red; read the S14 decision instead.

/// Two clamped vectors, same degree, different length: the span drawn
/// from the longer one names a control point the shorter curve does not
/// have, and `eval_in_span` indexes straight past the end of its own
/// control array.
#[test]
#[should_panic(expected = "index out of bounds")]
fn a_span_from_a_longer_vector_indexes_past_a_shorter_curve() {
    // Degree 2, three control points, one span (index 2).
    let short = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("valid");
    // Degree 2, four control points, two spans (indices 2 and 3).
    let long = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).expect("valid");
    assert_eq!(short.degree(), long.degree());
    assert_eq!(short.control_count(), 3);
    assert_eq!(long.control_count(), 4);

    // The public, checked constructor. Nothing here is misuse of a
    // private door: `span` is `pub` on a `pub` type and returns `Some`
    // because the span IS valid — for `long`.
    let span = long
        .span(3)
        .expect("span 3 is in range and nonempty for `long`");

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

    // Window base is 1, basis row is `degree + 1 = 3` long, so this
    // reads control[1], control[2], control[3] — and the curve has 3.
    let _ = curve.eval_in_span(span, 0.75);
}

/// The same pairing one dimension up is worse than a panic: it is
/// silently wrong. `window_of` takes two `Span`s that carry no
/// direction, so a caller who swaps them gets a window that is in range
/// and names the wrong control points — no refusal, no panic, an answer.
/// Pinned as an inequality so the file records that the two windows
/// differ rather than asserting a particular wrong value.
#[test]
fn swapped_span_arguments_build_a_different_window_without_refusing() {
    use geom::NurbsSurface;

    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).expect("valid");
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).expect("valid");
    let (nu, nv) = (ku.control_count(), kv.control_count());
    #[allow(clippy::cast_precision_loss)]
    let control: Vec<Point3<f64>> = (0..nu * nv)
        .map(|i| Point3::new(i as f64, (i % 5) as f64, (i % 3) as f64))
        .collect();
    let surface = NurbsSurface::<f64>::new(ku, kv, control, vec![1.0; nu * nv]).expect("valid");

    let su = surface.knots_u().span(2).expect("valid u span");
    let sv = surface.knots_v().span(3).expect("valid v span");

    let right = surface.window_of(su, sv);
    // Typechecks. Nothing checks the order; nothing refuses.
    let wrong = surface.window_of(sv, su);
    assert_ne!(
        (right.span_u().index(), right.span_v().index()),
        (wrong.span_u().index(), wrong.span_v().index()),
        "the swapped call built the same window, so this row is no longer evidence"
    );
}
