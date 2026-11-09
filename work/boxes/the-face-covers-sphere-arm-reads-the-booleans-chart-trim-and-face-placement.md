---
id: the-face-covers-sphere-arm-reads-the-booleans-chart-trim-and-face-placement
kind: issue
title: topo::cover reads the boolean's curved containment door (sphere_chart_trim, curved_face_placement, face_placement), which [topo.homes] wants at the root
status: open
opened: 2026-11-08
priority: P2
cost: H
---


Found by `cover-topo-faces`, which moved topo's face and edge readings
to the crate root (`crates/topo/src/cover.rs`) under `[topo.homes]`
(`crates/topo/README.md`; gate `scripts/gates/engine-leaves.sh`).

Two arms of `topo::cover::face_cover` still read readings the boolean
engine hosts, so `cover.rs` carries a cited `names-boolean` pin
(`scripts/gates/engine-leaves.counts`):

- the sphere arm's chart window (`cover.rs` `sphere_window`) reads
  `boolean::solid_contain::sphere_chart_trim` and its
  `PointInSolidError`;
- the pole-side cut along an aim (`cover.rs` `pole_side`) reads
  `boolean::contain::curved_face_placement`, `CurvedPlacement` and
  `FaceContainment`.

They answer in engine types (`PointInSolidError`, `CurvedPlacement`,
`FaceContainment`, `ContainError`, all declared under `boolean/`), so
this is not a two-function move: it is the curved containment door's
move to the root (`boolean/contain.rs`, the sphere chart trim in
`boolean/solid_contain.rs`, and the region readers they read, with
their refusal types), after which the face cover reads it from there.
The face's nearest-distance reading (`cover.rs` `face_nearest`) reads
the planar `face_placement` through the same door, and is in the same
pin.

**Fix**: move the curved containment door and its types to root
modules, repoint the boolean's readers, and drop `cover.rs`'s
`names-boolean` line.

## Note from `onecopy/path-parity` (2026-11-09)

The chord_join ring rows on cone, torus and sphere sheets ask the curved
containment door whether the islands they plant lie in their faces, in
one test helper, `chord_join::island_rows::assert_in_face`, which reads
`curved_face_containment`, `FaceContainment` and `ContainError`. Its
`names-boolean` pin cites this row, and it reads the door from the root
once the door moves.

## Note from `onecopy/face-placement` (2026-11-09)

The boolean's one placement dispatch is now
`boolean::contain::placement_on_carrier` (a point held on the face's
carrier; `face_placement` is the decided-on-plane form), engine-private
under `[topo.homes]`. Two crate-root sites still spell their own
plane/curved split over `contfp` and `curved_face_containment`, because
they cannot reach it from the root: `census.rs` `placed` and
`cell_region.rs` `place`. When the door moves to the root, they read
it from there, each with its `OffCarrier → Out (trusted on carrier)`
policy at the call
(`work/onecopy/carrier-held-points-are-trusted-not-decided.md`).
