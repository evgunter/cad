---
id: ellipse-rims-tangent-with-no-vertex-at-the-touch-refuse-curved-pierce
kind: issue
title: Ellipse rims tangent at a point with no vertex at the touch refuse every op CurvedPierceUnsupported, where circles build
status: open
opened: 2026-11-01
priority: P2
cost: M
---


Found by `tangrim/one-segment-ellipse-rims`'s outcome probe (scratch,
not committed), on `main` `7979553590` and that branch alike.

## The pose

`a_closed_conic_in_the_partners_face.rs`
`ellipse_rims_tangent_outside_build_every_op`'s scene: the boss `r = 0.3`
and the hole `r = 0.1` about `(0.4, 0)`, both cylinders tilted about `x`
through the origin and cut by `z = 0`, so their rims are ellipses tangent
at `(0.3, 0, 0)`. Here both rims are one segment with each vertex
opposite the touch (hole `0`, boss `π`). Also the same inside: the hole
about `(0.2, 0)`, tangent at `(0.3, 0, 0)`, hole vertex `π`, boss `π`.

## What it reads

Every op in both orders, at rest and turned, at tilts `0.3` and `0.7`
rad and at ε = 1e-6, 1e-9 and 1e-12, refuses
`CurvedPierceUnsupported { operand: A, face: FaceKey(12v3), edge:
EdgeKey(6v3) }` (`FaceKey(2v1)`, `EdgeKey(16v1)` with the plate first).
The refusal is the one `boolean/reduce.rs` builds for a curved arm's
`CurvedEvent::Deferred` touch, raised by `settle_deferred` when the touch
is still unread on the edge's fragments (measured by instrumenting the
three `frontier` closures in `reduce.rs`, none of which fired). The
other vertex placements (both at the touch, either alone there, two
arcs) build.

At tilt `0` (circle rims) every op of both poses builds `SOUND`, but the
inside pose's unions, which refuse `TangentCrescentUnsupported`
(`a-union-across-a-tangent-crescent-refuses-unbuilt`).

## Cause (not yet measured)

The edge is the boss's rim, an ellipse, and the face is the bore's
cylinder wall, which it touches at one point. Which reader leaves the
touch unread (the curve × face route's tangency for an ellipse against a
cylinder, or the crossing layer's coplanar ellipse touch,
`work/cpair/two-coplanar-ellipses-have-no-meeting-method.md`) is not yet
measured. The circle against the same wall places its touch.
