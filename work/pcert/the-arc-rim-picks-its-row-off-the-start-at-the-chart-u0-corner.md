---
id: the-arc-rim-picks-its-row-off-the-start-at-the-chart-u0-corner
kind: issue
title: nurbs_iso_derive's arc rim picks its row by evaluating the chart at u = u0 against the carrier's start, so a reversed partial arc rim refuses no_boundary before its direction probe runs
status: open
opened: 2026-10-13
priority: P3
cost: E
---


Found by the review of PR 4955 (`pcert/off-iso-line-takes-its-general-row`),
whose body had called this pick covered by the direction probe. It is
not: the row pick runs first.

## What happens

`nurbs_iso_derive`'s arc rim (`crates/topo/src/pcurves.rs:1227`) picks
the row with

```rust
side_pick(&|cand| surface.eval(cu0, cand), &[cv0, cv1])
```

`side_pick` compares the carrier's START with each candidate, so this
asks whether the start is the chart corner `(cu0, v)`. That holds for a
rim that starts at the chart's `u0` edge: a full circle on a closed
chart (where `cu0` and `cu1` are one point) and a rim running with the
chart's `u`. A partial arc rim that runs AGAINST the chart's `u` starts
at `(cu1, v)`, so neither candidate holds, and the arm refuses
`IsoUnsupported` (`no_boundary`, "the carrier's start point lies on
neither chart boundary"). The u-direction pick (#327) that would have
chosen the reversed image runs after the row pick, so it never sees
this rim. A partial rim that starts strictly inside the chart's `u`
domain fails the same way.

## Why it is P3

The refusal is false (the rim is a boundary row, and its reversed
image certifies), but it is typed and loud, and nothing is stored
wrong. Reasoned from the code and not executed: no suite is known to
reach a reversed partial arc rim.

## Fix

Pick the row by a quarter-span probe over both directions and both
rows at once, the way the boundary-iso schedule does: offer the four
`(v, direction)` IsoArc candidates to `probe_pick`, which reads the
carrier in its interior, and drop the separate start-only row pick.
A row with an executed reversed partial arc rim (on main: `no_boundary`)
should pin it.
