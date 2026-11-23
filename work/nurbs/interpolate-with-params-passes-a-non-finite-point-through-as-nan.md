---
id: interpolate-with-params-passes-a-non-finite-point-through-as-nan
kind: issue
title: geom fit: interpolate_with_params and interpolate_on take a non-finite point and return NaN control points instead of NonFinitePoint
status: closed
opened: 2026-10-04
priority: P3
cost: E
closed: 2026-10-26
---


(SSI implementer on `ssi/banded-fit`, 2026-10-04, while checking what
the banded collocation solve must reproduce on a non-finite
right-hand side.)

`NurbsCurve3::interpolate` refuses a non-finite point by
`FitError::NonFinitePoint` (`chord_params` in
`crates/geom/src/curves/fit.rs`), and so does `interpolate_columns`.
The explicit-parameter doors do not: `interpolate_with_params` runs
`check_params`, which validates the parameters only, then
`interpolate_core`, which never looks at the points; the new
`interpolate_on` has the same shape. A NaN or infinite coordinate goes
into the collocation solve as a right-hand side, the pivots come from
the matrix alone so nothing refuses, and `NurbsCurve::new` validates
weights and counts, not control points. The caller gets a curve whose
control points are NaN.

Fix shape: `check_params` (or `solve_on`) refuses
`FitError::NonFinitePoint { index }` at the first non-finite
coordinate, as the chord door does. Callers reached through it:
`fit_branch` (`crates/geom-brep/src/ssi.rs`), `edge_nurbs.rs`,
`step-export`'s writer, and the test fixtures a grep for
`interpolate_with_params` lists.

## Re-homed from FLUX to KNOT (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. KNOT collects the spline, net and fit doors in `geom-core` and `geom`: refinement inside an enclosure, the net-state reads, the NURBS derivative and projection doors, and the fit's refusals and solves. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## Closed (2026-10-26, easy-fixes lane)

`crates/geom/src/curves/fit.rs` `nurbs_fit!`'s `check_params` now refuses `FitError::NonFinitePoint { index }` at the first point with a non-finite coordinate, after the count check and before `check_ordered`. Both `interpolate_with_params` and `interpolate_on` run it. `interpolate_on`'s `# Errors` names the arm. Pinned by `curves::fit::tests::the_explicit_parameter_doors_refuse_a_non_finite_point`: NaN and +∞ at row 5, through both doors. Verified: `cargo nextest run -p geom -E 'test(/fit|interpolat/)'`, 23 passed.
