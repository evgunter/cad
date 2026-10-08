---
id: a-subnormal-weight-face-now-passes-the-meters-refinement
kind: issue
title: A subnormal-weight face now passes the meters' refinement, and offset_fit's Elevation arm calls an underflow a kernel finding
status: open
opened: 2026-10-01
---


Found by REACH while fixing main's red plane x NURBS envelope
(branch `reach/pxn-envelope-red`).

`geom_core::spline::algebra::convex_step` (shared by
`CurvePlan::apply_certified` and `compose`'s `insert_once_ring`) now
meets each Boehm step with the hull of its two sources. A refined weight
enclosure therefore stays inside the described weights' range, and
`patch_bound::rational_cells`' `RefinedWeightLostPositivity` guard is no
longer reached from a door-valid face, even at `f64::from_bits(1)` beside
`1e2` (`patch_bound`'s
`refined_weight_lost_positivity_is_not_reached_from_a_valid_face`). That
arm is now worded as a kernel defect.

What this leaves on ENCL's ground, unmeasured:

- `offset_fit.rs` `OffsetFitError::Elevation`'s docs said a
  subnormal-weight face "refuses earlier, in the meters' rational
  refinement". That sentence is gone. The v elevation reads the weights
  the u elevation produced in `f64`, where a combination of two
  subnormal weights can round to zero, so such a face may now reach
  `Elevation`, whose message calls the refusal a kernel finding and asks
  for a report. For a face whose weights are all near the bottom of the
  `f64` range that is not a defect; the honest recourse is the one the
  patch-bound arm used to give (scale every weight up by one constant,
  which is the same surface).
- Whether any such face gets past the meters at all — the quotient rule
  in `rational_cells` divides by a weight hull of about 5e-324 — was not
  measured.
