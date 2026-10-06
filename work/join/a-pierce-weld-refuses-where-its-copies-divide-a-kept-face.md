---
id: a-pierce-weld-refuses-where-its-copies-divide-a-kept-face
kind: issue
title: The pierce weld refuses where a kept face's outer loop runs through two of a pierce's copies (214 runs on 315° and 225° corners), and a holed difference refuses 'two fragments of a pierced face meet one pinch'
status: closed
opened: 2026-10-04
priority: P0
cost: H
refs: [a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op]
closed: 2026-10-06
---


## What

Found by PR 4026's review r2 (m1), on `join_pierce_r2_probes.rs`
`r2_shapes_battery` and `r2_holed_battery` (branch
`join/pierce-two-out-runs-review-r2`).

1. **`JoinDesync { "a pierce's copies divide a face the zips kept" }`**
   (`boolean/finish.rs` `weld_pierce_copies`). Review r2 counts 214
   shapes lines, all ∪, in both orders, every one on the 315° and 225°
   reflex corners (R315 82, R315m 70, R225m 32, R225 30). On main they
   refused `kept_end`. After the zips, a kept face's OUTER loop runs
   through two of one pierce's copies, so welding them would divide
   that face into two regions meeting at the vertex. `weld_pinches`
   records such a division as a fragment row
   (`FinishOut::weld_fragments_*`). The post-zip pass refuses, because
   it has no fragment row to write after the graft.
2. **`JoinDesync { "two fragments of a pierced face meet one pinch" }`**
   (`pinch_site`, reached through the weld): the holed block's
   difference with the cube, in the edge-run family.

## The shape to give

Item 1: record the division's fragment row in result keys (an A face
is its own key; a B face goes through `graft_faces`), and admit the
`Chord` weld after the zips. Then rerun the shapes battery: the 214
lines should build, or show what else stands. Item 2: diagnose first.

## Reach on main `4cf3f8b9` (`join/pinch-uncrossed-residue`)

PR 4038's review batteries (`r2_pinch_probes`, `r1b_pinch_probes`) and
PR 4026's review r2 holed battery, re-run on main after PRs 4036 and
4038. Unchanged on that lane's head. This row's two refusals are the
other place a pinch is left uncrossed, after the zips rather than
before them:

- **Item 1** ("a pierce's copies divide a face the zips kept"), 260
  lines, every one a union in both orders apart from the holed block's:
  r2 cube 48, r2 near-tangent 66, r2 cylinder 40, r1 cube 92 (all ∪);
  holed 14 (block ∖ cube).
- **Item 2** ("two fragments of a pierced face meet one pinch"), 22
  lines, all block ∖ cube. One is pinned in
  `join_pierce_runs_sweep::an_island_face_pinched_to_its_holes_ring_crosses_and_builds`
  (`holed c00 side=4 g6.0`, whose cube ∖ block now builds).

## Built (branch `join/pinch-one-vertex-per-cone-build`)

Under Ev's ruling on PR 4057 (one vertex per cone), the post-zip
pierce weld (`finish::weld_pierce_copies`) retires. A pierce's copies
that a face meets stay apart, on one point key, and the face runs
through both.

Item 1 ("a pierce's copies divide a face the zips kept"), main
`f9bf3bca` vs head, release:
- r2 shapes battery: 214 → `SOUND`;
- r2 cube: 48 → `SOUND`;
- r1 cube: 92 → `SOUND`;
- holed: 14 → `SOUND`;
- r2 near-tangent: 66 → 59 `SOUND` and 7 `BAD` (tier-3′ `CensusEscalated`);
- r2 cylinder: 40 → 38 `BAD` on the operand check only (the curved
  legal-operand class) and 2 `ResultInvalid { VolumeUncomputable }`.

Item 2 ("two fragments of a pierced face meet one pinch"): holed 22 →
`SOUND`.

PR 4051's island families add 162 item-1 lines and 156 item-2 lines,
all → `SOUND`, and 4 item-1 lines on its cylinder walls → operand-only
`BAD`. Every newly built planar body meshes.

Pinned by `join_pierce_strut_facing`'s rows, each of which now
tessellates its bodies:
- `two_out_runs_at_the_corner_build_in_every_op`;
- `a_wide_run_builds_in_every_op`;
- `two_edge_runs_build_in_every_op`;
- `a_pinch_keeps_one_vertex_per_cone`.
