FINAL REVIEW — PR 7096, frozen head 4b9c52c3bc (base = merge base f722ed6c95). Reviewer lane.

**Verdict: NOT-MERGEABLE-AS-IS.** The walk is correct, within δ and at the claimed count on every ordinary body I could build (≈47k meshed cells, zero failures of my own oracle). Three things block: two demonstrated regressions against base on tight bodies, and the PR's own row reds hosted CI.

## Findings

**MAJOR-1 (DEMONSTRATED, base/head): a face that meshes on base refuses on head.** `ruled_walk.rs` `Rows::walk`, second-side fan (~:165-185). Body: apex cone, profile [(0,0),(tan a,0),(0,1)], revolved Partial(0.05) or Partial(0.5), EVERY line edge split at [1−1e-8, 0.5, 1e-8], placed obliquely. Base: Ok, 34 triangles. Head: `UnresolvedWinding` on the Cone face.
- default ε: scale 1000, a = 1°, 5°, 30°, 45°, δ 3000…3 (7 of 8 bodies; 30 cells in my sweep).
- ε 1e-12: also at scale 1 (1°, 5°, 30°, 0.1°, 0.001°, and a bicone [(0,0),(1,1),(0,3)]).
- One side split at the same fractions meshes.
- Cause (by reading, likely): a ONE-chord apex face walks to one triangle [a0,a1,apex]. The first side's fan consumes it, so the second side's `find_map` finds its edge in a fanned triangle [a1,apex,s1] and fans from s1, a point of the OTHER side. With s1 a hair from the apex and e1 a hair from the rim, [s1,a1,e1] is a sliver (height 1.4e-13 against allowance 1.8e-12).
- This makes two texts false: the comment "an edge of exactly one triangle, the first or the last walked", and the module doc's "fan edge from a side's point to the corner across from that side".

**MAJOR-2 (DEMONSTRATED, base/head): head returns `Ok` with a four-use edge where base returned a manifold mesh; "No edge of four triangles" is incomplete.** `ruled_walk.rs:38-66`. Body: a ball of radius 1.05 with a coaxial bore of radius 1, i.e. `bulge_loop([( (1,−h),0 ), ( (1,h), ±tan(θ/4) )])` revolved Full, h=√(R²−1), θ=2·atan2(h,1). Either sign: the neighbour is a Sphere or a Torus face that holds BOTH rims of the bore. Split the bore's seam.
- Head: `check_mesh` NonManifoldEdge count 4 in 30 cells (δ 3…0.1). Base: 15 of those cells. The 15 new ones are the sphere neighbour at [0.5] and the five-point splits.
- The argument's case list assumes a neighbour holds non-adjacent points only on the two SIDES. A neighbour holding both RIMS makes the same rim-to-rim edge a_i–b_j.
- The UNSPLIT body is non-manifold on both trees (54 cells), so the root defect is main's (rectangle lane against a one-strip zone, δ at or above the shell thickness). I found no filing for it; the nearest is `tesslie/two-coincident-edges…`. The PR extends it to split faces, and its pin row `no_walked_triangle_has_an_edge_a_neighbour_makes` has no body of this shape.

**MAJOR-3 (hosted CI, run 44177248088, head 4b9c52c3bc): `gate ok` FAILURE.**
- `test (app)` is red on this PR's own new row: `SLOW mesh::all split_rulings::no_walked_triangle_has_an_edge_a_neighbour_makes 0.564 s (new in this diff: 0.5 s)`. It is not in the slow set.
- `test (kernel)` is red on an existing row at its limit, not this PR's: `sweep::all blend_dual_tangent::the_blend_size_tangent_matches_central_differences 1.013 s (1.0 s)`.
- Success: lint, tools (k-lint, tess-lint, tess-meter), demos, mesh budget meter + falsifier, corrupt input, step import, viewer wasm, python, interval backend. Skipped: interval oracle, prime cache.

**MINOR-4 (DEMONSTRATED by mutant): 7054's re-split is now pinned by two integration rows only.** With `developed_runs` returning no runs, the only red rows are `a_run_on_a_cone_face_cut_by_an_oblique_plane_meshes` and `a_run_on_a_cut_cone_faces_seam_meshes`. Six of 7054's rows are walked and never reach the development; their names and docs still describe runs and anchors, and `.config/nextest.toml`'s comment on `every_run_…` does too:
- `a_run_of_ruling_points_the_development_merges_meshes`
- `every_run_of_a_draft_cones_rulings_meshes` (560 cells, slow set)
- `a_run_at_a_corner_is_anchored_along_its_ruling`
- `a_frustum_seam_split_…`
- `a_draft_cone_ruling_split_…`
- `two_runs_anchored_by_the_apex_mesh`

**MINOR-5 (DEMONSTRATED by mutant): two pieces of `walk_face` are dead or unpinned.** The whole `mesh` lib suite, every `split_rulings` row and my 47k cells are unchanged with either mutant:
- the duplicate-id `continue` disabled (trimmed.rs ~:3579; its comment claims a full-turn seam case that never fires);
- `[false; 3]` passed to `row.cert` in place of the singular flags.

