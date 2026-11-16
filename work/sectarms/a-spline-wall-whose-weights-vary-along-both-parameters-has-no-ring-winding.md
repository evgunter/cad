---
id: a-spline-wall-whose-weights-vary-along-both-parameters-has-no-ring-winding
kind: issue
title: A ring on a spline wall whose weights are constant along neither parameter has no winding: its iso curves are not cut, a product net's included
status: open
opened: 2026-11-12
priority: P3
cost: M
---

Filed by the `sectarms/spline-ring-winding` lane.

A ring on a spline wall is wound, and its bystanders re-homed, by paths
along the wall's iso curves (`ring_path::SplineWall`), each cut by
`geom_brep::interior_iso_u`. That collapse is offered only for a weight
net constant along `u` or constant along `v`
(`IsoRowError::WeightsNotSeparable`). A net whose weights vary along
both is not read, and that includes a product net `wu ⊗ wv` with both
factors varying, which is separable in the usual sense. So
`SplineWall::of` reads no such wall, and
`boolean::join::choose_roles` refuses its ring
`CurvedBooleanUnsupported { kind: Nurbs }` as it did every spline
wall's before. Pinned at the reader by
`ring_path::spline_wall_rows::a_wall_whose_weights_vary_along_both_parameters_and_a_point_off_the_wall_are_not_read`
and, for the product net,
`…::a_wall_paths_lean_is_levered_within_its_arriving_pieces_chord_and_turn`;
no boolean reaches it today (the loft's walls take their sections'
weights, constant along `v`).

A path's pieces need only lie on the wall and meet at a corner, so a
rational iso curve of full generality (its own weights, the collapse
of the homogeneous net) would serve; the collapse's refusal is about a
carrier two walls can share, which a path's piece is not.
