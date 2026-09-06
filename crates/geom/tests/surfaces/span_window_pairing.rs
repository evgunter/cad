//! The surface evaluators must read **exactly** the control points
//! their [`SurfaceWindow`] names — no more (which would make the result
//! depend on a control point the span pair cannot see) and no fewer
//! (which is what a truncating `zip` against a mis-sized basis row
//! would do). This is `tests/curves/span_window_pairing.rs` one
//! dimension up, and it additionally pins the **stride**: the tensor
//! window's extra failure mode is a row-major `iu·nv + iv` that walks
//! the wrong row.
//!
//! Tested behaviourally rather than argued: at a span pair's midpoint
//! every one of the `(pu + 1)·(pv + 1)` basis PRODUCTS is strictly
//! positive, so perturbing a control point moves the result **iff**
//! that point is in the tensor window. A dropped trailing term, a
//! borrowed extra term and a wrong stride all show up here as an exact
//! equality that should have been an inequality, or the reverse.
//!
//! The expected membership is computed from the two [`Span`]s alone
//! (`span_u.window()` × `span_v.window()`), never from the window's
//! `base`/`stride` — otherwise a stride bug would be compared against
//! itself.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

/// A surface on the two given knot vectors with deliberately irregular
/// control points and weights (nothing symmetric that could make a
/// dropped term cancel).
fn surface(ku: KnotVector, kv: KnotVector) -> NurbsSurface<f64> {
    let (nu, nv) = (ku.control_count(), kv.control_count());
    let mut control = Vec::with_capacity(nu * nv);
    let mut weights = Vec::with_capacity(nu * nv);
    for iu in 0..nu {
        for iv in 0..nv {
            #[allow(clippy::cast_precision_loss)]
            let (a, b) = (iu as f64, iv as f64);
            control.push(Point3::new(
                a * 0.9 + b * 0.05,
                b * 0.7 - a * 0.03,
                0.31 * a * b - 0.17 * a * a + 0.23 * b,
            ));
            #[allow(clippy::cast_precision_loss)]
            weights.push(0.55 + 0.3 * ((iu * 5 + iv * 3) % 7) as f64);
        }
    }
    NurbsSurface::new(ku, kv, control, weights).expect("valid surface")
}

/// `base` with the flat-index `moved` control point displaced.
fn perturbed(base: &NurbsSurface<f64>, moved: usize) -> NurbsSurface<f64> {
    let mut control = base.control().to_vec();
    control[moved] = control[moved] + Vec3::new(1.0, -2.0, 3.0);
    NurbsSurface::new(
        base.knots_u().clone(),
        base.knots_v().clone(),
        control,
        base.weights().to_vec(),
    )
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

/// Every component of the second-order jet, as bits.
fn jbits(win: geom::SurfaceWindow<'_, f64>, u: f64, v: f64) -> [(u64, u64, u64); 6] {
    let j = win.ders_in_span(u, v);
    [
        pbits(j.point),
        vbits(j.du),
        vbits(j.dv),
        vbits(j.duu),
        vbits(j.duv),
        vbits(j.dvv),
    ]
}

const COMPONENTS: [&str; 6] = ["point", "du", "dv", "duu", "duv", "dvv"];

fn check(ku: KnotVector, kv: KnotVector) {
    let base = surface(ku.clone(), kv.clone());
    let (nu, nv) = base.control_counts();
    for iu in ku.first_span()..=ku.last_span() {
        for iv in kv.first_span()..=kv.last_span() {
            let Some(win) = base.window(iu, iv) else {
                continue;
            };
            let u = 0.5 * (ku.knots()[iu] + ku.knots()[iu + 1]);
            let v = 0.5 * (kv.knots()[iv] + kv.knots()[iv + 1]);
            let e0 = pbits(win.eval_in_span(u, v));
            let j0 = jbits(win, u, v);
            for a in 0..nu {
                for b in 0..nv {
                    // Membership from the two SPANS — independent of
                    // the window's own base/stride, which is what this
                    // row is testing.
                    let in_window =
                        win.span_u().window().contains(&a) && win.span_v().window().contains(&b);
                    let moved = a * nv + b;
                    let moved_surface = perturbed(&base, moved);
                    // The window is minted afresh on the perturbed
                    // surface: it borrows the surface it evaluates, so
                    // "the same window on a different net" has no
                    // spelling. Its SHAPE is a fact about the knot
                    // vectors and the control counts, both unchanged,
                    // so the two windows name the same indices.
                    let moved_win = moved_surface
                        .window(iu, iv)
                        .expect("the perturbation keeps the knot vectors");
                    let e1 = pbits(moved_win.eval_in_span(u, v));
                    assert_eq!(
                        e1 != e0,
                        in_window,
                        "eval_in_span at span pair ({iu}, {iv}): control ({a}, {b}) \
                         {} the window {:?}×{:?} but {} the result",
                        if in_window { "is in" } else { "is outside" },
                        win.span_u().window(),
                        win.span_v().window(),
                        if in_window { "did not move" } else { "moved" },
                    );
                    let j1 = jbits(moved_win, u, v);
                    for (k, name) in COMPONENTS.iter().enumerate() {
                        assert_eq!(
                            j1[k] != j0[k],
                            in_window,
                            "ders_in_span {name} at span pair ({iu}, {iv}): \
                             control ({a}, {b}), in_window = {in_window}"
                        );
                    }
                }
            }
        }
    }
}

fn clamped(knots: Vec<f64>, degree: usize) -> KnotVector {
    KnotVector::clamped(knots, degree).expect("valid knot vector")
}

#[test]
fn bicubic_reads_exactly_its_tensor_window() {
    check(
        clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0, 3.0], 3),
        clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.5, 2.0, 2.0, 2.0, 2.0], 3),
    );
}

