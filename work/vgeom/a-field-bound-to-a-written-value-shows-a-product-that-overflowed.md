---
id: a-field-bound-to-a-written-value-shows-a-product-that-overflowed
kind: issue
title: Two number_field sites bind a unit-converted product, so a field shows inf for a value the document holds
status: open
opened: 2026-09-21
priority: P1
cost: D
refs: [renders-that-multiply-a-finite-guarded-length-spell-the-product-inf, a-bare-field-still-commits-its-own-render]
---


Found by `vgeom/render-spelling`'s re-derivation of
`renders-that-multiply-a-finite-guarded-length-spell-the-product-inf`'s
own sweep. **It is a fourth and fifth member of that row's class, and
the row could not see them because its population rule was applied to
the wrong call sites.**

## What the row's rule produces, and what it was applied to

The rule is *every call to `readout::number`, `widgets::number_text`
and `props::render_number` under `crates/viewer/src`, then read each
argument for whether it is a product or a quotient.* Applied to
`number_text` it concludes — correctly about the function —
*"`widgets::number_text` renders its argument rather than a product of
one, so it is not a member of this class."*

But `number_text`'s call sites are all inside `widgets.rs`: it is the
`custom_formatter` of `widgets::number_field` and the crate-wide
`egui::Style::number_formatter`. **The argument is a caller's, and the
population that answers the rule is the `number_field(` call sites.**
Thirteen of them under `crates/viewer/src`, eleven in production, and
two bind a value formed by dividing a canonical one by a unit factor:

- **`crates/viewer/src/widgets.rs`'s `named_field`** —
  `let mut written = props::in_written(*canonical, unit);` then
  `number_field(&mut written, …)`. Reached by `point_fields` and
  `unit_field`, which are every dimensioned field of every creation
  form.
- **`crates/viewer/src/widgets.rs`'s `value_field_ops`** —
  `number_field(&mut number, writing.tick)` over `FieldShowing.number`,
  which `crates/viewer/src/pane/properties.rs` builds as
  `field.shown(row.value.as_f64())`, and `forms::FieldWriting::shown`
  is `props::shown_in`, which is `in_written`. That is the property
  panel's slot field and its document-parameter field.

The other nine bind a value as held: three vector components
(`vec3_row`, `vec3_row_ops`), `named_scalar`, two counts, a direction
component (`pane/create.rs`), and the new-parameter draft
(`pane/properties.rs`). No conversion at the site, so no product.

## The defect

`props::written` (added by `vgeom/render-spelling`) is the question
these two do not ask: a canonical length above `f64::MAX * MILLI`
(`1.7976931348623156e305` m) has no millimetre value, and a canonical
angle of exactly `5e-324` rad divides by `pi rad`'s π to `0.0`. So a
slot holding `1e306` m, shown in millimetres, puts `inf` in a
`DragValue` — and the field is the text an edit starts from.

**What is NOT wrong here, since `vgeom/p0-fields` landed:** the field
will not commit that text at it. `widgets::number_field` now refuses
text equal to its own render (`props::echoed`), so the round trip that
`a-fields-text-commits-within-the-renders-own-tolerance` is about
cannot fire on the marker or on `inf`. What is wrong is only what the
field SHOWS — which is the render half, and is this class.

## Why it was not fixed where it was found

`props::field_text` and `Bounds::wording` could take the repair
straight, because each composes a `String` and could put
`props::no_reading` in it. A `number_field` cannot: it is
`egui::DragValue::new(&mut Num)` and the formatter is handed the
`f64` the widget holds, with no unit in scope and no way to say *this
field has no value to show* other than not drawing a field. The
choices — draw the row with the value in its CANONICAL notation and
say so, draw a disabled field with the marker beside it, or refuse the
conversion one level up so `FieldShowing`/`named_field` never form it
— are a door decision about what a field IS when its notation cannot
name its value, and `number_field` had just been rewritten by
`vgeom/p0-fields` (#3007).

## Reachability

Two gestures: type a large literal into a slot written in metres, then
move the unit picker to `mm`. Nothing in the chrome's `f64` fields
carries an `egui::DragValue::range`, and `Expr::literal` refuses only
a non-finite value, so the magnitude reaching the conversion is
whatever a user typed.

## Fence

`crates/viewer/src/widgets.rs` (VGEOM, CHROME, VIEW) and
`crates/viewer/src/pane/properties.rs` (VGEOM, VNEWS — a field's VALUE
is VGEOM's).
