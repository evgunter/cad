---
id: a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets
kind: issue
title: a drag field renders a length at a precision its drag speed sets, and can show a length as zero
status: open
opened: 2026-09-12
refs: [fixed-precision-length-renders-can-read-as-a-value-they-cannot-be, parameter-row-field-has-no-text-door]
---



(VIEW) Found by the re-derivation for
`fixed-precision-length-renders-can-read-as-a-value-they-cannot-be`,
as **the member that item's sweep could not match**. That sweep ranged
over a precision spec in a format string (`{…:.N}`) and stated its own
blind spots — no spec at all, a variable precision, a hand rounding.
The property is *a length the chrome renders at a precision fixed
independently of the length*, and there is a fifth way to write one that
is none of those three: hand the number to a widget that picks the
precision itself.

## What the widget does

`egui::DragValue` with no `max_decimals` derives its precision from its
DRAG SPEED. `drag_value.rs:476-480` (egui 0.36.1):

```
let auto_decimals = (aim_rad / speed.abs()).log10().ceil().clamp(0.0, 15.0) as usize;
let max_decimals  = max_decimals.unwrap_or(auto_decimals + 2).at_least(min_decimals);
```

`aim_rad` is `physical_pixel_size()` — `1.0 / pixels_per_point`
(`input_state/mod.rs:804-807`). The text is then
`format_with_decimals_in_range(value, auto_decimals..=max_decimals)`
(`drag_value.rs:531-535`), which is the same SHAPE as this crate's own
render — shortest decimals in a range that reads back — with one
difference that is the whole defect: **when nothing in the range reads
back it returns the range's maximum anyway**
(`emath-0.36.1/src/lib.rs:231-247` — the loop, then
`format!("{value:.max_decimals$}")` under a comment that says *"show the
full value"*), where `crate::readout::number` changes form instead.

So the precision is a function of the drag speed and the display
scaling, and of nothing about the value.

## The arithmetic, for a length field

`forms::FIELD_DRAG_SPEED` is `0.0005` metres — half a millimetre
(`crates/viewer/src/forms.rs:364`), and a field shown in millimetres
passes `props::in_written(speed, unit)`, so `speed = 0.5`
(`crates/viewer/src/widgets.rs:275`). At `pixels_per_point = 1`,
`auto_decimals = ceil(log10(1 / 0.5)) = 1` and `max_decimals = 3`: the
field's coarsest spelling is `{:.3}` over millimetres, which is the
exact spec `delta-field-renders-a-sub-micrometre-delta-as-zero` was
filed against. At `pixels_per_point = 2` it is `{:.2}`. A slot holding
40 nm reads `0.00`, and a slot holding 1.6 µm reads `0.002`.

## The sites

Every `DragValue` in the crate over a value that is a LENGTH. The
census is `grep -rn 'DragValue::new' crates/viewer/src` — eleven hits —
then reading each for what its value denotes, because the pattern finds
the TYPE and the property is the DIMENSION, and nothing in the call says
which. Blind spot of the grep: a `DragValue` built through a helper that
wraps it, of which this crate has none today.

Members, all in production chrome:

- `crates/viewer/src/widgets.rs:275` — `named_field`, the creation
  forms' unit-bearing field, and the one whose speed is converted into
  the display unit, so the arithmetic above is exactly its.
- `crates/viewer/src/widgets.rs:207-209` — `vec3_row_ops`, and
  `crates/viewer/src/widgets.rs:226` — `vec3_row`. Both take the speed
  from the caller, which is `forms::FIELD_DRAG_SPEED` wherever the
  vector is a length (a datum origin, a placement translation) and
  `UNIT_DRAG_SPEED` where it is a direction. The call site decides
  membership, which is why neither function can be read as one.
- `crates/viewer/src/pane/properties.rs:80` — a parameter row's value,
  at `field.tick`, which is `forms::drag_tick(dimension)` and is
  `FIELD_DRAG_SPEED` for a `Length`.
- `crates/viewer/src/pane/properties.rs:550` — a slot row's value. It
  carries a `custom_parser` and no `custom_formatter`, so the authoring
  half is this crate's and the display half is still egui's.
- `crates/viewer/src/pane/properties.rs:196` — a new parameter's value,
  at the same tick through `FieldWriting`.

Not members, and each for its own reason rather than by a shared rule:

- `crates/viewer/src/widgets.rs:293` — `named_scalar`, dimensionless by
  its own doc.
- `crates/viewer/src/pane/create.rs:1007` — a pattern COUNT at
  `COUNT_DRAG_SPEED`, whole by construction and `range`-clamped.
- `crates/viewer/src/widgets.rs:769` — a length in millimetres at speed
  `0.5`, and it WOULD be a member but for sitting inside
  `#[cfg(test)] mod tests` (`crates/viewer/src/widgets.rs:645-646`): a
  gesture harness, not chrome a user reads.

An angle field is not a member for the reason
`fixed-precision-length-renders-can-read-as-a-value-they-cannot-be`
gives at the camera readout: `0.0` is an angle a thing really has.

## Why it was not taken with the four

Three reasons, and the third is the one that decides it.

1. It is a different mechanism. The four were format strings this
   crate writes; this is a precision a library derives, and the repair
   is `DragValue::custom_formatter` rather than an edit to a sentence.
2. It is an EDITABLE field, not a readout. `DragValue` writes back only
   on a real edit (`drag_value.rs:520-527` rounds only when
   `change != 0.0`), so a field reading `0.00` does not commit `0.00` —
   the value shown is wrong, the value held is not. That is a weaker
   defect than the badge's and a different argument.
3. **A custom formatter is one decision about every numeric field in
   the chrome**, not five patches: it would have to answer what a drag
   gesture means when the rendered text no longer matches the tick the
   drag steps by, which is a question about the gesture and not about
   the render. `crate::readout::number` is the rule to hand it; what it
   is not is the answer to that question.

Related but not this: `work/chrome/parameter-row-field-has-no-text-door`
is about the same widget lacking a PARSER, which is the authoring half.
This row is the display half and neither subsumes the other.
