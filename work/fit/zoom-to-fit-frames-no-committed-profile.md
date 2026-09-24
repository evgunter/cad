---
id: zoom-to-fit-frames-no-committed-profile
kind: issue
title: Zoom to fit frames bodies only, so a document holding only profiles has nothing to frame
status: open
opened: 2026-09-18
priority: P0
cost: D
---


Found while checking `committed-profiles-are-not-drawn-in-the-viewport`
in the running viewer (branch `vgeom/overlay-lanes`).

## Finding

A committed profile is now drawn every frame from the landed
evaluation (`sketch::committed`, filled into `EdgeOverlay::profiles`
in `pane::viewport`). The camera still frames only the scene MESH:
`ViewerApp::new` frames `mesh.bounds()` (`Camera::framing`), and the
fit that `pending_fit` requests goes through `CameraOp::Frame` with the
scene's bounds (`camera::apply`, `Camera::fitted`).

So a document whose roots are profiles and no body opens with the
toolbar reading

> camera: the scene bounds give a radius of 0, which is not a positive
> extent to frame against

and the profile, drawn where it lies, is wherever the unfitted camera
happens to put it. Driven with a frame plus one annulus profile as the
only root: the viewport showed the grid and the frame's arrows, and the
profile was not on screen.

## What a fix has to decide

Whether the framed box is the mesh's bounds joined with every drawn
committed profile's world-space loops (the same `sketch::committed`
drawing the viewport makes), and whether datums join it too — a datum
has no extent of its own (`datums::draws` sizes it against the view),
so a datum-only document may be correctly unframeable.
