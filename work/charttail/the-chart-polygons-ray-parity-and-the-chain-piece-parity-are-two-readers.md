---
id: the-chart-polygons-ray-parity-and-the-chain-piece-parity-are-two-readers
kind: issue
title: A chart-space point against a closed boundary has two ray-parity readers: ray_walk over a polygon's segments and the piece parity over a chain's boxes
status: open
opened: 2026-11-14
priority: P3
cost: M
---


Filed by `tesslane/spline-ring-door`, whose class sweep this is. That unit
made the ring door's analytic reader and the spline trim one ray-parity loop
over a closed chain of chart pieces (`crates/topo/src/pcurves_rings.rs`,
`ray_parity_as` over `ChainPiece`): each piece boxed by its family, decided
clear of the query in metres through the chart's inf-side arms, halved where
it is not, and counted off its two ends under the half-open rule `v ≥ v_c`.

The chart region reads a point against a chart polygon another way:
`crates/topo/src/chart_region.rs` `point_in_polygon`, through
`crates/topo/src/ray_walk.rs` (`walk`, `ray_crossings`), tries a fixed
schedule of ray directions, reads each straight segment exactly, and sets a
grazing ray aside. A chart polygon is a chain of segment pieces, so a chart
point against a closed boundary has two readers, with two in-band stories
(a ray set aside on a graze and refused once the schedule runs out,
`ChartRegionError::RayExhausted`; a query held `Near` within the band of a
piece) and two sets of work bounds.

Not the same class, and not part of this row: the planar loops in model
space (`splitting/containment.rs` `point_in_vertex_polygon` and
`point_in_loop`, whose arc members count their crossings through
`PiecedArc::crossings`) and the point-in-face door's model-space ray cast
(`validate.rs` reports its `GRAZED` ending) read in a plane in space, not in
a chart, by the same `ray_walk`; `cell_region` draws no polygon at all.

Decide whether the chart polygon is read by the piece parity (a segment is
a piece whose box is its own), or say why the two stay apart, in the chart
region's module docs.
