---
id: a-copy-placed-from-a-previewed-copy-is-not-carried
kind: issue
title: The placement editor's preview does not carry a copy placed from a copy it moves
status: closed
pr: 6938
branch: placing/p3p4-cleanup
opened: 2026-11-14
priority: P3
cost: M
closed: 2026-11-14
---


Found by `placing/editor-preview` (PR 6916, the review's MINOR-4).

The placement editor draws its waiting draft's copies where the door's
scratch solve stands them (`ViewerApp::placement_preview`,
`crates/viewer/src/app.rs`; `display::PreviewedPlacement`). It carries
each placement whose *pose* the solve changes: `Stood::copies`
(`DocSession::bundle_judged`, `crates/viewer/src/session.rs`) is each
placement's space and relative pose, and a copy is carried by
`stands · drawn⁻¹` (`PreviewedCopy::carry`, `crates/viewer/src/display.rs`).

A placement whose *source* is a copy the draft moves keeps its relative
pose, so it is not carried, yet its body is drawn as its pose composed
over its source's frame (`wire_place`, `Framed::moved`), and it moves
when the change lands. Until then the preview draws it where it stood.
The README passage *The placement editor commits a placement's whole
bundle* says so ("A copy placed from a previewed copy is drawn where it
stands now").

**Owed.** Carry a copy by the change of its whole frame in its space,
not of its relative pose alone: the scratch solve's composed frame of
each drawn copy against the landed one. **Done when** a row holds a
waiting slide on a copy another placement places from, and reads the
second copy drawn moved with it.
