1. FINDINGS (one row per distinct defect after dedup). Columns: # | name | coding | A label/severity | B label/severity | class | A evidence / B evidence. DEM = demonstrated by execution, INSP = by inspection, "-" = not raised.
1 | Spur cap panics off its two pinned cells (12 of 96 new cells on head, 32 vs base 20; "keeps refusing" holds at pinned cells only) | UNILATERAL-A | MAJOR-1 (also style Q3, silent deviation) | - (states the spur "6 rounds then refuses": contradiction, not a raise) | code (+test-gap) | DEM (spur scan, both trees) / -
2 | Free-side filter held by no row (mutant survives) | BILATERAL | MINOR-1 | M1 MINOR (+ style Q3 sure, silent deviation) | test-gap | DEM / DEM
3 | Three-constraint (centroid) fallback reached by no body or row | UNILATERAL-A | part of MINOR-1 (split) | - | test-gap | DEM (323 cells, none reach it) / -
4 | Rectangle lane has no typed refusal for a point on a constraint (rim split / `joined` debug_assert panic) | BILATERAL | MINOR-3 | inside M1, MINOR (split) | code | INSP / DEM (prefer-constraint mutant panics)
5 | Unit's own delta row (trimmed.rs:5212 ratio) passes with sphere arm at delta/2 | UNILATERAL-A | MINOR-2 | - (ran the same mutant, did not note it) | test-gap | DEM (mutant) / -
6 | Round fires on `Exceeded` only; `Unresolved` gets no point, against README "reads over delta" | UNILATERAL-A | MINOR-4 (+ silent deviation) | - | code / contract-API | INSP / -
7 | Fold drops a correction point's grid corner, re-queued under a new index | UNILATERAL-A | NOTE-1 | - | code | INSP / -
8 | Fold and shared-side budget (6) is one count across all correction rounds | UNILATERAL-A | NOTE-2 | - | code | INSP / -
9 | Lemma row's closing also closes the unsettled `LATTICE_SHARE` bullet | UNILATERAL-A | NOTE-3 first half (split); claim 11 "corrected"; silent deviation | - | doc | INSP / -
10 | 0.5% volume oracle is a fit, not a bound | BILATERAL | NOTE-3 second half; claim 12 | N3 NOTE; claim 12; silent deviation | test-gap | DEM (120 cells, worst 0.292%; prism arithmetic) / DEM (150 readings, max 0.292%) + INSP
11 | `work/tesslane/log.md` entry stands twice | UNILATERAL-A | NOTE-4 | - | doc | INSP / -
12 | Stale "centroid / inside its triangle" prose in `trimmed.rs` | BILATERAL | style Q4 sure | M2 MINOR (+ style Q5 sure) | doc | INSP / INSP
13 | Test name `two_holes_across_a_column_and_a_half…` no longer describes its cells | UNILATERAL-B | - | M3 MINOR (+ style Q5) | doc | - / INSP
14 | "Reads no epsilon" doc imprecise | BILATERAL | claim 7 remark; style Q2 likely | N1 NOTE | doc | INSP / INSP
15 | Fold branch returns before the correction; PR's stated cause for the folds is wrong; precedence unargued | UNILATERAL-B | - | N2 NOTE (+ claim 12, style Q7 unsure) | code / claim | - / INSP
16 | "Is meshed coarser": 13 `mesh` faces hold more triangles | UNILATERAL-A | claim 2 and 13 correction, unlabelled | - (reports "never above 1" per probe meshing; took totals, not per-face) | claim | DEM / -
17 | "Most any face took is 3": the unit's forced row takes 4 | UNILATERAL-A | claims 4 and 13; style Q5 sure | - (affirms 3: contradiction) | claim | DEM / -
18 | "Most measured is 3" has no guard at its claim site | BILATERAL | style Q5 sure | style Q6 likely | doc | INSP / INSP
19 | Correction rate as stated does not hold (A 2.8% overall, `mesh` 4.2%; B `mesh` ~4%, `sweep` ~2.1%) | BILATERAL | claims 4, 12, 13, unlabelled | claim 12, unlabelled | claim | DEM / DEM
20 | Rectangle lane's correction held only by its forced row, no natural body | BILATERAL | claims 5 and 9 ("held by no row") | claims 5 and 9; listed as a reported deviation | test-gap | DEM (mutant) / DEM (mutant)
21 | PR's section 5 list omits two splits inside step functions | UNILATERAL-A | claims 6 and 13, unlabelled | - | claim | INSP / -
22 | "One round each" on the two-hole cells is unpinned | UNILATERAL-A | claims 9 and 13; silent deviation | - | test-gap / claim | INSP / -
23 | `spheres_crossing…` faces fold on the first pass, not "after six fold rounds" | UNILATERAL-A | claim 12 (iii), unlabelled | - | claim | DEM / -
24 | "131 modules / 934 tests" not reproducible (A's set 92 / 811) | UNILATERAL-A | claim 13, unlabelled | - (lists it as "not checked", own set 77 / 709) | claim | DEM / -
25 | Walk and planar lanes run no correction; `CertificateExceeded` doc reworded past them | UNILATERAL-A | claim 5 "corrected"; style Q5 likely | - | claim / doc | INSP / -
26 | PR says only `sweep`'s snowman catches the centroid mutant; `mesh` point counts do too | UNILATERAL-B | - (mutant data agree, not framed as a discrepancy) | claim 9, unlabelled | claim | - / DEM
27 | Correction-point rule has two homes, plus a third spelling of `uv_centroid` | BILATERAL | style Q1 sure | style Q1 sure | code (structure) | INSP / INSP
28 | Two `STEP_SCALE` test thread-locals | BILATERAL | style Q1 sure | style Q1 likely | code (structure) | INSP / INSP
29 | `curved` reads budget and hooks hosted in `trimmed` (`trimmed::tests` made `pub(crate)`) | BILATERAL | style Q1 sure (part) | style Q1 likely (part) | code (structure) | INSP / INSP
30 | Four test seams on the production path; `grid_counts` cfg pair | UNILATERAL-A | style Q1 sure (split from 29) | - | code (structure) | INSP / -
31 | Loop tail (dedupe set and continue condition) written twice | UNILATERAL-B | - | style Q1 likely (split from 29) | code (structure) | - / INSP
32 | `CORRECTED` and two-hole tables pin the rule's arithmetic, not a property | BILATERAL | style Q3 sure | style Q3 likely | test-gap | INSP / INSP
33 | `curved`'s forced row asserts only `check_mesh`, samples no distance | UNILATERAL-B | - | style Q3 sure | test-gap | - / INSP
34 | Doc "one evaluation a side" is wrong | UNILATERAL-A | style Q5 sure | - | doc | INSP / -
35 | `free` and three arrays built for every kept triangle in the hot loop | UNILATERAL-A | style Q7 likely | - | code (perf) | INSP / -
36 | Parameter `attempt` (trimmed.rs:1096) misnamed and misdocumented | UNILATERAL-A | style Q5 sure | - | doc | INSP / -
37 | Closing paragraph triplicated in work files with a stale number; clearance row has no owner or date | UNILATERAL-A | style Q6 likely | - | doc | INSP / -
38 | A face can be triangulated 13 times (7 before); ceiling stated nowhere | UNILATERAL-A | style Q7 unsure | - | code / doc | INSP / -
39 | `trimmed.rs` module header ~330 lines, growing a section per unit | UNILATERAL-B | - | style Q8 unsure | doc | - / INSP
40 | Head red on the fold rows (PR-disclosed) | BILATERAL | verdict blocker, unlabelled | verdict blocker, unlabelled | code | DEM (local + CI) / DEM (local + CI)

2. COUNTS (40 distinct findings)
- Bilateral: 13 (rows 2, 4, 10, 12, 14, 18, 19, 20, 27, 28, 29, 32, 40).
- Unilateral-A: 21 = MAJOR 1 (row 1); MINOR 3 (3, 5, 6; row 3 is a split of MINOR-1); NOTE 4 (7, 8, 9, 11); unlabelled claim-section corrections 7 (16, 17, 21, 22, 23, 24, 25); style 6 (30, 34, 35, 36, 37, 38).
- Unilateral-B: 6 = MAJOR 0; MINOR 1 (13); NOTE 1 (15); unlabelled claim-section correction 1 (26); style 3 (31, 33, 39).
- Severity divergences on bilateral findings: 2 by label — row 12 (A style, B MINOR) and row 14 (A style / claim remark, B NOTE).
- Other divergences on bilateral findings: row 4 is MINOR on both sides, but A has it by inspection and B demonstrated it. Row 20 is an unheld mechanism for A and a "reported, not silent" deviation for B. Rows 18, 28, 29 and 32 differ only in style confidence (A sure, B likely).

3. CLAIM RULINGS
1. A: held in the suites, corrected off them (MAJOR-1). B: held. PARTLY DISAGREE — same five lost faces in the suites; B's probe found none lost.
2. A: held, with a correction to "is meshed coarser" (13 faces). B: held. AGREE on the ruling; A adds a correction B does not have.
3. A: held with gaps (MINOR-3; filter by argument only). B: held with a test gap (M1). AGREE.
4. A: held (2.8%; forced row takes 4). B: held (most 3). AGREE on the ruling; they differ on the maximum rounds.
5. A: corrected (walk and planar do not correct; spur off the suites). B: held. DISAGREE.
6. A: held (PR's list omits two splits). B: held. AGREE.
7. A: held in `mesh` ("reads no epsilon" loose). B: held (N1). AGREE.
8. A: held. B: held. AGREE.
9. A: no single word; mutants listed, three things "held by no row". B: "rows hold most mechanisms; one does not". AGREE in substance; A lists more unheld items.
10. A: held. B: held for `mesh`. AGREE; neither ran the viewer rows.
11. A: held on soundness, corrected on one closing (`LATTICE_SHARE`). B: held. PARTLY DISAGREE.
12. A: held for the suites, with rate, oracle and first-pass-fold corrections. B: corrected in part (wrong cause, rate, oracle). LABELS DIFFER — overlap on rate and oracle; N2 is B-only, row 23 is A-only.
13. A: seven numbers do not hold as written. B: the numbers it could check hold. DISAGREE.
14. A and B: hosted CI red on the fold row alone (budget meter 316/317, `test (app)`), gate red as the aggregate, lint / kernel / wasm green. AGREE.

4. VERDICTS AND RUBRIC
- A: NOT-MERGEABLE-AS-IS, for two reasons: the head's own two red rows and MAJOR-1. With the fold cure in and MAJOR-1 answered it would read APPROVE-WITH-FIXES.
- B: NOT-MERGEABLE-AS-IS "only on the PR's own account" (the two red rows). On the build itself APPROVE-WITH-FIXES, no MAJOR.
- Rubric, A / B: idiom and structure 3 / 3; test quality 3 / 3; doc and comment honesty 3 / 3.

5. TALLY CANDIDATES
One: row 1, A's MAJOR-1 (spur cap panics off its pinned cells).
- (a) Unilateral and MAJOR: B never raises it at any severity, and B's only statement on the spur is that it refuses, which holds at the suite's cells.
- (b) Class code: a panic in `tessellate::joined`'s debug_assert where a typed refusal stood, with a test-gap side.
- (c) Dedup: A's style Q3 item, silent deviation, claim 1 / 5 / 13 corrections and "test worth keeping" all fold into it.
- (d) Demonstrated: a spur scan on both trees, 96 (tip, delta) cells; 12 newly panic on the head and 2 more turn `UnresolvedWinding`.
No other finding qualifies: B raised no MAJOR, and every other unilateral finding is MINOR, NOTE, unlabelled or style.

6. NOT CODED CONFIDENTLY
- Row 1 evidence: A labels it DEMONSTRATED and lists the scan under Executed, but says "source and raw lines in my earlier message", which is not in the file. I coded (d) on the report's statement, not on evidence I saw. A also says the base already panics at 20 cells, so only the 12-cell widening is the unit's.
- Row 4 merge: I joined A's MINOR-3 with the panic B found inside M1 as one defect (rectangle lane lacks the chart lane's typed check). If these are separate, A's MINOR-3 becomes UNILATERAL-A MINOR by inspection and B's panic UNILATERAL-B MINOR demonstrated; the tally is unaffected. A's "lands on an existing vertex, skipped and re-queued" half is A-only either way.
- Row 3: "demonstrated" here is negative evidence (no cell reached the fallback), not a red probe.
- Row 24: B mentions the same count mismatch but files it as "not checked". I coded it UNILATERAL-A; a looser reading of "mention" makes it bilateral.
- Row 26: A's mutant results contain the same fact, but A does not present it as a PR discrepancy. Coded UNILATERAL-B.
- Rows 8 and 15: both concern how folds and corrections interact, but describe different behaviour (budget shared across rounds; fold exits before a correction). Kept separate.
- Rows 16 and 17: B's contrary statements are at a different granularity (per-meshing totals in its probe; natural suite faces), so "contradiction" is partial for 16 and direct for 17.
- Row 40: PR-disclosed and a verdict reason for both; neither gives it a severity label. Included as a finding; dropping it makes 12 bilateral of 39.
- Claim 9: A gives no ruling word; "agree in substance" is my reading.
- Class for rows 27 to 31 and 35: style items about structure or performance, labelled "code (structure/perf)"; none is MAJOR, so no tally effect.
