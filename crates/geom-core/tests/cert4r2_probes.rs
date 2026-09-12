//! CERT-4 review lane R2 — adversarial probes for the centred period
//! fold (issue 1191, PR 1303). Local-only; never pushed.
//!
//! Probes the PR's structural-zero claim ("the two windows agree
//! BITWISE on [0, pi)") at the points the pinned 4000-point sweep
//! cannot reach, and takes the differentials the hit-list dispositions
//! claim by inspection.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Real;

const TAU: f64 = core::f64::consts::TAU;
const PI: f64 = core::f64::consts::PI;

/// **Falsification witness**: the two windows do NOT agree bitwise on
/// the whole of [0, pi). At the largest f64 strictly below pi,
/// `d/tau + 0.5` rounds to exactly 1.0 (round-half-to-even), so the
/// centred window's jump sits ONE ULP EARLY and `extent - setback`
/// is tau, not 0. The old spelling's second fold was the identical
/// expression, so this is a claim overstatement, not a regression.
#[test]
fn the_two_windows_disagree_at_the_top_ulp_of_the_shared_interior() {
    let d = f64::from_bits(PI.to_bits() - 1); // largest f64 < pi
    assert!((0.0..PI).contains(&d), "d is in [0, pi)");
    let extent = <f64 as Real>::reduce_periodic(d, TAU);
    let setback = <f64 as Real>::reduce_periodic_centred(d, TAU);
    // The claim would demand bit-equality here; the truth:
    assert_eq!(extent.to_bits(), d.to_bits(), "extent is the identity");
    assert_eq!(
        setback,
        d - TAU,
        "the centred window jumped one ulp early: floor(d/tau + 1/2) = 1"
    );
    assert_eq!(extent - setback, TAU, "the fit margin is tau, not 0");
    // One ulp further down both windows agree again.
    let d2 = f64::from_bits(PI.to_bits() - 2);
    assert_eq!(
        <f64 as Real>::reduce_periodic(d2, TAU).to_bits(),
        <f64 as Real>::reduce_periodic_centred(d2, TAU).to_bits()
    );
}

/// At d = -0.0 (reachable: a +0.0 difference times a -1.0 turn) the two
/// windows disagree bitwise (+0.0 vs -0.0) but the margin is still
/// exactly zero — the structural-zero GUARANTEE survives, only the
/// bitwise-agreement PROSE is wrong.
#[test]
fn negative_zero_disagrees_bitwise_but_the_margin_is_still_exact() {
    let d = -0.0f64;
    let extent = <f64 as Real>::reduce_periodic(d, TAU);
    let setback = <f64 as Real>::reduce_periodic_centred(d, TAU);
    assert_eq!(extent.to_bits(), 0.0f64.to_bits(), "extent is +0.0");
    assert_eq!(setback.to_bits(), (-0.0f64).to_bits(), "setback is -0.0");
    assert_eq!(extent - setback, 0.0, "the margin is exactly zero anyway");
    // And the reachable spelling: (to - from) * turn with to == from.
    let (to, from, turn) = (1.25f64, 1.25f64, -1.0f64);
    assert_eq!(((to - from) * turn).to_bits(), (-0.0f64).to_bits());
}

/// **Differential, not inspection**: the three "respelled,
/// bit-identical" branch-pin sites (chord_join.rs:1643 old spelling
/// `(q + 1/2).floor()` with `q = (prev - raw)/tau`; pcurves.rs
/// 1230/1244; replace_face.rs:1914) against `periodic_branch` over an
/// adversarial value sweep. Identical op sequence, so identical bits.
#[test]
fn the_branch_pin_respells_are_bit_identical_by_differential() {
    let half = 0.5f64;
    let mut xs: Vec<f64> = vec![
        0.0,
        -0.0,
        1e-300,
        -1e-300,
        5e-324,
        PI,
        -PI,
        TAU,
        -TAU,
        1e15,
        -1e15,
        f64::from_bits(PI.to_bits() - 1),
    ];
    let mut s = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..100_000 {
        s = s
            .wrapping_mul(0xd128_1b58_5f9d_3fa7)
            .wrapping_add(0x2545_f491_4f6c_dd1d);
        let f = f64::from_bits((s >> 12) | 0x3ff0_0000_0000_0000) - 1.0; // [0,1)
        xs.push((f - 0.5) * 40.0);
    }
    for p in [TAU, PI, 1.0, 3.0e-8, 7.5e11] {
        for &x in &xs {
            let q = x / p;
            let old = (q + half).floor();
            let new = <f64 as Real>::periodic_branch(x, p);
            assert_eq!(old.to_bits(), new.to_bits(), "x={x:e} p={p:e}");
        }
    }
}

/// The classify.rs:277 / chord_join select_arc respells are NOT
/// bit-identical — the old spellings added and subtracted half a
/// period around `reduce_periodic`, two extra roundings. Witness that
/// f64 bits genuinely move there (the PR's "0 of 3153 moved" is a
/// corpus measurement, not an identity).
#[test]
fn the_centred_respells_move_f64_bits_off_corpus() {
    let half_tau = TAU * 0.5;
    let mut moved = 0u32;
    let mut total = 0u32;
    for k in -2000..2000 {
        let x = f64::from(k) * 7.7e-4 * PI; // c - mid, spread over periods
        let old = <f64 as Real>::reduce_periodic(x + half_tau, TAU) - half_tau;
        let new = <f64 as Real>::reduce_periodic_centred(x, TAU);
        total += 1;
        if old.to_bits() != new.to_bits() {
            moved += 1;
        }
        // Where they differ it is by rounding, never by a branch:
        assert!(
            (old - new).abs() <= 4.0 * f64::EPSILON * TAU,
            "x={x:e}: old {old:e} vs new {new:e}"
        );
    }
    assert!(
        moved > 0,
        "the respell moved no bits over {total} samples — then it WAS bit-identical \
         and this probe's premise is wrong"
    );
}
