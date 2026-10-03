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

## Disposition from the ladder unit (CLEAVE `cleave/ladders`, PR 3716)

PR 3716's sweep listed this probe as "not this unit: a different
question". That holds for its first pass, which looks for ANY vertex of
`inner` that is `In` and continues past `Out`. It does not hold for the
residue this row is about. When every vertex reads `OnBoundary`, the
remaining question is the ladder's own: which side of `outer` is
`inner` on. So this row's prescription stands.

The ladder is now one function, `shell_witness.rs` `complex_side`. It
reads vertices, then edge carrier midpoints, then certified planar-face
interior points, and passes over in-band readings. Census cannot call
it as it stands. Census probes a SOLID's face selection
(`point_in_solid_faces`), and `complex_side` probes a whole body
(`point_in_solid`). The fix makes the probe a parameter of the ladder,
or moves it next to `SolidFaces`, and runs the ladder on `AllOn`.
