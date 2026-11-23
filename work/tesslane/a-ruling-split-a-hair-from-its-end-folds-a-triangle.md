---
id: a-ruling-split-a-hair-from-its-end-folds-a-triangle
kind: issue
title: a cone face whose ruling is split a hair from its end refuses FoldedTriangle on the chart lane
status: closed
opened: 2026-11-17
priority: P3
cost: M
closed: 2026-11-17
---


Found by PR 7049's review (`a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it`).

## Finding

A cone of half-angle 20° through its apex, revolved `2π − 0.3` about
`y`, one ruling split `3·10⁻⁷` of its range from its start
(`topo::Body::split_edge`) and placed obliquely, refuses
`FoldedTriangle` on its cone face at δ 0.01 and 0.001. The face is the
chart lane's (`walk::splits_a_straight_side`), whose cone arm
triangulates in the development and finds a resolved triangle wound
against the face (`trimmed::TrimmedFace::emit`'s winding census). On
the base, where the rectangle lane took the face, the same cells
answered `UnresolvedWinding`, so this is no regression, and no mesh is
returned either way. The review measured the same answer at 1° and 5°
with a split `10⁻⁹` from an end at ε `10⁻¹²` (at the default ε
`split_edge` reads that split as none).

Pinned while open: `split_rulings::a_ruling_split_a_hair_from_its_end_folds_a_triangle`, now `a_ruling_split_a_hair_from_its_end_meshes` (below).

## Not measured

Which triangle folds: by its shape, the split vertex's two edges are a
sliver along the ruling beside the end vertex, and a lattice node or
the sliver's own corners within a hair of the ruling decide its
winding.

## Closed (2026-11-17, TESSLANE, unit 4c, `tesslane/ruled-density`)

The face is walked (`tessellate::Lane::Walked`, `crate::ruled_walk`), not
triangulated in the development: the split's point and the ruling's end
are two corners of the one triangle the walk fans from the rim point
beside them. The 20° cone meshes at the default ε and `10⁻¹²`, and a 5°
cone split `10⁻⁹` from its end at `10⁻¹²`
(`split_rulings::a_ruling_split_a_hair_from_its_end_meshes`). A 1° cone's
`10⁻⁹` split builds at none of the three ε here.
