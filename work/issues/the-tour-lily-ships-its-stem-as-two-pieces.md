---
id: the-tour-lily-ships-its-stem-as-two-pieces
kind: issue
title: The tour's lily ships its stem as two pieces although the glue now builds
status: open
opened: 2026-10-27
priority: P3
cost: E
---


Found by SECT's lily lane (`sect/lily-parallel-tori-certificate`), which
retired lily wall 1, 2026-10-27.

## Measured

`demos/tour/src/lily.rs` `wall_probes`: wall 1 (`try_union(stem, arch)`)
now builds one shell, and the walls run it live as RETIRED. The scene's
`plant` still ships `lily_stem` and `lily_arch` as two pieces, each with
its own caps (`assert_cap`), mesh budget rows and downstream walls (wall
2 welds the lantern to `lily_arch`).

## What the taker owes

Ship the stem as the glued body, the way a user would once the glue
builds, re-pointing what reads the two pieces (caps, mesh budgets, wall
2's weld) and re-baselining the lily's frames with a `[render]` commit.
The owner is the tour's; no program holds `demos/tour`.
