//! **`x − sin x` keeps its digits at every `x`** ([`Real::less_sin`]),
//! at each scalar by that scalar's own contract.
//!
//! Three oracles, none the series under test. [`TRUTHS`] holds the
//! function at nine points as a nearest `f64` and what is left over,
//! from an 80-digit sum of the series. Below `2⁻¹⁰` the first two terms
//! bracket it, `x³/6·(1 − x²/20) ≤ x − sin x ≤ x³/6`, to under a unit in
//! the last place. The triple-angle identity
//! `f(x) = 3·f(x/3) + 4·sin³(x/3)`, a sum of terms of one sign, carries a
//! reading across the cutoff.
//!
//! What no row here sees: which arm reads the cutoff itself (both are
//! right there), and the `Interval` tail read as the point `1` in place
//! of `[0, 1]` (under a hundredth of a unit in the last place, inside
//! the rounding an enclosure already carries).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::interval::Interval;
use geom_core::real::LESS_SIN_SERIES_BELOW;
use geom_core::sym::with_session_rules;
use geom_core::{Bounds, Dual, Real, Sym, SymBudget, SymRules};

const ULP: f64 = f64::EPSILON;

/// `(x, t, r)`: `x − sin x` is `t + r`, `t` its nearest `f64`.
const TRUTHS: [(f64, f64, f64); 9] = [
    (1.1641532182693481e-9, 2.629536350736706e-28, 1.4929365188064102e-44),
    (9.5367431640625e-7, 1.4456028966472734e-19, 6.41976648143314e-36),
    (0.00146484375, 5.238688920161689e-10, 3.840472857692797e-26),
    (0.03125, 5.08601467392126e-6, 5.774541248819496e-24),
    (0.5, 0.020574461395796998, 1.6345229086023983e-18),
    (1.0, 0.1585290151921035, -1.776845092935536e-18),
    (1.90625, 0.9619889413142455, 2.8020804737642804e-17),
    (1.96875, 1.0468940578142722, 1.223145636473661e-17),
    (2.0, 1.0907025731743183, 1.4020906557816256e-17),
];

/// `got − (t + r)` in units of `t`'s last place. `got − t` is exact for
/// a reading within a factor of two of `t`.
fn ulps_off(got: f64, t: f64, r: f64) -> f64 {
    ((got - t) - r) / (t * ULP)
}

/// Each reading is within two units in the last place of the truth, and
/// within three quarters of one at the three points by the cutoff, where
/// the last kept term `x²³/23!` is over one unit.
#[test]
fn a_reading_is_the_truth_to_the_last_place() {
    for (x, t, r) in TRUTHS {
        let off = ulps_off(x.less_sin(), t, r);
        let room = if x > 1.9 { 0.75 } else { 2.0 };
        assert!(off.abs() <= room, "x = {x}: {off} units in the last place off");
    }
}

/// The two-term bracket of `x − sin x` for `0 < x ≤ 2⁻¹⁰`.
fn bracket(x: f64) -> (f64, f64) {
    let lead = x * x * x / 6.0;
    (lead * (1.0 - x * x / 20.0), lead)
}

#[test]
fn a_small_span_reads_inside_the_two_term_bracket() {
    for k in 10..=40 {
        for mantissa in [1.0, 1.3, 1.9] {
            let x = mantissa * 0.5_f64.powi(k);
            let (lo, hi) = bracket(x);
            let got = x.less_sin();
            assert!(
                lo * (1.0 - 4.0 * ULP) <= got && got <= hi * (1.0 + 4.0 * ULP),
                "x = {x:e}: {got:e} outside [{lo:e}, {hi:e}]"
            );
            assert_eq!(
                (-x).less_sin().to_bits(),
                (-got).to_bits(),
                "x = {x:e}: the reading is odd"
            );
        }
    }
}

/// From `x ≈ 3⁻⁷` up past a turn, both arms and the cutoff between them.
#[test]
fn the_triple_angle_identity_holds_across_the_cutoff() {
    let mut x = 7.0 / 2187.0;
    while x < 7.0 {
        for at in [
            x,
            LESS_SIN_SERIES_BELOW,
            LESS_SIN_SERIES_BELOW * (1.0 + ULP),
        ] {
            let third = at / 3.0;
            let want = 3.0 * third.less_sin() + 4.0 * third.sin().powi(3);
            let got = at.less_sin();
            assert!(
                (got - want).abs() <= 16.0 * ULP * want,
                "x = {at}: {got:e} against the identity's {want:e}"
            );
        }
        x *= 1.37;
    }
}

/// A point's enclosure holds the truth and is thin; a box across the
/// cutoff holds its points'.
#[test]
fn an_enclosure_holds_the_truth_is_thin_and_holds_across_the_cutoff() {
    for (x, t, r) in TRUTHS {
        let got = Interval::from_f64(x).less_sin();
        assert!(got.is_certified(), "x = {x:e}: {got:?}");
        assert!(
            ulps_off(got.lo(), t, r) <= 0.0 && 0.0 <= ulps_off(got.hi(), t, r),
            "x = {x:e}: {got:?} does not hold {t:e} + {r:e}"
        );
        assert!(
            got.hi() - got.lo() <= 64.0 * ULP * t,
            "x = {x:e}: {got:?} is wider than a short chain's rounding"
        );
    }
    let wide = Interval::from_bounds(1.5, 2.5).less_sin();
    for x in [1.5, 1.75, LESS_SIN_SERIES_BELOW, 2.25, 2.5] {
        let at = Interval::from_f64(x).less_sin();
        assert!(
            wide.lo() <= at.lo() && at.hi() <= wide.hi(),
            "[1.5, 2.5] reads {wide:?}, which does not hold {x}'s {at:?}"
        );
        let third = Interval::from_f64(x) / Interval::from_f64(3.0);
        let identity = Interval::from_f64(3.0) * third.less_sin()
            + Interval::from_f64(4.0) * third.sin().powi(3);
        assert!(
            at.lo() <= identity.hi() && identity.lo() <= at.hi(),
            "x = {x}: {at:?} and the identity's {identity:?} share no point"
        );
    }
}

/// The value channel of a dual and of a recording scalar is the base
/// scalar's door, and a dual's slope `1 − cos x` keeps its digits too.
#[test]
fn a_dual_and_a_recording_scalar_carry_the_base_scalars_value() {
    for x in [1.0e-7, 0.4, 3.0] {
        let want = x.less_sin();
        let dual = Dual::variable(x).less_sin();
        assert_eq!(
            dual.value.to_bits(),
            want.to_bits(),
            "x = {x}: the dual's value"
        );
        // `1 − cos x = x²/2·(1 − x²/12 + …)`, to a part in 1e12 below 1e-6.
        let slope = if x < 1.0e-6 {
            x * x / 2.0
        } else {
            1.0 - x.cos()
        };
        assert!(
            (dual.deriv - slope).abs() <= 1.0e-12 * slope,
            "x = {x}: slope {:e} against {slope:e}",
            dual.deriv
        );
        let budget = SymBudget {
            max_terms: 4096,
            max_degree: 128,
        };
        let (sym, _) = with_session_rules(budget, SymRules::shipped(), || {
            Sym::<f64>::from_f64(x).less_sin().value
        });
        assert_eq!(sym.to_bits(), want.to_bits(), "x = {x}: the recorded value");
    }
}
