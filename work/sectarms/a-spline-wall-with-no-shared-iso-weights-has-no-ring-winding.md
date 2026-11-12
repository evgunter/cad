---
id: a-spline-wall-with-no-shared-iso-weights-has-no-ring-winding
kind: issue
title: A ring on a spline wall whose weights factor along neither parameter has no winding: its iso curves are not cut
status: open
opened: 2026-11-12
priority: P3
cost: M
---


Filed by the `sectarms/spline-ring-winding` lane.

A ring on a spline wall is wound, and its bystanders re-homed, by paths
along the wall's iso curves (`ring_path::SplineWall`), each cut by
`geom_brep::interior_iso_u`. That collapse is offered only for a weight
net constant along `u` or along `v`
(`IsoRowError::WeightsNotSeparable`), so `SplineWall::of` reads no wall
whose weights vary along both, and `boolean::join::choose_roles`
refuses its ring `CurvedBooleanUnsupported { kind: Nurbs }` as it did
every spline wall's before. Pinned at the reader by
`ring_path::spline_wall_rows::a_wall_with_no_shared_iso_weights_and_a_point_off_the_wall_are_not_read`;
no boolean reaches it today (the loft's walls share their sections'
weights along `v`).

A path's pieces need only lie on the wall and meet at a corner, so a
rational iso curve of full generality (its own weights, the collapse
of the homogeneous net) would serve; the collapse's refusal is about a
carrier two walls can share, which a path's piece is not.
