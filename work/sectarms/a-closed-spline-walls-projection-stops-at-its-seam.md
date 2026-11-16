---
id: a-closed-spline-walls-projection-stops-at-its-seam
kind: issue
title: A closed spline wall's projection stops at its seam: a point just below it has no foot, and no ring-lane path
status: open
opened: 2026-11-13
priority: P3
cost: M
---


Found by the review of PR 6891; filed by the
`sectarms/spline-ring-winding` lane's fix pass.

`ring_path::SplineWall::foot` seeks a point's chart parameters by
`NurbsSurface::project`, whose Newton steps are clamped to the knot
rectangle (`geom/src/surfaces/projection.rs`, "periodic wraparound is a
designed absence until the periodic form exists"). On a wall closed
along `u`, a point just below the seam seeds toward `u = 0`, steps
against the clamp and stagnates there, short of its true foot near the
domain's far end. The foot is then not the point
(`split_ring_path_wall_foot`), so the point is offered no path: the
ring lane says nothing from it, never a wrong side. On a whole
cylinder as one rational wall with its seam at azimuth 0, a point at
azimuth −0.01 has no path to any other point, while points at −0.3 and
−2.0 read, pinned by `topo`
`ring_path::spline_wall_rows::an_island_across_a_closed_walls_seam_is_read_but_not_from_just_below_the_seam`.

The curve half has the same stop:
`work/nurbs/a-closed-splines-projection-stops-at-its-start-short-of-the-seam.md`.
A ring lane whose every outer reference stands just below a seam would
refuse `RingIslandUnread`; no boolean reaches that today.
