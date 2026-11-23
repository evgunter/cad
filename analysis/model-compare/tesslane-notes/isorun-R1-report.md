REVIEW REPORT, PR 7526, frozen head 553d4d6310 (reviewer lane; base = origin/main da61db71, whose `crates/` equals the merge base 31a697d3's)

## Verdict: NOT-MERGEABLE-AS-IS
One reason: the unit's own condition ("the widest form that loses no face at today's target") is false, shown by a body that meshes on main and refuses on the head. Everything else is fix-level. The loss is narrow (8 tuned cells; 0 of 160 random cells), main refuses two sibling cells the head meshes, and the spec's fallback (key it to the class) would not cure it, since a holed wall is in the class. So the decision this needs is a ruling on whether that trade is accepted and filed, not a rebuild. If it is accepted, this is APPROVE-WITH-FIXES.

## Claims
1. **FALSIFIED (demonstrated, both trees).** A pipe wall with two small holes at mid-height, 1.5 to 1.6 column steps of free wall between them, centred half a column off a rim chord point: `CertificateExceeded` on the head at ε 1e-9 and 1e-12 in 8 cells (r=1 δ=1e-2 h=1.0, 1.9; r=1 δ=1e-3 h=1.33; r=1e-3 δ=1e-5 h=1.0, 1.9; bounds 1.08 to 1.26 δ) and at ε 1e-6 in the 3 cells it reaches. Main meshes all 8, and `check_mesh` plus the body's geometry hold on them. Source and raw lines were sent in my 06:5x message.
   - Counter-evidence I owe: on the same 180-cell grid main refuses 2 cells the head meshes (r=10, δ=0.1, h=1.0 and 1.33, bound 0.108). An unbiased sample (160 cells, random gap 0.6 to 2.6 columns, phase, hole size, height, δ 1e-2/1e-3) refuses 0 on both trees.
   - Mechanism, BY INSPECTION: with no node between the holes the CDT joins their facing corners; that side spans 1.5 to 1.6 columns and the headroom's ½ covers √2. Which heights refuse follows the power-of-two rounding in `AnalyticRow::metric` (`column step / window height`).
   - Held on the rest: one hole (300 cells, δ 1e-1 to 1e-4, h 0.3 to 40), tall walls h=10 and 100 notched, a comb of four notches, sections to within 0.002 rad of tangency with a rim, slivers with a long ruling, δ to 1e-4: 0 refusals, 0 bad meshes on the head. Scale 1e3 not read (a test helper panics before the mesher).
   - Suites: no head-only refusal in `mesh` at three ε or in `sweep`'s meshing modules at 1e-9.
2. **HELD against merged main.** My own per-patch digest (positions' bits of each patch's triangles, keyed by a body hash and δ), `mesh` whole at ε 1e-9, 1e-6, 1e-12 and `sweep`'s 66 mesh-touching modules at 1e-9: every moved patch is a cylinder's; plane, cone, sphere, torus, spindle, NURBS and Approx patches all identical (e.g. 1e-9 `mesh`: 34,433 plane, 5,501 cone, 1,826 sphere, 2,838 torus, 565 NURBS same). Slots where my body hash collides (22 in `mesh`, 43 in `sweep`) differ in cylinder rows only. Walked faces: BY INSPECTION (`tessellate_trimmed` returns to `walk_face` before the lattice); my log carries no lane.
   - **The unpaired faces: CORRECTED/explained.** In my runs 52 keys each way do not pair, all from two tests that draw random bodies every run (`an_annular_tube_through_a_plate::the_annulus_builds_at_random_azimuth_pairs`, `review_chamfer_r1_probes::a_skewed_wedge_chamfers_with_every_face_outward`; `fuzz::start`, `CAD_FUZZ_SEED` replays). Nothing hides: refusal rows in `sweep` are the same 14 on both trees, none in those tests. The PR's "their body digests differ run to run" is true and unexplained there.
3. **Mechanism not re-derived by me (no attempt log); counts CORRECTED.** `mesh`: 13 denser patches per ε (12 saddle faces, the rod sliver at 2e-2), matching the PR's 14 counted by row. `sweep`: 6 more (`bisector_nodes`, 112 → 114), which the PR's "max ratio 1.018" implies and its "14 faces" omits. Non-convergence: not found. Crossing rods at six pairs and δ 0.3 to 1e-3 give identical outcomes on both trees, worst 3.15 n; the head is denser only at δ ≥ 5e-2 and far sparser below (1e-3: 4,940 against 14,748). Two cells refuse `Triangulation` on both trees (0.999 across 1 at 0.8 rad, δ 0.3 and 5e-2): main's. A pinned cost is acceptable; 3.25 is a fit with nothing arguing it.
4. **HELD.** The row passes on the head; a mutant that accepts at the last attempt turns it red at δ 5e-2 (`OverusedSide` in place of `UnclearedSharedSide`). On main the 5e-3 cell still refuses typed at attempt 0, so the fixture move is this change's.
5. **Relations CORRECTED: two of the four bounds are fits.**
   - Tilt, `≤ 1/cos φ`: FALSIFIED, demonstrated. The count is `(1 + cos^-1.5 φ)/2` to three figures at every φ tried, and passes `1/cos φ` at φ ≈ 1.18: φ=1.25 reads 3.28 to 3.31 against 3.17, φ=1.35 reads 5.31 to 5.35 against 4.57. The row's doc argues "chorded at the lane's step along its length, which is the rim's over cos φ at most"; the ellipse takes cos^-1.5 φ times the rim's chords.
   - Holed, `≤ 1.4`: a fit to one bar. The ratio is 1 plus the holes' share of the turn: bar half-width 0.6 r reads 1.48 to 1.63, 0.9 r reads 1.76 to 1.88.
   - Notched `≤ 1.1` and split `+1 per point`: not broken on my bodies.
   - Mutant "rows laid again" (main's `chart_table.rs`/`trimmed.rs` under the head's tests, three ε): rows 1, 2, 4 red, row 3 green, as the PR says. Row 3 is reddened by no mutant of this unit.
6. **HELD for the two viewer pins, reproduced.** Main's mesher: `cut_cylinder` 4,520/7,816 and 81,208/84,504; tally (451,512, 147,850, 16,596, 8,142), so main is six grazes off its own pin of 147,856 and the filed `work/green` file exists on main. Head: 13/13. `d9_mesh_goldens` and `route_rows` go red on main's mesher under the head's pins. The ray count moves because the aim is cast from every (subsampled) mesh vertex; the test's own header says it moves with any retessellation.
7. **HELD, with two leftovers.** The four route files are byte-identical to origin/main; no `IsoBounded`, `runs_on_iso_lines` or env read in `crates/mesh/src`. Leftovers under Style. The deleted `debug_assert`: its literal condition cannot arise with no node; the fan itself is still observed by the emit census (`trimmed.rs:1145-1167`), so nothing is unwatched.
8. **HELD by inspection and by run.** The cylinder arm's new path reads the polygon's UV box, `hu` and no band; the digests of rows 1 to 4 are identical at three ε in my runs.
9. **Not falsified; not exercised.** The split is on `Surface`'s variant, so a cylinder cannot reach the cone arm. I built no near-zero-half-angle cone and removed no cone rows. The spline-stored cylinder: read only.
10. **Mostly true; three corrections.** (a) "14 faces denser" omits `sweep`'s 6. (b) `lattice_rows`' doc says the measurement covers both suites "at three ε"; the brief says `sweep` was read at one. (c) the docs' "rows made 4 to 220 times" against the table's 1.2 to 223. Census figures not re-taken.
11. **Hosted CI: no test job has run on this head.** `change filter` succeeded at 06:58Z; 11 jobs are queued (`test (kernel)`, `test (app)`, lint, tools, python, step import, demos, the mesh meter, interval, corrupt input, viewer app); two skipped. In its place, locally, debug: `mesh` whole with the slow set on the head, 551/551 of its own rows at ε 1e-9, 1e-6 and 1e-12; `sweep`'s meshing modules 601/602 with main's known red `bisector_nodes::the_near_node_poses_at_the_witness_band` on both trees; the viewer's `display_budget` and `review_pick_r2` 13/13. Not run: fmt, clippy, gates, doc gate, tools roots, python, the tour.
12. See Spec deviations.

## Findings
- **MAJOR, DEMONSTRATED.** `chart_table.rs` `AnalyticRow::lattice_rows` (cylinder arm): 8 two-hole walls that mesh on main refuse `CertificateExceeded` (claim 1). No row, no issue file and no sentence of the PR holds the shape.
- **MINOR, DEMONSTRATED.** `rowless_cylinders.rs` `a_wall_cut_on_a_tilt_...`: the bound and its stated argument are false past φ ≈ 1.18.
- **MINOR, DEMONSTRATED.** `rowless_cylinders.rs` `a_notched_or_holed_wall_...`: 1.4 holds for this bar only.
- **MINOR, DEMONSTRATED.** PR body, "the unit makes 14 faces denser": 6 more in `sweep`.
- **NOTE, DEMONSTRATED.** Main refuses two-hole cells too (r=10, δ=0.1) and a crossing-rod pair refuses `Triangulation` on both trees; neither is this unit's, neither is filed that I found.
- **NOTE, BY-INSPECTION.** `crates/mesh/README.md` `[mesh.sizing]` ("The lattice keeps clear of the boundary": a triangle joining the boundary to a node reaches `1 + BOUNDARY_CLEARANCE` steps) describes a lattice the cylinder arm no longer has. The PR does not say whether that clause was read.

## Spec deviations
- Reported: built one ingredient for every chart-lane cylinder face, not a cure for the class; no δ target, no refinement; tour not censused; pre-merge differential base.
- SILENT: (1) the spec's fork "if something refuses, the class is the route's fact and the faces that need their rows are filed" was closed on suite evidence alone; stage 1's corpus held holed cells but no two-hole wall. (2) Stage 2 owed rows that "pin a relation"; two of four are fits (claim 5), and row 4 is a pinned digest. (3) `sweep` at one ε, against a doc sentence saying three.
- Was it the right unit: yes on the measurement (391 + 1,122 faces, 10 to 100 times fewer triangles, no suite face lost).

## Style
Exercised: Q1 (prose and constants sweep over `chart_table.rs`, `trimmed.rs`), Q2, Q3, Q4, Q5, Q6, Q7. Q8 NOT done in full: I read `lattice_rows`, `lattice_steps`, `metric`, `steps`, their callers and every changed doc whole, and `rowless_cylinders.rs` whole, but not `trimmed.rs` end to end.
- `trimmed.rs:1536-1537` (Q4, sure): "the INGREDIENT half ... is checked at the cylinder arm's `nu`" cites the check this PR deletes. The doc rotted; the code is right.
- `curved.rs:1096` (Q4, sure): "rows every `lever·hu` on a ruled chart" describes `lattice_steps` as it was. The PR's sweep names this caller as "not this unit's" and left its sentence.
- `trimmed.rs:363-365` (Q5, sure): `UNPROVED_HEADROOM`'s first sentence says the cylinder arm "sizes its lattice" at this share; the next paragraph says it lays none.
- `chart_table.rs:476-478` (Q5, likely): `metric`'s doc says the cylinder's is "its window's height over its column step"; the code computes column step over window height.
- `chart_table.rs:384-393` (Q6, sure): a measured claim at the claim site ("no cylinder face refuses or folds", "4 to 220 times") with no guard and no reason it can have none; it is true of the suites and false of the two-hole wall.
- `chart_table.rs:384-388` (Q2, likely): "a row does not shorten a triangle's span round the axis" is why rows buy no accuracy on a strip; it is not why a boundary-point CDT stays within the headroom, which nothing argues. The lane does so because of the metric, which no comment says.
- `chart_table.rs` `metric`, cylinder arm (Q7, likely): the plane the CDT runs in is now set by the face's window height rounded to a power of two, so a small change in height flips the triangulation and, in my probe, which heights refuse. A knob nobody chose.
- `chart_table.rs:455` (Q7, unsure): the split is by surface kind; a 1 mrad draft cone keeps graded rows beside a cylinder with none. The property that matters is radius variation over the window.
- `trimmed.rs:653-656` (Q7, unsure): `nu` is still computed and `pole_side_columns` still called on the cylinder arm, to feed `metric` only.
- `reach_probes.rs:2163-2175` and the `13 * n` assert (Q3, likely): pins the cure's present arithmetic, not a contract; a better cure that splits sides differently reddens it. Measured numbers are marked measured; the bound is not argued.
- `rowless_cylinders.rs` row 4 (Q3, likely): three typed digests; any intended mesh change re-pins them, so it is a golden under a relation's name. It does go red on an ε read.
- `rowless_cylinders.rs:24` `TARGET_SHARE = 0.5` (Q1, sure): a second spelling of `trimmed::UNPROVED_HEADROOM`, reconciled by a comment ("where it moves, this share moves with it"); nothing makes it move.
- `rowless_cylinders.rs` `digest` (Q1, likely): another FNV-1a mesh digest beside `d9_mesh_goldens`' and `route_rows`'; I did not diff them.
- `review_pick_r2.rs:90-97` (Q2, likely): "fewer rays land" is not what moved the count (fewer vertices are aimed from); and the pin's comment now records main's own red as its baseline.
- `trimmed.rs:3749-3760` (Q7, unsure): `id_repeats_apart` moved into `tests` with a doc still calling it "the second ingredient" of a check that no longer exists outside tests.
- Class, not instance: every stale sentence above is "prose citing the cylinder's lattice"; I would also look in `work/tesslane/plan.md`, `docs/TESS-BUDGET.md` and `the-trimmed-cylinder-lane-has-no-worst-triangle-theorem.md`, none of which I read.

## Rubric
- Idiom/structure: 4. A two-line change at the right home (the chart's row), route untouched; loses a point for the metric's accidental role.
- Test quality: 2. Rows 1 and 2 pin fits their docs present as arguments, one of them false; nothing holds a face against refusal off the suite's bodies.
- Doc/comment honesty: 3. Measured numbers are mostly marked; three stale sentences, one inverted, one over-claimed ε coverage.

## Executed / read only
Executed: probes of my own on both trees (two-hole grid, random two-hole sample, one-hole, tall/comb/tangency/sliver shapes, tilt and hole relations, crossing rods); per-patch digest differential, `mesh` × three ε and `sweep` × 1e-9, base and head; the "rows laid again" mutant at three ε; the last-attempt mutant; both viewer pins on both trees. All local edits are removed; nothing committed or pushed.
Read only: the spec, `[mesh.sizing]`, the PR body and check runs, `curved.rs`' caller, the `sweep` and viewer test sources. Not read: PR 7526's thread, any `analysis/` branch, any other review.

Reviewer test worth keeping: the two-hole cell (r=1, h=1, δ=1e-2, gap 1.5 columns, phase 0.5) as a pinned witness of whichever outcome is ruled.
