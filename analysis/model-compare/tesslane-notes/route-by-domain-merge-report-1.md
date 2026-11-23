Merge pushed: 4884dc875aa6c03dc73e172837724aa0e9106e4d, a merge commit of origin/main 326c40c3f0. `git ls-remote` confirms it.

**Resolution.**
- crates/mesh/src/tessellate.rs: one conflict, in the lane call. I kept my `Route::Trimmed(Some(lift))` arm (the domain read by the route) and gave it main's `crate::trimmed::BOUNDARY_CLEARANCE` argument. Main's `UnresolvedWinding` check, where faces join the mesh, auto-merged and is untouched.
- crates/mesh/src/trimmed.rs: main's new unit-test helper `lane_answers` called the retired `lane_of`. It now reads `Routes::witness` (`Routed::To(Lane::Trimmed)`), and `face_domain` takes `tol.band`. Behaviour is unchanged.
- .config/nextest.toml: main's slow-set line, with my rename. Main's `a_notched_pipe_wall_refuses_at_the_shape_door_and_a_holed_one_meshes` is `a_notched_or_holed_pipe_wall_meshes` here, and I kept `a_notched_sphere_face_meshes_on_the_sphere_row`.

**What moved and why.**
1. route_rows constants, re-read on new main 326c40c3f0 at ε 1e-9, 1e-6 and 1e-12. The merged head prints the same digests as new main at all three ε.
   - Moved only because main moved (its clearance changes the trimmed lane's meshes): all eight tilted-ball cell digests, and the mesh digests of three K-corpus bodies (ball less a slab turned 0.3; the slab at 0.3 turned 2e-9; at 0.6 turned 8e-9).
   - Every K-stream digest, run and memo hit, is unmoved, and so is every other body's bytes.
   - I re-pinned these, and the file header now names 326c40c3f0.
2. D9 goldens: unmoved, green with no re-pin.
3. Notched faces under the clearance: 30 bodies (pipe through and half in, sphere 0, −45 and top, six ops each) × 9 δ (5e-2 to 1e-4) × 3 ε, frozen head 3494faec95 against the merged head.
   - Nothing refuses, `UnresolvedWinding` or otherwise, and there are 0 zero-area triangles on either head.
   - Pipe notches and "sphere top": identical counts.
   - Only the notched sphere face's count drops, the same at all three ε:
     - sphere 0, A∪B and B∪A: δ 5e-2 292→270, 2e-2 1012→994, 5e-3 2526→2488. A∖B: 280→258, 1000→982, 2514→2476.
     - sphere −45, A∪B and B∪A: δ 5e-2 360→352, 2e-2 884→866, 5e-3 3480→3442, 5e-4 34052→33934. A∖B: 348→340, 872→854, 3468→3430, 34040→33922.
   - Every other δ is unchanged.

**Runs on 4884dc875a.**
- mesh whole, `--ignore-default-filter`: 503/503 at 1e-9, 1e-6 and 1e-12.
- Green: `run-gates.sh`, `work.py lint`, `doc-gate`, `clippy -p mesh --all-targets -D warnings`.

**Result to follow:**
- the viewer's two slow-set rows (`display_budget`, `review_pick_r2`), and other slow-set rows in crates that tessellate;
- the armed mesh suite against new main;
- the armed sweep and tour, detached.

**Changed PR-body paragraphs:**
1. Byte and K-stream evidence, the last bullet: "Both rows' constants were read on a worktree of `326c40c3f0` (main after the trimmed lane's boundary clearance) at ε 1e-9, 1e-6 and 1e-12 and agree with this head at all three."
2. Notched faces, a new last bullet: "Under the boundary clearance (PR 7043) no notched face refuses `UnresolvedWinding`; the pipe notches mesh to the same counts as before the merge, and the notched sphere face gives up 8–118 triangles at some δ (e.g. sphere 0 ∪ at δ 5e-2, 292 → 270)."
3. Cost line: as in my previous note.
