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

## 2026-10-18 — nested solids no direction separates (RESTREAD)

A turned direction does not reach every case: two solids one inside
the other (a rod in a tube's bore, the two thin walls a shell leaves of
a hollow body) have nested boxes along every direction, so
`SolidSeparation::certify` refuses `SolidsMeet` on them at every pose.
The census's arm 1 had the same hole and now clears such pairs by a
gap in depth about an axis, a centre or a circle that either face
proposes (`census::depth_gap`, RESTREAD's
`census-cross-solid-curved-pairs-undecidable-on-shell-results`), after
the world axes and `boolean::separating::apart`'s turned directions.
That reading takes two faces of one body; a door that reads face pairs
behind its box overlap can call it for its pairs too. Read off the
code; no `separation` fixture was built for it.
