---
id: on-the-picked-face-makes-a-profile-and-extrudes-it-in-one-checkbox
kind: issue
title: The add-profile form's "on the picked face" checkbox makes a profile and extrudes it, as one op
status: open
opened: 2026-11-02
priority: P1
cost: H
---

**Reported against an older build** (Ev, in chat, 2026-11-02: "i had
accidentally built a slightly old version of the gui"). Reproduce on
current main before building anything; if main no longer shows it, close
the row saying so.

Reported by Ev (in chat, 2026-11-02): "it's not clear what it was
supposed to do and it ?automatically extruded the profile? which seems
like it can't possibly be a feature that people actually want to do in
one checkbox". Filed by the VSEAM orchestrator.

## What it does

The add-profile form has a checkbox, "on the picked face"
(`crates/viewer/src/pane/create.rs`, `Drafts::sketch_on_face`). Ticked,
it commits `SessionOp::SketchOnFace { face, loops, distance, spin }`
(`crates/viewer/src/session/op.rs`). That op carries an extrusion
`distance`, so one click draws the profile on the face's frame AND
extrudes it from the face. The checkbox's label says neither, and the
form's other shapes add a profile only.

## Why it is this program's

Whether one op is one act is VSEAM's op-vocabulary question (charter,
`work/vseam/plan.md`). Precedent: `session-save-is-two-acts`. Here
"place a profile on a face" and "extrude it" are two acts spelled as
one, and the second is hidden behind a checkbox about the first.

## What deciding it needs (`design`)

Whether a profile can stand on a face's frame by itself (a profile node
placed in the face's space, as `AddDrawnProfile` draws on a frame),
with the extrude its own later step through the ordinary Extrude form.
Or whether "sketch on face" is a kernel or library door that only
exists fused. Read `pncad::trace::traced_frame` and `drawn_profile` for
the frame-drawing precedent, and `SketchOnFace`'s door in `session.rs`
for what the fused op does with `spin`. A design question the
designers weigh before any lane builds.

## 2026-11-02 — ruled (Ev, `[ev]` PR 6298): a sketch on a face is drawn there

This is the build. Spec: `[placement]` and REFERENCES DM1 as merged.
- **The sketch.** A sketch on a face is a profile drawn on a frame read
  off the face (`ProfileProgram.on`, `DrawnOn` with no traced faces), in
  the face's body's space. The frame is `Spin { Through { axis, point },
  about z, by spin }`:
  - z is the face's outward normal;
  - x points toward a cell of the body off the face's normal, as the
    gesture names one;
  - on a face with none (a bare disc), x is the space's x carried by
    the least turn, as a placement's spin is measured;
  - `spin` is a field, the constant 0 unless named.
- **New pose construction.** A `Flag` builder in
  `crates/editor-core/src/pose.rs`.
- **Signed depth.** An extrude's depth is a signed `Length`: + is a
  boss, − is a pocket's tool, 0 refuses.
- **Retire the fused act.** `SessionOp::SketchOnFace`,
  `Drafts::sketch_on_face`, `Recording::sketch_on_face` and pncad-py's
  `Doc.sketch_on_face` / `SketchedOnFace` all go. The checkbox now just
  draws the profile on the face.
- **A body on a face stays a placement** (DM1).

It crosses editor-core, pncad, pncad-py and the viewer. Cut it into
units on those owners before dispatch. Re-priced D → H.
