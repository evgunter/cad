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
(`self_touch_verdict`, `Link::tangent_of`, `FaceSector::tangent`).
`rest::one_material` decides the point wherever the cones' first order
does (`rest::first_order`). Where it does not, the verdict is
`SelfTouch::SecondOrder`, and the point is skipped, as every such point
was before. The classes, as `SelfTouch::SecondOrder` lists them:

- **A cone's apex** in the link, or a vertex in the band of its axis
  (`corner_normal`: `VertexFace::Apex` or `NearAxis`).
- **A cusp corner**: a face whose two edges leave the vertex along one
  tangent (`corner_fan`) — a line tangent to an arc at its end, or two
  arcs tangent there. A spur's tip (the loop back along the half-edge it came by) is read as the
  whole turn, not a cusp.
- **A first-order tangency**: a bound lying on another sheet's plane or
  tangent plane where the faces keep no such coincidence, or two whole
  tangent planes on one plane (`rest::first_order`).
- **Sectors read lying on each other** (`Degenerate`): coinciding
  tangent sectors, or a slit or doubled sheet of planar faces in a link
  with a curved face.
- **A face the tangent reading cannot build**:
  - a spline face held off a corner;
  - a face whose foot normal (`foot_normal`), reach (`face_reach_from`)
    or corner face (`sector_face::resolve`) does not read;
  - a curved edge with no certified curve.

## Measured (head `edff118e41`, merge base `c3626bf54a`, topo + sweep, slow set, default ε)

The unit is a distinct (test, point, verdict) triple: the witness a
dumped reading names, as printed, in one test. Readings repeat across
a test's poses and orders: 688 readings collapse to 124 triples, over 80
distinct exact points in 37 tests and 17 test files. The tangent path's
own witness rows (`census::self_touch_curved`) are left out.

- **Read at first order: 34.**
  - 33 `Rest`, none `Crossing`.
  - 1 refused in band:
    `m5_pr9_boss_union::the_curved_inventory_is_admitted_and_the_bogus_record_is_stale`,
    which stays green.
- **Left to second order: 90.**

  | class | triples |
  |---|---|
  | cusp corner | 43 |
  | an arc's tangent lying in a planar face | 27 |
  | sectors on each other (`Degenerate`), all at `bisector_nodes`' equal-cylinder nodes | 10 |
  | a bound on another sector's bound | 4 |
  | a cone's apex | 2 |
  | an arc's tangent in a curved face's tangent plane | 2 |
  | a foot normal | 1 |
  | a `resolve` refusal | 1 |

The skip now lets each test run on past its first residue point, so
more points are reached than when the residue refused. The first
measurement refused it, at head `95102fc368`, and read 33 / 1 / 73.
The 8 distinct `Degenerate` exact points (4 when rounded to 1e-6) are
the reviewers' 20 and 8, counted under their dedupe keys.

Refusing the residue (`BoundaryCrossesItselfUndecided`) turned 13
tests red at that first head, every one in `crates/sweep/tests/`:

- `a_closed_conic_in_the_partners_face::a_rim_flush_in_its_partners_top_tangent_inside_builds_or_meets_a_pinned_limit`
  (two cylinders tangent inside);
