---
id: a-ruling-split-within-the-developments-resolution-merges-two-points
kind: issue
title: a cone face whose ruling is split closer to its end than the development resolves refuses DevelopedMerge at fine eps
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Found by PR 7049's review (`a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it`); ruled a residual of that PR (TESSLANE, 2026-11-17).

## Finding

At ε `10⁻¹²` only, a cone face with a ruling split `10⁻⁹` of its range
from an end refuses `SphereTrimShape { DevelopedMerge }` at every δ.
Measured on two bodies, revolved a full turn about `y` and placed
obliquely, each cone face's line edge split in turn:

- the frustum wall `(1, 0), (3, 0), (2, 1), (1.5, 1)` (two one-face
  full-turn cone faces, each walking its seam twice);
- the frustum `(0, 0), (2, 0), (1, 1), (0, 1)`, through the axis (two
  half faces).

The review measured 168 such cells over frustums, frustum walls and
diamonds, full, half, thin and wide; every one meshed on the base,
where the rectangle lane took the face. At the default ε and at
`10⁻⁶` `split_edge` reads a split that near an end as no split, so the
body is not built.

## Mechanism

The face is the chart lane's (`walk::splits_a_straight_side`), whose
cone arm triangulates in the development (`trimmed::developed`). Its
slant radius is cut to 27 significant bits and its direction to 26, so
every product is exact and a ruling's points lie on one exact line
through the apex, which is the rule that keeps a triangle off one
ruling. Two points of one ruling whose radii agree to within `2⁻²⁷` of
the radius develop to one point, and `trimmed::developed_merge` refuses
the face, since the CDT would hold two mesh vertices as one. A split
`10⁻⁹` from an end is such a pair.

Pinned at ε `10⁻¹²`:
`split_rulings::a_ruling_split_within_the_developments_resolution_refuses`.

## The cure seen

Give the radius more bits on a face whose points need them, choosing
the split of the 53 bits between radius and direction per face from
its closest pair along a ruling and its closest pair of columns, with
a refusal when neither fits. That is a design choice about the
development's cut, and it moves every chart-lane cone patch. For these
faces the final state (unit 4c: a ruled rectangle meshed without the
development's CDT) removes the limit.
