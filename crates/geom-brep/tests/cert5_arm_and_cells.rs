//! **The two things the rational lane's shipped rows could not see**
//! (CERT-5 review pass): whether the cells are really knot-aligned,
//! and whether the `w`-uniform-in-v exact arm is really the arm being
//! taken.
//!
//! Both gaps were found the same way — by planting the mutation and
//! watching nothing go red — so both rows here are written against a
//! planted mutation rather than against a number.
//!
//! # Why the body-level rows cannot do this
//!
//! `cert5_offgrid_knot_rational.rs` drives whole bodies, and a body's
//! `volume_pad` is dominated by the AREA rule's pad (~9.8e-7 on those
//! blades). The flux remainder the knot alignment governs lives in the
//! seventh digit of that number, so dropping the alignment moves the
//! pad far too little to cross any threshold those rows assert. These
//! rows drive the patch door directly and check the bracket against an
//! independent oracle, where a wrong rule shows up as a bracket that
//! EXCLUDES the truth rather than as a slightly different width.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::RingInterval;
use geom_core::spline::KnotVector;

use crate::shared::patch::{face_posture, oracle_patch};
use crate::shared::ring::pt;

/// Drive the patch door at this file's fixed perimeter and return the
/// flux bracket, or `None` on any honest typed refusal — which of the
/// door's refusals count as honest is
/// [`crate::shared::patch::face_posture`]'s to say, and the rows here
/// are about the WIDTHS of the brackets that do come back.
///
/// `#[track_caller]` so that a dishonest posture from the door names
/// the ROW, not this wrapper.
#[track_caller]
fn drive(
    ku: &KnotVector,
    kv: &KnotVector,
    control: &[[RingInterval; 3]],
    weights: &[f64],
    eps: f64,
) -> Option<(f64, f64)> {
    face_posture(ku, kv, control, weights, 8.0, eps)
        .ok()
        .map(|fb| (fb.flux.lo(), fb.flux.hi()))
}

/// A rational wall whose u direction is a quarter-arc (so the weights
/// are non-unit and vary in u ONLY) and whose v direction is a cubic
/// with a deliberate S-bend, so the v curvature — and with it the v
/// Taylor remainder the exact arm removes — is what dominates the
/// composite's enclosure.
///
/// `perturb` breaks the v-uniformity of the weight net by ONE ulp on a
/// single entry. That is a change of 1 part in 10^16 to the surface and
/// no change at all to its shape, but it is exactly the hypothesis the
/// exact arm rests on, so the engine must fall back to the composite.
fn s_bend_wall(perturb: bool) -> (KnotVector, KnotVector, Vec<[RingInterval; 3]>, Vec<f64>) {
    let c = std::f64::consts::FRAC_1_SQRT_2;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let arc = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let wu = [1.0, c, 1.0];
    // The S-bend: four v control points swinging hard in z and in the
    // radial scale, which is what makes `f_vv` large.
    let scale = [1.0, 2.6, 0.35, 1.0];
    let zs = [0.0, 3.1, -2.4, 1.0];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (p, w) in arc.iter().zip(&wu) {
        for (s, z) in scale.iter().zip(&zs) {
            control.push([pt(p[0] * s), pt(p[1] * s), pt(*z)]);
            weights.push(*w);
        }
    }
    if perturb {
        // The LAST v entry of the middle u row: still positive, still
        // finite, still the same surface to fifteen digits.
        let idx = 4 + 3;
        weights[idx] = f64::from_bits(weights[idx].to_bits() + 1);
    }
    (ku, kv, control, weights)
}

/// **The arm-selection pin.** The two rows below are the same surface
/// to fifteen digits — one ulp on one weight is the entire
/// difference — so anything that separates their ENCLOSURES by more
/// than that perturbation could account for is the choice of
/// algorithm, and nothing else.
///
/// That is what this row pins, and it is why it pins a ratio rather
/// than a width: if the exact arm stops being taken, both rows run the
/// composite and their brackets agree to roundoff; if it is ever taken
/// where its hypothesis fails, both run the exact arm and they agree
/// again. Either way this goes red. The direction is deliberately NOT
/// asserted — see the measurement note below, which is a finding
/// rather than a baseline.
///
/// **Measured, and not what the arm was expected to buy:** at
/// `eps = 1e-6` the v-uniform (exact-arm) bracket is WIDER than its
/// composite twin — width 2.07e-2 against 7.45e-3. The arm removes the
/// v truncation error completely, but it pays a full Newton–Cotes node
/// set per knot span per u-cell where the composite pays one midpoint,
/// and each of those is a de Boor recurrence contributing its own ring
/// rounding. Once the schedule has refined far enough that the
/// composite's v remainder is already negligible, the arm's extra
/// rounding is the larger term. What the arm buys unconditionally is
/// COST (the v cell count drops to the block count) and exactness in v
/// on carriers the schedule cannot refine into; it is not a uniform
/// tightening, and the unit's PR body says so.
#[test]
fn the_exact_arm_is_taken_exactly_where_its_hypothesis_holds() {
    let (ku, kv, cp, ws) = s_bend_wall(false);
    let uniform = drive(&ku, &kv, &cp, &ws, 1e-6).expect("the v-uniform wall certifies");
    let (ku, kv, cp, ws) = s_bend_wall(true);
    let perturbed = drive(&ku, &kv, &cp, &ws, 1e-6).expect("the perturbed twin certifies");
    let (wu, wp) = (uniform.1 - uniform.0, perturbed.1 - perturbed.0);
    eprintln!("CERT5-ARM v-uniform width {wu:e} | one-ulp-perturbed width {wp:e}");
    // The two brackets must OVERLAP: they bound the same surface, and
    // a disjoint pair would mean one of the two arms is unsound.
    assert!(
        uniform.0 <= perturbed.1 && perturbed.0 <= uniform.1,
        "the two arms must bracket the same surface: {uniform:?} vs {perturbed:?}"
    );
    let ratio = if wu > wp { wu / wp } else { wp / wu };
    assert!(
        ratio > 1.5,
        "one ulp on one weight cannot change an enclosure width by itself, so \
         these two rows differing only in roundoff means ONE arm ran for both: \
         either the `w`-uniform-in-v exact arm stopped being taken, or it was \
         taken on the perturbed net, whose weights are not constant along v and \
         for which its exactness argument is simply false. widths {wu:e} vs \
         {wp:e} (ratio {ratio})"
    );
}

