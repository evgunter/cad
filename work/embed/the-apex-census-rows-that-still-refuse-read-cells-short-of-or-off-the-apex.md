---
id: the-apex-census-rows-that-still-refuse-read-cells-short-of-or-off-the-apex
kind: issue
title: Twenty of the apex census rows still refuse Departure once the cone's apex side is read, at cells short of the apex or off it: a cone cell beside a ruling short of the apex the ruling's plane does not sign, a two-ruling corner with no plane, and cone parts Local away from the apex
status: open
opened: 2026-11-21
priority: P1
cost: M
---


## Finding

Filed by lane `embed/apex-separator` (2026-11-21). With the cone's cell
on its apex side signed by the plane through the apex
(`a-cone-cell-presented-by-its-apex-side-is-signed-by-no-separator`),
19 of that row's 39 census tests pass with the gate asking (measuring
branch `embed/apex-separator-measure`: main `0c1eb97ad`, the gate and
the trace lane's prints, f64). The other 20 refuse
`Uncertified { Departure }` at a shared edge (one is now
`Budget(Pairs)`), at a last cell pair (`CV_TRACE`, depth 40) of one of
these shapes. Counts are tests; one traced cell each. Bodies valid.

**A. The cone's cell beside the shared ruling, short of the apex** (4).
The cone by the ruling's side (`v ≈ −0.01` to `−0.018`, `u` about 0.05
to 0.11 wide), the plane by the ruling's side near the apex;
`planes_through`'s plane through the ruling and the graph sign the plane
only (`(None, Some(Negative))`):
- `sweep::all review_5956_r1_probes::r1_other_half_angles`: cone `6v1`
  `u ∈ (3.43612, 3.53429)`, `v ∈ (−0.016114, −0.015866)`, `Side(XHi)`;
- `r1_partial_windows`, `r1_a_wedge_over_a_half_turn_splits_through_its_apex_without_panicking`;
- `a_plane_through_a_cones_apex::a_plane_through_a_30_degree_cones_apex_cuts_its_closed_forms`:
  cone `4v3` `u ∈ (2.84159, 2.89616)`, `v ∈ (−0.018324, −0.018042)`.

Unchecked: whether the cone's rate across the ruling straddles because
the cell is wide in `u` (the cell short of the apex not halved in `u`,
as in case A of the pole lane's
`two-half-cones-sharing-both-rulings-refuse-departure-at-their-apex-in-a-turned-frame`,
PR 7115) or because the bisecting plane is near the cone's tangent.

**B. A plane cornered by two shared rulings against a cone cell with no
zero, short of the apex, and no plane** (3): `planes = []`.
- `a_plane_through_a_cones_apex::a_plane_through_a_45_degree_cones_apex_cuts_its_closed_forms`:
  cone `4v3` `u ∈ (2.89616, 2.90843)`, `v ∈ (−0.0027621, −0.0013811)`,
  `Zero::None`; plane `Zero::Corner`;
- `review_5956_r1_probes::r1_a_cone_on_a_cylinder`, `r1_apex_down`.

The two-line arm needs the shared vertex in the cell; the cone's cell
does not hold it. The pole lane's case C is this shape on two half-cones.

**C. A cone cell with no zero against the ruling's side, the plane
signed only** (2): `a_cone_apex_on_an_edge::an_edge_along_the_axis_holds_a_quarter_of_the_cones_top`
(cone `u ∈ (3.13546, 3.14159)`, `v ≈ −0.5198`, the plane `2v3` by its
side), `a_plane_moved_to_a_cone_apex::a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge`.

**D. A part `Local` away from the apex** (6), at the cover or a corner:
- `a_cone_apex_on_an_edge::an_edge_ending_just_off_the_apex_refuses_box_first_and_builds_cone_first`:
  cone `5v3` `u ∈ (3.54043, 3.54350)`, `v ∈ (−0.63667, −0.63598)`, at
  the corner of ruling `4v5` (`XLo`) and circle `8v1` (`YHi`) at `6v1`:
  `sides_are_the_faces` finds the ruling's range `[−0.6364, 0]` short of
  the cell's `v` below the corner (`CVL chart`);
- `an_edge_across_the_apex_holds_half_the_cones_top`, `an_80_degree_cones_top_against_a_box_clear_of_its_rim_builds`,
  `a_cone_apex_meets_a_vertex::a_small_box_whose_side_holds_the_wrap_seam_builds_every_op`,
  `an_edge_through_the_apex_builds_box_first`,
  `a_plane_moved_to_a_cone_apex::a_cap_moved_off_a_needle_apex_refuses_the_hyperbola`.

Not traced past the exit. The collinear-cover lane's `sides_are_the_faces`
reading may hold the first.

**E. A part `Local` at the apex** (3):
`review_5956_r1_probes::r3_seeded_random_apex_poses_never_panic_or_mislead`
(cone `1v1` at `u ∈ (0, 8.8e-4)`, `v ∈ (−9.8e-4, 0)`, the ruling and
the apex in one cell), `review_5956_r2_probes::r2_partial_cones` (now
plane/plane `3v1`/`6v3` at `5v1`, both `Local`),
`r2_partial_over_a_half_turn_never_panics` (now `Budget(Pairs)`; the
test passes).

**F. A cone part `Structural` at a shared conic edge** (3): added to
`curved-presentation-reads-plane-and-cylinder-images-only`.

## What is needed

Each shape traced past its exit before a unit is cut: A and B against
the pole lane's cases A and C once PR 7115 lands, D against the
collinear-cover lane.
