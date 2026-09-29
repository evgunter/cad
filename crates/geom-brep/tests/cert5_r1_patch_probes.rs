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
//! - cell-rule edge cases: interior knots one ulp apart (cells thinner
//!   than an ulp of parameter) and interior knots within an ulp of the
//!   trim rectangle's edges;
//! - a genuine C0 jump at off-grid interior knots, which a cell rule
//!   that ignored the knots would integrate across.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Bounds;
use geom_core::spline::KnotVector;
use geom_core::{Interval, Tol};

use crate::shared::patch::{face_posture, oracle_patch};
use crate::shared::ring::pt;

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
    control: &[[Interval; 3]],
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
