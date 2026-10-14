---
id: main-reds-the-lily-tessellation-table-and-the-saddle-wall-offset-fit-since-4984
kind: issue
title: Main red (slow set and demos, unseen by per-PR CI): the lily leaf tessellation table and the saddle-wall offset fit moved; suspected PR 4984
status: closed
opened: 2026-10-14
priority: P0
cost: E
branch: nurbs/lily-table-after-4984
closed: 2026-10-14
---


## The finding

On FUSE's PR 5030 (head 0fa506959) the FUSE lane hit two red rows that reproduce on a pure `origin/main` worktree at `4533c4b12`. Main's own CI never ran them: the change filter seeds the demos job and a crate's slow set only from what a diff touches, and #5030's topo diff happened to seed both.

- **demos-tour** `lily::review_probes::finding_13_tessellation_table_reproduces` (`demos/tour`):
  - lily_leaf_b: 5576 → 5374 triangles
  - lily_leaf_c: 5164 → 4962 triangles
- **sweep slow set** `saddle_wall_offset_fit::a_saddle_wall_certifies_on_a_grid_longer_in_one_direction_than_a_square_cap` fails.

Neither row has been bisected. The suspect is PR 4984, "one second-order quotient recipe for rational bounds (patch_bound, chords, offset floor)", merged 2026-10-13 at 14:51. It changes the rational chord and patch bounds that drive refinement, and so both the triangle counts and the offset fit.

Reproduce:
- `(cd demos/tour && cargo nextest run -E 'test(finding_13_tessellation_table_reproduces)')`
- `cargo nextest run -p sweep -E 'test(a_saddle_wall_certifies_on_a_grid_longer_in_one_direction_than_a_square_cap)'`

## What it needs

Bisect first. If the new counts are right, re-baseline the table and say what moved. If the saddle fit's refusal or failure is a regression, fix it at its source. Either way, sweep main's skipped slow and demo rows for others of this kind.

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

The saddle half was fixed by PR 5053: the certified bound fell from 3.8534e-10 to 3.8503e-10 because the regularity floor rose (1.97616 → 1.97751), still under the sampled 1.97754.
