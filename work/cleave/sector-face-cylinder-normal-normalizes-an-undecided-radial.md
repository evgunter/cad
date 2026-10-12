---
id: sector-face-cylinder-normal-normalizes-an-undecided-radial
kind: issue
title: sector_face's cylinder arm normalizes the vertex's radial off the axis without deciding its length
status: open
opened: 2026-10-06
priority: P3
cost: E
---


## What

`crates/topo/src/sector_face.rs` `resolve`, the `Surface::Cylinder`
arm, takes the chart normal at a corner as
`(w - *axis * w.dot(*axis)).normalize()` with `w` the base vertex
minus the carrier's origin: a hand Gram–Schmidt residual whose length
is the vertex's distance from the cylinder's axis, normalized with no
decision. For a vertex on its face that length is the carrier radius,
which the surface datum gate holds only to `radius > 0`
(`crates/geom/src/surfaces.rs`, "positive by convention"), not to the
band; a radius, or a vertex, within the band of the axis normalizes a
direction made of rounding into the sector's `OutwardNormal`, which
both the boolean's and the splitting lane's sector algebra read. The
torus and cone arms beside it say where their axis poison goes; this
arm says nothing.

Unmeasured: this filing built no fixture, and a cylinder whose radius
is in band may be refused earlier by an operand gate.

## Shape

`UnitVec3::levered` (or `OrthoFrame::from_aim_and_reference` against
the axis) under a `split_`/`bool_`-shared K name, levered by the
radius, refusing through `SectorFaceError`. Found by the
`recl-flanker-representative-normalizes-an-undecided-residual` sweep
(branch `cleave/recl-flanker`) for the hand Gram–Schmidt shape.
