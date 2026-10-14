---
id: the-lily-leaf-tessellation-table-moved-on-main
kind: issue
title: demo-tour's lily finding-13 tessellation table fails on main: lily_leaf_b 5576→5374 and lily_leaf_c 5164→4962 triangles at 0.002
status: open
opened: 2026-10-14
priority: P1
cost: E
---


## What

`demos/tour` `lily::review_probes::finding_13_tessellation_table_reproduces` fails on main at `2bb23b3ab9` (reproduced locally). Only the two lofted leaves moved:

| row | pinned | measured |
|---|---|---|
| `("lily_leaf_b", 0.002)` | 5576 | 5374 |
| `("lily_leaf_c", 0.002)` | 5164 | 4962 |

The stem, arch and lantern rows are unchanged.

**Why main's CI shows green:** main's per-PR CI runs demos only when a diff touches them, so it has not run this test. It surfaced on EMIT's PR 5039, whose topo change seeds the demos job.

**Suspected cause (not bisected):** PR 4984 (`nurbs/quotient-second-order`). It changed the mesh crate's rational bounds, `chords`' rational sup|C''| and the patch area quotient floor, and the leaves are rational lofted patches.

**Owed:** confirm the cause, judge whether the new counts are right, then re-baseline the table with the reason or fix the mesh.
