---
id: a-unions-line-of-several-edges-is-oriented-by-its-least-edge
kind: issue
title: A union orients a line by its least edge, so where flush edges or a seam's sides run opposite ways a crossing's sense and a group's ranks read which was minted first
status: open
opened: 2026-10-20
priority: P3
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
and both order as minted. Where the edges of the line run the same way
(every fixture in `tests/name_invariance.rs`) the orientation is one
whichever is least, and nothing moves. Where they run opposite ways, or
a seam's two sides swap order under another authoring history, the
sense flips (`Enters` and `Leaves`) or the ranks reverse, and the table
differs up to the bijection of minted ids: NS1 says no name reads an
order. Unmeasured: no fixture with opposite-running flush edges or a
swapped-side seam crossing group is in `name_invariance.rs`'s history
set.

## What it needs

An orientation of a line that is the same in every authoring history,
or a sense and rank that need none (an unordered pair of the two sides'
classifications, say), decided under N0 (PR 5455); then a history
fixture with opposite-running flush edges to pin it.

