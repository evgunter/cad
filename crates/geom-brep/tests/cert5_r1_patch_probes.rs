//! CERT-5 review lane R1 patch-level probes (blinded adversarial
//! review of PR 1314, frozen head 3fc450d6).
//!
//! **Adopted into the unit by merge, authorship kept** — this
//! project's convention for review probes that earn a place. Three
//! edits since: rustfmt, a clippy fix, and the reviewer's dense
//! oracle moved out to `crate::shared::patch`, where the one other
//! suite that was already reaching into this file for it
//! (`cert5_arm_and_cells.rs`) now reaches instead. The probes are
//! otherwise unchanged.
//!
//! What these attack, per the review brief:
//! - the exact `w`-uniform-in-v arm's CONTAINMENT on a patch of the
//!   reviewer's own construction (off-grid interior knots in BOTH
//!   directions — not the blades, not dm1);
//! - a nearly-uniform-weight patch (one weight one ulp off) must fall
//!   back to the composite arm and still enclose the truth;
//! - cell-rule edge cases: interior knots one ulp apart (cells thinner
//!   than an ulp of parameter) and interior knots within an ulp of the
//!   trim rectangle's edges.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::props::PropsError;
use geom_brep::props::quad::nurbs_patch_face;
use geom_core::spline::KnotVector;
use geom_core::{Band, RingInterval, Tol};

use crate::shared::patch::{face_posture, oracle_patch, pt};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Run the engine; on `Ok`, both brackets must contain the dense
/// oracle (with a 1e-9-relative slack for the oracle's own f64
/// drift); a refusal must be a typed quadrature posture. Returns the
/// certified widths when certified.
///
/// The oracle is what a certified bracket is checked against, so it is
/// evaluated inside the `Ok` arm only — a typed refusal has no bracket.
/// Its two resolutions, 12 and 24 cells per span, must agree before
/// either is believed; why two rungs a factor of two apart settle it is
/// [`crate::shared::patch::dense_over`]'s doc, which is where that
/// argument lives.
///
/// `#[track_caller]` so that a dishonest posture from the door names
/// the ROW, not this wrapper: `face_posture` panics at its caller's
/// location and this is that caller.
#[track_caller]
fn drive(
    name: &str,
    ku: &KnotVector,
    kv: &KnotVector,
    control: &[[RingInterval; 3]],
    weights: &[f64],
    perimeter: f64,
    eps: f64,
) -> Option<(f64, f64)> {
    match face_posture(ku, kv, control, weights, perimeter, eps) {
        Ok(fb) => {
            let pa = oracle_patch(ku, kv, control, weights);
            let (of1, oa1) = pa.dense(12);
            let (of2, oa2) = pa.dense(24);
            assert!(
                (of1 - of2).abs() < 1e-7 * (1.0 + of2.abs())
                    && (oa1 - oa2).abs() < 1e-7 * (1.0 + oa2.abs()),
                "{name}: oracle did not converge, so the containment assertions below \
                 would compare against a number that is not the truth: flux {of1} vs \
                 {of2}, area {oa1} vs {oa2}"
            );
            let sf = 1e-9 * (1.0 + of2.abs());
            let sa = 1e-9 * (1.0 + oa2.abs());
            eprintln!(
                "CERT5-R1 {name}: flux [{:.12e}, {:.12e}] oracle {of2:.12e} width {:.3e}; \
                 area [{:.12e}, {:.12e}] oracle {oa2:.12e} width {:.3e}",
                fb.flux.lo(),
                fb.flux.hi(),
                fb.flux.hi() - fb.flux.lo(),
                fb.area.lo(),
                fb.area.hi(),
                fb.area.hi() - fb.area.lo(),
            );
            assert!(
                fb.flux.lo() - sf <= of2 && of2 <= fb.flux.hi() + sf,
                "{name}: FLUX ENCLOSURE EXCLUDES THE TRUTH: [{:.15e}, {:.15e}] vs oracle {of2:.15e}",
                fb.flux.lo(),
                fb.flux.hi()
            );
            assert!(
                fb.area.lo() - sa <= oa2 && oa2 <= fb.area.hi() + sa,
                "{name}: AREA ENCLOSURE EXCLUDES THE TRUTH: [{:.15e}, {:.15e}] vs oracle {oa2:.15e}",
                fb.area.lo(),
                fb.area.hi()
            );
            Some((fb.flux.hi() - fb.flux.lo(), fb.area.hi() - fb.area.lo()))
        }
        Err(e) => {
            eprintln!("CERT5-R1 {name}: typed refusal {e}");
            None
        }
    }
}

