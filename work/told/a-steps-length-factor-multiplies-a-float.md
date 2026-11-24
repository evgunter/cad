---
id: a-steps-length-factor-multiplies-a-float
kind: issue
title: A STEP file's numbers of sixteen or seventeen figures reach the unit door already rounded to a float
status: open
opened: 2026-11-21
priority: P3
cost: M
---


`step-import` scales every length a file states by its unit's factor as one float multiply: `crates/step-import/src/entities.rs`, the `let value = raw * factor;` beside `length_factor` (about line 2867), and the same factor onto points in `crates/step-import/src/recognize.rs` (`p.x * scale`, about line 1923). A STEP `REAL` is decimal text, and the file's unit is a decimal factor (`.MILLI.` is 1/1000, a conversion-based inch states 25.4 in the file), so a coordinate written `300.04` in a millimetre file lands one float past the float nearest 0.30004 m, where `quantity::UnitDef::canonical` (the once-rounded door, `a-number-in-a-unit-is-rounded-once`) would land on it.

The question for the import's owner: whether a file's number in its unit is read through the same decimal door (the token text is in hand at the lexer, and a conversion-based factor is a decimal the file wrote), or whether the import's contract is the float the file's number parses to, times the float factor, and says so. Not decided in the once-rounded unit, whose doors are the text parser, `number * unit` in Rust and Python, `WrittenLength::in_unit` and the told rule.

## Built in PR 7291, for numbers of at most fifteen figures

Built in PR 7291's fix pass (the project's ruling that every production site making a unit's number by a float multiply goes through the door in that PR): `step-import`'s length and angle factors are held exactly (`crates/step-import/src/units.rs`, `Scale`: the prefix table as powers of ten, a conversion-based unit's stated factor as the decimal the file wrote, times its base exactly), and every number the file states in its unit is read as its decimal times that factor, rounded once (`Scale::apply`, through `quantity::Decimal::times` and the canonical door). The identity factor moves no bits. Two contexts are the same scale when their exact factors are equal.

## What is left

`Scale::apply` receives the float the file's number already parsed to (`as_real`), so a length is "the file's decimal times the exact factor, rounded once" only where that float's shortest decimal is the file's own token: every token of at most fifteen significant figures. STEP writers commonly emit sixteen or seventeen, and for those the token was rounded to a float before the door saw it (`work/told/a-float-door-loses-what-the-text-door-keeps.md` is the same loss at the Rust and Python doors). The fix is to carry the token's text, or its `Decimal`, from the lexer to `as_length`/`as_angle`, which is not local to the unit door (the lexer's `Value::Real` holds an `f64`). The cost of the door per length (PR 7291's second fix pass): about 6–8 µs on a loaded box where a float multiply costs nanoseconds, most of it `Decimal::of_float`'s formatting, which carrying the token would also remove.
