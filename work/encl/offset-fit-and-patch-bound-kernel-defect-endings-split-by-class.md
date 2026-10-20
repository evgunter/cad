---
id: offset-fit-and-patch-bound-kernel-defect-endings-split-by-class
kind: issue
title: geom-brep: six offset-fit and patch-bound refusals end in KERNEL_DEFECT_ENDING; two are reachable by input, and the other four are bug-only or not yet shown to be
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

Doors for the `OffsetFitError` arms: `fit_offset`, `fit_offset_at`,
`approx_offset_surface[_at]`, and the shell/offset doors through
`ReplaceFaceError::Fit`. Doors for the `PatchBoundError` arms:
`patch_cells`, `pub fn patch_cells_refined(n, splits)`,
`integral_area_floor` (`patch_bound.rs` ~938), `offset_fit`'s integral
lane (~2204, ~2215), `offset_meters.rs` (`MeterError::PatchBound`,
~807), and mesh `nurbs_cert.rs` `face_err` (~151), which imported NURBS
faces reach.

- `crates/geom-brep/src/offset_fit.rs` `OffsetFitError` `Display`,
  `Fit(_)` (~764). **Class 2, unsure**: the samples are the refinement
  schedule's own, over a face whose non-finite points refuse first
  (`NonFiniteSample`). The live risks are the parameters collapsing
  under rounding in `normalized`, and a solve whose pivot is not
  certified. Either one would make this class 1.
- `offset_fit.rs` `OffsetFitError` `Display`, `Structure(_)` (~769).
  **Class 2, unsure, possibly class 1**: `KnotVector::on_domain(lo, hi)`
  (geom-core `spline/knot_values.rs`) refuses breakpoints that rounding
  collapses. A legal face on a domain far from the origin (u in
  [1e15, 1e15 + 1]) may collapse the averaged knots, which is
  `DerivedKnots`' family. **Before any panic, prove that rounding cannot
  collapse them; otherwise the site is class 1.**
- `offset_fit.rs` `OffsetFitError` `Display`, `MintLimb` (~875).
  **Class 2, unsure**: the mint's certification re-derives the bound the
  loop accepted on the same fit, so a sample above it is a
  contradiction. The variant's other arm, a round's residual coming out
  NaN, may be overflow on finite input, which would make it class 1.
- `crates/geom-brep/src/patch_bound.rs` `PatchBoundError::note`,
  `RefinementFailed` (~201). **Class 1, sure**: the test
  `colliding_split_points_do_not_zero_the_second_partial` records it on
  a valid face. That falsifies the variant doc's "insertion into a
  valid clamped vector is total" (~150), and the doc goes with the
  repair.
- `patch_bound.rs` `PatchBoundError::note`, `QuotientRefused` (~211).
  **Class 2, likely**: every `None` of `cell_second_quotients` and
  `polynomial_area_pieces` is structural (geom-core
  `spline/derivative.rs`), and the gates before the call exclude each
  one.
- `patch_bound.rs` `PatchBoundError::note`, `DerivedKnots` (~216).
  **Class 1, sure**: the variant's own doc says a valid face reaches it,
  when a span a few ulps wide is cut by the equal-split schedule. Its
  lever is the knot span (merge or widen it).

`patch_bound.rs`'s `every_patch_bound_error_arm_names_a_recourse` pins
all three arms to the defect ending, and has to move with them.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.
- **Class 1**: the decision's own lever, in its own words (`[tolerance.recourse]` (i)), with a test that reaches it through the door.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.
