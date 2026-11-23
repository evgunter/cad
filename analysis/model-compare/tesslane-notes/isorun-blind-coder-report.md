1. FINDINGS (one row per distinct defect after dedup). Columns: # | name | coding | A label | B label | class | A evidence / B evidence. DEM = demonstrated by execution, INSP = by inspection, "-" = not raised.
1 | Two-hole pipe wall refuses CertificateExceeded on head, meshes on main (8 cells); no row/issue holds the shape (folds in A's SILENT-1: fork closed on a corpus with no two-hole wall) | UNILATERAL-A | MAJOR | - (claim 1 "Held": affirmative opposite) | code (+test-gap) | DEM (mechanism INSP) / -
2 | Tilt row: 1/cos phi is a fit, the doc's argument does not produce it | BILATERAL | MINOR | MINOR | test-gap | DEM / DEM (arithmetic on the row's measured numbers)
2b | Tilt bound numerically violated past phi~1.18 (split from A's MINOR) | UNILATERAL-A | MINOR | - ("held on my tilts") | test-gap | DEM / -
3 | Holed <=1.4 is a fit to one bar | BILATERAL | MINOR | unlabelled (claim 5 mention) | test-gap | DEM (1.48-1.88 measured) / INSP (own readings 1.24-1.375 stay under)
4 | Notched <=1.1 false in general (six-notch comb 1.30x, 2.1x) | UNILATERAL-B | - ("not broken on my bodies": affirmative opposite) | MINOR | test-gap | - / DEM (reference is a closed form, not measured)
5 | PR body "14 faces denser" omits sweep's 6 | BILATERAL | MINOR | MINOR | claim | DEM / DEM
6 | Main: crossing-rod pair refuses Triangulation on both trees, unfiled | BILATERAL | NOTE | NOTE | code (main's) | DEM / DEM
7 | Main refuses two-hole cells (r=10, delta=0.1) the head meshes (split from A's NOTE) | UNILATERAL-A | NOTE | - | code (main's) | DEM / -
8 | README [mesh.sizing] "lattice keeps clear of the boundary" stale for the cylinder arm | UNILATERAL-A | NOTE | - | doc | INSP / -
9 | Spec's first "Done when" ([mesh.sizing] names the faces the headroom covers) superseded, unsaid | UNILATERAL-B | - | SILENT deviation (unlabelled) | doc/claim | - / INSP
10 | Hosted lint red (payload-rung-sweep --check) is main's, unfiled | UNILATERAL-B | - (reports lint as queued) | NOTE | code (main's CI) | - / DEM
11 | lattice_rows doc says sweep measured at three eps; it was one | UNILATERAL-A | unlabelled (claim 10b, SILENT-3) | - (lists the missing 1e-12 sweep run as reported, not the doc sentence) | doc | INSP / -
12 | Docs "4 to 220 times" against the table's 1.2 to 223 | UNILATERAL-A | unlabelled (claim 10c) | - | doc | INSP / -
13 | PR "391" moved cylinder faces; 390 paired | UNILATERAL-B | - (cites 391 uncorrected) | unlabelled (claim 10) | claim | - / DEM
14 | Row 3 (split points) weak: no mutant reddens it / bound counts every line edge | BILATERAL (low confidence) | unlabelled (claim 5) | style, likely | test-gap | DEM (mutant green) / INSP
15 | Last-attempt mutant reddens as an unreachable! panic, not the row's own assertion | UNILATERAL-B | - | unlabelled (claim 4) | test-gap | - / DEM
16 | Emit comment trimmed.rs:1531-1538 cites the deleted nu check | BILATERAL | style, sure | MINOR | doc | INSP / INSP
17 | curved.rs:1096 "rows every lever*hu" stale | UNILATERAL-A | style, sure | - | doc | INSP / -
18 | UNPROVED_HEADROOM doc: "sizes its lattice" contradicts its next paragraph | UNILATERAL-A | style, sure | - | doc | INSP / -
19 | Nothing argues a rowless CDT stays within the headroom; docs understate it (A at chart_table.rs:384-388, B at the UNPROVED_HEADROOM doc) | BILATERAL (low confidence) | style, likely | style, unsure | doc | INSP / INSP
20 | metric doc inverted (height over step vs step over height) | BILATERAL | style, likely | style, sure (+claim 10) | doc | INSP / INSP
21 | Corpus measurement in lattice_rows doc with no guard | BILATERAL | style, sure | style, sure | doc | INSP / INSP
22 | Cylinder CDT metric is an unchosen artefact (A: power-of-two window height; B: nv=1, also SILENT and absent from "What changed") | BILATERAL | style, likely | style, likely + SILENT | code | DEM in part (heights that refuse) / INSP
23 | Split by surface kind; near-cylinder cone keeps rows (B adds: cone refusal has no row or issue) | BILATERAL | style, unsure | style, likely + unsure (two items) | code | INSP / INSP
24 | nu and pole_side_columns still computed on the cylinder arm, for metric only | UNILATERAL-A | style, unsure | - | code | INSP / -
25 | 13n / 3.25n cap is a fit pinning the implementation | BILATERAL | style, likely (+claim 3) | style, likely (+claim 3) | test-gap | INSP / INSP (both quote measured 3.03-3.15n)
26 | Row 4 pinned digests are a golden, not a relation | BILATERAL | style, likely (+SILENT-2) | style, likely | test-gap | INSP / INSP
27 | TARGET_SHARE = 0.5 hand copy of UNPROVED_HEADROOM | BILATERAL | style, sure | style, sure | code | INSP / INSP
28 | Another FNV-1a mesh digest helper | UNILATERAL-A | style, likely | - | code | INSP / -
29 | review_pick_r2 doc "fewer rays land" is the wrong mechanism | BILATERAL | style, likely | NOTE | doc | INSP / INSP
30 | review_pick_r2 pin comment records main's own red as baseline (split from A's item) | UNILATERAL-A | style, likely | - | doc | INSP / -
31 | id_repeats_apart moved into tests with a stale doc | UNILATERAL-A | style, unsure | - | doc | INSP / -
32 | Circular helper dependency reach_probes <-> rowless_cylinders | UNILATERAL-B | - | style, likely | code (test structure) | - / INSP
33 | Retry-loop doc "drops its lattice corners" untrue on a rowless face | UNILATERAL-B | - | style, unsure | doc | - / INSP
34 | Class pointer: other docs may cite the cylinder lattice (unread, no instance) | UNILATERAL-A | style | - | doc | INSP / -
35 | Unpaired digest records are two random-body tests; PR leaves it unexplained | BILATERAL | unlabelled (claim 2) | unlabelled (claim 2) | claim | DEM / DEM

