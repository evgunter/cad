---
id: a-merged-face-pieces-borders-in-a-union-depend-on-member-order
kind: issue
title: A union's merged-face piece borders different walls by member order (cross: 28 absent names)
status: open
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
