---
id: TESS-4
kind: unit
title: check_mesh decides the empty mesh: what the validator claims, and a name for zero triangles
status: closed
opened: 2026-09-22
priority: P0
cost: E
branch: tess/4-empty-mesh
refs: [check-mesh-passes-the-empty-mesh]
pr: 3094
closed: 2026-09-22
---


## Closed (2026-09-22, PR 3094 merged at `9d3b4183e`)

`check_mesh`'s contract is the mesh of a solid: `MeshError::NoTriangles`
(no patch, or every patch empty) and `EmptyPatch { face }`, decided
before the edge census so the report names the cause — D2 row 1,
because the empty body is tier-1/2 valid and `tessellate` on it
legitimately yields the empty mesh; `tessellate` stays count-blind.
Rows red without the guards, including the discriminating one (an
empty patch beside a closed tetrahedron, which the edge census cannot
see). `docs/GUIDE.md`'s contract sentence and `Mesh`'s headline
re-worded with the change (LIB's page, announced seam). Filed: the STL
writers' `solid` file for zero triangles (EXCH); the Python closure
cross-checks that pass on nothing (LIB); the hand-built one-patch
meshes' four homes (TINT); the question whether a zero-face body should
refuse at `tessellate`'s door (TESS, P3). Middle tier: one review, no
row.
