---
id: extent-scan-carrier-tangency-off-the-faces-refuses
kind: issue
title: A carrier tangency on the no-crossings path refuses though the touch point lies off every face
status: open
opened: 2026-10-02
priority: P1
cost: M
refs: [ball-inside-a-two-sphere-body-refuses-at-the-extent-scan]
---


Found by the class sweep of the face-scoped extent scan
(`work/reach/ball-inside-a-two-sphere-body-refuses-at-the-extent-scan.md`).

## Measured (branch `reach/extent-scan-faces`)

Both against the snowman `u1 = ball(1, y=0) ∪ ball(0.8, y=1.4)` (the
`snowman.rs` constructors), each pair disjoint or nested with no edge
touching a face, so the no-crossings path runs:

| operand | outcome |
|---|---|
| a ball of radius 0.4 centred at `0.6·(0, 0.7, 0.3)/‖(0, 0.7, 0.3)‖`, internally tangent to the radius-1 sphere at a point the snowman's trim cuts away | `SpheresMeet { verdict: Zero(..) }` |
| `brick((1.5, 3), (1, 2), (−1, 1))`, whose bottom plane is tangent to the radius-1 sphere at `(0, 1, 0)`, outside the plane face | `Escalated { Sphere(AgainstPlane) }`, `bool_sphere_extent_gap` margin 0 |

The union is the snowman plus the brick, and the snowman alone,
respectively.

## The shape

`boolean::ops::sphere_extent_scan` decides the CARRIER margin first
(`bool_sphere_extent_gap`, `bool_sphere_sphere_nested`) and refuses on
a decided zero before any face is read. A tangency of carriers touches
at one point (sphere × plane, internal sphere × sphere), and that point
lies on the two faces or it does not: placing it against each face
(`place_witness`, the chart trim on a sphere face, `contfp` on a plane)
and finding it `Out` of one certifies the faces apart at the touch.
The crossing case of the same arms now asks the faces
(`sphere_faces_apart`); the tangent case still refuses on the carrier.

The section certificate's R-tan arms (`section_cert`: every
`section_*` margin `signs` turns into `Section::Tangent`) have the same
shape at the pass: a tangency of carriers refuses whether or not the
touch locus meets the faces. Unmeasured there except for the coincident
cylinder pair, which is
`rounded-stack-subtract-and-intersect-refuse-fallback-extent` (on PR
3657's branch) and a different question (a 2-D overlap of trims, not a
touch locus).
