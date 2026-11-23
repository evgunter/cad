1. FINDINGS (A = reviewer-A.md, B = reviewer-B.md; DEM = demonstrated by execution, INSP = by inspection, "-" = not raised)

# | Finding | Coding | A label/sev | B label/sev | Class | A / B evidence
F1 | Nine on-axis T1 cells go Ok -> UnresolvedWinding; PR body silent, no pinned row, open issue file stale | BILATERAL | MINOR-1 (+SILENT deviation) | MAJOR (+SILENT deviation, claims 3/10) | claim + test-gap + doc (both call the refusal itself correct) | DEM / DEM
F2 | emit fold decision on bare `> 0.0` passes all 489 rows (reader 2 unpinned) | UNILATERAL-A | MINOR-2 | - (B: "all caught" on its own 7 mutants; did not run this one) | test-gap | DEM (surviving mutant) / -
F3 | Cone "smaller of two gaps" choice unpinned (larger-gap mutant survives) | UNILATERAL-A | MINOR-3 (first half) | - (B names the choice only as a reading, see F12) | test-gap | DEM (surviving mutant) / -
F4 | Development-vs-chart reading on the cone held by one unit row only | BILATERAL | MINOR-3 (second half) + style "likely" (1.001 one-way sampling) | NOTE + style 7 "likely" + claim 2 | test-gap | DEM / DEM
F5 | No completed hosted CI on the frozen head (run cancelled; green run is on a later commit) | UNILATERAL-A | MINOR-4 | - CONTRADICTED: B says run 43984526769 is on this head, all 15 checks complete | claim | DEM (check-run lookup) / -
F6 | `Side::window` for a side from a lone apex copy: `lay` lays 10 nodes `meets` rejects; doc sentence false | UNILATERAL-A | NOTE-1 | - | code + doc | DEM / -
F7 | Page's "evaluation rounding" sentence not covered or quantified by the allowance | BILATERAL (low confidence, see 6) | NOTE-2 (`coordinate_scale` drops the anchor) | NOTE (margin about a/2 per coordinate) | doc | INSP / DEM
F8 | Band row asserts `Meshed` only; "none laid" and "0.67" read by nothing | UNILATERAL-A | NOTE-3 | - | test-gap | INSP / -
F9 | README clearance bullet "no crease line" untrue of the crease's own nodes | UNILATERAL-A | NOTE-4 (claim 12) | - | doc | INSP / -
F10 | README clearance bullet "reaches 1 + BOUNDARY_CLEARANCE steps" untrue where no node is laid | UNILATERAL-B | - | claim 12 + style 6 "unsure" | doc | - / INSP
F11 | Crease clearance thinly pinned | BILATERAL (medium confidence) | NOTE-5 (one ulp off the line only, none at c*h) | style 7 "likely" + claim 11 (unit row only) | test-gap | INSP / DEM (mutant reds 1 row)
F12 | "Own step" permissive reading: `lay` keeps a two-band location if either band clears (B adds: min gap chosen at a second site, argued separately) | BILATERAL | style "unsure" | claim 2 + style 2 "likely" | code / contract-API | INSP / INSP
F13 | `inside_point` not strictly inside in f64; a miss is now a `Triangulation` refusal where main retried | UNILATERAL-B | - (A claim 5 holds the opposite by inspection, "short of rounding"; 0 `Triangulation` seen) | MINOR | code | - / INSP ("not demonstrated")
F14 | Spline arm: PR gives a measurement and no reason | BILATERAL | claim 3 mention, no severity | SILENT deviation + claim 3 | claim | A measured a reason (962/964 iso-line sides) / INSP
F15 | Rule barely exercised on a spline chart / reviewer's own probes built no trimmed spline face | BILATERAL (low confidence) | claim 3 mention, no severity | NOTE | claim (coverage limit) | DEM (suite instrumented, 328 faces, 0 moved) / DEM (own builds failed)
F16 | `crates/stl/tests/common/mod.rs:44-51` stale comment citing a test that does not exist | UNILATERAL-A | style "sure" | - | doc | INSP / -
F17 | "eight epsilons" prose restates `cert::ROUNDING_ULPS` (validate.rs:347, types.rs:971/972, test literal) | BILATERAL | style "likely" | style 1 "sure" | doc | INSP / INSP
F18 | `UnresolvedWinding { face }` payload carries no triangle, height or allowance | UNILATERAL-A | style "likely" | - | contract-API | INSP / -
F19 | `Winding` / `winding()`: three states, one production caller testing one | UNILATERAL-A | style "likely" | - | code | INSP / -
F20 | `Creases::cut_candidates`: crease node takes the step of the line's first node | UNILATERAL-A | style "unsure" | - | code | INSP / -
F21 | One thing under five names; retired-mechanism vocabulary remains | UNILATERAL-A | style "sure" | - | code (naming) | INSP / -
F22 | `Side::Developed` stores `turn` and `nappe` per side though they are the plane's | UNILATERAL-A | style "unsure" | - | code | INSP / -
F23 | `boundary_clearance()` `cfg(test)` thread-local override inside the production path | BILATERAL | style "unsure" | style 4 "likely" | code | INSP / INSP
F24 | Trimmed lane knows a triangle is unresolved in `emit`, still runs its retry budget | UNILATERAL-A | style "unsure" | - | code | INSP / -
F25 | `Creases::on` exact `contains` on f64 decides crease exemption | UNILATERAL-B | - | style 3 "unsure" | code | - / INSP
F26 | `UnresolvedWinding` doc example names only the frustum cap | UNILATERAL-B | - (A claim 8: "Text true") | style 5 "likely" + claim 8 | doc | - / INSP
F27 | STL doc promises the normal's meaning, but the file stores f32 vertices | UNILATERAL-B | - (A claim 12: STL writer's doc "true") | style 8 "unsure" | doc / contract-API | - / INSP
F28 | `cert::coordinate_scale` doc "largest coordinate norm" reads as max abs coordinate | UNILATERAL-B | - | style 9 "likely" | doc | - / INSP
F29 | PR "Sweep" section checked, nothing missed (an affirmation, no defect) | UNILATERAL-A | style "sure" | - | claim | DEM (grep) / -

