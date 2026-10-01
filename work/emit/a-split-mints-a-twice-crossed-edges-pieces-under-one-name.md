---
id: a-split-mints-a-twice-crossed-edges-pieces-under-one-name
kind: issue
title: A split mints the same-side pieces and crossing vertices of an operand edge it crosses twice under one name, and refuses Duplicate
status: closed
opened: 2026-09-30
priority: P0
cost: M
refs: [a-cylinder-split-refuses-missing-upstream-once-its-pieces-rank, edge-dir-is-a-chord-so-curved-edge-pieces-misrank, curved-seam-pieces-have-no-ranking-direction, a-plane-split-of-a-curved-face-refuses-as-an-emission-bug, edge-pieces-are-named-by-their-ends]
closed: 2026-10-01
pr: 3629
---


## What

A plane crosses a straight edge at most once, but it can cross an arc
twice. The arc's middle piece then lies on one side and its two outer
pieces on the other, and both crossing vertices exist on both sides.
`emit_topo`'s split edge and vertex lane (`name_split_edges_vertices`)
spells each of them from the operand edge alone:

- an edge piece is `SplitFragment { side, parent }`, so the two outer
  pieces share one name;
- a crossing vertex is `CrossingVertex { side, edge }`, so the two
  crossings share one name on each side.

Neither lane has a multiplicity rule, so the second mint refuses
`NamingError::Duplicate`. Unlike the face lane, there is no
`insert_ranked_or_tied` for these.

## Evidence

`crates/editor-core/tests/emit_split_edge_lineage.rs` pins it: a
cylinder (`circle(0, 0, 0.5)` extruded 1.0) split by the plane through
`(0, 0.2, 0)` with normal `(0, 1, -1)`. That plane crosses the start
rim arc twice and leaves the wall and the start cap one piece per side,
so no face ranking is involved. The split refuses `Duplicate` on
`CrossingVertex { Above, RimEdge(Start, Piece(0)) }`.

A scratch probe (not committed) that sent both lanes through the tie
rows let the split evaluate. The only rows it had to tie were:

- `SplitFragment { Below, RimEdge(Start, Piece(0)) }`, two edges;
- `CrossingVertex { Above, … }`, two vertices;
- `CrossingVertex { Below, … }`, two vertices.

When this rule lands, that test should assert the names instead of the
refusal.

## The question

Which rule tells these pieces apart: an `OrderAlong` rank along the
parent, or a tie. It is the split's form of the same question
`curved-seam-pieces-have-no-ranking-direction` asks for seam pieces.
The rank direction `emit_topo::edge_dir` would supply is a chord, and
`edge-dir-is-a-chord-so-curved-edge-pieces-misrank` records why that
is not a direction along an arc over π or a closed edge. So this row
follows that fork's answer rather than choosing one of its own.

Every recipe that crosses an edge twice reaches this, whatever the
face lane does: the cylinder split through `(0, 0.2, 0)`, normal
`(0, 1, 0)`, reaches it too once its same-side wall pieces are let
through (`a-plane-split-of-a-curved-face-refuses-as-an-emission-bug`).

## Ruled (2026-10-01, PR 3553)

Ev took the recommendation of fork-log row 22 ("the recommendation
sounds good, including the change to what was decided in 512!"). The
rule is `crates/editor-core/src/names/README.md` N2. The split's edge
pieces take `Ends`, and its crossing vertices keep an ordinal along the
crossed edge by its carrier's own parameter, oriented as the operand
body stores the edge; equal values tie rather than refuse `Duplicate`.
`edge-pieces-are-named-by-their-ends` implements it and closes this row.

## Closed (2026-10-01, PR 3629)

The split's edge pieces take `Ends` and its crossing vertices an
ordinal along the crossed edge's carrier parameter. The evidence test
asserts the names now
(`emit_split_edge_lineage::a_rim_arc_crossed_twice_names_its_pieces_by_their_ends_and_ranks_its_crossings`).
