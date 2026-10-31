---
id: a-vertex-piercing-a-spline-face-reads-no-outward-normal
kind: issue
title: A vertex piercing a spline face refuses CurvedBooleanUnsupported at the vertex-on-face pass: the pierced face's outward normal at the point has no spline arm
status: closed
opened: 2026-10-30
priority: P1
cost: M
closed: 2026-10-31
branch: splice/vertex-on-spline-face
---


Filed by `a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite`'s
lane. This is lily wall 8's next stop once a spline span is read
against a curved face.

## What

`demos/tour/src/lily.rs` wall 8 unions the lower stem (A) and the long
leaf (B). The sweep now reads the leaf's spline seams against the stem's
torus wall and gets past them. The union then refuses
`CurvedBooleanUnsupported { operand: B, face, kind: Nurbs }`, where
`face` is a NURBS wall of `lily_leaf_a`. It does so at ε 1e-9 and 1e-6;
at 1e-12 the stem's declared frontier stands the walls down first.

The site, traced at both ε, is `boolean::vtxfac::classify_vertex_on_face`.
The stem's vertex `5v1` pierces the first of the leaf's two NURBS walls
(`3v1`), at (−0.0611, 0, 0.1045). The same vertex, wall and point are
reached at 1e-9 and at 1e-6. The pass reads the pierced face's oriented
datum at the point through `face_normal::face_outward_normal_at`, which
answers `Ok(None)` for a `Nurbs` or `Approx` surface ("no implicit
form"). The pass maps that answer to the C5 typed refusal.

## What it needs

An outward normal for a spline face at a point the sweep placed on it.
This is the chart's normal at the point's foot, read in the face's
sense. It must not be read across a crease
(`KnotStructure::crease_runs`, the class
`the-sector-walk-reads-no-nurbs-face-at-a-crossing-vertex` names) or at
a foot on another sheet. Then comes whatever the vertex-on-face pass
reads after the normal. Behind this is the NURBS × torus section
(SECTARMS `a-nurbs-face-against-a-torus-has-no-section-arm`).

Related: `spliceface/a-spline-wall-cannot-say-whether-it-holds-a-pierce-point`
is the same pass's `held_reads` asking a spline wall whether it holds a
pierce point. Here the spline face is the pierced face itself.

## Closed (2026-10-31)

A spline face gives its outward normal at the chart point the sweep
placed a point at, through a new door,
`face_normal::spline_outward_normal_at`. That chart point is either a
span's meeting with the chart (`reduce::spline_interior`) or the
placement read's in-trim foot (`contain::curved_face_placement_charted`).
`reduce::ContactAcc::charts` carries it to the pass, as
`ContactAcc::branches` carries a branch. No projection foot is read.

The door works in three steps.
- It certifies `S(uv)` within the band of the point.
- It reads the normal only where the chart has one:
  `geom_brep::regular_jet`, then `geom_brep::chart_normal_at`, the
  sector walk's pattern.
- It hands the face's bit to `chart_normal_at`, which folds it through
  `OutwardNormal::from_chart`.

A crease (`crease_runs`, a closed seam) or a vanishing `S_u × S_v` at
that point refuses by name: `NormalAtError::SplineCrease` or
`SplineIrregular`, which the boolean reports as `NormalAtSplineCrease`.
`Approx` is read as its fit (C5). The pierce's curvature lever on a
spline face is the chart's certified arm (`T::spline_chart_arm`) over
the sectors' reach. A pierce placed at no chart point refuses
`CurvedBooleanUnsupported`, as before.

Whether the point has a second foot on another in-trim sheet within the
band is the placement's claim. It is filed as
`spliceface/the-spline-placement-read-certifies-no-second-in-trim-foot`
(the SPLICE orchestrator's ruling, 2026-10-31).

Lily wall 8 now stops one site on, at the pierce's ring on the leaf's
wall (`a-pierce-of-a-spline-face-cannot-mint-its-ring-vertex`), at
1e-9 and 1e-6.
