---
id: a-refused-typed-value-reaches-no-word
kind: issue
title: A typed value the dimension cannot carry is dropped with no news, where the drag path gets a named refusal
status: closed
opened: 2026-09-21
priority: P2
cost: E
closed: 2026-09-25
---


## Finding

Filed by VGEOM's refusal-floor sweep, which created the case: closing
`work/vgeom/a-count-slot-launders-a-typed-nan-into-zero.md` made
`props::SlotValue::of` refuse a value the dimension cannot carry, and
a refusal at that door now has **two destinations and only one word**.

`crates/viewer/src/widgets.rs`, `value_field_ops`' typed arm (the
`Some(props::FieldEdit::Number(written))` match arm). The arm builds
the `SlotValue` and pushes the vocabulary's number door; when the
value is refused there is no `SessionOp` to push, so the frame emits
nothing at all and the field simply re-renders what the document
holds.

The DRAG half of the same value has a word already:
`crates/viewer/src/session.rs`'s `preview_gesture` maps
`GestureTarget::value_of`'s refusal through `Refusal::Dimension`, and
that reaches the status line like any other typed refusal. So the same
bad value, on the same field, is a named refusal when it arrives by
drag and silence when it arrives by keyboard.

**This is better than what it replaced** — the value used to commit as
an ordinary integer — and it is not what the module promises.
`props::field_edit`'s doc says a non-finite spelling reads as a Number
on purpose *because* the refusal downstream names the problem; for the
typed path into a `Count` slot the refusal now happens and the naming
does not.

## Not established here

Whether the right answer is a news item, a badge, or the field
rendering its own refusal. `undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words`
is the nearest sibling on this slate — a control refusing in silence
over a refusal that has words — and the two may want one answer.

## Fence

`crates/viewer/src/widgets.rs` and `crates/viewer/src/session/refuse.rs`
— the wording is this program's; the value's own door
(`crates/viewer/src/props.rs`) is VGEOM's and is already closed.

## Done

**The door: the frame's notices, through one refusal-to-line function.**
`crate::frame::refusal_message(&Refusal) -> Message` is now the one
place a `Refusal` becomes a line message; `frame::batch_status` (the
drag route, and every other operation's refusal) calls it, and
`widgets::value_field_ops`' typed arm pushes
`refusal_message(&Refusal::Dimension(error))` onto the frame's notices
when `props::SlotValue::of` refuses. The value is the one the drag
route's `preview_gesture` builds (`Refusal::Dimension` over the same
`SlotValue::of`), and the words are its `Display`, so the two routes
render one sentence composed once.

**Why the notices rather than an operation that refuses.** No
operation can carry the value: `SessionOp::SetSlot` and `SetParam`
take a `SlotValue`, which is exactly what failed to exist, and
widening them to a raw `f64` would move the typing of every keyboard
edit into VSEAM's session and loosen a payload VGEOM typed on purpose.
A new refusing op would be a new door. The notices list is the existing
door for a refusal the chrome meets on this frame with no operation
under it (`pane::view`'s δ field is the precedent), and the properties
pane already holds it (`ViewerBehavior::notices`). The one difference
from the drag route is rank: a notice ranks below a batch refusal in
`frame::frame_status`, so a frame that also carried a refused
operation would show that one. No such frame arises from one field's
keyboard edit.

**The undo/redo answer does not fit here.** #2960 had Undo and Redo
read their refusal from the value before the act and show it on the
disabled control. A text field has no "before": the refusal is only
knowable once the text is committed, and by then the act is done, so
the answer is news about the act, which is what the drag route already
gives it. The Create button of the add-parameter form has the #2960
shape, and is filed as
`work/vnews/the-add-parameter-create-button-drops-a-count-it-cannot-carry.md`.

**The row.** `widgets::value_field_tests::a_count_the_dimension_refuses_is_said_typed_as_dragged`
types `inf` into a pattern's `Count` slot and asserts: no operation and
no history step; one notice; the frame's line is the fixed text
`a literal value must be finite` about `Subject::Document`; the drag
route's `PreviewGesture` refusal is exactly
`Refusal::Dimension(DimensionError::NonFiniteLiteral)`; and the two
routes' `frame_status` are equal.

