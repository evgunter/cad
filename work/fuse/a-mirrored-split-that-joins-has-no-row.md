---
id: a-mirrored-split-that-joins-has-no-row
kind: issue
title: No split row takes the mirrored lane and joins, so the emitter's side check on a join record is unrowed
status: open
opened: 2026-10-08
priority: P3
cost: M
---


## The finding

The split's mirrored rerun (the pinch lane, `split_one_solid` in
`crates/topo/src/splitting/mod.rs`) flips its record's sides through
`SplitNaming::mirrored` (`crates/topo/src/splitting/finish.rs:163`), and
the emitter checks each join record's side against the half that holds
the edge it left (`crates/editor-core/src/names/emit_topo.rs:342`).
The flip has a unit row (`splitting::finish::mirrored`). The emitter's
check has none: no split row takes the mirrored lane AND joins. A probe
over 754 split rows (topo, sweep, editor-core) found none, so a check
that refused every mirrored joined split would go unnoticed.

Found by the delta review of PR 4307 (O4).

## What it needs

A body pinched on one side of the plane (the single-sided pinch the
mirrored lane exists for) whose split leaves a vertex between two edges
of one carrier on a side, so the mirrored run joins: a row that names
the joined edge through the editor, and so reads the flipped record's
side.
