---
id: equal-split-points-insert-a-value-twice-on-a-narrow-span
kind: issue
title: geom-core: equal_split_points drops a point that rounds onto a span end but inserts one that rounds onto another split point, so a narrow span refuses a refinement that is only a tightening
status: open
opened: 2026-10-20
priority: P3
cost: M
---


(Filed by the ENCL implementer of `offset-fit-and-patch-bound-kernel-defect-endings-split-by-class`, from its sweep.)

## What

`geom_core::spline::algebra::equal_split_points` (`crates/geom-core/src/spline/algebra.rs`) skips a split point that floating point rounds onto a span end, because "refinement is a tightening, never a correctness condition, so a dropped point costs a wider piece and never an invalid one". It does not skip a point that rounds onto another split point of the same span. On a span a few ulps wide (a valid face's `[1e15, 1e15 + 1]`, or `½, ½ + 3 ulp`) the points collide with each other, and the chain inserts one value several times:

- past the direction's degree, `refine_plan` refuses (`KnotAlgebraError::MultiplicityOverflow`), which geom-brep reports as `PatchBoundError::RefinementFailed`;
- up to the degree, the refined vector carries a knot of full multiplicity, its first derivative is discontinuous, and geom-brep reports `PatchBoundError::DerivedKnots`.

Both are pinned in geom-brep: `patch_bound.rs` `colliding_split_points_do_not_zero_the_second_partial` and `tests/offset_fit.rs` `a_knot_span_a_few_ulps_wide_refuses_on_its_span`. Both refusals now end in the narrow span's lever (`geom_brep::recourse::NARROW_SPAN_RECOURSE`), which is honest but asks the user to fix what is only the schedule's rounding.

## Consumers

- `patch_bound.rs` `refine_split` / `integral_cells_refined`
  (`TensorChannels::refine_equal_split`): the two arms above.
- `offset_fit.rs` `seed_direction` calls `equal_split_points(kv, 3)`
  directly. On a two-ulp span (`[1, 1 + 2ε]`) it seeds
  `[1, 1+ε, 1+ε, 1+2ε]`, and `interpolate_columns`' `check_ordered`
  refuses: `OffsetFitError::Fit(ParamCountMismatch)`, pinned by the
  "repeated seed" row of `a_knot_span_a_few_ulps_wide_refuses_on_its_span`.

## Repair shape

Drop a split point equal to the previous one kept in its span, as a
point equal to an end is dropped: the same argument covers it. What
moves once that lands:

- **The patch-bound narrow-span arms become bug-only.** The refinement
  never inserts a value twice, so `RefinementFailed` is unreachable,
  and the refined vector keeps the C¹ gate's multiplicities, so
  `DerivedKnots` is too. Per `[failure]` both become panics (or the
  variants go), and their rows in `patch_bound.rs` and
  `tests/offset_fit.rs` move to `#[should_panic]`.
- **The seed arm changes.** `seed_direction` no longer seeds a repeated
  parameter, so `Fit(ParamCountMismatch)` from the seed goes. The
  two-ulp span then seeds three distinct parameters, `[1, 1+ε, 1+2ε]`,
  and refuses `TooFewPoints` instead; its door-test row moves with it.
- **`Fit(TooFewPoints)` stays input-reachable.** A direction holding
  fewer than four `f64` values (one or two one-ulp spans) has no fourth
  parameter to seed, however the schedule rounds. It keeps the narrow
  span's lever.

`patch_bound.rs` `piece_extent` restates the schedule's rounding for
the rational arm's pieces; check it still agrees.
