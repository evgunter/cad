---
id: a-cut-by-a-sphere-bearing-tool-on-a-planar-seat-aborts-on-its-re-chart
kind: issue
title: A cut whose fused tool the cut re-charts on a sphere shell refuses JoinDesync, a kernel-bug class, with no row and no pin
status: closed
opened: 2026-11-12
priority: P2
cost: M
closed: 2026-11-23
---


Filed by SEAL's orchestrator from the delta review of PR 6860 (the cut reads its tool as material), 2026-11-12.

## What

`material::Through::restate` (`crates/topo/src/boolean/material.rs:790`) refuses `BooleanError::JoinDesync` when the cut re-charted a sphere shell of the fused tool (`recharted[1]` non-empty), because the re-chart's turned cells read back to no cell of the tool. `JoinDesync` is classed as an abort ("kernel bug or corrupt reduction"), but this arm is a known limit of the read-back, not a corrupt state.

No test pins the arm and no row covers it. The reviewer judged it likely reachable: a tool holding a sphere face, resting on a planar seat that the cut fuses, with a block cut by it that the cut re-charts.

## Done when

- Either the read-back maps a tool-side re-chart's cells (the re-chart's own lineage to the sphere face it turned), and the cut builds with its tiers and volume pinned;
- or the arm refuses a typed limit that names the face, not an `Aborts` class.

Either way, a fixture reaches it and pins it, and a mutant restoring `JoinDesync` dies.

## Closed

Closed 2026-11-23 by ORBIT's PR 7602 (`orbit/one-sphere-escape-cut`), which retires the rotation re-chart. `Through::restate`'s `recharted[1]` arm is deleted with `BooleanNaming::recharted`: no cut re-charts a shell of its tool any more. A tool's closed ball is cut in on the material along its own meridian instead. `Through::restate` reads the cut's `chart_cuts[1]` and division rows back to the tool, where a key the material did not mint passes through as the cut's own. Pinned by `crates/sweep/tests/a_cut_by_a_seated_tool_cuts_its_ball_in.rs`. The slab is cut by one body of two seated bricks (one planar record) and a ball poking through the slab's top. The test checks that the cut builds at its closed form, tier-3 valid, with the ball cut in on the material, and that every operand-B face its holdings and rows name is a face of the tool. A brick that crosses the slab puts that op on the crossings path, where the ball meets the interior-loop guard (`work/germ/interior-loop-cut-in.md`), not this row's arm.
