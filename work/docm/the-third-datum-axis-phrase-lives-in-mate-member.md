---
id: the-third-datum-axis-phrase-lives-in-mate-member
kind: issue
title: mate/member.rs hand-writes the third copy of "datum axis", which eval::phrase::DATUM_AXIS is now the home for
status: open
opened: 2026-09-12
---



## Finding

Found by WIRE's operand-door unit while closing
`composed-expected-phrases-are-hand-copied-across-sites`. The path is
DOCM's and MSOLVE's, so the unit announced it rather than editing it.
Accurate at the merge base of that unit's PR.

`crates/editor-core/src/mate/member.rs`'s `axis_datum` writes
`expected: "datum axis"` as a literal. That phrase now has a home —
`crate::eval::phrase::DATUM_AXIS`, composed at compile time from the
family word `"datum"` so the phrase and the `found:` word beside it
cannot drift — and WIRE's two copies (`tube_args` and `stepped_map`'s
circular arm, both in `eval/wire.rs`) were retired onto it. This is the
third and last copy in the tree.

The rule the const's module states is *"no `expected:` is a literal
written at a call site"*, and this is the one site left that breaks it.
The change is one word for one path: swap the literal for
`crate::eval::phrase::DATUM_AXIS`. The refusal text does not move, so
`crates/editor-core/tests/msolve3_placer_refused.rs`'s
`kind.contains("datum axis")` stays green by construction.

## What is NOT owed here

`axis_datum`'s `found:` is already right: it comes from
`crate::eval::node_value_kind(doc, other)`, the recipe-side reading of
the family. WIRE's `node_operand` in `eval/wire.rs` is the same shape
given a home, and `axis_datum` is its natural second consumer — but
that is a refactor across a fence, not this row. This row is the
literal.
