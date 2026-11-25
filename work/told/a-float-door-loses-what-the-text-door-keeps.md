---
id: a-float-door-loses-what-the-text-door-keeps
kind: issue
title: The Rust and Python number-times-unit door reads a float, so it loses digits and magnitudes the text door keeps
status: open
opened: 2026-11-21
priority: P3
cost: M
---


`number * MM` in Rust and `number * mm` in Python read the number as the shortest decimal of the float it already is (`quantity::UnitDef::canonical_of_float`), so the float door agrees with the text door on every decimal of at most fifteen or so figures, and differs where the decimal was rounded before the door saw it: the text `1000.000000000000111022302462515654042363166809082031251 mm` is the float above 1 m (a tie broken by the last digit), while the same literal in Rust or Python is first the float 1000.0000000000001, whose shortest decimal over 1000 is the float 1 m. A Python integer past `f64::MAX` raises on conversion, while `1e309 mm` reads as text. The stored leaf of a written literal is the canonical float (`Formula::literal_with_unit`), not the decimal as written.

The question: whether the float doors take the decimal itself (a `Decimal` argument, or a string, beside the float one), and whether a written literal stores its decimal, so a document read back writes what was typed. Neither is needed for the once-rounded door to be right on what it is given.

Measured by PR 7291's reviews:

- From sixteen significant figures the two doors read a typed decimal apart: of 4,000 decimals, 0 at fifteen figures, 295 at sixteen, 1,226 at seventeen. Example, pinned in `quantity::tests::a_sixteen_figure_decimal_reads_apart_at_the_two_doors`: `9683.884588090202 in` is `0x406ebf0fb776f8fa` as text and `...f8fb` through `* IN`. The viewer's panel field commits through the float door (`viewer::props::from_written`), so a sixteen-figure number typed there can land one float from the same text typed in the expression field.
- For a computed (not typed) float, `x * MM` read by its shortest decimal differs from the float nearest `x × 1/1000` by one float in 19% of a uniform sample of `x` in `[1, 1000]` (e.g. `847.5863032002954`). `Length::in_unit` (the exact quotient) and `* unit` (the shortest decimal) therefore read a float two ways.

That is the ratified reading of a float at the Rust and Python doors ("read a float by its shortest decimal", PR 7261); it is recorded here as a question, not a defect.
