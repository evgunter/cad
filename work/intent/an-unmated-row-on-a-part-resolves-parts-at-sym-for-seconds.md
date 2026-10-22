---
id: an-unmated-row-on-a-part-resolves-parts-at-sym-for-seconds
kind: issue
title: the door resolves a document's parts at Sym for any unmated row standing on one: a fused-instance document's door job goes from 0.2 s to 3.9 s
status: open
opened: 2026-10-22
priority: P2
cost: M
refs: [the-door-reads-a-mated-seat-by-its-mate]
---


## What

With `REPLAYS_PLACED_COPIES` retired (`the-door-reads-a-mated-seat-by-its-mate`, PR 5587), the door's replay resolves the parts a document instances whenever a row the first rung leaves needs one (`crates/editor-core/src/coincide/replay.rs` `rungs`, `stands_on_a_part`). A row of a boolean over pinned instances (no mate states it) is now read at `Sym` and comes back `Numeric`, recourse `Budget`, where main answered `ThroughAPart`/`Assert` without building the parts. That is the honest answer, and it is what `asm2a_instantiate::a_symbolic_run_builds_a_part_and_the_door_reads_a_row_on_one` pins.

## The cost

Measured locally, debug, with the resolution on and with it forced off for node rows:

| row | parts resolved | not resolved |
|---|---|---|
| `viewer::all assembly_display::fused_geometry_refuses_both_display_ops_typed` | 3.99 s | 0.24 s |
| `viewer::all assembly_display::a_fused_instances_section_is_drawn_and_its_display_controls_are_refused` | 3.91 s | 0.20 s |
| `editor-core::all asm2a_instantiate::a_symbolic_run_builds_a_part_and_the_door_reads_a_row_on_one` | 2.2 s | 0.46 s |

Both viewer rows went into the slow set in PR 5587. The viewer's door runs off the UI thread, so this is a latency of the checks readout, not of the frame. In the asm2a row, what the price buys is `Numeric` with the budget's recourse, not a proof; the viewer rows' answers were not read.

## Owed

Decide whether an unmated row on a part is worth resolving the parts at `Sym` for: keep it (and cut the cost, e.g. by resolving only the parts the left rows' cells reach), or answer `ThroughAPart` again for node rows and resolve only for at-rest face pairs.
