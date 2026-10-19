---
id: the-lily-leaf-tessellation-table-moved-on-main
kind: issue
title: demo-tour's lily finding-13 tessellation table fails on main: lily_leaf_b 5576→5374 and lily_leaf_c 5164→4962 triangles at 0.002
status: closed
opened: 2026-10-14
priority: P1
cost: E
branch: nurbs/lily-table-after-4984
closed: 2026-10-14
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

## Resolution (NURBS build A, 2026-10-14)

**Cause: PR 4984.** Bisected with a fresh `CARGO_TARGET_DIR` per commit: the table passes at `a71403e3` and fails at `2639b944` (4984's merge), with exactly 5374 and 4962. 4984 bounds a rational patch's second derivatives with one Bernstein quotient, tighter than the old recipe.

**The new counts are right.** I ran `mesh::budget` in deviation mode (8 samples per edge) on both leaves at δ = 2e-3, before and after:

| | face | `muu` | `muv` | `mvv` | grid cells | worst certificate | worst sampled \|S − Π\| |
|---|---|---|---|---|---|---|---|
| leaf b, before | 3 | 0.1378 | 0.0426 | 0.5085 | 1728 | 2.58e-4 | 9.1e-5 |
| leaf b, after | 3 | 0.1261 | 0.0234 | 0.5044 | 1632 | 2.35e-4 | 8.4e-5 |
| leaf b, before / after | 4 | 0.0599 / 0.0569 | 0.0158 / 0.0112 | 0.503 / 0.502 | 1152 / 1152 | 2.25e-4 / 2.21e-4 | 5.1e-5 / 5.4e-5 |
| leaf c, before | 3 | 0.1215 | 0.0343 | 0.3386 | 1632 | 2.38e-4 | 7.4e-5 |
| leaf c, after | 3 | 0.1095 | 0.0177 | 0.3354 | 1536 | 2.30e-4 | 8.0e-5 |
| leaf c, before / after | 4 | 0.0517 / 0.0488 | 0.0122 / 0.0084 | 0.335 / 0.334 | 1056 / 1056 | 2.25e-4 / 2.21e-4 | 5.0e-5 / 5.3e-5 |

- **Same sizing path:** the per-cell schedule on the same faces.
- **Why it drops:** the sups it reads are lower, so the blade face builds 96 fewer cells, which is about 200 triangles.
- **Still within budget:** every face's worst certificate and every sampled deviation stays far under the face's `delta_s = 1e-3`.

The table is re-baselined, with the cause, in `demos/tour/src/lily.rs`.
