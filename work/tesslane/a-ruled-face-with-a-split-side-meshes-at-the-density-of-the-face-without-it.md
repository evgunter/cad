---
id: a-ruled-face-with-a-split-side-meshes-at-the-density-of-the-face-without-it
kind: unit
title: A cylinder or cone face with a split side meshes at the density of the same face without it; the cures are measured, then one is built
status: closed
opened: 2026-11-17
priority: P1
cost: H
branch: tesslane/ruled-density
closed: 2026-11-22
pr: 7096
---

Unit 4c of the chart lane (`work/tesslane/plan.md`). It follows the design fork of 2026-11-17 (row 279 of `docs/DESIGN-FORK-LOG.md`) and Ev's answer to it on PR 7046: no rule about HOW a lane makes its triangles goes on the design page. `[mesh.sizing]` and `[mesh.winding]` already say what a mesh must be (each triangle within δ of its face, each winding resolved), and the per-triangle certificate holds that whatever a lane does; Ev prefers the triangulation left at that level of abstraction, "any division into triangles so that it never deviates by more than delta", over constraints laid on it. So this unit is engineering held to the page as it reads. It measures three cures and builds one.

## Terms

- **Ruled face**: a face on a cylinder or a cone. Its chart lines `u = const` are straight: *rulings*. On a cone every ruling passes through the apex.
- **Row**: a rim of the face, or the apex counted as a row of one point.
- **Split side**: a straight side of the face carried by two or more edges, so it has a vertex part-way along. Booleans make these.
- **Rectangle lane / chart lane**: `mesh::curved` and `mesh::trimmed`.
- **Headroom**: the chart lane sizes at half of δ where it has no theorem (`UNPROVED_HEADROOM`).

## The defect

A cylinder or cone face whose straight side holds a vertex is the chart lane's since PR 7049 (`walk::splits_a_straight_side`). The chart lane lays lattice rows on it, which a face that is straight along its rulings does not need, and sizes at the headroom. So splitting one straight edge of a body multiplies the triangles of the faces beside it: 14.6 times at worst in the suites, 36 times on a rod measured in PR 7049's review, and by the designers' table below 7 to 86 times on a plain cone, 150 to 500 on a slender one. The same face without the split is the rectangle lane's and meshes at one triangle strip per column.

