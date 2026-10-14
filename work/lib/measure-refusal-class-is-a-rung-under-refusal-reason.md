---
id: measure-refusal-class-is-a-rung-under-refusal-reason
kind: issue
title: MeasureRefusalClass is a payload rung under RefusalReason that no curated list carries
status: open
opened: 2026-10-03
---


(PROPS' recourse-grammar lane, PR 3942, from
`scripts/payload-rung-sweep.py --check` firing on the new type.)

## What

`editor_core::drive::MeasureRefusalClass` is the closed type
`RefusalReason::MeasureRefused` now carries in place of a
`class: &'static str` (`work/props/measure-refused-reduces-the-typed-refusal-to-its-name.md`):
`SelectionKind` for the wiring's own arm, `Clearance(ClearanceRefusal)`
holding the engine's refusal itself.

`RefusalReason` is curated on the analysis list; this rung is on none, so
a façade consumer that holds a drive receipt and wants the arm cannot
name it. **It is a regression in nameability, narrowly**: the old field
was a `&'static str`, which a consumer could match as a string. The
serialized receipt is unchanged (`MeasureRefusalClass::name()` renders
the same two vocabularies), so what a consumer reads has not moved — only
what it can match on.

## Why it is filed rather than carried

The carry is this crate's row, and it owes what the other rungs in
`DISPOSITIONS` owe: a façade re-export, a word for the Python door
(`crates/pncad-py`), and a `test_binding_census` answer about whether the
namesake spells the MEMBERS or only the name — and `Clearance`'s payload
is `ClearanceRefusal`, an eleven-arm enum, so the carry decision is about
that type too, not only this one.

## What would close it

Either carry it on the analysis list with `ClearanceRefusal` beside it,
or argue the non-carriage with its falsifier where the other interior
rungs' arguments live, and move the `DISPOSITIONS` row from `filed` to
`argued` in the same change.
