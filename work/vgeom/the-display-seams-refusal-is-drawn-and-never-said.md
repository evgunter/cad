---
id: the-display-seams-refusal-is-drawn-and-never-said
kind: issue
title: The f32 seam's overlay refusal drops a leg and its pane refusal drops a frame, and neither reaches a reader
status: open
opened: 2026-09-21
priority: P2
cost: D
---



Filed by `vgeom/f32-seam`, which gave the `f64` → `f32` narrowing one
home (`crate::narrowing::Narrow`) and a refusal, and is naming the
dispositions it could not make sayable rather than leaving them in a
PR body.

## Three refusals, and only one of them is heard

`Narrow::narrow` answers `None` for a value whose narrowing is not a
finite `f32`. What the caller does with that is the caller's, and the
three callers differ:

- **`crate::scene::SceneMesh::build` refuses the scene**, typed, as
  `SceneError::UndrawablePosition`. That reaches the reader: the δ
  seam already renders a `SceneError` through `frame::delta_refusal`.
- **`crate::camera::Camera::view_projection_f32` refuses the frame**,
  typed, as `CameraError::UndrawableProjection`, and
  `pane::viewport::viewport_ui` writes it into the projection fault
  the toolbar already badges (`frame::projection_badge`). That reaches
  the reader too.
- **The overlay lanes draw nothing and say nothing.**
  `marks::segments_of` omits a leg whose end does not narrow and
  `pane::viewport::push_segment` does the same; both are documented
  and neither is reported. So is the pane's own extent: if
  `[viewport.width_px, viewport.height_px]` or `pixels_per_point`
  does not narrow, `viewport_ui` returns without adding the paint
  callback and writes no fault.

## Why it was not fixed where it was found

The channel the third case wants is a **badge** — a read of held state
true on every frame until the picture changes — and the badge family
is `crate::frame`'s, which is VNEWS's ground rather than this
program's. The value not reaching the screen is this program's charter
(*a wrong number, or no number, reaches the screen*), which is why the
row is filed here; the sentence beside it is VNEWS's, which is why it
was not written across that fence.

`work/vseam/projection-fault-has-no-sweeper.md` is the neighbouring
hole in the same field and should be read with this.

## What a fix would have to decide

Whether an overlay lane that dropped a leg is worth a badge at all. An
argument against, stated so it is not lost: at the magnitudes that
reach these arms — a coordinate past `f32::MAX`, about `3.40e38` — the
picture is already nowhere, so the badge would name a leg nobody could
have seen. An argument for: a leg that is *missing* from an outline
reads to a person as an authoring mistake, and the drop is the one
thing that could tell them otherwise. The datum lane already has the
shape to copy (`datums_vanished`, a count recomputed every frame and
badged).
