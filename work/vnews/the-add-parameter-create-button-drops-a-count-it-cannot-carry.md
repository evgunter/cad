---
id: the-add-parameter-create-button-drops-a-count-it-cannot-carry
kind: issue
title: The add-parameter form's Create button drops a Count draft SlotValue::of refuses, with no word
status: open
opened: 2026-09-25
refs: [a-refused-typed-value-reaches-no-word, a-disabled-control-says-why-in-four-shapes]
priority: P3
cost: E
---

## Finding

Found by `a-refused-typed-value-reaches-no-word`'s sweep for a
`props::SlotValue::of` refusal the chrome drops.

`crates/viewer/src/pane/properties.rs`, the add-parameter form in
`ViewerBehavior::properties_ui`'s parameter section (the
`create.clicked() && … && let Ok(value) = SlotValue::of(dimension,
self.drafts.new_param_value)` chain). The draft field is a
`widgets::number_field`, whose parser reads `inf` and `NaN` as numbers
(`props::field_edit`), so a `Count` draft can be handed a value the
dimension cannot carry (whether `egui`'s `DragValue` stores it
unclamped has not been driven by a row). The Create button is live
over it, and a
click on it runs the `SlotValue::of` door, gets
`DimensionError::NonFiniteLiteral`, and does nothing: no operation, no
notice, the draft left standing.

## Not established here

The shape differs from its sibling's. There the value arrives by an
act already done (a keyboard edit committed), so the refusal is the
frame's news. Here the refusal is knowable BEFORE the click, which is
the shape `undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words`
closed: the button reads its refusal from the value and shows its words
(`app::refusable_button`). The existing "pick a dimension first" hover
text is a second reason the same button carries, so the fix has to
decide how the two compose.


