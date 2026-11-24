---
id: vacuous-reads-a-cell-at-a-corner-local-at-every-depth
kind: issue
title: injective's vacuous reads a cell that touches a face's corner from outside Local at every depth, because the touching line's edge is read without running on past the corner
status: open
opened: 2026-11-01
priority: P0
pr: 7322
branch: embed/collinear-cover
---


## Finding

POTDOC's review found this (PR #6172), and it predates that PR.

`crates/topo/src/injective.rs`, `vacuous`, reads a cell outside the face
as holding none of it only where a line the cell touches has an edge
covering the cell's whole side. It takes that edge's range from plain
`edge_ranges` and `covered`, with no run-on past a corner.

A cell beside a face's corner, outside the face, straddles the corner
along the touching line at every depth. For example, at the corner
`(2, 1)` of a rectangle `u ∈ [1, 2]`, `v ∈ [0, 1]`, take the cell
`u ∈ (2, 2 + h)`, `v ∈ (1 − h, 1 + h)`.

The `u = 2` edge stops at `v = 1` inside the cell's side, so the cell is
never covered. It is not read `Vacuous`, and `present_chart` returns
`Local` for it at every depth. A pair whose touching cells keep one such
cell splits to the depth limit and refuses, though the cell holds no
point of the face. That is an over-refusal.

`present_chart`'s clip sides now run on past a corner (`run_on`, read
by `sides_are_the_faces`). Whether `vacuous` wants the same reading or a
corner-aware one is this row's question: the cell there is past the
corner on both lines' non-face sides.

## Instances in the per-solid census (EMBED trace lane, 2026-11-21)

Traced on main `c2a0c41f40` with the at-rest gate
(`embed/gate-measure`) and prints of `vacuous`'s inputs and of
`sides_are_the_faces`' cover (`embed/trace-cone-torus-measure`). Two
shapes, both a torus cell lying outside its face across a line it
touches. Valid bodies, P0 at the gate's landing.

**Two collinear edges on one line, neither covering the cell's side
(13 tests, cylinder/torus).** A fillet band on a disc's or a bore's rim
splits the torus's foot parallel into arcs at the rim's joints.
In `blend_value_decided_rows::a_disc_rim_records_its_joints_and_its_supports`
the last pair is cylinder `3v1` against torus `9v1`; the torus cell at
`u ∈ (4.18777, 4.19084)`, `v ∈ (6.27091, 2π)` lies below its face's
parallel `v = 2π` (`YLo`, face side above), so it is outside every
line, and `vacuous` is asked. The parallel holds two edges:
`ei 2` over `u ∈ [4.18879, 2π]` and `ei 4` over `u ∈ [2.0944, 4.18879]`,
meeting at the joint `u = 4π/3` inside the cell. `vacuous` asks
`covered` of each edge's range alone, and neither covers the cell's
side; their union does. So `present_chart` returns `Local`
(its "outside every near line and not vacuous" arm) at every depth.
`blend_bore_two_rims::each_wall_seam_is_split_once_per_band_and_named_after_its_source`
is the same at `v = π`. Checked against the printed ranges. Of the
bucket's 13 rows, 12 are rims of this kind (`blend_bore_two_rims`,
`closed_chain_junctions`, their review probes,
`rim_of_structural_review_probes`, `tier_gate`'s import corpus) and
one is not (`germ_plane_torus_chord::tangs_tube_on_a_latitude_builds_its_true_body`):
each was run, see "The joint reading" below.
Cheapest: `sweep::all blend_value_decided_rows::a_disc_rim_records_its_joints_and_its_supports` (0.76 s).

**Outside one line and inside the other, at a corner (11 tests,
plane/torus and torus/plane, Boolean results).** In
`contact11_torus_chart_l::a_notched_half_donut_answers_its_volume` the
torus cell at `u ∈ (0, 7.7e-4)`, `v ∈ (0, 6.1e-3)` sits at the chart
corner where the meridian `u = 0` (face side `u > 0`, over
`v ∈ [−π/2, 0]`) meets the parallel `v = 0`, the face below it. The
cell is inside the meridian's half and outside the parallel's, so
`vacuous` is not asked (`inside` holds). The outside line is dropped,
the meridian is the cell's `XLo` clip, and `sides_are_the_faces` finds
its edge does not cover the cell's `v` range (`want = (0, 6.1e-3)`,
`ranges = [−π/2, 0]`): `Local` at every depth. The cell holds none of
the face's interior. Of the 7 plane/torus Boolean rows and the 4
torus/plane ones, 4 are this shape and 7 are not: each was traced, see
"The corner shape, traced" below. Cheapest: `sweep::all contact11_torus_chart_l::a_notched_half_donut_answers_its_volume` (1.1 s).

Neither instance is closed by PR 7115's topo diff (checked for the
Boolean one).

## The joint reading (EMBED collinear-cover lane, 2026-11-22)

