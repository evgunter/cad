FINAL REVIEW — PR 7096, frozen head 4b9c52c3bc (supersedes my interim)

**Verdict: APPROVE-WITH-FIXES.** No MAJOR. The walk is correct on everything I could build, and it changes nothing outside its class. Hosted CI is red on the PR's own new row (M1 below).

## Claims
1. **Stage-1 table / ruling: CORRECTED.** I did not re-run the stage-1 probe. I read the prototype at 0d78da36bc. Cure 1 used the spline arm's centroid insert (`refine_lane`) under a 400-pass budget in the lattice metric, with cure 4 in the isometric metric for cylinders.
   - A centroid insert cannot shorten the angular span of an edge the CDT draws from a side point to far rim points. That is the term the certificate reads.
   - So "cure 1 does not converge" is a property of this refinement rule, not of certificate-driven refinement in general (unsure).
   - Cure 2's refused cells are the development's 27-bit merge, not held edges: cure 2 run in the chart would not meet them (likely).
   - I would still have ruled the walk, for its proof and its simplicity, but placed it in the rectangle lane (see Style).
2. **Every walked mesh correct: HELD.** My own probe (not the PR's oracle; kept at /tmp/claude-0/-home-user-cad/d484124a-0ba4-5a04-b0e2-fb843aed7ea5/scratchpad/review_7096.rs.keep), release, default ε only. Per cell it checks:
   - directed-edge census by vertex id (each edge once, reverse once);
   - every polyline segment used by exactly 2 triangles, and every patch-boundary edge on a polyline;
   - winding against the analytic meridian normal, and 78-point sampled deviation per triangle;
   - Pappus volume, an obliquely placed copy, and the count relation against the unsplit body.
   
   Bodies: cones through the apex at 0.001/0.01/1/20/45/80/89°, each full, half and 0.3 rad; frustums including radii 10:0.1 at 30°; cylinder and frustum walls, inner faces reversed; wedges of 0.05 and 1e-3 rad and 2π−0.01; scale 1e-3 and 1e3. δ 10 to 1e-4. Splits: every line at once and each alone, at 1, 2 and 5 points and at 1e-8 / 1−1e-8.
   
   Results: ~4,000 meshing cells, zero census, winding or count failures, worst deviation 0.9981 δ.
   - Refusals: the half frustum with every line split 1e-8, placed, refuses UnresolvedWinding on its PLANAR face, as base does. The 0.001° cone split 1e-8 from the apex, placed, refuses SingularUndecided: not walked, by design.
   - Base also refused BoundaryEar on the 0.001° cone split 1e-8 unplaced; head meshes it at 18 triangles.
   - Base ran over 17 min of CPU on one half-frustum cell (1−1e-8 split, placed) and on the 0.001° cone; head meshes both instantly.
   - NOT run: ε 1e-6 and 1e-12 in my probe.
3. **Within δ by the argument: HELD.**
   - The invariant and its advance are proved correctly. Ties go to row a; b can exhaust before a only at i=n, given monotone rows and equal end angles.
   - The ruled_walk text does not state the apex case (m=0, b has no b_{j+1}); sizing.rs does.
   - 0.998 δ is by design (the step is solved; the count is ceil'd).
   - I found no triangle over δ by sampling, and no certificate refusal. A frustum with rims 100× apart stays under δ because both rims chord at the cone column step (`arm_steps` at ρ_max).
4. **Count relation: HELD.** The rows pin the relation against the unsplit body read in the same run, not numbers. It held on all my cells, including δ 10 (larger than the face), 1e-4, the scales, and 1e-3 rad wedges.
5. **Sizing at δ moves only what it should: HELD.** My own instrumented base/head differential: merge base f722ed6c vs head, release, CAD_FUZZ_SEED=1. Suites: mesh whole (523/523 head), step-import 332/332, and sweep's 62 modules that call mesh::tessellate (578/578). This is after the 7054 merge; the PR's digests were before it.
   - 455 common tests; 81,157 patches byte-identical.
   - Moved patches appear only in 10 split-side tests: 7 split_rulings rows, 2 curved::tests, and sweep a_box_along_an_inflection_ruling.
     - Cone 386,447 → 29,815 triangles; Plane 50,908 → 39,500; Cylinder 5,544 → 940.
   - Nothing moved in step-import. NO refusal added or removed.
   - The thin planar neighbour (slender wall, δ 0.1) is gone on head: `a_split_side_adds_one_triangle_per_point` passes.
6. **One reading: HELD, with a caveat.** `walked()` is the one predicate behind `face_target` (and through it `edge_target`/`arm_steps`, so the chord pass) and behind the dispatch.
   - `walk_face` re-derives the line flags and the apex (from the singular set) and refuses `MissingEntity` if they disagree with `walked()`, rather than falling back.
   - Faces falling back to lattice+CDT: ring or trim carrier; a loop not of the two shapes; an undecided apex. Only the undecided apex was reached (above).
   - Against 7041: walked is a second routing level inside Trimmed, invisible to `route_of` (likely conflict in `face_target`).
7. **Shared-edge hazard: HELD.** Case by case the argument is sound. I also checked a lens prism (two cylinders sharing both rulings): when one face is one chord wide, its caps degenerate exactly as in the plane case. My census found no edge used more than twice on any cell, including half cylinder, half cone, 0.05 and 1e-3 rad wedges, both sides split.
8. **7054 rows: CORRECTED.**
   - The re-split mutant (`developed_runs` returns no runs) turns red ONLY `a_run_on_a_cone_face_cut_by_an_oblique_plane_meshes` and `a_run_on_a_cut_cone_faces_seam_meshes`.
   - Six other 7054 rows pass with the re-split off: every_run_of_a_draft_cones…, a_run_of_ruling_points…, a_run_at_a_corner_is_anchored…, two_runs_anchored_by_the_apex…, a_frustum_seam_split…, a_draft_cone_ruling_split…. Their bodies are walked now.
   - No assertion was weakened. The two renamed rows now assert meshing plus the count relation, a stronger contract on a different mechanism.
   - The needle refusal has no pin. A row on the oblique-cut cone (not walked; it reaches `developed_runs`) is the natural home; the issue file says honestly that no row pins it.
9. **Mutants: HELD, plus mine.**
   - M2, the seam's two sides swapped: red on 10 rows.
   - M3, a side's fan wound backwards: red on 17 rows.
   - M7, the walk's direction fixed forward: red on 10 rows.
   - Survivors:
     - M1, ties advance b: equivalent (both walks valid; my probe agrees).
     - M5, apex not flagged a pole in the cert: equivalent.
     - M4, the seam duplicate-id skip removed: changes nothing on 40 rows and ~4,600 probe cells. The skip is unreached.
     - M6, a face of two rims through the apex walked: unreached (no body builds one; disclosed).
10. **[mesh.sizing]: FALSIFIED (SILENT).** README:46-52 says the worst triangle is "half a grid cell, or a Delaunay sliver", and "the theorem is pinned by a test next to the lane's step function".
    - The walk's worst triangle (a rim chord against the next row) is neither.
    - Its cited pin (`ruled_walk::tests::the_walk_covers_the_face_once_within_one_chord`) is chart combinatorics, not an equality row at `sagitta_step`/`cone_grid_step`.
    - The PR body quotes no README sentence.
11. **No probe ships: HELD.** `ruled_probe`, `CURE` and `ruled_cures_probe` are absent from the head. The sweep's blind-spot list omits nothing I found: ellipse rims fall under `has_trim_carrier`.
12. **CI: FALSIFIED (red).**
    - test (app) is red on this PR's own row: `SLOW split_rulings::no_walked_triangle_has_an_edge_a_neighbour_makes 0.564 s (new in this diff: 0.5 s)`, not in the slow set. So "gate ok" fails.
    - test (kernel) is red only on SLOW `sweep blend_dual_tangent::the_blend_size_tangent_matches_central_differences` at 1.013 s against 1.0 s. sweep is untouched, so not this PR's.
    - All other jobs are green.
    - Locally, head in release: mesh 523/523, step-import 332/332, the sweep meshing modules 578/578.
    - NOT run: viewer, tess-meter bytes, demos.

## Findings
- **MINOR (DEMONSTRATED):** hosted test (app) is red. `split_rulings::no_walked_triangle_has_an_edge_a_neighbour_makes` must go into the slow set (.config/nextest.toml) or be made faster.
- **MINOR (DEMONSTRATED):** split_rulings.rs, 7054's rows (named in claim 8). Six rows no longer reach the development's re-split, so the re-split mechanism lost six of its eight pins.
  - Their docs still describe it. `a_run_at_a_corner_is_anchored_along_its_ruling` (~l.790) says "The lane adds the run's triangle beside the neighbour along the ruling (`trimmed::Anchor`)… refuses at δ 0.01" under the mutant; false now.
  - SILENT in the PR.
- **MINOR (BY-INSPECTION):** README `[mesh.sizing]` (claim 10). The spec asked the PR to quote any sentence the build falsifies; the PR is silent.
- **NOTE (DEMONSTRATED):** trimmed.rs walk_face, the duplicate-id skip ("A full turn's seam corners…") is unreached; M4 changes no byte.
- **NOTE (BY-INSPECTION):** sizing.rs:20-22 still says "the trimmed cylinder and trimmed sphere… size at UNPROVED_HEADROOM". A walked cylinder sizes at δ.
- **NOTE (BY-INSPECTION):** the needle class (P2) has no pin. The oblique-cut cone rows are a non-walked body that reaches `developed_runs`. The PR body's "no pin until a non-rectangle row exists" is loose wording: such a row exists; it just pins no needle.

## Spec deviations
- Reported by the PR: the digests were taken before the 7054 merge (I re-took them after: same picture); the not-searchable both-nappe face.
- SILENT:
  - the [mesh.sizing] sentences;
  - the 7054 rows' loss of mechanism coverage and their stale docs;
  - the CI slow-set red;
  - the spec file still reads `status: dispatched` (cosmetic).

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (ruled_walk.rs whole; the trimmed.rs module header and dispatch; the README section).
- ruled_walk.rs:71-76 and 104-106: `rim_runs`' filter is copied verbatim into `Rows::of`, undisclosed. **sure**
- trimmed.rs:461-476: `walked()` hand-rolls "is this vertex the apex" via `vertex_at_singular_point`. `AnalyticRow::is_singular` (chart_table.rs:546) and walk.rs `singular_point_at` (l.1049) already say it, so this is a third spelling. `walk_face` then reads the apex a fourth way (the lane's singular set). **likely**
- trimmed.rs:449-458: `has_trim_carrier` is redundant with the loop's `_ => return Ok(false)` two lines later; two walks over the same edges. **sure**
- trimmed.rs ~3567: a shape disagreement between `walked()` and `Rows::of` refuses as `MissingEntity { "a walked face's two rows" }`, a mis-typed refusal of a predicate/reader split. **likely**
- trimmed.rs ~3579: the seam duplicate-id skip is unreached code whose comment asserts a case nothing builds (M4). **likely**
- ruled_walk.rs:193: `unreachable!` panics in the mesher on a structural assumption. It is true by my proof check, but the guard is a panic rather than a typed refusal. **unsure**
- Q7: the class is routed to the chart lane (`lane_of`), then special-cased back out of the CDT by a second predicate. Meanwhile the rectangle lane already writes one strip per column for the unsplit face, under the same column theorem. One strip writer with a side fan in the rectangle lane, or the walk as its own route in 7041's `route_of`, is how I would have done it. As built there are two strip writers and two statements of one bound (the rectangle lane's column theorem and the walk's text at the step functions). **sure** (taste)
- trimmed.rs:418-421: sizing for one class is decided by a lane-internal predicate called from the chord pass (`!walked` carves out of the headroom), not by the lane naming its theorem. **likely**
- trimmed.rs:418 / chords.rs:475,733: `walked()` (a loop walk plus apex reads) re-runs on every `face_target` call, per edge and per lattice. **unsure** (perf)
- trimmed.rs:1-13 and 36-38, module docs: still "CDT + even-odd… routes here by its outer loop's carriers, or by holding a ring"; no mention of the split route or the walk. **likely**
- README:46-52: the theorem kinds and the "pinned next to the step function" rule. The walk's pin is not of that shape. **likely**
- split_rulings.rs, the 7054 rows' docs (Anchor, runs, re-split) describe a mechanism their bodies no longer reach; the class sweep belongs over every row whose name says "run" or "development". **sure**
- ruled_walk.rs:19-27: the invariant is stated for two rims; the apex row (m=0) is covered only in sizing.rs. **unsure**
- The `2 => apexes == 0` guard (trimmed.rs ~479) is unpinned (M6), disclosed as not searchable. **sure**
- The PR body's "no pin until a non-rectangle row exists" against the issue file's "no row yet pins a needle there": one says a row is missing, the other that a pin is. **unsure**

## Rubric
- Idiom/structure 3/5: the walk itself is small and clean; the routing (route → lane → bypass), four apex readings and duplicated filters cost it.
- Test quality 4/5: the relation is pinned against the same-run unsplit body, mutants are mostly killed, and the shared-edge row is meaningful; it loses a point for six inherited rows that no longer test their name and for an unreached branch.
- Doc/comment honesty 3/5: the proof text is accurate; the module header, sizing.rs:20, README and 7054's row docs are stale.

## What I executed
- My probe on head over ~4,600 cells, plus base/head focus cells.
- Seven walk mutants and the 7054 re-split mutant, each against the 40 split_rulings / ruled_walk / reach_probes / curved rows and my probe.
- An instrumented per-patch digest differential on base and head over mesh, step-import and sweep's meshing modules.
- Hosted CI logs for both red jobs.

## What I read only
- The stage-1 prototype (cure 1/2 code).
- PR 7041's interaction.
- tess-meter, the viewer and the demos were not run.

The tree is restored to the frozen head; nothing committed or pushed.
