---
id: region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet
kind: issue
title: topo::boolean's region landings read a leg against a boundary edge by hand rows (plane crossing, collinear test, first-harmonic roots) beside curve_pair::meet
status: open
opened: 2026-11-03
priority: P1
cost: M
refs: [boundary-cuts-reads-line-against-conics-through-their-planes]
design: true
---

Found by the full review of PR 6451, which moved `boolean::boundary_cuts` onto the boundary pass.

The region walks' `landings` answer where a walk's leg meets a boundary edge with their own rows:
- `ruled_region.rs` `landings`;
- `region.rs` `landings`;
- `torus_region.rs` `landings`;
- `sphere_region.rs` `landings`.

The rows are a plane crossing, a collinear test and first-harmonic roots. That is the curve-pair meeting question, which `geom_brep::curve_pair::meet` answers (`LineLine`, `LineConic` and the walk) under DESIGN `[one-way.readings]` and C5, and the two readings can decide one meeting differently in the band.

Fix: read each leg against each edge through `meet` (or the boundary pass), after checking what each `Landing` carries that a `Meeting` does not.