**MINOR-6 (BY-INSPECTION): no 3-D row beside the step functions lands a walked triangle at δ.** `sizing.rs`' new theorem text cites `ruled_walk::tests::the_walk_covers_the_face_once_within_one_chord`, which is combinatorial and in the chart. README `[mesh.sizing]` says "pinned by a test next to the lane's step function".

**MINOR-7 (BY-INSPECTION): the needle refusal has no pin and no scheduled row.** Disclosed, with "until a non-rectangle row exists" as the only schedule. The both-nappe and undecided-apex fallbacks in `walked` have no row either.

**NOTE-8:** the tie mutant (`<=` → `<`) survives; it is equivalent, since either advance is valid at a tie.

**NOTE-9:** refusals in my sweep that are identical on base, so not this PR's:
- a Plane face with two hair-splits at one corner → `UnresolvedWinding`;
- a 0.001° cone split 1e-8 from the apex → `SingularUndecided` through the CDT fallback.

## Claims

1. **Stage-1 table — not re-measured; ruling agreed, by reading and geometry.** I read the prototype (0d78da36bc, ca8d1f6721): cure 1 is the spline arm's centroid insert, budget 400, cones in the development. In an isometric development a side point at slant t lies inside the circumcircle of (apex, a_k, a_k+1) whenever t < R·cos((k+½)h)/cos(h/2). So Delaunay must fan it across columns, and any interior point then needs isotropic spacing. That is a property of Delaunay in the development, not of the prototype's refinement. The cylinder may differ: in the lane's cell metric a column is about a unit square, and cure 1 plausibly needs no insert there; the table does not separate walls from frusta. I would have ruled (3). (2) is the only alternative that survives, and it keeps the development's merge limit.
2. **Every walked mesh correct — held, except MAJOR-1 and MAJOR-2.** My oracle: directed-edge census by id; every polyline segment used exactly twice; every patch boundary edge on a polyline of that face's own edges; winding against my closed-form normal; 66-sample deviation against analytic cylinder, cone and plane; Pappus volume. Zero failures at:
   - default ε: 47,022 meshed cells;
   - ε 1e-6: about 29.6k;
   - ε 1e-12: likewise, but for MAJOR-1's cells.
