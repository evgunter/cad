---
id: kills-remove-half-edges-another-record-names
kind: issue
title: kef, kev and kemr remove their two half-edges proving only the links they rewrite: a third half-edge's next or prev, a loop's first, a vertex's emanating or another edge's slot naming one is left dangling through Ok
status: review
pr: 3592
branch: topo/kill-proves-half-edges
opened: 2026-09-30
priority: P3
cost: M
refs: [kef-kvfs-and-mekr-remove-a-face-shell-solid-or-edge-another-record-names, kef-and-kev-take-a-mate-whose-own-edge-is-another]
---

## What

Found by the receipt of the kill-proofs unit (PR 3570), which proves,
for every loop, vertex, face, shell, solid and edge a kill removes, that
no record the kill keeps names it, and then mapped every production
`.remove(` on a topology arena in `crates/topo/src` to its proof. The
half-edge removals are the ones with no such proof:

- `Body::kef` (`crates/topo/src/euler_kill.rs`, `kef_with`'s kill
  phase), `Body::kev` (`kev_execute`) and `Body::kemr`
  (`crates/topo/src/euler_ring.rs`) each remove the killed edge's two
  halves. Their plans prove the links they rewrite
  (`prev`/`next` of both halves, the re-anchored loops' `first`, the
  endpoints' `emanating`, `Body::require_kill_anchors`), and PR 3570
  proves the edge claims exactly the two
  (`require_halves`, `Body::require_edge_unnamed`). Nothing proves that no OTHER record
  names a killed half: a third half-edge whose `next` or `prev` is
  torn onto one, a loop the kill does not re-anchor whose `first` is
  one, a vertex the kill does not rewrite whose `emanating` is one, or
  an edge other than the killed one whose slot holds one. Each is left
  naming a dead half-edge.

`mekr` removes no half-edge; `kvfs` removes none.

Not measured. `review_d18::kill_anchors_on_torn_bodies` has no column
for a dead half-edge left named; its `NextForeign` tear plants the
first shape, and the release sweep's `LoopAnchorForeign`,
`EmanatingDangling` and `EdgeBijection` tears the others.

## The shape to give

One helper per relation, as PR 3570's are: a scan of the half-edges'
`next`/`prev`, the loops' `first`, the vertices' `emanating` and the
edges' slots for a killed half, outside the records the kill rewrites,
bounded as `Body::require_run_of`'s `Whole` proof is. The refusal is
the variant each relation's decision already has, or
`EulerOpError::KillLeavesDangling` where none does. Add the column to
the probe and require it 0.
