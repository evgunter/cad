---
id: zero-of-reads-local-where-a-shared-vertex-is-the-corner-of-no-one-clip-pair
kind: issue
title: zero_of reads Local where the cell holds a shared vertex that no zero side holds and that is the corner of no one pair of its clips, so a torus beside a coaxial seam refuses Departure
status: open
opened: 2026-11-21
priority: P0
cost: M
---


## Finding

From EMBED's per-solid census trace (lane `embed/trace-cone-torus`,
2026-11-21), main `c2a0c41f40` with the gate and its trace switches
(`embed/trace-cone-torus-measure`).

`zero_of` (`crates/topo/src/injective.rs`), in the arm with no zero
side and one loose shared vertex, counts the corners of two clips on
different axes holding the vertex and refuses `Local` unless there is
exactly one (`match corners.as_slice()`, `_ => Local`).

One traced row ends there at depth 40,
`germ_circle_torus::a_coaxial_seam_on_the_torus_is_placed_by_its_arc`:
torus `3v1` against torus `4v3` across `11v3`. `4v3`'s cell at
`u ∈ (π/4, π/4 + 1.5e-3)`, `v ∈ (5.9518, 5.9641)`, holding the shared
vertex `8v1`, is `Local` at that arm; `3v1` is presented by a side
(`Zero::Side(YHi)`).

**Unchecked:** whether the count is zero (the shared edge is not a clip
of the cell, so its end is no corner) or two. The diagnostic print sits
on the other arm. A cell that holds a shared vertex the shared edge ends
at, with that edge not a clip, is the shape to test first.

## Count

1 test (torus/torus). A valid body, built at its closed form.

**Reproducer:** `sweep::all germ_circle_torus::a_coaxial_seam_on_the_torus_is_placed_by_its_arc`
(1.3 s).

## A candidate second instance, cylinder against torus (EMBED collinear-cover lane, 2026-11-22)

`sweep::all germ_plane_torus_chord::tangs_tube_on_a_latitude_builds_its_true_body`
(the census's cylinder/torus bucket) runs out of depth with `zero_of`
refusing `Local` on its torus cell, on `embed/collinear-cover-measure`
with the gate and the trace: cylinder `3v1` against torus `9v3` across
`6v3`, the shared cell holding vertex `8v1`. At depth 40 the cylinder's
cell `u ∈ (0, 1.53e-3)`, `v ∈ (−1, −0.992)` is presented by a side
(`Zero::Side(YLo)`); the torus's cell `u ∈ (0, 3.07e-3)`,
`v ∈ (0.78540, 0.79153)` is refused by `zero_of` and then presented by
`partial_zero` as `Zero::Past(YLo)`; and the separators read
`planes = (Positive, Positive)`, `graph = (Positive, None)`, which
certifies nothing.

**Not told apart:** whether the torus cell would be signed had
`zero_of` read its side (this row's cause), or whether `departure`
signs no `Past` side here whatever `zero_of` reads. The row was
assigned to the rim bucket of
`vacuous-reads-a-cell-at-a-corner-local-at-every-depth` by family and
is unchanged by that row's joint reading.

## Census tests the cylinder fix leaves at zero_of (EMBED lane `embed/iso-end`, 2026-11-22)

In these 2 the refusing pair's partner exits at `zero_of`'s loose-vertex arm (the shared vertex is the corner of no one clip pair).

Measured on `embed/iso-end-measure2` (`origin/embed/gate-measure` @ `ebb2e6f8e` plus `embed/iso-end`'s diff), where each refusal is the same pair, test by test, as on the trace run that named the exits (`embed/iso-end-measure` @ `030c75763`, `CV_TRACE`).

- `sweep::all a_minted_vertex_on_a_curved_face::a_wedge_on_the_ruling_answers_every_op_in_both_slots`
- `sweep::all parallel_cylinder_join::a_seam_lying_on_the_other_wall_joins_along_the_carried_ruling`

## After embed/clip-vertex: the arm is gone, the row still refuses (EMBED, lane embed/clip-vertex, 2026-11-22)

`zero_of`'s arm no longer refuses where the corner count is not one: a
lone loose vertex on one clip is that clip's side through it, and any
other names no zero. Measured on `embed/clip-vertex-measure` (main
`b99f71c3d`, the gate logging and passing):

- `germ_circle_torus::a_coaxial_seam_on_the_torus_is_placed_by_its_arc`:
  8 asks, 3 refused, before and after. Its `zero_of` `Local` exits fall
  2,543 to 2,198, all now `zero_of`'s last arm (more zero sides than a
  corner has); its 1,554 `present_chart` exits are unchanged. Which of
  the two is the refusing cell's exit now is not traced.
- `germ_plane_torus_chord::tangs_tube_on_a_latitude_builds_its_true_body`
  (the candidate second instance): 20 asks, 3 refused, before and
  after; `zero_of` exits 1,634 to 224. The torus cell is presented
  `Past(YLo)` as before and the conic's plane reads both faces
  `Positive`, so the question the collinear-cover lane left open is
  answered on one side: reading the vertex does not sign the pair. Both
  faces leave the circle's plane to one side, and the graph separator
  does not read a side past the shared cell.
