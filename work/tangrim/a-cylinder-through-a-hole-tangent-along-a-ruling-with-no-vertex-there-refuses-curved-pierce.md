---
id: a-cylinder-through-a-hole-tangent-along-a-ruling-with-no-vertex-there-refuses-curved-pierce
kind: issue
title: A cylinder through a plate, a hole tangent to its wall along a ruling with no vertex of either rim there, refuses CurvedPierceUnsupported in every op
status: open
opened: 2026-10-31
priority: P2
cost: M
---


Found sweeping `a-one-segment-cylinder-through-a-plate-tangent-to-a-hole-refuses-single-site`
for its class.

## The pose

The cylinder `r = 0.3`, `z ∈ [−1, 1]`, through the plate
`[−1, 1]² × [−0.5, 0]` holed through by `r = 0.1` about `(0.4, 0)`
(tangent outside the wall along the ruling through `(0.3, 0)`) or about
`(0.2, 0)` (tangent inside), with no vertex of either rim on that
ruling: the cylinder's at azimuth 1 or π, the hole's at 0 (outside) or
π (inside), one-segment rims; and the review probe's two-arc pose with
the cylinder's vertices at 1 and 1 + π and the hole's at 2 and 2 + π.

## What it reads

Every op in both orders refuses `CurvedPierceUnsupported`, at rest and
turned, on `main` and on
`cleave/one-segment-cylinder-through-a-holed-plate` alike (the PR 5808
review's probe `rev5808r`, S4, extended; its own poses, the vertices at
1 and 2, at the default ε, `1e-6` and `1e-12`, the others at rest at the
default ε):

- one segment: `CurvedPierceUnsupported { operand: B, face: FaceKey(2v1), edge: EdgeKey(5v3) }`
  with the cylinder first, `operand: A` with the plate first;
- two arcs (vertices at 1 and 2): `face: FaceKey(4v1)`, `edge` the
  hole's rim (`6v1` outside, `5v3` inside).

The two walls are tangent along the ruling between the plate's faces
and run on together past each rim, so the reduction's covered arm
(`boolean/reduce.rs` `interior`, `sheets_part`) keeps the rims' pair
deferred where the sheets do not part; which of `reduce.rs`'s five
`CurvedPierceUnsupported` raises answers here is unmeasured. With a
vertex of either rim on the ruling the pose builds (the hole's alone,
or the cylinder's alone, but for the op keeping both walls, which
refuses `EdgeInFace` with the `CurvedFace` gap; both there, every op).

## Slate triage

2026-11-01: claimed from CLEAVE by `git mv` after PR 6177 merged (CLEAVE's split note: the cylinder-tangent rows 6177 files move to TANGRIM); id and body unchanged. (TANGRIM orchestrator)
