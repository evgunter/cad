---
id: the-cone-split-relation-has-two-homes-and-the-vertex-pass-two-ladders
kind: issue
title: The cone-split relation lives in two places, name_boolean_vertices dispatches through two ladders, and Obstacles::records is threaded by hand
status: open
opened: 2026-10-13
priority: P4
cost: M
---


## What

Three shape debts, found in review of PR 4958 (branch
`emit/pair-meeting-run`). None changes a name.

- **One relation, two homes.** The zip's splits per cone
  (`topo::BooleanNaming::cone_splits`, `(split, new)`) are read forward
  in `topo::Settler::settle_cones`, which walks from a vertex to its
  copies, and backward in `emit_topo::Fused`, whose `split_from` map
  reverses the list for `Fused::origin`, which walks a copy back to the
  vertex it copies. Both directions of one relation could live with the
  record: one type in topo answering "the copies of `v`" and "the vertex
  `k` copies".
- **Two dispatch ladders in one pass.** `emit_topo::name_boolean_vertices`
  matches a seam vertex's operand edges first (`let pair = match
  (a_edges, b_edges)`), where the converted arm builds its own
  `Crossing` or `EdgeCrossing` and `continue`s. The arms that fall
  through yield a parent pair, which a second match (`match (pa.kind,
  pb.kind)`) turns into a `Crossing`, an `EdgeCrossing` or a `Seam`. The
  two ladders spell the same heads twice. One ladder from the vertex's
  lines and faces to its head would hold N2's vertex rule in one place.
- **A counter threaded by hand.** `names::borders::Obstacles::records`
  counts the booleans recorded so that a discard's dead boundary edges
  key as `(record, operand, edge)`. Every caller has to record in order
  for the key to mean anything. The record index could come with the
  record it keys.

## Next

Refactor when the vertex pass is next opened. Each item stands alone.

## Re-checked (2026-10-26, tracker sweep)

- **One relation, two homes: fixed.** `BooleanNaming::cone_splits` is
  now `topo::ConeCopies` (`crates/topo/src/lineage.rs`, a `Minted` list
  of `(copy, vertex it copies)`), and `emit_topo::Fused` holds it and
  answers `Fused::origin` with `self.cones.root(k)`; the reversed
  `split_from` map is gone.
- **Two dispatch ladders: re-read before taking.** The two matches this
  row quotes (`match (a_edges, b_edges)`, `match (pa.kind, pb.kind)`)
  are no longer in `name_boolean_vertices` (`emit_topo.rs:2701`); the
  pass now builds its `Crossing`, `EdgeCrossing` and `Seam` heads around
  `:2936`–`:3100`. Whether one head is still spelled on two routes is
  unchecked.
- **The hand-threaded counter: still real.** `Obstacles::records`
  (`names/borders.rs:118`) still counts the booleans recorded.
