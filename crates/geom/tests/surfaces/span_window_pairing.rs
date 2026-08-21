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

// ---------------------------------------------------------------
// The foreign window: refused, not indexed
// ---------------------------------------------------------------
//
// A `SurfaceWindow` is not branded to its surface, so a window minted
// on surface A is a representable argument to surface B's evaluators.
// `NurbsSurface::admits` decides that pairing in three integer
// compares — both directions' `KnotVector::admits`, plus
// `stride == knots_v.control_count()` — and the three doors answer a
// window it refuses with all-poison rather than an out-of-bounds index
// into `control`/`weights` (D9: the kernel never panics on any input).
//
// Each row below is built so that ONE of the three compares is the
// only thing standing between it and a panic: delete that compare and
// the row goes red, delete either other and it stays green. That is
// the property a shared fixture list cannot be relied on to have —
// a guard whose suite exercises half of it passes while half-broken.

fn is_poison_point(p: Point3<f64>) -> bool {
    p.x.is_nan() && p.y.is_nan() && p.z.is_nan()
}

fn is_poison_vec(v: Vec3<f64>) -> bool {
    v.x.is_nan() && v.y.is_nan() && v.z.is_nan()
}

/// Every one of the three doors refuses `win` on `s`, and refuses it
/// with poison in **every** component — a partly-finite jet would be a
/// door that took the window after all.
fn all_three_doors_refuse(
    s: &NurbsSurface<f64>,
    win: geom::SurfaceWindow,
    u: f64,
    v: f64,
    why: &str,
) {
    assert!(!s.admits(win), "{why}: the window should not be admitted");
    assert!(
        is_poison_point(s.eval_in_span(win, u, v)),
        "{why}: eval_in_span must poison a foreign window"
    );
    let j = s.ders_in_span(win, u, v);
    assert!(
        is_poison_point(j.point)
            && is_poison_vec(j.du)
            && is_poison_vec(j.dv)
            && is_poison_vec(j.duu)
            && is_poison_vec(j.duv)
            && is_poison_vec(j.dvv),
        "{why}: ders_in_span must poison a foreign window in every component"
    );
    let j3 = s.ders3_in_span(win, u, v);
    assert!(
        is_poison_point(j3.jet.point)
            && is_poison_vec(j3.jet.du)
            && is_poison_vec(j3.jet.dv)
            && is_poison_vec(j3.jet.duu)
            && is_poison_vec(j3.jet.duv)
            && is_poison_vec(j3.jet.dvv)
            && is_poison_vec(j3.duuu)
            && is_poison_vec(j3.duuv)
            && is_poison_vec(j3.duvv)
            && is_poison_vec(j3.dvvv),
        "{why}: ders3_in_span must poison a foreign window in every component"
    );
}

/// The demonstrated defect, inverted. Two degree-2 surfaces, 4×4 and
/// 3×3 control nets, public constructors throughout: `big`'s window at
/// `(0.75, 0.75)` used to index `small`'s 9-element arrays at 9 and
/// panic. It is refused now.
///
/// `window_at` is TOTAL, so making `window_of` private does not reach
/// this: the window is minted through a public door and is simply
/// handed to the wrong surface.
#[test]
fn a_bigger_surfaces_window_is_refused_not_indexed() {
    let big = surface(
        clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2),
        clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2),
    );
    let small = surface(
        clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2),
        clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2),
    );
    assert_eq!(big.control_counts(), (4, 4));
    assert_eq!(small.control_counts(), (3, 3));

    let win = big.window_at(0.75, 0.75);
    // This pair is OVER-DETERMINED and the row says so rather than
    // implying otherwise: the span index is out of `small`'s range AND
    // the stride is 4 against `small`'s 3, so two of the three
    // compares refuse it independently. It is here because it is the
    // construction that was demonstrated to panic, not because it
    // isolates anything — `an_index_that_only_the_index_compare_
    // catches` below is the isolating row for the index compare.
    assert_eq!(win.span_u().index(), 3);
    assert_eq!(small.knots_u().last_span(), 2);
    assert_ne!(win.stride(), small.knots_v().control_count());
    all_three_doors_refuse(
        &small,
        win,
        0.5,
        0.5,
        "span index out of the shorter vector",
    );

    // And the surface that DID mint it still evaluates finitely — the
    // guard refuses foreign windows, not all of them.
    assert!(!is_poison_point(big.eval_in_span(win, 0.75, 0.75)));
}

