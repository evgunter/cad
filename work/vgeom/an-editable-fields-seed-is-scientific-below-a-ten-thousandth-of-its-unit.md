---
id: an-editable-fields-seed-is-scientific-below-a-ten-thousandth-of-its-unit
kind: issue
title: An editable field holding less than 1e-4 of its unit is seeded with scientific text: is a field's seed the exact positional spelling or the told one
status: closed
opened: 2026-11-20
priority: P1
cost: M
closed: 2026-11-25
pr: 7775
---


**A regression a user meets in the default notation**, from PR 7229
(the SCALAR notation unit), which moved the viewer's readout onto
`geom_core::Told`. A told number is spelled in the notation
`geom_core::Readable` picks by magnitude: positional from `1e-4` up
to `1e16`, scientific outside. The retired `readout::number` preferred
a decimal wherever one read back.

Every field that goes through `widgets::number_text`
(`crates/viewer/src/widgets.rs`, the formatter `number_field` installs
and `install_number_formatter` sets as the context default) now shows
scientific text whenever egui's own spelling does not name the value
and the value is below 1e-4 of the field's unit, and **that text is
the field's keyboard seed**: an `egui::DragValue` starts an edit from
the text it showed. `4.0e-5` in a millimetre field reads `4e-5`
(`widgets::field_tests::a_value_the_range_cannot_name_gets_a_text_that_names_it`),
and a digit typed at its end lands in the exponent. In the default
notation (metres) that is every length under a tenth of a millimetre.

The fields, by call site of `number_field` (13 in production code:
`widgets.rs` 9, `pane/create.rs` 3, `pane/properties.rs` 1; the kinds
were not each traced): the Properties pane's value fields (length,
angle and scalar slots and vector components), the creation forms'
fields and trace rows, the free-move fields, and count fields (which
are integral and exempt by construction). The View pane's δ field is
a text edit seeded from `DisplayTolerance::render_in`
(`crates/viewer/src/pane/view.rs`) and has the same seed: the default
δ of 0.05 mm seeds `5e-5` in metres, and
`pane::view::tests::the_field_reads_and_commits_in_the_working_unit`
was moved to a δ of 0.5 m because of it.

## The question

Is an editable field's seed the **exact positional spelling** or the
**told** one? `crates/geom-core/README.md`, "A number told to a
person", says an editable field's text is not told text and is
spelled exactly. The field door has rendered on the readout's grid
since before PR 7229 (a field's own render commits nothing:
`props::echoed`), so the told spelling reached the seed when the grid
moved. The candidates: the exact shortest number that types back
(`quantity::UnitDef::authored`), positional at every magnitude a
field can hold; the told number with a positional spelling for
fields only; or the told number as it is now.

Not changed in PR 7229: the question is this program's.
`the-field-door-tells-a-written-number-with-no-unit-in-hand` is the
neighbouring row (the field door's dimension).

## Decided

Ev, PR 7261 (2026-11-21): an editable field over a written value shows its exact positional spelling in the unit it was written in, never exponent form (`crates/geom-core/README.md`, "A number told to a person"). The row is now a build, not a question.

## Closed (2026-11-25)

PR 7775: every editable field (Properties slots and variables, creation forms and trace rows, free-move boxes, counts, the mate value and offset, the δ field) seeds its value's exact shortest spelling in its own unit, positionally (`props::FieldNumber`, `quantity::Decimal::positional`), and commits through `FieldNumber::canonical_of`. It types back through the text door; the float door's residue is `told/a-float-door-loses-what-the-text-door-keeps`. A formula's source is the open question `whether-a-formulas-source-spells-its-numbers-positionally`.
