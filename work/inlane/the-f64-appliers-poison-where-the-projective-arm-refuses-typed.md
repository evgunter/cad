---
id: the-f64-appliers-poison-where-the-projective-arm-refuses-typed
kind: issue
title: A stand-in plan is refused two ways: the f64 appliers answer poison or NaI, the projective arm refuses typed
status: open
opened: 2026-10-19
priority: P2
---


Found by reviewer B's delta-read of C6 S5b (PR 5322). The INLANE orchestrator ruled it to its own item at P2.

## Finding

A plan decided against a stand-in (`CurvePlan::is_reference() == false`, `crates/geom-core/src/spline/algebra.rs:341`) is refused in two shapes:
- **The `f64` appliers answer in-band.** `CurvePlan::apply_points` (`algebra.rs:373`, the check at `:380`) returns a poison polygon. `CurvePlan::apply_certified` (`algebra.rs:558`, the check at `:559`) returns NaI. A caller that does not check the result reads a value.
- **The projective arm refuses typed.** `PlanChain::projective` (`crates/geom/src/curves/nurbs.rs:740-742`) returns `KnotAlgebraError::StandInPlan`.

Both refuse the same condition: a plan whose `f64` values are a stand-in's.

## What the S5z end state wants

The ruled S5 end state is typed-refusing accessors in place of loud ones. For this refusal, that points past both shapes, to making it **unrepresentable**:
- split the plan by type, so that a reference plan (point inputs, `f64` values its own) and a held plan (decided over held knots and weights, structure and schedule only) are distinct types;
- only the reference plan exposes `apply_points`, `apply_certified` and `weights()`;
- both carry `apply_rational` and `apply_homogeneous`, which read the held values.

The poison and NaI answers, and the `StandInPlan` guard (`a-stand-in-plan-refusal-is-unreachable`), then go away rather than being reconciled.

If a type split costs more than S5z can carry, the fallback is the typed shape everywhere: the `f64` appliers return `Result<_, KnotAlgebraError>` with `StandInPlan`. The in-band poison stays only for a malformed plan, which the constructors rule out.

## Owed

The type split, or the typed fallback, by S5z. Rows: a stand-in plan cannot reach an `f64` applier. With the split, that is a compile-fail doc test; with the fallback, an `Err` row.
