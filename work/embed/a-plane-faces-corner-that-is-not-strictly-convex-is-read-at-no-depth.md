---
id: a-plane-faces-corner-that-is-not-strictly-convex-is-read-at-no-depth
kind: issue
title: present_plane reads a two-line corner only where it is strictly convex, so a vertex on a straight run of a plane face's boundary, or a reflex corner, is read by no presentation at any depth and a valid body refuses at its door
status: dispatched
opened: 2026-11-21
priority: P0
cost: M
branch: embed/corner-pieces
---


## Finding

Traced by the planar lane of the per-solid census
(`embed/trace-planar`, 2026-11-21), on `embed/gate-measure` at
`1ca743418c` with diagnostic exit lines (`embed/trace-planar-measure`).

`topo::injective`'s `present_plane` (`crates/topo/src/injective.rs`)
reads a plane face's part of a cell in two shapes only: one line edge
(the `[e]` arm), or two line edges at a vertex whose corner is strictly
convex (the `[e1, e2]` arm). The second arm asks both `lean(n1, bv)` and
`lean(n2, a)` to decide `Positive` at `SELF_INTERSECTION_CORNER`, and
returns `Unpresented::Local` otherwise. Two corners of a valid face fail
that gate, and every smaller cell holding the vertex holds the same two
edges, so they fail it at every depth:

- **a straight corner**, a vertex in the middle of one straight run of
  the face's boundary. A Boolean leaves one wherever it splits a face's
  edge at a vertex of the other operand. Both leans are exactly 0.
- **a reflex corner**. Both leans are negative. That the arm does not
  present a reflex corner as its quadrant is right, and
  `a_planes_reflex_corner_of_two_lines_is_not_its_wedge` holds it. But
  no other presentation reads that corner either.

When the pair's shared cell is that vertex, or an edge ending there, the
pair is `Split { Departure }` down to the depth floor and the walk
refuses it as `Uncertified { arm: Departure }`.

## Trace

Cheapest: `topo::all review_m9_1_probes::probe_records_partialeq_bites_on_mutation`
(0.11 s). It is the union of the bricks `[0,1]³` and
`[0.5,1.5] × [0.25,1.25] × [1,2]`, which is one valid solid. The gate
refuses `Uncertified { a: 4v1, b: 10v1, shared: Vertex(14v1), arm: Departure }`.
Here vertex 14 is `(1, 0.25, 1)`, face 4 is `a`'s side `x = 1` and face
10 is `b`'s side `y = 0.25`. The two walls meet at that point only. On
each of them the point splits a straight edge in two (face 4's top edge,
and face 10's bottom edge), so each face has a straight corner there.
Of the 152 asks of that pair, 134 exit at the lean gate on both faces.
The rest have more than two edges in the cell, coarser. At the finest
cells (`u` width 2⁻¹⁰) every lean the gate read was exactly
`0.000e0`: 627 straight-corner exits in the test, against 4 reflex ones.

## Count

On the census of main `c2a0c41f40`:

- **departure, vertex, plane/plane: 57 of the 96 traced** (of 101).
  The refusing pair's finest cells exit at the lean gate on one face or
  both. Re-run for the lean values: 43 tests meet straight corners
  (lean exactly 0) and 10 meet only reflex ones. The other 4 are the
  slow Boolean corpus rows and were not re-run.
- **departure, edge, plane/plane: 32 of the 111 traced** (of 116).
  These exit at the same gate at a shared edge's end, on one face or
  both. Their lean values were not re-run.

- **`Budget(Depth)`, probably.** In 2 of the 10 depth-bucket tests
  re-run, the deepest touching pairs exit at the lean gate. In
  `verbs_f7_r2_probes::r2_attack_midvertex_chord_split` that is 251 of
  413 asks, and in `holes_meeting_at_a_vertex::runs_one_between_others_build_in_every_op`
  59 and 34 of 106. The census reads `Budget(Depth)` there because a
  distance pair beside them reaches the floor first. Not checked by a
  flip.

The families are the flush unions and their naming rows
(`emit_union_rim_piece_ranks`, `emit_seam_junction`,
`union_flush_onto_edge_contact`), the slits and keels
(`a_slit_tip_on_a_curved_face_reads_its_full_turn`,
`a_keel_on_a_bores_ruling`, `a_keel_on_the_floor_subtracts_as_a_slit`),
`m9_1` and `m3_pr6` records, `three_solids_on_one_line` and
`split_pinch_keeps`. Every one is an ordinary Boolean result.

## What a fix needs

The straight corner is one line. With the two collinear edges read as
a single edge through the vertex, the `[e]` arm's half-plane presents
it, with `Zero::Side` over the shared edge or `Zero::Point` at the
vertex. The reflex corner is a union of two half-planes, not their
intersection. It wants a presentation of its own, or a split of the
cell at the vertex into two convex sectors, each read as a convex
corner against the partner. The quadrant reading stays refused
(`a_planes_reflex_corner_of_two_lines_is_not_its_wedge`).

Cheapest edge reproducer:
`topo::all records_cite_their_decision::a_cell_reached_from_both_sides_cites_its_shortest_chain`
(0.12 s).
