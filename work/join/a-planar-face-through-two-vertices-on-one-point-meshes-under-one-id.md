---
id: a-planar-face-through-two-vertices-on-one-point-meshes-under-one-id
kind: issue
title: A planar face whose loop passes two vertices on one point meshes both under one id and panics the chord census (main ships 13+ such SOUND bodies)
status: review
opened: 2026-10-05
priority: P0
cost: M
refs: [a-pinch-no-kept-face-can-cross-refuses, a-boolean-ships-a-face-whose-loop-passes-two-vertices-on-one-point, a-pinch-union-body-trips-the-watertight-census-in-one-member-order]
parent: a-pinch-no-kept-face-can-cross-refuses
branch: join/pinch-one-vertex-per-cone
pr: 4074
---


## What

Found by `join/pinch-one-vertex-per-cone` at its step 0, on main
`f1a4a317`. Under Ev's ruling on PR 4057 ("a pinch is one vertex per
cone"), a face whose outer loop passes two distinct vertices on one
point is a right body. Main already ships such bodies as `SOUND`
(`work/cleave/a-boolean-ships-a-face-whose-loop-passes-two-vertices-on-one-point.md`).
`mesh::tessellate` panics on every one of them.

**Cause.** The planar lane (`crates/mesh/src/planar.rs`, the CDT
insertion pass that fills `meta`) inserts each boundary point into
spade, which dedups equal positions to one handle. `meta` gains an
entry only for a new handle (`h.index() == meta.len()`), so the second
vertex on the point keeps the first one's mesh id. The face's triangles
at the pinch all carry that id. The neighbouring faces at the other
vertex emit their chords under its own id (`tessellate_impl` gives
every topology vertex its own id). Those chord segments go unpaired, and
the census over `unpaired_chord_segment` in `tessellate_impl` fires.
`[profile.release]` sets `debug-assertions = true`, so it fires in
release as well. Same-position dedup is how the lane cancels a slit
seam's two traversals (`planar.rs` module docs, "Slit note"). A pinch is
two vertices on one position that must stay two ids.

**Measured** (review r2's `r2_pinch_probes` on
`join/pierce-pinch-families-review-r2`, run on main with
`mesh::tessellate(body, 0.05)` and `check_mesh` added per built body).
Every body flagged `FACE2V` (one face's loops through two vertices at
one point) panics. Every other body meshes, including the `pts=1`
bodies whose two vertices on one point lie on different faces. Lines:
- `notch307 fib117`: `edge psi=0.3 pc S`, `edge psi=1.9 cp S`,
  `edge psi=4 cp S`.
- `notch307 fib113`: `edge psi=4 cp S`.
- `notch307 fib105`: `edge psi=0.3 pc U` and `cp U`, `edge psi=4 cp S`,
  `corner psi=1.9 cp S`.
- `vee300 fib100`: `edge psi=4 cp S`, `corner psi=0.3 pc U` and `cp U`,
  `corner psi=1.9 cp S`.
- `vee224bot fib1`: `edge psi=0.3 cp S`.

That is 13 panics against 13 `FACE2V` bodies, and 0 elsewhere on those
poses.

**History.** `a-pinch-union-body-trips-the-watertight-census-in-one-member-order`
(closed, PR 3796) met this shape and read the body as wrong: "The
tessellator is not at fault". It welded the pinch (`finish::weld_pinches`).
The ruling retires that weld, so the shape becomes the norm at every
boolean pinch. `crates/editor-core/tests/union_pinch_member_order.rs`
tessellates the body the weld built.

## The shape to give

At a CDT handle holding several topology vertices, each triangle takes
the id of the vertex whose corner (wedge) of the face contains it. The
wedges come from the boundary walk: each vertex's two boundary
neighbours at that point. The lane's assertion that it grows no
interior point still holds. Check the curved and trimmed lanes for the
same dedup; review r2's cylinder poses (`Lbot cyl fib4 psi=0.9 seam cp
S`, `notchbot cyl fib15 psi=0.9 seam cp S`) reach them. Pin a row on
`notch307 fib117 edge psi=0.3 pc S` that tessellates and passes
`check_mesh`.

## Built (branch `join/pinch-one-vertex-per-cone`, PR 4074)

Claimed from TESS onto JOIN as the pinch unit's prerequisite (the
JOIN orchestrator's call on PR 4074).

`mesh::planar::Pinches` covers a CDT handle the boundary walk brings two
or more distinct mesh ids to.
- **Each id's wedge.** Its two boundary sides at the handle are the
  first constraint sub-edges of its in-segment and out-segment.
- **Each inside triangle at the handle** rotates round it to the nearest
  constraint edge on each side. It takes the id of the one wedge whose
  sides those are.
- **Refusals.** No such wedge, or two, refuses `TessellateError::PinchWedge`
  (the loops cross at the point). So does one id meeting a pinch handle
  twice.
- **The slit's dedup is untouched.** A handle met only by one id is not
  a pinch.
- **Both lanes that dedup this way share it.** The planar lane, and the
  trimmed lane, whose CDT dedups boundary points the same way (a
  cylinder wall through two vertices on one point). The curved lane
  dedups the same way but is unchanged. It meshes only a walk that is
  its own UV box (`require_swept_rectangle`). A box's walk meets one
  point twice only at a pole, where one id repeats. That reading is
  unmeasured: no pinch on a curved iso face was built.

Pinned by:
- `sweep::all pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates`.
  It has 15 rows: the 14 planar `FACE2V` bodies of review r2's cube
  poses, and `Lbot cyl fib4 psi=0.9 seam cp S` on the trimmed lane.
  Each asserts the face is there, then that `tessellate` passes its
  chord census and `check_mesh` passes.
- `mesh planar::tests::a_pinch_gives_each_triangle_the_id_of_the_corner_holding_it`.
- `mesh planar::tests::crossed_corners_at_a_pinch_refuse_typed`.

With the wedge read disabled (every triangle keeps the handle's first
id), all 15 sweep rows panic at the census and both unit rows go red.
With only the trimmed lane's read disabled, the cylinder row panics.
