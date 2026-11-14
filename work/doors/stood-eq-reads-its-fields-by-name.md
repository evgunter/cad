---
id: stood-eq-reads-its-fields-by-name
kind: issue
title: viewer session's Stood has a hand-written PartialEq that reads its fields by name, so a field added to it falls outside equality silently
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Filed by GREEN (`green/censuses`, PR 6921), reconciling the censuses with main `2c843a2e77`.

## What

`impl PartialEq for Stood` (`crates/viewer/src/session.rs`) compares `self.copies` and the evaluation's `Arc` by name. A field added to `Stood` would compile and sit outside equality. The test-utils census `hand_written_impl_census::every_hand_written_walk_is_held_to_its_declaration` reports it: `impl PartialEq for Stood — reads self.copies with no exhaustive pattern`. The impl arrived with PR 6938's merge (`461095c52b`, `placing/p3p4-cleanup`): the file at its first parent has none, and at the merge it has one.

## Pinned meanwhile

GREEN pinned it in the census's `KNOWN_HAND_LISTED`, keyed `(session.rs, PartialEq, Stood)` and naming this row. `every_known_hand_listed_impl_is_still_found` goes red the day the impl destructures.

## Owed

Destructure both sides exhaustively (`let Self { copies, evaluation } = self;`) and compare the bound names. Then strike the entry.

    cargo nextest run -p test-utils -E 'test(hand_written_impl_census)'
