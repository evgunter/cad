---
id: a-general-image-ending-on-a-closed-charts-seam-takes-its-foot-on-the-first-branch
kind: issue
title: a General image whose edge ends on a chart's closed-in-v seam takes its end foot on the v₀ branch, folding the image back across the chart
status: open
opened: 2026-10-19
priority: P3
cost: M
---


Filed by PCERT unit C (`pcert/net-v-period`), from
`work/pcert/the-arc-rim-pick-on-a-chart-closed-in-v-has-no-row.md`. It
was first written on PR 5137's branch, which unit C supersedes.

## What

Take a spline chart closed in `v`, whose first and last control rows
are the same points. An edge on it that ends at a vertex on the seam
row has an end point with two chart feet, `v = v₀` and `v = v₁`. The
`General` image's foot schedule takes the `v₀` one whichever way the
edge approaches it. The path:
- `nurbs_iso_derive`'s fall-through calls `derive_general_image`
  (`crates/topo/src/pcurves.rs:1376`);
- that reaches `FittedLane::general_image`
  (`crates/geom-brep/src/fitted_lane.rs:206`);
- whose schedule projects each sample on its own
  (`wall.project`, `crates/geom-brep/src/edge_nurbs.rs:1820-1836`),
  with no reference to the previous foot, so a foot on the seam row
  lands where the projection lands, `v₀` here;
- `chart_foot` (`crates/geom-brep/src/edge_nurbs.rs:1864`) is the
  same projection for a single foot.

## Seen on

The repro is `seam_tube` in
`crates/sweep/tests/an_arc_rim_picks_the_row_it_lies_on.rs` (PR 5137's
fixture, carried over by unit C):
- a quarter tube (R = 1, r = 0.25);
- its wall is one face on the exact rational torus net, closed in `v`;
- `v₀` sits at the outer equator, which is the wall's seam.

Each cap circle is split into two half-arcs at the two equator
vertices. Their derived images, as PR 5137 recorded them:

| half-edge | runs on the cap at | entry → exit (u, v) |
|---|---|---|
| he1 | u = 0 | (0, 0) → (0, 0.5) |
| he3 | u = 0 | (0, 0.5) → (0, **0**) |
| he12 | u = 1 | (1, 0) → (1, 0.5) |
| he10 | u = 1 | (1, 0.5) → (1, **0**) |

The bold exits should be `v = 1`: those half-arcs run over the second
half of the tube circle. Each such image is a fit that turns back
across the chart in its last span.

**What it costs today.** The seam has one image, the `v₀` row (C4;
`a_seam_circle_on_a_chart_closed_in_v_has_one_image`), and the walk
lifts a seam crossing by the chart's `v` period, the knot domain's
length (unit C). But `mint_pcurves` on `seam_tube` refuses before any
joint is read: the folded cap fit refuses certification as
`Certify { error: FittedLimb { limb: HullValueChart } }`, margin
1.16e-4 at `Tol::witness` (measured on `pcert/net-v-period`). So the
wall never reaches the walk, and no body with such a wall mints.

## Fix shape

C4 says every whole-period question uses the channel's own period
through the marks. The end foot's `v` is such a question: decide it
against the previous foot's `v` with the chart's `v` period
(`Period::decide`), and take the foot on the branch continuous with the
image's interior. The walk then reads the folded image's lift as a
joint element, as it does for a seam crossing.

The same can hold at a `u` seam. There, the walk's `u` period repairs a
whole image that sits a period off, but not one that folds partway
through.

Once fixed, `a_seam_circle_on_a_chart_closed_in_v_has_one_image` should
also mint the wall and read the seam halves' joint elements (`kv` of 0
and ±1).
