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
