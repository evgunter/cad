---
id: planar-strip-meter-encloses-the-strip-in-its-bounding-rectangle
kind: issue
title: blend: the planar local carve meters a support's other edges against the strip's bounding rectangle, refusing a non-convex support whose edge only enters the rectangle
status: open
opened: 2026-10-06
priority: P3
cost: M
---

## Finding

`surgery::strip_clearance` (arm (d) of `ring_clearance_pass`) meters
each outer-boundary edge of a planar support that the local carve
leaves in place against the RECTANGLE that bounds one requested edge's
strip (the slabs along the edge and across it), not against the
strip's own quadrilateral. With an oblique cut-off the quadrilateral
is a trapezoid, and an edge of a NON-CONVEX support can enter the
rectangle's corner beyond the trapezoid without touching the strip:
that request refuses `FaceClearanceUncertified` (the arm refuses as
predicate 2, whose closed form it is) though it would carve. Conservative
(never a silent pass), measured by no fixture.

Close: meter against the quadrilateral itself — its four edges' half
planes, an edge missing it when it lies wholly outside one of them —
with a row that builds such a support.

## The same shape, a second time (2026-10-07)

Found by the sweep of
`ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver`.
`strip_clearance`'s margin is `max` over the four slab terms, each
over the WHOLE edge. Even against the rectangle itself, then, a
straight edge that misses it by passing its corner diagonally (outside
the `m` slab at one end, outside the `d` slab at the other) reads
not-clear. The close above, "an edge missing it when it lies wholly
outside one of them", keeps that gap for the quadrilateral as well. The
cut-off's cap meter closed the same gap for straight edges by reading
the edge point by point (`CapSliver::line_clearance`,
`crates/sweep/src/blend/open/end_face.rs`): the least, over the
segment, of the largest term, at the segment's ends and the terms'
pairwise crossings. For half-planes alone, every crossing is linear.
