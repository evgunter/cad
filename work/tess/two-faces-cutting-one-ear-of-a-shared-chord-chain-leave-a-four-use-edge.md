---
id: two-faces-cutting-one-ear-of-a-shared-chord-chain-leave-a-four-use-edge
kind: issue
title: two faces that both cut the same ear of a shared chord chain leave an edge of four triangles that only check_mesh sees
status: open
opened: 2026-10-28
---


Filed by TESS's sphere-arm unit
(`sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane`),
which hit it and steered round it.

**Measured.** `crates/sweep/tests/snowman.rs`,
`a_cut_ends_at_the_nearest_of_several_boundary_hits`, `Intersect` at
latitude −60°: the result is a cap of height 0.001 — one sphere face
and one plane face sharing one closed circle edge, nothing else. With
the sphere arm's CDT in raw lattice-metric UV, `mesh::tessellate`
answered `Ok`, and `check_mesh` refused it
`NonManifoldEdge { edge: (5, 7), count: 4 }`: both faces cut the ear on
chord points 5, 6, 7 of the circle, so the triangle (5, 6, 7) was
emitted once by each face, opposite-wound, and the diagonal 5–7 is an
edge of four triangles.

**The class.** An ear of a shared chord chain is the triangle of three
consecutive chord points, and it is the same three mesh vertices
whichever face cuts it. Nothing in any lane forbids cutting one: the
planar lane cuts ears freely, and a curved or trimmed face cuts one
wherever no interior point lies in the ear's circumcircle. Two faces
across a chain that both cut the same ear make this pocket, and
`tessellate` returns it as a mesh; only `mesh::validate::check_mesh`
sees it. In practice a lattice point near the chain keeps a curved face
from the ear, which is why it surfaced on a face about two lattice
cells across.

**What the unit did.** The sphere arm's CDT now runs in the chart's
metric at the trim box's mid latitude (`(u·cos v_mid, v)` to a power of
two, `trimmed::tessellate_trimmed`'s sphere arm), where a small face's
boundary circle is near round and every ear's circumcircle holds the
face's interior points, so the arm cuts none there, and the row meshes.
That is a steer, not a guarantee.

**What would settle it.** A census at the fold that refuses typed (or
re-triangulates) when two patches emit the same triangle on a shared
chain, or a lane rule that no triangle has all three corners on one
edge's chord list.
