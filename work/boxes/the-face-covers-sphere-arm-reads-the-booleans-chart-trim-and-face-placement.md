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

Three chord_join test fixtures now ask the curved containment door
whether a ring they plant lies in its face: `cone_ring_rows`'
`assert_island_in_face`, and `torus_ring_rows`' and `sphere_island_rows`'
`empty_ring`, each reading `curved_face_containment` and
`FaceContainment`. Their `names-boolean` pins cite this row, and they
read the door from the root once it moves.
