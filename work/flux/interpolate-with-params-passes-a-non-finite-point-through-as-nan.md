---
id: interpolate-with-params-passes-a-non-finite-point-through-as-nan
kind: issue
title: geom fit: interpolate_with_params and interpolate_on take a non-finite point and return NaN control points instead of NonFinitePoint
status: open
opened: 2026-10-04
priority: P3
cost: E
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
