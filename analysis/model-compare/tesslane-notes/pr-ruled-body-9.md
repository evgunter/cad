Unit 4c of the chart lane (`work/tesslane/a-ruled-face-with-a-split-side-meshes-at-the-density-of-the-face-without-it.md`): splitting one straight edge of a body no longer multiplies the triangles of the cylinder and cone faces beside it, and no longer makes them refuse. A ruled face whose straight side holds a split point is walked, sized at δ, and meshes with the unsplit face's triangles plus one per point its side gained.

**This description was rewritten on 2026-11-22 for the integrated head** (`8ad2d3fd0f`, and the small commits after it). Numbers taken before the integration with main say so where they stand.

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

**The route carries the walk.** `tessellate::route_of` decides each face's lane once per run, and no lane re-derives it. A ruled rectangle with a split straight side (the route's fourth fact) is `Trim::Walked` (`Lane::Walked`) where `trimmed::walkable` answers yes: two runs of rims with no vertex at the apex, or one run whose sides meet at the apex, the apex read by `chart_table::at_a_pole`. `face_target`, the chord pass, the memo key and the dispatch read that value. Split-side faces `walkable` refuses stay `Trim::Structural` on the lattice and the CDT: a loop through the apex onto the other nappe (no body built here makes one), and an apex the band cannot decide.

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
- **The cut over-cuts, and that is a density cost this pull request keeps.** With the cut off, the walk shares an edge only at δ 3, 1 and 0.3 (torus neighbour) and δ 3 to 0.1 (sphere neighbour): fourteen cells over the two split patterns. The rule also cuts at δ 0.1 to 0.04 (torus) and 0.05 to 0.04 (sphere), where the neighbour lays a row between the rims and makes no such side: ten bored-ball cells, e.g. 220 triangles against 134 without the cut and 152 on main. It reads where the neighbour's surface lies, not how the neighbour is meshed. One review measured the same on bodies of its own that no row here holds (a bored torus, a cone-bored ball, a bead), where EVERY cut is an over-cut: against merged main, 256 of its about 2,640 comparable cells are denser here (+2 on the bored ball, +8 on the bored torus and the cone-bored ball at δ 0.1 to 3, +4 to +16 on the bead at δ 0.3 to 3), every one manifold on both trees, and no cell refuses here that meshes on main.
- It also cuts on a one-chord band face beside a wider face of the same cylinder (24 cells of `no_walked_triangle_has_an_edge_a_neighbour_makes`, 2 to 7 cuts each), where a same-surface neighbour cannot be excluded by distance.
- Against merged main, one review counted 440 cells of its own denser here, each by two triangles: 416 bore cells at δ at or past the shell's thickness (62 → 64, 78 → 80, 102 → 104 on the 1.05 ball), and 24 thin D-prism cells (0.5 rad at δ 0.1, 0.2 rad at δ 0.03; 12 → 14). Those 24 are not cuts: the row pins no cut on any D-prism cell and passes there with the cut off; the two triangles are the count relation's own count against main's chart-lane mesh of the same body. On bores of other thicknesses (balls of 1.3 and 3) the rule cuts on about 450 of that review's cells where the census needs none. Over the same 19,426 common cells the head writes 0.80 M triangles against main's 9.76 M.
- The exact cure (cut only what the census reports) would make a face's patch depend on its neighbours' meshes; that is a design question and is filed, not built here.

**Main's own defect, found here.** The UNSPLIT bored ball is the rectangle lane's one strip against a neighbour holding both rims. Before PR 7198 it returned `Ok` with an edge of four triangles; on merged main the census refuses it `OverusedSide` on seven cells (torus neighbour δ 3, 1, 0.3; sphere neighbour δ 3, 1, 0.3, 0.1) and it meshes below. PR 7198's description says the rectangle lane needs no shared-side rule because one chain's points lie on one iso line; two chains held by one neighbour is the case that argument does not cover. Filed: `work/tesslane/a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles.md`.

## The count relation, and its stated exception

A face that holds no interior point has n − 2 triangles over its n boundary points, and a split adds a point to each traversal of its edge. So a walked face and its planar neighbours have the unsplit face's triangles plus the traversals their loops gained. `split_rulings::holds_the_relation` checks this on every cell, with the number of sides cut stated per cell: a walked face owes exactly two triangles per cut point beyond the relation, and every other face owes exactly the relation, so a cut the rule does not ask for is red. Where main refuses the unsplit body, the walked face's baseline is Euler's count on its own patch.

