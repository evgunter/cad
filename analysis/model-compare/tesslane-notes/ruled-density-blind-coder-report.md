CODING OF REVIEWER A vs REVIEWER B (PR 7096, head 4b9c52c3bc). Only the two review files were read.
Key: BI = bilateral, UA / UB = unilateral-A / -B. DEM = demonstrated by execution, INSP = by inspection, "-" = not raised.

1. FINDINGS (one row per distinct defect after dedup)

| # | Finding | Coding | A label | B label | Class | A / B evidence |
|---|---|---|---|---|---|---|
| 1 | One-chord apex cone face with every line edge hair-split: base Ok (34 tris), head UnresolvedWinding. Folds in A's "fan comment false", "unreachable! premise false", and the spec's "a split never makes a face refuse" | UA | MAJOR-1 | - (contradicts: claim 2 HELD, "NO refusal added", unreachable! "true by my proof check") | code | DEM / - |
| 2 | Bored ball (neighbour holds both rims): head returns Ok with a 4-use edge in 30 cells, base in 15; ear argument incomplete; pin row has no such body | UA | MAJOR-2 | - (contradicts: claim 7 HELD, "no edge used more than twice") | code (+test-gap) | DEM / - |
| 3 | Hosted `test (app)` red on the PR's own new row (0.564 s vs 0.5 s, not in slow set), so `gate ok` fails | BI | MAJOR-3 | MINOR; claim 12; silent list | code (CI/test config) | DEM / DEM |
| 4 | Hosted `test (kernel)` red on an existing sweep row at 1.013 s; not this PR's | BI | inside MAJOR-3 | claim 12 | code (not PR's) | DEM / DEM |
| 5 | Six of 7054's rows no longer reach the re-split (two pins left); names and docs stale | BI | MINOR-4 | MINOR; silent; style "sure" | test-gap (+doc) | DEM / DEM |
| 6 | Duplicate-id skip in `walk_face` unreached; its comment asserts a case nothing builds | BI | MINOR-5a | NOTE; style "likely" | code / test-gap | DEM / DEM |
| 7 | Apex singular flags to `row.cert`: mutant survives | BI (low confidence) | MINOR-5b "dead or unpinned" | claim 9 M5 "equivalent", no finding | test-gap | DEM / DEM |
| 8 | README `[mesh.sizing]` sentences made false and not quoted by the PR | BI | claim 10 + silent list, no severity | MINOR; claim 10 FALSIFIED; style "likely" | doc | INSP / INSP |
| 9 | Cited sizing pin is chart-combinatorial; no 3-D or equality row beside the step function | BI | MINOR-6 | inside claim 10 / README MINOR | test-gap | INSP / INSP |
| 10 | Needle refusal has no pin (A adds: unscheduled) | BI | MINOR-7; Q6 "sure" | NOTE; claim 8 | test-gap | INSP / INSP |
| 11 | Both-nappe guard (two rims through the apex) has no row | BI | MINOR-7 (part); claim 6 | style "sure"; M6 unreached | test-gap | INSP / DEM |
| 12 | Undecided-apex fallback has no row (split from A's MINOR-7) | UA | MINOR-7 (part) | - (reached only in B's own probe) | test-gap | INSP / - |
| 13 | Tie-order mutant survives; equivalent | BI | NOTE-8 | claim 9 M1 | none (equivalent) | DEM / DEM |
| 14 | Refusals identical on base: planar face UnresolvedWinding under hair-splits; 0.001° cone SingularUndecided | BI | NOTE-9 | claim 2 | code (base's) | DEM / DEM |
| 15 | Row `a_routed_face_is_chorded_for_the_chart_lane` deleted without mention | UA | claim 8 + silent list, no severity | - ("No assertion was weakened") | test-gap / claim | INSP / - |
| 16 | Stage-1 table has no home in the tree; unit file's "Not measured" still says the walk is unbuilt | UA | claim 11; Q6 "sure" | - | doc | INSP / - |
| 17 | Spec file still reads `status: dispatched` | UB | - | silent list (cosmetic) | doc | - / INSP |
| 18 | Claim 1: cure-1 non-convergence belongs to this refinement rule, not to refinement in general | UB | - (states the opposite for cones) | claim 1, "unsure" | claim | - / INSP |
| 19 | Claim 1: cylinder may differ; the table does not separate walls from frusta | UA | claim 1 | - | claim | INSP / - |
| 20 | Claim 1: cure 2's limit is the development's merge, not held edges | BI | claim 1 | claim 1, "likely" | claim | INSP / INSP |
| 21 | Style: two strip writers; the walk belongs in the rectangle lane; bound stated twice | BI | Q7 "likely" | Q7 "sure" (taste); claim 1 | code (structure) | INSP / INSP |
| 22 | Style: `walked` is a second route predicate inside the lane (re-derives the `lane_of` arm, invisible to 7041's `route_of`, decides sizing from the chord pass) | BI | Q7 "likely" | claim 6 caveat; style "likely" (418-421) | code (structure) | INSP / INSP |
| 23 | Style: `rim_runs` filter copied verbatim into `Rows::of` | BI | Q1 "sure" | "sure" | code | INSP / INSP |
| 24 | Style: `edge_carrier` is a fourth spelling of an edge's certified carrier | UA | Q1 "sure" | - | code | INSP / - |
| 25 | Style: `walk_face` per-triangle loop is a third copy | UA | Q1 "likely" | - | code | INSP / - |
| 26 | Style: "No edge of four triangles" prose restated three times, enforced by nothing (its wrongness is rows 1-2) | UA | Q2 "sure" | - (contradicts: "argument is sound") | doc / code | INSP / - |
| 27 | Style: `unreachable!` is a panic, not a typed refusal (premise held true) | UB | - (flags the same line for a false premise, folded into row 1) | "unsure" | contract-API | - / INSP |
| 28 | Style: trimmed.rs module header never mentions the walk or the split route | BI | Q2/Q5 "sure" | "likely" | doc | INSP / INSP |
| 29 | sizing.rs:20-22 still says trimmed cylinder sizes at headroom | BI | Q2/Q5 "sure" | NOTE | doc | INSP / INSP |
| 30 | Style: `holds_the_count_relation` drops to winding-only, uncounted; ε-dependent `continue`s can read zero bodies | UA | Q3 "likely" | - | test-gap | INSP / - |
| 31 | Style: five work/ files and reach_probes ~:1995 cite renamed or removed rows | UA | Q4 "sure" | - | doc | INSP / - |
| 32 | Style: trimmed.rs ~6,500 lines, third dispatch branch at its tail (split from A's Q8) | UA | Q8 "unsure" | - | code (structure) | INSP / - |
| 33 | Style: `walked()` re-runs on every `face_target` call (perf) | BI | Q8 "unsure" | "unsure" | code | INSP / INSP |
| 34 | Style: `walked()` hand-rolls the apex test; third and fourth spellings | UB | - | "likely" | code | - / INSP |
| 35 | Style: `has_trim_carrier` redundant with the loop's fallthrough | UB | - | "sure" | code | - / INSP |
| 36 | Style: `MissingEntity` is a mis-typed refusal for a `walked` / `Rows::of` disagreement | UB | - (describes the behaviour without objecting) | "likely" | contract-API | - / INSP |
| 37 | Style: ruled_walk invariant text omits the apex row (m=0) | UB | - | "unsure"; claim 3 | doc | - / INSP |
| 38 | Style: PR body "no pin until a non-rectangle row exists" conflicts with the issue file's wording | UB | - (quotes the phrase, raises no conflict) | "unsure"; NOTE wording | claim | - / INSP |

2. COUNTS
- Bilateral: 18 (rows 3-11 except none skipped, 13, 14, 20, 21, 22, 23, 28, 29, 33).
- Unilateral-A: 12. MAJOR 2 (rows 1, 2); MINOR 1 (row 12, a split part); NOTE 0; style 6 (24, 25, 26, 30, 31, 32); no severity label 3 (15, 16, 19).
- Unilateral-B: 8. MAJOR 0; MINOR 0; NOTE 0; style 6 (27, 34, 35, 36, 37, 38); no severity label 2 (17, 18).
- Severity divergences on bilateral findings with labels on both sides: 3. Row 3 (MAJOR vs MINOR), row 6 (MINOR vs NOTE), row 10 (MINOR vs NOTE).
- Softer divergences: row 7 (MINOR vs "equivalent"), row 8 (unlabelled vs MINOR), row 11 (MINOR part vs style), row 29 (style vs NOTE); confidence only on rows 21 and 28.

3. CLAIM RULINGS
1. A: not re-measured, ruling agreed, cylinder caveat. B: CORRECTED, would still rule the walk but in the rectangle lane. PARTIAL: same outcome, different label, opposite reading of why cure 1 fails.
2. A: held except MAJOR-1 and MAJOR-2. B: HELD. DISAGREE.
3. A: held. B: HELD. AGREE.
4. A: held. B: HELD. AGREE.
5. A: held on the mesh integration suite only. B: HELD on mesh, step-import and sweep meshing modules. AGREE.
6. A: held. B: HELD with the 7041 caveat. AGREE (A puts the same caveat under style).
7. A: falsified. B: HELD. DISAGREE.
8. A: corrected. B: CORRECTED. AGREE.
9. A: no ruling word; own mutants, the PR's four not re-run. B: HELD plus own. NOT COMPARABLE as rulings; the shared survivors match.
10. A: "implementer's answer is incomplete". B: FALSIFIED (SILENT). AGREE in substance.
11. A: held. B: HELD. AGREE.
12. A: falsified, as MAJOR-3. B: FALSIFIED (red), as MINOR. AGREE on the ruling, not the severity.

4. VERDICTS AND RUBRIC

| | A | B |
|---|---|---|
| Verdict | NOT-MERGEABLE-AS-IS (3 MAJOR) | APPROVE-WITH-FIXES (no MAJOR) |
| Idiom/structure | 3 | 3/5 |
| Test quality | 3 | 4/5 |
| Doc/comment honesty | 2 | 3/5 |

A states no scale; /5 is assumed.

5. TALLY CANDIDATES: two, both A's.
- Row 1 (A MAJOR-1) meets (a)-(d): unilateral MAJOR, class code, one defect, shown by a base/head run (base Ok with 34 triangles, head `UnresolvedWinding`). The cause is marked "by reading, likely"; the regression itself is executed.
- Row 2 (A MAJOR-2) meets (a)-(d): unilateral MAJOR, class code (+test-gap), counted once, shown by `check_mesh` NonManifoldEdge count 4 in 30 head cells against 15 on base.
- Row 3 (A MAJOR-3) is excluded: it is bilateral, B raising it as MINOR.
- B raised no MAJOR, so there is no B candidate.

6. NOT CODED CONFIDENTLY
- Row 7: both saw the same surviving mutant; A calls it a MINOR defect, B calls it equivalent. I coded it bilateral because B mentions it; under the contradiction clause it could be UA MINOR. The tally is unaffected.
- Row 27 / row 1: both flag the same `unreachable!` for different reasons, and B affirms the premise A calls false. I folded A's part into row 1 and coded B's panic-vs-refusal point as UB; it could be read as one bilateral style item.
- Row 1 reproduction: B's probe lists apex cones, scale 1e3, default ε and every line split at 1e-8 / 1-1e-8 with zero failures, which looks close to A's failing conditions. The two files do not show whether B built A's exact cell (the three-fraction split on a one-chord face), so the non-reproduction is unexplained.
- Row 2 attribution: A says the root defect is main's (the unsplit body is non-manifold on both trees). Only the 15 new cells are the PR's; I kept it as one MAJOR as A labelled it.
- Row 14: A's "Plane face with two hair-splits at one corner" and B's "half frustum, planar face, every line split 1e-8" are assumed to be the same base refusal; the bodies are not provably identical.
- Rows 16 and 17: both concern a stale work/spec file but name different fields and possibly different files; coded as separate.
- Rows 22 and 35 touch the same lines (trimmed.rs:449-458) with different complaints; kept separate.
- Rows 18-20: claim-1 commentary carries no severity; the "opposite" reading in row 18 rests on A's sentence about cones only.
- Claim 5: A reports one Err-to-Ok (`SphereTrimShape`) on the mesh suite; B reports no refusal added or removed over a wider scope. Neither raised it as a finding, and the files do not explain the difference.
- Row 3's defect class (a CI time-budget row) fits "code" only loosely; it does not affect the tally.
