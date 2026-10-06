---
id: loft-arc-columns-end-on-the-carrier-not-the-vertex
kind: issue
title: an arc's NURBS column (skin::segment_curve) starts and ends on the carrier, not on the stored vertices; pinning them flips step-import's seam rung
status: open
opened: 2026-09-25
---


Found by PATHS' `geom-brep-sketch-segment-full-turn` (the unit that
moved `skin::segment_curve` onto the segment's stored carrier and
sweep). Filed here because `crates/sweep/src/skin.rs` is CARVE's.

## What stands

`sweep::skin::segment_curve`'s arc arm builds the rational-quadratic
column from the carrier: its first control point is `on(start)` and its
last is `on(start + θ)`, each `centre + r·(cos, sin)(angle)`. A clamped
NURBS interpolates its end control points, so the column starts and ends
on the carrier's rounding of the vertices rather than on the stored
vertices themselves. A line's column (`world(a)`, `world(b)`) ends on
them exactly, so an arc column and its neighbour meet only to the arc
endpoint's rounding.

## What pinning them does (measured, then reverted)

Setting the arc column's first control point to `world(a)` and its last
to `world(b)` — the stored vertices, verbatim — was measured on the
unit's branch and taken out again, because it flips a decision:

- `step-import`'s
  `recognize_pins::the_mixed_arc_prism_imports_first_class_over_the_intersection_pcurve_arm`
  at ε = 1e-9: the seam that takes the declare-and-check plane × NURBS
  rung goes from 1 to 0. The test's own prose names the cause: that
  seam's carrier differs from the arc wall's column "by the arc
  endpoint's rounding", so it has no bitwise `IsoCurve` match. With the
  vertices pinned the match is bitwise. The 1e-12 row was not
  re-measured. (It was a typed refusal carrying a 6.3e-12 sup; since
  PROPS' PR 3524 took the convex insertion form that sup is
  3.5528237131349995e-14 and the 1e-12 row is the FIRST-CLASS cell, with
  the refusal below 1e-13 — so whatever this row does about the 1e-12
  cell, it is a different cell from the one described here.)
- `step-export`'s `swept_elbow.step` corner reads `(…, 3.0, 3.0)`
  instead of `2.9999999999999996`, and the elbow's `KERNEL_VOLUME_*`
  sidecar moves in the last digit.
- The loft mass-properties digests (`sweep`'s reporting-door,
  thread-count and shell-census tables) move by 1–2 ulps in `v`/`a`,
  with every verdict hash unchanged.

## What is owed

A decision on whether the column's ends are the vertices (the
he_plus / vertex-authority reading) or the carrier's (today's), made
with the step-import posture change in view, and the re-baselines that
follow from it.

## After PATHS 5b (#3774)

5b put `segment_curve`'s arc column on the circle `SketchSegment::eval`
describes, so that the curve and the certification meters read the same
circle (`work/paths/store-constructed-carriers.md`, item 3). Its first
control point is now `world(a)` verbatim and each on-arc control is
`Arc2::point_from(a, k/m)`. The last one is `eval`'s end, which is `b` only
as far as `eval` is exact there (`sketch-segment-eval-could-be-exact-at-both-ends`).

That moved the postures this item predicted (each confirmed by restoring
main's `segment_curve` and watching the row go back):
- `step-import`'s mixed arc prism now imports first-class at ε 1e-9, 1e-6,
  1e-12 and 1e-14. Both arc-wall seams take the bitwise `IsoCurve` rung,
  so no seam is left for declare-and-check. The row is re-baselined in
  #3774. The importer's declare-and-check is still exercised there by
  `a_displaced_seam_carrier_refuses_with_the_measured_residual`, and in
  geom-brep by `m7_8_plane_nurbs_edge`.
- `swept_elbow.step` reads `3.0` at the corner, and its `KERNEL_VOLUME_*`
  sidecar moves.
- `mesh`'s `d9_mesh_goldens` swept_elbow digests move at both δ.
- `sweep`'s thread-count digests move by ulps, and every verdict is unchanged.

What is left of this item is the end column. It is the vertex only when
`eval(1)` is `b`, so it reduces to the eval issue named above.
