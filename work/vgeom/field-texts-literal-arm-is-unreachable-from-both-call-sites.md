---
id: field-texts-literal-arm-is-unreachable-from-both-call-sites
kind: issue
title: props::field_text's literal arm is unreachable from both call sites, and its prose is the only statement of what a literal field shows
status: open
opened: 2026-09-22
---


Found by `vgeom/field-product` while moving the no-reading refusal
above the widget for
`a-field-bound-to-a-written-value-shows-a-product-that-overflowed`.

## The finding

`crate::props::field_text` matches on `(&row.driver, &row.value)` and
its first arm is `(SlotDriver::Literal, Ok(value))` — the arm that
asks `written` and answers `no_reading` where the notation cannot name
the value. **Neither caller can reach it.** Both call `field_text`
only where the row shows SOURCE rather than a number:

- `crates/viewer/src/pane/properties.rs`, `slot_value_ui`'s `fixed`:
  `else if row.driver.is_driven() || row.value.is_err()`;
- `crates/viewer/src/widgets.rs`, `value_field_tests::Row::field`,
  which replicates that rule.

`SlotDriver::is_driven` is `matches!(self, Self::Expression { .. })`,
so `Literal` is never driven, and `Ok` is never `is_err`. The
conjunction the arm needs is exactly the conjunction both guards
exclude.

## Why it matters rather than being dead code

The arm's doc paragraph is the only prose in the crate that states
what a LITERAL slot's field shows — *"a bare literal therefore shows
its number ALONE, in the unit the row is written in"*, and *"a literal
whose value the notation cannot name shows `no_reading`"*. Neither
sentence describes production. A literal that evaluated shows the
number `egui` formats through `crate::widgets::number_text`, and since
`vgeom/field-product` a literal whose notation cannot name its value
shows no field at all — `crate::widgets::value_field_ops` draws
`props::no_reading` in the field's place. So a reader who wants to
know what a literal field shows is pointed at a function that does not
answer it.

`props::props_tests::a_literal_with_no_millimetre_value_does_not_show_one`
is the row over it, and it hand-builds a `SlotRow` with
`driver: SlotDriver::Literal` and `value: Ok(..)` — a combination no
caller constructs. It is a green row about a branch nothing runs.

## The fork

- **Delete the arm** and let `field_text` be what its callers use it
  for: *the text a row shows INSTEAD of its number*. The `unreachable!`
  in its `None` sub-arm goes with it, and the test moves to the door
  that now answers the question
  (`widgets::value_field_tests::a_field_is_not_drawn_for_a_value_its_notation_cannot_name`).
- **Or widen the callers** so the literal case does route through it,
  which would mean a literal slot's field showed a pinned text rather
  than the number `egui` is dragging — the thing `slot_value_ui`'s own
  comment says freezes the field mid-gesture.

The first looks right and the second is written down only so the
deletion is a choice rather than a default.

## Fence

`crates/viewer/src/props.rs` (VGEOM, CHROME, VIEW, AUTHOR).
