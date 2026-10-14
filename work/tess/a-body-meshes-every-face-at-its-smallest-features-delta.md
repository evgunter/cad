---
id: a-body-meshes-every-face-at-its-smallest-features-delta
kind: issue
title: A body meshes every face at one chordal delta, so a small arc on one face re-meshes the whole body
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## What

`mesh::tessellate(body, delta, tol)` takes one absolute chordal
deviation for every face of the body. A body whose faces span radii
pays its smallest feature's budget on its largest faces.

The tour now states a per-body delta (`SceneBody::finer`,
`demos/tour/src/main.rs`; `work/show/a-tour-scene-meshes-every-body-at-one-delta.md`),
which is the most a caller can do through this door.
`tiltedcut/tiltedcut_above` (`demos/tour/src/curvedcut.rs`) is the case
that is left: the engraved glyphs' inner arcs (radius 0.1 to 0.15) want
2e-3, and they sit on the same body as the cylinder. In
`docs/tess-budget-data/tess-budget-baseline.csv` the two cylinder walls,
faces 1 and 25, carry 1044 triangles each, 2088 of the body's 2608.

The lower half has the same walls and no glyphs. At 1e-2 the whole
lower half is 480 triangles against 2232 at 2e-3. The scene still meshes
both halves at 2e-3, because one half at 1e-2 beside the other at 2e-3
renders visibly coarser on a cylinder the frame presents as one piece.
The fix here would let both halves drop their walls to 1e-2 together.

## The shape of a fix

One of these:

- a delta relative to each face's curvature radius;
- a per-face delta the caller can name.

Either one is a change to the contract of the door, so it is a design
question for TESS rather than a demo change.
