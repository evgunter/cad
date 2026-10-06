---
id: a-second-crossing-by-one-face-renames-the-first-and-its-pieces
kind: issue
title: The crossing ordinal is not local: a second crossing of an edge by a face that already crosses it renames the first crossing and every piece whose Ends cite it
status: review
opened: 2026-10-01
priority: P1
cost: M
refs: [edge-pieces-are-named-by-their-ends]
pr: 4203
branch: emit/crossing-sense
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

## Ruled (Ev, PR 4134, 2026-10-06)

Ev approved the recommendation with the same-sense crossings ranked along
the edge (option A), in the names README (N2 *Vertices* and *Edge pieces*,
the flush paragraph, and N5's group-size bullet; fork-log row 73).

**What to build** (the sense first; `Ends`-always must not land before it):
- **Every crossing carries its sense.** The sense is whether, at the vertex,
  the crossed edge, in its stored orientation, enters or leaves the closed
  body the crossing face belongs to. A side with no portion of the edge
  counts as outside. It is read at the minting step and carried through a
  union's collapse, and it flips where `RankRule::Reverse` reorders a seam
  pair.
- **Spelling.** The edge × face vertex of a boolean or union gets a new
  `Crossing { edge, face, sense }` head, replacing `Seam { a, b }` there.
  The Split's `CrossingVertex` gains a `sense` field. An edge × edge vertex
  carries each edge's sense against the other operand's closed body.
- **Same-sense crossings** of one edge by one face keep `OrderAlong` over
  that group only. An equal pair ties.
- **Recording the classification.** The boolean records each operand edge
  piece's in/on/out classification as a birth row (D5). This is the clean
  form; deciding it with `point_in_solid` is the fallback.
- **`Ends` on every piece of a divided edge,** a lone one included. Drop the
  `[one] ⇒ base` shortcut in `emit_topo::name_edge_pieces`.
- **Tests.**
  - Rebuild the pinned test
    (`emit_edge_piece_locality::a_second_crossing_…`) as ONE document edited
    in place (`SetParam` on the datum). Today it compares two separately
    built documents whose Split ids differ, so it proves nothing. Assert
    that P and its pieces keep their names.
  - Add a row for the in-face union vertices in `wire_legal_union_refusals`
    (an end-touch: the sense is read at minting).
  - Add a row for a same-sense group.
- **Goldens.** They move broadly: every crossing vertex, every `Ends` that
  cites one, and every lone edge piece. Re-baseline them and say what moved.
