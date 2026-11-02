---
id: a-self-touch-point-its-tangent-cones-leave-to-second-order-passes-unread
kind: issue
title: A self-touch point whose curved faces' tangent cones leave it to second order passes unread
status: open
opened: 2026-11-02
priority: P1
cost: H
refs: [tier-3-prime-reads-no-side-where-a-solid-touches-itself-at-a-point-on-a-curved-face]
---


Found by `tier-3-prime-reads-no-side-where-a-solid-touches-itself-at-a-point-on-a-curved-face`'s
lane (branch `restread/self-touch-curved-sector`).

## What

**A named silent pass.** Where a solid touches itself at a point with
a curved face or edge in its link, `sweep_self_touch_points`
(`crates/topo/src/census.rs`) reads the faces' tangent cones
(`self_touch_verdict`, `Link::tangent_at_point`, `FaceSector::tangent`)
and `rest::one_material` decides the point wherever the cones' first
order does (`rest::first_order`). Where it does not, the verdict is
`SelfTouch::SecondOrder` and the point is skipped, as every such point
was before. The classes:

- **a cone's apex** in the link (`corner_normal`: `VertexFace::Apex`
  or `NearAxis`);
- **a cusp corner**: a face whose two edges leave the vertex along one
  tangent (`corner_fan`), a line tangent to an arc at its end, or two
  arcs tangent there;
- **a first-order tangency**: a bound lying on another sheet's plane
  or tangent plane where the faces keep no such coincidence, or two
  whole tangent planes on one plane (`rest::first_order`), and two
  tangent sectors read lying on each other (`Degenerate`);
- a spline face held off a corner, a curved edge with no certified
  curve, a face whose foot normal does not read.

## Measured (merge base `c7759b036c`, topo + sweep, slow set, default ε)

Before the tangent reading the skip took 107 (test, point) readings in
37 tests over 17 test files. The tangent reading reads 33 of them (all
`Rest`, no `Crossing`), refuses 1 in band
(`m5_pr9_boss_union::the_curved_inventory_is_admitted_and_the_bogus_record_is_stale`,
which stays green), and leaves 73 to second order: 69 `Unreadable`,
4 `Degenerate`.

Refusing those 73 (`BoundaryCrossesItselfUndecided`) turns 13 tests
red, every one in `crates/sweep/tests/`:

- `a_closed_conic_in_the_partners_face::a_rim_flush_in_its_partners_top_tangent_inside_builds_or_meets_a_pinned_limit`
  (two cylinders tangent inside);
- `a_cone_apex_on_a_plane_through_it::a_plane_meeting_a_tipped_cone_at_its_apex_alone_keeps_it_whole`
  (a cone's apex);
- `a_split_through_a_wrap_edges_vertex::` ×5 —
  `a_boolean_tangent_to_a_holes_rim_at_a_vertex_builds_sound`,
  `a_boolean_tangent_to_a_two_arc_holes_rim_builds_sound`,
  `a_plane_tangent_to_a_holes_rim_at_a_vertex_splits_sound`,
  `a_plane_tangent_to_a_two_arc_holes_rim_splits_sound`,
  `a_split_through_a_hole_walls_strut_vertex_keeps_one_face_where_the_piece_goes_round`
  (cusp corners, a line on an arc's tangent);
- `bisector_nodes::a_rod_against_a_bore_builds_every_operation`,
  `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself`
  (equal cylinders' nodes, all four tangent planes one: `Degenerate`);
- `split_across_a_revolve_seam::a_plane_touching_a_bore_rim_splits_alike_under_either_normal`,
  `split_across_a_revolve_seam::a_plane_touching_a_bore_rim_at_its_seam_splits_alike_under_either_normal`
  (cusp corners);
- `two_wedge_ends_on_one_line::a_brick_floating_in_a_lune_beside_unread_wedge_ends_refuses_unexamined`,
  `two_wedge_ends_on_one_line::two_lune_cavities_kissing_at_their_cusps_refuse_their_wedge_ends`
  (lune cusps).

Five more tests reach the residue and stay green refused, their
assertions taking a refusal: `review_6020_seam_probes::` ×2 (cusps and
an arc's tangent on a planar face, 252 readings over 33 points),
`review_5956_r2_probes::r2_pit_secants` (a cone's apex, 35),
`reach_wall_chord_rows::a_vertex_on_a_curved_face_is_confirmed_by_its_trim`
(a foot normal), and `census::tests::a_vertex_on_edge_record_off_a_circle_edge_reads_stale`
(a resolve refusal).

## The fix

A reading of second order, as the cross-solid curved lanes take one:
at a cusp, which side of the shared tangent each edge bends to
(`geom_brep::conic_bend_side`); at a tangency, the sheets' relative
curvature along the shared plane; at a cone's apex, the cone of
rulings itself (`sector_face::ConeApex`). Each must refuse where it
does not decide. Whether the residue refuses
(`BoundaryCrossesItselfUndecided`) in the meantime, and the 13 rows go
red with it, is a decision for the orchestrator with Ev.
