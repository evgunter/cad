---
id: main-red-a-cylinder-wall-through-a-prism-corner-escalates-its-chart-junction-at-1e-12
kind: issue
title: main red (1e-12 row): a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta refuses the union at the gate, check 7's chart junction escalating
status: open
opened: 2026-11-23
priority: P0
cost: E
---


Found by the WALKS lane on `walks/finish-completes` (PR 7497), whose PR
gate ran sweep's 1e-12 row because the diff touches sweep. Reproduced on
`origin/main` at `d8b6b353` in a clean worktree, so it is main's, not the
PR's:

```
CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep -E 'test(/a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta/)'
```

`crates/sweep/tests/a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta.rs`
(`a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`,
the `union.unwrap_or_else` line) panics: the union refuses at the
boolean's gate with `ResultInvalid { findings: [VolumeUncomputable {
solid: SolidKey(1v1), source: Face { face: FaceKey(15v3), source:
Escalated { cause: Indeterminate { margin: 2.8154559647670245e-12, band:
{ zero: 1e-12, escalate: 1e-11 }, predicate:
"props_chart_junction_closed" }, check: Inventory } } }] }`. The row
expects the union to build below 1e-6. The same margin on main and on the
PR's head, so the PR moves nothing here.
