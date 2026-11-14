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

## On this unit's path (2026-11-14, from the lift measurement)

Five of `crates/mesh/tests/common/`'s hand-built witnesses (keyway, oblique lens, slit, pole-crossing half cap, apex-crossing bowtie) store no pcurves: `Body::loop_lift` answers `LiftGap` on all ten of their loops, and `topo::mint_pcurves` accepts all five. A lane that reads stored lifts refuses them as built, so the rows that use them mint first or move with this unit (`lift_probes::the_unminted_witnesses_store_no_lift_and_mint_one`).
