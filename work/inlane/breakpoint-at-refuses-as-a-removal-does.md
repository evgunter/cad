---
id: breakpoint-at-refuses-as-a-removal-does
kind: issue
title: plan_inputs::breakpoint_at refuses as a removal does, though its name promises a lookup
status: open
opened: 2026-10-19
priority: P3
---


Found by reviewer A's delta-read of C6 S5b (PR 5322). The INLANE orchestrator ruled it to its own item.

## Finding

`spline::plan_inputs::breakpoint_at` (`crates/geom-core/src/spline/plan_inputs.rs:194`) certifies `u` with a removal's refusal past a wide domain end (`plan_inputs.rs:198`: `KnotAlgebraError::KnotNotPresent`).

Over a last run `[0.99, 1.0]`, `u = 1.0` therefore answers `KnotNotPresent`. Yet for the member whose end is `1.0`, `u` is that member's end breakpoint. A lookup named for what it returns ("the held breakpoint `u` names") answers instead as a removal would.

This is harmless in its one caller, `remove_knot`'s reinsertion (`crates/geom/src/curves/nurbs.rs:985`), because `remove_knot_plan` has already certified the same `u` with the same refusal (`plan_inputs.rs:157`).

## Owed

One of:
- **Rename it to its role** (e.g. `reinserted_breakpoint`) and keep it `pub(crate)`-narrow if the geom caller allows.
- **Give it a lookup's own answer** past a wide end: undecided, since some members have `u` as a breakpoint and others do not.

Either way, add a row that pins the chosen answer at a wide end.
