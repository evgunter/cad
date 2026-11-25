---
id: a-shells-dead-cells-name-every-outer-rim-as-their-pieces
kind: issue
title: The shell's lineage names every outer rim as the pieces of each seam, slit or apex it retired
status: open
opened: 2026-11-25
priority: P2
cost: C
---


Found by HELDRECORDS' `heldrecords/removed-members` lane.

## What

`Shelled::lineage` (`crates/topo/src/shell.rs`) states each operand seam,
slit and apex the chart reduction retired (`ShellNaming::dead`, the
`dead.edges` and `dead.vertices` loops) as `Fate::Divided` with
`pieces` set to `every_rim`: every outer rim face and hole region of
every designated chart. Those faces are not the dead cell's pieces.
They are the places a touch on it might be now. One of them, the rim
of the chart the cell lay in, holds what is left of its interior. The
carry reads a record on a dead cell against all of them. That is
harmless for correctness (a landing still needs the census to confirm
a touch), but the division states something false, and the rims of
other charts are read for nothing.

## What a narrowing needs

A map from each retired cell to the chart whose reduction retired it.
`RimNaming` holds `sources` (the chart's designated faces) but not the
seams and slits its reduction killed, and `ShellRetired` lists dead
keys with no chart. The reduction knows the chart when it kills the
cell (the chart-merge step), so the row would be a `dead` list per
`RimNaming`, or a `(dead cell, chart index)` row beside `dead`. With
it, the dead cell's pieces become that chart's `rims(rim)` alone.
Under (1a) (Ev on PR 7727) the opening stays unnamed, so a record on
the cell that lands on no rim of its chart refuses `Stale`.
