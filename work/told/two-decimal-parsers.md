---
id: two-decimal-parsers
kind: issue
title: The text door reads one number token through three parsers: Ratio::from_decimal, std's f64 parse and quantity::Decimal
status: open
opened: 2026-11-21
priority: P3
cost: M
---

`crates/editor-core/src/parse.rs`, `literal`, reads a number token through one of three readers: `quantity::Decimal::parse` for a unit-bearing number (the once-rounded door), std's `parse::<f64>` for a lone bare decimal written as a value, and `editor_core::Ratio::from_decimal` (`crates/editor-core/src/ratio.rs`, about line 81) for a bare decimal inside a formula. All three read `lex_number`'s grammar, and each token reaches exactly one of them. They agree on every value today (for a factor of one, std's parse and `UnitDef::canonical` are both correctly rounded) and differ only on absurd exponents: `Decimal` saturates its exponent at `2^60` and reads past the format as ±∞ or a signed zero, `Ratio` refuses past `i32`, std's parse reads ±∞ or zero. One grammar written three times: a `Ratio` could be built from the `Decimal` (its digits and exponent), and the lone value read through `M`'s door, so they cannot come to disagree.
