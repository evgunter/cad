//! **Review rows for PR 5572** (`normalizing_scale`, frame/normalize-overflow
//! at `e5d47354b`). Every row but the symbolic one is RED on that head:
//! each states a claim the PR makes (or the old spelling met) at an
//! input its own rows do not draw.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::Vec3;
use geom_core::{Bounds, Dual, Interval, Real};

fn pow2(e: i32) -> f64 {
    if e >= -1022 {
        f64::from_bits(u64::try_from(e + 1023).unwrap() << 52)
    } else {
        f64::from_bits(1u64 << (e + 1074))
    }
}

fn bits(v: Vec3<f64>) -> [u64; 3] {
    [v.x, v.y, v.z].map(f64::to_bits)
}

/// Claim "in range, no bit moves": the witness here is `≈ 1.4 · 2⁻⁴⁹⁹`,
/// inside the documented `(2⁻⁵⁰⁰, 2⁵⁰⁰)`, but `1 / (w · 2⁻⁵⁷⁵)` already
/// overflows (the reciprocal of anything at or below `2⁻¹⁰²⁴` is `∞`), so
/// the scale-up arm is taken for every witness up to `2⁻⁴⁴⁹`. Where a
/// smaller component's square is subnormal in the unscaled spelling, the
/// two differ in the last bit (found by a 60M-draw search: ~1 in 30M).
#[test]
fn a_witness_inside_the_documented_range_takes_the_plain_spelling() {
    let v = Vec3::new(
        f64::from_bits(2_363_443_713_053_405_803),
        f64::from_bits(2_300_203_861_003_783_978),
        f64::from_bits(2_286_391_720_758_537_409),
    );
    let w = v.norm_witness();
    assert!(w > pow2(-500) && w < pow2(-498), "{w:e}");
    assert_eq!(bits(v.normalize()), bits(v / v.norm()), "{v:?}");
}

/// Claim "the prescale loses no bit": past `2⁵⁰⁰` the `2⁻⁶⁰⁰` prescale
/// pushes a component below `~2⁻⁴²²` into the subnormal range or to
/// zero. For a witness in `[2⁵⁰⁰, 2⁵¹¹)` the unscaled spelling was
/// finite and answered this component correctly; the prescale answers 0.
#[test]
fn a_small_component_past_two_pow_500_keeps_its_direction() {
    for e in [500, 505, 510] {
        let small = 1.2345 * pow2(-480);
        let v = Vec3::new(pow2(e), small, 0.0);
        let want = small / pow2(e); // ≈ 1e-295, a normal number
        assert_eq!(v.normalize().y.to_bits(), want.to_bits(), "2^{e}");
    }
}

/// At the interval scalar: a component enclosure `[1e-160, 1]` with the
/// others exactly zero names `+x` at every point. The witness's lower end
/// sends it to the scale-up arm, whose norm overflows at the upper end,
/// and the answer's `x` loses its definite sign (lower end `-5e-324`);
/// the unscaled spelling kept it at `[1e-160, …]`.
#[test]
fn a_box_straddling_the_low_threshold_keeps_a_definite_sign() {
    let v = Vec3::new(
        Interval::from_bounds(1e-160, 1.0),
        Interval::from_f64(0.0),
        Interval::from_f64(0.0),
    );
    let old = v / v.norm();
    assert!(old.x.lo() > 0.0, "precondition: {:?}", old.x);
    assert!(v.normalize().x.lo() > 0.0, "{:?}", v.normalize().x);
}

/// At the dual scalar: the derivative channel is scaled with the value,
/// so `2⁶⁰⁰` overflows a derivative the unscaled spelling carried. Value
/// `2⁻⁵⁰⁵`, derivative `2⁵⁰⁰`: the true `dx̂/dt` is `≈ 1.2e302`, finite.
#[test]
fn the_derivative_channel_survives_the_scale_up_arm() {
    let (a, da) = (pow2(-505), pow2(500));
    let v = Vec3::new(Dual::new(a, da), Dual::new(a, 0.0), Dual::new(0.0, 0.0));
    let old = (v / v.norm()).x.deriv;
    assert!(old.is_finite(), "precondition: {old:e}");
    let new = v.normalize().x.deriv;
    assert!(new.is_finite(), "{new:e}");
}

/// The symbolic tier's `‖v̂‖ = 1` theorem fires on the prescaled arms
/// too (GREEN on the PR head): the branch changes the term's shape, not
/// what rule E can fold.
mod sym {
    use geom_core::linalg::Vec3;
    use geom_core::predicate::{Band, Margin, Sign};
    use geom_core::sym::with_session_rules;
    use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymRules};

    fn p(name: &str, v: f64) -> Sym<f64> {
        Sym::param(ParamSymbol::new(test_utils::symbol_id(name)), v)
    }

    #[test]
    fn the_unit_norm_theorem_fires_past_either_end() {
        for (x, y, z) in [(3e200, 4e200, 12e200), (3e-170, 4e-170, 12e-170)] {
            let (out, counts) = with_session_rules(
                SymBudget {
                    max_terms: 4096,
                    max_degree: 128,
                },
                SymRules::shipped(),
                || {
                    let m = Vec3::new(p("x", x), p("y", y), p("z", z))
                        .normalize()
                        .norm()
                        - Sym::from_f64(1.0);
                    geom_core::k_stats::decide(
                        "review_probe",
                        Margin::of(m),
                        Band::new(1.0e-9, 1.0e-8).unwrap(),
                    )
                },
            );
            assert!(
                matches!(out, Ok(Sign::Zero)) && counts.symbolic == 1,
                "({x:e}, {y:e}, {z:e}): {out:?}"
            );
        }
    }
}