2. COUNTS
- Distinct findings: 29 (28 defects plus one affirmation, F29).
- Bilateral: 9 (F1, F4, F7, F11, F12, F14, F15, F17, F23). Three of them (F7, F11, F15) are lower-confidence merges.
- Unilateral-A: 14. MAJOR 0; MINOR 3 (F2, F3, F5); NOTE 3 (F6, F8, F9); style 8 (F16, F18, F19, F20, F21, F22, F24, and the affirmation F29).
- Unilateral-B: 6. MAJOR 0; MINOR 1 (F13); NOTE 0; style 5 (F10, F25, F26, F27, F28).
- Severity divergences on bilateral findings: 3 tier divergences: F1 (A MINOR / B MAJOR), F4 (A MINOR / B NOTE), F11 (A NOTE / B style).
- Two more where A gave no label: F14 (B SILENT deviation), F15 (B NOTE).
- Same tier: F7 (NOTE/NOTE); F12, F17, F23 (style/style, confidence words differ).

3. CLAIMS
1. A held (one exception, NOTE-1) | B held | AGREE
2. A held, two unpinned choices (MINOR-3) | B held, one reading flagged | AGREE on ruling; caveats overlap only in part (F3, F4, F12)
3. A corrected | B FALSIFIED | DISAGREE on label; same evidence (the nine cells), and both report no other new refusal
4. A held (NOTE-3 on the row) | B held on own set | AGREE
5. A held | B held on own set, one concern by inspection (F13) | AGREE on ruling; B's concern runs against A's inspection argument
6. A held for the definition, reader 2 unpinned (MINOR-2) | B held | AGREE on ruling; A's caveat is A-only
7. A held, with NOTE-2 | B held at 1 ulp, margin NOTE | AGREE (margins consistent: A sees flips at 4 to 8 eps*scale, B at about a/2 = 4 eps*s)
8. A held | B held | AGREE (both reached the refusal on the trimmed lane)
9. A held | B held on own set | AGREE
10. A established | B established by measurement | AGREE (same account: apex plus two lattice nodes, 0.002 to 0.005 of an allowance, noise sign)
11. A: 11 mutants, 2 survive | B: 7 mutants, "all caught" | DISAGREE in summary. The sets differ: B ran neither of A's survivors. The shared mutants are killed in both.
12. A one sentence strained (NOTE-4) | B held but for one sentence (unsure) | AGREE on ruling; the sentences differ (F9, F10)
13. A corrected (no completed run on this head) | B gives no ruling word, reports all 15 checks complete | DISAGREE (a factual contradiction over which commit run 43984526769 is on)

