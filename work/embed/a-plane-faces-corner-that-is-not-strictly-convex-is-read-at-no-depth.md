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

## Census tests whose partner stops at the lean gate (EMBED lane `embed/iso-end`, 2026-11-22)

A plane face's reflex corner is read by the face's own carrier in the carrier arm, against the partner's presentation, where the partner is presented and lies on one side of the face's plane in the cell; no other first-order separator signs the face's part there (`a-cell-past-an-iso-edges-end-is-clipped-by-its-whole-line`, What a fix needs). Across a shared edge that holds for a valid body; at a vertex alone a valid partner may cross the plane in the quadrant the face lacks, and there the reflex half of this row stands (`embed/corner-pieces`). The census refusals below are partner readings.

**Cylinder bucket, 6 tests.** The cylinder's pair now certifies, and the test refuses at a plane pair whose face exits at `present_plane`'s lean gate:

- `editor-core::all emit_curved_rim_chord::a_cavity_cut_by_the_notched_prism_names_its_rim_pieces`
- `editor-core::all emit_curved_rim_chord::a_post_across_the_other_piece_of_a_notched_circle_unites`
- `editor-core::all wire_legal_union_refusals::two_bars_that_sever_a_sunk_boss_wall_only_together_name_the_glue`
- `editor-core::all wire_legal_union_refusals::two_blocks_that_open_a_sunk_boss_wall_only_together_name_the_glue`
- `editor-core::all emit_union_curved_flush::a_wall_across_two_flush_arcs_publishes_one_table_in_every_order`
- `editor-core::all wire_legal_union_refusals::a_chord_across_the_glued_pieces_line_names_wherever_the_cut_crosses`

**Plane/plane departure census.** Here the refusing pair holds a reflex lean (both < 0) on one face. Of 212 tests (vertex and edge buckets, ≤30 s each) on main `a29fd3695`, with the trace prints on `embed/iso-end-measure`, 31 do. In 13 of them the partner is unpresented (a straight corner, leans exactly 0, or another exit; not classed per test):

- `editor-core::all emit_union_borders::two_members_regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle`
- `step-import::all onb_wild_normal_census::no_wild_face_sits_on_the_frame_seam`
- `step-import::all wild::wild_bodies_are_a_fixed_point_of_our_own_dialect`
- `step-import::all wild::wild_files_import_and_agree_with_the_oracle`
- `sweep::all a_long_bound_beside_a_short_arm::a_boss_beside_a_holes_long_edge_at_every_angle`
- `sweep::all a_long_bound_beside_a_short_arm::a_boss_poking_out_of_a_hole`
- `sweep::all join2_r2_probes::a_channel_whose_arm_ends_join_two_ring_vertices_builds_sound`
- `sweep::all join2_r2_probes::an_island_of_ring_vertex_segments_builds_sound`
- `sweep::all reach_continuation::a_tangency_in_the_middle_of_an_edge_builds_in_either_operand_order`
- `sweep::all rest_zip_admission::a_box_dipping_into_a_plate_at_a_tangent_site_never_ships_the_overlap_twice`
- `sweep::all rest_zip_admission::the_tangent_site_keeps_building_pure_contacts`
- `topo::all a_corner_flat_at_its_short_bounds::a_long_edge_beside_a_corner_flat_at_its_short_bounds_is_classed_where_it_runs`
- `topo::all a_reflex_corner_pierced_through_a_plate::a_reflex_corner_whose_two_pieces_stand_out_of_a_plate_builds_or_refuses_typed`

## 2026-11-22: what the vertex-alone separator reads of a straight corner (embed/vertex-bisector)

At a vertex two faces share alone, `planes_through` parts two quadrant
presentations (`Zero::Point` on both) by the wedge plane
(`wedges_apart`): the point nearest the origin of the hull of the two
corners' axes, one corner's negated. A straight corner presented as a
quadrant has antiparallel axes (`a₁ = −a₂`), so the origin is in that
hull whatever the partner is, and the wedge plane never parts it. A
straight corner read through the `[e]` arm's half-plane at a vertex
alone needs a separator of its own there: one through the corner's
line, leaning off the partner, which the one-line arm builds only
where the line is a shared edge.
