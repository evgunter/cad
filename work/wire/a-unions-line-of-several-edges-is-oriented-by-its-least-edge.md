---
id: a-unions-line-of-several-edges-is-oriented-by-its-least-edge
kind: issue
title: A union orients a line by its least edge, so where flush edges or a seam's sides run opposite ways a crossing's sense and a group's ranks read which was minted first
status: open
opened: 2026-10-20
priority: P2
cost: M
---

Filed by WIRE's `wire-mintfirst` lane (branch
`wire/union-reads-not-mint-order`), from the sweep of
`crates/editor-core/src/names/emit_union.rs` for mint-order reads.

## What

Since that branch, a union names what several members hold at once for
the set of them (`Merged`), never the least, so no rule there chooses an
ENTITY by name order. Three rules still ORIENT a line by the least of
several things on it, and a name's sense or rank is read along that
orientation:

- `Flush::line_sense` and `line_head`: a line of several member edges
  flush on one line (`Merged` of them) carries the sense read along its
  least edge, and a group of crossings of it is ranked along that edge
  (`cite_member_edges`);
- `cite_member_edges`: a group sharing one `EdgeCrossing` run is ranked
  along the run's least line that carries a sense
  (`run.iter().find(|l| l.sense.is_some())`);
- `rank_along_seam` and N2's seam convention: a seam is oriented as the
  loop of its first side, the lesser name, runs along it.

Name order compares the minting node's id first, then a `From`'s read,
and both order as minted. Where the edges of a line run the same way the
orientation is one whichever is least, and nothing moves. Where they run
opposite ways the sense flips (`Enters` and `Leaves`) or the ranks
reverse with the authoring order, which NS1 forbids.

## Measured

`tests/name_invariance.rs`, history case "plate pinch, p2 extruded
down": `union_pinch_member_order::pinch` with `p2` the same solid
extruded `ExtrudeSide::Against` from its top (z = 1.7), so its leg runs
downward while `p1`'s runs up. The union lists `[plate, p1, p2]` in every
document. The pinch vertex on the plate's top is
`Crossing { edge: Merged{p1 leg, p2 leg}, face: plate Cap(End), sense }`:
`Leaves` when `p1` is authored first, `Enters` when `p2` is. That
vertex, and the leg pieces whose `Ends` cite it, are 18 names over the
five other authoring orders, admitted there against this row; the
admission goes red when this is fixed.

## What it needs

A design answer under N0 (PR 5455): what orients a line of several
edges that no mint order may choose, or a sense and rank that need no
orientation (an unordered pair of the two sides' classifications, say).
The names README states only that such a line's sense is read along one
of its edges and that which one is open, pointing here.

