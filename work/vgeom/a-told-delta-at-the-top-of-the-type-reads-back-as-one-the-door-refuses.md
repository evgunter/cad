---
id: a-told-delta-at-the-top-of-the-type-reads-back-as-one-the-door-refuses
kind: issue
title: A told δ at the top of the type reads back as a δ the door refuses
status: open
opened: 2026-11-25
priority: P3
cost: E
design: true
---


Found by `vgeom/field-seed` while repairing
`crates/viewer/tests/display_budget.rs`,
`no_delta_renders_as_a_number_a_delta_cannot_be`.

`viewer::scene::DisplayTolerance::render_in` tells δ as a computed
length (`readout::computed_number`), on the nearest point of the told
grid. At the coarsest δ the door accepts (`f64::MAX * 1e-3` m,
`COARSEST_DELTA` in that file) the grid is a decade step above the
floats' gap, and the nearest point, `1.797693134862316e308` mm, reads
back through the millimetre text door (`quantity::UnitDef::canonical`)
as the float above δ, whose millimetres overflow: a δ
`DisplayTolerance::new` refuses. The row asserts the case
(`reads_back_near` and the `is_err` sentinel beside it) instead of the
door-acceptance half it asserts everywhere else.

It was red on main before this was found, behind the subnormal end of
the same row, which failed first: the test read the render as a
millimetre float, which cannot hold this number at all.

Since `vgeom/field-seed` the δ field no longer shows this render: its
text is δ's exact spelling (`DisplayTolerance::field_text_in`), so
what is left is the sentences that quote δ (`scene.rs`'s budget
sentence, `frame.rs`'s δ badge). The question is the role a δ is told
in. If a δ sentence invites typing the number back, it is the
*Offered* role in `crates/geom-core/README.md`, "A number told to a
person": the shortest spelling on the computed grid on the side that
works, which here is down. Otherwise the row's claim, "no δ renders as
a number a δ cannot be", is stronger than the rule and should say so.

