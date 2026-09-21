---
id: the-fields-door-has-no-width-bound-at-all
kind: issue
title: number_text has no width bound at all: a large field value is spelled in hundreds of characters
status: dispatched
opened: 2026-09-16
priority: P0
cost: E
branch: vgeom/render-spelling
---


(VIEW) `crates/viewer/src/widgets.rs`'s `number_text` is the fields'
door: `egui::Style::number_formatter` for every `DragValue` and `Slider`
in the chrome (`install_number_formatter`). It keeps the widget's own
spelling wherever `readout::reads_back` accepts it, and falls to
`readout::number` otherwise. **`readout::reads_back` judges truth and
says so** — *"Width is no part of reading back, which is why it is no
part of this"* — so the widget's spelling is taken at any width.

That is unbounded, and it is already asserted. The widget's spelling of
`1.0e9` in the millimetre range is `1000000000.0`, twelve characters
against a `readout::MAX_CHARS` of ten, and
`nothing_at_or_above_one_display_unit_renders_differently` sweeps to
`1.0e9` asserting exactly that text. At the top of `f64` it is far
worse: `emath::format_with_decimals_in_range` returns
`format!("{value:.1}")` on its first iteration (`almost_equal(inf, inf)`
is `a == b`, `emath-0.36.1/src/lib.rs:255-262`), which is the exact
decimal expansion — **311 characters** — and that parses back to the
value, so `number_text` returns it and `readout::number` is never
reached.

**So `MAX_CHARS` is not a crate-wide width guarantee and never was.** It
bounds one function's decimal search; the δ field's `FIELD_WIDTH` is the
only widget sized against it, and it is fed by `render_mm`, not by this
door. The chrome's parameter field also has no `.range()` —
`pane/properties.rs`'s three `number_field` sites are unranged, and only
`pane/create.rs`'s integer pattern count carries one — so the value
reaching this door is whatever a user typed.

**What a repair has to decide** is whether a *field* owes a width bound
when a *label* does not, which is the question
`the-scientific-arm-rounds-out-of-the-type` answered for `readout::number`
and left open here. Note the asymmetry that unit turned on: a clipped
draft still holds and commits the whole text, so width in a field costs
legibility rather than correctness — which is an argument for leaving
this alone and stating it, not only for fixing it. What is not
defensible is the current silence: `number_text`'s doc argues at length
about truth and says nothing about width, and the row over it asserts a
twelve-character text without remarking on it.

Not a regression: pre-dates the third arm and is untouched by it.
