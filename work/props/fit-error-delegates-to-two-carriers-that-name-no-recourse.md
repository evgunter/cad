---
id: fit-error-delegates-to-two-carriers-that-name-no-recourse
kind: issue
title: FitError's Lsq and KnotAlgebra arms delegate to LsqError and KnotAlgebraError, neither of which names a repair
status: review
opened: 2026-09-21
branch: props/recourse-grammar
---


(FIX implementer, filed from the second-hop recourse unit, which took
`FitError` and could not honestly close the chain behind two of its
arms.)

## What

`FitError` (`crates/geom/src/curves/fit.rs`) now carries
`every_fit_error_arm_names_a_recourse`: each of its seven own arms
names a repair, and `Structure` is asserted TRANSITIVELY because
`SplineError` has an enforcement row of its own
(`crates/geom-core/src/spline/knots.rs`).

Two arms are asserted as **delegations only**, which is all that is
honest about them:

- `Lsq(LsqError)` — `crates/geom-core/src/linalg/lsq.rs`. Its
  renderings state the condition (a degenerate pivot at an elimination
  step, a row-length mismatch, an RHS shape mismatch, a shape that
  rules out the requested solve) and name no repair.
- `KnotAlgebra(KnotAlgebraError)` — `crates/geom-core/src/spline/algebra.rs`.
  Its `Structure` arm delegates further to `SplineError` (sound as of
  the unit that filed this); its own arms state the condition.

Both were read, not verb-matched. Re-derive the counts before taking
the row: the parent class found its counts wrong twice.

## Why it matters here

`FitError::Lsq` and `FitError::KnotAlgebra` contribute five characters
(`"fit: "`) over the carried message, so whatever the carrier fails to
say is simply absent from what a user reads — the class the parent row
of this chain states. The fit door is public API
(`NurbsCurve3::interpolate`, `::approximate`), so these messages are
read by callers.

## What this needs

The shape is established: per carrier, ground each repair in the
module's or the variant's own docs, add
`every_<carrier>_arm_names_a_recourse`, prove it red by mutation, then
turn `every_fit_error_arm_names_a_recourse`'s two delegation
assertions into transitive ones.

## Fence

Both files are **PROPS's**.

## Resolved (props/recourse-grammar)

Counts re-derived as the row asks. `LsqError` has five arms and
`KnotAlgebraError` six (one of them `Structure`, delegating further to
`SplineError`, which has its own enforcement row); every one of the
eleven stated its condition and stopped.

Each now names a repair, grounded in the variant's or the module's own
doc:

- `LsqError::LsqDegenerate` — supply rows that determine the unknowns; a
  zero pivot means two do not, a non-finite one means a value upstream
  is not a number (the variant's doc: "including the NaN both become on
  non-finite input"). No reorder is offered: D9, no magnitude pivoting.
- `RowLengthMismatch`, `RhsShapeMismatch`, `Empty` — supply the shape
  the solve takes.
- `Underdetermined` — supply at least as many rows as columns, with the
  variant's own note that `solve_square` reuses it for any non-square
  shape.
- `KnotAlgebraError::ParameterOutsideDomain`, `KnotNotPresent`,
  `MultiplicityOverflow` (which now names how many more copies are left),
  `RemovalExceedsMultiplicity` — ask for a value the knot vector admits.
- `WeightCollapse` — drop this removal; the curve the chain would leave
  is not a valid rational one.

Two enforcement rows added, proved red by mutation before they were
accepted: `every_lsq_error_arm_names_a_recourse` and
`every_knot_algebra_error_arm_names_a_recourse`, each counting exactly
one labelled repair per arm through
`test_utils::refusal::recourse_markers` plus a vocabulary floor.

`every_fit_error_arm_names_a_recourse`'s two DELEGATION assertions are
now TRANSITIVE, and it additionally reads that the carrier's repair
survives into the message the public fit door's caller sees — `"fit: "`
adds no second marker.
