---
id: knot-insertion-can-round-a-weight-to-zero
kind: issue
title: Knot insertion (and degree elevation) can round a weight to zero; the plan checks its input weights but not the ones it writes
status: open
opened: 2026-10-12
priority: P3
cost: E
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
