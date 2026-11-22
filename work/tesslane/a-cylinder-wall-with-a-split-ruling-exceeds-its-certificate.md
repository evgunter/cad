---
id: a-cylinder-wall-with-a-split-ruling-exceeds-its-certificate
kind: issue
title: a cylinder wall whose ruling is split into two edges exceeds its certificate on the rectangle lane
status: closed
opened: 2026-11-17
closed: 2026-11-17
priority: P2
cost: E
refs: [a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it, a-cone-face-with-a-split-side-has-no-worst-triangle-theorem]
branch: tesslane/cone-axis-winding
---

## Finding

A revolved cylinder wall (profile `(1,0)–(2,0)–(2,1)–(1,1)`, full turn
or half), one ruling split at its midpoint or at two points with
`topo::Body::split_edge`, placed obliquely: on the rectangle lane
(`curved::tessellate_curved`) it refuses `CertificateExceeded` at
δ 0.01 and 0.001 and meshes at 0.1. The mirror-nappe diamond's split
cylinder wall refused at 0.1 as well (the four refusals of
`curved::tests::a_multiply_carried_iso_side_is_bitwise_straight_and_meshes_watertight`).

The cause is the lane's triangulation. `curved::grid_counts`' cylinder
arm lays no rows (`(nu, 1)`): its theorem holds every triangle between
two neighbouring rulings, with both rims chorded on the column step.
The split vertex is a boundary point on neither rim and off the grid,
and the chart CDT fans it across several columns to the far rim, so
those triangles span more than one column and exceed the sagitta bound.

## Closed (2026-11-17)

By `a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it`:
a cylinder or cone face whose straight side holds a point goes to the
trimmed lane (`walk::splits_a_straight_side`, read in
`tessellate::route_of`). Every cell above meshes there and passes
`check_mesh` (`split_rulings::a_split_side_adds_one_triangle_per_point`),
at the trimmed lane's `UNPROVED_HEADROOM`; that density is the debt of
`a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`.
