Unit 4c of the chart lane (`work/tesslane/a-ruled-face-with-a-split-side-meshes-at-the-density-of-the-face-without-it.md`): splitting one straight edge of a body no longer multiplies the triangles of the cylinder and cone faces beside it, and no longer makes them refuse. A ruled face whose straight side holds a split point is walked, sized at δ, and meshes with the unsplit face's triangles plus one per point its side gained.

**This description was rewritten on 2026-11-22 for the integrated head `8ad2d3fd0f`.** Numbers taken before the integration with main say so where they stand; the runs on the integrated head are added under "Run" as they finish.

## Why this cure: three were measured first

The unit measured three cures over about 1,660 cells at the default ε (1,040 at 10⁻⁶, 1,714 at 10⁻¹²: the same picture) and stopped for a ruling before building one.

| cure | meshes (walls / cones / frustums) | worst triangles against the same face without the split | notes |
|---|---|---|---|
| the chart lane before this unit | 530 of 590 / 615 of 635 / 295 of 315 | 196× on walls; 17,618× on a 0.01° cone | the defect |
| 1. no rows; the triangulation left alone; refine where the certificate refuses | 501 of 590 / 568 of 635 / 295 of 315 | 24× on walls, 102× on cones | adds `Fold`, `Triangulation` and `CertificateExceeded` refusals; does not converge on a needle wall in 400 passes |
| 2. no rows, plus one held edge per split side | all but the cells the cone's development merges (55 / 10 / 20) | 2.4× | the same triangle count as the walk |
| 3. the walk | 590 of 590 / 635 of 635 / 315 of 315, and every cell at the other two ε | 2.25× | never enters the cone's development |

**What stage 1 measured, and what it did not.** Cure 1 did not converge on the class, but what failed to converge was one rule: the centroid insert, at 400 passes, with the lattice's metric. A centroid insert cannot shorten the angular span of the edge the certificate reads, so this does not show that certificate-driven refinement in general fails. Cure 2's refusals came from the development's 27-bit merge (`DevelopedMerge`), not from holding the edges. Two cures were not measured: a refinement that splits the offending edge, and cure 2 run in the chart instead of the development.

One finding cuts across all three: sizing the face at δ instead of at the lane's headroom is what stops a planar neighbour narrower than δ from refusing, whatever makes the triangles. The orchestrator ruled cure 3 on these numbers. Every triangle the walk writes is still accepted or refused by the same per-triangle certificate and winding rule as any other.

## The defect

A cylinder or cone face whose straight side holds a vertex (`walk::splits_a_straight_side`; booleans and `split_edge` make these) is the chart lane's. That lane laid lattice rows such a face does not need and sized it at its unproved headroom. A split therefore multiplied the face's triangles by up to 196× in the suites' bodies, and by 17,600× on a 0.01° cone. It also refused faces the split did not make harder: `DevelopedMerge` where two ruling points fall within the development's 27-bit cut, `FoldedTriangle` beside a split a hair from an end, and `Triangulation` on a planar neighbour narrower than δ.

## The cure

**The walk** (`crate::ruled_walk`). No lattice and no triangulation step: the face's triangles join its two rows of boundary points (its rims, or a rim and the apex) in order of position along the rulings, and each split side's points are fanned from the corner opposite that side. The fans take their triangles by construction: the first walked triangle holds the start side and the last the end side. On a one-chord face to the apex with both sides split the one walked triangle holds both sides, and `Rows::zip_sides` joins the two sides' points to each other in order of distance from the apex (k + m + 1 triangles for k and m side points).

**The route carries the walk.** `tessellate::route_of` decides each face's lane once per run, and no lane re-derives it. A ruled rectangle with a split straight side (the route's fourth fact) is `Trim::Walked` (`Lane::Walked`) where `trimmed::walkable` answers yes: two runs of rims with no vertex at the apex, or one run whose sides meet at the apex, the apex read by `AnalyticRow::is_singular`. `face_target`, the chord pass, the memo key and the dispatch read that value. Split-side faces `walkable` refuses stay `Trim::Structural` on the lattice and the CDT: a loop through the apex onto the other nappe (no body built here makes one), and an apex the band cannot decide.

