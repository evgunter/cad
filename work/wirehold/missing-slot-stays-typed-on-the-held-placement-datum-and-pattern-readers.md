---
id: missing-slot-stays-typed-on-the-held-placement-datum-and-pattern-readers
kind: issue
title: MissingSlot stays a typed arm at the datum, placement, pattern and instance slot readers the D10 hold keeps; every other reader panics
status: open
opened: 2026-10-20
priority: P3
cost: E
---


Filed by `wire-evalarms` (WIRE, `eval-typed-defect-arms-go-by-row-zero-or-panic`).

## Finding

`NodeErrorKind::MissingSlot` is a kernel bug refused typed: a slot
absent from a node's evaluated values means `slots::eval_slots` and
`Node::slots` disagree with the reader, which no input reaches (D9
`[failure.taxonomy]` row 4). The row converted every reader outside the
D10 hold to `eval/wire.rs`'s `#[track_caller]` `declared` reader,
which panics with that reason. The readers on held ground still raise
the typed arm:

- `eval/wire.rs` `need_scalar`, `need_vec3`, `need_point3`, read by
  `wire_datum`, `frame_from_slots` (datum frames), `stepped_map`
  (pattern steps) and `placement.rs`'s `Placement::chain_motion`;
- `eval/wire.rs` `wire_pattern` and `wire_placed_union` (the `Count`
  slot), `wire_part` (`SlotId::Instance`) and `instance_of`
  (`SlotId::Index`, read from `reads_projected` and `copy_of`).

## What to do

When the hold lifts, re-read these readers against the tree. Each
surviving one reads through `declared` (`slot_scalar`, `slot_count`, or
a `vec3` / `point3` twin of `slot_point2`), its `?` dropped, and then
`MissingSlot` goes with its `Display` arm, `NodeErrorClass::MissingSlot`,
pncad-py's `missing_slot` tag rows (`tags.rs`, `tests.rs`), the viewer's
`tree.rs` arm, the `refusal_concision_chains` and `display_contract`
rows.

## A sibling on the same ground

`NodeErrorKind::FaceFrameReadback` (`eval/mod.rs`, "an
evaluation-internal inconsistency between the emitted table and the
body, surfaced typed") is the same class on the datum-frame ground:
when the hold lifts, the D9 test (can a correct kernel reach it from
any input?) decides whether it is row 4 or a misfiled limit.

## D10 exit walk (2026-10-29)

Most of the named readers are gone (`wire_datum`, `frame_from_slots`, `stepped_map`, `wire_pattern`, `wire_placed_union`, `wire_part`, `need_vec3`, `need_point3`), and so is `FaceFrameReadback`. Four still raise `NodeErrorKind::MissingSlot` typed, so the scene still stands: `need_scalar` and `need_angle` (`crates/editor-core/src/eval/wire.rs:1201`, `:1209`, read by the loft's station moves at `:4357`), `instance_of` (`:2560`) and `member_of` (`:2590`). Open.
