REVIEW — PR 7049 @ 4f1bb5c44c (unit = 9391ee0177...4f1bb5c44c). FULL review, final.

**Verdict: NOT-MERGEABLE-AS-IS.** The route, the deletion and the suites are as the PR says. Two body classes that meshed on the base refuse on the head, and the PR states the first does not happen.

## Claims
1. HELD, two corrections. My base table (4677 cells) reproduces every row of the PR's. Corrections: "frustum: meshes" is true only of the axis-touching frustum; frustum walls (half/thin/wide) and one thin frustum refuse CertificateExceeded on some base cells. The density table's "1581" (T1, δ 1e-3) is no base cell: all six δ 1e-3 cells refuse on 9391ee0177 (only 81 and 340 meshed).
2. HELD, with one exception at claim 8. No false negative: no cell and no suite row reaches `SPLIT_SIDE_ROUTED_HERE` (splits at 1, 2, 5 points, 3e-7 and 1e-9 from an end, every ruling at once, one column wide (0.05 rad), reversed inner walls, three ε). Loop start and walk sense: by inspection (cyclic `any`, symmetric product). Ring: by inspection (`has_ring` arm is earlier). False positives: flipping the `v` comparison reds 11 rows (slit, rim-free loops, D9); the same-kind rule reds the PR's 5. Band: `SizingTols::band` (`Band::linear`), read only inside `require_iso_rectangle_face`; the `v` test is bare f64 `signum`. A face can route two ways at two ε only where the door's own banded answer flips, and the other route is then the door's refusal, not a rectangle-lane mesh. Not observed.
3. HELD by inspection, UNPINNED by execution. Every reader goes through `lane_of(.., band)` (dispatch, `chords::tighten`, `FaceLattice::of`, `trimmed::face_target`); the memo key folds the decided `Lane`. No second decider in mesh, viewer or tools. But see MINOR-3(c).
4. HELD for reach, UNPINNED. `bare_sides` grep-absent; cone arm `(nu, 1)`. A non-rectangle class face meets the door first (`curved.rs` door call precedes the polygon), so it answers `UnsupportedCurvedShape`. See MINOR-3(a,b), NOTE-1.
5. HELD in the suites, FALSIFIED off them. Own digests: mesh 69 class (53 cyl, 16 cone) / 92 neighbours (76 plane, 8 cyl, 8 cone) / 0 other; sweep (578 rows, 62 modules) 30 class cyl / 60 plane / 0 other, all in `a_box_along_an_inflection_ruling`. Exactly the PR's table. No verdict change in either suite except the four ERR→OK. Off-suite: MAJOR-1, MAJOR-2.
6. FALSIFIED in named cells: MAJOR-1, MAJOR-2, MINOR-1. Every other class cell meshes and passes check_mesh, check_winding, outward, within-δ and volume (against the unsplit body) at default ε and 1e-6; at 1e-12 likewise.
7. HELD. Every row I re-measured matches. Worst ratio: 14.6× in the suites (9 → 131), 2.9× in my mid-side splits, 36× for a rod whose ruling is split 3e-7 of its length from an end (894 → 32038 triangles, δ 1e-4; 11.7× at 1e-3).
8. CORRECTED. Cylinder wall: meshes, watertight along the seam (check_mesh, three ε). Cone wall: refuses SeamTwice (MAJOR-1). BothNappes: read only; `the_apex_crossing_bowtie_refuses_at_the_shape_door` is green on the head, the split variant is not a row.
9. Done: see mutants below.
10. (a) HELD: ball with each pole meridian split at 1, 2, 3 points, oblique and aligned, δ 0.1–1e-3, 36 cells outward and within δ. (b) UNSURE: the PR's second pass grepped `crates/mesh` and `crates/geom-brep` only; step-import adopts B-spline surfaces and sweep lofts were outside that grep. I built none.
11. step-import: right (its row asserts split == plain counts; hosted job green; read, not run). SeamTwice: WRONG for a cone (MAJOR-1). BothNappes: not falsified. Run rule: right (mutant). "Done when": all four items met; "named refusals ... each pinned if a door builds it" is NOT met.
12. HELD. Hosted CI on this head: change filter, lint, test (kernel), test (app), corrupt input, python suite, viewer wasm32, step import, demos, tools, mesh budget meter, interval backend, gate ok all success; interval oracle and cache prime skipped. Both viewer rows are slow-set, so CI does not run them: locally both are red on both trees with byte-identical output (same 68-row answer table; tally (451512, 147856, 16596, 8142) vs pin (451800, 147952, 16824, 8244)). Not this unit's.

