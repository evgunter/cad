---
id: two-dangling-null-edges-with-one-segment-refuse-shared-vertex-crossings
kind: issue
title: Two dangling null edges with one segment refuse SharedVertexCrossings: which holds the other is not read off the geometry
status: open
opened: 2026-10-03
priority: P1
cost: M
---


## What

At a vertex that several crossing pairs cut, two pairs whose
dangling null edges have one segment are two pieces touching along
both of its ends' directions (pinch lines in the corner's face). Each
strut's other way round is the whole orbit, so
`insert::reconcile_shared` (`crates/topo/src/boolean/insert.rs`)
refuses `SharedVertexCrossings`, as on main. Pinned:
`two_dangling_null_edges_with_one_segment_refuse_typed`
(`crates/topo/tests/union_flush_onto_edge_contact.rs`), with `y`
built both ways, plus the inner lens notched, with and without a lens
in the notch.

## What was tried (FUSE, PR 3950)

A strut whose segment holds another whole hangs the inner one at its
tip (`insert::holds_whole`). That builds when the inner one's segment
is strictly inside at one end at least. With both ends tied, each
strut holds the other, and one has to be chosen:

- **By mint order.** It builds with `y = cut ∪ lens`. With
  `y = lens ∪ cut`, all six ops refuse `JoinDesync` ("every chord arc
  separates a loose scaffolding pair").
- **By the run's side code (`from.1.0`).** Same failure. That code is
  not geometric for a strut whose germs share a sector entry: the
  union order reverses the record order, which flips it.
  Normalizing every shared strut to leave from its lower germ flips
  the side codes, and nothing nests.

The build that worked had the lens's strut (In on the segment)
holding the block's (Out), the reverse of strict nesting, where the
Out run holds.

The end guard passes either way round. The record order also flips
every end kind (the dangling-strut side swap in `insert::mint_run`),
so the holder's tip is its In end in one order and its Out end in
the other.

## Owed

Work out what the join needs at a tip whose region is empty, then
choose the holder from geometry, not from record order. Flip the pin
to building in all six ops, with `y` built both ways.
