---
id: a-ruling-split-a-hair-from-its-end-folds-a-triangle
kind: issue
title: a cone face whose ruling is split a hair from its end refuses FoldedTriangle on the chart lane
status: open
opened: 2026-11-17
priority: P3
cost: M
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

Pinned: `split_rulings::a_ruling_split_a_hair_from_its_end_folds_a_triangle`.

## Not measured

Which triangle folds: by its shape, the split vertex's two edges are a
sliver along the ruling beside the end vertex, and a lattice node or
the sliver's own corners within a hair of the ruling decide its
winding.
