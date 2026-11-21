---
id: two-decimal-parsers
kind: issue
title: editor-core's Ratio::from_decimal and quantity::Decimal parse one decimal grammar twice
status: open
opened: 2026-11-21
priority: P3
cost: M
---


`editor_core::Ratio::from_decimal` (`crates/editor-core/src/expr.rs`) reads a bare decimal as the rational it spells, and `quantity::Decimal::parse` (`crates/quantity/src/decimal.rs`) reads a unit-bearing one as a decimal; the text door (`crates/editor-core/src/parse.rs`, `literal`) calls one or the other on the same token text. One grammar written twice: a `Ratio` could be built from the `Decimal` (its digits and exponent), so the two cannot disagree on what a token spells.
