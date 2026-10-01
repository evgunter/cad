---
id: the-census-material-probe-reads-only-vertices-so-a-flush-nested-solid-is-undecided
kind: issue
title: The census material probe reads only an inner solid's vertices, so a solid nested flush on every side its vertices touch is CensusUndecidable(AllOn) where a face interior would decide it
status: open
opened: 2026-10-01
priority: P2
cost: M
---



## What

The census's material test between two solids of one body (`census.rs`,
the `probe` closure beside `enum Probe`) walks only the inner solid's
vertices with `point_in_solid_faces`. When every vertex answers
`OnBoundary` it returns `Probe::AllOn`, which is pushed as
`ValidationError::CensusUndecidable { what: Undecided::AllOn }`. The
refusal is honest about what ran out. But a solid nested inside another
and flush with it on every side its vertices touch (a block inside a
longer block, flush on four walls) can be decided by a point inside one
of its faces that is not flush, as the boolean's uncut-shell witness now
does (`crates/topo/src/boolean/shell_witness.rs`, found by CLEAVE's
`a-contained-flush-operand-with-every-vertex-on-the-boundary-refuses-as-ray-exhausted`).

## Measured

Not measured. This comes from reading the code during CLEAVE's sweep
for vertex-only witness walks. No fixture has been built that reaches
`AllOn` with a decidable pair.

## What a fix would be

Read the witness from `shell_witness` (tiers 2 and 3) after the
vertices run out. It is `pub(super)` to `boolean` today.