/// The 270-degree arc (three 90-degree rational sub-arcs, interior u
/// knots 1/3 and 2/3 — OFF the composite's dyadic grid, unlike the
/// blades' quarter-circle whose knot sits at 1/2): 7 homogeneous
/// control points on the unit circle, weights `1, c, 1, c, 1, c, 1`.
fn arc270() -> (Vec<[f64; 2]>, Vec<f64>) {
    let c = std::f64::consts::FRAC_1_SQRT_2;
    let deg = |d: f64| d.to_radians();
    let on = |a: f64| [a.cos(), a.sin()];
    let mid = |a: f64| [a.cos() / c, a.sin() / c];
    (
        vec![
            on(deg(-135.0)),
            mid(deg(-90.0)),
            on(deg(-45.0)),
            mid(deg(0.0)),
            on(deg(45.0)),
            mid(deg(90.0)),
            on(deg(135.0)),
        ],
        vec![1.0, c, 1.0, c, 1.0, c, 1.0],
    )
}

const THIRD: f64 = 1.0 / 3.0;
const TWO_THIRDS: f64 = 2.0 / 3.0;

/// The reviewer's own wall: the 270-degree arc extruded along a
/// QUADRATIC z-spline whose interior v knots sit at 0.3777 and 0.6123
/// (nothing dyadic) — off-grid interior knots in BOTH directions.
/// Weights vary in u only, so the patch satisfies the exact arm's
/// hypothesis as stated.
fn wall() -> (KnotVector, KnotVector, Vec<[RingInterval; 3]>, Vec<f64>) {
    let ku = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, THIRD, THIRD, TWO_THIRDS, TWO_THIRDS, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.3777, 0.6123, 1.0, 1.0, 1.0], 2).unwrap();
    let (xy, wu) = arc270();
    let zc = [0.0, 0.45, 1.05, 1.55, 2.0];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (p, w) in xy.iter().zip(&wu) {
        for z in &zc {
            control.push([pt(p[0]), pt(p[1]), pt(*z)]);
            weights.push(*w);
        }
    }
    (ku, kv, control, weights)
}

/// E2E probe (patch half): the reviewer's own rational wall with
/// off-grid interior knots in both directions, exact-arm eligible,
/// must certify and CONTAIN the independent oracle.
#[test]
fn own_wall_offgrid_both_directions_exact_arm_contains() {
    let (ku, kv, control, weights) = wall();
    // Reported, not demanded: a 270-degree wall may be schedule-limited
    // at the default eps (the half-cylinder floor's family). What is
    // asserted is soundness (containment when certified) and, via
    // `drive`, that any refusal is nowhere near the retired floor.
    let widths = drive(
        "own-wall-exact-arm",
        &ku,
        &kv,
        &control,
        &weights,
        8.0,
        1e-7,
    );
    assert!(
        widths.is_some(),
        "at eps 1e-7 (target 1.024e-4, far above the 3.35e-6 schedule \
         width) the exact arm must certify, so its CONTAINMENT is exercised"
    );
}

/// One weight nudged by ONE ULP: the exact-arm hypothesis (exact f64
/// equality) must fail, the composite arm must carry the patch, and
/// the enclosure must still contain the truth.
#[test]
fn near_uniform_weights_take_the_composite_arm_soundly() {
    let (ku, kv, control, mut weights) = wall();
    // Middle u-row, middle v entry: break v-constancy by one ulp.
    let nv = kv.control_count();
    let idx = 3 * nv + 2;
    weights[idx] = weights[idx].next_up();
    let widths = drive(
        "own-wall-ulp-perturbed",
        &ku,
        &kv,
        &control,
        &weights,
        8.0,
        1e-7,
    );
    assert!(
        widths.is_some(),
        "the one-ulp-perturbed twin must certify through the composite arm \
         at eps 1e-7, exercising the composite arm's containment"
    );
}

/// Interior knots ONE ULP apart, and an exactly-uniform non-unit
/// weight net (all 1.5 — the rational lane and the exact arm, same
/// surface as the weight-1 patch): cells thinner than an ulp of
/// parameter must not panic, drop area, or double-count; the answer
/// must contain the oracle or refuse typed.
#[test]
fn knots_one_ulp_apart_stay_sound() {
    let half_up = 0.5f64.next_up();
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, half_up, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    // A gentle non-planar sheet: x sweeps, y arcs up and back, z is v.
    let xs = [0.0, 0.5, 1.0, 1.5, 2.0];
    let ys = [0.0, 0.6, 0.8, 0.6, 0.0];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (x, y) in xs.iter().zip(&ys) {
        for z in &[0.0, 1.0] {
            control.push([pt(*x), pt(*y), pt(*z)]);
            weights.push(1.5);
        }
    }
    drive(
        "ulp-twin-knots",
        &ku,
        &kv,
        &control,
        &weights,
        8.0,
        Tol::witness().get().eps,
    );
}

