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

## 2026-11-02 — the add-profile form is gone; New profile is where this lands

Evidence from FORMS' profile unit (branch `forms/profile-unit`, which
closes `work/forms/the-profile-create-form-is-a-second-profile-editor`).
The add-profile form, its "on the picked face" checkbox and
`Drafts::sketch_on_face`, `sketch_face`, `sketch_depth` and
`sketch_spin` are deleted. The creation section's one profile door is
now a **New profile** button (`ViewerBehavior::new_profile_ui`,
`crates/viewer/src/pane/create.rs`), which inserts a profile with no
loops and selects it, so the profile editor opens on it
(`crates/viewer/README.md`, "A create gesture commits the node":
"New profile inserts a profile with no loops, on a picked face's frame
when a face is picked").

No door on main inserts a profile on a face's frame yet: the
editor-core half
(`work/doctail/a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face`)
is open, and `SessionOp::SketchOnFace` extrudes and fuses, which the
ruling retires. So the button inserts on the profile's own plane
whatever is picked, and the GUI has no sketch-on-face gesture until
this unit lands. What this unit's build becomes:

- **"On the picked face"** is no checkbox now. With a face picked, New
  profile inserts the empty profile on that face's frame through the
  editor-core door, and the editor fills it in. Its picks are the
  face and the click's point (the x cell); spin is the frame's slot.
- `SessionOp::SketchOnFace` and `session.rs`'s `sketch_on_face` are
  still on main, with no GUI caller; their retirement is unchanged.
  The viewer's held-face rows that used the checkbox now take the
  traced-profile form's plane latch, and perform `SketchOnFace`
  directly where they need a takeover (`app.rs`, `holding_the_top_cap`
  and `sketch_on`); they move to whatever takeover remains once it
  retires.