## Findings
- **MAJOR-1, DEMONSTRATED.** `trimmed.rs` `singular_row_shape` via `walk::splits_a_straight_side`: a one-face full-turn cone wall with a split seam. `revolve` of (1,0),(3,0),(2,1),(1.5,1) about y, Full; either cone face's seam split. Base: 60 of 72 cells mesh clean (12 CertificateExceeded). Head: all refuse `SphereTrimShape { SeamTwice }`, at every δ and all three ε. The PR's "it is in the new row (... the three full cones)" is wrong: those cones and the frustum are two half faces each. Qualification: a `split_edge` fixture, like the PR's whole row; `at_rest` rejects such a body (JoinableVertexAtRest), and I found no finished body with a bare vertex on a one-face seam.
- **MAJOR-2, DEMONSTRATED, ε 1e-12 only.** Cone faces (frustums, frustum walls, diamonds; full, half, thin, wide) with a ruling split 1e-9 of its length from an end: 168 cells mesh clean on the base and refuse `SphereTrimShape { DevelopedMerge }` on the head. At default ε `split_edge` refuses that split, so it does not arise.
- **MINOR-1, DEMONSTRATED.** Cones at 1°, 5°, 20° over a 2π−0.3 turn, ruling split 3e-7 (default ε, 20° only) or 1e-9 from an end: `FoldedTriangle` on the head. Base: UnresolvedWinding, so no regression, but "every class face meshes" is false there.
- **MINOR-2, DEMONSTRATED.** `curved::tests::a_ruled_face_with_a_split_ruling_meshes` doc and the PR: "a full turn's seam, which its one face walks twice" holds for one body of six.
- **MINOR-3, DEMONSTRATED (surviving mutants, 494/494 each).** (a) `walk.rs` route with the `require_iso_rectangle_face` confirmation dropped. (b) `curved.rs` `SPLIT_SIDE_ROUTED_HERE` refusal deleted. (c) chord pass blind to the route: 3000 of my 4677 cells change triangle count, all still clean, no row notices.
- **NOTE-1, BY-INSPECTION.** `curved.rs:300`: the refusal fires on any polygon entry strictly between rows at a `u` extreme, including a chord point `chords::tighten` puts on a ruling at a spline neighbour's ask. The route reads vertices only and cannot see that. Base: the cylinder arm ignored it. Reached by no row in either suite.
- **NOTE-2, BY-INSPECTION.** `walk.rs` `reach`: `f64::signum(0.0)` is 1.0 and NaN compares false, so a far end at exactly the junction's `v` reads as above. Zero-length edges only.

Mutants (one switchable build, whole mesh suite): route removed → the PR's 4 rows. Widened to every ruled face → 27 red, both D9 rows (PR: 26; one of mine is a multi-minute run I killed, one died for memory). Same-kind rule → the PR's 5. `v` flipped → 11. Three survive, above.

## Spec deviations
Reported: the four corrections (three right, one wrong). SILENT: (1) a face that meshed on the base refuses on the head, which the spec makes a stop; (2) the SeamTwice refusal a door builds is unpinned and declared absent; (3) the spec asks for `bare_sides` "moved ahead of the route"; the unit wrote a second predicate and kept the first as the refusal.

