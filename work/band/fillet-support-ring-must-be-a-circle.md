---
id: fillet-support-ring-must-be-a-circle
kind: issue
title: A fillet whose planar support carries a polygonal ring refuses: the ring carry-through check reads only circles
status: open
opened: 2026-10-02
priority: P2
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

Its doc says circles are "the only rings this kernel mints on planar
faces at rest". That stopped being true when booleans could stand
prisms on a face: a union leaves each prism's foot as a POLYGONAL ring
of the face it stands on.

## Reached by

`demos/tour/src/heatsink.rs`, wall 2 (`wall_probes`): the heat sink's
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

When it lands, the heat sink's wall 2 refuses no longer and its probe
says what to change in the scene.
