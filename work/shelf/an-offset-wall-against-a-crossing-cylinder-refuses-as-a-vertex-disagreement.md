---
id: an-offset-wall-against-a-crossing-cylinder-refuses-as-a-vertex-disagreement
kind: issue
title: An offset wall against a crossing cylinder now refuses as a vertex disagreement, not as an unroutable neighbour pose
status: open
opened: 2026-10-31
priority: P3
cost: M
---


Found by SECTARMS's cylinder × cylinder stage 1 (PR 6139), whose
review asked what `replace_face`'s neighbour gate does now that the C5
pose question serves every cylinder pose.

## Measured

`sweep/tests/crossing_equal_rods.rs`,
`offsetting_the_mitred_wall_refuses_typed`. `A ∪ B` at spins `(0, 0)`
puts the bisector ellipse between two radius-1 walls on meeting axes.
Moving `A`'s wall out by 0.05 (`topo::replace_faces_offset` over every
face that wears it) asks the pose of a radius-1.05 and a radius-1 wall.

- **Before PR 6139:** `route_pose` routed that pose to the general
  rung, and the gate in `replace_face` (`neighbour_section`'s
  `pose_route`, ~2774) refused `NeighborPoseUnroutable`.
- **Since:** the pose is served (two loops about the thinner wall), so
  the gate passes. The door then refuses `VertexDisagreement`, with a
  gap of 0.116 m at the moved vertex.

The refusal is still typed, but it now names a geometric disagreement
where the cause is an unbuilt neighbour section: no lane re-derives an
edge on a marched cylinder pair. `replace_face` has a second copy of
the gate (~3226).

## What a fix has to decide

Either the offset door gates on what it can build (a closed-form or
marched section it re-derives), not on what the pose question serves,
or it builds the re-derivation over the section value's carried
component.