**What landed.** A cell's side is covered where the edges of its line,
together, cover it: `joints` (`crates/topo/src/injective.rs`) reads two
edges of one line as end to end where the high end of one and the low
end of the other are one vertex by key, and `vacuous`,
`sides_are_the_faces` (through `run_on`) and the zero side's
`on_edges` all read their cover through it (`line_ranges`). The joint
is read by the vertex and nothing is compared. On the closed-form
charts only.

**Why by key and not by the ranges.** The two edges' stored ends at a
joint are not equal. Measured on the disc rim's three joints
(`a_disc_rim_records_its_joints_and_its_supports`, f64): `9v1` equal
bit for bit, `8v1` one ulp apart (`2.0943951023931953` against
`2.0943951023931957`), `7v1` across the seam (`6.283185307179586`
against `5.55e-17`). Each end is its own row's pcurve evaluated at its
own parameter and lifted as a point
(`the-lift-reads-a-rows-ends-as-points-at-f64-so-a-joints-two-rows-read-a-rounding-apart`).
The probe that tells this from "the union alone closes it": a build
that joins a line's edges only where the two ends' enclosures meet
closes 0 of the bucket's rows, and moves the disc rim's refusal from
the outside cell (pair `3v1`/`9v1`, `vacuous`) to the cell on the
face's side of the same joint (pair `4v1`/`9v1`, `sides_are_the_faces`:
`want = (2.09235, 2.09542)`,
`ranges = [5.55e-17, 2.0943951023931953], [2.0943951023931957, 4.18879]`).
So `sides_are_the_faces`, which already took a clip's edges together,
failed on the same joint by one ulp, and the cause of the bucket was
the joint's two readings, in both readers, not the one-edge reading
alone.

**The bucket, through the gate** (`embed/collinear-cover-measure`, the
census's 13 cylinder/torus rows, f64): 0 of 13 pass on main, 11 of 13
with the joint reading. The two left:
- `step-import::all tier_gate::every_corpus_import_passes_the_shared_gate`
  no longer refuses at a rim; its payload is now
  `Unsupported { carrier: "a free-form face with a boundary edge that is not an iso of its chart", face: 1v1 }`,
  a later body of the corpus.
- `sweep::all germ_plane_torus_chord::tangs_tube_on_a_latitude_builds_its_true_body`
  is no rim: cylinder `3v1` against torus `9v3` across `6v3` runs out
  of depth with `zero_of` refusing the torus cell, which is then read
  `Zero::Past` and signed by no separator, unchanged (the trace is on
  `zero-of-reads-local-where-a-shared-vertex-is-the-corner-of-no-one-clip-pair`).

The census's one torus/cylinder row,
`editor-core::all coincidence_door::the_coaxial_stage6_unions_are_green_at_f64_on_their_coaxial_rows`,
was run beside them and is unchanged (torus `5v1` against cylinder
`8v3` across `11v5`: both cells presented, the torus's with no zero
side, and no separator decides; not traced further).

## The corner shape, traced (EMBED collinear-cover lane, 2026-11-22)

Each of the 11 plane/torus and torus/plane Boolean rows was run with
the gate and the trace, with a print of the face's extent as
`face_extent` reads it. Nothing was built for them.

**4 of the 11 are the corner shape.** In each the torus cell lies past
the END of the one edge on its clip line, beside the corner where that
edge meets a line the cell is outside of, and `sides_are_the_faces`
finds the clip's edge does not reach the cell's range:
- `contact11_torus_chart_l::a_notched_half_donut_answers_its_volume`.
  The extent is KNOWN, `u ∈ [0, π]`, `v ∈ [−π/2, π/2]`, and it is
  consulted: the cell `u ∈ (0, 7.7e-4)`, `v ∈ (0, 6.1e-3)` meets it.
  The face is an L in its chart (`[0, π] × [−π/2, 0]` and
  `[π/2, π] × [0, π/2]`), and the cell lies in the L's notch, which
  the hull of the L holds. No extent of this face can miss that cell:
  an extent is a rectangle.
- `germ_plane_torus_chord::a_bar_on_the_donuts_meridian_plane_builds_its_true_body`
  (`want = (2.7300758075223053, 2.73049)`, the edge over
  `[π/2, 2.7300758075223044]`),
  `…::a_box_edge_piercing_a_one_face_torus_builds_its_true_body` and
  `…::a_one_face_torus_whose_vertex_lies_on_the_box_face_builds_its_true_body`.
  In all three the extent WRAPS (`u` a whole turn), so `sheets` leaves
  both axes free and no cell is read against it.

So `Missed` cannot answer this shape: where the extent is known the
cell is inside it, and where the face wraps there is no extent to
miss. `a-coordinate-line-on-a-face-of-unknown-extent-is-unplaced` is
not the cause in any of the four (none has an unknown extent). The
lune lane's cell was closed by an extent because that face is a
rectangle in its re-posed chart; an L, or a face round a whole turn,
is not.

**What it calls for** is this row's own question, unchanged: a reading
for a cell past an edge's end at a convex corner. The cell is outside
one line along an edge that covers its side, and the line it is inside
of has no edge beside it (its edge ends at the corner vertex the two
lines share). `vacuous` would read it empty if asked, but `inside` is
set by a line whose edge does not reach the cell. A reading that asks
`vacuous` when every line the cell is inside of ends, at a vertex
shared with a covering outside line, before the cell's range would
close all four; it changes what the cover requires at a corner, so it
is a design question (the touch and cover readings are frozen with the
sliver), not a diff.

**7 of the 11 are not this row's.** They never reach a depth stop:
the pair is refused `Structural` on both sides ("stuck"), raised in
`beside_curve` at "a corner where a curved edge meets another at a
vertex of the shared cell, which no corner reading reads"
(`curved-presentation-reads-plane-and-cylinder-images-only`, to which
they are added; that the curved edge is a plane section of the torus is
read off the fixtures, not printed):
`germ_plane_torus_frame::a_nearest_tie_in_band_on_a_bar_across_the_hole_escalates_by_name`,
`germ_plane_torus_spiric::the_donuts_spiric_poses_build_their_true_bodies`,
`germ_plane_torus_spiric::the_fat_corner_bars_lens_is_the_true_body`,
`germ_torus_doors::a_chord_across_the_hole_is_pierced_not_passed`,
`mesh::all lift_probes::a_ring_sits_in_its_outer_loops_polygon_but_on_the_ball_less_the_tube`,
`mesh::all reach_probes::a_ring_stored_a_period_away_meshes_where_the_door_moves_it`
and
`mesh::all reach_probes::a_torus_wall_holding_a_spiric_ring_meshes_on_the_torus_row`.

## One line by key, and the measurements on the merged tree (EMBED collinear-cover lane's fix pass, 2026-11-22)

**Which edges are one line is read by key too.** The review of PR 7322
showed the joint was reached only where two rows' held values met:
`vacuous` gathered a line's edges by their lines holding a common
value, and `present_chart`'s merge refused two clips on one side whose
values did not. Two rows of one parallel hold its value a rounding
apart in a frame that is not the chart's (the three-arc disc mirrored
across a generic plane: `0` and `4.0e-16`), so the joint was never
read and the body refused at depth 40. Now edges holding one axis, the
face on the same side of each, joined at a vertex by key with one
continuation, are one line whatever their rows hold, and the line's
value is the hull of theirs; edges no vertex joins stay two lines
unless their values meet, as before. The joint is listed among
`injective`'s ε concessions with its bound (`4ε` in metres, from each
row's `pcurve_envelope` and the edge certificate's end pins).

