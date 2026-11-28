---
id: a-sphere-face-whose-hole-grazes-its-pole-folds-at-the-mesher
kind: issue
title: a sphere face whose hole passes a micrometre from its pole, reached by a 1 µm wrap edge, folds at the mesher (FoldedTriangle) though tier 3 passes
status: open
opened: 2026-11-25
---


Found by the dual review of `lathe/import-wrap-tip` (PR 7774, R1's
re-review). Read and compared in that lane.

## What

`crates/step-import/tests/fixtures/wrap-tip-review/hole_grazing_the_pole.step`:
a R = 10 mm ball less a cap about `(sin 0.3, 0, cos 0.3)` of angular
radius `0.3 − 1e-4`, closed by a plane. The north pole lies about 1 µm
outside the hole, so the sphere face holds both poles.

Import reaches each pole by one wrap edge to the hole's point nearest
it. The body passes tier 3 and measures its closed form
(4.182624669233374e-6 m³) exactly. `mesh::tessellate(body, 1e-3, tol)`
refuses `FoldedTriangle { face }` on the sphere face, after about 60 s.

What changed, compared on the import's edges:

- The north pole's wrap edge is 1.0e-6 m long on both the reviewed head
  (3bc03bd, which chose the azimuth by a sampled centroid) and the
  fixed head (fc3fa6e52c, nearest boundary point). The short edge is
  the geometry's, not the rule's.
- The south pole's wrap edge moved. The sampled centroid put it at
  y = 1.5e-3, about 0.28 rad off the hole's symmetry meridian; the
  centroid itself was a bias of how the boundary's edges were sampled.
  The geometric rule puts it on that meridian, so both rim split
  vertices sit at azimuth 0, the top and bottom of the hole. That head's
  body meshed; this one folds.

The import is geometry-only by ruling (the wrap edge's azimuth may not
depend on how a file splits its edges), so the meshing of this valid
body is the mesher's to make hold.

The row that goes red the day this closes is
`crates/step-import/tests/wrap_tip_review_witnesses.rs`,
`a_sphere_face_whose_hole_grazes_its_pole_meshes`, pinned red at every ε.

## What would close it

The curved lane meshes a sphere face whose wrap tip sits a micrometre
from its boundary, with two wrap edges on one meridian, without a folded
triangle. Then the pinned row passes and loses its pin.
