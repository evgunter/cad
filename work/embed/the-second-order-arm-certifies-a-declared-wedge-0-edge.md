---
id: the-second-order-arm-certifies-a-declared-wedge-0-edge
kind: issue
title: self-intersection's second-order arm is not built: a declared wedge-0/2π edge refuses Uncertified{SecondOrder} past CUSP_DEPTH
status: open
opened: 2026-10-28
priority: P2
cost: H
---


## Finding

The *Self-intersection scope* clause (`docs/ERROR-DESIGN.md`, E7)
lists three local certificates for a touching pair. SELFX built two:
departure and monotone (`crates/topo/src/injective.rs`). The third,
**second order at a declared wedge-0/2π edge**, is not built.
`docs/SELFX-SPEC.md` item 2 allows that order: land the first two,
have the cusp seam refuse typed, and file the arm as its own row. This
is that row.

## Where it stands

- `cusp` (`crates/topo/src/injective.rs:1863`) reads a wedge-0/2π
  shared edge: the two faces' outward normals are opposed at the
  middle of the side each presents on it, decided `Zero` at
  `self_intersection_cusp`. `departure` then returns `Outcome::Cusp`
  (`:1688`).
- The sweep (`crates/topo/src/clearance.rs:1092`) splits such a pair
  like any uncertified one until `CUSP_DEPTH` (`:151`, 10). Past that
  depth it refuses `EngineRefusal::Uncertified { arm: SecondOrder }`,
  naming the faces and the shared edge. The split depth keeps the
  cells away from the edge decided, and a violation there reported
  first.
- Pinned by
  `crates/sweep/tests/self_intersection_proves_injectivity.rs:363`
  (`a_declared_wedge_0_kiss_refuses_naming_the_second_order_arm`): the
  kiss refuses with `SecondOrder` on the kiss edge. It costs about 340
  cell pairs, and about 38 s unoptimized.

## What the arm owes

At a wedge-0 edge the two faces leave the edge on the same side, so no
first-order separator exists. The arm decides the sign of the faces'
separation at second order: the difference of their normal curvatures
across the edge, along the edge, under one presentation of both. When
the arm lands, the kiss row flips from a refusal to `Holds`.

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.

## Census: 35 cylinder/cylinder tests at the at-rest gate, and 4 at a vertex (EMBED, lane embed/trace-cylinder, 2026-11-21)

Traced from the per-solid census (`measure/gate/census-c2a0c41f40.json`
on `embed/gate-measure`, bucket "second order, edge, cylinder/cylinder"),
on `embed/trace-cylinder-measure` with `CV_TRACE=1`. All 35 refuse at
the sweep's `Outcome::Cusp` past `CUSP_DEPTH` (`CVR cusp depth=10`),
both faces presented `Of(Side)` on the shared edge: this row's ground
exactly, on valid bodies built legal at rest on purpose
(`a_knife_edge_fillet_builds_where_its_ball_is_defined` 9,
`m10_2_measure` 4, `a_swept_cusp_is_legal_at_rest` 4,
`wedge_end_doors` 3, `topo r2_probes` 2, `blend_recourse_followability`
2, `two_wedge_ends_on_one_line` 2, 9 singletons). Cheapest:
`editor-core::all m10_2_measure::a_split_half_of_a_cusp_extrude_gathers`
(0.10 s). With the at-rest gate these are 35 finished bodies refused at
their door, which is P0's description; the priority above predates the
gate.

Four tests of the cylinder/cylinder vertex bucket look like the same
ground at a vertex the pair shares alone, read by departure, not by the
cusp test: `review_4831_probes` p1 (a bore inside a tall disc touching
its wall), p8, p10, and `a_closed_conic_in_the_partners_face::a_kept_rim_leaving_along_the_dropped_faces_boundary_c…`.
Both faces are presented `Of(Point)`; the one plane `planes_through`
builds at the vertex signs one face and gives `None` on the other at
depth 40. The fixtures are tangent poses, so the two walls share a
tangent plane at the vertex; that they do is read off the fixtures,
unchecked, and so is whether `the-vertex-alone-plane-leans-along-a-cells-long-metric-side-so-an-anisotropic-pair-splits`
is the nearer cause for any of them.

## Instances in the per-solid census (EMBED trace lane, 2026-11-21)

**19 tests refuse `Uncertified { SecondOrder }` at an edge** in the
census's small buckets (main `c2a0c41f40`, the at-rest gate): cusp
bodies the suite holds legal at rest (`a_swept_cusp_is_legal_at_rest`,
`a_knife_edge_fillet_builds_where_its_ball_is_defined`,
`a_minted_vertex_on_a_curved_face`, `an_arc_tangent_at_a_boolean_vertex`,
`a_box_along_a_fillets_line_arc_junction`, `pi_seam_and_kiss_through_the_boolean`,
`m10_2_measure`'s cusp documents), over plane/cylinder, cylinder/plane,
sphere/cone, cone/sphere and NURBS pairs. One traced:
`a_knife_edge_fillet_builds_where_its_ball_is_defined::a_crescent_whose_arc_is_shorter_than_its_foot_refuses`
refuses `SecondOrder` on its cusp edge `6v1` in 0.03 s, the fixture's
extrude finished body refused at the gate. The rest are assigned by the
refusal's arm, unchecked. Two more of that arm are not cusps and are
not counted here: `review_r1_tier_gate_probes::r1_a_wedge_in_band_only_at_its_apex_edges_length_ships_as_a_corner`
(in band by its own construction) and
`spheres_crossing_off_every_edge::a_ball_on_the_centre_line_builds_at_every_radius_and_depth`
(untraced).

## Tangent ruling contacts between two cylinders in the census (EMBED lane `embed/iso-end`, 2026-11-22)

In these 4 the partner cylinder is presented and the carrier arm is tried. The partner's rate across its side straddles zero (`departs=None`) because the two cylinders are tangent along the shared ruling, a second-order contact no first-order arm reads.

Measured on `embed/iso-end-measure2` (`origin/embed/gate-measure` @ `ebb2e6f8e` plus `embed/iso-end`'s diff), where each refusal is the same pair, test by test, as on the trace run that named the exits (`embed/iso-end-measure` @ `030c75763`, `CV_TRACE`).

- `editor-core::all emit_curved_rim_chord::a_post_across_the_piece_of_a_notched_circle_through_0_unites`
- `sweep::all parallel_cylinder_join::rods_across_the_wall_join_along_their_rulings`
- `sweep::all parallel_cylinder_join::the_walls_join_along_their_carried_rulings`
- `sweep::all a_ruling_through_a_box_corner_edge::a_wall_through_a_box_edge_builds_every_op_but_the_ring`
