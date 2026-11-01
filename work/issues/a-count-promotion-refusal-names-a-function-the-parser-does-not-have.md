---
id: a-count-promotion-refusal-names-a-function-the-parser-does-not-have
kind: issue
title: the count-promotion refusal tells a reader to use count_to_scalar, which the formula parser does not know
status: open
priority: P3
cost: E
opened: 2026-11-01
---

## Finding

Found reviewing CHROME's #6242 (2026-11-01). `DimensionError::CountNeedsExplicitPromotion`'s `Display` (`crates/editor-core/src/expr.rs`, the `impl core::fmt::Display for DimensionError` arm) says *"`{op}` on a count needs an explicit promotion (use count_to_scalar)"*. That is the Rust method's name (`Expr::count_to_scalar`), but this sentence reaches a person typing a formula, and the formula parser knows the promotion only as `scalar` (`crates/editor-core/src/parse.rs`, the function table's `"scalar" => ("scalar", Function::Unary(Formula::count_to_scalar))`). Typed as told, `count_to_scalar(n)` is an unknown function. The refusal should name the function the reader can write, `scalar(n)`.

No program's `paths` claim `crates/editor-core/src/expr.rs` (`python3 scripts/work.py territory --files -`), hence this slate.

