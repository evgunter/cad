---
id: carried-records-keep-per-kind-vectors-beside-the-one-list
kind: issue
title: OperandRecords, ops::Rows and ReductionContacts keep per-kind vectors beside ContactRecords' one list, with seven hand-written per-kind translations
status: open
opened: 2026-10-13
priority: P3
---


Found on PR 4921's review (`contacthold/face-face-touch-row`). Class,
not that unit.

## What

D1 makes a body's contact records one list of unordered cell pairs
(`ContactRecords`, `records.rs`), its kind read from the cells. Three
working types beside it keep one vector per kind instead:
`OperandRecords` (`vv`, `vf`, `ve`, `ee`, `ef`, `ff`, `boolean/mod.rs`),
`ops::Rows` (`ops.rs`) and `ReductionContacts` (`boolean/mod.rs`). Each
kind therefore has a hand-written arm in seven places:
`ContactRecords::carried`, `ops::carried_rows`, `ops::Rows::push`,
`ops::Rows::of`, `ops::carry_rows`, `validate_declarations` and
`OperandRecords::is_empty`. The split earned its place while only some
kinds carried; now every kind carries, and a kind added or a check
changed has to be repeated seven times (PR 4921 added `ef` and `ff` to
each, and its review found `Rows::push` filing `vv`, `ee` and `ff`
pairs without canonical order, which one list keyed by `Contact::new`
would have done once).

`FfContact` also names two key spaces told apart only by its doc: two
faces of one operand (a carried record) or an A face and a B face (the
reduction's row).

## Owed

Carry one list of `Cited<Contact>` (or of cell pairs tagged with the
operand) through the op door and the carry, with the per-kind reads
derived from it, and give the reduction's A×B face pair its own type.
