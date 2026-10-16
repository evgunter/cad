---
id: an-unrecorded-curved-edge-meeting-is-unseen-at-tier-three-prime
kind: issue
title: Tier 3′'s edge-edge pass reads lines only: an unrecorded meeting of a curved edge with another edge reads clean
status: closed
closed: 2026-10-15
branch: fuse/curve-pair-reader-c3
opened: 2026-10-14
priority: P3
cost: M
refs: [5027]
---

Found by FUSE's sweep for `a-corner-on-a-circles-interior-is-unseen-at-tier-three-prime`
(branch `fuse/census-tier3-corners`).

That unit gave tier 3′ a pass 2′, `census::sweep_vertex_curved_edge`,
which finds an unrecorded vertex on a curved edge's interior through
`census::curved::on_curved_interior`. Its sibling stands: pass 5
(`sweep_edge_edge`) pairs only the `Line` edges the snapshot keeps
(`census::snapshot`, `edge_is_line`). So an unrecorded crossing,
touch or overlap of a curved edge with another edge is no finding. An
op that lost such a record ships a body whose meeting no tier sees.

The confirm lane can read such a pair
(`census::curved::curved_interiors_meet`), so a pass 5′ over it is
the shape. It is held on `[ev]` PR 5027: the curve-pair reading it
would sweep with is what that PR decides. Related, and narrower:
`work/contacthold/two-coincident-curved-edges-touching-record-no-edge-edge-row.md`
(the writer side, for two coincident curved edges).

## Closed — pass 5′ sweeps the curved edges (fuse/curve-pair-reader-c3)

`census::sweep_curved_edge_edge` runs after pass 5. It reads each curved edge against the line edges and the later curved edges whose padded boxes meet its own, through `census::curved::interiors_meeting`: the confirm lane's reading of `geom_brep::curve_pair::meet`, keeping a meeting on both interiors and no point of the run about a meeting at an end (that meeting is a vertex event, passes 1, 2 and 2′'s). An unbacked meeting at a point is an `UnrecordedContact` `EdgeEdgeCross`, and an unbacked overlap along one curve an `EdgeEdgeOverlap`. Either is backed by the pair's edge-edge record or by the face rung `Declared::sides_face_backed`: the crossing rung is planar-first, so a crossing takes the grandfathered rung pass 2′ takes for its vertex, and the module docs list the new site under it. The differential suites see the pass as `CensusTrace::cee`.

A curved boundary crossing bounding an edge-on-face cell (`CutAt::CurvedCrossing`, which now names the boundary edge) is backed by that pair's edge-edge record, where nothing backed it before.

Rows:
- `census::curved_edge_edge::an_unrecorded_curved_edge_meeting_is_found_at_tier_three_prime`: a brick's vertical edge standing on the half-disc cap's arc is one unrecorded crossing; recorded, backed and confirmed; 1 mm out, nothing.
- `census::curved_edge_edge::two_arcs_meeting_inside_both_are_found_unrecorded`: two perpendicular half-discs whose arcs touch inside both.
- `census::curved_edge_edge::two_arcs_of_one_circle_overlapping_are_found_unrecorded`: a cylinder sheet's rim along the cap's arc is an `EdgeEdgeOverlap`.
- `census::curved::tests::a_tangent_touch_at_an_arcs_end_holds_no_interior_meeting_for_pass_five_prime`: an arc ending tangent on a line, and a slot's joint, owe no edge-edge record.

Re-baselined, each now seeing a meeting no lane read before: `census::tests::a_pass_holding_no_region_door_refuses_the_conformal_pair_typed` (the second sheet's rims cross the first's seam), `tier3_tests::line_conic_review::a_wedge_edge_touching_the_rim_transversally_never_validates_clean` (the wedge edge touches the rim inside both). `a_patch_record_backs_the_pair_and_confirms_through_both_doors` holds through the face rung.
