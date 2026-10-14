---
id: a-general-image-ending-on-a-closed-charts-seam-takes-its-foot-on-the-first-branch
kind: issue
title: a General image whose edge ends on a chart's closed-in-v seam takes its end foot on the v₀ branch, folding the image back across the chart
status: open
opened: 2026-10-14
priority: P3
cost: M
---


(Filed by PCERT from `the-arc-rim-pick-on-a-chart-closed-in-v-has-no-row`.)

## What

Take a spline chart closed in `v`, whose first and last control rows
are the same points. An edge on it that ends at a vertex on the seam
row has an end point with two chart feet, `v = v₀` and `v = v₁`. The
`General` image's foot schedule (`edge_nurbs`, reached through
`FittedLane::general_image` from `nurbs_iso_derive`'s fall-through in
`crates/topo/src/pcurves.rs`) takes the `v₀` one whichever way the
edge approaches it.

## Seen on

The fixture is `sweep/tests/an_arc_rim_picks_the_row_it_lies_on.rs`'s
`seam_tube`:
- a quarter tube (R = 1, r = 0.25);
- its wall is one face on the exact rational torus net, closed in `v`;
- `v₀` sits at the outer equator.

Each cap circle is split into two half-arcs at the two equator
vertices. Their derived images:

| half-edge | runs on the cap at | entry → exit (u, v) |
|---|---|---|
| he1 | u = 0 | (0, 0) → (0, 0.5) |
| he3 | u = 0 | (0, 0.5) → (0, **0**) |
| he12 | u = 1 | (1, 0) → (1, 0.5) |
| he10 | u = 1 | (1, 0.5) → (1, **0**) |

The bold exits should be `v = 1`: those half-arcs run over the second
half of the tube circle. Each such image is a fit that turns back
across the chart in its last span.

Two consequences:
- **With the seam halves both on `v₀`** (before the arc rim's per-side
  pick), the walk read no gap anywhere. The folded fit then refused
  certification as `FittedLimb { limb: HullValueChart }`, margin about
  1.2e-4 at `Tol::witness`.
- **With the seam halves on their own sides**, the walk refuses
  `LoopDiscontinuity` at the seam half on `v₁`.

## Question

Which branch should the end foot take: the one continuous with the
image's interior feet, i.e. the foot of the sample before it?

The same can hold at a `u` seam. There, the walk's u-period shift
(`chart_u_period`) repairs a whole image that sits a period off, but
not one that folds partway through.
