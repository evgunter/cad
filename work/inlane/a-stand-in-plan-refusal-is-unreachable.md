---
id: a-stand-in-plan-refusal-is-unreachable
kind: issue
title: KnotAlgebraError::StandInPlan is unreachable: a defensive refusal, or dead code?
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found by both reviewers' delta-reads of C6 S5b (PR 5322). The INLANE orchestrator ruled it to its own item.

## Finding

`PlanChain::projective` (`crates/geom/src/curves/nurbs.rs:740-742`) refuses `KnotAlgebraError::StandInPlan` (`crates/geom-core/src/spline/algebra.rs:120`, Display at `:189`) when a plan in the chain is no reference plan. No input reaches it:
- every projective scalar (`f64`, `Probe`, and `Dual`/`Sym` over them) answers `point_value` from an `f64` value channel;
- `Knots::new` and `Weights::new` refuse poison;
- so a carrier at a projective scalar holds point knots and weights only, and `spline::plan_inputs` makes only reference plans for it.

Both reviewers executed this: deleting the guard leaves every test green, and no row can reach it through the public API.

## Owed

A decision, recorded here:
- **(a) A defensive refusal.** Keep it, and say in its docs that it is unreachable and why.
- **(b) Dead code.** Delete the guard and the variant.

The choice turns on the end state of `the-f64-appliers-poison-where-the-projective-arm-refuses-typed`. If that item makes a stand-in plan a different type, this guard becomes unreachable by type and (b) follows.