4. VERDICT AND RUBRIC
Item | A | B
Verdict | APPROVE-WITH-FIXES | APPROVE-WITH-FIXES
Labelled findings | 0 MAJOR, 4 MINOR, 5 NOTE, 12 style | 1 MAJOR, 1 MINOR, 3 NOTE, 9 style
Idiom/structure | 4 | 4
Test quality | 3 | 4
Doc/comment honesty | 4 | 3

5. TALLY CANDIDATES
None. Condition (a) fails for every finding:
- A raised no MAJOR.
- B's only MAJOR is F1, which is bilateral (A raised the same nine cells as MINOR-1, demonstrated).
- The unilateral, demonstrated test-gap findings (F2, F3) were raised as MINOR.
- B's unilateral MINOR (F13) is by inspection.

6. NOT CODED CONFIDENTLY
- F7: both target the page's "evaluation rounding" sentence, but by different mechanisms (A: the scale drops the anchor; B: the margin is only about a/2). If split, it becomes one UNILATERAL-A NOTE and one UNILATERAL-B NOTE. A's claim 7 measures the same margin as B without raising it as a finding.
- F11: A's point is that the threshold value is unpinned; B's is that only a unit row goes red. I merged them as "crease clearance thinly pinned". If split, each is unilateral (A NOTE, B style).
- F15: B's NOTE is a limit on its own coverage, not a defect in the change. A states a similar limit, but did observe 328 spline faces through the instrumented suite. The merge is loose.
- F3 vs F12: B's style 2 names the min-gap choice as a permissive reading; A raises the same choice as an unpinned test-gap. I split it: the "unpinned" defect is UNILATERAL-A, the reading itself is bilateral under F12. A's own style item covers only the `lay` half of F12.
- F9 vs F10: the same README bullet, different sentences and different inaccuracies; coded as two unilateral findings. Merged, they would be one bilateral doc finding (A NOTE / B style).
- F5: the two reviews contradict each other on fact, and I cannot adjudicate without the repository. Coded per the instrument as unilateral-A with the contradiction noted.
- F1 class: mixed. Both reviewers say the code behaviour is right and the fix is disclosure, a pinned row and an issue-file update. Moot for the tally, since it is bilateral.
- F13, F26, F27: A's "held" or "text true" statements sit against B's findings without addressing them directly. I treated these as non-mentions with soft contradictions, not as raising.
- Evidence labels: A labels NOTE-3 by inspection though it cites a mutant result ("clearance 0 does not red it"). B gives its spline NOTE no evidence label. I kept each reviewer's own label where one was given.
- Endpoints-only mutant: A lists 6 red rows, B reports 4. Both count it as killed; the difference is not coded as a finding.
- Not coded as findings: reviewer-reported "not run" items, PR-reported deviations, A's position-dependence observation (claim 7), A's "side and crease-link refusals have no row" (claim 5; A lists it among PR-reported deviations and calls it right), and B's claim 9 remark that the page permits triangles `MIN_SHAPE` refused.
