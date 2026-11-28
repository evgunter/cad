---
id: a-spline-charts-ring-is-placed-by-its-chains-first-point-which-may-sit-on-a-degenerate-corner
kind: issue
title: A spline chart's ring is placed by its trim chain's first point, which may sit where the chart's cells have no arms
status: open
opened: 2026-11-25
priority: P2
refs: [an-analytic-outer-loops-whole-span-boxes-never-tighten-under-the-ring-reader]
cost: E
---


Found by the LATHE outline-by-decision lane (PR 7581, from its review).

## What

The analytic ring door now places a ring by an interior point of its
rows, the one farthest from a chart singularity
(`crates/topo/src/pcurves_rings.rs`, `reference_point`). It no longer
uses the ring's first vertex, which on a sphere face can be a pole: no
window round a pole has arms, so the reading never settled.

The spline door still places a ring by the first point of its trim chain
(`crates/topo/src/spline_trim.rs:948`, `ring_decks`: `chain.first()` →
`trim.ring_shift(.., first.ends().0, ..)`). That point is a vertex of the
ring. On a spline chart with a degenerate edge or corner (a collapsed
row of control points, as a loft or skin closing to a point produces, or
an imported surface with a pole) the vertex can sit where the chart's
cells meter no arm. The reading then answers undecided for a ring that a
point inside its chain would place.

## Reach

No measured witness. It is reachable by spline faces whose chart has a
degenerate side and that carry a ring with a vertex on it: loft, skin,
and imported spline faces. The spline trim meters clearance by the
chart's Bézier cells, not by an azimuth lever, so the analytic rule does
not carry over as it stands. The fix is the same shape: rank the chain's
interior points by the cells' arm at each, and place by the best one.

## Done when

`ring_decks` places a ring by an interior point of its chain chosen by
the cells' arms, pinned by a row whose ring has its first vertex on a
degenerate corner of a spline chart.
