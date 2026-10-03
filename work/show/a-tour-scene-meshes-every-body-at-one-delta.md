---
id: a-tour-scene-meshes-every-body-at-one-delta
kind: issue
title: A tour scene meshes every body at one chordal delta, so a small arc in one body re-meshes the whole scene
status: closed
opened: 2026-10-02
priority: P3
cost: M
closed: 2026-10-03
pr: 3905
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

## Closed

By #3905. `SceneBody::finer(d)` lets a body mesh finer than its scene; a
coarser or equal delta panics (`scene_body_delta` tests). The lily runs
at 5e-3 with every curved-section body at 2e-3 and only the lofted,
straight-sectioned blades at the scene's δ (72k → 45k triangles).
tiltedcut stays at one 2e-3: two halves of one cylinder at different δ
read as a defect. What a per-body δ cannot fix, a body whose small
feature drags its plain faces fine, is
`work/tess/a-body-meshes-every-face-at-its-smallest-features-delta.md`.
