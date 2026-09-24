---
id: net-refinement-copies-the-differencing-skeleton-and-the-plan-ratio-is-optional
kind: issue
title: geom-core: refine_u/refine_v copy diff_u/diff_v's per-line skeleton, and Step::Combo.ratio holds an invariant by convention
status: open
opened: 2026-09-22
---


Filed by TESS-2 from a blinded review of its head, on PROPS' slate
because `crates/geom-core/src/spline/` is this program's ground. Two
findings in one row: both are about the shape TESS-2 added there, and
both are cosmetic-plus (one is a duplication, one is a type that lets a
runtime refusal stand in for a compile-time one).

## 1. Four per-line loops where one would do

`TensorNet` now carries four methods with one skeleton: `diff_u`,
`diff_v` (TESS-2 did not add these) and `refine_u`, `refine_v` (it did).
Each walks the lines of one direction, applies a step to each line, and
scatters the answer back into a net of the new extent, poisoning what it
cannot fill. They differ in three things: which direction's lines
(`column(j)` versus `row(i)`), the new extent (`n - 1` for differencing,
the plan chain's count for refinement), and the step (a caller's closure
versus a fold of `apply_ring`).

All three are parameters, so one

```rust
fn map_lines_u(&self, n_new: usize, step: impl Fn(&[RingInterval]) -> Vec<RingInterval>) -> Self
```

plus its `v` twin would host all four, with the four public methods
becoming two-line wrappers that name their extent rule. The scatter
arithmetic (`i * nv + j` for `u`, `i * nv_new + j` for `v`) is where a
transposed index would hide, and it is written out four times today.

TESS-2's own rows already cover the behaviour
(`refinement_is_the_per_line_chain_in_each_direction` pins that each
direction's answer is its line's own chain, with per-slot distinct
values so a transposed scatter cannot pass), so this is a
consolidation with its tests already in place.

## 2. `Step::Combo.ratio: Option<Ratio>` is an invariant by convention

`CurvePlan::apply_ring` is defined only for an INSERTION plan: removal
and degree elevation combine with coefficients that are not ratios of
knots, so their steps carry `ratio: None` and the applier poisons the
target. That is fail-loud and correct, but the thing it is protecting
against is a caller handing it the wrong KIND of plan — which a type
could refuse instead.

The shape that would: a `RefinementPlan` (or an `InsertionPlan`) that
`insert_knot_plan` and `refine_plan` return and `remove_knot_plan` /
`elevate_plan` do not, carrying `Ratio` unconditionally, with
`apply_ring` a method on it. `apply_ring` becomes total in the strong
sense — no arm of it can refuse — and `Option` leaves `Step::Combo`
entirely. The cost is a second plan type and a conversion for the
`apply_points` path, which all four kinds share.

Worth doing when something else touches `algebra`'s plan types; not
worth a PR of its own, and TESS-2 deliberately did not widen its fence
to it.
