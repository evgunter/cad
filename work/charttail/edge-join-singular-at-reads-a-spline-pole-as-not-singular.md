---
id: edge-join-singular-at-reads-a-spline-pole-as-not-singular
kind: issue
title: edge_join::singular_at answers false for a spline chart by its last arm, so a pole on a spline face is read as regular
status: dispatched
opened: 2026-11-01
priority: P3
cost: E
rides_with: a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint
branch: charttail/collapsed-column
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
