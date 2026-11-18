---
id: tracing-reads-the-faces-chart-and-traced-frame-retires
kind: unit
title: A traced profile draws on the face's chart like any sketch on a face; traced_frame and the New-traced-profile act retire, and the coincidence door proves the tangency by rung 3
status: open
opened: 2026-11-18
priority: P1
cost: H
refs: [the-coincidence-doors-construction-facts-have-no-one-source]
---


Ruled by Ev on `[ev]` PR 7073 (2026-11-18). Both designers converged on it (the PR body's A and B sections hold the full reasoning).

## The build
- **One act.** A profile on a face may trace faces of its body (`DrawnOn.faces`), drawn on the face's chart (`a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face`). `editor_core::trace::traced_frame`, `TracedFrameFault`, `pncad::trace`'s `traced_frame` re-export and py `ev.traced_frame` retire. `drawn_profile` over any frame stays.
- **What that also fixes:**
  - `traced_frame` takes x and the origin from the first vertex `solid.vertices()` lists (`[intrinsic.contingent]`);
  - its squareness to the picked face is a one-time check at authoring (`TRACED_FRAME_SQUARE`), with the plane never recorded;
  - its axis sense makes a bottom-cap "boss" extrude into the body (`intent_s6_e_traced_frame` `a_boss_in_the_bottom_caps_plane_…` pins it). That row is re-spelled.
- **Tangency stays structural.** Today the door's `swept_tangent` reads a heading only through `stated::frame_z`'s `Axis` reads. Make it compare the sweep's direction with the traced face's sliding direction, the chart's z being its plane's normal.
  - **Try rung 3 first** (Ev 2026-10-06: output definitions plus rung 3). An extrude's cap normal and its wall's generator are both its `d`, as the box mitre's `dot(d, cross(d, t))` is identically zero. Measure it.
  - **Only if rung 3 fails** (too slow or undecidable), add the stated fact, "an extrude's caps are normal to its heading and its walls slide along it", beside `axis_of`, pinned against the symbolic evaluation. Ev allowed this on 7073 ("this can be added to the side channel if the rung 3 test you're running fails, since we'll be revisiting it later"). The revisit is `work/coincide/the-coincidence-doors-construction-facts-have-no-one-source.md`.
  - Expect `a_frame_read_off_a_rim_asks_for_the_walls_axis` to flip, and every existing traced row to stay proven. `Recourse::ReadTheAxis`'s wording changes.
- **Viewer and pncad** (crossings, announced): `NEW_TRACED_PROFILE` / `SessionOp::AddDrawnProfile` / `traced_op` fold into the profile-on-face gesture as an optional "trace" pick. The app's `trace_tool` test is re-spelled.
- The chart is `PoseDef::FaceChart`, and `Recording::draw_on_face` already passes `DrawnOn.faces` through (closed `a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face`). The chart's `z` is read by `stated::frame_z` as no axis today: `FaceChart` falls to its `_` arm.
