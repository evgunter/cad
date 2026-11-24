---
id: a-cone-cell-presented-by-its-apex-side-is-signed-by-no-separator
kind: issue
title: A cone face's cell presented by its apex's collapsed side is signed by no separator, so a plane face sharing a ruling to the apex refuses Departure at the apex in the plain frame
status: closed
opened: 2026-11-21
priority: P0
cost: M
refs: [7115]
pr: 7315
branch: embed/apex-separator
closed: 2026-11-23
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

**What landed** (`crates/topo/src/injective.rs`). `departure` tries one
more separator after the graph's, where the pair is near shared edges
and exactly one shared vertex `v` (`one_shared_vertex`, also the reader
of a vertex the pair shares alone): the plane through `v` alone
(`planes_through`'s vertex arm), each face presented against `v`
alone. Against `v` a plane face is its corner (`Zero::Point`), a cone
its apex side and a sphere its pole side (`Zero::Side`, `Along::at`),
and the existing readings sign them.

**Why it is sound** (the argument the code relies on, stated at the
arm). Each presentation against `{v}` is a superset of the face's part
of its cell, and `apart` accepts only where the plane is strictly of
one sign on each off its zero set, the signs opposite. Against `{v}`
both zero sets lie at `v` (a zero carrying an edge reads nothing under
a plane listing `v` alone). So a common point other than `v` would
carry both signs, and none exists. A shared ruling's points in both
cells, or a second ruling the plane face holds, are such common
points, so `apart` never accepts there: tangency, a second meeting and
a vertex that is no apex are refused by the same reading, not by
premises of their own. **Why a collapsed side is read at all:**
`sign_on` reads any side by the mean value theorem along the axis
across it, at each value of the other; across a collapsed side the
chart's derivative is a ruling's or a meridian's direction, and the
one that vanishes is never read. The side's image is `v` within the
collapse's own reading: `SELF_INTERSECTION_APEX` at an apex,
`vertex_at_singular_point` (`chart_singularity_at_vertex`) at a pole,
both existing concessions, now listed among the module's separator
concessions.

**Where it fires.** The reviewer's firing census over 8,105 topo and
sweep tests: 3 (the wedge row and the unit positive, plane/cone at the
apex; the snowman, sphere/sphere at a pole). On the tour only
sphere/sphere at the dice's pips' poles (`8v3`/`39v3`, `10v3`/`40v3`,
`12v3`/`41v3`, `14v3`/`42v3`, `36v3`/`75v1`, `37v3`/`73v1`,
`38v3`/`71v1`); no cone fires there. In this row's census through the
gate, the plane/cone wedges and box faces at the apex.

**The census tests, with the gate asking** (measuring branch, main
`7669fcfdd` with the gate, f64; one of the 39 has left main, 38
remain): main's `injective.rs` 0 pass, this change's **18**. Named,
with the 21 that do not hold, in
`the-apex-census-rows-that-still-refuse-read-cells-short-of-or-off-the-apex`.

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

**The turned frame.** On main `0cae9969f` a wedge turned about a
generic axis refused: its last pair was the plane's cell at the apex
against two cone cells beside it, both spanning the face's whole `u`
(`u ∈ (π, 7.864)` on its sheet). One stops short of the apex
(`Err(Local)` against `v`); its sibling holds the apex, is presented by
its apex side, and the plane through `v` reads the cone's rate across
it `[−2.9e-4, 1.7e-4]`, signing the plane and not the cone. The cone
face wraps 270°, so no plane through the apex has every ruling of it on
one side; the cell was never halved in `u` (the case A of
`two-half-cones-sharing-both-rulings-refuse-departure-at-their-apex-in-a-turned-frame`).
After merging main `7669fcfdd` (PR 7120, a face's window from its
edges' covers) the turned wedge holds at both scalars, θ = 1.5π + 0.01
and 5.5. Traced on the merged head (θ = 1.5π + 0.01, f64, 479
discharged, prints only): the plane through the apex certifies 6 of the
72 cell pairs it is asked, plane `1v1` or `2v1` against cone `3v1` at
the apex `2v1`, and every cone cell it certifies holds the apex, is
presented by its apex side, and is a quarter or an eighth of a turn
wide in `u` (`(4.712, 5.498)`, `(4.320, 4.712)`, `(0, 0.785)`,
`(5.498, 6.283)`), its `v` `(−0.123, 0.096)` clamped to `(−0.123, 0)`
by the nappe. Before, the apex-holding cell spanned the face's whole
`u`; now the cone's cells reaching the apex are split in `u` from
coarse depths on, each narrow enough that the plane through the apex
has its rulings on one side. **Not bisected:** which line of PR 7120
changes the cone's root window (its description puts it in
`boundary_of`'s cover boxes). Pinned holding in
`self_intersection_proves_injectivity::a_cone_wedge_past_a_half_turn_is_embedded_at_its_apex`
with seven other poses. The same wedge revolved by `−θ` refuses off the
plain frame:
`a-mirrored-cone-wedge-refuses-departure-at-its-apex-off-the-plain-frame`.

**Elsewhere.** The snowman (`a_two_ball_union_holds_with_its_poles_read`)
holds at `Interval` (579 pairs). Unchanged: the vertex-alone pairs of
`the-vertex-alone-plane-leans-…` (11 run, payloads identical; the arm
needs a shared edge near), the pole lane's 90 delta cases on PR 7115's
head `ee1958ede` (verdict classes identical, case B not closed), and a
plane through a meridian to a sphere's pole (held before, identical
receipts). The tour walk on main `7669fcfdd` holds 78 of 85 stops, this
change 81: `diepips`, `diecomposed` and `diechamfer` move from
`Departure` on the pip pair `8v3`/`39v3` to `Embedded`, every other stop
identical.

The residue is filed as
`the-apex-census-rows-that-still-refuse-read-cells-short-of-or-off-the-apex`.