/// Different degrees per direction — the case a stride bug survives
/// when `nu == nv` and the two windows have the same length.
#[test]
fn mixed_degrees_read_exactly_their_tensor_window() {
    check(
        clamped(vec![0.0, 0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0, 1.0], 3),
        clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2),
    );
    check(
        clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1),
        clamped(
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.3, 1.0, 1.0, 1.0, 1.0, 1.0],
            4,
        ),
    );
}

/// Interior knot multiplicity in each direction: the empty spans have
/// no window at all, and the nonempty ones still read exactly theirs.
#[test]
fn interior_multiplicity_reads_exactly_its_tensor_window() {
    check(
        clamped(vec![0.0, 0.0, 0.0, 0.3, 0.3, 1.0, 1.0, 1.0], 2),
        clamped(
            vec![0.0, 0.0, 0.0, 0.0, 0.6, 0.6, 1.7, 2.0, 2.0, 2.0, 2.0],
            3,
        ),
    );
}

// ---------------------------------------------------------------
// The foreign window: unrepresentable, not refused
// ---------------------------------------------------------------
//
// A `SurfaceWindow` borrows the surface it was minted from, and the
// three evaluators live on the window and read that surface. A window
// minted on surface A therefore evaluates surface A wherever it goes;
// "surface B's evaluator applied to A's window" has no spelling,
// because no door takes a surface beside a window. The rows that drove
// the old three-compare refusal — a bigger surface's window, an
// equal-sized one of another degree, a mismatched stride, an index
// past the target, an empty target span — each named a state that
// cannot be built now, and are `compile_fail` doctests on `Span` in
// `geom_core::spline::knots` instead.
//
// What the rows below pin is what replaced them: the window answers
// for its own surface, both knot vectors and the stride come out of
// that one borrow, and the residue one level down — a control net
// related to its knot vectors by COUNT alone — is still open.

fn is_poison_point(p: Point3<f64>) -> bool {
    p.x.is_nan() && p.y.is_nan() && p.z.is_nan()
}

