---
id: contfp-walks-the-vertex-polygon-of-an-arc-bearing-loop
kind: issue
title: contain::contfp answers an arc-bearing loop (LoopShape::ArcParity) from its vertex polygon, which is not the region: a false Out in the lune an outward arc bows past, on a door with live callers in reduce, ops, census and chart_region
status: dispatched
opened: 2026-09-26
priority: P0
cost: D
parent: CONTACT-4
---


Filed by the CONTACT orchestrator from ATREST's note on this log
(2026-09-28, ATREST-9 / PR #3204).

`boolean::contain::contfp` sends `LoopShape::Polygon` and
`LoopShape::ArcParity` to `point_in_loop`, which is the vertex-polygon
ray parity. `LoopShape::ArcParity`'s own doc says that polygon "is NOT
this loop's region": an arc bowing outward leaves region between the
polygon and the boundary, and a point there reads `Out` when it is in.
This was measured on a bored D-rod's transverse cap. The variant's doc
defers the general case to #1076.

That general walk now exists.
`splitting::containment::point_in_carrier_loop` counts each edge's
crossings on its carrier (lines, circular and elliptic arcs; spiric and
spline carriers refuse locally). Two independent reviews measured it
at about 43,000 adversarial probes with zero wrong answers.

`contfp` has live callers:
- `boolean/reduce.rs` (three sites)
- `boolean/ops.rs`
- `census.rs` (the pierce/containment arm)
- `chart_region.rs`

A false `Out` in any of them is a wrong answer, not a refusal.
