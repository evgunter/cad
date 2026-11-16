---
id: a-cut-by-a-sphere-bearing-tool-on-a-planar-seat-aborts-on-its-re-chart
kind: issue
title: A cut whose fused tool the cut re-charts on a sphere shell refuses JoinDesync, a kernel-bug class, with no row and no pin
status: open
opened: 2026-11-12
priority: P2
cost: M
---


Filed by SEAL's orchestrator from the delta review of PR 6860 (the cut reads its tool as material), 2026-11-12.

## What

`material::Through::restate` (`crates/topo/src/boolean/material.rs:790`) refuses `BooleanError::JoinDesync` when the cut re-charted a sphere shell of the fused tool (`recharted[1]` non-empty), because the re-chart's turned cells read back to no cell of the tool. `JoinDesync` is classed as an abort ("kernel bug or corrupt reduction"), but this arm is a known limit of the read-back, not a corrupt state.

No test pins the arm and no row covers it. The reviewer judged it likely reachable: a tool holding a sphere face, resting on a planar seat that the cut fuses, with a block cut by it that the cut re-charts.

## Done when

- Either the read-back maps a tool-side re-chart's cells (the re-chart's own lineage to the sphere face it turned), and the cut builds with its tiers and volume pinned;
- or the arm refuses a typed limit that names the face, not an `Aborts` class.

Either way, a fixture reaches it and pins it, and a mutant restoring `JoinDesync` dies.
