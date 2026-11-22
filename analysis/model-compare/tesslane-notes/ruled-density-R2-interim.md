INTERIM (PR 7096 review, frozen head 4b9c52c3bc), still running.

Verdict so far: leaning APPROVE-WITH-FIXES; no MAJOR found yet.

Executed:
- My own probe (crates/mesh/tests/review_7096.rs, local only, release, default ε). It checks directed-edge census by vertex id, polyline segments used exactly twice, patch-boundary edges on polylines, winding against the analytic meridian normal, sampled deviation against the analytic surface, Pappus volume, a placed copy, and the count relation against the unsplit body.
- Coverage: cones through the apex at 0.001°–89°, full / half / 0.3 rad; frustums including radii 10:0.1 at 30°; cylinder and frustum walls; wedges of 0.05 and 1e-3 rad and 2π−0.01; scale 1e-3 and 1e3; δ 10 to 1e-4; every line edge split, and each one alone, at 1, 2 and 5 points and at 1e-8 / 1−1e-8.
- Results: ~4,000 meshing cells with zero census, winding or count-relation failures, and worst sampled deviation 0.9981 δ.
- Refusals:
  - The half frustum with every line split 1e-8 from its ends, placed, refuses UnresolvedWinding on its planar face. Base refuses the same, so it is not this PR's.
  - The 0.001° cone with the apex-end ruling split 1e-8, placed, refuses SingularUndecided (the apex vertex is undecided, so the face is not walked). The base comparison is running.
- Base hangs, cured on head: base (merge base f722ed6c) ran more than 17 min of CPU on one half-frustum cell (a side split 1−1e-8, placed, δ0.1) that head meshes at 26 triangles.

Notes so far (by inspection):
- README [mesh.sizing] says "the worst triangle is half a grid cell, or a Delaunay sliver". The walk's worst triangle (a rim chord against the next row) is neither. The PR body does not quote this sentence (SILENT).
- sizing.rs:20-22 still says the trimmed cylinder sizes at the headroom.
- rim_runs' filter is duplicated verbatim in Rows::of.
- walked() and Rows::of read the apex two ways (vertex_at_singular_point vs the lane's singular set); a disagreement refuses as MissingEntity.
- The needle pin is lost; cut_cone_cells (oblique-cut cone, not walked) looks like a body that could re-pin it, so the PR's "no pin until a non-rectangle row exists" may be wrong. I am checking this.

Next: mutants, the 7054 rows under the re-split mutant, and hosted CI (only 'change filter' queued when I looked).
