---
id: on-the-picked-face-makes-a-profile-and-extrudes-it-in-one-checkbox
kind: issue
title: The add-profile form's "on the picked face" checkbox makes a profile and extrudes it, as one op
status: open
opened: 2026-11-02
priority: P1
cost: M
design: true
---

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
