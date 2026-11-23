PR 7615 review — INTERIM (about 75 min in), frozen head 1cfa09180c, base a497926d13. Verdict so far: APPROVE-WITH-FIXES leaning, no MAJOR demonstrated yet.

Executed so far:
- `mesh`, whole suite plus slow set (`--profile ci --ignore-default-filter`, release), at ε 1e-9, 1e-6 and 1e-12, on both trees. Base: 562/562 at each ε. Head: 562/563 at each ε. The one red is `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause` (Folded(Sphere) vs Meshed), which the PR names.
- A probe of my own, on both trees at all three ε: 13 bodies (pipes cut on tilts 0.3 to 1.45, a hole beside a rim, balls cut by tilted planes near and far from the pole, slender cones cut on tilts up to 1.5), at δ/size 0.3 down to 1e-4 and at scales 1, 1e-3 and 1e3: 208 meshings. My torus builders did not build.
  - No meshing is lost.
  - On head, no triangle's sampled distance is over δ (worst 0.9998 δ).
  - Head/base triangles per body: 0.44 to 1.00, never above 1.
  - Head counts are the same at all three ε.
  - At most 1 correction round.
- Rounds instrumentation (local only, not committed), over the `mesh` suite at ε 1e-9:
  - The most rounds any natural face took is 3 (`m5_pr11_trimmed::a_wide_tilted_cylinder…`).
  - Faces taking one round: cone 17, cylinder 31, torus 6, sphere 0.
  - The rectangle lane's correction is reached only by its forced row and the spur (6 rounds, refuses).
  - The spline correction is reached only by `a_face_left_slivered_refuses_typed_through_tessellate` (6 rounds, refuses).
- Hosted CI on 1cfa09180c: run 45248370651 has been QUEUED since 19:23 UTC. No job has run on this head.

Findings so far (BY-INSPECTION):
- MINOR, stale prose. `trimmed.rs` still says a refinement or correction point is a centroid, or strictly inside a kept triangle: module docs at line 113 and 274, the loop comment at 719, the insert_points comment at 1318 and 1367, `uv_centroid`'s doc at 2486. The shipped point is the middle of a side.
- NOTE. The module doc says "the verdict reads no ε". `cert::FaceVerdict::read` takes ε, but only to split Certified from Unresolved; Exceeded does not depend on ε, so claim 7 holds in substance.
- Style (Q1, sure). `curved::correction_point` restates the Lattice arm of `TrimmedFace::correction_point` (middle, sag, choice), and its centroid restates `uv_centroid`. The rectangle lane also reads the budget and the test hooks from `trimmed`: a core hosted in one of its two consumers.
- NOTE (claim 12). The PR says "no triangle reads over δ there, so the correction does not reach them". The "so" is the wrong cause: in `attempt` the fold branch returns before the correction is consulted, so the correction would not run there even if a triangle read over δ.

Next: per-face differential of lane, chart, triangles and boundary points over the suite (claim 2), the mutants, and `sweep`'s meshing modules with the `bisector_nodes` margin.
