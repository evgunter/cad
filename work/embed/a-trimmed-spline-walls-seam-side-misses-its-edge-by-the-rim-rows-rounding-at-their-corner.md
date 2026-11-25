---
id: a-trimmed-spline-walls-seam-side-misses-its-edge-by-the-rim-rows-rounding-at-their-corner
kind: issue
title: At the corner where two trimmed spline walls' seam meets the row that trims them, the seam side starts a rounding off its edge, so the pair splits to the depth floor
status: open
opened: 2026-11-25
priority: P0
cost: M
refs: [a-trimmed-free-form-face-has-no-window-in-the-injectivity-engine, edge-piece-misses-a-chart-sides-stored-carrier-by-the-charts-rounding]
---


## Finding

Found by lane `embedwalk-trimmed` (branch
`embedwalk/trimmed-freeform-window`), measuring.

With a trimmed spline face given a window (the knot domain, read as
`Region::Trimmed`), the at-rest gate's ask on
`encl_curved_loft_shell::shelling_the_vase_refuses_at_a_walls_fit_and_its_cap_moves_alone_onto_a_wall_row`
(sweep: the vase with its bottom cap moved alone onto the walls' row
`v = 0.025`) gets past the window and refuses

`Uncertified { a: FaceKey(3v1), b: FaceKey(4v1), shared: Edge(EdgeKey(6v1)), arm: Departure }`

at the depth floor (`max_cell_depth: 40`), on the cell pair
`(0.9990234375, 1.0) × (0.0244140625, 0.025390625)` of wall 3 against
`(0.0, 0.0009765625) × (0.0244140625, 0.025390625)` of wall 4: the
corner where the walls' shared seam meets the rim row that now trims
both. The unmoved vase, whose walls are whole patches, reads
`Embedded` (984 discharged, 364 certified).

Both pieces present (`whole == false`): each zero is
`Zero::Side` on the seam with `stops: [the corner vertex]`, its
rectangle clipped below at the rim row's stored image
`v = 0.024999999999974973`. The seam edge starts at the joint
`v = 0.024999999999999998` (the reproducer's own comment states the
2.5e-14 gap), so the side runs that rounding below its edge's end and
reads `on_edges == false`. A side with no parameters read
(`Along::whole` false on a spline chart) is under the graph only where
it lies on its edges (`Along::under_graph`), so no separator parts the
pair, and every smaller corner cell reads the same gap. One cell above
the corner the same sides read `on_edges == true` and certify.

## What is needed

Read the seam side at the corner as ending at the vertex both its clip
and its edge reach, the gap between the row's stored image and the
seam's joint charged as the chart's rounding (as `chart_slack` charges
an extent's copy), or have the rim row's image and the seam's joint
agree. The whole-patch case never meets it: there the row is the
domain's exact bound.
