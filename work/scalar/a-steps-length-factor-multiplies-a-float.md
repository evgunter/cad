---
id: a-steps-length-factor-multiplies-a-float
kind: issue
title: A STEP file's lengths are a float times a float factor, not the once-rounded door
status: open
opened: 2026-11-21
priority: P3
cost: M
---


`step-import` scales every length a file states by its unit's factor as one float multiply: `crates/step-import/src/entities.rs`, the `let value = raw * factor;` beside `length_factor` (about line 2867), and the same factor onto points in `crates/step-import/src/recognize.rs` (`p.x * scale`, about line 1923). A STEP `REAL` is decimal text, and the file's unit is a decimal factor (`.MILLI.` is 1/1000, a conversion-based inch states 25.4 in the file), so a coordinate written `300.04` in a millimetre file lands one float past the float nearest 0.30004 m, where `quantity::UnitDef::canonical` (the once-rounded door, `a-number-in-a-unit-is-rounded-once`) would land on it.

The question for the import's owner: whether a file's number in its unit is read through the same decimal door (the token text is in hand at the lexer, and a conversion-based factor is a decimal the file wrote), or whether the import's contract is the float the file's number parses to, times the float factor, and says so. Not decided in the once-rounded unit, whose doors are the text parser, `number * unit` in Rust and Python, `WrittenLength::in_unit` and the told rule.
