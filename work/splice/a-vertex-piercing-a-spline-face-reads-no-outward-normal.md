---
id: a-vertex-piercing-a-spline-face-reads-no-outward-normal
kind: issue
title: A vertex piercing a spline face refuses CurvedBooleanUnsupported at the vertex-on-face pass: the pierced face's outward normal at the point has no spline arm
status: open
opened: 2026-10-30
priority: P1
cost: M
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
