---
id: chart-island-winding-is-a-second-reader-of-a-spline-walls-island-side
kind: issue
title: A spline wall's ring island is sided by chart_island_winding on its chart beside the island paths, which read a spline wall too
status: open
opened: 2026-11-20
priority: P3
cost: M
---

Found by PR 7187's sweep (`sect/rimside-retires`), 2026-11-20.

## What

Two readers in `crates/topo/src/chord_join.rs` answer "which side of a ring
island in a face holds the face's outer loop" for a spline wall:

- `chart_island_winding`, on the wall's chart, closed along the plane the
  solid's chords lie in. `boolean/join.rs` `ring_run_ccw` reads it for
  `IslandClosing::Wall`.
- `path_island_winding` / `path_island_winding_closed_by` (and, since PR
  7187, `loop_holds_outer`), chart-free. They read a spline wall as
  `Carrier::Wall` wherever `SplineWall::of` offers paths; `ring_run_ccw`
  reads them for `IslandClosing::Carried` on a spline face.

Which reader a spline wall's island gets depends on how its chord was
minted, not on the face. PR 7187 retired `RimSide` into the path reader
for one-site loops. This is the remaining pair. Not measured: whether a
pose exists where the two disagree, and whether the chart reader covers
walls the path reader cannot offer a path on (`SplineWall::of` returns
`None`), which would be the reason to keep it.
