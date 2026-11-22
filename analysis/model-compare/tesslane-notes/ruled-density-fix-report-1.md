(4c implementer) REPORT, part 1 of 2. The digests come in part 2, when the head run finishes (it is running alone now).

**Fix head: 7a6f5bfe6e** (pushed). It merges origin/main at 54938d6a95, which includes 7041.

**Per item: what changed, and where**
- MAJOR-1 (a one-chord apex face with both sides split): `ruled_walk::Rows::zip_sides` writes k+m+1 triangles. Rows: `ruled_walk::tests::a_one_chord_face_to_the_apex_zips_its_sides` (asserts the exact triangle list) and `split_rulings::a_one_chord_face_through_the_apex_with_both_sides_split_meshes`.
- MAJOR-2 (the bored ball's four-use edge): `trimmed::walk_face` cuts a side at its chart midpoint where one neighbouring face holds both ends (`faces_across`, adjacency by mesh id) and may make the side (`trimmed::may_make`). Row: `split_rulings::a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`; it cuts at δ ≥ 0.04 and is exact at δ ≤ 0.035. Main's unsplit-bore defect is filed in work/tesslane.
- Your three on the red heads (3f0f9434a8):
  - pncad-py projects RouterDefect, and the binding census lists it (7a6f5bfe6e).
  - `may_make` is a three-way decision: `k_stats::decide(Margin::of(|m−foot| − δ(1+ALLOWANCE_SHARE)), tol.band)`. Decided Positive means no cut; anything else cuts. There is no margin constant in mesh. The `SizingTols::band` doc now names it as the fourth consumer.
  - main is merged.
- B (merge 7041; "walked" carried in the route value): `tessellate::{Trim::Walked, Lane::Walked, Ready::Walked}`. The fourth fact is `trimmed::walkable`. Dispatch passes `lane == Lane::Walked` to `tessellate_trimmed`. memo.rs and chords.rs have their arms. `TessellateError::RouterDefect` covers a walked face the lane cannot walk (a spline chart, or no `Rows::of`).
- The narrower shared-side test (`may_make`): built as above.
- C1 and C7: the PR-body paragraphs below.
- C2/C3 (7054's behaviours on the cut family): `cut_apex_cone` and `split_beside`. The rows are run-of-three, two-runs-anchored-by-the-apex, the needle (refuses Unresolved) and the undecided apex (refuses SingularUndecided). Each is red under its own 7054 mutant: anchor, three, two and resplit.
- C4–C6:
  - ruled_walk's invariant states the m=0 row and the zip.
  - trimmed.rs's header says the walked class is walked at δ, `may_make` cuts, and other analytic faces size at UNPROVED_HEADROOM.
  - The needle-hour item and the needle item have dated lines.

**K-stream answer: no re-pin.** `walkable` adds one `chart_singularity_at_vertex` decision per loop vertex, on cone fact-4 faces only. route_rows' k-stream row is unchanged and passes. `may_make`'s new decide runs inside the lane, and every route_rows row passes on the head.

**D**
- gates (`bash scripts/run-gates.sh`) rc=0; fmt clean; doc gate (--workspace-pass) OK.
- clippy -D warnings clean: the workspace (all-features, viewer excluded), viewer --features app, and pncad-py --features python.
- The python suite (CI's unittest discover, against a wheel built here): 960 OK.
- mesh+geom-brep whole: 3293/3294. The red is geom-brep's window-oracle fuzz (CAD_FUZZ_SEED=0x2809be1edc2b5ff5), in code this branch does not touch. The torus-row red from an earlier run is already filed (work/splice/the-torus-rows-door-answer-panics-on-a-ring-torus-convention-refusal.md).
- Tools roots (k-lint, tess-lint, tess-meter): fmt, clippy and tests all green.
- Changed rows (split_rulings, route_rows, errors, ruled_walk, sizing, eps inventory, wedge) at the default ε, 1e-6 and 1e-12: 75/75 each.
- Slow-set rows: viewer display_budget + review_pick_r2 13/13; sweep's snowman row passes.

**Mutants, each red under its own rows:**
- zip off: 4 red, including the exact-triangles unit row.
- route's walked fact off: 8 split_rulings rows red.
- shared side never cut: the bored-ball row red.
- shared side always cut: the bored-ball row red (its Exact cells at δ ≤ 0.035).
- walk reached by a face the route did not mark (`Lane::Trimmed` passed as walked): 19 red across reach_probes, route_rows (including the k-stream row) and split_rulings.

**What still refuses** (each pinned):
(a) The needle triangle, on a cut cone that keeps its apex. A full-turn 2° cone with its first ruling split a hair beside the section end is not walked, and it refuses UnresolvedWinding:
  - default ε: a 1e-8 gap refuses at δ 0.01;
  - 1e-12: a 1e-10 gap refuses at every δ from 10 to 0.01;
  - 1e-6: meshes.
  It is maximised by the narrowest cone with the smallest gap.
(b) An apex the band cannot place. A 0.001° cone with a split 1e-8 from its apex is not walked, and it refuses SphereTrimShape{SingularUndecided} at δ 0.1 and 0.01 at the default ε. It meshes at 1e-12.
(c) Within the walked class, the bored ball's band face against a same-surface neighbour: the band's +4 triangles remain (two cuts). This is extra density, not a refusal.

**Changed PR-body paragraphs:**
=====
**What stage 1 measured, and what it did not.** Cure 1 (refine the chart lane where a triangle's certificate fails) did not converge on the class, but what failed to converge was one rule: the centroid insert, at 400 passes, with the lattice's metric. That does not show certificate-driven refinement in general fails. Cure 2 (keep the split edges and mesh the strip between them) refused, but the refusals came from the development's 27-bit merge (`DevelopedMerge`), not from holding the edges. Two cures were not measured: a refinement that splits edges, and cure 2 run in the chart instead of the development.
=====
**`[mesh.sizing]` sentences that now say less than the code does** (`crates/mesh/README.md`; this pull request does not edit the README. The wording is for Ev.)

1. "Each lane (curved, trimmed, NURBS) chooses its grid step from a bound on its own worst triangle, and takes the coarsest step that bound allows." The walk has no grid. It takes each rim's chord step from the rectangle lane's bound. Proposed: "Each lane (curved, trimmed, NURBS) chooses its step from a bound on its own worst triangle, and takes the coarsest step that bound allows: a grid step, or, on a ruled face the trimmed lane walks, the chord step of each rim."
2. "Depending on the lane, the worst triangle is half a grid cell, or a Delaunay sliver beside a point the grid did not place (a trim or chord point)." This leaves out the walk's worst triangle. Proposed: "... or a Delaunay sliver beside a point the grid did not place (a trim or chord point), or a walked triangle, whose corners lie on two rulings at most one chord apart on each rim, so that it lies in one column of the rectangle lane's grid."
3. "The theorem is pinned by a test next to the lane's step function." The walk has no step function of its own. It reads `sizing::sagitta_step` and `sizing::cone_grid_step`. Its proof is in `ruled_walk`'s module doc, and its test (`sizing::a_walked_triangle_lands_at_delta`) sits beside those two functions. Proposed: "The theorem is pinned by a test next to the step function the lane reads."
=====
**A deleted row.** `a_routed_face_is_chorded_for_the_chart_lane` is gone. It pinned that a face the route sends to the chart lane is chorded for that lane. The count relation now checks this on every cell of `a_split_side_adds_one_triangle_per_point` and `no_walked_triangle_has_an_edge_a_neighbour_makes`: a face chorded for the wrong lane does not give the unsplit face's count plus one per side point.
=====
**"A split never makes a face refuse" does not hold everywhere, and two families are the counter-cells.** Both are pinned, and neither face is walked:
- **A run anchored by the apex on a needle triangle.** A full-turn cone of half-angle 2°, cut near its rim with its apex kept, has its first ruling split a hair beside the section end. The chart lane still triangulates the face in its development, and the face refuses `UnresolvedWinding`. At the default ε, a gap of 10⁻⁸ refuses at δ 0.01 only. At 10⁻¹², a gap of 10⁻¹⁰ refuses at every δ from 10 to 0.01. At 10⁻⁶, the smallest split `split_edge` admits (10⁻⁵) meshes. Row: `a_run_anchored_by_the_apex_on_a_needle_refuses_unresolved`. Item: `work/tesslane/a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind.md`.
- **A split beside an apex the band cannot place.** A cone of half-angle 0.001° through its apex has a ruling split 10⁻⁸ from the apex end. The band does not decide whether the split vertex is the apex, so `walkable` answers no. The lattice and the CDT then refuse the face `SphereTrimShape { SingularUndecided }` at δ 0.1 and 0.01. At 10⁻¹² the band decides, the face is walked, and it meshes. At 10⁻⁶ the cone's profile does not validate. Row: `a_split_beside_the_apex_of_a_needle_cone_refuses_undecided`.
=====
**The needle pin moved.** The bodies this item pinned are now walked, and they mesh (`a_split_beside_a_large_cones_rim_end_meshes` replaces that pin). The needle is now pinned where it still refuses, on a cut cone that keeps its apex (`cut_apex_cone`, `a_run_anchored_by_the_apex_on_a_needle_refuses_unresolved`).
=====
