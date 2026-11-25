---
id: point-in-solid-refuses-a-ruled-face-a-spline-bounds
kind: issue
title: point_in_solid refuses PartialConeFace and WallOutlineUnsupported near a cone or cylinder face a marched section's spline edge bounds: ruled_face_region reads lines and conics only
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
rod across the narrowing frustum. Every op builds; `∪` and `∩` hold their
closed-form volumes and pass tier 3. The cone face that the marched cone × cylinder
section bounds then refuses `point_in_solid`: for example, `∪` at
`(−0.9214, 0.6714, 0)` refuses `PointInSolidError::PartialConeFace`, and
the rod's wall refuses `∩` at `(−0.8071, 0.3286, 0.0429)` with
`WallOutlineUnsupported`.
The row reads membership at every grid point the door answers, and counts
the refusals (`membership_off_the_cone_face`).

## Where

`topo` `solid_contain.rs`, the `FaceGeo::PartialCone` and
`FaceGeo::PartialWall` arms. The cone arm refuses a
point on the cone within reach of the face, and a ray that meets the cone
ahead within reach, wherever `ruled_region::ruled_face_region` answers
`None`. It answers `None` for a boundary edge that is not a line, a circle
or an ellipse. The wall arm refuses a ray's hit the face could contain on
the same `None`. Every cone pair the section value marches (cone × cylinder,
cone × cone, cone × sphere) leaves its cone face, and a cylinder partner's wall, bounded by such a
spline edge.

## What a fix supplies

A cone or cylinder face's region bounded by a fitted edge, read through the edge's
pcurve on the cone's chart, for example.

