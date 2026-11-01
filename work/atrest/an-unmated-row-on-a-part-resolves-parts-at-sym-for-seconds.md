---
id: an-unmated-row-on-a-part-resolves-parts-at-sym-for-seconds
kind: issue
title: the door does not replay a node's row on a part, because resolving the parts at Sym costs seconds; owed, making that resolution cheap enough to read such rows
status: open
opened: 2026-10-22
priority: P2
cost: M
refs: [5587]
---


## What

The door's replay resolves the parts a document instances only for an
at-rest face pair it reads off placed copies
(`crates/editor-core/src/coincide/replay.rs` `rungs`, `reads_copies`).
A row of a node that stands on a part (`stands_on_a_part`), such as a
boolean over pinned instances that no mate states, is answered
`Unreplayed::ThroughAPart` with the assertion recourse, without a
replay. `asm2a_instantiate::a_symbolic_run_builds_a_part_and_the_door_leaves_a_row_on_one_unreplayed`
pins that answer, with the evaluation's resolver and without one.

So such a row is never proven, even where the replay could prove it.

## The cost of reading them

Measured locally in PR 5587, debug, with the parts resolved for node
rows on parts and with them not resolved:

| row | parts resolved | not resolved |
|---|---|---|
| `viewer::all assembly_display::fused_geometry_refuses_both_display_ops_typed` | 3.99 s | 0.24 s |
| `viewer::all assembly_display::a_fused_instances_section_is_drawn_and_its_display_controls_are_refused` | 3.91 s | 0.20 s |
| `editor-core::all asm2a_instantiate::a_symbolic_run_builds_a_part_and_the_door_leaves_a_row_on_one_unreplayed` | 2.2 s | 0.46 s |

With the parts resolved, the asm2a wall row came back `Numeric`, its
form outgrowing the door's budget: the price bought no proof there.

## Owed

Make part resolution at `Sym` cheap enough to replay a node's row on a
part, for instance by resolving only the parts the left rows' cells
reach, or by reusing a part's symbolic evaluation across door jobs.
Then read those rows through the replay, and move the asm2a row's
expectation to whatever the replay answers.
