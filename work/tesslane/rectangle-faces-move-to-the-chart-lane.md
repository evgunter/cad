---
id: rectangle-faces-move-to-the-chart-lane
kind: unit
title: an iso-rectangle is a case of the chart lane, and the curved lane and its chart-inverse walk retire
status: parked
opened: 2026-11-14
priority: P1
cost: H
design: true
blocked_on: [a-curved-face-is-routed-by-its-domain-not-its-carriers]
---


Unit 6 of the chart lane (`work/tesslane/plan.md`, The chart lane). Follows unit 4. **This one is Ev's** (`design: true`).

## What it is

An iso-rectangle is the chart lane's case in which every boundary point is on a lattice line. Moving those faces over leaves one reader of a loop's place in its chart (the stored lift; `walk::loop_polygon` re-derives it from carriers, with its own tie rules at poles), one roster of charts, one pole rule and one lattice emitter. `curved.rs`, the chart-inverse walk and its doors retire, and `lane_of` reads the surface kind only. It closes `one-element-grid-axes-drop-schedule` structurally.

## Why it waits for Ev

Everything before this unit turns refusals into meshes and moves nothing the mesh emits today. This unit moves every swept face's mesh: the D9 replay digests, `docs/TESS-BUDGET.md` and the tour pins re-baseline at once. Both designers lean towards doing it and both say the choice need not be made until unit 4 has landed. It also re-cuts `crates/mesh/README.md` `[mesh.sizing]`'s list of lanes, which is ratified text.

It goes to Ev as an `[ev]` PR with one measurement in hand: rectangle faces through the chart lane against the curved lane, triangle for triangle (`tools/tess-meter`). If the chart lane cannot match the curved lane's density on rectangles, the answer is to stop at unit 4.

## A column the stored lift has exactly and the walk re-derives as dust (2026-11-14, TESSLANE)

Measured by the turned-ball unit (`a-sphere-face-tilted-by-an-ulp-against-its-copies-fails-the-cdt`). On `sweep::test_support::ball_poled_z(√2, 0)`, a revolved ball turned a quarter turn about `x`, the walk reads the u = 0 seam meridian's column as `topo::chart_iso::mid_azimuth`, `atan2(p·v_ref, p·u_ref)` at the meridian carrier's mid point. The turned chart's `v_ref` is `(0, 1, −6.1e−17)`, so `p·v_ref` is an exact zero computed in floats, and it comes out `−6.842e−49`: the column is `u = −4.838e−49`. That is below spade's coordinate floor, and the face refused `Triangulation` until every CDT feed went through `mesh::cdt::spade_point`. The same meridian's stored image on both faces is `Pcurve::Harmonic { u: Angle { phase: 0.0, winding: 0 }, .. }`: the stored lift has the column exactly. Reading the column from there rather than inverting a carrier point is this unit's, and the floor stays under it as the backstop.
