---
id: a-subnormal-weight-face-now-passes-the-meters-refinement
kind: issue
title: A subnormal-weight face now passes the meters' refinement, and offset_fit's composite reads its weight underflow as an unbounded cell
status: review
opened: 2026-10-01
priority: P3
branch: encl/subnormal-weight-face
cost: E
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

- `offset_fit.rs` `Composite::build` raises a degree-1 base direction
  in the ring (`PatchSpans::elevated`), not at `f64`, so the
  `OffsetFitError::Elevation` arm is gone (PR 4616). The underflow it
  could report has moved, not gone. The raised weight coefficients are
  binomial-quotient combinations of the described weights, rounded
  outward, so on a face whose weights are subnormal a coefficient's
  lower end can round to `0`. Even without a raise, `wt = w·w_fit`
  and the `w³` in `cell_terms` underflow the same way. `cell_terms`
  then refuses the cell (`!(w_lo > 0.0)`, and the same guard on `wt`)
  and returns `None`. `cell_bound` reads that as `+∞`, and the loop
  ends `OffsetFitError::BoundNotFinite` with `best: None`. That arm's
  recourse points at the regularity floor and `|d|`. The honest
  recourse is the one the patch-bound arm used to give: scale every
  weight up by one constant, which describes the same surface. So the
  row is still a real issue, a refusal that names the wrong lever. Its
  arm moved, and it is still unmeasured end to end.
- Whether any such face gets past the meters at all — the quotient rule
  in `rational_cells` divides by a weight hull of about 5e-324 — was not
  measured. If none does, the bullet above is unreachable and the row
  closes on that measurement.

## Measured

The rational quarter cylinder (weights `1, 1, s, s, 1, 1`) at
`d = 0.3`, `tolerance = 1e-3`, every weight times one power of two:

| scale | meters | `fit_offset_at` |
|---|---|---|
| `2⁰` … `2⁻¹⁰⁰` | pass | certifies, 3 rounds |
| `2⁻²⁰⁰` | pass | `WallReached` (sample cap), bound `9.6e159` |
| `2⁻³⁰⁰` | pass | `BoundNotFinite { best: None }` |
| `2³⁰⁰` | `Escalated` on `NormalFloor`, margin `Invalid` | the meters' |
| `2⁻¹⁰⁰⁰` | `NormalFloor`, floor read with speed lever `6.8e139` | the meters' |

So the bullet above is reachable: a uniformly scaled face passes the
meters and ends `BoundNotFinite` with no bound behind it, and the
meters themselves misread the scale at both ends of the range. A spread
face (`f64::from_bits(1)` beside `1e2`, or beside `1`) never reaches the
composite: the meters refuse it on `NormalFloor` with a floor of `0`,
which is the chart's own degeneracy at that spread.

## Resolution

The fit rescales the weights itself, exactly:
`patch_bound::unit_scaled_weights` multiplies every weight by the one
power of two that carries the largest into `[1, 2)`, and reads the net
as described when that would drop a bit. `rational_cells` and
`Composite::build` (base and fit) read the rescaled net. Every row of
the table then certifies in 3 rounds, with cells and cell bounds equal
to the unit face's to the bit. Pinned by
`patch_bound::tests::a_uniform_weight_scale_moves_no_cell`,
`patch_bound::tests::the_weight_rescale_is_taken_only_where_it_is_exact`
and `offset_fit::tests::a_uniform_weight_scale_moves_no_cell_bound`.

Residue on QUAD:
`a-uniformly-scaled-rational-patch-refuses-the-quadrature-as-a-straddling-weight`.