**What "done" is:** such a face meshes within δ, at every δ and the three ε, at a triangle count close to the same face without the split (the designers' probes reached the unsplit count plus one triangle per vertex, and 1.25 to 1.45 times it), and a split never makes a face refuse.

## The three cures, in the order to try them

1. **No rows; the triangulation left alone; the lane refines where the certificate refuses.** The chart lane lays no lattice rows on a ruled face (the step along a straight ruling is unbounded), triangulates its boundary points by its ordinary constrained Delaunay step, and where the per-triangle certificate refuses a triangle it adds points and re-triangulates, as the spline arm does. One designer's probe with rows off and plain Delaunay meshed an UNSPLIT cone, frustum and cylinder at every δ at 1.25 to 1.45 times the rectangle lane's count; with a split side it refused `CertificateExceeded` from δ 0.03 down, because Delaunay fans the mid-side vertex across several columns. Whether refinement on the certificate converges there, to what count, and at what cost, nobody has measured. This is the cure Ev's preference points at, so it is measured first and fully.
2. **No rows, plus one held edge per split side.** As (1), with one constraint edge of the kind the triangulation already holds for boundary sides, from each mid-side vertex across the face. Prototyped in the rectangle lane on about 300 cells: the unsplit count plus one triangle per vertex.
3. **The walk.** No triangulation step on such a face: its triangles are written by walking its two rows in order of angle about the axis. Not built; the argument that it is within δ is below. It was the designers' agreed recommendation, chosen over (2) by a principle Ev has declined to put on the page; it stays a candidate on its merits (it cannot write a triangle along one ruling, and it does not pass through the cone's development, so it has none of that lane's merge limit).

## Owed, in two stages

**Stage 1: measure, then stop and report.** Build each cure as far as measuring it needs, behind a switch that does not ship. Corpus: `crates/mesh/tests/split_rulings.rs`' bodies; `reach_probes`' T1; plain and split cones at half-angles 0.01° to 80°, frustums (whose two rims are chorded at different counts from δ 0.03 down), cylinder walls, one-face full-turn walls (a seam walked twice), needle and slender walls; each line edge split at 1, 2 and 5 points and close to an end; δ 0.1 to 0.001; ε 1e-9, 1e-6, 1e-12. For each cure and cell: meshes or refuses and why; triangles against the same face without the split on the rectangle lane; `check_mesh`, the winding rule, the certificate's worst reading, sampled deviation; the passes cure (1) takes to converge and its wall time against today's. Also: what each cure does to the three residuals at the end of this file, and to a planar neighbour's chord count. **Report the table and a recommendation, and build nothing further until the orchestrator answers**: which cure is built is a choice with more than one defensible answer.

**Stage 2: build the chosen cure**, with rows that pin the count relation (a split adds at most so many triangles) rather than raw counts, mutants, the §5 sweep, and the digests showing that no face without a split side moves. If the cure changes what `[mesh.sizing]` says a lane does, the new text is proposed to Ev with the measurement, as an `[ev]` PR of its own.

It closes `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem` if the built cure gives such a face a bound, and unit 6's density measurement (`rectangle-faces-move-to-the-chart-lane`) is taken per chart after it.

## Cure 3: why the walk would be within δ

The front edge `a_i–b_j` keeps `max(a_i, b_j) ≤ min(a_{i+1}, b_{j+1})` (it holds at the start, where the rows' ends coincide, and each advance preserves it). So the triangle `(a_i, a_{i+1}, b_j)` spans at most one chord of row `a` or one of row `b` in angle. A triangle whose corners' angles lie within a range lies in the convex hull of the surface between the two rulings at the ends of that range, and deviates by no more than the sagitta of the larger rim's chord over it: what `sizing::cone_grid_step` and `sizing::sagitta_step` already hold to δ at the face's column step. Triangles made by a side vertex lie in the plane of the triangle they divide.

**Owed as written proof beside the step functions:** the convex-hull step for a cone triangle on three rulings (two designers rate it sure, one likely).

## Measured by the designers (PR 7043's head, debug, default ε)

Cone-face triangles, unit 45° cone, full turn:

| δ | plain, rectangle lane | plain, chart lane today | split side, chart lane today | split side, strip |
|---|---|---|---|---|
| 0.1 | 8 | 58 | 60 | 10 |
| 0.01 | 24 | 604 | 606 | 26 |
| 0.001 | 72 | 6168 | 6170 | 74 |

The strip column is a prototype that held edges in the rectangle lane, not the walk; it emits the counts the walk would. About 300 cells (split cones, frustums, cylinders, cones of half-angle 5° and 1°; δ 0.1 to 0.001) mesh at the unsplit count plus one per vertex and pass `check_mesh` and the winding check; T1's 18 cells also pass the probes' outward and true-distance checks. A frustum's rims are chorded at different counts from δ 0.03 down (51 against 43 points at δ 0.001 on a half turn, radii 2 and 1), which the walk covers and a rule joining "partners at one angle" does not.

## Not measured

As this file stood before stage 1; what stage 1 and the build then measured is under "Closed" below.

- A refinement that splits the offending edge (cure 1 was measured with one rule only, a centroid insert).
- Cure (2) run in the chart instead of the cone's development.
- A face on both nappes of a cone (no body built here makes one).

## What the interim route left (2026-11-17, PR 7049)

The interim this unit follows has merged: a ruled face whose straight side holds a vertex is the chart lane's (`walk::splits_a_straight_side`), and the rectangle lane's fallback lattice is deleted. Its review measured what the route costs beyond density, each filed and pinned in `crates/mesh/tests/split_rulings.rs`, and stage 1 says of each what every cure does to it:

- a planar neighbour narrower than δ beside a routed face refuses `Triangulation` where the rectangle lane's coarser rim meshed it (373 cells in the review's sweep; `a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta`);
- two points of one ruling closer than 2⁻²⁷ of a cone's slant radius refuse `DevelopedMerge`, which on a draft-angle cone is reached at the default ε (408 cells; `a-ruling-split-within-the-developments-resolution-merges-two-points`, with a cure inside the development in flight as `a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation`). Only the walk leaves the development;
- a ruling split a hair from its end folds a triangle (`a-ruling-split-a-hair-from-its-end-folds-a-triangle`).

## Unit (2026-11-17, TESSLANE)

**Letter H. Tier: dual review.** The cure changes how the chart lane sizes and triangulates every cylinder and cone face it takes with a split side, and candidates (1) and (3) each add a mechanism to that lane (a refine loop on the certificate; a second way of writing triangles); the count relation it must hold has no oracle but measurement. The two stages above are one unit: stage 1's report is not reviewed by the tier, stage 2's build is.

Branch `tesslane/ruled-density`, cut from `tesslane/ev-ruled-strips` (PR 7046, the tracker change that carries this file) on main `184e484ed2`. Two pull requests in `crates/mesh` are in review and reach main beside this unit: PR 7054 (`trimmed.rs`: the cone arm's `insert_points` and `emit`, a run of merged ruling points re-split) and PR 7041 (`tessellate.rs`, `curved.rs`: one route per face; it takes `walk::splits_a_straight_side` into `tessellate::route_of`). Stage 1's switches are throwaway, so it does not wait for them; merge main before stage 2 and build on what has landed.

**Pre-push for stage 2**, on the pushed head: fmt, `scripts/run-gates.sh`, the doc gate, clippy (`mesh` and `geom-brep` all features and targets, the viewer with `--features app`, `demos/tour`); `mesh` whole with the default profile (slow set included) and the changed rows at ε 1e-6 and 1e-12; `mesh --features budget`; the three `tools` roots; `step-import`, `stl`, `sweep`'s meshing modules; the viewer with `--ignore-default-filter` for its slow-set rows pinned on mesh output (`display_budget`, `review_pick_r2`); `demos/tour` with nextest's `--cargo-profile ci-tour`; the tess-budget meter's bytes (`tools/tess-meter`), since a density change moves them and each moved row is argued.

## Closed (2026-11-22, PR 7096)

**Stage 1 measured the three cures** over about 1,660 cells at the default ε (the same picture at 10⁻⁶ and 10⁻¹²):

| cure | meshes (walls / cones / frustums) | worst triangles against the unsplit face |
|---|---|---|
| the chart lane as it was | 530 of 590 / 615 of 635 / 295 of 315 | 196× on walls; 17,618× on a 0.01° cone |
| 1. no rows, refine where the certificate refuses | 501 of 590 / 568 of 635 / 295 of 315 | 24× on walls, 102× on cones |
| 2. no rows, one held edge per split side | all but the cells the development merges (55 / 10 / 20) | 2.4× |
| 3. the walk | every cell, at three ε | 2.25× |

Cure 1's failure to converge was measured with ONE refinement rule (a centroid insert, 400 passes); it does not show that refinement in general fails. Cure 2's refusals are the development's 27-bit merge. One finding cuts across all three: sizing the face at δ, not at the headroom, is what stops a planar neighbour narrower than δ refusing. The orchestrator ruled the walk on these numbers.

**Built:** `crate::ruled_walk`. A ruled rectangle with a split straight side is `Trim::Walked` in the route's value (`tessellate::route_of`, `trimmed::walkable`), sized at δ by the rectangle lane's theorem, and has the unsplit face's triangles plus one per point its side gained, pinned as a relation against the unsplit body in the same run (`split_rulings::holds_the_relation`). Against merged main over one review's 19,426 common cells the head writes 0.80 M triangles to main's 9.76 M.

What the sections above had wrong or did not foresee, each found by a review or by the integration:

- **"No edge of four triangles" was false.** Where ONE neighbour holds both rims of the walked face (a ball bored through), the neighbour's triangles can join the same two rim points. The walk cuts such a side at its chart midpoint, only where the neighbour could make it (`trimmed::may_make`). The rule reads where the neighbour's surface lies, not how the neighbour is meshed, so it **over-cuts** at coarse δ on bored bodies: a kept, stated density cost, with the number of cuts pinned per cell. The question of one cure for the whole mesh is `the-mesh-cures-a-side-two-faces-can-both-make-three-ways`.
- **A one-chord face to the apex with both sides split** needed its own triangles (`Rows::zip_sides`); the fan-from-the-opposite-corner rule put two fans on one triangle there.
- **"A split never makes a face refuse" does not hold everywhere.** A run anchored by the apex on a needle triangle of a cone that is not walked; a split beside an apex the band cannot place; and twelve cells of a D prism thinner than δ with every line edge split, which mesh on main and refuse here at a planar cap of no area (the class of `a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta`). Each is pinned.
- **Where the walk is chosen.** Dispatched as "inside the chart lane", because PR 7041 was rebuilding the route; both first reviews named the cost of a second predicate in the lane, and after PR 7041 merged the route's value carries it.
- **A walked face the lane cannot walk is a kernel bug**, and by `[failure.taxonomy]` it is `unreachable!` with its row-0 answer at the site, not a typed error. The orchestrator ruled a typed variant twice before a review cited the page.
- **The walk's reach test read ε** (`trimmed::may_make`, on the orchestrator's instruction in the fix pass: a three-way comparison on the run's band). With δ held fixed the bored ball's first cut then sat at a different δ for each ε, against `[tolerance.chordal]` and `[mesh.winding]` ("No ε is read"). Neither review raised it; a designer reading the tree for another question did, after the reviews were in. It now reads none: the side stays uncut only past `δ·(1 + ALLOWANCE_SHARE)` plus two of the certificate's own rounding allowances, and `split_rulings::the_walks_cut_is_the_same_at_every_eps` holds the mesh's bytes across ε either side of the threshold.
- **A body thinner than δ meshes inside out, on main as on this head** (`a-body-thinner-than-delta-meshes-inside-out-and-nothing-refuses-it`). The bored-ball rows pass such cells: they hold the edge count and the cut count there, and nothing of the mesh's sign.

Main moved under the unit three times, the last by about 2,500 commits during a four-day outage, with TESSLIE's PR 7198 (one shared-side rule on every triangulating arm; a census that refuses an overused side) on the same ground. The walk reads that pull request's one test (`TrimPolygon::faces_holding_side`) and keeps only its own narrowing and its own cure.

`[mesh.sizing]` names two kinds of worst triangle and the walk's is neither; the clause is before Ev in PR 7486, whose text drops the list.

Filed by the unit: `a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles` (main's unsplit bore); `a-walked-sides-reach-exclusion-is-read-only-by-the-walk`; `a-body-thinner-than-delta-meshes-inside-out-and-nothing-refuses-it`; `pole-identification-lets-eps-choose-between-two-meshes-of-a-cone-face`; main's red slow-set row `work/green/main-red-the-viewers-pick-tally-reads-six-grazes-short-of-its-pin.md`. Closed by it: `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`, `a-ruling-split-a-hair-from-its-end-folds-a-triangle`.
