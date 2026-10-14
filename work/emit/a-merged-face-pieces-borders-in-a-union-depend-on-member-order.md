---
id: a-merged-face-pieces-borders-in-a-union-depend-on-member-order
kind: issue
title: A union's merged-face piece borders different walls by member order (cross: 28 absent names)
status: closed
closed: 2026-10-13
opened: 2026-10-12
priority: P2
cost: M
---


## What

Found building the vertex run (branch `emit/vertex-run`, 2026-10-12),
which made every union vertex and seam piece order-free. In
`crates/editor-core/tests/emit_union_rim_piece_ranks.rs`'s `cross` case
(`KNOWN_ABSENT`), 28 names are still published in some fused orders and
absent in others: 2 names in each of 14 orders, 4 distinct names, every
one a merged face piece `Merged([…]) + Fragment(Borders([…]))`.

- The merged top caps of two members border one other member's wall in
  2 orders, and that wall and a fourth member's in the other 12.
- The merged bottom caps of the other two members border one merged
  wall in 2 orders, and that and both members' own walls in the other 12.

No name rebinds: `a_name_two_member_orders_both_publish_denotes_the_same_geometry`
holds every name two orders both publish to one geometry.

## Where to look

A piece's `Borders` set is read over every fold step's discards
(`names/borders.rs`, `Obstacles::split`, fed by `emit_union::Fold::step`'s
`obstacles.record`). Which walls a discard records, and so which the
piece borders, looks to depend on the step that discarded them. Check
whether the set should be read off the finished body's walls at the
piece rather than off the fold's discard rows.

## Closed (branch `emit/union-order`, 2026-10-13)

**Cause.** The `Borders` set is read over the fold's discard rows, and
each piece edge is chased back to the discards recorded along it
(`names/borders.rs`, `Obstacles::walls`). Two links in that chase
were missing, both member-order dependent:

- An edge the output stage joins at a later step is a new edge; the
  seams it was made of are recorded only on the earlier discards. The
  chase now follows `EdgeJoin` (`kept` to every `gone`), and the
  earlier discards' seams follow the join to the edge it kept.
  Discards that one piece edge reaches are one obstacle: both border
  it on its far side.
- A seam a later step joined away is dead in the finished body, so
  `Body::edge_provenance_of` no longer tells which edge it was split
  from. `Obstacles::record` now keeps the `SplitEdge` lineage of every
  recorded step's result (`split_from`), beside the chains the
  discards carry.

In `cross` the merged top caps of `a` and `b` lose the strip under the
bar to two discards, one per member. In the orders that fold `b` last,
the right piece's edge along the bar is a join of `a`'s seam piece and
`b`'s, and the chase reached only `b`'s discard. That discard
bordered one piece, so the bar's wall was not cited.

The set is still read off the discard rows, not off the finished body
alone. With both links in place the rows give the obstacle that N2
describes (a connected part of the parent's lost region) in every order
of the corpus.

**Measured** (`emit_union_rim_piece_ranks`):
- `cross`: 28 absences before (2 names in each of 14 fused orders),
  0 now. The 10 orders that refused `Emission` ("a piece of a face
  held as several borders no recorded discard between them") also fuse.
  All 24 orders now publish one table.
- `KNOWN_ABSENT` and `KNOWN_MIXED` are gone. The row now asserts that
  no case has an absence or a mixed refusal.
- `emit_union_flush_names`'s `KNOWN_REFUSING` is empty.
- `emit_shared_rim_several` fuses all 282 cells, up from 272. Its
  `RESIDUES` admission is gone, and any refusal fails it.
- `union_member_orders_decide_alike`'s emit-corpus row no longer skips
  `cross`, and it decides alike in every order.
