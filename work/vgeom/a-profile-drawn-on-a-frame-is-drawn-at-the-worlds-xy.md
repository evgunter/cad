---
id: a-profile-drawn-on-a-frame-is-drawn-at-the-worlds-xy
kind: issue
title: A profile drawn on a frame, and a trace shown in it, are drawn at the world's xy rather than on the frame
status: open
opened: 2026-10-30
---

A profile drawn on a frame (`ProfileProgram::on`, INTENT stage 6 C) evaluates in the frame's coordinates, and the frame stands in a body's space: its `xy` is the sketch's plane there. The viewer draws every committed profile on its validated plane in its own coordinates (`viewer::sketch::committed`, `crates/viewer/src/sketch.rs`, the `CommittedProfile { plane: *profile.validated.plane(), .. }` arm), which is the world's `xy` for a free profile, as `sketch::profile_plane` says. A drawn profile is drawn there too, not on its frame carried through its space's placements to the world.

The traced path's trace preview inherits the same placement (`viewer::sketch::trace_preview`, on `profile_plane()`; INTENT stage 6 E). On the stage's scene (`boss_tangent_by_value`'s cylinder standing at the world's origin, the frame its base) the two coincide, so nothing shows it; a frame off the body's origin, or a body placed off the world's, draws the profile and its trace in the wrong place.

The fix reads the drawn profile's frame and its space's world placement and draws the loops (and the preview) on that plane; the evaluation holds the frame only inside the profile's resolution (`editor_core::eval::wire::trace::drawn`'s `Drawn::frame`), so the profile value, or a read beside `editor_core::traced_faces`, would have to hand it out.
