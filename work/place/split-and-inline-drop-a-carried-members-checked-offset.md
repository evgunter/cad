---
id: split-and-inline-drop-a-carried-members-checked-offset
kind: issue
title: split and inline drop a carried member's checked offset: carry inserts mates through the mate door, which clears it
status: closed
opened: 2026-10-02
priority: P1
cost: M
parent: placement-split-and-inline-at-a-gauge-are-refused-until-p2-split
closed: 2026-10-02
branch: place/carry-keeps-offsets
pr: 3885
---


## What

`refactor::carry` (`crates/editor-core/src/refactor.rs`) inserts every carried mate through the ordinary mate insert, whose `clear_joined_offsets` (`edit.rs`) clears the offsets of `a`'s group when a placing mate joins two placed groups. A split that moves verbatim, holding a group whose non-root member carries a checked offset, therefore loses that offset in the part: the outcome reports `OffsetCleared` in `part_maintenance` and does not refuse. Inline's carry is the same code. Seen on `ef90c4dba` by the P2-split spec survey's probe (a placed pair, the top's offset set to its solved pose, plus a second placed base, cut all four: the part's top has `offset: None`).

A11 (2) says a further statement of where an instance sits is never silently ignored, and A4's round trip is shape-exact. No pose moves — the offset was a checked statement — so the loss is of a verification, not of geometry.

## Done when

`docs/EDIT-PLACEMENT-SPEC.md` P2-split ruling 9 and row C1: the part (or spliced host) holds exactly the source's offsets, the edit list replays to it without a solve, and no `OffsetCleared` for a carried node survives in the outcome.

## Closed

PR 3885. `refactor::carry` re-states, with a recorded `DocEdit::SetOffset` after every carried node is in, each carried instance's offset that a carried mate's insert cleared; the edit list replays the clear and the re-statement with no solve. `Recording` reports its maintenance through `MaintenanceNet`, so no `OffsetCleared` for a carried node survives in `part_maintenance` or an inline's `maintenance`. Rows in `crates/editor-core/tests/p2_gauges.rs`: `a_verbatim_split_keeps_a_carried_members_checked_offset`, `an_empty_offset_inline_keeps_a_carried_members_checked_offset`, `a_carry_keeping_a_checked_offset_replays_without_a_solve`. Why not skipping the door's clear, the mutants and the sweep are in the PR body.
