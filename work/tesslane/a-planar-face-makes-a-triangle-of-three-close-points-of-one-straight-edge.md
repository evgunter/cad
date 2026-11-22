---
id: a-planar-face-makes-a-triangle-of-three-close-points-of-one-straight-edge
kind: issue
title: a planar face makes a triangle of three points of one straight edge 1e-8 apart, and refuses UnresolvedWinding
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Found building `a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation`; present on that unit's base.

## Finding

A straight edge split three times `10⁻⁸` of its range apart leaves three
vertices on one line that close together. A planar face holding the
edge triangulates in its own chart frame, where the three are not
collinear to rounding, and makes a triangle of the three: the face
refuses `UnresolvedWinding`.

Measured at the default ε, δ 0.01: the solid `(0, 0), (2 + tan 1°, 0),
(2, 1), (0, 1)` revolved a half turn about `y`, a line edge of its planar
cut face split at `0.5`, `0.5 + 10⁻⁸` and `0.5 + 2·10⁻⁸` of its range,
placed obliquely (`split_rulings`' `split_and_placed`):

    UnresolvedWinding { face: FaceKey(5v1), corners:
      [-0.8418165343391498, -0.3191037934773505, 5.314433496692783],
      [-0.8418164959864886, -0.31910379427719876, 5.314433484183443],
      [-0.8418165151628191, -0.3191037938772746, 5.314433490438113],
      height: 4.57e-16, allowance: 9.57e-15 }

The three corners are the three split vertices. Over the unit's measured
grid (the wall and the solid above at five half-angles, half turns, three
δ): 27 cells at the default ε (gaps `10⁻⁸` and `3·10⁻⁸`), 300 at ε
`10⁻¹²` at gaps `10⁻¹⁰` and up, and 143 more at `10⁻¹¹`. Two splits that
close mesh; it takes three.

The refusal is typed and nothing is meshed wrong. Not pinned: the cells
are the planar lane's, and the unit that found them changed the cone
arm of the chart lane only.

## Not measured

Whether the triangle is an ear the CDT cuts because the middle point
projects a rounding inside the line of the other two, or a sliver the
even-odd classification keeps from outside the face.
