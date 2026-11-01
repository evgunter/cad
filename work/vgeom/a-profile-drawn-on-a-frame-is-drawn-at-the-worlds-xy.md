---
id: a-profile-drawn-on-a-frame-is-drawn-at-the-worlds-xy
kind: issue
title: A profile drawn on a frame, and the trace the sketch tool shows in it, are drawn at the world's xy rather than on the frame
status: open
opened: 2026-10-30
priority: P2
---

A profile drawn on a frame (`ProfileProgram::on`, INTENT stage 6 C) evaluates in the frame's coordinates, and the frame stands in a body's space: its `xy` is the sketch's plane there. The viewer draws every committed profile on its validated plane in its own coordinates (`viewer::sketch::committed`, `crates/viewer/src/sketch.rs`, the `CommittedProfile { plane: *profile.validated.plane(), .. }` arm), which is the world's `xy` for a free profile, as `sketch::profile_plane` says. A drawn profile is drawn there too, not on its frame carried through its space's placements to the world.

**The sketch tool's trace preview shows it** (INTENT stage 6 E, `viewer::sketch::trace_preview`, drawn on `profile_plane()`). The tool's frame stands its origin on the traced face's axis, at a vertex of the picked plane face's foot: on a cylinder whose disc is centred off its body's origin (the review probe `r1_offset_cylinder_frame_moves`, now the row `trace_tool::an_offset_cylinder_keeps_its_boss_through_a_centre_edit`), the frame's origin is the axis's foot, so the shown trace and the committed footprint are drawn about the world's origin, off the cylinder by the disc's centre; on a top-cap pick they are drawn at `z = 0` rather than on the cap. The boss built from them stands where it should; only the drawing is off.

The fix reads the drawn profile's frame and its space's world placement and draws the loops (and the preview) on that plane. The evaluation holds the frame only inside the profile's resolution (`editor_core::eval::wire::trace::drawn`'s `Drawn::frame`), so the profile value, or a read beside `editor_core::traced_faces`, would have to hand it out, and the preview would read its frame the same way (`pncad::trace::sketch_trace` already lowers it).
