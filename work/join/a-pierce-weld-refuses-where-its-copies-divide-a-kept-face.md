---
id: a-pierce-weld-refuses-where-its-copies-divide-a-kept-face
kind: issue
title: The pierce weld refuses where a kept face's outer loop runs through two of a pierce's copies (214 runs on 315° and 225° corners), and a holed difference refuses 'two fragments of a pierced face meet one pinch'
status: open
opened: 2026-10-04
priority: P0
cost: H
refs: [a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op]
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
