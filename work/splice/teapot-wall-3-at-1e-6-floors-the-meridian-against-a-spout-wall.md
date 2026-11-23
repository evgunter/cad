---
id: teapot-wall-3-at-1e-6-floors-the-meridian-against-a-spout-wall
kind: issue
title: Teapot wall 3 at ε 1e-6 refuses SplineCrossingUnsettled { why: Floor }: the belly meridian's interior walk against a spout NURBS wall floors one wall before the held pierce
status: closed
opened: 2026-11-03
priority: P0
cost: M
closed: 2026-11-04
pr: 6519
---


Filed by the split-parameter lane (`splice/split-parameter`), from teapot
wall 3's measurement at the tour's three ε rows.

## What

With a boundary meeting's parameter carried to the seam split
(`a-meeting-on-a-spline-boundary-edge-has-no-split-parameter`), teapot
wall 3 (`demos/tour/src/teapot.rs`, `wall_probes`) gets past the split at
every row. On main (d1ac6881) the default and 1e-6 rows refused at the
split, `PointSplitCarrierUnsupported { operand: B, edge: 10v1 }`, both
measured on main's own tree. Main's 1e-12 row was not measured. Past the
split the rows part:

- default and 1e-12: `HeldPierceUnbuilt { operand: B, kind: Nurbs,
  pierced_kind: Sphere, placed: false }`
  (`a-spline-wall-cannot-say-whether-it-holds-a-pierce-point`);
- 1e-6: `SplineCrossingUnsettled { operand: A, edge: 9v1, face: 9v1,
  face_of: B, why: Floor }`, one wall before.

The edge is the belly's quarter meridian. The face is one of the spout's
bore walls: its chart's root corner sits `SPOUT_BORE · SPOUT_R0` below
the spout's root. Where it stands against the seam the meridian was
split on is not measured.
`teapot::wall3_pinned` pins both by geometry at that row.
`geom_brep::roots::Unsettled::Floor` is a piece at the walk's floor whose
enclosure is wider than the band, so the walk ran out of depth, not into
a reading in band.

## Not yet measured

Where on the meridian the floored piece sits. The guess, unchecked: the
meridian lies in the pot's plane of symmetry, as the bore wall's seams
do, so it may graze the bore wall along a seam, where the walk cannot
clear a piece at 1e-6's coarser floor. The interior arm is
`reduce::spline_interior` → `geom_brep::NurbsLane::chart_meetings`. The
first measurement is the floored piece's parameter interval on the
meridian and its distance from the fragment's ends.

## Re-banded P0 (SPLICE, 2026-11-03)

This is teapot wall 3's stop at ε 1e-6 since PR 6441, and `issues/curved-demo-joins-build` (P0) waits on it; a blocker inherits the band of what waits on it (`work/README.md`, Priority).

## Closed (SPLICE, PR 6519, 2026-11-04)

The floor was not the meridian against the bore wall at all. The spline-face walk (`geom_brep::curve_face::spline_face::isolate`) reads a window past each end of the span it is asked about, END_REACH·escalate/pace (10.2 mm of arc at 1e-6). There it refused on a floor box 4.47 mm past the span's end, where the meridian crosses the bore wall's seam edge transversally, a meeting the next piece owns. The window scales with ε and the floor with √ε, so only the 1e-6 row's window held it. A floor box now settles (no meeting, no refusal) only when its curve range lies wholly past an end and the curve over it is decided farther than the band from that end's point (`Pair::past_an_end`, K name `spline_face_past_the_end`). Every other floor box refuses as before. The continuation piece reads the crossing in its boundary pass. Teapot wall 3 now stops at `GermFrameUnsupported { 6v1 Sphere, 3v3 Nurbs }` at every ε, under one pin. Single FULL review: APPROVE-WITH-FIXES. The fix pass adopted the reviewer's six-cell unit row, which kills the always-settle, raw-compare and wrong-end mutants at three ε. Residue filed: `a-meeting-on-the-charts-edge-nearer-a-spans-end-than-the-walks-floor-refuses` (P3), and HONEROOT's `a-circle-touching-a-torus-off-the-arc-escalates-the-arcs-side-decision` (P3).
