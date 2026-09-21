---
id: a-typed-field-hands-its-text-over-on-two-frames
kind: issue
title: a DragValue parses its buffered text on two consecutive frames, so one typed number reaches the chrome twice
status: open
opened: 2026-09-21
---


(AUTH-2) Found while re-shaping the property panel's no-op guard;
measured, not inferred.

## The finding

`egui::DragValue` keeps the text of an open keyboard edit in
`ui.data` under the widget's id and parses it in TWO places
(`egui-0.36.1/src/widgets/drag_value.rs`): once inside the
keyboard-editing branch, when the text box reports `lost_focus`, and
again at the top of the next frame, where `mem.lost_focus(id)` is
STILL true and the buffered text is removed and parsed a second time.
`Response::lost_focus()` therefore answers true on both frames, so it
cannot be used to tell them apart either.

A chrome that turns a parse into a session operation emits that
operation twice for one typed number. Measured at the property
panel's parameter row with a real `egui::Context` and a real
`DocSession`: typing `1002` over a field showing `1000.0` emits
`SetParam` on two consecutive frames, with identical payloads.

## What holds it today, and where it does not reach

The two panel value fields are covered, at the document rather than at
the widget: `DocSession::writes_nothing` refuses to submit an edit
that writes what the document already holds, so the second hand-over
costs no undo step. `widgets::value_field_tests::
the_second_hand_over_of_one_typed_text_changes_nothing` is the row,
and it fails loudly if the toolkit ever stops doing this.

Two things that rule does not reach:

- **A door with no "already" to compare.** `writes_nothing` answers
  `false` for every edit that is not a slot literal or a declaration's
  value or notation, which is the conservative direction — the second
  hand-over would cost an undo step rather than losing an edit. A
  future field whose door mints something (an insert) would be
  charged twice.
- **The field's own guard cannot see it.** `props::echoed` compares
  the typed text against the field's render, and by the second frame
  the document has moved and the render is no longer the text the user
  left in the box. So the two rules are genuinely two, and a reader
  who collapses them re-opens this.

## What a fix would answer

Whether the chrome should recognise the repeat AT the field — which
means remembering, per widget id, the text a field last turned into an
operation, and clearing it on `gained_focus` so a deliberate re-type
is still an act. That is state the panel does not otherwise keep, and
it would let the field emit at most one operation per keyboard edit
rather than relying on every door below it being idempotent.
