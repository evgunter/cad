---
id: a-cone-cell-presented-by-its-apex-side-is-signed-by-no-separator
kind: issue
title: A cone face's cell presented by its apex's collapsed side is signed by no separator, so a plane face sharing a ruling to the apex refuses Departure at the apex in the plain frame
status: dispatched
opened: 2026-11-21
priority: P0
cost: M
refs: [7115]
pr: 7315
branch: embed/apex-separator
---


## Finding

From EMBED's per-solid census trace (lane `embed/trace-cone-torus`,
2026-11-21), main `c2a0c41f40` with the gate (`embed/gate-measure`)
and its trace switches, plus diagnostic prints of the separators
(`embed/trace-cone-torus-measure`).

A plane face and a cone face share a ruling that ends at the apex: a
partial revolve's wedge plane, or a box face through the apex. The pair
splits to `Depth { max_cell_depth: 40 }` and refuses
`Uncertified { Departure }` across the ruling.

**The cell.** In every traced run the last cell pair is the cone's
cell on the apex's collapsed side, beside or on the ruling, against the
plane's cell at the apex. In `review_m2_pr6_walk_shapes::survives_near_full_wedge_pole_junction_unwrap`:
cone `3v1` at `u ∈ (4.7185, 4.7247)`, `v ∈ (−1.4e-3, 0)`, plane
`1v1` at `u ∈ (0, 9.8e-4)`, `v ∈ (−1, −0.999)`, shared ruling `5v1`,
apex vertex `2v1` in the cell.

**Both faces are presented**: the plane by the ruling's side
(`Zero::Side(XLo)`, the affine corner map), the cone by the apex's
collapsed side (`Zero::Side(YHi)` at `v = 0`).

**No separator signs the cone.** `departure`'s three separators, each
read on both presentations:
- the plane's carrier on the cone, and the cone's carrier on the plane:
  `None` both;
- `planes_through` (one plane): `(Some(Positive), None)`, the plane
  signed and the cone not;
- `graph_of`: `(Some(Positive), None)`, the same.

The same in `a_cone_apex_on_an_edge::an_edge_ending_just_off_the_apex_refuses_box_first_and_builds_cone_first`
(`(Some(Positive), None)`) and, the faces swapped,
`a_cone_apex_meets_a_vertex::a_box_corner_at_the_apex_holds_the_cones_quadrant`
(`(None, Some(Negative))`). So the departure arm falls through to
`split` at every depth.

**Where in `sign_on` it fails is unchecked.** One candidate, read off
the code and not probed: under a plane (`on` some), a `Zero::Side`
counts as zero only where `on.holds(a, false)` and `a.on_edges` or
`on.whole && a.whole`, and the apex's side is `Along::at(apex)`, a point
along no edge. The carrier arm (`on` none) reads the side by value, so
that is not the whole story; its `departs` rate over a cone cell at the
apex is the other place to look.

**Not PR 7115.** Every reproducer refuses the same pair on PR 7115's
topo diff applied to the harness (`embed/pole` at `164f92cb0`): the
apex is placed and the extent reaches it already, so this is neither
the extent nor the cover of the apex cell's side. It is the sibling of
B in `two-half-cones-sharing-both-rulings-refuse-departure-at-their-apex-in-a-turned-frame`
(on PR 7115's branch): there two half-cones, a turned frame, and
`sign_on` reading one face only. Here it is a plane against a cone, in
the plain frame.

## Count

39 tests: departure at an edge, plane/cone 26 and cone/plane 13, all
f64. 35 are partial-revolve wedges or apex fixtures (`germ_cone_apex_closure`,
`review_5956_*`, `a_cone_apex_*`, `a_plane_*_apex`, `review_m2_pr6_walk_shapes`,
`cone_operand_rows`, `germ_cone_doors`). Four were traced to the cell
above. Four do not name the apex and are assigned unchecked:
`a_ring_on_a_cone_face::a_box_edge_through_a_cone_wall_crosses_it_at_the_rings_corners`,
`reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause`
and the two `split_rulings` rows.

The bodies are valid: a wedge of a revolved cone, and a box meeting a
cone at its apex, which the tests build and check against closed forms.

**Cheapest reproducer:** `mesh::all review_m2_pr6_walk_shapes::survives_near_full_wedge_pole_junction_unwrap`
(0.09 s).

## What is needed

`sign_on`, or a separator built for it, signs a cone's cell whose zero
is the apex's collapsed side: there the image's `v`-rate is the ruling
direction, which a plane through the shared ruling departs at first
order wherever the cell is off the ruling (and at the ruling's own
corner as the two-sided corner reading does).
