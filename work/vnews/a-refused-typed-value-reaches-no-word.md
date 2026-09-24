---
id: a-refused-typed-value-reaches-no-word
kind: issue
title: A typed value the dimension cannot carry is dropped with no news, where the drag path gets a named refusal
status: open
opened: 2026-09-21
priority: P2
cost: E
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
