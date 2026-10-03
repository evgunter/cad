---
id: edge-tangent-to-a-curved-carrier-off-the-face-refuses-at-the-pierce
kind: issue
title: An edge tangent to a curved carrier at a point off the face refuses CurvedPierceUnsupported
status: open
opened: 2026-10-03
priority: P1
cost: M
---


Found by the blind-spot pass of `extent-scan-carrier-tangency-off-the-faces-refuses`
(branch `reach/extent-scan-off-face-tangency`), whose fix reads a
carrier touch against the faces in the extent passes but not in the
crossing layer.

## Measured (that branch)

The lens `ball(1, y=0) ∩ ball(0.8, y=1.4)` (the `snowman.rs`
constructors) against `brick((−1, 1), (−2, −1), (0, 1))`. The brick's
edge `y = −1, z = 0` is tangent to the unit sphere's carrier at
`(0, −1, 0)`, which the lens's unit-sphere face (it keeps `y > 0.83`)
does not hold. The two bodies are disjoint. Every op, both orders:

```
CurvedPierceUnsupported { operand: B, face: FaceKey(1v1), edge: EdgeKey(3v1), .. }
```

(operand A in the other order).

## The shape

`reduce::curved_face_arm` decides the edge against the face's CARRIER
(`frontier()` on a tangent or unsettled root) before any witness is
placed against the face. The touch is one point of the edge, and the
extent passes' rule applies: a point certified `Out` of the face, with
no edge of the face reaching the ball the decided margin admits about
it, is no event. Distinct from
`an-uncovered-edge-tangent-to-a-fillet-at-the-curved-operands-vertex-refuses`,
whose touch lies ON both faces.
