//! The surface evaluators must read **exactly** the control points
//! their [`SurfaceWindow`] names — no more (which would make the result
//! depend on a control point the span pair cannot see) and no fewer
//! (which is what a truncating `zip` against a mis-sized basis row
//! would do). This is `geom-curves/tests/span_window_pairing.rs` one
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
fn jbits(s: &NurbsSurface<f64>, win: geom::SurfaceWindow, u: f64, v: f64) -> [(u64, u64, u64); 6] {
    let j = s.ders_in_span(win, u, v);
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
            let e0 = pbits(base.eval_in_span(win, u, v));
            let j0 = jbits(&base, win, u, v);
            for a in 0..nu {
                for b in 0..nv {
                    // Membership from the two SPANS — independent of
                    // the window's own base/stride, which is what this
                    // row is testing.
                    let in_window =
                        win.span_u().window().contains(&a) && win.span_v().window().contains(&b);
                    let moved = a * nv + b;
                    let moved_surface = perturbed(&base, moved);
                    // The window is a fact about the knot vectors and
                    // the control COUNTS, both unchanged.
                    let e1 = pbits(moved_surface.eval_in_span(win, u, v));
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
                    let j1 = jbits(&moved_surface, win, u, v);
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
