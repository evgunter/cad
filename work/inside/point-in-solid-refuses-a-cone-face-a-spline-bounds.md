---
id: point-in-solid-refuses-a-cone-face-a-spline-bounds
kind: issue
title: point_in_solid refuses PartialConeFace near a cone face a marched section's spline edge bounds: ruled_face_region reads lines and conics only
status: open
opened: 2026-11-24
priority: P2
cost: M
refs: [the-join-reads-the-cone-pairs-on-the-value]
---


Filed by SECTCONE's join cone-pair unit (`work/sectcone/the-join-reads-the-cone-pairs-on-the-value.md`, PR 7735).

## Measured

`crates/sweep/tests/reach_cone_root_lane.rs`
`every_op_on_a_cone_wall_answers_its_truth_or_refuses_typed`, the tilted
rod across the narrowing frustum. Every op builds, at its closed-form
volume and through tier 3. The cone face that the marched cone × cylinder
section bounds then refuses `point_in_solid`: for example, `∪` at
`(−0.9214, 0.6714, 0)` refuses `PointInSolidError::PartialConeFace`.
The row reads membership at every grid point the door answers, and counts
the refusals (`membership_off_the_cone_face`).

## Where

`topo` `solid_contain.rs`, the `FaceGeo::PartialCone` arm: it refuses a
point on the cone within reach of the face, and a ray that meets the cone
ahead within reach, wherever `ruled_region::ruled_face_region` answers
`None`. It answers `None` for a boundary edge that is not a line, a circle
or an ellipse. Every cone pair the section value marches (cone × cylinder,
cone × cone, cone × sphere) leaves its cone face bounded by such a spline
edge.

## What a fix supplies

A cone face's region bounded by a fitted edge, read through the edge's
pcurve on the cone's chart, for example.