2. COUNTS
- Distinct findings: 36 (rows 1-35 plus 2b).
- Bilateral: 16 (rows 2, 3, 5, 6, 14, 16, 19, 20, 21, 22, 23, 25, 26, 27, 29, 35).
- Unilateral-A: 13. MAJOR 1 (row 1), MINOR 1 (2b), NOTE 2 (7, 8), style 7 (17, 18, 24, 28, 30, 31, 34), unlabelled in-claim 2 (11, 12).
- Unilateral-B: 7. MAJOR 0, MINOR 1 (4), NOTE 1 (10), style 2 (32, 33), unlabelled in-claim or silent-deviation 3 (9, 13, 15).
- Label divergences on bilateral findings: 4, none involving MAJOR. Row 3 (A MINOR, B unlabelled mention); row 14 (A unlabelled, B style); row 16 (A style, B MINOR); row 29 (A style, B NOTE).
- Affirmative contradictions: row 1 (B "Held"), row 4 (A "not broken"), row 2b (B "held on my tilts"), row 10 (A lists lint as queued).

3. CLAIMS
1. A: FALSIFIED, demonstrated on both trees. B: Held (352 head cells, none two-hole). DISAGREE.
2. A: Held against merged main, unpaired faces explained. B: Held, same explanation. AGREE.
3. A: mechanism not re-derived, count corrected (+6 in sweep), non-convergence not found. B: mechanism held, count corrected (+6), retry budget not exhausted. AGREE on the correction; A did not rule on the mechanism.
4. A: Held (mutant red). B: Held (mutant red, as a panic). AGREE.
5. A: corrected; tilt falsified, holed a fit, notched and split not broken. B: partly falsified; notched a fit, holed a margin, tilt a fit that held numerically, row 3 loose. AGREE on the ruling, DISAGREE on which rows: opposite on notched, opposite on whether the tilt bound breaks.
6. A: Held, reproduced. B: Held, reproduced on both trees. AGREE.
7. A: Held, two leftovers. B: Held, one stale comment. AGREE.
8. A: Held by inspection and run. B: Held. AGREE.
9. A: not falsified, not exercised. B: Held by inspection. AGREE (neither executed).
10. A: mostly true, three corrections. B: mostly true, three corrections. AGREE on the ruling; only "14 faces" is common to both lists.
11. A: no hosted test job ran; lint queued. B: no hosted test job ran; lint FAILED, main's red. AGREE on the ruling, differ on lint.
12. A: right unit "on the measurement"; three silent deviations. B: right unit; two silent deviations. AGREE on the ruling; the silent lists do not overlap, apart from the row 22 metric, which A raised as style.

