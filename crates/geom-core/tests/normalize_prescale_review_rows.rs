//! **`normalize`'s prescale, at the inputs its cuts are about**: a
//! witness just inside them, components far apart in magnitude, and
//! the interval, dual and symbolic scalars. The cuts themselves are
//! pinned beside `normalizing_scale` in `linalg/vec.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::Vec3;
use geom_core::{Bounds, Dual, Interval, Real, pow2};

fn bits(v: Vec3<f64>) -> [u64; 3] {
    [v.x, v.y, v.z].map(f64::to_bits)
}

/// A witness `≈ 1.4 · 2⁻⁴⁹⁹`, inside the unscaled range, whose smaller
/// components square into the subnormals: the unscaled spelling,
/// bit for bit (found by a 60M-draw search against a cut at `2⁻⁴⁴⁹`).
#[test]
fn a_witness_inside_the_unscaled_range_takes_the_plain_spelling() {
    let v = Vec3::new(
        f64::from_bits(2_363_443_713_053_405_803),
        f64::from_bits(2_300_203_861_003_783_978),
        f64::from_bits(2_286_391_720_758_537_409),
    );
    let w = v.norm_witness();
    assert!(w > pow2(-500) && w < pow2(-498), "{w:e}");
    assert_eq!(bits(v.normalize()), bits(v / v.norm()), "{v:?}");
}

/// Past `2⁵⁰⁰` and below the cut at `2⁵¹¹`, the unscaled spelling
/// answers a component whose entry is a normal `≈ 1e-295`; so does
/// `normalize`.
#[test]
fn a_small_component_past_two_pow_500_keeps_its_direction() {
    for e in [500, 505, 510] {
        let small = 1.2345 * pow2(-480);
        let v = Vec3::new(pow2(e), small, 0.0);
        let want = small / pow2(e);
        assert_eq!(v.normalize().y.to_bits(), want.to_bits(), "2^{e}");
    }
}

/// **Components far apart in magnitude**, carried through every binade
/// the witness can take: `y`'s entry is `≈ 2⁻¹⁰⁰⁰`, a normal number at
/// the bottom of the range, and no arm may lose a bit of it. The
/// expected value is the unscaled spelling of the same vector at
/// witness `1.5`, where every component is exact.
#[test]
fn a_mixed_binade_vector_keeps_every_normal_entry_at_every_scale() {
    let base = Vec3::new(1.5, 1.25 * pow2(-1000), -0.75 * pow2(-300));
    let want = base / base.norm();
    assert!(want.y.is_normal(), "fixture: {:e}", want.y);
    for e in -72..=1023 {
        let v = base * pow2(e);
        assert_eq!(bits(v.normalize()), bits(want), "2^{e}: {v:?}");
    }
}

/// At the interval scalar: a component enclosure `[1e-160, 1]` with the
/// others exactly zero names `+x` at every point, and the answer's `x`
/// keeps a definite sign (the low cut reads the lower end, and the
/// `2²⁵⁶` arm holds both ends in the format).
#[test]
fn a_box_straddling_the_low_cut_keeps_a_definite_sign() {
    let v = Vec3::new(
        Interval::from_bounds(1e-160, 1.0),
        Interval::from_f64(0.0),
        Interval::from_f64(0.0),
    );
    let old = v / v.norm();
    assert!(old.x.lo() > 0.0, "precondition: {:?}", old.x);
    assert!(v.normalize().x.lo() > 0.0, "{:?}", v.normalize().x);
}

/// At the dual scalar the tangent is scaled with the value; inside the
/// unscaled range it is not scaled at all. Value `2⁻⁵⁰⁵`, derivative
/// `2⁵⁰⁰`: the true `dx̂/dt` is `≈ 1.2e302`, finite.
#[test]
fn the_derivative_channel_survives_inside_the_unscaled_range() {
    let (a, da) = (pow2(-505), pow2(500));
    let v = Vec3::new(Dual::new(a, da), Dual::new(a, 0.0), Dual::new(0.0, 0.0));
    let old = (v / v.norm()).x.deriv;
    assert!(old.is_finite(), "precondition: {old:e}");
    let new = v.normalize().x.deriv;
    assert_eq!(new.to_bits(), old.to_bits(), "{new:e} vs {old:e}");
}

/// The symbolic tier's `‖v̂‖ = 1` theorem fires on the prescaled arms
/// too: the branch changes the term's shape, not what rule E can fold.
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
        for (x, y, z) in [
            (3e200, 4e200, 12e200),
            (3e160, 4e160, 12e160),
            (3e-170, 4e-170, 12e-170),
            (3e-300, 4e-300, 12e-300),
        ] {
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
