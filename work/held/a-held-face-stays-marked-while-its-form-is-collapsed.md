---
id: a-held-face-stays-marked-while-its-form-is-collapsed
kind: issue
title: A held face stays marked while the Add-feature section is collapsed, and the latch stops following the selection
status: open
opened: 2026-09-30
priority: P3
cost: M
rides_with: a-seated-tools-held-node-is-drawn-nowhere
---


Found by AUTH-10's correctness review (F3's second half), filed rather
than fixed there.

## What

The add-datum form latches the viewport's face pick in its
frame-on-face rows (`ViewerBehavior::datum_face_frame_rows`,
`crates/viewer/src/pane/create.rs`), and those rows run only while the
Properties pane's "Add feature" section is OPEN. `Drafts::held_face`
answers the latch whenever the form's KIND is frame-on-face, whether or
not the section is open, and the viewport marks what it answers
(`pane::viewport::frame_marks`).

So with the section collapsed:

- the held face stays striped in the viewport, although no form on
  screen says it is holding anything;
- a new face pick no longer moves the latch — the rows that write it
  are not drawn — so the stripes stay on the OLD face while the
  selection marks the new one.

## Why it is a design question

The section's open state is egui's collapsing-header memory, not a
value the drafts or the viewport can read, so "held" cannot simply be
gated on it. The candidates are: the latch writes outside the rows
(which changes what "the form's picker" means), the section's open
state becomes a value the frame owns, or a collapsed form releases its
pick. Each moves the latch's contract, which `Drafts::datum_face`'s
doc states.

## Decided (2026-10-25)

Decided together with `a-seated-tools-held-node-is-drawn-nowhere`, which carries the design. This row rides with it. The body above cites the add-datum form (`datum_face_frame_rows`, `Drafts::datum_face`), which is gone. Its successor is the add-profile form's on-face latch: `Drafts::sketch_face`, written by `sketch_face_rows`. That latch becomes `ToolKind::SketchOnFace`, and the open tool's panel is drawn outside the collapsing sections, which closes both symptoms.

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.

## Unparked (2026-11-01)

Its trigger, D′3 (PR 5636), merged at `29110056`. A PR number in `blocked_on` is not checked by lint, so the row read as parked on a trigger that had already fired. Re-read it against D′3's placements and tools before dispatch. (DOORS orchestrator)

## Moved (2026-11-01)

Cut from DOORS into HELD (DOORS over budget at 52 of 30 once D′3's triggers fired; `work/doors/log.md`, 2026-11-01). Id and body unchanged. (DOORS orchestrator)
