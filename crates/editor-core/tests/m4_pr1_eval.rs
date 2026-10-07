//! Evaluator behavior (spec D4): scalar-generic over `Real`, units
//! erased at the boundary (GQ5), typed environment errors, and the
//! pinned Interval instantiation.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the code it tests (TCOST-1). The claim is the evaluator's: a
// scalar-generic `eval`/`eval_count` over `Real`, units erased at the
// boundary, typed environment failures, and an `Interval` instantiation
// that encloses the `f64` answer. It rests on the evaluator itself, on the
// expression algebra and its `Dimension`/`DimensionError` vocabulary, and
// on `geom-core`'s two scalar implementations — a change to `real.rs` or
// `interval.rs` moves what the enclosure row asserts without touching
// `editor-core/`. `quantity/src/` carries the unit table the erasure is
// against, and `src/test_support.rs` the literal doors the rows write
// their operands with.
test_utils::gated_to![
    "crates/editor-core/src/eval/",
    "crates/editor-core/src/expr.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/interval.rs",
    "crates/quantity/src/",
    "crates/editor-core/src/test_support.rs",
];

use editor_core::test_support::{ang, len, scl};
use editor_core::{
    Dimension, EvalError, Formula, ParamValue, VarEnv, VarId, VarName, eval, eval_count,
};

fn env_with(var: VarId, v: ParamValue<f64>) -> VarEnv<f64> {
    let mut env = VarEnv::default();
    env.bindings.insert(var, v);
    env
}

#[test]
fn param_lookup_and_typed_failures() {
    let id = VarId::new(0, 0x3fa9_c1d2_a0b1_0001);
    let depth = Formula::var(id, Dimension::Length);
    // Bound correctly: the raw kernel-unit value comes back.
    let env = env_with(
        id,
        ParamValue::Continuous {
            dim: Dimension::Length,
            value: 0.002,
        },
    );
    assert_eq!(
        eval(&editor_core::test_support::stored_expr(&depth), &env).unwrap(),
        0.002
    );
    // Unbound: typed.
    assert_eq!(
        eval(
            &editor_core::test_support::stored_expr(&depth),
            &VarEnv::<f64>::default()
        )
        .unwrap_err(),
        EvalError::UnresolvedVar { var: id }
    );
    // Bound at a different dimension: typed.
    let wrong = env_with(
        id,
        ParamValue::Continuous {
            dim: Dimension::Angle,
            value: 0.002,
        },
    );
    assert_eq!(
        eval(&editor_core::test_support::stored_expr(&depth), &wrong).unwrap_err(),
        EvalError::VarKindMismatch {
            var: id,
            bound: Dimension::Angle,
            read: Dimension::Length,
        }
    );
    // An authored name never reaches evaluation: a formula reading one
    // has no stored form until a scope resolves it.
    assert_eq!(
        editor_core::Expr::try_from(&Formula::named(
            VarName::from_static("depth"),
            Dimension::Length
        )),
        Err(editor_core::NameFault {
            name: VarName::from_static("depth"),
            dim: Dimension::Length,
            why: editor_core::Unlowered::Unheld,
        })
    );
}

#[test]
fn count_param_is_exact_i64() {
    let n = Formula::var(VarId::new(0, 7), Dimension::Count);
    let env = env_with(VarId::new(0, 7), ParamValue::Count(7));
    assert_eq!(
        eval_count(&editor_core::test_support::stored_expr(&n), &env).unwrap(),
        7
    );
    // A Count param under continuous eval is a typed refusal.
    assert_eq!(
        eval(&editor_core::test_support::stored_expr(&n), &env).unwrap_err(),
        EvalError::CountExprInContinuousEval
    );
}

#[test]
fn count_to_scalar_range_guard() {
    // Within i32 range the promotion is exact (i32::try_from +
    // f64::from — ruled at the review, replacing the ±2^53 guard)…
    let ok = Formula::count_to_scalar(Formula::count(i64::from(i32::MAX))).unwrap();
    assert_eq!(
        eval(
            &editor_core::test_support::stored_expr(&ok),
            &VarEnv::<f64>::default()
        )
        .unwrap(),
        2_147_483_647.0
    );
    // …outside it: typed refusal.
    let too_big = Formula::count_to_scalar(Formula::count(i64::from(i32::MAX) + 1)).unwrap();
    assert_eq!(
        eval(
            &editor_core::test_support::stored_expr(&too_big),
            &VarEnv::<f64>::default()
        )
        .unwrap_err(),
        EvalError::CountToScalarOutOfRange(i64::from(i32::MAX) + 1)
    );
    // Regression (review r2): i64::MIN must be the SAME typed error,
    // never a panic (the old guard called i64::abs first).
    let min = Formula::count_to_scalar(Formula::count(i64::MIN)).unwrap();
    assert_eq!(
        eval(
            &editor_core::test_support::stored_expr(&min),
            &VarEnv::<f64>::default()
        )
        .unwrap_err(),
        EvalError::CountToScalarOutOfRange(i64::MIN)
    );
}

