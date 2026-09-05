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
// The pairing, and the panic that is now unwritable.
//
// A `Span` borrows the `KnotVector` it is a proof about, and the curve
// doors read the basis from THAT vector — never from `self.knots`
// beside it, and they take no knot vector of their own. So a span
// drawn from a longer vector cannot be handed to a shorter curve's
// evaluator *together with the shorter vector*: there is no second
// vector, and the old panic — window base 1, basis row 3 long,
// `control[3]` of a 3-element array — has no spelling. The rows that
// used to drive it are `compile_fail` doctests on `Span` in
// `geom_core::spline::knots` now.
//
// What is NOT closed is one level down, and these rows are its
// evidence: the curve's control points are related to the span's
// vector by COUNT alone, so a span from another vector of the same
// control count evaluates this curve's points on that vector's basis.
// Wrong rather than refused, and the rows below pin exactly that — in
// particular that the basis comes from the SPAN's vector, which is the
// claim the whole change rests on.

/// **The basis is read from the span's vector, not from the curve's.**
///
/// Two cubics with the same degree and the same control count but
/// different interior knots. A span of the second, handed to a curve
/// built on the first, must produce the value the SECOND vector's
/// basis gives against the first curve's control points — not the
/// value the first vector's own span of that index gives.
///
/// Computed independently of the door, from `basis_funs` on each
/// vector, so this row would still separate the two if the door
/// silently went back to reading `self.knots`.
#[test]
fn the_curve_door_reads_the_basis_from_the_spans_own_vector() {
    let mine = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3)
        .expect("valid");
    let theirs = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0], 3)
        .expect("valid");
    assert_eq!(mine.control_count(), theirs.control_count());

    let c = curve(mine.knots().to_vec(), mine.degree());
    let index = 4;
    let t = 0.6;
    let own = mine.span(index).expect("nonempty in `mine`");
    let foreign = theirs.span(index).expect("nonempty in `theirs`");

    // The oracle: the rational combination of THIS curve's control
    // points against the given vector's basis row, written out here so
    // the row does not test the door against itself.
    let oracle = |span: geom_core::spline::Span<'_>| {
        let n = geom_core::spline::basis::basis_funs::<f64>(span, t);
        let first = span.first_control();
        let (mut num, mut den) = (Point3::new(0.0, 0.0, 0.0), 0.0);
        for (j, nj) in n.iter().enumerate() {
            let i = first + j;
            let cw = nj * c.weights()[i];
            let p = c.control()[i];
            num = Point3::new(num.x + cw * p.x, num.y + cw * p.y, num.z + cw * p.z);
            den += cw;
        }
        (num.x / den, num.y / den, num.z / den)
    };

    let with_own = pbits(c.eval_in_span(own, t));
    let with_foreign = pbits(c.eval_in_span(foreign, t));
    let (ox, oy, oz) = oracle(own);
    let (fx, fy, fz) = oracle(foreign);
    assert_eq!(with_own, (ox.to_bits(), oy.to_bits(), oz.to_bits()));
    assert_eq!(
        with_foreign,
        (fx.to_bits(), fy.to_bits(), fz.to_bits()),
        "the door read a basis the span did not name"
    );
    assert_ne!(
        with_own, with_foreign,
        "the two vectors must disagree at this index, or the row proves nothing"
    );
}

/// The residue, stated as behaviour: a span from another curve's
/// vector of the same control count is **answered, not refused** —
/// with that vector's basis over this curve's control points.
///
/// Nothing panics anywhere in the cross product, which is D9's claim
/// and the reason the retired guard existed: every span names a window
/// inside its own vector's control count, so a curve of the same
/// control count is indexed in range whatever span it is given.
#[test]
fn no_cross_vector_pairing_of_equal_control_count_panics() {
    let vectors: Vec<KnotVector> = span_families()
        .into_iter()
        .map(|(knots, p)| KnotVector::clamped(knots, p).expect("valid"))
        .collect();
    let (mut same_answer, mut different_answer) = (0usize, 0usize);
    for source in &vectors {
        for index in source.first_span()..=source.last_span() {
            let Some(span) = source.span(index) else {
                continue;
            };
            for target in &vectors {
                if target.control_count() != source.control_count() {
                    // The count relation is the one guard left, and it
                    // lives at construction: a curve simply has no
                    // control point at the window this span names.
                    continue;
                }
                let c = curve(target.knots().to_vec(), target.degree());
                if span.index() >= c.control().len() {
                    continue;
                }
                let pt = pbits(c.eval_in_span(span, 0.3));
                let own = target
                    .span(index)
                    .map(|s| pbits(c.eval_in_span(s, 0.3)))
                    .unwrap_or(pt);
                if pt == own {
                    same_answer += 1;
                } else {
                    different_answer += 1;
                }
            }
        }
    }
    assert!(
        different_answer > 0,
        "the family stopped producing a cross-vector disagreement, so this \
         row no longer exercises the residue"
    );
    assert!(same_answer > 0, "the family produced no agreeing pairing");
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
