---
id: a-closed-spline-walls-projection-stops-at-its-seam
kind: issue
title: A closed spline wall's projection stops at its seam: a point just below it has no foot, and no ring-lane path
status: parked
blocked_on: [a-node-arcs-chord-misses-its-at-rest-chart-image]
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

## Parked (2026-11-17, SECTARMS orchestrator)

Ev's decision on PR 7032 makes the image of a carrier on a spline wall
one function whose projection follows the carrier continuously from its
start vertex's stored chart point, across a closed wall's seam. The
build of that decision (`a-node-arcs-chord-misses-its-at-rest-chart-image`
and its two siblings) is briefed to say which projection defects it
fixes. Re-read this row when that item closes.

## Beside `sectarms/one-image-function` (2026-11-23)

Not fixed there. That lane's wall image of a carrier
(`geom_brep::carrier_image`) reads a closed wall's seam from the side
its foot path runs on (each foot is Newton's from the last, and a start
on the seam takes the far side of the rectangle where the path leaves
into it from there; `carrier_image_rows::the_foot_path_reads_a_closed_walls_seam_from_the_side_it_runs_on`).
`ring_path::SplineWall::foot` is a point's own foot by
`NurbsSurface::project`, which that lane does not touch: a point just
below the seam still has no foot here.
