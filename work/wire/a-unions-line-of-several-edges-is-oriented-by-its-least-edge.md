---
id: a-unions-line-of-several-edges-is-oriented-by-its-least-edge
kind: issue
title: A union orients a line by its least edge, so where flush edges or a seam's sides run opposite ways a crossing's sense and a group's ranks read which was minted first
status: closed
opened: 2026-10-20
priority: P2
cost: M
branch: wire/nothing-is-ranked
closed: 2026-10-26
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

The design answer is ruled (below). What is left is the build: the
emitters and `canonical.rs` brought to N2 as it now reads, and the 18
admitted names in `name_invariance`'s history case retired with the
admission.

## Ruled

FORK-ORIENT (`docs/DESIGN-FORK-LOG.md` row 197), Ev, 2026-10-26: the
no-rank rule ("cool, seems good!", after asking for the simplest stable
rule and for measurement first). Landed in N2
(`crates/editor-core/src/names/README.md`) by PR 5705, merge
`964083e5`:

- An orientation is read, never chosen (N0 NS1). A line of one edge is
  read as the operand body stores the edge; a line of several things (a
  `Merged` line of flush edges, or a seam, whose two sides' loops run
  opposite ways) is read along each of them.
- A transversal crossing of such a line is `Through { entering }`: the
  subset of its edges, or its seam's sides, in name order, whose
  direction runs into the body. A touch is `Outside` or `Inside` on any
  line. A seam in a run carries its sense as any line of several things
  does.
- Nothing is ranked: crossings, and vertices of one run, that their
  names do not tell apart tie (N4).
- `Reverse`, `flip_senses`, `RefuseRun` and `OrderAlong`'s `of` retire.

Measured before the switch (fork-log row 197): no new tie in the
integration tests, and the 18 seam-run groups all told apart.

The code is not built that way yet (verified on main at `d621f67e`):
`Flush::line_sense` (`crates/editor-core/src/names/emit_union.rs:1144`),
`rank_along_seam` (`emit_union.rs:1930`), `RankRule::{Reverse,
RefuseRun}` and `flip_senses` (`crates/editor-core/src/names/canonical.rs:70`,
`:276`) and `rank_crossings` (`crates/editor-core/src/names/emit_topo.rs:5231`)
all stand. This row is that build.


## Closed by PR 5758 (2026-10-26)

`wire/nothing-is-ranked` builds Ev's PR 5705 rule: an orientation is read,
never chosen, and nothing is ranked. A `Merged` line of flush member edges crosses `Through { entering }` the edges, or seam sides, whose direction enters; `line_head` and the least-edge reading are gone, and `name_invariance`'s admission for "plate pinch, p2 extruded down" is removed (`ADMITTED_HISTORY` is empty). The status change is the
orchestrator's.

Closed by PR 5758 (2026-10-26, "nothing is ranked", Ev's ruling on PR 5705). An orientation is read, never chosen: `Through { entering }` on a line of several things, a seam in a run carries its sense, and nothing is ranked. `ADMITTED_HISTORY` is empty.
