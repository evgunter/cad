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
does not run it gets a mesh that is not a closed manifold.

## Its class

One step up from `two-faces-cutting-one-ear-of-a-shared-chord-chain-leave-a-four-use-edge`:
the same pocket with a diagonal in place of the ear's short side.

## The cause (measured by the lane that owns the cone arm, on five cells; not re-verified here)

The body is sound: `at_rest` passes, and the mesh passes `check_volume`.
The four-use edge joins two points of one shared ellipse chord chain that
are not neighbours on it (four or five chord steps apart). The plane face
across that chain has no interior points, so its triangulation uses chords
of the chain freely, and it uses that chord as an interior edge; the cone
face, one or two lattice cells across, uses the same chord as an interior
edge between two of its lattice points. No triangle is made twice and none
sits on three consecutive chord points, so the trimmed lane's ear test
cannot fire. Over a 288-mesh grid of that builder every failure is one edge
used twice by a plane patch and twice by the cone patch.

## What is owed (not chosen here)

- The cone arm's part is being cured in PR 6965: a kept triangle side may
  not join two non-adjacent boundary points that one neighbouring face also
  holds; the triangle's inside point is queued and the face rebuilt, as for
  an ear.
- Lane-wide, the same side can be kept by a trimmed cylinder, sphere or
  torus face, by the NURBS arm and by the curved lane. The ear row's "what
  is left" names the backstop that would turn every such mesh into a typed
  refusal on every lane: a census of triangle uses across patches where
  `tessellate` joins them.
