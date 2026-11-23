---
id: blend-an-edge-in-another-shell-moves-predicate-4-at-a-cavity-corner
kind: issue
title: blend: adding one island edge to a sealed cavity's twelve-edge request makes predicate 4 refuse ChainNotG1 at a cavity corner the twelve alone pass (measure first)
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found by the lane that built predicate 2's reach. The sealed `[1,3]³`
cavity in the `[0,4]³` block, with the island `[1.05,2.95]³` unioned in
as a second solid: `fillet_edges` over the void's twelve edges passes the
battery (and refuses at the reach, correctly), and over all twenty-four
(void + island) builds (`blend_per_shell_carry::a_blend_beside_another_solid_builds_when_it_clears_it`).
Over the void's twelve plus ONE island top edge (the island's
`y = 1.05, z = 2.95` edge), `run_battery` refuses `ChainNotG1` at a
vertex with margin `1.9` — the island edge's own length — so a cavity
corner appears to be read as a junction of the island link
(`crates/sweep/tests/blend_band_reach.rs`,
`a_co_requested_band_replaces_only_its_own_strip`, which pins the reach
at the meter for this reason). Measure which vertex and which two links
`walk_chains` (`crates/sweep/src/blend/battery.rs`) paired before
assuming a walk defect.