## Style
Exercised: Q1, Q3, Q4, Q5, Q6, Q7; Q8 on the four named pieces plus `grid_counts` and `tessellate_curved`'s prologue, not a whole file end to end; Q2 without blame.
- S1 (Q1, sure) `walk.rs:690`, `curved.rs:293`, `curved.rs:300-304`, `topo/src/chart_iso.rs:150`: "a side holds a point" now has two predicates that are not the same (loop vertices by `v` order; polygon entries between rows), beside the run rule. The polygon expression is written twice four lines apart. The fix for an admission test read after routing minted a second test. Class: look at `has_ring` vs `RINGED_FACE_ROUTED_HERE` for the same pair.
- S2 (Q1, likely) `walk.rs:696`, `curved.rs:301`: "ruled face" is the literal `Cylinder | Cone` at each site; the chart row already knows which charts are ruled.
- S3 (Q5, sure) `curved.rs:1046-1054`: `grid_counts`' doc says the cone arm reads `has_pole` and `meridian_interior`; it reads neither now. `pole_columns`' doc still argues about cone walls (likely).
- S4 (Q5, likely) `sizing.rs:356-365`, `tessellate.rs:361`, `memo.rs:31`: the band is "consumed by three sites" / "its only consumer"; the dispatch and the chord pass read it now.
- S5 (Q3, sure) the three surviving mutants: three clauses of the PR body are held by no row.
- S6 (Q3, likely) `curved.rs` new row: bound `δ·(area + area)` loosens as triangles are added; no outward or within-δ check (reach_probes' `geometry` cannot take a `split_edge` body). It pins watertight and a loose volume.
- S7 (Q5, likely) `curved.rs:181`, `curved.rs:305`: `MissingEntity` for a state where nothing is missing, and "router defect" where (NOTE-1) the router is right.
- S8 (Q7, unsure) `chords.rs` `edge_target` → `face_target` → `lane_of`: a face's lane is recomputed per edge per side, and for a class face each call now runs props' door. I would decide once per face.
- S9 (Q7, unsure) `walk.rs` `reach(before) * reach(after) < 0.0`: a sign product where an ordering comparison says it; the vector is named `walk` inside `walk`.
- S10 (Q4, likely) `work/tesslane/a-cone-face-met-on-its-axis-...md` ("the chart lane, which meshes every one of them today") and the re-pointed row: false for MAJOR-1's face; the tracker carries no SeamTwice row.
- S11 (Q6, likely) the spline-row gap is disclosed, "not filed", and its grep was blind to the import and loft doors: disclosed and unscheduled.
- S12 (Q6, likely) the density price is a measurement with a schedule (the re-pointed row) but no guard; the one count pin (`40`) did not move under mutant (c).

## Rubric
- Idiom/structure 3: one small function, one call site, clean deletion; but two predicates for one concept and a per-edge re-decision.
- Test quality 2: the route's presence and `v` order are pinned both ways; the door confirmation, the refusal, the one-route claim and the one-face cone are not, and the new row cannot check outwardness.
- Doc/comment honesty 3: the cause and the price are told straight; the SeamTwice paragraph and the "one face" sentence are false as written.

## Executed
Own per-patch digest hook on both trees: mesh suite whole (head 494/494, base 493/493), sweep's 62 meshing modules (578/578 each). Own sweep, 4677 cells: head at default ε, 1e-6, 1e-12; base at default and 1e-12. Seven mutants. The PR's four rows at 1e-6 and 1e-12 (green). Both viewer rows on both trees. Read only: step-import, BothNappes, `trimmed.rs` beyond `face_target`/`arm_steps`/`singular_row_shape`, the README. Nothing committed, pushed or posted; both trees restored.

Test worth keeping (mesh `curved::tests`, with that module's helpers): `solid(&[(1.0,0.0),(3.0,0.0),(2.0,1.0),(1.5,1.0)], Revolution::Full)`, split each line edge of a cone face at 0.5 via `split_each_edge_then_place`, and assert `crate::tessellate(placed, 0.01, Tol::witness())` is `Ok` and `check_mesh` passes. Green on 9391ee0177, red (SeamTwice) on the head.