/// The fixture the drop-knots mutant is LETHAL on, and the reason it
/// has to be this one.
///
/// Two properties have to hold at once. The v direction is DEGREE 1,
/// so the second-derivative grid differentiates to nothing: `f_vv` is
/// `None` and reads as an exact zero, which means an unaligned cell
/// straddling the jump carries NO remainder at all and the bracket is
/// narrow and wrong rather than wide and sound. (On a degree-2 v
/// direction the block hull of `f_vv` is a control-net fact that does
/// not assume smoothness, so it bounds the jump anyway and the mutant
/// merely widens — which is why the C0 probes at degree 2 do not catch
/// it.) And the weights VARY IN V, so the `w`-uniform-in-v exact arm
/// is not taken: that arm integrates v per knot span whatever the cut
/// list says, so under it the mutation is invisible in v by
/// construction.
fn c0_jump_composite_arm() -> (KnotVector, KnotVector, Vec<[RingInterval; 3]>, Vec<f64>) {
    let c = std::f64::consts::FRAC_1_SQRT_2;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.377, 0.61, 1.0, 1.0], 1).unwrap();
    let arc = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let wu = [1.0, c, 1.0];
    let scale = [1.0, 1.35, 0.8, 1.15];
    let zs = [0.0, 0.3, 0.7, 1.0];
    // The v-dependent weight factor: this is what puts the patch on
    // the composite arm.
    let wv = [1.0, 0.86, 1.21, 0.94];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (p, w) in arc.iter().zip(&wu) {
        for ((s, z), fv) in scale.iter().zip(&zs).zip(&wv) {
            control.push([pt(p[0] * s), pt(p[1] * s), pt(*z)]);
            weights.push(*w * *fv);
        }
    }
    (ku, kv, control, weights)
}

/// **The knot-alignment gate.** A genuine C0 jump in v: a degree-1 v
/// direction whose sections vary in size, so `S_v` really is
/// discontinuous at the interior knots, at off-grid parameters.
///
/// The bracket must CONTAIN the true flux. It does when the cells are
/// cut on the knots, because every cell is then one smooth piece. It
/// does not when they are not: the midpoint rule is applied across the
/// jump, and the v remainder it would need is not merely too small but
/// identically zero (a degree-1 direction differentiates to nothing, so
/// the `f_vv` grid is `None` and reads as zero) — so the bracket is
/// narrow, confident and wrong.
///
/// Adopted from the CERT-5 R1 review lane's `genuine_c0_jump_is_
/// contained`, authorship kept, promoted from a review probe to a gate
/// because it is the row the unit's own red-first fixtures could not
/// be.
#[test]
fn a_genuine_c0_jump_stays_contained() {
    let (ku, kv, control, weights) = c0_jump_composite_arm();
    let got = drive(&ku, &kv, &control, &weights, 1e-6).expect(
        "the C0-jump wall must certify at eps 1e-6, so that the containment \
         assertion below is actually exercised",
    );
    // The truth, from the plain-`f64` Cox-de-Boor + Gauss-Legendre
    // oracle in `crates/geom-brep/tests/shared/patch.rs` — independent
    // of `props::quad`, which is the door under test — computed here
    // rather than pinned, and believed only after it agrees between two
    // resolutions a factor of two apart. Why that settles it is
    // `dense_over`'s own doc, there.
    let pa = oracle_patch(&ku, &kv, &control, &weights);
    let (f12, _) = pa.dense(12);
    let (f24, _) = pa.dense(24);
    assert!(
        (f12 - f24).abs() < 1e-7 * (1.0 + f24.abs()),
        "the oracle must converge before it is believed: {f12} vs {f24}"
    );
    let true_flux = f24;
    eprintln!("CERT5-CELLS c0-composite bracket {got:?} oracle {true_flux:e}");
    assert!(
        got.0 <= true_flux && true_flux <= got.1,
        "the enclosure must CONTAIN the true flux across a genuine C0 jump: \
         got [{:.15e}, {:.15e}], truth {true_flux:.15e}",
        got.0,
        got.1
    );
}
