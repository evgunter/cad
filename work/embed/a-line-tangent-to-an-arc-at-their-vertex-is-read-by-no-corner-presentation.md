---
id: a-line-tangent-to-an-arc-at-their-vertex-is-read-by-no-corner-presentation
kind: issue
title: A plane face's corner where a line continues its shared arc tangentially is read by no corner presentation, so every fillet band whose rim arc meets its straight foot at a tangent joint refuses Departure at the joint
status: dispatched
opened: 2026-11-21
priority: P0
cost: M
refs: [departure-reads-no-curved-corner-off-its-shared-edge]
pr: 7320
branch: embed/tangent-corner
---


## Finding

From EMBED's per-solid census trace (lane `embed/trace-cone-torus`,
2026-11-21), main `c2a0c41f40` with the gate and its trace switches,
plus prints at each of `graph_read`'s exits
(`embed/trace-cone-torus-measure`).

A fillet band's corner patch is a torus face whose foot on the support
plane is an arc, joined at each end to a cylinder fillet's straight
foot. The pair is the plane against the torus across the arc; it splits
to `Depth { max_cell_depth: 40 }` and refuses `Uncertified { Departure }`.

**The cell.** In `band_mixed_corner::an_inset_corner_with_all_three_edges_requested_builds_at_its_closed_form`
the last pair is the plane `1v1` at `u ∈ (−0.10059, −0.09961)`,
`v ∈ (0.09961, 0.10156)` against the torus `15v3` at its parallel
`v = 3π/2`. The plane cell holds the vertex `21v1` where the shared arc
`33v1` ends and the next foot begins. The torus is presented by its
parallel (`Zero::Side(YHi)`); the plane is `Local`.

**The exit.** `present_curved` reads the arc through the vertex
(`through_of`) and `graph_read` fails in both columns:
- column `V` (the arc as `u` over `v`): `monotone_run` fails. The arc's
  `v` over the cell is `(0.099759, 0.1)`, and `0.1` is its value at the
  vertex: the arc's `v` turns at the vertex, so it is no graph over `v`;
- column `U` (as `v` over `u`): the run is chosen and is monotone, and
  `graph_read` then returns `None` in its corner block (after `apart`
  and `alone`, which do not fire): the premise check on the vertex or
  `leaves_graph`.

So `present_curved` returns `Local` at every depth, and the corner is
in every cell along the arc's end.

The same cell shape in two more traced rows of the family,
`band_mixed_rim_shared_supports::a_cross_drilled_plates_top_rim_fillets_with_the_bore_on_its_mates`
and `band_closed_mixed_chain_shapes::a_creased_corner_refuses_where_definite_and_builds_below_the_band`:
the arc at its turning point at the vertex in one column, and the other
column's run lost in the corner block.

**The cause, partly checked.** The arc's `v` turning at the vertex is
read off the traced reaches. That the next edge leaves the vertex along
the arc's tangent is the fillet band's construction (a foot is tangent
to the next at a joint), not measured. If so, `leaves_graph`'s rate of
the line off the continued arc is zero at the vertex, which no
first-order reading signs. Which clause of the corner block returns is
unchecked.

`departure-reads-no-curved-corner-off-its-shared-edge` covers corners
of two different conics and a plane's line edge meeting an arc; this is
a curve read through a corner whose other edge continues it at first
order.

## Count

24 tests, all `band_*` and `blend_recourse_followability` fillets with
no Boolean record (departure at an edge, plane/torus, 23 f64 and one
`Interval`). Three traced. Not counted: the two `lamina_annulus` rows
of that bucket, whose plane cell is `Local` at `beside_curve`'s last arm
(two curved walls, no corner read), untraced.

Possibly the same geometry on a plane against a cylinder, unchecked:
`a_split_through_a_wrap_edges_vertex::a_plane_tangent_to_a_two_arc_holes_rim_splits_sound`
refuses at a vertex alone, the plane's corner `Local` at
`present_plane`'s two-line corner lean check (the plane cut tangent to
the rim at a vertex). That module has five rows in the vertex,
plane/cylinder bucket.

The bodies are valid: fillets built at their closed forms.

**Cheapest reproducer:** `sweep::all band_mixed_corner::an_inset_corner_with_all_three_edges_requested_builds_at_its_closed_form`
(0.35 s).

## What is needed

A corner reading for a curve continued at first order by its next edge:
the next edge's departure from the curve read at second order (its
curvature against the arc's), or the two edges read as one `C¹` run
through the vertex.

## Census tests the cylinder fix leaves at present_curved and graph (EMBED lane `embed/iso-end`, 2026-11-22)

In these 9 the cylinder's pair is refused with the partner, a plane face, unpresented at `present_curved` (no column's graph reads, `graph_read`'s `Local`). Six are the tangent-boss documents (`intent_s6_*`, `coincidence_door`'s boss), built tangent by construction. In the last three (a rod glued in its bore, a rod at a box edge, the ring lane) the exit is the graph's; whether they are tangent was not checked.

Measured on `embed/iso-end-measure2` (`origin/embed/gate-measure` @ `ebb2e6f8e` plus `embed/iso-end`'s diff), where each refusal is the same pair, test by test, as on the trace run that named the exits (`embed/iso-end-measure` @ `030c75763`, `CV_TRACE`).

- `editor-core::all coincidence_door::a_boss_typed_tangent_to_a_cylinder_glues_one_unproven_tangency`
- `editor-core::all intent_s6_d_tangency::a_boss_traced_off_the_walls_axis_is_tangent_by_construction`
- `editor-core::all intent_s6_d_tangency::a_boss_traced_on_one_cylinder_is_unproven_against_its_image`
- `editor-core::all intent_s6_d_tangency::a_boss_traced_on_the_walls_other_half_proves`
- `editor-core::all intent_s6_d_tangency::a_heading_typed_along_the_tangent_is_unproven`
- `editor-core::all intent_s6_d_tangency::a_frame_slid_spun_or_flipped_along_the_axis_still_proves`
- `sweep::all bisector_nodes::a_rod_glued_in_its_bore_answers_alike_in_either_order`
- `sweep::all curved_coincident_fates::a_rod_at_a_box_edge_borders_alike_in_either_order`
- `sweep::all verbs_germarms_interval::the_ring_lane_builds_at_the_certified_scalar`
