---
id: seam-chain-ranks-are-oriented-a-first-so-an-operand-swap-may-reverse-them
kind: issue
title: Seam-edge chains are ranked along an A-first direction, and the union's Seam canonicalization keeps those ranks, so reordering a union's members silently rebinds OrderAlong names
status: dispatched
opened: 2026-09-23
priority: P0
cost: E
branch: emit/seam-chain-ranks
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

For a pair boolean this is loud. `Seam{x, y}` becomes `Seam{y, x}`, so
no name survives the swap with a different binding.

`Node::Union` is the silent case. `emit_union.rs` canonicalizes the
`Seam` pair by name order and keeps the `OrderAlong` tail as the pair
emitter ranked it. The same name therefore binds different geometry
depending on member order.

## Measured (PR #3114's reviewer)

Fixture `slab_rib`:
- slab: [0,3]×[0,2]×[0,1];
- rib: an inverted U with arms at x∈[0.5,1] and x∈[2,2.5], z 0.5..2, y 0.5..1.5.

The seam is a 2-edge collinear chain on each of the rib's two caps.

`Node::Union` with members `[slab, rib]` against `[rib, slab]`: the
identical name `Seam{M2.Cap(End), M5.Cap(End)}#OrderAlong{0 of 2}` binds
the seam edge at x∈[0.5,1] in one order and x∈[2,2.5] in the other.
Four names swap bindings this way. It is a live, silent rename.

Scratch: `~/.local/share/cad-work/emit-barena-review/`
(`review-probes.patch`, `union.txt`).

Found while sweeping for `b-arena-edges-skip-the-split-lineage-chase`.
It is not a key-layout shortcut: both layouts follow the same A-first
convention. Related: `work/wire/the-pair-verbs-declared-merge-is-asymmetric-in-its-operands.md`
(a ruling on the same class of operand asymmetry, for declared merges).

## Fix shape

At the union collapse, where the `Seam` pair is canonicalized: when
canonicalization swaps the pair, re-rank the `OrderAlong` tail against
the canonical orientation (`rank' = of − 1 − rank`, if the chain
direction is exactly `na × nb`; verify that). The pair emitter's own
names do not move. Also measure same-pair seam vertices under a member
swap.

Red row: the `slab_rib` union in both member orders, asserting each
name binds the same edge.
