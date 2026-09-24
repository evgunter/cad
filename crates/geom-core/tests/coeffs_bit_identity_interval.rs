//! **Bit identity across the coefficient-hull doors, `Interval` brackets.**
//!
//! The same six vectors, weights and doors as `coeffs_bit_identity.rs`
//! (whose helpers this suite shares), with the coefficients carried as
//! `Interval` brackets — the third `CertifiedEnclosure` lane. 480 `f64`
//! values by their bits; `DIGEST` was captured at the merge base through
//! the free `(coeffs, span)` and `(kv, coeffs)` spellings and is
//! unchanged here.
//!
//! **Interval-gated, whole file**: it runs in the six `interval` test
//! jobs. The default lane's coefficient digest carries no feature gate.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Interval;

use crate::coeffs_bit_identity::{Rows, check, drive, values, vectors, weights};

/// The corpus: `Interval` brackets of the shared values.
fn rows() -> Rows {
    let mut o = Vec::new();
    for (vname, kv) in vectors() {
        let n = kv.control_count();
        #[allow(clippy::cast_precision_loss)]
        let c: Vec<Interval> = values(n)
            .iter()
            .enumerate()
            .map(|(i, x)| Interval::from_bounds(x - 0.02, x + 0.01 * i as f64))
            .collect();
        for rational in [false, true] {
            let w = weights(n, rational);
            let tag = if rational { "rat" } else { "nr" };
            drive(&mut o, &format!("{vname}.{tag}.interval"), &kv, &c, &w);
        }
    }
    o
}

const ROW_COUNT_INTERVAL: usize = 480;
/// **Re-captured when the C9 ring became a newtype over the backend**
/// (`0x077f_2c93_d2ac_b9b5` before): the ring's unconditional one-step
/// outward pad per operation is gone, and the backend's exactness
/// witnesses stand in its place. 222 of these 480 rows moved TIGHTER
/// and none moved looser — the same direction, and the same cause, as
/// the default lane's corpus.
const DIGEST_INTERVAL: u64 = 0xa75e_f69a_a5a0_c32b;
const SPOT_INTERVAL: &[(&str, u64)] = &[
    ("d1.nr.interval.hull@1.lo", 0xbff8_51eb_851e_b852),
    ("d2m2.nr.interval.sup_domain", 0x4002_8d4f_df3b_645a),
    ("d2m2.rat.interval.hull_rat@4.hi", 0x4002_8d4f_df3b_645a),
    // Tighter by three steps: the derivative-hull fold's differences
    // are exact at these coefficients and are no longer padded.
    ("d3.nr.interval.dhull@5.lo", 0xc002_9062_4dd2_f1ab),
    // Tighter by four steps, same cause one door over.
    ("d3.rat.interval.ddomain.hi", 0x4020_4ed9_1687_2b02),
    ("d3m2.rat.interval.sup@6", 0x4002_8d4f_df3b_645a),
    ("d3m3.nr.interval.domain.hi", 0x4002_8d4f_df3b_645a),
    ("d3m3.rat.interval.sup_domain_rat", 0x4002_8d4f_df3b_645a),
    // Tighter by four steps (a `hi` on a negative value, so the bit
    // pattern rises): a chain of exact differences the ring padded.
    ("d4.nr.interval.dcoeff.3.hi", 0xc016_d4fd_f3b6_45a2),
    ("d4.rat.interval.domain_rat.lo", 0xbff8_51eb_851e_b852),
];

#[test]
fn every_coefficient_door_is_bit_identical_on_interval_brackets() {
    check(&rows(), ROW_COUNT_INTERVAL, DIGEST_INTERVAL, SPOT_INTERVAL);
}
