1. FINDINGS (one row each; A-label | B-label; class; evidence A/B; DEM = demonstrated, INSP = by inspection)

 #  Finding | Coding | A label/severity | B label/severity | Class | Evidence A / B
 1  `Ok` + NonManifoldEdge(4) on cone-tip T1 turned-wedge cells, more cells than merge base; unpinned | BILATERAL | MAJOR (+ SILENT deviation) | M1 MAJOR | code (+test-gap) | DEM / DEM
 2  cert_cone certifies non-finite input (f64::max drops NaN); "NaN-sticky" doc overstated | BILATERAL | MINOR + Style 1 (sure) | F3 MINOR | code (+doc) | DEM / DEM
 3  cert_cone axis term under-reports (5.1e-9 rel, ~1e5 allowances) and over-reports up to 14% | UNILATERAL-B | never raised; affirms opposite (max loss 4.8 eps*scale, allowance 8) | F2 MINOR | code | - / DEM
 4  cone_scale doc rationale/precondition does not match the certificate's real inputs | BILATERAL (low confidence, see 6) | Style 9 (unsure) + claim-2 caveat | S4 (likely) + doc half of F2/S3 | doc | DEM (measured, in allowance) / DEM via F2; S4 INSP
 5  Needle cut just above apex: FoldedTriangle at some delta, same on base, unfiled; title/row name "meshes at every delta" overclaims | UNILATERAL-B | - | N1 NOTE + S8 (sure) | code (pre-existing) + claim | - / DEM
 6  Gate red at head: `test (kernel)`, `gate ok`, four non-mesh rows; not checked against main | UNILATERAL-B | - (says CI still running; verdict assumes green) | N2 NOTE | undetermined | - / DEM (red CI rows)
 7  `nv = rows.len()+1` is a filtered count fed to `metric`, which the cone discards (dead input) | BILATERAL | Style 5 (likely) + claim-12 leftover | N3 NOTE | code | INSP / INSP
 8  Two homes for a cone face's rows (`lattice_steps` hv vs `lattice_rows`) | BILATERAL | Style 6 (likely) | S1 (likely) | code | INSP / INSP
 9  Rows test recomputes the implementation's own expressions; pins the formula, not a property | BILATERAL | Style 3 (likely) + rubric | S2 (sure) + rubric | test-gap | DEM (mutants red only there) / DEM (floor mutant)
10  432-run "finer delta never refuses" claim is a one-off with no guard | BILATERAL | Style 8 (sure) | S7 (sure) | test-gap | INSP / INSP
10a "No pinned count moved" also one-off (split from A Style 8) | UNILATERAL-A | Style 8 (sure), part | - | test-gap | INSP / -
10b Refinement trial also one-off, no guard (split from B S7) | UNILATERAL-B | - | S7 (sure), part | test-gap | - / INSP
11  No mesh-level row sees a certificate mutant (x0.9, axis term, floor) | BILATERAL | unlabelled (claim 8 + rubric) | unlabelled (claim 8 + rubric) | test-gap | DEM / DEM (surviving at mesh level)
12  "Refinement does not converge" shows only how it was tried (one centroid per round, retry budget) | BILATERAL | unlabelled (claim 9, "NOT RE-RUN") | claim 9 "corrected" | claim | INSP / INSP
13  Thin-triangle ratio falls ~30x per decade of delta; reaches MIN_SHAPE near delta 1e-8 / 3e-8 | BILATERAL | Style 11 (unsure) | unlabelled (claim 10) | code (margin) | measured + extrapolated, both
14  Unturned wedge refuses in the boolean (GermSectionOpenConic); that half of the PR's report not reproduced | BILATERAL | claim 11 "CORRECTED" | bullet inside M1 | claim | DEM / DEM
15  Filed work row still `status: parked`, title still says "refuses" | UNILATERAL-A | NOTE (+ SILENT deviation) | - | doc | INSP / -
16  `lattice_rows` sag uses cancelling `1-cos` form; cert uses `2 sin^2`; spelled two ways | UNILATERAL-A | Style 2 (likely) | - | code | INSP / -
17  `lattice_rows` doc reads as a bound but is a heuristic (2x target at span -> pi); over-long | UNILATERAL-A | Style 4 (unsure) | - | doc | INSP / -
18  `lattice_rows` doc writes `1+cos(du/2)` where the certificate has `|cos|` | UNILATERAL-B | - | S9 (unsure) | doc | - / INSP
19  Work file `one-element-grid-axes-drop-schedule.md`: stale `trimmed.rs:737`, stale sentence | UNILATERAL-A | Style 7 (sure) | - | doc | INSP / -
20  reach_probes `geometry` samples a 6-grid; "within delta, sampled" is weak (pre-existing) | UNILATERAL-A | Style 10 (unsure) | - | test-gap | INSP / -
21  `cert_cone` doc ~60 lines for ~35 of code (its false sentences are row 3/4) | UNILATERAL-B | - | S3 (sure) | doc | - / INSP
22  Break-it sweep oracle too coarse (24-step grid, 1e-9); one pose holds axis-term tightness | UNILATERAL-B | - (says only that the oracle is independent) | S5 (likely) | test-gap | - / INSP
23  PR sweep sentence on the curved lane untrue of the split-side cone arm; unswept | UNILATERAL-B | - | S6 (likely) | claim/doc | - / INSP
24  `root` signed needlessly; axis block gate on `pole == [false;3]` redundant | UNILATERAL-B | - | S10 (unsure) | code | - / INSP
25  Floor compares against std `.cos()` while the replay note leans on `Real`'s libm | UNILATERAL-B | - | S11 (unsure) | code | - / INSP
26  Over-long doc lines, chart_table.rs:13-15 and 349 | UNILATERAL-B | - | S12 (sure, trivial) | doc | - / INSP
27  Spindle near its apex unmeasured (box bound, possibly conservative) | UNILATERAL-A | unlabelled (claim 13 remark) | - | claim | INSP / -

