---
id: a-cone-cell-presented-by-its-apex-side-is-signed-by-no-separator
kind: issue
title: A cone face's cell presented by its apex's collapsed side is signed by no separator, so a plane face sharing a ruling to the apex refuses Departure at the apex in the plain frame
status: open
opened: 2026-11-21
priority: P0
cost: M
refs: [7115]
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
cell on the apex's collapsed side, half a turn from the shared ruling
in `u` (measured below; not beside it), against the plane's cell at the
apex beside the ruling. In `review_m2_pr6_walk_shapes::survives_near_full_wedge_pole_junction_unwrap`:
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

**Where in `sign_on` it fails** (measured 2026-11-21, below): neither
candidate the trace named. Under the plane through the shared ruling
the apex's side is zero by membership; `departs` straddles because the
cone's cell holds the second ruling that plane's carrier cuts from the
cone through the apex, half a turn from the shared one.

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

A separator that signs a cone's cell whose zero is the apex's collapsed
side where the plane through the shared ruling cannot: that plane holds
a second ruling of the cone through the apex, and where the cone face
holds it (a wedge past a half turn, a box face cutting the cone) the
cone's cells about it lie on both sides of every plane through the
shared ruling. Landed: the plane through the apex alone, each face
presented against the apex alone (below).

## Measured and landed (2026-11-21, lane `embed/apex-separator`)

**Where the cone's reading returns `None`: neither candidate above.**
Probed with the traced cell's inputs (`CV_SIGN` prints in `sign_on` and
`present_chart` on the measuring branch `embed/apex-separator-measure`,
main `0c1eb97ad` with the gate and the trace lane's prints). Under the
plane through the shared ruling the apex's side IS zero by membership
(`zero_on` true: the plane lists the apex, `on.whole && a.whole`). What
fails is `departs`, and not by an enclosure's width: the cone's
rectangle holds a second zero of the plane.
- `review_m2_pr6_walk_shapes::survives_near_full_wedge_pole_junction_unwrap`
  (θ = 1.5π + 0.01): the cone face's extent is `u ∈ [−π, 1.5808]`, its
  rulings at `−π` (`2v1`) and `1.5808` (`5v1`, shared). The cell
  `u ∈ (4.7185, 4.7247)` is `u ≈ −1.5617` on the face's sheet, mid-face,
  no line clipping it. It holds `u₀ + π` (`4.7224`) for the shared
  ruling's `u₀ = 1.5808`: the wedge plane holds the axis, so its carrier
  meets the cone in the shared ruling and the ruling half a turn away,
  and past a half turn that ruling is on the cone face. `n·Φ_v = n·r(u)`
  over the cell is `[−2.6e-6, 3.4e-6]`, a true sign change.
- `a_cone_apex_on_an_edge::an_edge_ending_just_off_the_apex_refuses_box_first_and_builds_cone_first`:
  extent `[−π, π/2]`, shared ruling at `π/2`, the cell's `XHi` at
  `3π/2 ≡ −π/2`, the ruling half a turn away.
- `a_cone_apex_meets_a_vertex::a_box_corner_at_the_apex_holds_the_cones_quadrant`:
  shared ruling at `−2.7416`, the cell `u ∈ (0.3988, 0.4050)` holds
  `0.4000`, half a turn away; here the plane is the box face, cutting the
  cone in two rulings through the apex.

So no plane through the shared ruling signs these cells, at any depth:
in its carrier the plane face's sector and the cone's second ruling lie
on the same side of the shared line, and every plane through that line
sees both on one side. The row's "What is needed" assumed the
contrary; corrected there.

