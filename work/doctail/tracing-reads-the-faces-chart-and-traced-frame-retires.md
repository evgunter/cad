---
id: tracing-reads-the-faces-chart-and-traced-frame-retires
kind: unit
title: A traced profile draws on the face's chart like any sketch on a face; traced_frame and the New-traced-profile act retire, and the coincidence door proves the tangency by rung 3
status: closed
opened: 2026-11-18
priority: P1
cost: H
refs: [the-coincidence-doors-construction-facts-have-no-one-source]
closed: 2026-11-18
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

## Closed

Closed by the DOCTAIL unit's PR (2026-11-18).

- **Rung 3 proves the heading; no stated fact was added.** Where the first rung leaves a traced sweep at its heading, `coincide::swept_along` (in the replay) reads the traced face as the profile reads it at `Sym`. It takes the face's cylinder axis into the sketch's coordinates, and discharges that axis's two components across the sketch's `z` as polynomial identities. Every chart row proves this way (`Rung::MarginIdentity`). Each one costs 2 to 5 ms of the door in a debug build, on DAGs of 107 to 613 nodes over at most 4 symbols, all within the door's budget. That includes a cylinder tilted by a free angle, whose chart turns by the least turn. A frame spun by a variable about its own `x` stays unproven (`Err(Otherwise)`), as it should.
- **The review fix pass.**
  - **The lever.** Each sine is levered at the decided pair's reach (`carrier_pair_arm`), entered as the power of two at or above it. Levered at the reach's own symbolic form, the mated and tilted rows ran `OverTheBudget`. Levered at an opaque re-entry, they ran `KeepsAnOpaqueAtom`.
  - **Rows added:** a wall parallel to the face only by value; a box mated flat onto the cap, tilted; a boss traced on a hole drilled through the same chart; a missing member; and a frame turned by the float `π/2`.
  - **The float `π/2` row.** It asks for the axis, and a frame read off the wall's axis proves it. A quarter turn written as a fraction of a turn ran over the budget, so the recourse does not offer one.
  - **The viewer.** The trace pick now lets go on commit and on Cancel, and has its own "trace nothing" button. Its branch rows show only while a face to trace is latched.
  - **Smaller changes.** `chart_on_face` is the one defaulting home, `nearest_to_face` fails loud (`FaceUnread`), and `traced_faces` binds counts before definitions.
- **Retired:** `traced_frame`, `TracedFrameFault`, `TRACED_FRAME_SQUARE`, `mate::corner::read_of_member` with `PinRefusal::{UnwrittenIndex, NoMember}` (they had no other reader), the pncad re-export, and the py `Evaluation.traced_frame` binding, its stub and its test.
- **Viewer:** the traced-profile form is now the profile-on-a-face form (`New profile on a face…`, `SessionOp::DrawOnFace`, `Drafts::on_face_op`). The trace is an optional pick, and without it the form inserts an empty profile on the face's chart. The trace it shows is read on the chart (`face_chart`, `nearest_to_face`, now public beside `draw_on_face`).
- **Rows that moved:** `a_frame_read_off_a_rim_*` and C's union row flip to proven. The spin row is re-spelled with a variable angle: about the rim's axis it proves, about its own `x` it asks. The traced-frame rows are re-spelled on the chart as `intent_s6_e_traced_on_a_face`, where a bottom-cap `+` depth stands outward and its `−` depth runs up the wall.
