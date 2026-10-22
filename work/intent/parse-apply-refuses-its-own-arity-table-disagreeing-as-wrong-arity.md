---
id: parse-apply-refuses-its-own-arity-table-disagreeing-as-wrong-arity
kind: issue
title: parse.rs apply refuses an arity-table disagreement as WrongArity with a sentinel count (0, usize::MAX)
status: open
opened: 2026-10-20
priority: P3
cost: E
---


Filed by `wire-evalarms` (WIRE), from the sweep for typed kernel-defect
refusals that D9's rewrite (PR 5411) left in `crates/editor-core`.

## Finding

`crates/editor-core/src/parse.rs`, `apply` (the call's function applied
to its bracketed arguments): after `args.len() != arity` has refused,
two branches meet only the parser disagreeing with itself:

- `let (Some(a), b) = (it.next(), it.next()) else { .. }` — the
  arguments are non-empty and arity-checked — refuses
  `ParseError::WrongArity { found: 0, .. }`;
- the `(_, _)` arm of the name × second-argument match ("Arity was just
  checked; unreachable, kept typed") refuses
  `ParseError::WrongArity { found: usize::MAX, .. }`.

Both raise an input refusal for a kernel bug, with a count that is not
the input's. D9 `[failure.taxonomy]` row 4 (PR 5411): a state only a
kernel bug reaches panics, its message saying why it cannot occur.

## What to do

Row 0 first: an arity table whose entries carry the constructor
(`fn(Formula) -> ..` for one argument, `fn(Formula, Formula) -> ..`
for two) makes both branches unrepresentable. Failing that, each
becomes an `unreachable!` naming the table entry and the count.
