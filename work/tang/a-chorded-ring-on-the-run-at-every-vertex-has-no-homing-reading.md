---
id: a-chorded-ring-on-the-run-at-every-vertex-has-no-homing-reading
kind: issue
title: A ring already chorded, every vertex on the run, still refuses RingHomingAmbiguous: ring_side reads vertices only
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [a-pierce-strut-at-a-pinch-has-no-vertex-off-the-run]
---


## What

`ChordJoiner::rehome_rings` (`crates/topo/src/chord_join.rs`) defers a
ring every vertex of which lands `OnBoundary` only when the ring is still
all null edges, an unjoined pierce ring (`is_pierce_ring`). Once a
pierce ring has been chorded to another site by a `mekr`, it carries a
real chord edge. If every vertex it then has sits on a later run, it
refuses `RingHomingAmbiguous` as before, although the chord's interior
is off the run and says which side the ring is on. `ring_side` reads
vertices only.

The shape needs a polygon with at least two sites on another polygon's
outline. Its chorded sites would all have to lie on that outline, which
takes a non-convex neighbour: say, a triangle in a U's concavity
touching it at its corners. No body has been built that reaches it. A
polygon that completes inside a ring still pending refuses through
`place_pending` instead (`a_mef_inside_a_pending_loop_refuses`).

## Candidate

Read a chord edge's midpoint after the vertices, in `ring_side` and
`chart_ring_side`. The first job is a body that reaches the shape.
