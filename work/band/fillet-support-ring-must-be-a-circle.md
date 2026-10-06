---
id: fillet-support-ring-must-be-a-circle
kind: issue
title: A fillet whose planar support carries a polygonal ring refuses: the ring carry-through check reads only circles
status: open
opened: 2026-10-02
priority: P2
cost: M
---

## What

Rounding a box's twelve edges refuses once anything has been unioned
onto one of its faces. `ring_circle` (`crates/sweep/src/blend/surgery.rs`,
the ring carry-through check) reads every ring of a band's planar
support as one circle and refuses anything else:

```
Blend { verb: Fillet, error: UnsupportedGeometry { at: Edge(..),
  detail: "a ring edge's carrier is not a circle, the only ring the
  clearance check covers" } }
```

A union leaves each prism standing on a planar face as a POLYGONAL
ring of that face, so its doc's "the only rings this kernel mints on
planar faces at rest" does not hold.

## Reached by

`demos/tour/src/heatsink.rs`, wall 1 (`wall_probes`): the heat sink's
`3 × 1 × 0.25` base unioned with five `0.1875 × 0.75` fins (sunk 1/16
into it, or flush and declared: the same rings either way), then
`Node::fillet(union, 31.25 mm, <the union's twelve FromA edges>)`. The
fins stand 1/8 inside the long edges and 1/4 inside the end, so at five
fins every trimline clears every ring by at least 3/32. The scene rounds the base
BEFORE the union instead, which builds; the count edit then cannot
re-run the blend.

## What it wants

The clearance check over line rings: the margin between a straight
ring edge and a straight (or circular) trimline is closed-form, as the
circle cases are.

When it lands, the heat sink's wall 1 refuses no longer and its probe
says what to change in the scene.

## Findings (BAND lane, 2026-10-06)

Measured against the tree, the claims hold: `ring_circle` refused
every ring with a line edge, at both of the pass's ring readers (arm
(a), a ring against an open link's trimline; arm (b), a ring of a
closed rim's host against its trim circle), and the heat sink's wall 1
refused with exactly the quoted detail.

One further defect in the same reader: an all-ARC ring adopted its
first arc's circle for the whole ring. Two overlapping bores leave a
ring of arcs of two circles, and in one subtraction order a fillet at
r = 0.252 carved through the second bore's arc into a body that failed
tier 3 (`RingMeetsOuter`), while the other order refused. A ring is now
read as one circle only when it is one closed circle edge; every other
ring is metered edge by edge, exactly, in arm (a) against the
trimline and in the support-boundary walk against the trim circle.
Pinned by `review_fillet_e2_probes::a_ring_of_arcs_of_two_circles_is_metered_arc_by_arc`.
