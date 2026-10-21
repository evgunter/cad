---
id: loop-coordinates-narrowing-has-no-bound
kind: issue
title: eval/wire.rs's loop_coordinates raises NamingError::Emission for a profile past u32::MAX loops, though nothing bounds a profile's loop count, so it is not a kernel bug
status: open
opened: 2026-10-21
---


Found by `wire-emithelpers` (branch `wire/emit-helpers-row-four`), 2026-10-21, while converting `to_u32`'s callers to row 4 (`work/wire/emit-rs-shared-helpers-raise-emission-for-torn-reads.md`).

`loop_coordinates` (`crates/editor-core/src/eval/wire.rs:1906`) narrows a validated profile's loop count to the `u32` a program loop is addressed by. It raises `NamingError::Emission` past `u32::MAX`, and `Emission` is the kernel-defect category. Nothing bounds that count, though:
- a profile is recipe input, not a `topo` arena, so no `SlotMap` cap applies;
- `profile::Profile::validate` caps nothing.

So the narrowing is not row 4. It now carries its own `u32::try_from` instead of going through `names::to_u32`, which panics. The true ending is either a profile-door refusal naming the count, or a cap on the profile door that makes this row 4. That is a ruling on where the bound lives.
