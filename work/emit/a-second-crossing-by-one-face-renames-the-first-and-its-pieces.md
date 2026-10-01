---
id: a-second-crossing-by-one-face-renames-the-first-and-its-pieces
kind: issue
title: The crossing ordinal is not local: a second crossing of an edge by a face that already crosses it renames the first crossing and every piece whose Ends cite it
status: open
opened: 2026-10-01
priority: P1
cost: M
design: true
refs: [edge-pieces-are-named-by-their-ends]
---

## What

Edge pieces are named by their ends (`Ends`), which made a cut elsewhere
on a parent leave the other pieces' names alone — unless the cut is by a
face that already crosses the parent. Crossings of one edge by one face
keep an ordinal (`OrderAlong { rank, of }`, `emit_topo::rank_crossings`
→ `insert_ranked_or_tied`, `crates/editor-core/src/names/emit_topo.rs`),
and a lone crossing has none. So a second crossing by the same face:
- renames the first crossing (`CrossingVertex`/`Seam` gains `#k of 2`),
  though it did not move;
- renames every piece whose `Ends` cite it.

Pinned by
`emit_edge_piece_locality::a_second_crossing_by_the_same_face_renames_the_crossing_and_the_pieces_ending_at_it`.
It is the non-locality fork row 22 ruled out for edge pieces, one level
down, on vertices.

## The question

Whether a crossing should take something other than an ordinal over its
group: for instance the crossed edge's pieces or neighbouring crossings
it lies between (which reads beyond the vertex), or an ordinal that does
not spell the group's size. A design fork, to weigh before a lane builds
it.
