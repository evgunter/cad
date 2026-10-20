---
id: offset-fit-and-patch-bound-kernel-defect-endings-split-by-class
kind: issue
title: geom-brep: six offset-fit and patch-bound refusals end in KERNEL_DEFECT_ENDING; two are reachable by input, and the other four are bug-only or not yet shown to be
status: closed
closed: 2026-10-20
branch: encl/patch-bound-endings
pr: 5476
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

## Disposition

(ENCL implementer, 2026-10-20, on main c032d6377f.)

- `RefinementFailed`, `DerivedKnots`: **class 1**, one decision, the
  narrow knot span (`recourse::NARROW_SPAN`): both end in
  `NARROW_SPAN_RECOURSE` ("widen the face's narrow knot spans, or move
  its parameter range nearer zero"). `DerivedKnots`' two raisers in `offset_fit.rs`
  `Composite::build` were **class 2**: they read the base's own vector
  right after `check_direction`, which refuses a discontinuous first
  derivative as a crease; they are `unreachable!` now, and the variant
  is raised only on a refined vector. Reached through the door by
  `tests/offset_fit.rs` `a_knot_span_a_few_ulps_wide_refuses_on_its_span`.
- `QuotientRefused`: **class 2**, the variant is gone. Every `None` of
  `cell_second_quotients` and `polynomial_area_pieces` is a degree-0
  axis, a block not `(p + 1)(q + 1)` long, or a product past
  `QUOTIENT_MAX_DEGREE` (`spline/derivative.rs` `cell_whole`,
  `product`, `Grid::new`, `raised_to`); the C¹ gate, the span window and
  the ceiling checks before each call exclude all three.
- `Fit(_)`: **class 1** at `FitError::TooFewPoints`, falsified: a valid
  face on `[4e15, 4e15 + 1]` with an interior knot has two one-ulp
  spans, and the seed's points round onto their ends (three distinct
  parameters for a cubic's four). `ParamCountMismatch` too (review of
  PR 5476): a rational degree-1 face on `[1, 1 + 2ε]` seeds
  `[1, 1+ε, 1+ε, 1+2ε]`, and the fit's counts always pair, so only its
  ordering half is left. Both end in the narrow span's lever. Every
  other arm ends in `KERNEL_LIMIT_RECOURSE`.
- `Structure(_)`: **unproven**, ends in `KERNEL_LIMIT_RECOURSE`. A far
  domain does not reach it: `[1e15, 1e15 + 1]` refuses earlier on the
  narrow span (the test above), and a direct probe of
  `interpolate_columns` + `KnotVector::on_domain` over 20 000 random
  parameter sets on the `[1e15, 1e15 + 8]` grid collapsed none.
- `MintLimb`: **unproven**, ends in `KERNEL_LIMIT_RECOURSE`. Not reached
  by a quarter cylinder shifted to 1e3 … 1e9 at tolerances 1e-4 …
  1e-10, nor scaled to 1e300 … 4e307 (the meters refuse first).

## Open questions

Moved to their own rows: `offset-fit-unproven-refusals-end-in-the-last-resort`
(`Structure`, `Fit` past its narrow-span arms, `MintLimb`) and
`narrow-span-refusals-do-not-say-which-span` (the location the lever
needs). What the NURBS fix would move is in
`nurbs/equal-split-points-insert-a-value-twice-on-a-narrow-span`.
