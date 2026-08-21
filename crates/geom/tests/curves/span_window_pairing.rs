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
