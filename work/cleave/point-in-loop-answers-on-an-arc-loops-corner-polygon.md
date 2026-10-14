---
id: point-in-loop-answers-on-an-arc-loops-corner-polygon
kind: issue
title: the public splitting::point_in_loop reads an arc-bearing loop as its corner polygon with no refusal: a point inside a bore's circle reads Out, its centre OnBoundary
status: closed
opened: 2026-10-02
priority: P1
cost: E
branch: cleave/ptloop-arcs
pr: 3917
closed: 2026-10-03
---

Found by the `plane-section-polygons-drop-their-arcs` lane's class
sweep (corner-only reads of a loop that may carry arcs).

**Measurement.** `topo::point_in_loop` (`splitting::containment`,
re-exported at the crate root) documents its loop as "a planar polygon
(line carriers — the F5 regime)" but does not check it: `loop_points`
collects the corners and the parity walk runs on them. On
`bored_brick(0, 0, 1)` (`crates/sweep/tests/common/bores.rs`), each
cap's ring is the bore's circle, two corners at `(±1, 0)`. Against the
`z = 2.5` cap's ring, with normal `+z`:

- `(0, 0.5, 2.5)` — inside the circle — reads `Ok(Out)`;
- `(0, 0, 2.5)` — the centre — reads `Ok(OnBoundary)` (it lies on the
  chord between the two corners).

So an outside caller handed this door gets a silent wrong verdict on
any arc-bearing loop. The crate's own callers route arcs to
`point_in_carrier_loop` (which reads each edge on its carrier and
falls back to this walk only for an all-chord loop), and
`chord_join::rehome_rings`'s direct call is already filed
(`rehome-rings-reads-an-arc-bearing-run-through-the-polygon-walk`).

**Fix shape.** Either refuse typed when an edge is not a line (a
`PointInLoopError` variant naming the carrier), or make the public
door `point_in_carrier_loop` and keep the polygon walk crate-private.

