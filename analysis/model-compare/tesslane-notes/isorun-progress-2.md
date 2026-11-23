ISO-RUN-DENSITY progress note 2 (stage 1, measured, debug build, default ε and 1e-12 complete; 1e-6 partial, re-running).

The density gap is real and it is two different gaps, one per chart:
- Cylinder walls (notched pipe, 130 cells + scaled/holed): today 4–86× the reference (median 22×, slender wall 223×), growing as 1/√δ: the chart lane's rows, as 4c found on ruled faces. Reference = the rectangle lane's count summed over the iso-rectangles the face cuts into along lines through its corners (computed in the lane from the trim polygon; argued in the report).
- Sphere faces (wedged ball, 40 cells + scaled): today 1.65–4.4× (median 1.96×): that is the headroom alone (count ∝ 1/target). No extra mechanism is in play.

Cures, default ε (same picture at 1e-12):
- Size at δ alone: spheres 0.75–1.0× ref (3.0× at one δ 0.1 cell), every cell meshes, cert ≤ 0.99δ. Cylinders still 43× and 10/130 refuse CertificateExceeded at δ 1e-3.
- Cure 1 (δ, rows off on the cylinder, refine on refusal): EVERY cell meshes at 1.00–1.40× ref (centroid) / 1.00–1.20× (widest-edge split), ≤ 1 refinement round, cert ≤ 0.998δ, check_mesh + winding clean. Unlike 4c, refinement converges at once here: no split side fans a vertex across columns.
- Cure 2 (held iso lines through corners + cure 1): identical counts to cure 1 on cylinders; spheres ≈ the same. Earns nothing measured.
- Cure 4 (aligned lattice at δ): spheres 0.86–1.02× (best), cylinders 3–120× with rows; with rows off it IS cure 1 (no rows, no lattice nodes on a cylinder).
- Cure 3 (iso-rectangle decomposition): not built; its count is the reference by construction.
Still to do before the report: the "rows off, no refinement" control (is refinement needed at all?), wall times on a quiet box, neighbour/rim chord effects, the 1e-6 row, the census over sweep/step-import (in progress), and the same switches over the whole mesh suite (they currently apply to every chart-lane analytic face, not only the class). Leaning: cure 1's ingredients (δ target + rows off), possibly with no refinement at all.