4. VERDICTS AND RUBRIC
- Verdict. A: NOT-MERGEABLE-AS-IS, on the row 1 MAJOR alone; APPROVE-WITH-FIXES if the trade is ruled accepted and filed. B: APPROVE-WITH-FIXES, no MAJOR.
- Idiom/structure: A 4, B 4.
- Test quality: A 2, B 3.
- Doc/comment honesty: A 3, B 3.

5. TALLY CANDIDATES
One: row 1, the two-hole wall refusal, from reviewer A.
- (a) Unilateral and MAJOR: B never mentions a two-hole wall and rules claim 1 Held, which is an affirmative opposite, not a raise.
- (b) Class is code: a regression against main, with a test-gap side (no row holds the shape).
- (c) Dedup: A's claim 1, its MAJOR finding, SILENT-1 and the style Q6 remark "false of the two-hole wall" are one defect.
- (d) Demonstrated: A reports probes on both trees, 8 cells refusing on the head at eps 1e-9 and 1e-12 that mesh and pass check_mesh on main. Only the mechanism is by inspection.
Nothing from B qualifies, because B raised no MAJOR.

6. NOT CODED CONFIDENTLY
- Row 1 evidence: A says source and raw lines were "sent in my 06:5x message", which is not in the file. I coded DEMONSTRATED on A's report of execution, not on output I saw. A also records its own counter-evidence: main refuses 2 sibling cells the head meshes, and a 160-cell random sample refuses 0 on both trees.
- Rows 8 and 9: both concern README [mesh.sizing] not being reconciled with the unit, but name different sentences. I coded them as two unilaterals; merged, they would be one bilateral (A NOTE, B SILENT), giving bilateral 17, unilateral-A 12, unilateral-B 6.
- Row 19: A's site is the lattice_rows doc, B's is the UNPROVED_HEADROOM doc. I paired them on the shared point that nothing bounds the span on a rowless face. B's item could instead pair with row 18 (same doc, different complaint) or stand alone.
- Row 14: A's remark is that no mutant reddens row 3; B's is that its bound is loose. I paired them as one weak-row defect; they could be two unilaterals.
- Row 6: the refusing cells differ (A: 0.999 across 1 at 0.8 rad, delta 0.3 and 5e-2; B: theta 0.6, A minus B, every delta). I treated them as one main-side defect.
- Rows 2 and 2b: the split follows the instrument, but B's "held on my tilts" cites only phi 0.6 while B's probe list includes 1.2, 1.45 and 1.55, so it is unclear whether B measured the ratio past 1.18.
- Rows 22 and 24 are adjacent (leftover lattice parameters feeding metric); B's nv=1 item may partly cover row 24.
- Unlabelled in-claim items (rows 9, 11, 12, 13, 15, 35) carry no severity; I counted them separately instead of assigning one.
- Factual differences between the reviews that are not findings: sweep cylinder faces moved (A "1,122", B "1,361, the PR's numbers exactly"); sweep meshing modules 66 vs 63, and 601/602 vs 595/596; row 4 digests "three" vs "four"; A could not read scale 1e3 (a helper panics) where B ran it; B has no base differential at 1e-12.
