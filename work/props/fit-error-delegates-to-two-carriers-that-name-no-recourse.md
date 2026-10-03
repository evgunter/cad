---
id: fit-error-delegates-to-two-carriers-that-name-no-recourse
kind: issue
title: FitError's Lsq and KnotAlgebra arms delegate to LsqError and KnotAlgebraError, neither of which names a repair
status: open
opened: 2026-09-21
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
