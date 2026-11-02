---
id: on-the-picked-face-draws-the-profile-there
kind: unit
title: The add-profile form's "on the picked face" draws the profile on the face, and SketchOnFace retires
status: open
opened: 2026-11-02
priority: P1
cost: M
blocked_on: [a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face, doc-sketch-on-face-retires-for-a-profile-drawn-on-the-face]
parent: on-the-picked-face-makes-a-profile-and-extrudes-it-in-one-checkbox
---


Cut by the VSEAM orchestrator (2026-11-02) from
`on-the-picked-face-makes-a-profile-and-extrudes-it-in-one-checkbox`.
That row's dated "ruled" section is the spec (Ev, `[ev]` PR 6298).

Blocked on `work/doctail/a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face`
and `work/lib/doc-sketch-on-face-retires-for-a-profile-drawn-on-the-face`.

## The build

- **"On the picked face" draws the profile there, nothing more.** One
  profile node on the face's frame, through the editor-core door. The
  extrude and the combine are the ordinary Extrude and Combine forms
  that follow. The form's distance field and its spin field go. Spin
  stays a slot on the frame, editable like any other.
- **The click must carry a point.** The x cell is the corner nearest
  where the person clicked, so the viewport pick has to carry its hit
  point to the form. The form names the corner it read.
- **The Extrude form takes a signed depth**, and its label says which
  side is which.
- **Retire** `SessionOp::SketchOnFace`, `Drafts::sketch_on_face` and
  `session.rs`'s `sketch_on_face`.
- **Then delete the fused act from editor-core**: `edit.rs`'s
  `sketch_on_face`, `Recording::sketch_on_face`, `SketchedOnFace` and the
  `lib.rs` re-exports. The lib unit has removed the other caller by
  then. This deletion crosses into doctail's ground, and the VSEAM
  orchestrator authorises it as deletions only.
- Close the parent row with this unit.
