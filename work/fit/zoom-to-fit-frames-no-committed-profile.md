---
id: zoom-to-fit-frames-no-committed-profile
kind: issue
title: Zoom to fit frames bodies only, so a document holding only profiles, committed or being authored, has nothing to frame
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

## Also a profile being AUTHORED, and Ev's steer on the empty case — 2026-09-24

Reported by Ev from the running viewer: **making a profile in an
otherwise empty scene, Zoom to fit fails** with

> camera: the scene bounds give a radius of 0, which is not a positive
> extent to frame against (from frame the given bounds at aspect
> 1.4754263191763193)

That is this row's defect reached by a second route the row does not
name. The row is about **committed** profiles; this is a profile still
being authored — a **preview**. Previews are rendered in the viewport:
`app.rs` holds them as `profile_previews` (the create and edit doors,
`ProfileDoors<Option<Result<ProfilePreview, PreviewError>>>`) and
`pane/viewport.rs` draws them as `EdgeOverlay::preview`. The fit at
`pane/viewport.rs` (the `pending_fit` block) still builds
`CameraOp::Frame { bounds: self.scene.bounds(), .. }` — the mesh alone.
So the geometry a reader is looking at, and is most likely to want
framed at that moment, is drawn and not framed.

**Ev's steer, in Ev's own words, recorded as input to "What a fix has to
decide" above rather than as a ruling:**

- *"profile previews should ideally be treated like rendered actual
  geometry for the purposes of zoom to fit"* — so the framed extent is
  the union of what is **drawn**, and a preview joins it alongside a
  committed profile. That settles the population question above for
  previews in the same direction the row already leans for committed
  profiles: frame what the viewport shows, not what the scene mesh
  holds.
- *"and possibly there should be a sensible default when there's no
  geometry at all"* — this cuts against the row's closing lean that *"a
  datum-only document may be correctly unframeable"*. Ev said
  "possibly", so it is a preference to weigh and not a decision; but the
  lane taking this row should treat **a default framing** as the
  expected answer for an extent-free scene and argue if it disagrees,
  rather than defaulting to the refusal.

**What the fix now has to cover**, restated with this evidence: the
extent a fit frames is the union of the scene mesh, every drawn
committed profile's loops, and **every drawn preview's** loops (a
preview that failed to replay draws nothing and so contributes
nothing); and when that union has no positive extent, the fit lands on
a stated default rather than refusing — or the row says why a refusal is
right. The wording of the refusal is a separate defect and has its own
row, `a-degenerate-frame-refusal-prints-an-aspect-nobody-can-act-on`.
