---
id: the-mesh-walk-places-a-one-face-ball
kind: issue
title: the mesh walk places a one-face ball (one wrap meridian pole to pole) as the two sides of its wrap, not a zero-width column
status: parked
opened: 2026-10-29
priority: P1
cost: E
blocked_on: [sphere-faces-take-a-per-face-chart-or-pole-vertices]
---


Ev ruled on `[ev]` PR 6014 (2026-10-29, fork row 215) that a chart singularity (a sphere's pole, a cone's apex) with no written edge to it is reached by ONE wrap edge, walked out and back: the wrap tip (DESIGN `[topology]`, the seam clause and tiers 2/3), "if the complexity it adds is tolerable", asking that keeping that complexity down be investigated. The probe that measured each reader is on branch `probe/foreign-wraptip` (edits E1–E7).

Owed here: `mesh::walk::loop_polygon` places a rimless loop of one edge walked up and back as the two sides of its wrap, a period apart (the probe's E7: before it `SingleColumnCurvedFace`, after it watertight, 960 triangles, the two-band ball's signed volume). Hemisphere and cone caps meshed with no edit.

Signed: (FOREIGN orchestrator)

## Released (2026-10-30)

`loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk` closed with PR 6088. The shared structure is `Body::both_sides_face`, read through two views: `surface_cycles` (region) and `chart_image` (spur-free: integrators and chart readers). Read the tip through these, never by a turn of your own. (RESTCURVE orchestrator)

## Held (2026-10-30)

Re-parked on `sphere-faces-take-a-per-face-chart-or-pole-vertices` (a ruling open on `[ev]` PR 6085). Ev pointed out that a sphere face other than the whole sphere has a singularity-free chart, which may retire the pole vertex and the wrap tip for spheres. Do not build on the wrap tip until it is ruled. (RESTCURVE orchestrator)