/// Interior knots within an ulp of the trim rectangle's EDGES: the
/// first/last cells are ulp-thin. Same soundness contract.
#[test]
fn knots_hugging_the_trim_edges_stay_sound() {
    let lo_in = 1.0e-9;
    let hi_in = 1.0 - 1.0e-9;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, lo_in, hi_in, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let xs = [0.0, 0.5, 1.0, 1.5, 2.0];
    let ys = [0.0, 0.4, 0.9, 0.4, 0.0];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (x, y) in xs.iter().zip(&ys) {
        for z in &[0.0, 1.0] {
            control.push([pt(*x), pt(*y), pt(*z)]);
            weights.push(2.0);
        }
    }
    drive(
        "edge-hugging-knots",
        &ku,
        &kv,
        &control,
        &weights,
        8.0,
        1e-7,
    );
}

/// The DISCRIMINATOR for a residual knot-coupled floor, shaped unlike
/// the unit's own greps: the same 270-degree wall with 2 vs 6 off-grid
/// interior v knots. Under the retired defect the width scaled with
/// the off-grid count (~1.07e-4 each); post-fix the two widths must
/// agree to a few percent — the schedule, not the knots, sets them.
#[test]
fn refusal_width_does_not_scale_with_offgrid_knot_count() {
    let (ku, _, _, _) = wall();
    let (xy, wu) = arc270();
    let build = |vknots: Vec<f64>, zc: &[f64]| {
        let kv = KnotVector::clamped(vknots, 2).unwrap();
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for (p, w) in xy.iter().zip(&wu) {
            for z in zc {
                control.push([pt(p[0]), pt(p[1]), pt(*z)]);
                weights.push(*w);
            }
        }
        (kv, control, weights)
    };
    // The door is called directly rather than through
    // `shared::patch::face_posture`: this row reads the budget
    // refusal's `width_len` payload, which is the number it compares,
    // and it drives an EXPLICIT rectangle rather than the knot
    // vectors' domain.
    let width = |name: &str, kv: &KnotVector, control: &[[RingInterval; 3]], weights: &[f64]| {
        let out = nurbs_patch_face::<f64>(
            &ku,
            kv,
            control,
            weights,
            (0.0, 1.0, 0.0, 1.0),
            8.0,
            0.0,
            Tol::witness().get().eps,
            band(),
        );
        match out {
            Ok(fb) => {
                let w = fb.flux.hi() - fb.flux.lo();
                eprintln!("CERT5-R1 {name}: certified, flux width {w:.6e}");
                w
            }
            Err(PropsError::QuadratureBudget { width_len, .. }) => {
                eprintln!("CERT5-R1 {name}: budget, width_len {width_len:.6e}");
                width_len
            }
            other => panic!("{name}: unexpected outcome {other:?}"),
        }
    };
    let (kv2, c2, w2) = build(
        vec![0.0, 0.0, 0.0, 0.3777, 0.6123, 1.0, 1.0, 1.0],
        &[0.0, 0.45, 1.05, 1.55, 2.0],
    );
    let (kv6, c6, w6) = build(
        vec![
            0.0, 0.0, 0.0, 0.13, 0.27, 0.3777, 0.51, 0.6123, 0.87, 1.0, 1.0, 1.0,
        ],
        &[0.0, 0.2, 0.5, 0.8, 1.1, 1.4, 1.7, 1.9, 2.0],
    );
    let a = width("v-knots-2", &kv2, &c2, &w2);
    let b = width("v-knots-6", &kv6, &c6, &w6);
    assert!(
        b < 3.0 * a,
        "the width scales with the off-grid v knot count — a knot-coupled \
         floor survives by some spelling the unit's grep could not see: \
         2 knots -> {a:e}, 6 knots -> {b:e}"
    );
}

/// A GENUINE C0 jump: a degree-1 v direction whose sections VARY in
/// size, so `S_v` (and the flux integrand) really jumps at the
/// off-grid interior v knots — unlike the unit's blade-8, whose
/// identical evenly-stacked sections make the locus a smooth
/// extrusion and the multiplicity-equals-degree knots structural
/// only. Under the shipped engine this must certify (at a loose eps)
/// and CONTAIN the dense oracle; under a corrupted cut list that
/// ignores knots, the midpoint rule integrates across the jump with a
/// ZERO v remainder (the derivative grid differentiates to nothing)
/// and the bracket should exclude the truth — which is exactly the
/// mutant the unit's own red-first rows fail to catch.
#[test]
fn genuine_c0_jump_is_contained() {
    let c = std::f64::consts::FRAC_1_SQRT_2;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.377, 0.61, 1.0, 1.0], 1).unwrap();
    let arc = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let wu = [1.0, c, 1.0];
    let scale = [1.0, 1.35, 0.8, 1.15];
    let zs = [0.0, 0.3, 0.7, 1.0];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (p, w) in arc.iter().zip(&wu) {
        for (s, z) in scale.iter().zip(&zs) {
            control.push([pt(p[0] * s), pt(p[1] * s), pt(*z)]);
            weights.push(*w);
        }
    }
    let out = drive("genuine-c0-jump", &ku, &kv, &control, &weights, 6.0, 1e-6);
    assert!(
        out.is_some(),
        "the C0-jump wall must certify at eps 1e-6 so its containment is exercised"
    );
}