**What landed** (`crates/topo/src/injective.rs`): `departure` tries one
more separator after the graph's, where the pair is near shared edges
and exactly one shared vertex (`at_end_vertex`): the plane through that
vertex alone (`planes_through`'s vertex arm), each face presented
against the vertex alone. The plane face is then its corner
(`Zero::Point`), the cone its apex side (`Zero::Side`, `Along::at`), and
the existing readings sign them: `departs` across the apex side is
`n·Φ_v = n·r(u)`, the rate along each ruling. Soundness: for each `u`,
`ψ∘Φ(u, ·)` is `v·n·r(u)` plus the apex's offset from the vertex (the
apex reading's ε concession, `SELF_INTERSECTION_APEX`), linear in `v`,
zero at `v = 0`; a one-signed `n·r(u)` over the rectangle signs it on
the rectangle off the side. That is the side argument per ruling; it
never reads `Φ_u`, which is what vanishes there. It needs the rectangle
on one nappe and the apex side among its clips, which `present_chart`
gives (the nappe clip at `v = 0`); it needs nothing of the shared
ruling: a cell on it reads one sign on both faces along the ruling, so
it does not certify, and the zero sets are the vertex, on the shared
cell, so a pair that meets off it cannot be signed opposite.

**The 39 census tests, with the gate asking** (the census filter, run
on the measuring branch before and after the diff, `f64`): before, 0
pass; after, **19 pass**. The 20 that still refuse, by the last cell
the trace prints (`CV_TRACE`), none at a cone cell on the apex side
against a cell beside the shared ruling:
- the cone's cell beside the shared ruling short of the apex
  (`v ≈ −0.01` to `−0.018`), the plane by the ruling's side,
  `planes = [(None, Some(Negative))]`: `r1_other_half_angles`,
  `r1_partial_windows`, `r1_a_wedge_over_a_half_turn_splits_through_its_apex_without_panicking`,
  `a_plane_through_a_30_degree_cones_apex_cuts_its_closed_forms`;
- the plane by a `Corner` of two shared rulings against a cone cell with
  no zero short of the apex, `planes = []`:
  `a_plane_through_a_45_degree_cones_apex_cuts_its_closed_forms`,
  `r1_a_cone_on_a_cylinder`, `r1_apex_down`;
- a cone cell with no zero against the ruling's side, the plane signed
  only: `an_edge_along_the_axis_holds_a_quarter_of_the_cones_top`
  (`v ≈ −0.52`), `a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge`;
- a part `Local` away from the apex (the cover or a corner):
  `an_edge_ending_just_off_the_apex_refuses_box_first_and_builds_cone_first`
  (`sides_are_the_faces`, the ruling's range short of the cell below a
  corner at `6v1`), `an_edge_across_the_apex_holds_half_the_cones_top`,
  `an_80_degree_cones_top_against_a_box_clear_of_its_rim_builds`,
  `a_small_box_whose_side_holds_the_wrap_seam_builds_every_op`,
  `an_edge_through_the_apex_builds_box_first`,
  `a_cap_moved_off_a_needle_apex_refuses_the_hyperbola`;
- a part `Local` at the apex: `r3_seeded_random_apex_poses_never_panic_or_mislead`,
  `r2_partial_cones` (now plane/plane, `3v1`/`6v3` at `5v1`),
  `r2_partial_over_a_half_turn_never_panics` (now `Budget(Pairs)`; the
  test itself passes);
- a cone part `Structural` against a plane read by its curve, at a
  shared conic edge with no vertex: the three below.

**The four assigned unchecked.** `split_rulings::a_ruling_split_a_hair_from_its_end_folds_a_triangle`
belongs here: it holds after. The other three do not: their payloads
are the same before and after, and their last cells are a cone part
`Structural` at a shared conic edge with no vertex near
(`reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause`:
cone `1v1` `Structural`, the plane by its curve, edge `9v1`;
`a_ring_on_a_cone_face::a_box_edge_through_a_cone_wall_crosses_it_at_the_rings_corners`
and `split_rulings::a_run_on_a_cone_face_cut_by_an_oblique_plane_meshes`
the same shape), which is
`curved-presentation-reads-plane-and-cylinder-images-only`; added
there.

The residue is filed as
`the-apex-census-rows-that-still-refuse-read-cells-short-of-or-off-the-apex`.
