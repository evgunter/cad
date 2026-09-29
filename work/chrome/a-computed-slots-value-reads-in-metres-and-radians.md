---
id: a-computed-slots-value-reads-in-metres-and-radians
kind: issue
title: viewer: a computed slot's value reads in metres and radians, whatever the document is written in
status: open
opened: 2026-09-29
priority: P3
cost: M
design: true
refs: [a-driven-slots-field-draws-its-expression-source-at-any-width]
---


Found by the review of `chrome/slot-width` (PR 3478).

A driven slot's field shows the value its expression equals
(`crate::props::field_text`, through `crate::props::computed_text`),
and so does the refusal's affordance (`Refusal::affordance`,
"(currently …)"). Both write it in the notation
`crate::props::rendering_unit` gives a value with no remembered unit,
which is the CANONICAL one: `thickness * 2` over a parameter declared
`4 mm` reads `= 0.008 m`, and a driven revolve angle of 45° reads
`= 0.7853981634 rad`. Since PR 3478 the symbol is said, so nothing
misreads it, but a user who writes millimetres and degrees reads a
number in neither.

`rendering_unit`'s doc argues for the canonical choice: it is how
`unparse` renders such a value, so the panel and the text door agree
by construction. A different notation for a COMPUTED value (the unit
of the parameters the expression reads, the unit its literals were
written in, or a document-level preference) is a design choice, not a
bug fix, and would move that agreement.
