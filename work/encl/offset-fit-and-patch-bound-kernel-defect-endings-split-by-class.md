---
id: offset-fit-and-patch-bound-kernel-defect-endings-split-by-class
kind: issue
title: geom-brep: six offset-fit and patch-bound refusals end in KERNEL_DEFECT_ENDING; two are reachable by input, one is the interval bound's limit, three are bug-only
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---

(ENCL implementer, from the census in
`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`,
on main 5df6e6c854.)

`[failure]` (D2) splits every state in two: one input reaches, which
is a typed refusal whose ending must name the decision's lever, and one
only a kernel bug reaches, which MUST panic ("downgrading one … to a
typed error launders a bug into a supported outcome"). A typed refusal
ending in `geom_core::KERNEL_DEFECT_ENDING` is therefore wrong in either
class. The one exception is a kernel approximation limit, which takes
`[tolerance.recourse]`'s last resort (`KERNEL_LIMIT_RECOURSE` /
`KERNEL_LIMIT_LAST_RESORT`), not this ending.

## Sites

- `crates/geom-brep/src/offset_fit.rs` `OffsetFitError` `Display`,
  `Fit(_)` (~764). Doors: `fit_offset`, `fit_offset_at`,
  `approx_offset_surface[_at]`, and the shell/offset doors through
  `ReplaceFaceError::Fit`. **Class 2, unsure**: the interpolation's
  samples are the refinement schedule's own, over a face whose
  non-finite points refuse first (`NonFiniteSample`). Unsure because a
  face with a collapsed edge was not ruled out as giving coincident
  samples; if it does, this is class 1.
- `offset_fit.rs` `OffsetFitError` `Display`, `Structure(_)` (~769).
  Same doors. **Class 2, likely**: the knots and points are computed
  over the face's validated domain.
- `offset_fit.rs` `OffsetFitError` `Display`, `MintLimb` (~875). Same
  doors. **Class 2, unsure**: the mint's certification re-derives the
  bound the loop accepted on the same fit, so a sample above it is a
  contradiction. The variant's other arm, a round's residual coming out
  NaN, may be overflow on finite input, which would make it class 1.
- `crates/geom-brep/src/patch_bound.rs` `PatchBoundError::note`,
  `RefinementFailed` (~201). Door: `pub fn patch_cells_refined(n,
  splits)`. **Class 1, unsure**: any `NurbsSurface` and any `splits`
  reach `refine_split`. An insertion chain refusing on a span a few ulps
  wide (the collision `DerivedKnots` documents) was not ruled out.
- `patch_bound.rs` `PatchBoundError::note`, `QuotientRefused` (~211).
  Doors: `patch_cells`, `patch_cells_refined` (the rational arm).
  **Class 3, unsure**: the quotient statement refusing under its degree
  ceiling is the interval enclosure's own limit on legal but extreme
  weights.
- `patch_bound.rs` `PatchBoundError::note`, `DerivedKnots` (~216).
  Doors: `patch_cells_refined`, and `offset_fit`'s integral lane
  (~2204, ~2215). **Class 1, sure**: the variant's own doc says that a
  valid face with a span a few ulps wide, cut by the equal-split
  schedule, reaches it. Its lever is the knot span (merge or widen it).

`patch_bound.rs`'s `every_patch_bound_error_arm_names_a_recourse`
pins all three arms to the defect ending, and has to move with them.

## Repair shape

- **Class 2** (only a kernel bug reaches it): `unreachable!` or
  `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0
  first (a type that cannot hold the state is preferred). The tests
  that pin the refusal's text move to `#[should_panic]`, or are deleted
  where they only built the error value by hand.
- **Class 1** (input reaches it): the decision's own lever, written in
  its own words (`[tolerance.recourse]` (i)), and a test that reaches it
  through the door.
- **Class 3** (an approximation limit): the last-resort recourse.

Re-check each `likely` or `unsure` argument before acting on it: one
that fails moves the site to class 1.
