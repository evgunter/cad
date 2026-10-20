---
id: a-coincidence-meet-of-tied-cells-refuses-as-a-kernel-defect
kind: issue
title: A Meet row whose two cells are one tied name refuses MEET_OF_ONE as a kernel defect, though a legal tie can plausibly reach it
status: open
opened: 2026-10-20
priority: P3
cost: M
---



Filed by WIRE's `wire-emis-rest` lane (branch `wire/emission-rest`), on
`names-emission-raises-panic-or-take-their-true-ending`.

## What

`crates/editor-core/src/coincide.rs`'s `NamedCoincidence::meet` sorts
and deduplicates a meeting's cells and refuses
`NamingError::Emission { what: MEET_OF_ONE }` where fewer than two
remain. That variant ends with the kernel-defect ending.

Two callers reach it:

- `name_rows` (same file) names each kernel cell by its name in its
  input's table. Two distinct entities of one kind that a tie names
  under ONE name (`Entry::Tied`) both name to the same `NamedCell`, so
  a `Meet` of two tied edges at a point dedups to one cell. Ties are
  legal (N2), so this looks reachable by input.
- `eval/wire.rs`'s `in_member_space` spells a fold row's two cells as
  member cells and meets them; the unit test
  `union_row_tests::a_fold_row_is_the_member_cells_that_meet` pins the
  refusal on a hand-built row whose two cells are one name.

Not measured end to end: no document is known to reach it. It was left
typed for that reason rather than made `unreachable!`.

## What it needs

A document with a tie at a point meeting (two tied edges meeting at a
vertex the boolean or split records as `Meet`), then the true ending:
either the row is dropped as stating nothing (a ruling, since silent
discard is forbidden) or it is a missing rule (category-4 word, ending
`NOT_YET_ENDING`).
