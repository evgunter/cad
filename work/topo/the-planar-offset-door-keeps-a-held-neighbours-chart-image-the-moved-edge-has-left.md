---
id: the-planar-offset-door-keeps-a-held-neighbours-chart-image-the-moved-edge-has-left
kind: issue
title: offset_planes_together keeps an edge's image in a held neighbour's chart, and certification refuses the sound move because the edge has left that image
status: closed
opened: 2026-10-05
priority: P3
cost: E
refs: [an-offset-door-restates-a-neighbour-chart-rim-the-describing-door-cannot-vouch-for]
closed: 2026-10-06
branch: topo/planar-offset-neighbour-chart
---

## What

Found while fixing
`an-offset-door-restates-a-neighbour-chart-rim-the-describing-door-cannot-vouch-for`
(PR 4060), whose sweep of the offset doors' restate arms turned up a
third arm of the same shape that the row did not name.

`crates/topo/src/offset_together.rs` `restate`, the
`EdgeDescription::Chart(c)` arm, restates every chart image verbatim:
`Chart { surface: c.surface, image: Some(c.pcurve), .. }`. Its doc
reads "here EVERY named surface moves", but `offset_planes_together`
also takes moves of zero, which keep their chart (the rim lift moves
one chart and holds the rest). Where the image is in a held plane's
chart and the edge's other face moves, the edge slides within the held
plane, the kept image still draws the old locus, and certification
refuses the sound move:
`ReplaceFaceError::Op { error: RechartFalsifies { error: ResidualExceeded { check: ChartResidual, .. } } }`.

Witness (a probe, not committed): `sweep::test_support::cube(1.0)`, its
top edge on `y = 0` re-described (public `set_edge_curve`) as
`EdgeDescriptionSpec::chart(side)`; `offset_planes_together` moves the
top by 1/64 and holds every other chart. It refuses as above on
`origin/main` (`b553731d`) and on PR 4060's head alike, so it is not a
regression. Moving the side instead (the image's own chart) is taken.

No native constructor or step fixture is known to produce a
neighbour-chart image on a planar body; an explicit re-description does.

## Shape of a fix

The one PR 4060 gave the other two arms (`offset_axial.rs` `restate`,
`replace_face.rs` `plan_edge`): where the chart an image names holds
and the edge's other face moves, state the edge as
`Intersection(moving, held)`, or, when a declaration rides the image,
as an image in the moving face's own chart with the declaration
translated. The door has each plane's distance in hand
(`MovedPlane`), so the held side is known where it restates.

## Closed 2026-10-06

`offset_planes_together` restates through the one restate core the
three offset doors share, `crate::offset_restate`
(`restate`, `held_neighbour_image`, `beside_moving`, and
`chart_moves`, the one decision of whether a chart moves), called by
`replace_face::plan_edge` (its held-neighbour arm), `offset_axial` and
`offset_together`. An image in a chart that holds while the edge's
other side moves becomes the section of the two (or, declared, an
image in the moving side's chart). Any other image is derived afresh
from the moved carrier exactly where the edge slides within its chart
(two distinct planes, one of them moving: an image whose own chart
moves beside a moving side refused the same `ChartResidual`), and kept
where it does not slide (a seam, which translates rigidly with its
re-minted plane, and an edge whose planes both hold).

Witnesses in `crates/sweep/tests/offset_restates_a_neighbour_chart_rim.rs`:
`the_planar_door_restates_an_edge_in_a_held_sides_chart_as_the_section`,
`the_planar_door_moves_a_declared_edge_into_the_moved_faces_chart`,
`the_planar_door_derives_the_image_afresh_when_both_charts_move`.
