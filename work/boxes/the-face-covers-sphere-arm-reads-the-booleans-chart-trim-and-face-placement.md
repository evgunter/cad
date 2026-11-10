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
  `solid_contain::sphere_chart_trim` and its
  `PointInSolidError`;
- the pole-side cut along an aim (`cover.rs` `pole_side`) reads
  `contain::curved_face_placement`, `CurvedPlacement` and
  `FaceContainment`.

They answer in engine types (`PointInSolidError`, `CurvedPlacement`,
`FaceContainment`, `ContainError`, all declared under `boolean/`), so
this is not a two-function move: it is the curved containment door's
move to the root (`contain.rs`, the sphere chart trim in
`solid_contain.rs`, and the region readers they read, with
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
`contain::placement_on_carrier` (a point held on the face's
carrier; `face_placement` is the decided-on-plane form), engine-private
under `[topo.homes]`. Two crate-root sites still spell their own
plane/curved split over `contfp` and `curved_face_containment`, because
they cannot reach it from the root: `census.rs` `placed` and
`cell_region.rs` `place`. When the door moves to the root, they read
it from there, each with its `OffCarrier → Out (trusted on carrier)`
policy at the call
(`work/onecopy/carrier-held-points-are-trusted-not-decided.md`).

`crate::boolean::face_placement`, which census already reads from
the root (`census::face_holds`, `census/in_plane.rs`), is not
theirs: it decides the on-plane margin, and these two trust the point
on the plane (the follow-up row's subject); making them decide it is
that row's outcome-moving change, not a reroute.

## A second held door waits on this move too (PR 6741, 2026-11-10)

`census.rs` `placed` is the census's held placement. It checks the
point's premise (`Held::check`), then places it with no on-carrier
decision: a plane through `contfp` (census `contain`), a curved face
through `crate::boolean::curved_face_placement`. That is a near-copy of
the boolean's `placement_on_carrier` + `placement_unheld`. The census
cannot call those, since they are engine-private
(`pub(in crate::boolean)`, taking the `pub(crate)` `Held`).

PR 6741 gave the copy the boolean's policy: a curved `OffCarrier` is the
kernel's fault (`census_fault`), no longer read as `Out`. It also routed
two more census sites through it (`curved_overlap_cells`, point.rs
`in_face`). `placed` now serves four held sites:
- `in_overlap_closure`'s touch, `constructed`;
- `meets_at`'s probe, `in_band`;
- the two above, `in_band`.

Their probe readings were 100% Zero (6669: 40,379 calls; 6741: 276 and
60 calls), so no production answer moved. The change raised `census.rs`'s
`names-boolean` pin from 432 to 434, cited to
`work/onecopy/the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes.md`,
whose placement move retires these names.

**When containment moves to the root**, `placed` should become a call to
the moved `placement_on_carrier`, with the census mapping its
`OffCarrier` to `census_fault`. Census `contain`'s `contfp` site and
`placed`'s two `crate::boolean` lines go with it.
