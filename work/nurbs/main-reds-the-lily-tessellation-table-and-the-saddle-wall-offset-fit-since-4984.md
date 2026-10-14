---
id: main-reds-the-lily-tessellation-table-and-the-saddle-wall-offset-fit-since-4984
kind: issue
title: Main red (slow set and demos, unseen by per-PR CI): the lily leaf tessellation table and the saddle-wall offset fit moved; suspected PR 4984
status: open
opened: 2026-10-14
priority: P0
cost: E
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
