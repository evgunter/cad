---
id: a-tour-scene-meshes-every-body-at-one-delta
kind: issue
title: A tour scene meshes every body at one chordal delta, so a small arc in one body re-meshes the whole scene
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## What

The tour's render door takes one chordal deviation per scene:
`Stop::delta` (`demos/tour/src/main.rs`), handed by `run_body` to
`mesh::tessellate` for every `SceneBody` alike. `mesh::tessellate`
itself takes a delta per call, so the gap is the tour's: a scene has
no way to say that one body, or one feature, wants a finer budget.

Two scenes pay for it:

- `tiltedcut` (`demos/tour/src/curvedcut.rs`, at its `delta`): the
  engraved glyphs' inner arcs (radius 0.1 to 0.15) want 2e-3, and the
  whole cylinder is meshed at it too. The review measured the upper
  half at 684 triangles at 1e-2 and 2608 at 2e-3, about 4×.
- `lily` (`demos/tour/src/lily.rs`, the comment above its `delta`):
  at 2e-3 the lantern is smooth and a 0.06 m stem tube costs about
  2e5 triangles.

## The shape of a fix

A per-body delta on `SceneBody` (defaulting to the scene's), or a
delta relative to each face's curvature radius in `mesh`'s budget.
The first is the tour's alone; the second is TESS's door.
