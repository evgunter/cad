---
id: measure-plane-of-folds-a-planar-face-by-hand
kind: issue
title: editor-core's measure plane_of folds a planar face's outward normal by hand, beside face_normal's door
status: open
opened: 2026-11-13
---


## Finding

`crates/editor-core/src/eval/measure.rs` `fn plane_of` reads a face key,
matches `Surface::Plane { origin, normal, .. }` and mints
`OutwardNormal::from_chart(*normal, face.sense)` itself. That is the
planar fold `topo::face_normal` states as its door's: `face_plane` (the
origin and outward normal as a `PlaneDesc`) and `face_outward_normal`
(the outward normal alone). A second spelling of the fold outside topo,
where the door's guard row (which walks `topo/src` only) cannot see it.

`plane_of` also hands out the CHART normal beside the outward one,
because `angle` reads the chart normal on purpose
(`work/coincide/a-face-read-as-a-plane-has-no-stated-sense.md`). So the
fold cannot simply be replaced by `face_outward_normal`. Which plane a
face read as a plane hands out is that row's open question, and this
row waits on its answer. `face_plane` is `pub(crate)`, so
`face_normal::face_carrier_desc`'s plane arm is the public door that
answers the origin and outward normal today.

The sibling in `sweep`, `blend/build.rs` `fn outward_of`, is already
`work/census/S57.md`'s.

Found by S173's sweep for hand-built planar folds (`from_chart(` or
`chart_outward_normal(` in a file that also destructures
`Surface::Plane { .. normal .. }`, over `topo/src`, `sweep/src` and
`editor-core/src`).
