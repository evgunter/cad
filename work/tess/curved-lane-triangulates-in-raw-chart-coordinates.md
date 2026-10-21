---
id: curved-lane-triangulates-in-raw-chart-coordinates
kind: issue
title: The curved lane's CDT runs in raw chart coordinates, so a cone or torus whose two axes scale differently may fan an off-row boundary point across many columns
status: open
opened: 2026-10-21
priority: P3
cost: E
---


## What

From #5531's FULL review (Q4), the sibling of
`trimmed-cylinder-lane-triangulates-in-raw-angle-by-height`.

`crates/mesh/src/curved.rs` inserts its points into spade at raw chart
coordinates (`insert`, `SpadePoint::new(u, v)`). On a cone the chart is
(radians, metres); on a torus it is (radians, radians) at different radii.
So a lattice that is square in 3-D can be a strip in the CDT's plane. #5531
measured that this is the shape that makes Delaunay fan an off-row point
across many columns. In this lane the off-row points are boundary chord
points: rim chords at the rims' own spacing and meridian chords at theirs,
beside an interior grid sized separately.

Unverified: the lane's domain is a convex rectangle, and its certificates
refuse typed, so a fan here would be a refusal and not a bad mesh.

## To do

Probe first, as the cylinder item does. Take a cone and a torus whose chart
aspect is far from 1 (a long thin cone; a torus with R/r large), and run the
review's twenty δ. If any refuses at a fan, or fans wide under the
falsifier, the curved lane takes the same face-wide power-of-two metric
(`nurbs_cert::lattice_metric`, applied at insertion only), and this item and
the cylinder item can ride one unit. If none does, close both with the
measurement.