**Sizing.** A walked face sizes at δ, not at the headroom, by a theorem stated at `sizing::sagitta_step` and `sizing::cone_grid_step` and in `ruled_walk`'s module doc:
- the walk's front edge keeps `max(a_i, b_j) ≤ min(a_{i+1}, b_{j+1})`, so every triangle's corners lie within one chord of one row in angle (the apex is a row of one point);
- a triangle on rulings within `Δu` lies in the hull of the surface between the end rulings, so its deviation is that column's sagitta, the rectangle lane's bound;
- a side's points lie on its ruling, so the triangles they fan or zip divide one walked triangle in its plane.

`sizing::tests::a_walked_triangle_lands_at_delta` lands the real walk at the solved step in 3-D on a cylinder, a frustum and a face to the apex: every triangle within δ, the worst at δ to 1%. It does not exercise the zip or a cut side.

## The shared side

A walked side joins a point of one rim to a point of the other. Where ONE neighbouring face holds both rims (the sphere or torus face round a bore through a ball), that neighbour's own triangles can join the same two points, and the side is then an edge of four triangles. The first review demonstrated it: a ball of radius 1.05 with a coaxial bore of radius 1, its seam split.

**What main has since** (PR 7198, merged while this pull request was in review): one shared-side rule on every trimmed arm that triangulates (a kept triangle may not have a side joining two polygon points that are not loop neighbours and that one face across the loop's edges holds both of; a point is inserted and the face rebuilt), and a census where patches join, which refuses `OverusedSide` on any mesh with a side used past twice per chord.

**What this pull request does with it:**
- One test of "one face across holds both ends": main's inline check, lifted unchanged into `TrimPolygon::faces_holding_side`. Main's arms read whether it is empty, as before; the walk reads the faces it returns.
- A walk has no triangulation to rebuild, so its cure is its own: it cuts the side at the surface point at its chart midpoint, and each triangle on the side becomes two between the same rulings.
- The walk cuts only where the neighbour could make the side (`trimmed::may_make`). A triangle the neighbour emits passes its certificate, so it lies within δ·(1 + `cert::ALLOWANCE_SHARE`) of the neighbour's surface. Where the side's midpoint is decided farther than that from the neighbour's surface (`geom::Surface::foot`; `geom_core::k_stats::decide` on the run's band), the neighbour cannot hold the side and it is not cut. Decided not farther, undecided, or a surface with no closed-form foot: cut. There is no margin constant in `mesh`.
- The census is the backstop and never the cure: no walked body reaches `OverusedSide` on any cell run. With the walk's cut switched off, the census refuses on exactly the cells that need it (below).

