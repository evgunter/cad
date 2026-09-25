---
id: one-segment-loop-through-builders
kind: unit
title: Admit a one-segment closed loop through validate, extrude, revolve and loft: one periodic wall with a seam strut, one self-loop rim per cap
status: parked
opened: 2026-09-25
priority: P1
cost: H
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
blocked_on: [geom-brep-sketch-segment-full-turn]
---


Unit 3 of the #3218 lowering. `build_loop_segs` admits n = 1 with |Δθ| = 2π. Extrude builds `mvfs` + `mef(Lone)` (holes: `kemr` + `mef(Lone)` + `kfmrh`), and the strut is described as a seam. `build_chain` gets an n = 1 arm, and so do `axis.rs`'s full-carrier arms. `resolve_chain_opt` admits n = 1. `SWEEP_FRONTIER`'s premise is re-worded. Red-first rows from raw fixtures, including a boolean on an extruded periodic wall with a seam strut (no evidence one has ever run). Survey §2.