**Measured on the branch merged with main `a3d5ce89c`**
(`embed/collinear-cover-measure`, one switch between main's reading and
the branch's):
- **The census's 13 cylinder/torus rows, gate asking, f64:** 0 pass
  with main's reading, 10 with the branch's. Left:
  `tier_gate::every_corpus_import_passes_the_shared_gate` (its rim
  refusal `1v1`/`6v1` across `2v1` is gone; it now stops at
  `1v1`/`2v1` across `3v1`, another body);
  `germ_plane_torus_chord::tangs_tube_on_a_latitude_builds_its_true_body`
  (unchanged, no rim);
  `rim_of_structural_review_probes::every_circle_edge_of_the_public_door_corpus_answers_against_the_oracle`,
  which passed with the joint on the lane's first base and on this main
  refuses `1v1`/`2v1` across `1v1` with either reading: a refusal main
  has gained since, on a body the joint does not read. Not traced.
- **The corpus bushing**
  (`coincidence_door::a_bushing_stacked_on_its_own_axis_glues_its_walls`):
  refuses `1v19`/`3v1` across `20v5` with main's reading and **passes**
  with the branch's, both solids' asks `Embedded`.
- **The tour walk, 85 stops:** main holds 78, the branch 79. The one
  line that differs is `lily/lily_lantern`: `Refused Uncertified
  { 3v1, 12v1, Edge(19v1), Departure }` (76 refused cells) on main,
  `Embedded` on the branch (`cand 33 disc 5620 refused 0 splits 5587`).
  `diepips`, `diecomposed` and `diechamfer` refuse `8v3`/`39v3` on both.
- **The mirrored disc** holds at `f64`; at `Interval` it runs out of
  depth off the cover
  (`the-mirrored-rim-band-at-interval-runs-out-of-depth-in-the-distance-arm`, on EMBEDWALK's slate).

**Cost, left as it is:** `Lines::of` is built up to three times in one
presentation (`vacuous` or the merge, `sides_are_the_faces`, the zero
side) and `joints` once more inside the merge where two clips' values
do not meet; each is a few comparisons over the cell's near edges.