/// **A window answers for the surface it borrows.** Two surfaces of
/// equal degrees and equal control counts but different interior knots
/// and different control nets: each window's answer is its own
/// surface's, and the two differ — so a window is not a shape token
/// that any surface of the right shape could interpret.
///
/// Under the retired guard this pairing was *admitted and wrong*: the
/// compares related a window to a surface's shape, never to its knot
/// values, so `b.eval_in_span(a.window_at(..), ..)` answered finitely
/// with `a`'s spans over `b`'s net. That expression no longer exists.
#[test]
fn a_window_answers_for_its_own_surface() {
    let a = surface(
        clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2),
        clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2),
    );
    let b = surface(
        clamped(vec![0.0, 0.0, 0.0, 1.9, 2.0, 2.0, 2.0], 2),
        clamped(vec![0.0, 0.0, 0.0, 0.1, 2.0, 2.0, 2.0], 2),
    );
    assert_eq!(a.control_counts(), b.control_counts());

    let wa = a.window_at(1.5, 1.5);
    let wb = b.window_at(1.5, 1.5);
    let (pa, pb) = (wa.eval_in_span(1.5, 1.5), wb.eval_in_span(1.5, 1.5));
    assert!(!is_poison_point(pa) && !is_poison_point(pb));
    assert_ne!(
        pbits(pa),
        pbits(pb),
        "the two surfaces must disagree here, or this row proves nothing"
    );
    // Each window's answer is the whole-surface door's at the same
    // parameters — the door that locates its own span pair.
    assert_eq!(pbits(pa), pbits(a.eval(1.5, 1.5)));
    assert_eq!(pbits(pb), pbits(b.eval(1.5, 1.5)));
    // And the borrow is what says so: the window names its surface —
    // through BOTH public mints, which is the whole of the surface
    // half's claim.
    assert!(core::ptr::eq(wa.surface(), &a));
    assert!(core::ptr::eq(wb.surface(), &b));
    let indexed = a.window(2, 2).expect("nonempty span pair");
    assert!(core::ptr::eq(indexed.surface(), &a));
    assert!(core::ptr::eq(indexed.span_u().knots(), a.knots_u()));
    assert!(core::ptr::eq(indexed.span_v().knots(), a.knots_v()));
}

/// **Both knot vectors and the stride come from the one borrow.** The
/// stride was the term with no one-dimensional analogue and the one a
/// span pair alone could not fix: a window whose spans both fit but
/// whose stride came from a wider net walked past the end of a shorter
/// row. It is now `knots_v.control_count()` of the window's own
/// surface, so a surface of a different v control count simply has a
/// different window.
#[test]
fn the_stride_and_both_vectors_come_from_the_windows_surface() {
    let wide = surface(
        clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2),
        clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.5, 2.0, 2.0, 2.0], 2),
    );
    let narrow = surface(
        clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2),
        clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2),
    );
    let (ww, wn) = (wide.window_at(1.5, 1.5), narrow.window_at(1.5, 1.5));
    assert_ne!(ww.stride(), wn.stride());
    assert_eq!(ww.stride(), wide.control_counts().1);
    assert_eq!(wn.stride(), narrow.control_counts().1);
    assert!(core::ptr::eq(ww.span_u().knots(), wide.knots_u()));
    assert!(core::ptr::eq(ww.span_v().knots(), wide.knots_v()));
    // Both evaluate finitely, each on its own net; nothing indexes out
    // of the other's row.
    assert!(!is_poison_point(ww.eval_in_span(1.5, 1.5)));
    assert!(!is_poison_point(wn.eval_in_span(1.5, 1.5)));
}

/// **Where the count-only relation lives, and what it does and does
/// not refuse** — `NurbsSurface::new`, once, at construction.
///
/// The window doors read their net through the borrow, so no
/// evaluation door has this relation any more; the constructor does.
/// This row is falsifiable in both directions: it reds if `new` ever
/// starts *refusing* a net of matching counts (the pairing closed) and
/// it reds if `new` ever stops refusing a net of the wrong counts (the
/// bound that keeps every window in range).
#[test]
fn the_constructor_relates_the_net_to_the_vectors_by_count_alone() {
    let ku = clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2);
    let kv = clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2);
    let mine = surface(ku.clone(), kv.clone());
    let (nu, nv) = mine.control_counts();
    assert_eq!(mine.control().len(), nu * nv);

    // A net that is not this surface's, at the same counts: accepted.
    let mut theirs = mine.control().to_vec();
    theirs[0] = Point3::new(1.0e6, -1.0e6, 1.0e6);
    let hybrid = NurbsSurface::new(ku.clone(), kv.clone(), theirs, mine.weights().to_vec())
        .expect("the count relation is all `new` checks");
    assert_ne!(
        pbits(hybrid.window_at(0.25, 0.25).eval_in_span(0.25, 0.25)),
        pbits(mine.window_at(0.25, 0.25).eval_in_span(0.25, 0.25)),
        "the two nets must disagree, or this row proves nothing"
    );

    // A net of the wrong count: refused — and that refusal is what
    // makes every window's `row(i) + j` a construction fact.
    let short = mine.control()[..nu * nv - 1].to_vec();
    let short_w = mine.weights()[..nu * nv - 1].to_vec();
    assert!(NurbsSurface::new(ku.clone(), kv.clone(), short.clone(), short_w.clone()).is_err());
    // And a net whose length matches but whose WEIGHTS do not.
    assert!(NurbsSurface::new(ku, kv, mine.control().to_vec(), short_w).is_err());
}