/// Same control counts, different degree — so `index <= last_span()`
/// holds in both directions and only the DEGREE compare separates the
/// pair. A degree-1 span's window is 2 wide; the degree-2 surface
/// reads 3 basis terms off it and walks a row past the end.
#[test]
fn an_equal_sized_surface_of_another_degree_is_refused() {
    let linear = surface(
        clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1),
        clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1),
    );
    let quadratic = surface(
        clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2),
        clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2),
    );
    assert_eq!(linear.control_counts(), quadratic.control_counts());
    assert_eq!(
        linear.knots_u().last_span(),
        quadratic.knots_u().last_span(),
        "the index compare must not be what separates this pair"
    );

    let win = linear.window(3, 3).expect("nonempty span pair");
    assert_eq!(win.stride(), quadratic.knots_v().control_count());
    all_three_doors_refuse(&quadratic, win, 1.5, 1.5, "degree disagreement");
}

/// Both directions admit and only the STRIDE compare separates the
/// pair — the term with no one-dimensional analogue. A 4×4 net's
/// window is handed to a 4×2 net: `span_u.index() = 3 <= 3` and
/// `span_v.index() = 1 <= 1`, degrees agree, and the stride of 4
/// carries `row(1)` to flat index 12 in an 8-element array.
#[test]
fn a_window_whose_spans_both_admit_is_still_refused_on_stride() {
    let wide = surface(
        clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1),
        clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1),
    );
    let narrow = surface(
        clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1),
        clamped(vec![0.0, 0.0, 1.0, 1.0], 1),
    );
    assert_eq!(wide.control_counts(), (4, 4));
    assert_eq!(narrow.control_counts(), (4, 2));

    let win = wide.window(3, 1).expect("nonempty span pair");
    // The two `KnotVector::admits` compares BOTH pass against
    // `narrow` — stated as an assertion, not as prose, because it is
    // the whole reason the third compare exists.
    assert!(narrow.knots_u().admits(win.span_u()));
    assert!(narrow.knots_v().admits(win.span_v()));
    assert_ne!(win.stride(), narrow.knots_v().control_count());
    all_three_doors_refuse(&narrow, win, 2.5, 0.5, "stride from a wider net");
}

/// Same degrees and the same **v** control count — so the stride
/// compare and both degree compares pass — and only `span_u.index() <=
/// last_span()` separates the pair. Without it, a window from the
/// 4×3 net puts `row(2) + 2` at flat index 11 in the 3×3 net's
/// 9-element arrays.
#[test]
fn an_index_that_only_the_index_compare_catches() {
    let shared_v = || clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2);
    let tall = surface(
        clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2),
        shared_v(),
    );
    let short = surface(clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2), shared_v());
    assert_eq!(tall.control_counts(), (4, 3));
    assert_eq!(short.control_counts(), (3, 3));

    let win = tall.window(3, 2).expect("nonempty span pair");
    // Everything except the u index compare passes against `short` —
    // asserted, because that is what makes this row isolating.
    assert_eq!(win.stride(), short.knots_v().control_count());
    assert_eq!(win.span_u().degree(), short.knots_u().degree());
    assert!(short.knots_v().admits(win.span_v()));
    assert!(win.span_u().index() > short.knots_u().last_span());
    all_three_doors_refuse(
        &short,
        win,
        0.75,
        0.5,
        "u span index past the shorter vector",
    );
}

/// The residue, pinned so it is a decision rather than an oversight:
/// two surfaces of equal degrees and equal control counts but
/// different interior knots admit each other's windows, and evaluation
/// is then a **wrong answer rather than a refusal**. Closing this wants
/// a brand, not a compare.
#[test]
fn equal_shape_different_knots_is_admitted_and_wrong() {
    let a = surface(
        clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2),
        clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2),
    );
    let b = surface(
        clamped(vec![0.0, 0.0, 0.0, 1.9, 2.0, 2.0, 2.0], 2),
        clamped(vec![0.0, 0.0, 0.0, 0.1, 2.0, 2.0, 2.0], 2),
    );
    let win = a.window_at(1.5, 1.5);
    assert!(b.admits(win), "the compares relate shapes, not knot values");
    let p = b.eval_in_span(win, 1.5, 1.5);
    assert!(
        !is_poison_point(p),
        "admitted: a finite answer, not a refusal"
    );
    let q = b.eval_in_span(b.window_at(1.5, 1.5), 1.5, 1.5);
    assert!(
        pbits(p) != pbits(q),
        "and the wrong one — the residue this check does not close"
    );
}
