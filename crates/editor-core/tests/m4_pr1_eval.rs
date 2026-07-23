//! Evaluator behavior (spec D4): scalar-generic over `Real`, units
//! erased at the boundary (GQ5), typed environment errors, and the
//! pinned Interval instantiation (feature `interval`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{Dimension, EvalError, Expr, ParamEnv, ParamName, ParamValue, eval, eval_count};

fn env_with(name: &str, v: ParamValue<f64>) -> ParamEnv<f64> {
    let mut env = ParamEnv::default();
    env.bindings.insert(ParamName::new(name), v);
    env
}

#[test]
fn param_lookup_and_typed_failures() {
    let depth = Expr::param(ParamName::new("depth"), Dimension::Length);
    // Bound correctly: the raw kernel-unit value comes back.
    let env = env_with(
        "depth",
        ParamValue::Continuous {
            dim: Dimension::Length,
            value: 0.002,
        },
    );
    assert_eq!(eval(&depth, &env).unwrap(), 0.002);
    // Unbound: typed.
    assert_eq!(
        eval(&depth, &ParamEnv::<f64>::default()).unwrap_err(),
        EvalError::UnknownParam(ParamName::new("depth"))
    );
    // Bound at a different dimension: typed.
    let wrong = env_with(
        "depth",
        ParamValue::Continuous {
            dim: Dimension::Angle,
            value: 0.002,
        },
    );
    assert_eq!(
        eval(&depth, &wrong).unwrap_err(),
        EvalError::ParamDimensionMismatch {
            name: ParamName::new("depth"),
            expected: Dimension::Length,
            found: Dimension::Angle,
        }
    );
}

#[test]
fn count_param_is_exact_i64() {
    let n = Expr::param(ParamName::new("n"), Dimension::Count);
    let env = env_with("n", ParamValue::Count(7));
    assert_eq!(eval_count(&n, &env).unwrap(), 7);
    // A Count param under continuous eval is a typed refusal.
    assert_eq!(
        eval(&n, &env).unwrap_err(),
        EvalError::CountExprInContinuousEval
    );
}

#[test]
fn count_to_scalar_range_guard() {
    // Within i32 range the promotion is exact (i32::try_from +
    // f64::from — ruled at the review, replacing the ±2^53 guard)…
    let ok = Expr::count_to_scalar(Expr::count(i64::from(i32::MAX))).unwrap();
    assert_eq!(
        eval(&ok, &ParamEnv::<f64>::default()).unwrap(),
        2_147_483_647.0
    );
    // …outside it: typed refusal.
    let too_big = Expr::count_to_scalar(Expr::count(i64::from(i32::MAX) + 1)).unwrap();
    assert_eq!(
        eval(&too_big, &ParamEnv::<f64>::default()).unwrap_err(),
        EvalError::CountToScalarOutOfRange(i64::from(i32::MAX) + 1)
    );
    // Regression (review r2): i64::MIN must be the SAME typed error,
    // never a panic (the old guard called i64::abs first).
    let min = Expr::count_to_scalar(Expr::count(i64::MIN)).unwrap();
    assert_eq!(
        eval(&min, &ParamEnv::<f64>::default()).unwrap_err(),
        EvalError::CountToScalarOutOfRange(i64::MIN)
    );
}

#[test]
fn arithmetic_matches_f64_semantics() {
    // (2 m + 3 m) * 0.5 - 1 m = 1.5 m — plain f64 arithmetic, units
    // erased (GQ5: eval returns raw kernel units).
    let e = Expr::sub(
        Expr::mul(
            Expr::add(
                Expr::literal(2.0, Dimension::Length).unwrap(),
                Expr::literal(3.0, Dimension::Length).unwrap(),
            )
            .unwrap(),
            Expr::literal(0.5, Dimension::Scalar).unwrap(),
        )
        .unwrap(),
        Expr::literal(1.0, Dimension::Length).unwrap(),
    )
    .unwrap();
    assert_eq!(eval(&e, &ParamEnv::<f64>::default()).unwrap(), 1.5);
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
            let e = Expr::mul(
                Expr::add(
                    Expr::literal(a, Dimension::Length).unwrap(),
                    Expr::literal(b, Dimension::Length).unwrap(),
                )
                .unwrap(),
                Expr::literal(k, Dimension::Scalar).unwrap(),
            )
            .unwrap();
            let got = eval(&e, &ParamEnv::<f64>::default()).unwrap();
            prop_assert_eq!(got.to_bits(), ((a + b) * k).to_bits());
        }

        /// Count arithmetic is exact integer arithmetic wherever i64
        /// does not overflow (spec D4).
        #[test]
        fn count_arithmetic_is_exact(a in -1_000_000i64..1_000_000, b in -1_000_000i64..1_000_000) {
            let sum = Expr::add(Expr::count(a), Expr::count(b)).unwrap();
            let prod = Expr::mul(Expr::count(a), Expr::count(b)).unwrap();
            prop_assert_eq!(eval_count(&sum, &ParamEnv::<f64>::default()).unwrap(), a + b);
            prop_assert_eq!(eval_count(&prod, &ParamEnv::<f64>::default()).unwrap(), a * b);
        }
    }
}

/// The pinned Interval instantiation (spec D4/D8): the evaluator is
/// generic over `Real` with no branches, so the certified scalar runs
/// the SAME code path and must enclose the f64 result.
#[cfg(feature = "interval")]
mod interval_lane {
    use super::*;
    use geom_core::Interval;
    use geom_core::real::{Bounds, Real};

    #[test]
    fn interval_instantiation_encloses_f64() {
        // sin(τ/8) * 2 — exercises literal embedding, trig, and
        // arithmetic through the one generic evaluator.
        let e = Expr::mul(
            Expr::sin(Expr::literal(std::f64::consts::FRAC_PI_4, Dimension::Angle).unwrap())
                .unwrap(),
            Expr::literal(2.0, Dimension::Scalar).unwrap(),
        )
        .unwrap();
        let at_f64 = eval::<f64>(&e, &ParamEnv::default()).unwrap();
        let at_interval = eval::<Interval>(&e, &ParamEnv::default()).unwrap();
        assert!(at_interval.lo() <= at_f64 && at_f64 <= at_interval.hi());
        // The enclosure is tight (a point input), not vacuous.
        assert!(at_interval.hi() - at_interval.lo() < 1e-12);
    }

    #[test]
    fn interval_param_env_embeds_exactly() {
        let depth = Expr::param(ParamName::new("d"), Dimension::Length);
        let mut env: ParamEnv<Interval> = ParamEnv::default();
        env.bindings.insert(
            ParamName::new("d"),
            ParamValue::Continuous {
                dim: Dimension::Length,
                value: <Interval as Real>::from_f64(0.003),
            },
        );
        let v = eval(&depth, &env).unwrap();
        assert_eq!(v.lo(), 0.003);
        assert_eq!(v.hi(), 0.003);
    }
}
