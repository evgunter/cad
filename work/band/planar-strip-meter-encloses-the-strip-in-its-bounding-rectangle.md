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
that request refuses `RingClearance` though it would carve. Conservative
(never a silent pass), measured by no fixture.

Close: meter against the quadrilateral itself — its four edges' half
planes, an edge missing it when it lies wholly outside one of them —
with a row that builds such a support.
