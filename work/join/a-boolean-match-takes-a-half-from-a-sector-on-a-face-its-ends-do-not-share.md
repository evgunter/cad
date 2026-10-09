---
id: a-boolean-match-takes-a-half-from-a-sector-on-a-face-its-ends-do-not-share
kind: issue
title: A boolean match takes a germ half from a sector on another face than the one its two ends share, and the join refuses on the wrong face
status: closed
opened: 2026-10-06
closed: 2026-10-08
priority: P2
cost: M
refs: [which-fragment-of-a-divided-face-holds-a-segment-is-spelled-three-ways]
---

## What

Found by PR 4131's review (CLEAVE, `cleave/fragment-lineage`), on the
reviewer's probe over topo + sweep (the probe patch rides in the
review's scratch, `review-probes.patch`).

`boolean::join`'s `bool_connect` joins a matched segment between the
two germs' halves (`open[..].a[..].0.he`, the `HalfGerm::he` each germ
was minted with in `boolean/insert.rs` and `boolean/vtxfac.rs`).
`JoinPlan::of` (`chord_join.rs`) plans on the first half's face. The
plan saw the two halves on two faces 82 times (68 of them
Plane/Plane):

- **22 of the 82**: the segment's two end vertices DO share a face, but
  one half was taken from a sector on another face. The segment does
  not straddle two faces; the half is the wrong one.
- **14 of those 22**: the refusal names a Cylinder face while the
  face the ends share is a Plane.

Every such join is refused today, by whatever runs next on the first
half's face: the curve's lane (`chord_spec`, `SectionInvariant` with
the tangent-frontier words or "endpoints coincide along the ruling")
or the join's `mekr` across the two faces (`Euler(NotSameFace)`). The
declared-REST door rescues some unions; elsewhere the refusal stands,
naming a face the segment does not lie in.

Poses (sweep tests):
`join2_r1_probes::unions_through_ring_vertices_and_like_far_ends_build_sound`,
`join2_r2_probes::a_like_far_ends_tie_is_decided_by_the_partner_faces_trim`,
`reach_continuation::a_tangency_in_the_middle_of_an_edge_builds_in_either_operand_order`
(the 22 come from these three); the two-face plans also reach
`join2_r2_probes::a_channel_whose_arm_ends_join_two_ring_vertices_builds_sound`,
`…an_island_of_ring_vertex_segments_refuses_at_the_zip_frontier` and
`rest_nested_strut::a_pinch_apex_meeting_one_vertex_refuses_as_the_frontier_in_either_order`.

## The shape to give

Take a germ's half from the sector on the face its segment lies in: where
the two ends share a face, the halves the join reads are that face's.
Then decide what is left (halves on two faces whose ends share none)
at the plan, typed with its own words, and say which of today's
refusals were this.

## Closed (2026-10-08, INTENT stage 4 A, `intent/s4-a-join`)

The two-face plans were germs only tangent to a bound of their sector (a straight edge passing a fillet's tangent point), minted in that sector while `sectors::germ_loci` reads them in the face across the bound, the face the segment lies in. `boolean::insert::across_tangent` now mints such a germ in the sector across the bound, keeping its crossing codes, and the run logic decides strut or fan from there. Measured with a probe in `JoinPlan::of` over the topo and sweep suites (5111 rows, main `b7eec503` merged): **no plan has its halves on two faces**, where this row counted 82. The poses named above all build in the join (`join2_r1_probes`, `join2_r2_probes`, `reach_continuation`, `rest_nested_strut`), so nothing is left for a typed two-face refusal to name.