3. **Within δ by argument — held.** The front-edge invariant is sound, including exhaustion (a row cannot finish early: its last advance needs the other row's next point at the end angle), ties, the apex row and the seam. The cone hull step is right: T·hull(arc), T·sin α ≤ ρ_max. Worst sampled deviation 0.999992 δ (cylinder), 0.9952 δ (cones); never above δ. No `CertificateExceeded` anywhere, so close to δ is by design.
4. **Count relation — held** on every meshed cell (unsplit + gained traversals, read in the same run). The PR's rows pin the relation, not numbers.
5. **Sizing moves only what it should — held on `mesh`'s integration suite.** Not taken: `sweep`, `step-import`. Own digest hook on both trees, `CAD_FUZZ_SEED=1`, 10,404 common (body, δ) keys:

   | class | moved | same | triangles, moved patches |
   |---|---|---|---|
   | class faces | 719 | 0 | 381,005 → 28,581 |
   | neighbours (Plane) | 775 | 1,118 | 47,532 → 36,690 |
   | neighbours (Cone) | 0 | 45 | — |
   | every other face | 0 | 40,976 | — |

   No patch grew. No Ok→Err; one Err→Ok (`SphereTrimShape`). The Ok→Err cases are MAJOR-1's, on my bodies, not the suites'. The thin-planar-neighbour refusal is gone on the slender wall's row.
6. **One reading of "walked" — held.** `trimmed::walked` is read by `face_target` (the chord pass) and by the dispatch with the same arguments; `Rows::of` disagreeing is a hard `MissingEntity`, and I found no reachable divergence. The memo key covers what it reads. Fallbacks to the CDT are the undecided apex and both nappes; no row reaches either.
7. **Ear argument — falsified** (MAJOR-2), and its premise fails on a one-chord apex face (MAJOR-1, where a walked edge joins the two sides). On the PR's own shapes it holds: band 0.2 to 3 rad, D-prism, half cylinder and cone, wedges; 1,302 meshed cells with a clean census.
8. **Residuals and 7054's rows — corrected.** The two renamed rows are honest conversions. See MINOR-4 and MINOR-7. `a_routed_face_is_chorded_for_the_chart_lane` was deleted without mention.
9. **Mutants.** Mine, against `split_rulings`, `reach_probes`' cone row and `curved`:
   - seam sides swapped: red, 7 rows;
   - fan wound backwards: red, 13 rows;
   - tie order: survives (equivalent);
   - apex flags and duplicate-id skip: survive (MINOR-5).
   The PR's four were not re-run.
10. **`[mesh.sizing]` — the implementer's answer is incomplete.** No sentence is quoted anywhere. Made false or incomplete:
    - "the worst triangle is half a grid cell, or a Delaunay sliver…" (a walked face's is neither);
    - "Each lane … chooses its grid step" (the trimmed lane now has two sizings);
    - "pinned by a test next to the lane's step function" (MINOR-6).
11. **Nothing of stage 1 ships — held** (no `ruled_probe`, `CURE`, or new `#[ignore]`). The §5 reader list is true. The stage-1 table lives only in the PR body; the unit file's "Not measured" still says the walk is unbuilt.
12. **CI: MAJOR-3.** Locally, `mesh --test all` whole in release (slow rows included) is 291/291 on head and `mesh --lib` is 232/232. NOT run: viewer slow rows, `sweep`, `step-import`, tess-meter bytes (the hosted tools and meter jobs are green).

## Spec deviations
- Reported by the PR: digests not re-taken after the 7054 merge; needle pin lost; clippy and tools subsets not run.
- SILENT: the deleted row (claim 8); the README sentences (claim 10); the spec's "a split never makes a face refuse", which is not held (MAJOR-1).

## Style
Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (ruled_walk.rs whole; trimmed.rs dispatch and `walk_face`).
- **Q7, likely.** I would have put this strip in one place. The rectangle lane already writes one strip per column for the unsplit face and refuses a side point; the walk is a superset that could write both. As built, "a ruled rectangle" is read four times (`splits_a_straight_side`, `walked`, `Rows::of`, curved's shape door), and the bound is stated twice at each step function (sizing.rs ~:420-441, :505-529).
- **Q7, likely.** `walked` re-derives which arm of `lane_of` fired (ring, trim carrier, split side: trimmed.rs:449-451). Under 7041's one route value per face this is a route variant computed a second time, inside the lane.
- **Q1, sure.** `rim_runs` (ruled_walk.rs:74-78) is repeated verbatim as the `starts` filter in `Rows::of` (:106-108).
- **Q1, sure.** `edge_carrier` (trimmed.rs:488) is a fourth spelling of "an edge's certified carrier", beside `carried_by_a_line`, `has_trim_carrier`'s body and walk.rs:719-725.
- **Q1, likely.** `walk_face`'s per-triangle loop (winding, outward, cert, verdict, worst fold) is a third copy, after the CDT emit pass and curved.rs ~:430-455. I'd look for a fourth in the NURBS arm.
- **Q2, sure.** "No edge of four triangles" is about 30 lines of prose replacing removed code, restated at trimmed.rs:3534 and split_rulings.rs:520; nothing enforces it, and it is wrong in two cases (MAJOR-1, MAJOR-2). The fix that removed the shared-side split minted the defect it argued away.
- **Q2, likely.** `unreachable!` at ruled_walk.rs ~:181 rests on a comment that is false on one-chord apex faces. It happens not to fire.
- **Q2/Q5, sure.** trimmed.rs' module header (:1-300) never mentions the walk and still describes the lane as lattice + CDT. sizing.rs:20-22 says "trimmed cylinder and sphere size at headroom", against :83 as edited here.
- **Q3, likely.** `holds_the_count_relation` drops to `check_winding` alone when `volume_read` is false (no outward, deviation or volume check), and nothing counts how many of the 330 or 154 cells took that arm. Several rows have ε-dependent early `continue`s that read zero bodies at some ε.
- **Q4, sure.** Five work/ files cite renamed or removed rows: `a-thin-arc-…:173`, `a-ruling-split-a-hair…:30`, `a-run-anchored-on-a-needle…:71`, `a-ruling-split-within-the-developments-resolution…:89`, `a-large-right-cone-does-not-revolve…:23`. reach_probes.rs ~:1995 still says the cone met on its axis runs in the development.
- **Q6, sure.** The stage-1 measurement has no home in the tree. The needle pin is disclosed and unscheduled.
- **Q8, unsure.** trimmed.rs is about 6,500 lines with a third dispatch branch appended at its tail; `face_target` now runs `lane_of` and then `walked` (so `splits_a_straight_side` and the shape door twice) per adjacent face of every edge.

## Rubric
- **Idiom/structure: 3.** The module is small and clean; it sits as a second strip-writer and a second route predicate.
- **Test quality: 3.** The relation rows are real and the mutants go red; the ear pin misses the two shapes that break, and the dead branches are unpinned.
- **Doc/comment honesty: 2.** The δ proof is sound; the ear proof and the fan invariant are stated as fact and are false in demonstrated cases; headers are stale.

## Executed / read only
- **Executed** (release, this container): my probe suite on head at three ε; targeted cells and the bored ball on head and on a base worktree; six mutants; the `mesh` integration suite with a digest hook on both trees; `mesh --lib`; hosted check runs and the two failing job logs.
- **Read only:** the stage-1 commits; claims 6, 10, 11; the test-diff part of 8; work/ files.
- **Nothing committed or pushed.** The probe is kept at /tmp/claude-0/-home-user-cad/fee90143-d817-5dc2-aa84-ecd6d6613c3a/scratchpad/rv_probe.rs in this container (it needs seven `split_rulings` helpers made `pub(super)`); `rv_cmp` and `rv_bore` are the two rows worth keeping.
