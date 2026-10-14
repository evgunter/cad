---
id: replace-face-cylinder-radial-normalizes-an-undecided-rejection
kind: issue
title: replace_face's cylinder transport normalizes a radial rejection without deciding its length
status: open
opened: 2026-10-06
priority: P3
cost: E
---


## What

`crates/topo/src/replace_face.rs`, the transport of an edge across an
offset (`Surface::Cylinder` arm for `Curve3::Line` and `Curve3::Nurbs`
edges), takes `radial = (mid - *origin).reject_from(*axis).normalize()`
and moves the edge by `radial * d`. The rejection's length is the
edge midpoint's distance from the cylinder's axis — the carrier radius
for a point on the face, held only to `radius > 0` by the surface
datum gate, not to the band — and it is normalized with no decision: a
midpoint within the band of the axis transports the edge along a
direction made of rounding.

Unmeasured: this filing built no fixture; an offset door may refuse
such a cylinder before the transport reads it.

## Shape

`UnitVec3::levered` on the rejection, levered by the radius, under a
K name this lane owns, refusing through `TransportError`. Found by the
`cleave/recl-flanker` sweep's second pass (`reject_from(…).normalize()`)
for the hand Gram–Schmidt shape.
