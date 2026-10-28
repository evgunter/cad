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

**The steer did not hold (PR 5920's review, B4).** The sphere arm's CDT
was moved to the chart's metric at the trim box's mid latitude
(`(u·cos v_mid, v)` to a power of two), where a small face's boundary
circle is near round. The review then measured the same row at δ 4e-3
(`Intersect` at latitude 60°): `tessellate` answered `Ok` and
`check_mesh` refused it `NonManifoldEdge { edge: (0, 3), count: 4 }`
twice (sphere patch 1v1 with planar 8v5, and 5v3 with 8v1). The fix
pass's δ sweep of the PR's witness corpus (554 bodies × 9 δ from 4e-2
to 2.5e-4) found 17 such meshes: this cap at both latitudes from 4e-2
to 4e-3, the pole-strut slab at 4e-2, and a slab tilted −20° at 2e-2.

**Why no patch sees it alone.** The four-use edge is the ear's
DIAGONAL, and each patch uses it twice: the ear on one side, its
neighbour triangle on the other, as any interior edge. A per-patch
count of segment uses does not see it. The first fix pass added that
census too; it fired on none of the 17, its mutant survived, and the
second pass deleted it with its variant.

**What ships (the cure).** A curved face on an analytic arm (trimmed
cylinder or sphere) keeps no ear. A kept triangle on three consecutive
points of the trim polygon has its UV centroid inserted and the face
rebuilt, which splits it (`TrimmedFace::attempt`'s ear cure). An ear
the retry budget does not clear refuses `BoundaryEar`. The neighbour
may still cut the ear: a plane cap's convex polygon cuts ears freely.
But the curved side then holds three triangles about an interior
point, so no triangle is made twice. After the cure the same sweep
reads all 5535 meshes manifold and enclosing their bodies' volumes. The
witness is pinned in `crates/sweep/tests/snowman.rs`,
`a_cut_ends_at_the_nearest_of_several_boundary_hits`, at δ 4e-2, 2e-2,
1e-2, 5e-3 and 4e-3.

**What the cure moved.** It reaches every trimmed cylinder and sphere
face, not only the witnesses: the second review counted it firing on 60
trimmed-cylinder faces of
`m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta`, all
of which still mesh watertight within the chordal volume bound. The D9
goldens did not move with it. They moved with the same pass's
bulk-loaded CDT (8 of 40 body/δ pairs, every one a trimmed body), and
the tour's frames are re-rendered for both (`[render]`).

**What is left.** The NURBS arm and the curved lane still may keep an
ear, and the planar lane cuts them by design. A face pair where neither
side is an analytic trimmed face (curved lane against planar, say) is
not covered. A census at the fold that reads triangle uses across
patches would cover every lane.
