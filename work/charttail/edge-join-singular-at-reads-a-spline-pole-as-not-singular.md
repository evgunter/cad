---
id: edge-join-singular-at-reads-a-spline-pole-as-not-singular
kind: issue
title: edge_join::singular_at answers false for a spline chart by its last arm, so a pole on a spline face is read as regular
status: closed
opened: 2026-11-01
priority: P3
cost: E
rides_with: a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint
branch: charttail/collapsed-column
pr: 7325
closed: 2026-11-23
---

Disclosed and left unscheduled by PR 6220's sweep for last-arm
`Ok(false)` readers (2026-11-01): `edge_join::singular_at` falls to
`_ => Ok(false)` for a spline chart, so a degenerate (pole) point of a
spline face is read as regular. No witness measured. Find a spline face
with a pole (a revolved or lofted cap converging to a point) reaching
the edge join, and either read the pole or refuse typed.

## Claimed by CHARTTAIL (2026-11-22)

Moved from PARTING to ride with
`a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint`. That
unit gives a spline chart its singular set: a boundary iso whose
control points are all one stored point (DESIGN `[extensibility]`,
read at one home, `geom_brep::chart_singularity::singular_points`).
`crates/topo/src/boolean/edge_join.rs` `singular_at` now reads a NURBS
or `Approx` face's collapsed sides from that home: the distance from
the point to the nearest side's point, decided as its other arms
decide theirs. A plane and a cylinder answer `false` by name, not by a
last arm. Its witness is a loft wedge
(`crates/sweep/tests/pinned_corners_build.rs`). No join of an edge
ending at its pole was built to pin the boolean reading; the arm
reads the same home the loop walk and tier 3 read.

## Closed (2026-11-23)

By PR 7325, on Ev's ruling on PR 7287: a spline chart's singular set is read exactly off its net, with one home, `singular_points(&Surface)`. A vertex on a collapsed side takes `AzimuthRule::OnCollapsedSide` (the free channel along the side, both ends checked on it), and the loft builds a pinned corner (class 1), a hinge and a lune exactly. `edge_join` and `cell_region` ask the home. The full turn's single pinned vertex is LATHE's `a-full-turns-pinned-vertex-refuses-at-the-loft-door`, and the closed collapsed side is `a-closed-collapsed-side-has-no-turn-or-side-reading`.
