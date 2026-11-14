---
id: a-cone-face-near-its-apex-meshes-an-edge-of-four-triangles
kind: issue
title: a ringless cone face near its apex meshes Ok with an edge four triangles use: tessellate returns a mesh check_mesh refuses
status: open
opened: 2026-11-14
priority: P0
cost: M
---


Found by the review of TESSLANE unit 3 (PR 6960,
`a-curved-faces-domain-is-read-from-every-loop`), on its merge base and its
head alike: not that unit's change, and not a ringed face.

## What

`mesh::tessellate` returns `Ok` with a mesh `mesh::validate::check_mesh`
refuses `NonManifoldEdge { count: 4 }`: an edge of the mesh used by four
triangles. The bodies are T1's cone (`reach_probes`' T1: the revolve of
`[(0, 0), (1.2, 0), (0, h)]`, `h = 1.2 / tan(π/6)`, turned a quarter about
`y`) and its box `brick((0, 6), (0, 6), (−3, 3))` turned −50° about `z` and
moved so its edge sits at `(xf·s, h − 1.5·s, 0)`, which puts the box's cut
near the cone's apex for small `s`; the ops are the intersections, whose
cone face holds no ring.

Measured (`Tol::witness()`, both trees):

| `s` | `xf` | op | δ | answer |
|---|---|---|---|---|
| 0.03 | 0.5 | A ∩ B, B ∩ A | `5·10⁻³` | `Ok`, `check_mesh` `NonManifoldEdge { edge: (7, 11), count: 4 }` |
| 0.03 | 0 | B ∩ A | `2·10⁻²` | the same class |
| 0.01 | 0 | A ∩ B | `2·10⁻²`, `5·10⁻³` | the same class |
| 10⁻³ | 0 | A ∩ B, B ∩ A | `2·10⁻²`, `5·10⁻³`, `10⁻³` | `NonManifoldEdge { edge: (10, 14), count: 4 }` |

Pinned for the first row by `reach_probes::a_ringless_cone_face_near_its_apex_meshes_an_edge_of_four`.

`tessellate` does not run `check_mesh` (its doc says so), so a caller that
does not run it gets a mesh that is not a closed manifold. Not diagnosed:
which faces share the edge, and whether it is the cone arm's development
near the apex (`the-cone-arm-certifies-the-triangle-its-development-makes`
is changing the cone arm's lattice there) or the box's planar faces.