**The bored ball, measured** (radius 1.05, bore 1, placed obliquely; default ε; triangle totals for the body; "main" is merged main without the walk, where the chart lane and PR 7198's rule mesh the split body):

| seam split at its middle | δ 3 | 1 | 0.3 | 0.1 | 0.05 | 0.04 | 0.035 | 0.03 |
|---|---|---|---|---|---|---|---|---|
| torus neighbour: main | 62 | 62 | 102 | 152 | 216 | 242 | 330 | 358 |
| torus neighbour: the walk with its cut (the fix head before the integration) | 64 | 64 | 104 | 220 | 310 | 350 | 298 | 322 |
| sphere neighbour: main | 62 | 62 | 62 | 64 | 120 | 134 | 142 | 152 |
| sphere neighbour: the walk with its cut | 64 | 64 | 64 | 88 | 120 | 136 | 110 | 116 |

- The walk is coarser than main at δ ≤ 0.035, where the rule proves no cut is needed and the face sizes at δ.
- **The cut over-cuts, and that is a density cost this pull request keeps.** With the cut off, the walk shares an edge only at δ 3, 1 and 0.3 (torus neighbour) and δ 3 to 0.1 (sphere neighbour): fourteen cells over the two split patterns. The rule also cuts at δ 0.1 to 0.04 (torus) and 0.05 to 0.04 (sphere), where the neighbour lays a row between the rims and makes no such side: ten bored-ball cells, e.g. 220 triangles against 134 without the cut and 152 on main. It reads where the neighbour's surface lies, not how the neighbour is meshed. The second delta review measured the same on bodies of its own (a bored torus, a cone-bored ball, a bead): 208 of about 1,100 cells denser than the base, every one manifold on both trees.
- It also cuts on a one-chord band face beside a wider face of the same cylinder (24 cells of `no_walked_triangle_has_an_edge_a_neighbour_makes`, 2 to 7 cuts each), where a same-surface neighbour cannot be excluded by distance.
- The exact cure (cut only what the census reports) would make a face's patch depend on its neighbours' meshes; that is a design question and is filed, not built here.

**Main's own defect, found here.** The UNSPLIT bored ball is the rectangle lane's one strip against a neighbour holding both rims. Before PR 7198 it returned `Ok` with an edge of four triangles; on merged main the census refuses it `OverusedSide` on seven cells (torus neighbour δ 3, 1, 0.3; sphere neighbour δ 3, 1, 0.3, 0.1) and it meshes below. PR 7198's description says the rectangle lane needs no shared-side rule because one chain's points lie on one iso line; two chains held by one neighbour is the case that argument does not cover. Filed: `work/tesslane/a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles.md`.

## The count relation, and its stated exception

A face that holds no interior point has n − 2 triangles over its n boundary points, and a split adds a point to each traversal of its edge. So a walked face and its planar neighbours have the unsplit face's triangles plus the traversals their loops gained. `split_rulings::holds_the_relation` checks this on every cell, with the number of sides cut stated per cell: a walked face owes exactly two triangles per cut point beyond the relation, and every other face owes exactly the relation, so a cut the rule does not ask for is red. Where main refuses the unsplit body, the walked face's baseline is Euler's count on its own patch.

- `a_split_side_adds_one_triangle_per_point`: eleven bodies, every line edge split at 1, 2 and 5 points and every line edge at once, placed obliquely, δ 0.1 to 0.001, at each of the three ε. No cut anywhere.
- `no_walked_triangle_has_an_edge_a_neighbour_makes`: 218 cells of the tightest bodies (a band cut so that a one-chord face and a wide face each hold both of the other's sides; a D prism; a half cylinder and a half cone through the axis; a 0.5 rad wedge; one-chord faces with both sides split); cuts on the 24 band cells only.
- `a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`: the bored ball, both neighbours, both split patterns, δ 3 to 0.03; cut points pinned per cell (torus neighbour 15, 15, 25, 43, 69 at δ 3, 1, 0.3, 0.1, 0.04; sphere neighbour 15, 15, 15, 21, 16; none at 0.035 and 0.03), the same at three ε.

## One router-defect refusal

`TessellateError::RouterDefect { face, what }` ("the route misread a face (a kernel defect)") is new, for a face the route marks walked that the lane cannot walk. No body reaches it. Main reported its own router defects as `MissingEntity { what: "… (router defect)" }`, whose text says a key the body's topology references failed to resolve and that the input is corrupt. This pull request moves those sites onto the new variant too, nine in all, each carrying its face: `curved.rs` (a ringed face, a pinched face, a side count), `chords.rs` (no rectangle; an uncertified half-edge), `walk.rs` (an uncertified half-edge; a rim with no row) and the walk's two. Tag, censuses and the Python binding follow; the Python projection drops the face, as every variant's does.

**Seam with TESSLIE.** Those sites are in files TESSLIE holds, and the move is the split its open row asks for (`work/tesslie/tessellate-missing-entity-renders-every-detail-as-a-key-that-failed-to-resolve.md`, which stays open for the cache misses, the absent chart and `LoopEdgesError::Corrupt`; a dated line is on it). `scripts/work.py territory` lists 16 paths in other programs' territory: the `mesh` files both programs claim, and `pncad-py`'s projection, tag list and binding census.

## What moved

Patch digests, the merge base `54938d6a95` against the fix head before the integration (`3f0f9434a8`, mesh code as `7a6f5bfe6e`), over `mesh`, `sweep` and `step-import` whole, every tessellate call hashing each patch. **Not yet re-taken on the integrated head.**

- 86,190 base patches. No patch that the base meshes is refused, and no refusal changed.
- Moved, in six tests (`curved`'s multiply-carried iso side and split-then-placed wedge, `sweep`'s `a_box_along_an_inflection_ruling`, `reach_probes`' cone met on its axis, and two `split_rulings` rows): 130 walked patches, 172 plane neighbours, and **24 patches on the rectangle lane (16 cone, 8 cylinder), each sharing a rim with a walked face**, all in one row at δ 0.1, each one or two triangles coarser (12 or 13 → 11; 11 → 10). The walked face no longer asks the shared rim for the headroom's chords, so its neighbour across that rim gets the rim the unsplit body gives it. The orchestrator accepted these as in scope.
- The second delta review's own differential on the same heads: 80,639 patches byte-identical; cone faces 59,557 → 1,585 triangles, cylinder 5,544 → 940, plane 4,376 → 3,672; no refusal added or removed.
- Two fuzz rows move on unseeded draws and are not comparable.

## Where a split still changes an answer

The unit's spec said a split never makes a face refuse. It does not hold everywhere; each case is pinned, and no face in them is walked:
- **A run anchored by the apex on a needle triangle.** A full-turn 2° cone, cut near its rim with its apex kept, its first ruling split a hair beside the section end, is triangulated in its development and refuses `UnresolvedWinding`. Measured cells: at the default ε a gap of 10⁻⁸ refuses at δ 0.01 and meshes at δ 10, 1 and 0.1; at ε 10⁻¹² a gap of 10⁻¹⁰ refuses at every δ from 10 to 0.01; at ε 10⁻⁶ the smallest split `split_edge` admits (10⁻⁵) meshes. No wider family was measured. Row `a_run_anchored_by_the_apex_on_a_needle_refuses_unresolved`; item `a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind`.
- **A split beside an apex the band cannot place.** A 0.001° cone through its apex, a ruling split 10⁻⁸ from the apex end: `walkable` answers no, and the lattice and CDT refuse `SphereTrimShape { SingularUndecided }` at δ 0.1 and 0.01. At ε 10⁻¹² the band decides, the face is walked, and it meshes. Row `a_split_beside_the_apex_of_a_needle_cone_refuses_undecided`.
- **A D prism thinner than δ, with a cap's straight chord split** (found by a delta review; 12 cells: 0.5 rad at δ 0.1 and 0.2 rad at δ 0.03, every line edge split, six split patterns each). These mesh on main and refuse here, and that is a regression this pull request keeps and states. At such a δ the arc of each cap is one chord, so the cap is a planar face of no area, and with a split point on its straight chord the planar lane meets three collinear points and refuses (`UnresolvedWinding`, or `Triangulation` at five points). The refusing face is the cap, not the walked face. The unsplit body meshes on both trees, and the body with only a cap's chord split refuses on both trees: main meshes the all-edges-split body only because the chart lane's headroom chorded the ruled face's rim twice, which gave the cap an area. The class is the open row `work/tesslane/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md` (a valid thin arc-bounded planar face refuses at a coarse δ); its cure is the planar lane's. Pin and a dated line on that row: with the fix pass's last items.

## `[mesh.sizing]` sentences that now say less than the code does

`crates/mesh/README.md` is not edited here; the clause came through Ev's channel and its wording goes to Ev as its own pull request.
1. "Each lane (curved, trimmed, NURBS) chooses its grid step from a bound on its own worst triangle, and takes the coarsest step that bound allows." The walk has no grid; it takes each rim's chord step from the rectangle lane's bound.
2. "Depending on the lane, the worst triangle is half a grid cell, or a Delaunay sliver beside a point the grid did not place (a trim or chord point)." The walk's worst triangle is neither: it is a triangle whose corners lie on two rulings at most one chord apart on each rim.
3. "The theorem is pinned by a test next to the lane's step function." The walk has no step function of its own; it reads `sizing::sagitta_step` and `sizing::cone_grid_step`, and its test sits beside those.

## PR 7054's rows

Six of that pull request's eight rows are on bodies the walk now takes, so they no longer reach the development's re-split. Four are renamed to what they assert (`a_draft_cones_seam_split_within_its_developments_resolution_meshes`, `every_close_split_of_a_draft_cones_rulings_meshes`, `a_split_beside_a_corner_of_a_draft_cone_meshes`, `a_one_chord_face_to_the_apex_split_beside_both_rim_ends_meshes`); two keep their names with docs restated. The re-split's behaviours are pinned again on cone faces that still reach the development (a cone cut near its rim with its apex kept): a run of three, two runs anchored by the apex, the anchor along the ruling, and the needle refusal. With `developed_runs` returning no runs, five rows go red (two at the frozen head).

A deleted row: `a_routed_face_is_chorded_for_the_chart_lane`. What it held (a routed face's neighbours are chorded for its lane) is asserted on every cell by the count relation, which a face chorded for the wrong lane fails.

## Mutants

On the fix head before the integration; re-run on the integrated head with the report.

| mutant | red |
|---|---|
| the zip off | 4 rows, the unit row's exact triangle list among them |
| the zip's order reversed (delta review) | 2 |
| the route's walked fact off | 8 `split_rulings` rows |
| the walk reached by a face the route did not mark | 19 across `reach_probes`, `route_rows` and `split_rulings` |
| the shared side never cut | the bored-ball row (on the integrated head, by `OverusedSide`) |
| the shared side always cut | the bored-ball row's exact cells |
| the cut's threshold at δ alone (delta review) | the bored-ball row |
| the headroom kept on a walked face | 7 |
| PR 7054's anchor / run of three / two runs / re-split | each its own row; re-split off, 5 |
| undecided read as greater | being built |

## Sweep (§5)

- **Readers of `walk::splits_a_straight_side`:** `tessellate::route_of`'s fourth fact, once.
- **Sites that decide a face is walked:** `trimmed::walkable`, called by the route only.
- **Sites that lay a lattice on a ruled face:** `trimmed`'s lattice, reached now only by ruled faces with a ring or a trim carrier and by split-side faces the walk does not take.
- **Sites that test a shared side:** `TrimPolygon::faces_holding_side`, read by main's arms and by the walk.
- **Blind spot:** a ruled face with a ring or trim carrier and a split side keeps the CDT (measured on T1 at δ 0.02: 300 triangles against 299).
- **Not searchable here:** a both-nappe cone face.

## Filed and closed

- Filed: `work/tesslane/a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles.md` (main's unsplit bore).
- Closed by this unit: `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`, `a-ruling-split-a-hair-from-its-end-folds-a-triangle`.
- Dated notes on `a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind` (its row is on the cut-cone family now) and on `a-needle-cone-through-its-apex-takes-most-of-an-hour-to-mesh-one-cell` (the head answers that cell in about 0.1 s; the item stays open).

## Run

**On the integrated head `8ad2d3fd0f`** (main `54e6a5c5b8` merged): `scripts/run-gates.sh`, fmt, clippy for the workspace (all features) and for `pncad-py`: clean. `split_rulings`, `errors` and `route_rows` pass at the default ε, 10⁻⁶ and 10⁻¹². The whole-suite runs, the mutants and the digests on this head are still to come and are added here.

**On the fix head before the integration `7a6f5bfe6e`:** gates, fmt, the doc gate; clippy for the workspace, the viewer's app feature and `pncad-py`; the Python suite against a wheel built locally, 960 tests; `mesh` whole with the slow set, 539/539 at ε 10⁻⁶ and at 10⁻¹²; `mesh` and `geom-brep` whole at the default ε, 3293/3294, the red a random-seed fuzz row of `geom-brep` (`curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance`, seed `0x2809be1edc2b5ff5`) in code this branch does not touch; the three `tools` roots; the viewer's `display_budget` and `review_pick_r2` rows, 13/13.

Hosted CI on `7a6f5bfe6e` was green but for `test (kernel)`, red on a random-seed fuzz row of `profile` this branch does not touch (commented below).

## Review

Dual tier: two concurrent reviews, isolated from each other.
- **First reviews**, on the frozen head `4b9c52c3bc`. One: not mergeable as it stood, three MAJORs (a one-chord cone face to its apex with both sides split refused where the base meshed; the bored ball's edge of four triangles; hosted CI red on this pull request's own row). The other: approve with fixes, no MAJOR. Every finding of both was taken.
- **Fix pass** to `7a6f5bfe6e`: the zip; the shared-side cut and its narrowing; the route carrying the walk, after PR 7041 merged; PR 7054's behaviours re-pinned; the smaller findings.
- **Delta reviews** on `7a6f5bfe6e`. One is final: approve with fixes, both MAJORs cured, nothing outside the class moved, the over-cut measured, this description stale (rewritten here). The other was cut off by an outage after its interim (both MAJORs cured on its cells; the over-cut; the D-prism cells) and is owed.
- **Since then** main moved about 2,500 commits and PR 7198 rebuilt the shared-side ground; the branch integrated it (`8ad2d3fd0f`). Both reviewers re-read the integrated head.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01BJC3B1cF4p5C8uaoTcK91p
