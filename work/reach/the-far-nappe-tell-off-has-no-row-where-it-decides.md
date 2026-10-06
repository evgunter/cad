---
id: the-far-nappe-tell-off-has-no-row-where-it-decides
kind: issue
title: No row reaches the cone crossing lane's far-nappe tell-off where it decides: every cone face the rows build has a trim that already places a mirror-nappe root outside
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [an-edge-crossing-a-cone-face-has-no-root-lane]
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
