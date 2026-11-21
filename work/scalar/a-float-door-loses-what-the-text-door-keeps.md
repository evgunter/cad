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
