---
id: seam-chain-ranks-are-oriented-a-first-so-an-operand-swap-may-reverse-them
kind: issue
title: Seam-edge chains and same-pair seam vertices are ranked along an A-first direction, so swapping a union's operands may reverse their ranks
status: open
opened: 2026-09-23
priority: P3
cost: E
---


## What

`crates/editor-core/src/names/emit_topo.rs` ranks two same-name groups
along a direction fixed by operand ORDER, not by the geometry:

- `name_boolean_edges`, collinear seam-edge chains: `dir = na × nb`,
  with the A-descended face first ("A side first — descent decides which
  is which"). Swapping the operands swaps `na` and `nb`, which negates
  `dir`. `order_along` ranks by the signed parameter, so the chain's
  `OrderAlong` ranks reverse.
- `name_boolean_vertices`, same-pair seam vertices: ranked along
  `resolve_edge_carrier(&pa, a)` and falling back to B's parent, so the
  carrier is A's parent edge whenever it has one. After a swap it is
  the other operand's.

So `Seam{x, y}` rank 1 in `x ∪ y` can be the geometry named
`Seam{y, x}` rank 2 in `y ∪ x`.

Found while sweeping for `b-arena-edges-skip-the-split-lineage-chase`.
It is not a key-layout shortcut: both layouts follow the same A-first
convention. Related: `work/wire/the-pair-verbs-declared-merge-is-asymmetric-in-its-operands.md`
(a ruling on the same class of operand asymmetry, for declared merges).

## Band

P3 while unmeasured. None of the six fixtures in
`swapping_the_operands_swaps_the_sides_of_every_name`
(`crates/editor-core/tests/emit_boolean_vertex_keys.rs`) has a
multi-edge seam chain or a same-pair vertex group, so that row cannot
see this yet.

## First step

Add a fixture with a seam line cut into several collinear pieces (for
example a slab crossing two parallel ribs) to the swap row. If the row
goes red, the question of whether names are meant to survive an operand
swap belongs with the wire ruling above, and the band moves with it.
