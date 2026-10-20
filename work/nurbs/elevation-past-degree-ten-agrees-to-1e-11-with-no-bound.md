---
id: elevation-past-degree-ten-agrees-to-1e-11-with-no-bound
kind: issue
title: Degree elevation past degree 10 agrees to about 1e-11 of scale and surfaces no bound
status: open
opened: 2026-10-20
priority: P3
---


**P3.** Found by #5459's FULL review (MINOR-3,
https://github.com/evgunter/cad/pull/5459#issuecomment-6290152933).

**What.** `algebra::elevate_plan` (`crates/geom-core/src/spline/algebra.rs`)
recomposes by exact removals and surfaces no bound. After #5459 its
measured agreement, relative to the curve's largest control coordinate,
is within 1e-12 at degree ≤ 10 on every family the rows and their
held-out checks draw. Past degree 10 it is larger, and no row measures
it:

| input | relative error |
|---|---|
| degree 11, 200 near-uniform knots | 2e-12 |
| degree 12, 200 near-uniform knots | 1e-11 |
| degree 12, graded or jittered vectors | 1–2e-11 |
| degree 25, a `1e-9` pair, weights 1e-3–1e3 | 5e-11 |

On ±10 coordinates that is about 1e-10 absolute, 100× ε at
`CAD_TOLERANCE_EPS=1e-12`. The callers that elevate user geometry to
a common degree (`compose_chain` in `crates/geom/src/curves/compose.rs`,
`skin` in `crates/sweep/src/skin.rs`) take any degree the user gives
them, so a section of degree ≥ 11 reaches it silently. (Main is far worse
on the same inputs: it refuses, or reads 1e-5 to 1e+44.)

**Done when** either:
- `elevate_plan` surfaces a bound. The split estimate already computes
  one per pass (`conditioned_split`'s `worst(nf)` times unit roundoff
  times the coordinates' scale), and carrying it to the plan would let a
  caller refuse where the bound passes its ε; or
- a measured row pins the envelope past degree 10, and the callers'
  docs state it.