#[test]
fn arithmetic_matches_f64_semantics() {
    // (2 m + 3 m) * 0.5 - 1 m = 1.5 m — plain f64 arithmetic, units
    // erased (GQ5: eval returns raw kernel units).
    let e = Formula::sub(
        Formula::mul(Formula::add(len(2.0), len(3.0)).unwrap(), scl(0.5)).unwrap(),
        len(1.0),
    )
    .unwrap();
    assert_eq!(
        eval(
            &editor_core::test_support::stored_expr(&e),
            &VarEnv::<f64>::default()
        )
        .unwrap(),
        1.5
    );
}

mod props {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// eval is exactly f64 arithmetic on the erased values (GQ5:
        /// units erase; D9: bit-identical determinism) — pinned over
        /// arbitrary finite literals.
        #[test]
        fn literal_arithmetic_is_exact_f64(
            a in -1.0e9f64..1.0e9,
            b in -1.0e9f64..1.0e9,
            k in -1.0e3f64..1.0e3,
        ) {
            let e = Formula::mul(
                Formula::add(
                    len(a),
                    len(b),
                )
                .unwrap(),
                scl(k),
            )
            .unwrap();
            let got = eval(&editor_core::test_support::stored_expr(&e), &VarEnv::<f64>::default()).unwrap();
            prop_assert_eq!(got.to_bits(), ((a + b) * k).to_bits());
        }

        /// Count arithmetic is exact integer arithmetic wherever i64
        /// does not overflow (spec D4).
        #[test]
        fn count_arithmetic_is_exact(a in -1_000_000i64..1_000_000, b in -1_000_000i64..1_000_000) {
            let sum = Formula::add(Formula::count(a), Formula::count(b)).unwrap();
            let prod = Formula::mul(Formula::count(a), Formula::count(b)).unwrap();
            prop_assert_eq!(eval_count(&editor_core::test_support::stored_expr(&sum), &VarEnv::<f64>::default()).unwrap(), a + b);
            prop_assert_eq!(eval_count(&editor_core::test_support::stored_expr(&prod), &VarEnv::<f64>::default()).unwrap(), a * b);
        }
    }
}

/// The pinned Interval instantiation (spec D4/D8): the evaluator is
/// generic over `Real` with no branches, so the certified scalar runs
/// the SAME code path and must enclose the f64 result.
mod interval_lane {
    use super::*;
    use geom_core::Interval;
    use geom_core::real::{Bounds, Real};

    #[test]
    fn interval_instantiation_encloses_f64() {
        // sin(τ/8) * 2 — exercises literal embedding, trig, and
        // arithmetic through the one generic evaluator.
        let e = Formula::mul(
            Formula::sin(ang(std::f64::consts::FRAC_PI_4)).unwrap(),
            scl(2.0),
        )
        .unwrap();
        let at_f64 = eval::<f64>(
            &editor_core::test_support::stored_expr(&e),
            &VarEnv::default(),
        )
        .unwrap();
        let at_interval = eval::<Interval>(
            &editor_core::test_support::stored_expr(&e),
            &VarEnv::default(),
        )
        .unwrap();
        assert!(at_interval.lo() <= at_f64 && at_f64 <= at_interval.hi());
        // The enclosure is tight (a point input), not vacuous.
        assert!(at_interval.hi() - at_interval.lo() < 1e-12);
    }

    #[test]
    fn interval_var_env_embeds_exactly() {
        let depth = Formula::var(VarId::new(0, 3), Dimension::Length);
        let mut env: VarEnv<Interval> = VarEnv::default();
        env.bindings.insert(
            VarId::new(0, 3),
            ParamValue::Continuous {
                dim: Dimension::Length,
                value: <Interval as Real>::from_f64(0.003),
            },
        );
        let v = eval(&editor_core::test_support::stored_expr(&depth), &env).unwrap();
        assert_eq!(v.lo(), 0.003);
        assert_eq!(v.hi(), 0.003);
    }
}
