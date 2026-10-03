---
id: placement-step-slots-are-spelled-three-ways
kind: issue
title: A placement step's slot is addressed three ways: step 0's own slots, PlacementStep, and MateFrameStep
status: open
opened: 2026-10-03
priority: P3
cost: M
---



Found in review of PLACE's mate-frame-offset unit (PR 3961).

## What

One concept — "component `arg` of step `k` of a placement chain" — has
three slot spellings in `crates/editor-core/src/node.rs`:

- a transform's, gauge's or offset's step 0: `SlotId::Translation(ax)`,
  `RotationAxis(ax)`, `RotationAngle`;
- a later step of those chains: `SlotId::PlacementStep { step, arg }`
  (`SlotId::rigid` maps between the two, and `rigid_arg` back);
- a mate side's frame offset at any step:
  `SlotId::MateFrameStep { side, step, arg }`.

Each has its own vector families (`VectorSlot::Translation`/
`RotationAxis { step }` beside `MateFrameTranslation`/
`MateFrameRotationAxis { side, step }`) and its own label function
(`rigid_label` beside `mate_step_label`). Python mirrors the split in
its slot words (`translation_x` …, `placement_step`, `mate_frame_step`).

## Why it was not unified in the unit

One address with an owner — say `SlotId::Step { owner: PlacementOwner,
step, arg }` with `owner` a node's own chain or a mate side — would
re-spell step 0's slots. Those are persisted in every `SetParam` and
`SetExpression` a log holds, golden in `tests/golden/slot_tables.txt`,
and published as Python slot words that refusals answer in and doors
take back (`slot_word.rs`). That is a wire break and a binding break
for every transform, gauge and offset, out of a mate unit's scope.

## What it wants

One placement-step address with an owner, step 0 included, and the
wire, the goldens and the Python slot alphabet moved with it — or a
ruling that step 0's legacy spelling stays and the other two fold into
one `{ owner, step, arg }` address.

## Held (2026-10-03)

Re-homed from PLACE at its close: the slot alphabet is `node.rs`'s,
which is RECIPE's ground. The row falls under the hold on placement
and `Expr` slots (`[ev]` PR #3990, "every slot holds a typed
variable"), which may re-spell every slot anyway. Park it on
`one-way-to-say-dependency-and-intent` once that row is on main; the
blocker is not set here because the row does not exist yet.
