---
id: movefac-reads-a-mate-and-a-neighbour-it-never-proves
kind: issue
title: "movefac's labelling takes a mate from the edge's slots and a neighbour face from the mate's loop, and proves neither is the shell's"
status: open
opened: 2026-09-30
refs: [movefac-labels-components-by-an-unproven-cycle-walk, kef-and-kev-take-a-mate-whose-own-edge-is-another]
priority: P3
cost: E
---

## What

Found by the receipt pass of
`movefac-labels-components-by-an-unproven-cycle-walk`, which proves
each cycle walk of `Body::movefac`'s labelling
(`crates/topo/src/movefac.rs`, the precondition phase of `movefac`)
is its loop's whole cycle. Three other reads of that labelling carry
no proof:

- **The mate hop.** `Body::mate(member)` checks that the member's
  edge claims it and returns the edge's other slot. Nothing checks
  that the mate's own `edge` is that edge, the check `kemr` makes as
  `NotSameEdge`.
- **The neighbour.** The mate's `parent_loop`'s `face` is proven
  live (`require_key`) and never proven to be in the shell. A
  neighbour in another shell (tier 1's pass 10 fault) is labelled and
  its loops walked, so the labelling can join two components through
  a foreign shell. The move itself builds its lists from the shell's
  face list, so the foreign face is not moved, but the join stands.
- **A face's loops.** The walk glues a face to the loops its
  `outer`/`rings` list, and a mate to the face its loop's `face`
  names; nothing checks that the listed loops name that face back.

## Measured

On one-shell bodies the mate hop cannot misread: a torn edge that
claims a foreign mate stops claiming the half that still names it, so
that half's own walk refuses `StaleKey`. The labelling proof's tear
sweep (`movefac::tests::tears::movefac_on_torn_bodies`, seeds
1..=2000, one and two `EdgeBijection` tears on seven one-shell
bodies) sees 28000 calls, 27174 `Err` and 826 `Ok`, none of which
joins or splits the records' components, before and after the
proof. What escapes it is a mate or neighbour outside the shell's
walked loops, which the sweep's bodies never hold. Not measured.

## The shape to give

The labelling proves the mate names the member's edge, and that the
neighbour is a face of the shell (its `shell` is the shell, or the
shell lists it), refusing typed otherwise; pin a torn bijection whose
stranded half lies in another shell, and a neighbour in another
shell.
