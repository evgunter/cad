---
id: an-expression-refusal-spells-its-variables-as-text
kind: issue
title: kernel: an expression's evaluation refusal has no Say, so the variables it names reach a cause as text
status: open
opened: 2026-11-23
priority: P3
cost: E
refs: [some-spoken-causes-and-reports-are-still-text, viewer-pane-lines-spell-their-nodes-as-text]
---


Found by the review of `some-spoken-causes-and-reports-are-still-text` (PR #7576).

`expr::EvalError` (`crates/editor-core/src/expr.rs`) names variables by `VarId` (`UnresolvedVar { var }` and its siblings) and has no `Say`, only a `Display`. Two readers take its text:
- `Evaluation::reading_spoken` (`crates/editor-core/src/eval/mod.rs`), on `ObservedRefusal::Expr`, writes `Sentence::from(source.to_string())`. A stackup cause read off a value no measure is under therefore says its variables as words, by id.
- `ObservedRefusal`'s own `Say` writes `{source}` for the same arm.

The viewer's `Refusal::ConstantRefused` carries the same `EvalError` (listed in `viewer-pane-lines-spell-their-nodes-as-text`).

**The fix.** Give `EvalError` a `Say` that mentions each variable through `Speaker::var` (as `Speaker::formula` already does for a formula's readers). Then say it as a piece in both readers.
