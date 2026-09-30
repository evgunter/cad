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

## Built (AUTH-10, branch `author/held-face-mark`)

**The census.** Face picks held across frames: the add-datum form's
`Drafts::datum_face` and the mate tool's `MateToolState` — neither was
drawn. Edge picks: the blend tool's set, drawn in the SELECTED edge
lane. Node seats: seven seated tools over `seats::Seats`, drawn nowhere
— filed as `a-seated-tools-held-node-is-drawn-nowhere`, not built here.

**The mark.** A held pick wears the selected mark's colour, told from
the live selection by shape: a held face in diagonal stripes of it, a
held edge as a hollow line of it. It means "a choice a form or tool is
holding, which is not the live selection"; the meaning is on
`marks::Held`, the colour note on `Theme::selected`. No palette
entry is added. The held lanes rank below hover and selection.

**One home.** `pane::viewport::frame_marks` is the one place the
holders are gathered (`Drafts::held_face`, the mate tool's
`MateToolState::picks`, the blend tool), and `marks::compose` the one
composition; the blend block moved into it and into its own
`EdgeLane::Held`. The form's Add button gate reads the same
`held_face`, so it commits against exactly what is marked. Switching
the form's kind releases the pick (the latch is kept, not cleared).
