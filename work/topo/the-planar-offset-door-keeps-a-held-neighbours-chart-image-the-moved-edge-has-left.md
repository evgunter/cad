---
id: the-planar-offset-door-keeps-a-held-neighbours-chart-image-the-moved-edge-has-left
kind: issue
title: offset_planes_together keeps an edge's image in a held neighbour's chart, and certification refuses the sound move because the edge has left that image
status: open
opened: 2026-10-05
priority: P3
cost: E
refs: [an-offset-door-restates-a-neighbour-chart-rim-the-describing-door-cannot-vouch-for]
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