- `a_split_side_adds_one_triangle_per_point`: eleven bodies, every line edge split at 1, 2 and 5 points and every line edge at once, placed obliquely, δ 0.1 to 0.001, at each of the three ε. No cut anywhere.
- `no_walked_triangle_has_an_edge_a_neighbour_makes`: 218 cells of the tightest bodies (a band cut so that a one-chord face and a wide face each hold both of the other's sides; a D prism; a half cylinder and a half cone through the axis; a 0.5 rad wedge; one-chord faces with both sides split); cuts on the 24 band cells only.
- `a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`: the bored ball, both neighbours, both split patterns, δ 3 to 0.03; cut points pinned per cell (torus neighbour 15, 15, 25, 43, 69 at δ 3, 1, 0.3, 0.1, 0.04; sphere neighbour 15, 15, 15, 21, 16; none at 0.035 and 0.03), the same at three ε. A second ball of radius 1.3, whose unsplit body meshes at every δ so that every cut on it is an over-cut: torus neighbour 15, 21, 29 at δ 3, 0.5, 0.25; sphere neighbour 15, 15, 7; none at 0.2 and 0.1.
- `a_side_whose_reach_the_band_cannot_decide_is_cut`: a cell placed inside the band's undecided window at each ε; all 36 sides there are undecided and cut.

## A walked face the lane cannot walk is a kernel bug, and says so by the page

`[failure.taxonomy]` (DESIGN): a state that only a kernel bug can reach is made unrepresentable where a type can carry it (row 0), and otherwise panics with a message (row 4, `unreachable!`); a typed error is for input. A face the route marks walked that the lane then cannot walk is such a state: row 0 where a type can carry it, row 4 otherwise, and no typed error.

An earlier head of this pull request did the opposite: it added a public `TessellateError::RouterDefect` and moved main's seven `MissingEntity { "… (router defect)" }` sites onto it. That was the orchestrator's ruling, made without reading the page, and a delta review caught it. The variant is removed, and main's seven sites are as `origin/main` has them. They are the same deviation, already listed in `work/comb/S43.md`, and are not this unit's to cure. The removal is `826552a0e9`; both reviews held it.

## One certificate verdict, and the K stream

- **`cert::FaceVerdict`** (`read`, `worst`, `refusal`) holds a face's worst bound, whether any triangle exceeded δ, the largest unresolved allowance, and the face's one refusal from them. The rectangle lane, the triangulating arms' emit and the walk read it, where each spelled the accumulation itself before. No byte moved (the digest under "Run"); the ε inventory re-counts `cert.rs`'s carriers, since the band now passes through `read` to `verdict`. What stays per lane is the winding and fold reading: the triangulating arms drop a fold and retry, the walk and the rectangle lane refuse on it.
- **The K stream** (the recorded decisions). On a ruled face with a split side the route also asks `walkable`: one `chart_singularity_at_vertex` per singular point of the chart per outer-loop vertex. The lane records one `walked side beyond its neighbour's reach` per side and face across asked. Both are named at `tessellate::FaceRoute` and pinned by `route_rows::a_split_side_body_records_the_walks_decisions` (a frustum wall: 36 apex decisions, 12 of them `walkable`'s; the bored ball at δ 0.1: 43 reach decisions, one per cut). A memo hit records neither.

## What moved

**On the integrated head against merged main** (`54e6a5c5b8`), two per-patch differentials, one by each reviewer:
- Over `mesh` whole, `step-import` whole and `sweep`'s 63 modules that call `mesh::tessellate` (release, `CAD_FUZZ_SEED=1`): 81,521 patches byte-identical. Moved patches appear only in six split-side tests (`curved`'s multiply-carried iso side and split-then-placed wedge, `sweep`'s `a_box_along_an_inflection_ruling`, `reach_probes`' cone met on its axis, two `split_rulings` rows): cone faces 59,557 → 1,585 triangles, cylinder 5,402 → 940, plane 4,376 → 3,672. No refusal added or removed.
- Over `mesh`'s integration suite with a wider set of split bodies: 39,783 patches of every other face byte-identical; class faces 1,021 moved (cone 703,708 → 38,408 triangles; cylinder 54,802 → 3,890); plane neighbours 1,098 moved (71,306 → 55,358); no patch grew; no refusal added, seven removed.

**Rectangle-lane neighbours** (the implementer's digest, on the fix head before the integration, classed by lane): 24 patches on the rectangle lane (16 cone, 8 cylinder), each sharing a rim with a walked face, all in one row at δ 0.1, came out one or two triangles coarser (12 or 13 → 11; 11 → 10). The walked face no longer asks the shared rim for the headroom's chords, so its neighbour across that rim gets the rim the unsplit body gives it. The orchestrator accepted these as in scope.

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

On the final code head `e760867739`, over `split_rulings`, `route_rows`, `errors`, `ruled_walk`, `sizing` and `reach_probes` (91 rows, the `--lib` units included); each is red under its own rows.

| mutant | red |
|---|---|
| the zip off | 4: the unit row's exact triangle list, and three `split_rulings` rows |
| the shared side never cut | 4: the bored-ball row by `OverusedSide { uses: 4 }` (the census is the backstop), the 218-cell row's pinned band cuts, the undecided-reach row, the walk's K-stream row |
| the shared side always cut | 3: the bored-ball row's uncut cells, the undecided-reach row, the K-stream row |
| undecided read as greater | 1: `a_side_whose_reach_the_band_cannot_decide_is_cut` |
| the route's walked fact off | 11: ten `split_rulings` rows and the K-stream row |
| the walk reached by a face the route did not mark | 22: 17 `reach_probes` rows, `route_rows`' K stream and bytes, the notched wall and the needle row |
| the headroom kept on a walked face (earlier head) | 7 |
| PR 7054's re-split off (reviews, integrated head) | 5 |
| the zip's order reversed; the cut's threshold at δ alone (a review's) | 2; the bored-ball row |

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

**On the integrated head** (`8ad2d3fd0f`, then `469c4b6c54`; main `54e6a5c5b8` merged):
- `scripts/run-gates.sh`, fmt, clippy for the workspace (all features) and for `pncad-py`: clean.
- `mesh` whole with the slow set at `469c4b6c54`: 546/546 at the default ε, 10⁻⁶ and 10⁻¹², by the implementer and again by a reviewer.
- A reviewer's release runs at `8ad2d3fd0f`: `step-import` with three reds (`nist_ftc_09`'s snap relation in band) and `sweep`'s meshing modules with one (`bisector_nodes::the_near_node_poses_at_the_witness_band`), all four identical on merged main.
- On `826552a0e9` (the router-defect removal): clippy for `mesh` and `pncad-py`, gates, and the 134 rows of the files it touches.
- On the final code head `e760867739`: gates, fmt, clippy for the workspace (all features), `mesh`'s rustdoc; `mesh` whole with the slow set, 546/546 at the default ε, 10⁻⁶ and 10⁻¹² (one inventory pin re-counted for the shared verdict type, that commit); the mutants below; `test_binding_census` against a fresh wheel, 12/12; no panic on any cell across the apex's band (81 split gaps at two δ and two ε).
- A `mesh`-only patch digest of `e760867739` against `8ad2d3fd0f`: 58,154 patches in common, every one byte-identical, no refusal added; 102 patches are new, all from rows the last commits add.
- The viewer's slow-set rows, 12/13, and `sweep`'s meshing slow rows, 143/144: both reds are main's, identical on `54e6a5c5b8` (`review_pick_r2`'s pick tally; `bisector_nodes::the_near_node_poses_at_the_witness_band`, an escalation in the boolean before any mesh).
- The closing head `a020a3edca` is that code with main merged once more (no textual conflict; main moved nothing under `crates/mesh`; `cargo check -p mesh --tests` passes) and tracker and record commits.
- Hosted CI: no head between `7a6f5bfe6e` and the closing head completed a run; runs queued 20 to 45 minutes on 22 November and each was superseded by the next push before a job started. The merge waits on the closing head's run.

**On the fix head before the integration `7a6f5bfe6e`:** gates, fmt, the doc gate; clippy for the workspace, the viewer's app feature and `pncad-py`; the Python suite against a wheel built locally, 960 tests; `mesh` whole with the slow set, 539/539 at ε 10⁻⁶ and at 10⁻¹²; `mesh` and `geom-brep` whole at the default ε, 3293/3294, the red a random-seed fuzz row of `geom-brep` (`curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance`, seed `0x2809be1edc2b5ff5`) in code this branch does not touch; the three `tools` roots; the viewer's `display_budget` and `review_pick_r2` rows, 13/13.

Hosted CI on `7a6f5bfe6e` was green but for `test (kernel)`, red on a random-seed fuzz row of `profile` this branch does not touch (commented below).

## Review

Dual tier: two concurrent reviews, isolated from each other.
- **First reviews**, on the frozen head `4b9c52c3bc`. One: not mergeable as it stood, three MAJORs (a one-chord cone face to its apex with both sides split refused where the base meshed; the bored ball's edge of four triangles; hosted CI red on this pull request's own row). The other: approve with fixes, no MAJOR. Every finding of both was taken.
- **Fix pass** to `7a6f5bfe6e`: the zip; the shared-side cut and its narrowing; the route carrying the walk, after PR 7041 merged; PR 7054's behaviours re-pinned; the smaller findings.
- **Delta reviews** on `7a6f5bfe6e`. One finished: approve with fixes, both MAJORs cured, nothing outside the class moved, the over-cut measured. The other was cut off by an outage after its interim.
- **The integration.** Main moved about 2,500 commits in four days and PR 7198 rebuilt the shared-side ground; the branch integrated it (`8ad2d3fd0f`).
- **Second deltas**, by both, on the integrated head: both approve with fixes, no MAJOR. Both first MAJORs are cured there; no walked body reaches main's census or shared-side refusal; the lift leaves main's arms unchanged; the merge lost nothing of either side. Their findings: the router-defect variant against the ratified taxonomy (removed, `826552a0e9`, held by both); the over-cut, kept and stated; twelve thin D-prism cells that mesh on main and refuse here, kept and stated; this description's stale lines (corrected); `may_make`'s rounding premise unstated at its site; stale tracker citations.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01BJC3B1cF4p5C8uaoTcK91p
