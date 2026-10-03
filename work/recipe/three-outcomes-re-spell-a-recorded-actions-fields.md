---
id: three-outcomes-re-spell-a-recorded-actions-fields
kind: issue
title: RegaugeThenMateOutcome, SplitOutcome and InlineOutcome re-spell Recorded's fields instead of carrying one
status: open
opened: 2026-10-03
priority: P4
cost: M
refs: [three-loops-apply-several-edits-and-net-their-maintenance]
---

Found by the fix pass of
`three-loops-apply-several-edits-and-net-their-maintenance`.

## What

`editor_core::Recording::finish` answers a `Recorded<P>` (or the
action's refusal) — `doc`,
`edits`, netted `maintenance`, `minted` — and three public outcomes
are each built by destructuring one and re-spelling its fields
beside their own:

- `RegaugeThenMateOutcome` (`crates/editor-core/src/edit.rs`,
  `regauge_then_mate`): `doc`, `edits`, `maintenance`, plus `mate`.
- `InlineOutcome` (`crates/editor-core/src/refactor.rs`): `doc`,
  `edits`, `maintenance`, plus `node_map` and `step_map`.
- `SplitOutcome` (`refactor.rs`): two recorded actions spelled as
  `remainder`/`remainder_edits`/`remainder_maintenance` and
  `part`/`part_edits`/`part_maintenance`, plus `instance` and the maps.

Each field's doc comment restates `Recorded`'s (the order contract, the
net), and each drops `minted`, which a consumer composing a follow-up
action would read.

## Why not in that unit

All three are public and read field-by-field outside editor-core:
`crates/pncad-py/src/py/refactor.rs` and `py/doc.rs` (the Python
projections), `demos/tour/src/assembly.rs`, and the editor-core rows
`p2_gauges`, `p2_split`, `p2_gauge_poses_and_doors`,
`p2_gauge_offsets_and_spaces` and `fixture/round_trip.rs`. Carrying a
`Recorded` is a public signature change with a demo consumer, not a
local tidy.

## Shape of a fix

Each outcome carries `recorded: Recorded<P>` (`SplitOutcome` two of
them) beside its own fields; the consumers read through it.
