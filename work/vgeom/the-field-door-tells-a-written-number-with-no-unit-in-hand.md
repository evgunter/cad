---
id: the-field-door-tells-a-written-number-with-no-unit-in-hand
kind: issue
title: The field door tells a written number with no unit in hand, so its grid is a tenth of ε of the written number
status: open
opened: 2026-11-20
priority: P3
cost: M
---


`viewer::widgets::number_text` (`crates/viewer/src/widgets.rs`) is
handed the `f64` an `egui::DragValue` is bound to: the canonical value
already divided by its unit's factor, with no unit and no dimension.
Since the readout moved onto `geom_core::Told`
(`crates/viewer/src/readout.rs`, `field_number` and `names`), the
door tells that number as a computed length in the canonical unit,
because that reproduces the grid the retired `readout::number` applied
to every field: four figures, and a tenth of ε **of the written
number**.

That is not the rule (`crates/geom-core/README.md`, "A number told to
a person"): ε/10 is a length whatever the unit, so a millimetre field
resolves 1e-10 mm where the rule's grid is 1e-7 mm, and an angle or a
ratio field gets a length's cap where the rule gives four figures. It
is conservative (never coarser than the rule for a length; every
length unit's factor is at most one), and the dimension it claims is
false for every field that is not a length in metres.

The fix is to hand the formatter what the field holds:
`number_field`'s callers know the unit (`props::shown_value` made the
number from it), so the formatter can take the unit, recover the
canonical value by the text door's multiply, and call
`readout::computed_number`. The README also says an editable field is
spelled exactly, not on the grid; whether the field door moves to the
exact spelling (`UnitDef::authored`, the shortest number that types
back) is the same question and this row's to settle.
`install_number_formatter`'s context default has no unit at all
(`a-bare-field-still-commits-its-own-render`).

Filed by the SCALAR notation unit
(`work/scalar/a-refusal-is-told-in-the-readers-notation.md`).

**The δ field is the same question from the other side.** Its draft
is seeded from `DisplayTolerance::render_in`
(`crates/viewer/src/pane/view.rs`), which is told text, and told text
is scientific below 1e-4 of its unit: the default δ of 0.05 mm seeds
`5e-5` in a metre notation, and a digit typed after it lands in the
exponent (`pane::view::tests::the_field_reads_and_commits_in_the_working_unit`
was moved to a δ whose render is a decimal for that reason). An
editable seed wants the exact, positional spelling.