- `a_cone_apex_on_a_plane_through_it::a_plane_meeting_a_tipped_cone_at_its_apex_alone_keeps_it_whole`
  (a cone's apex);
- `a_split_through_a_wrap_edges_vertex::` ×5 (cusp corners, a line on an
  arc's tangent):
  - `a_boolean_tangent_to_a_holes_rim_at_a_vertex_builds_sound`
  - `a_boolean_tangent_to_a_two_arc_holes_rim_builds_sound`
  - `a_plane_tangent_to_a_holes_rim_at_a_vertex_splits_sound`
  - `a_plane_tangent_to_a_two_arc_holes_rim_splits_sound`
  - `a_split_through_a_hole_walls_strut_vertex_keeps_one_face_where_the_piece_goes_round`
- `bisector_nodes::a_rod_against_a_bore_builds_every_operation` and
  `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself`
  (equal cylinders' nodes, where all four tangent planes are one:
  `Degenerate`);
- `split_across_a_revolve_seam::a_plane_touching_a_bore_rim_splits_alike_under_either_normal`
  and
  `split_across_a_revolve_seam::a_plane_touching_a_bore_rim_at_its_seam_splits_alike_under_either_normal`
  (cusp corners);
- `two_wedge_ends_on_one_line::a_brick_floating_in_a_lune_beside_unread_wedge_ends_refuses_unexamined`
  and
  `two_wedge_ends_on_one_line::two_lune_cavities_kissing_at_their_cusps_refuse_their_wedge_ends`
  (lune cusps).

Five more tests reach the residue and stay green when it is refused,
because their assertions accept a refusal:

- `review_6020_seam_probes::` ×2 (cusps, and an arc's tangent on a
  planar face);
- `review_5956_r2_probes::r2_pit_secants` (a cone's apex);
- `reach_wall_chord_rows::a_vertex_on_a_curved_face_is_confirmed_by_its_trim`
  (a foot normal);
- `census::tests::a_vertex_on_edge_record_off_a_circle_edge_reads_stale`
  (a `resolve` refusal).

## The fix

A reading of second order, as the cross-solid curved lanes take one:
at a cusp, which side of the shared tangent each edge bends to
(`geom_brep::conic_bend_side`); at a tangency, the sheets' relative
curvature along the shared plane; at a cone's apex, the cone of
rulings itself (`sector_face::ConeApex`). Each must refuse where it
does not decide. Whether the residue refuses
(`BoundaryCrossesItselfUndecided`) in the meantime, and the 13 rows go
red with it, is a decision for the orchestrator with Ev.

## Read one order down (branch `restread/self-touch-second-order`)

`link::second_order` reads two of the classes: a cusp corner, by
which side of its ray each edge bends to (a horn, or the whole turn
but a horn), with a ring of horns read as the thorn it bounds; and a
bound tied on another sheet's plane or bound, by whether the sheets
along that ray part one order down. The argument, with the band
charged, is in `crates/topo/README.md` beside the first-order one.

**Re-measured at merge base `dc4e9b279e`** (same instrument, topo +
the ten sweep families that reach the skip, default ε). Main moved
since the head above: the tangrim program's crescent rows now reach
the skip too. 122 (test, point, verdict) triples were left to second
order there; now 67 read `Rest`, none `Crossing`, and 55 are still
skipped, named and counted:

| class still skipped | triples |
|---|---|
| a slit (an inside plane and a coplanar face of the other sense, `review_6020_seam_poses`), stopped at an arc's tangent in that plane | 30 (27, and 3 of the cusps) |
| sectors on each other (`Degenerate`), `bisector_nodes`' equal cylinders | 14 |
| a lune: a horn beside two cylinder sectors coinciding at first order (`two_wedge_ends_on_one_line`) | 4 (cusps) |
| a curved sheet holding the tied ray inside it, whose line `parts` does not read: an arc's tangent in a curved face's tangent plane (`a_rim_flush_…`, 2) and a bound on a bound (`a_hole_sharing_the_rim_…`, 1) | 3 |
| a cone's apex | 2 |
| a foot normal | 1 |
| a `resolve` refusal | 1 |
| a thorn the cone round it reaches (its ray not decidedly outside the cone's material, or inside it for a needle), which may be a real overlap | 0 |

The last class reaches no sweep row; it is witnessed at body level,
where a thorn grafted round a box corner's ray stays skipped
(`census::self_touch_curved`). A sign one order down left in band is
`Unreadable` with the structural refusals: the skip's `why` does not
part them.

Of the cusps, 36 read and 7 are still skipped; of the bounds on a
bound, 35 read and 1 is still skipped (4 triples carried both tags).

**Refusing what is left** (`BoundaryCrossesItselfUndecided`, a local
variant never committed) turns 7 sweep rows red: 6 of the 13 above
(`a_rim_flush_…`, the cone apex, `bisector_nodes` ×2,
`two_wedge_ends_on_one_line` ×2) and
`a_closed_conic_in_the_partners_face::a_hole_sharing_the_rim_along_an_arc_then_leaving_it_tangent`,
added since. All five `a_split_through_a_wrap_edges_vertex` rows and
both `split_across_a_revolve_seam` rows read whole and stay green.
Whether the remainder refuses is still Ev's call.

## After PR 6422

PR 6422 (`restread/self-touch-second-order`) merged on 2026-11-03 with
the reading above, so this row is open again for the 55 triples still
skipped. The two biggest classes are the slits (30) and
`bisector_nodes`' equal cylinders (14). Whether the remainder refuses
in the meantime is still Ev's call.
