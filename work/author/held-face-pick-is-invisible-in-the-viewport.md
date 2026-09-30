---
id: held-face-pick-is-invisible-in-the-viewport
kind: issue
title: The add-datum form's held face pick is drawn nowhere, so an author can commit against a face the viewport is not showing
status: review
opened: 2026-09-21
priority: P1
cost: D
refs: [face-pick-cannot-name-which-face, 2955]
branch: author/held-face-mark
pr: 3556
---

## What

`add_datum_ui`'s frame-on-face kind LATCHES the viewport's face pick
(`crates/viewer/src/pane/create.rs`, `datum_face_frame_rows`): the
selection is written into `Drafts::datum_face` and **nothing but a
second face pick clears it**. So the form keeps its pick after the
selection is cleared, and an author can press Add datum against a face
nothing in the viewport is showing. AUTH-1's reviewer filed this as
the second half of NOTE-6.

## The latch is right; the silence is not

The latch is deliberate and its reason is written where it is done: it
is what lets an author pick a face, type a spin, open the unit picker
and click the feature tree without losing the pick. Dropping it would
trade this defect for a worse one.

What is missing is that **a held pick is not drawn**. The viewport
marks the live selection and knows nothing about a form's latched one,
so the only evidence a pick exists at all is a line of form text — and
that line cannot say WHICH face either (`face-pick-cannot-name-which-
face`, the naming half of the same reviewer note).

The fix is therefore in the marks, not in the form's wording: a held
seat wants a mark of its own, distinguishable from the live selection,
that goes away when the seat is released.

## Why it is its own file

Split out of `face-pick-cannot-name-which-face` (2026-09-21) because
the two are one reviewer note and not one defect. That row is about
what a face is CALLED, its candidate answers are a names-layer
descriptor or a facade re-export, and its ground is `create.rs` and
`editor-core`. This one is about what is DRAWN, its answer is a mark,
and its ground is the viewport and `marks.rs`. One file, one item —
bundling them under a title about naming would have hidden this one
from anybody reading the slate for viewport work.

Dispatched 2026-09-30 as **AUTH-10** (`docs/AUTH-10-SPEC.md`, branch `author/held-face-mark`). Checked first: `marks::Highlight` carries one selected patch id, `marks::focus` already marks a set, and the viewport marks the blend tool's held edges but not the mate tool's held faces. That makes two unmarked held face picks, and both are in scope.

## Built (AUTH-10, branch `author/held-face-mark`, PR 3556)

**The census.** Face picks held across frames: the add-datum form's
`Drafts::datum_face` and the mate tool's `MateToolState`. Edge picks:
the blend tool's set. Node seats: seven seated tools over
`seats::Seats`, drawn nowhere — `a-seated-tools-held-node-is-drawn-nowhere`.

**The mark.** A held pick wears the selection's colour (`Theme::held`)
told apart by shape: a held face in diagonal stripes, a held edge as a
hollow line (`EdgeLane::Held`). Its meaning and its precedence
(selected over hovered over held) are stated on `marks::Held`. The
shader translates to GLSL ES 3.00 at every entry point, which
`gpu::tests::every_entry_point_translates_to_glsl_es_300` holds.

**One home.** `pane::viewport::frame_marks` gathers every holder
(`Drafts::held_face`, `MateToolState::picks`, `BlendTool::held_edges`)
and is the only door that mints `Composed`, the one value the renderer
takes; `marks::compose` composes it. Whether a held face is in the
picture is `marks::drawn_patch`'s answer — its own (node, body), on a
root the display does not hide — and both the mark and the add-datum
button's gate (`session::face_frame_seat_drawn`, refusing
`FaceFrameFault::NotDrawn`) read it, so the button never commits
against a face nothing marks. Switching the form's kind releases the
pick; a document replacement drops it (`Drafts::document_replaced`).

**Not here:** a held face stays marked while the Add-feature section is
collapsed (`a-held-face-stays-marked-while-its-form-is-collapsed`).
