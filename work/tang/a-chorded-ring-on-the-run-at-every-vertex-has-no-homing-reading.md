---
id: a-chorded-ring-on-the-run-at-every-vertex-has-no-homing-reading
kind: issue
title: A ring already chorded with every vertex on the run, or a chart ring every vertex of which has a degenerate ray, refuses RingHomingAmbiguous though a point of it decides
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

## The chart lane's degenerate rays (TANG review, 2026-10-03)

On a wall's chart, `chart_ring_side` skips a vertex whose azimuth ray
is degenerate: the ray passes a run vertex, or the vertex sits at a
crossing's height. A ring with every vertex skipped is
`RingSide::Undecided` and refuses `RingHomingAmbiguous`. That happens
even when the ring is decidably off the run, for example a vertex at
the same azimuth as a run vertex but at another height. The refusal
predates the strut fix. A pierce strut at a pinch on a wall always
reads this way, because its point is a run vertex. So the chart lane
defers nothing, and a declared curved union there falls through to the
REST door. `full_turn_bore_mate`'s shaft off the bore's seam is the
reachable case, where the strut at (−0.25, 0.433, 2.0) shares its
azimuth with the run at heights 0.5 and 1.0. Reading a degenerate
vertex's height against the run vertex's own height would separate
"on the run" from "off it at that azimuth". After that, a strut that is
genuinely on the run could defer on a chart too.

## Candidate

Read a chord edge's midpoint after the vertices, in `ring_side` and
`chart_ring_side`. The first job is a body that reaches the shape. On
the chart, split a degenerate ray's verdict by the endpoint's height
(above).
