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

**Done when** `elevate_plan` surfaces a bound, or its callers refuse
at ε. The split estimate already computes one per pass
(`conditioned_split`'s `worst(nf)`, times unit roundoff, times the
coordinates' scale). It is first-order, so "steering" grade as `fit.rs`
calls its bound unless the chain is interval-evaluated, and whether
`compose_chain` and `skin` should refuse at ε is a design fork: this
form goes through the designer pass when it is picked up.

**Not a live hazard today, as far as checked.** Every degree-≥11 path
is user geometry handed to `compose_chain` or `skin`, and no test row
outside these elevation rows builds one. Whether any tour scene does
was not checked.

## Pinned (2026-10-20, `nurbs/elevation-envelope-rows`)

The measured envelope is pinned by rows, each capped at about 3× its
worst of 90 curves (CI's test profile), and stated at both callers'
docs. Relative to the curve's largest control coordinate:

| row | shapes | worst | cap |
|---|---|---|---|
| `degree_eleven_elevates_within_its_measured_envelope` | 50 uniform / graded / jittered knots | 4.4e-12 / 2.4e-12 / 8.1e-12 | 2.5e-11 |
| `degree_twelve_elevates_within_its_measured_envelope` | the same | 1.8e-11 / 1.5e-11 / 3.6e-11 | 1.1e-10 |
| `high_degree_clusters_elevate_within_their_measured_envelope` | degree 16 / 20 / 25, clusters of 2 and 4 knots `1e-9` apart | 1e-13 / 4e-13 / 5.9e-12 | 2e-11 |

- **Red on the midpoint split**, which refuses or reads 9e-5 and up on
  every row.
- **Margin 8** turns the cluster row red (2–4e-11 at degree 25).
- **Margin 1** reads alike on these shapes.
- **Timing:** each row takes ≤ 0.15 s. Shapes with 200 knots cost about
  0.6 s per elevation at degree 11–12 and are left out of the fast set
  (degree 12 with 200 knots measured 1–6e-11).

The rows guard against regression; they are not a bound, so this item
stays open on the form above.
