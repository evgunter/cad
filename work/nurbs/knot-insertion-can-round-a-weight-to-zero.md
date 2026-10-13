---
id: knot-insertion-can-round-a-weight-to-zero
kind: issue
title: Knot insertion (and degree elevation) can round a weight to zero; the plan checks its input weights but not the ones it writes
status: closed
opened: 2026-10-12
priority: P3
cost: E
branch: nurbs/insert-weight-collapse
closed: 2026-10-12
pr: 4923
---


## What

`crates/geom-core/src/spline/algebra.rs:632` (`insert_knot_plan`)
forms each inserted weight as `wq = α·w_j + (1−α)·w_{j−1}` and stores
it without a check. `check_weights` (`algebra.rs:527`) runs on the
plan's **input** only. On a net of the least subnormal weight
(`f64::from_bits(1)`), a midpoint insertion rounds both halves to
zero (a tie, to even), so `wq = 0` and `λ = 0/0` poisons the point.
The curve then reaches `from_validated_parts` (`crates/geom/src/curves/nurbs.rs`,
`apply_plans`) holding a weight its constructor refuses.

The pin is `geom/tests/curves/speed_upper_bound.rs`
`a_weight_collapsed_by_refinement_reads_no_finite_bound`. It asserts
that the refined weight channel holds `0.0`, and that
`speed_upper_bound` reads non-finite there: it divides by a zero least
weight and reads `+∞` or NaN, never a low number. So the speed bound
is safe, but every other reader of the refined curve sees a weight
the type's invariant says cannot exist.

**The same shape elsewhere:**
- **Surface insertion and refinement** (`NurbsSurface::insert_knot_u`
  and `refine_knots_u`, `crates/geom/src/surfaces/nurbs.rs:1147`) run
  the same `insert_knot_plan` per `u` column (`map_u_columns`), and
  `v` by transpose. `refine_plan` is a fold of `insert_knot_plan`.
- **Degree elevation** (`elevate_plan`, `algebra.rs:1239`) inserts to
  Bézier form through `insert_knot_plan`. Its
  `elevate_bezier_stage` (`algebra.rs:1312`) then forms
  `wq = c·w_{i−1} + (1−c)·w_i` with the same missing check.
- **Removal** already refuses this as
  `KnotAlgebraError::WeightCollapse` (`remove_knot_plan`), so the fix
  has a typed error to reuse.

## Shape of the fix

Refuse a written weight that is not strictly positive and finite, as
removal does, in `insert_knot_plan` and `elevate_bezier_stage`. Then
retarget the pin to the refusal.

## Closed (2026-10-12)

Every plan writer now admits what it writes through one check,
`written_weight` in `spline/algebra.rs` (strictly positive and finite,
or `WeightCollapse` at the slot):
- `insert_once`, which serves insertion, refinement and elevation's
  Bézier decomposition, and the surface twins per column;
- `elevate_bezier_stage`;
- removal's two chains, whose inline guards it replaces.

The error's message now covers every writer, not only removal.

Rows:
- `a_blend_that_rounds_a_weight_to_zero_is_refused` (the plans, in
  `algebra.rs`);
- `a_knot_operation_that_would_write_a_zero_weight_is_refused` (the
  curve doors);
- `a_surface_knot_operation_that_would_write_a_zero_weight_is_refused`
  (insertion, refinement and elevation in `u` and `v`).

Each goes red with the insertion guard removed, and again with the
elevation guard removed. The pin
`a_weight_collapsed_by_refinement_reads_no_finite_bound` is deleted:
the insertion it read is now refused, so its speed-bound half has no
curve to read.

The review's fix pass (same PR) folded the predicate into one home.
`geom_core::spline::weight_in_regime` (strictly positive and finite) and
`check_weight_values` (the constructors' named refusal) are now what
these read:
- `written_weight`, `check_weights`, `compose`'s and `compose/tensor`'s
  constructors, and `hull`'s licence;
- `geom`'s `validate_counts` and the speed meter's rational guard;
- `geom-brep`'s `patch_bound` and `pcurve_cache` guards;
- `mesh`'s two chord guards.

Further changes in that pass:
- The `from_validated_parts` premise in `geom/src/scalar_lift.rs` names
  the plan path.
- Every door that reaches a plan names `WeightCollapse` in its
  `# Errors`, removal's reinsertion included.
- The surface row pins the index.
- `a_subnormal_least_weight_reads_no_low_bound` keeps the speed-bound
  neighbour the deleted pin had guarded.

