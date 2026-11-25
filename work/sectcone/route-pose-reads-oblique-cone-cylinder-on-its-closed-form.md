---
id: route-pose-reads-oblique-cone-cylinder-on-its-closed-form
kind: issue
title: route_pose reads oblique cone × cylinder on its coaxial closed form, while cone × cone and cone × sphere read the section value
status: open
opened: 2026-11-24
priority: P2
cost: E
---


Filed by the cone × sphere / cone × cone door lane (PR 7457, review r1 m3).

`geom_brep::intersect::route(Cone, Cylinder)` is implemented, and its note
says an oblique cylinder is marched on the section value
(`ssi::march_cone_ruled`). `route_pose` still reads the pair through
`cone_cylinder_section`, the coaxial closed form, which refuses every other
pose `RoutesToGeneralRung`, so `route_pose` answers an oblique pose
`implemented: false`. Cone × cone and cone × sphere read the value in
`route_pose` and answer `implemented: true` for their general poses.

The asymmetry is kept on purpose for now: the offset door (`topo`
`replace_face`) asks `route_pose` whether a moved edge's neighbour pair can
be re-stated, and its pose rows (`replace_face::pose_reach_rows`
`a_cone_cylinder_pose_is_read_at_the_apex`,
`a_cone_cylinder_tilt_is_levered_at_the_edges_distance_from_the_apex`)
pin the closed form's lever. The door re-states an edge only from a closed
form, so a cone × cone or cone × sphere neighbour now passes the gate and
refuses later (`ReplaceFaceError::EdgeSection`), while an oblique cylinder
refuses at the gate (`NeighborPoseUnroutable`): one situation refused two
ways.

The work: decide what `route_pose` answers, the arm's serving of the pose
(the value, every pose) or the closed form's (the door's question), and
read all three cone pairs one way. If it is the closed form's, the cone ×
cone and cone × sphere arms read `cone_sphere_section` and the closed cone
forms, and `verbs_offd`'s row returns to `NeighborPoseUnroutable`.
