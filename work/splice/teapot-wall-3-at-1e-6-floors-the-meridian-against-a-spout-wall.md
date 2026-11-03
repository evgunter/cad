---
id: teapot-wall-3-at-1e-6-floors-the-meridian-against-a-spout-wall
kind: issue
title: Teapot wall 3 at ε 1e-6 refuses SplineCrossingUnsettled { why: Floor }: the belly meridian's interior walk against a spout NURBS wall floors one wall before the held pierce
status: open
opened: 2026-11-03
priority: P2
cost: M
---


Filed by the split-parameter lane (`splice/split-parameter`), from teapot
wall 3's measurement at the tour's three ε rows.

## What

With a boundary meeting's parameter carried to the seam split
(`a-meeting-on-a-spline-boundary-edge-has-no-split-parameter`), teapot
wall 3 (`demos/tour/src/teapot.rs`, `wall_probes`) gets past the split at
every row. On main all three rows refused at the split,
`PointSplitCarrierUnsupported { operand: B, edge: 10v1 }` (1e-6 measured
on main's own tree, d1ac6881). Past it the rows part:

- default and 1e-12: `HeldPierceUnbuilt { operand: B, kind: Nurbs,
  pierced_kind: Sphere, placed: false }`
  (`a-spline-wall-cannot-say-whether-it-holds-a-pierce-point`);
- 1e-6: `SplineCrossingUnsettled { operand: A, edge: 9v1, face: 9v1,
  face_of: B, why: Floor }`, one wall before.

The edge is the belly's quarter meridian, a circle from
`(−3/64, 1/8, 0)` to `(−1/16, 1/64, 0)`. The face is the spout's NURBS
wall whose chart runs from `(−1/32, 3/64, −9/512)` to the symmetry plane
`z = 0`: the wall on the far side of the seam the meridian was just split
on. `teapot::wall3_pinned` pins both by geometry at that row.
`geom_brep::roots::Unsettled::Floor` is a piece at the walk's floor whose
enclosure is wider than the band, so the walk ran out of depth, not into
a reading in band.

## Not yet measured

Where on the meridian the floored piece sits. The guess, unchecked: at
the fragment's end on the seam, which is also the far wall's boundary,
where the span meets the face at its own end and the walk cannot clear
the piece at 1e-6's coarser floor. The interior arm is
`reduce::spline_interior` → `geom_brep::NurbsLane::chart_meetings`. The
first measurement is the floored piece's parameter interval on the
meridian and its distance from the fragment's ends.
