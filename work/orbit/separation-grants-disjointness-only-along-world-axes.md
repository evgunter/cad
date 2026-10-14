---
id: separation-grants-disjointness-only-along-world-axes
kind: issue
title: topo::separation grants two placements or solids disjoint only by world-axis box non-overlap, so whether it certifies depends on how they are turned
status: open
opened: 2026-10-06
priority: P1
cost: M
---


Found by the operand-gate pose lane
(`boolean-operand-gate-separates-only-along-world-axes`) sweeping
`topo/src` for box overlaps read as a verdict. Unmeasured: read off the
code; no fixture has been built for it.

## What

`separation` GRANTS on box non-overlap (`boolean::boxes` module docs:
`Ok(())` is the disjointness certificate). Both of its doors read
world-axis boxes:

- `Separation::certify` (`crates/topo/src/separation.rs`, the hull
  test, then a descent to faces) reads the placed hulls in the world,
  then carries placement `j`'s face boxes into placement `i`'s
  prototype frame. Turning the whole assembly leaves that second read
  alone, but the relative turn between two copies widens `j`'s carried
  boxes.
- `SolidSeparation::certify` (same file, `x.hull.overlaps(&y.hull)`, then
  `y.tree.overlapping(b)`) reads both solids' face boxes in the world.

Two separated placements or solids may therefore be refused
(`PlacementsMeet`, `SolidsMeet`) at one pose and certified at another.

## The general form

The boolean's narrow phase (`boolean::separating::apart`) is the
reading to share: behind an overlap, read both faces' reaches along the
axis between their anchors and along the planar normals of both
bodies, in `boxes::BoxFrame::aimed` frames
(`census::face_reach_in`). Measure first: a pair of placements of a
curved prototype, clear of each other, swept through relative turns.
