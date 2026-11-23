//! Review probes for PR 7652 (`running::Tracked`, `Rounded::step`), kept on
//! the review branch only. Each states the contract the door's doc states
//! and is judged in exact arithmetic; a red one is a finding.
#![allow(clippy::float_cmp)]

use geom_core::Interval;
use geom_core::exact::Expansion;
use geom_core::running::{Rounded, Tracked, UNIT_ROUNDOFF};

/// **`at_middle` is off the exact value by at most its bound**
/// (`Tracked::at_middle`'s doc: "off the exact value by its bound and by
/// the half-width of its bracket"): an enclosure one ulp wide, `[1, 1 +
/// 2⁻⁵²]`, whose middle rounds (to even) onto its lower end, so its upper
/// end, a member, lies a whole ulp from the middle against a bound of half
/// of one. Red on the frozen head.
#[test]
fn review_at_middle_holds_every_member_of_its_bracket() {
    for (lo, hi) in [
        (1.0, 1.0 + f64::EPSILON),
        (1.0, 1.0 + 3.0 * f64::EPSILON),
        (1e3, 1e3 + 3.0 * (1e3f64.next_up() - 1e3)),
    ] {
        let r = Tracked::exact(Interval::from_bounds(lo, hi)).at_middle();
        for member in [lo, hi] {
            let mut gap = Expansion::product(member, 1.0);
            gap.add(-r.value);
            let gap: f64 = gap.components().iter().sum::<f64>().abs();
            assert!(
                gap <= r.error,
                "[{lo}, {hi}] reads {:?}; its member {member} is {gap:e} off",
                r
            );
        }
    }
}

/// **A step's first-order bound reads zero where its error is not zero**
/// (`Rounded::step`): `x = 0 ± 1e-6`, `y = 0` exact, `t = 1 ± 3u` (a ratio
/// that rounded onto the power of two `1`, `Tracked::ratio`'s charge). The
/// bound is `|1 − t|·e_x + … = 0`, while `x' + (y − x')·t'` at `x' = 1e-6`,
/// `t' = 1 + 3u` is `−3u·1e-6`. Every share the bound drops is second order
/// (the module's contract), so this probe pins what that contract allows,
/// not a defect of the step alone: `Mul` on two zeros each carrying a bound
/// is the same shape.
#[test]
fn review_a_step_whose_first_order_shares_all_vanish_reads_a_zero_bound() {
    let x = Rounded::exact(0.0).with_charge(1e-6);
    let t = Rounded::exact(1.0).with_charge(3.0 * UNIT_ROUNDOFF);
    let s = x.step(Rounded::exact(0.0), t);
    let worst = 3.0 * UNIT_ROUNDOFF * 1e-6;
    assert_eq!(s.error, 0.0, "the first-order bound vanishes");
    assert!(worst > s.error, "the error it misses: {worst:e}");
}

/// **Underflow is charged nowhere**, so a product by a normal power of two
/// that lands in the subnormals is not exact and the bound reads zero: `y =
/// 2⁻¹⁰²² + 2⁻¹⁰⁷⁴`, `t = ½`, `x = 0`; the product is off by `2⁻¹⁰⁷⁵`.
/// Documented ("barring underflow"); recorded, since no site is near.
#[test]
fn review_a_power_of_two_step_into_the_subnormals_is_not_exact() {
    let y = f64::MIN_POSITIVE + f64::from_bits(1);
    let s = Rounded::exact(0.0).step(Rounded::exact(y), Rounded::exact(0.5));
    // A product by ½ is exact exactly where doubling it back returns `y`
    // (`Expansion` cannot see a 2⁻¹⁰⁷⁵ remainder: its own terms underflow).
    assert_ne!(s.value * 2.0, y, "the product rounded");
    assert_eq!(s.error, 0.0, "and the bound reads zero");
}
