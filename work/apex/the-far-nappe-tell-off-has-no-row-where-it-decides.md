---
id: the-far-nappe-tell-off-has-no-row-where-it-decides
kind: issue
title: No row reaches the cone crossing lane's far-nappe tell-off where it decides: every cone face the rows build has a trim that already places a mirror-nappe root outside
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [4135]
---


Left by the lane that gave a cone face its crossing lane
(`an-edge-crossing-a-cone-face-has-no-root-lane`).

## What

`reduce::wall_crossing` tells off a root of the double cone that lies
on the nappe its face does not (`bool_cone_root_nappe`, the root's
`geom_brep::cone_elevation` off the face's own nappe, read from
`face_nappe`) before the face's trim places it. Every cone face the
rows build — a revolved frustum on either nappe, a full cone — has a
chart trim whose slant window places a mirror-nappe point outside
already (`solid_contain::point_on_cone_in_face`, the window asked
first), so removing the tell-off leaves every row green, measured on
`sweep/tests/reach_cone_root_lane.rs`'s
`roots_off_the_faces_window_and_nappe_are_not_its_crossings`. The tell-off
decides where the trim declines (`PartialConeFace`: a cone face in
neither cone class, whose trim answers `Trim(None)` and keeps the
frontier) while its corners decide its nappe.

The row owed: such a face, with an edge crossing the double cone on
its other nappe and nowhere on its own, through the crossing sweep —
`Elsewhere` with the tell-off, the frontier without it.

## Both reviews of PR 4135 confirm it, and the route may not exist

- **r1** (MINOR-3, mutant M3, the tell-off disabled): every PR row stays
  green. A quarter-revolved frustum — a partial face, so its trim
  cannot place a root — crossed by a brick only on the mirror nappe,
  four times, answers correctly (no new vertex) with and without the
  tell-off: it does not reach the decision either.
- **r2** (n-1, mutant M5): the tell-off cannot decide where the trim
  answers, because `solid_contain::point_on_cone_in_face` refuses the
  far nappe first (`crates/topo/src/boolean/solid_contain.rs:3018-3022`,
  `bool_ray_cone_nappe`). It decides only on faces the trim declines —
  rings, `PartialConeFace` — and there it turns a frontier into
  `Elsewhere`. r2 could not build one through a public door.

So the `PartialConeFace` route suggested above may not exist: a partial
face's trim refuses its other roots too. The row owed may need a door
that hands the crossing layer a face its trim declines, or the tell-off
may prove unreachable through public doors, in which case it is a
second line behind the trim and says so.
