---
id: a-planar-face-through-two-vertices-on-one-point-meshes-under-one-id
kind: issue
title: A planar face whose loop passes two vertices on one point meshes both under one id and panics the chord census (main ships 13+ such SOUND bodies)
status: open
opened: 2026-10-05
priority: P0
cost: M
refs: [a-pinch-no-kept-face-can-cross-refuses, a-boolean-ships-a-face-whose-loop-passes-two-vertices-on-one-point, a-pinch-union-body-trips-the-watertight-census-in-one-member-order]
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
