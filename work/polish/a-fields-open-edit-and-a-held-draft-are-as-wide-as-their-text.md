---
id: a-fields-open-edit-and-a-held-draft-are-as-wide-as-their-text
kind: issue
title: viewer: a value field's open keyboard edit, and the draft a refused expression leaves in it, are as wide as their text
status: open
opened: 2026-09-29
priority: P4
cost: M
refs: [a-driven-slots-field-draws-its-expression-source-at-any-width]
---

## Question (answered by Ev, 2026-10-01)

How should a value field that edits expressions be shaped? A driven slot's field at rest is bounded since PR 3478. Two of its states are still as wide as their text: the open keyboard edit (egui's `DragValue` edit grows with its buffer) and a held draft (a refused expression shown in the field). Both run past the pane in a non-wrapping row. The choice is the field's shape: where its open edit is drawn, and where a held draft is said.


Found by `chrome/slot-width`, which bounded a driven slot's field AT
REST: the field shows `= value` (`crate::props::field_text`) and the
source is said under the row (`crate::pane::properties`'s
`slot_notes`). Two states of the same field are still as wide as a
text nothing bounds.

## The open keyboard edit

`egui::DragValue` edits through a `TextEdit::singleline` built with
`.clip_text(false)` and a `desired_width` of one interact size
(`egui-0.36.1/src/widgets/drag_value.rs`, the `is_kb_editing` arm), so
the box grows with its buffer. A driven slot's edit opens on its
whole source (`crate::widgets::value_field_ops`, `seed_edit`), and any
field grows with what is typed into it — so while an edit is open the
slot row (`slot_group_ui`'s `ui.horizontal`) runs past the pane by the
source's width, and a vector's row by up to three.

## The held draft

A refused expression (`frame::retype_draft`) is held in the field it
was typed into (`drafts.expr_target` / `expr_text`) so the user can fix
it, and `crate::pane::properties::slot_showing` shows it as typed: it
is what the next edit opens on. At rest, that field is as wide as the
refused text.

## What a fix has to decide

The field is the toolkit's; bounding the open edit means either a
field of our own for text (a clipped `TextEdit` that scrolls its
content, with the `DragValue`'s gesture beside it), or moving the open
edit out of the row. The draft is the same question at rest: it could
be shown as a bounded mark with the draft seeding the edit
(`seed_edit`, as a driven slot's source does) and said under the row.

## Ev's answer (2026-10-01, on PR 3606 and in chat)

Ev saw screenshot mockups of three states of a vector slot row: today, the edit under the row, and the edit in the row with a capped width. The mockups were throwaway branches `mockup/field-edit-under-row` and `mockup/field-edit-in-row`, built in the real viewer. Ev chose:

> i think edit in the row (capped). the downside is real but it's much more intuitive, now that i see it.

The ruling:
- The chrome owns its value field.
- The open keyboard edit stays **in the row**, in a box capped at `widgets::widest_number`, with the text scrolling inside it.
- A held draft leaves the field and is said under the row, as both designers recommended.

The downside Ev accepted: at the default pane width, the capped box still pushes a vector row's unit picker past the pane edge.

**Priority.** Ev also ruled that this row is *polish*: all the information is already available while scrolling works, only displayed less well. Polish rows are no longer dispatched from CHROME, and this row moves to the P4 polish program with the ruling above as its specification.
