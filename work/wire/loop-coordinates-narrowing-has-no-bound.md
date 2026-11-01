---
id: loop-coordinates-narrowing-has-no-bound
kind: issue
title: loop_coordinates and output_body raise NamingError::Emission for an input-reachable count past u32::MAX: a profile's loop count and a multi-body value's body count have no bound, so neither is a kernel bug
status: open
opened: 2026-10-21
priority: P3
cost: M
---


Found by `wire-emithelpers` (branch `wire/emit-helpers-row-four`), 2026-10-21, while converting `to_u32`'s callers to row 4 (`work/wirehold/emit-rs-shared-helpers-raise-emission-for-torn-reads.md`).

`loop_coordinates` (`crates/editor-core/src/eval/wire.rs:1906`) narrows a validated profile's loop count to the `u32` a program loop is addressed by. It raises `NamingError::Emission` past `u32::MAX`, and `Emission` is the kernel-defect category. Nothing bounds that count, though:
- a profile is recipe input, not a `topo` arena, so no `SlotMap` cap applies;
- `profile::Profile::validate` caps nothing.

So the narrowing is not row 4. It now carries its own `u32::try_from` instead of going through `names::to_u32`, which panics. The true ending is either a profile-door refusal naming the count, or a cap on the profile door that makes this row 4. That is a ruling on where the bound lives.

`output_body` (`crates/editor-core/src/names/emit.rs`) has the same mislabel. Found by the review of PR 5530, 2026-10-25.
- **What it narrows.** The output-body index of a multi-body value, to the `u32` an `EntityRef::body` holds, raising `NamingError::Emission` past `u32::MAX`.
- **Why no bound applies.** A pattern's count, which is recipe input, sets that body count, and no arena cap applies to it.
- **Callers:**
  - `name_pattern`, `name_in_part_copy` and `name_placed_copies` in `names/emit.rs`;
  - `wire_part`, `instance_of`, the placed-family flatten and `name_family` in `eval/wire.rs`;
  - `product.rs`'s two `crate::names::output_body(i).unwrap_or_else(…)` sites (`product.rs:625`, `:642`).
- **The D10 hold.** All of these sit on pattern, part or placement ground, which the hold covers, so `output_body` keeps its own `u32::try_from` until the hold lifts.
- **The same question as `loop_coordinates`.** Its true ending is a refusal at the door that sets the count, or a cap on that door.