Dedup applied: A Style 1 -> row 2; B S8 -> row 5; A's "T1 meshes unpinned" SILENT item and B's "no row covers M1" -> row 1; B S3's false-sentence half -> rows 3/4.

2. COUNTS
- Bilateral: 11 (rows 1, 2, 4, 7, 8, 9, 10, 11, 12, 13, 14); 10 if row 4 is coded unilateral.
- Unilateral-A: 7 = MAJOR 0, MINOR 0, NOTE 1 (15), style 5 (10a, 16, 17, 19, 20), unlabelled 1 (27).
- Unilateral-B: 11 = MAJOR 0, MINOR 1 (3), NOTE 2 (5, 6), style 8 (10b, 18, 21, 22, 23, 24, 25, 26).
- Severity divergences on bilateral findings: no MAJOR/MINOR label diverges. Lesser divergences:
  - row 7: A style (likely) vs B NOTE.
  - row 4: A style (unsure) vs B style (likely) plus part of a MINOR.
  - row 13: A style (unsure) vs B unlabelled remark.
  - row 12: A unlabelled vs B "corrected".
  - row 9: confidence only, likely vs sure.
- Row 1 has the same label but different weight and extent:
  - A treats it as approve-with-fixes ("you may weigh it as blocking"); B treats it as blocking.
  - A reports 4 non-manifold head cells (3 new), B reports 8 of 12 (7 new, adding s=0.1 and s=1e-3).
  - A says a tracker issue is open but does not cover it; B says "No tracker row found".
  - A lists it as a SILENT deviation; B says "SILENT: none".

3. CLAIMS (A ruling | B ruling | agree?)
 1  held, own 2400-pose oracle | held, 150 000 triangles, "except F2's pose" | agree (B with an exception)
 2  held with caveat (rounding at large |u|; non-finite) | falsified in the letter (F2, F3) | DISAGREE
 3  held (new above old on 0/2400) | held | agree
 4  held, by inspection | held, with notes (209/214 cells; N1) | agree
 5  held, demonstrated (21 meshes bit-identical; spline by inspection) | held (per-patch digests, whole suite) | agree
 6  held, demonstrated at three eps | held at default eps only; other eps not run | agree (B narrower)
 7  held as far as run, "see the MAJOR" | falsified in part (M1) | DISAGREE in label, same substance
 8  held, demonstrated, 7 mutants | no ruling word; 7 mutants, same reds | agree in substance
 9  not re-run; non-convergence is how it was tried | corrected; not re-run, same reading | same substance, labels differ
10  held | held | agree
11  corrected + demonstrated -> MAJOR | reproduced, "this unit widens it" (M1) | agree
12  held, by inspection | true of the code (emit pass read) | agree
13  held/unsure (spindle unmeasured) | held by inspection, one correction (S6) | agree, different caveats

4. VERDICTS AND RUBRIC
                    A                                           B
 Verdict            APPROVE-WITH-FIXES (assumes green CI)       NOT-MERGEABLE-AS-IS (M1 + red gate); cert and rows "APPROVE-WITH-FIXES grade"
 Idiom/structure    4                                           4
 Test quality       3                                           3
 Doc honesty        4                                           3

5. TALLY CANDIDATES
None. Criterion (a) fails for every finding: the only MAJOR in either review (row 1) is BILATERAL, both at MAJOR, both demonstrated.
Nearest misses, recorded only:
- row 3 (B F2): unilateral, code, demonstrated by a measured wrong value, but raised as MINOR.
- row 6 (B N2): unilateral, red CI rows, but raised as NOTE and class undetermined.

6. NOT CODED CONFIDENTLY
- Row 3 vs row 4 split. B's F2 and A's claim-2 caveat / Style 9 both concern rounding of the azimuth difference that `cone_scale`'s doc does not account for.
  - A's regime is large |u| (300-650), measured inside the allowance; B's is span near pi at |u| of 2-5, measured ~1e5 allowances outside.
  - I coded the code under-report as UNILATERAL-B with a contradiction (A affirms no pose beyond allowance), and the doc mismatch as BILATERAL.
  - The two doc complaints cite different assumptions (A: azimuths of order one; B: "span <= pi" and "relative to its bound"). If those are distinct defects, row 4 becomes one unilateral style item each way and bilateral drops to 10. The tally is unaffected either way.
- Rows 17 and 18 may be one defect. Both are "unsure" items on the same `lattice_rows` doc expression (`1+cos(du/2)` equals `2cos^2(du/4)`), but A objects that it reads as a bound and B that it lacks `|cos|`. Coded as separate unilaterals.
- Row 1 extent. A's "delta 1e-3 is manifold on both" conflicts with B's "s=1e-3 @ all three" only if A ran the turned wedge at s=1e-3. A reports turned-wedge results only for s=0.03 and 0.01, so I read it as a difference in cells run, not a contradiction.
- Row 6. A did not see CI finish, so this is non-observation, not a contrary statement. Its defect class cannot be assigned from the text.
- Rows 11-14 and 27 carry no severity label from one or both reviewers; I recorded them as "unlabelled" rather than assign one.
- Evidence for rows 9 and 13 is mixed: row 9 rests on mutant runs plus reading the test; row 13 on a measured ratio plus extrapolation.
- A could not build a spline trimmed body for claim 5 and B's Nurbs digests were identical. I treated that as a coverage difference, not a finding.