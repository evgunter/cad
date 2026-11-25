---
id: the-walk-drops-no-cell-of-a-cylinder-or-cone-face-a-spline-edge-bounds
kind: issue
title: The embedding walk drops no cell of a cylinder or cone face a spline edge bounds: the ruled region admits lines, circles and ellipses only
status: open
opened: 2026-11-25
priority: P2
cost: M
refs: [the-census-reads-no-ruling-inside-a-chorded-cylinder-face]
---


Filed by `embedwalk/share-nothing-region` (the fix of
`share-nothing-face-pairs-are-halved-over-their-carriers-crossing-off-the-faces`),
from its §5 sweep. Not measured on a body.

## What

The embedding walk (`crates/topo/src/clearance.rs`, `RegionMemo::ask`
and `RegionMemo::station`) drops a cell of an analytic face through
`cell_region::cell_region_lifted`, whose witness is the face door
`contain::curved_face_placement_reading`. That door now hands the
certifying scalar's spline reader (`region::certified_crossings`) to a
sphere's and a torus's region, so a cell off a sphere or torus face a
spline section edge bounds is dropped.

A cylinder or cone face is read by `ruled_region::ruled_face_region`,
whose roster is `[Line, Circle, Ellipse]` and which takes no reader. A
cylinder or cone face that a spline section bounds (every skew or
off-axis quadric pair: a cylinder against a sphere off its axis, two
skew cylinders) therefore has no region, `ruled_face_containment`
answers `Trim(None)`, and every cell of the face is `Undecided`. Where
two such faces of one solid share no cell, the walk halves them over
their carriers' crossing exactly as the parent row measured on the
sphere and the torus.

## Owed

The ruled region reading spline edges through `Legs::splines`, as the
sphere's and the torus's do: a roster widening, the reader handed in
through `ruled_face_containment`, a `SPLINE` `ArcRows` value for its
boundary pre-pass, and an end row. A ruling leg's plane holds the axis
and a parallel leg's is normal to it, so a spline is read across either
(`on_surface::direction` already reads `Nurbs` edge over leg for both).
Then a body whose cylinder and cone faces a spline bounds measured
through the walk.
