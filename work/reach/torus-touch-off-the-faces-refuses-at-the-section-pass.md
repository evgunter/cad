---
id: torus-touch-off-the-faces-refuses-at-the-section-pass
kind: issue
title: A torus touching a plane off the faces refuses R-tan at the section pass
status: open
opened: 2026-10-03
priority: P1
cost: M
---


Found by the blind-spot pass of `extent-scan-carrier-tangency-off-the-faces-refuses`
(branch `reach/extent-scan-off-face-tangency`), which gave the section
certificate a touch for sphere × plane, sphere × sphere, sphere ×
cylinder and skew cylinder pairs and left the torus arms alone.

## Measured (that branch)

The donut of `crates/sweep/tests/germ_torus_doors.rs` (`R = 2`,
`r = 0.5`, about y) against `brick((1, 2), (−1, 1), (2.5, 3.5))`, whose
plane `z = 2.5` touches the outer equator at `(0, 0, 2.5)`, outside the
plane face (`x ≥ 1`). The two bodies are disjoint and no edge touches
a face. Every op, both orders:

```
FallbackExtentUnsupported { operand: A, face: FaceKey(4v1),
  what: "a curved face and a face of the other solid have tangent or
  near-tangent carriers ..." }
```

The same box moved over the touch (`x ∈ [−1, 1]`) refuses the same way,
so the answer does not read the faces.

## The shape

`section_cert::torus_plane` (and `torus_sphere`, and the coaxial and
parallel-axis wall arms) return `Section::Tangent` on any `Zero` margin.
Two kinds of torus tangency are not pinches:

- a partner tangent to the tube at an ELLIPTIC point (the outer half of
  the tube) touches at one point, where a bound on the section like the
  sphere arms' `Touch` holds — the touch clears when certified out of a
  face (`section_cert`'s "A touch" docs, `ops::boundary_clear_of`);
- a coaxial wall or a plane normal to the axis tangent along a whole
  parallel touches in a circle whose neighbouring sections are
  parallels, essential on both carriers, so W2 can clear them as it
  clears the crossing pose.

A tangency at a hyperbolic point (the inner half of the tube) is a
pinch and keeps R-tan.
