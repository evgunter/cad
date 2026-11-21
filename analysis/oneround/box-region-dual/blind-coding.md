# Blind correspondence coding: PR 7225 at `5c3bb9b9`, reviewers A and B

Coded from `A.md`, `B.md` and `docs/DUAL-REVIEW-PROTOCOL.md` rules 5-7 only. Nothing was run, and the findings are not judged for correctness.
Verdicts (rule 5): A APPROVE-WITH-FIXES (1 MAJOR, 6 MINOR, 4 NOTE, 8 Style); B APPROVE-WITH-FIXES (0 MAJOR, 6 MINOR, 9 NOTE).

## 1. Findings by underlying defect

| # | Underlying defect | A | B | Code |
|---|---|---|---|---|
| 1 | Valid in-box bodies with a windowless sphere, windowless torus or spindle face refuse as a limit where the merge base admitted them; PR body (c) and the filed row report splines only | MAJOR 1 (DEMONSTRATED) | never mentioned | UNILATERAL-A, MAJOR |
| 2 | C5(c)'s one-fixture spline measure is too narrow; the row's P3 is too low and it names too little | inside MAJOR 1 ("suite fixtures stand near the origin", "P3 looks low") | NOTE F (`likely`); widens to imported trimmed NURBS, which A does not name | BILATERAL, MAJOR (part) vs NOTE |
| 3 | A rational arc's net overshoot is (√2−1) of the arc's radius measured to a box face, not "707 m from the origin" | MAJOR 1 tail; calls the brief's premise wrong | NOTE F states the same geometry, does not call the premise wrong | BILATERAL (weak), unlabelled vs NOTE |
| 4 | Hosted CI's "slow-set tests this diff adds or edits" step ran nothing; `interval` and `tools` jobs skipped | MINOR 2 (read the log: "0 tests run") | NOTE I (step ended within its second; "cannot say whether" the rows ran) | BILATERAL, MINOR vs NOTE |
| 5 | Mutants survive the PR's rows | MINOR 3, Style 5, Style 7 (seven survivors) | MINOR 3, NOTE B (headline "three"; seven survivors in its table) | BILATERAL, MINOR vs MINOR |
| 6 | The gate defers `SessionBoxUncovered` behind tier 3, against the README's "before anything is decided"; unguarded, not put to Ev | MINOR 4 (`sure`), Style 4 | MINOR 4 (`likely`) | BILATERAL, MINOR vs MINOR |
| 7 | `SessionBoxUncovered` takes the family recourse "place nearer the origin", which cannot cure it | MINOR 5 (`likely`) | inside MINOR 4, NOTE H | BILATERAL, MINOR vs MINOR sub-item |
| 8 | `tcost_k3_certificate`'s two rows lose their subject at ε = 1e-9; the fixture could have been re-stood | MINOR 6 | MINOR 6 | BILATERAL, MINOR vs MINOR |
| 9 | A fast-suite fixture, and any slender cone at the origin, reaches stage 2: the PR body's sentence is wrong, C6's logging claim fails for such bodies, and the ask costs 3-5x | MINOR 7 (DEMONSTRATED; tail `unsure`) | never raised | UNILATERAL-A, MINOR |
| 10 | The PR's cost table omits the ask where stage 1 does not admit (10-30x the old ask, about a fifth of the gate) | C5(b), Style 4 (unlabelled) | never raised | UNILATERAL-A, unlabelled |
| 11 | A spindle body truly past the box is refused as a limit ("may lie inside"): no inside read | NOTE 8 (23 placements, "by design") | MINOR 2 (DEMONSTRATED) | BILATERAL, NOTE vs MINOR |
| 12 | A torus body truly past the box is refused as a limit: the inside read does not show the far point | NOTE 8 (13 torus-part placements, cause untraced), plus a C2 remark | MINOR 1 (full torus, tilt ≤ 1e-6; cause `likely`) | BILATERAL at symptom level, NOTE vs MINOR |
| 13 | An undecided or escalated box margin admits | NOTE 9 | NOTE C | BILATERAL, NOTE vs NOTE |
| 14 | Stages 2 and 3 take band decisions before the box answers; the doc line "Reported alone, before any check reads a margin" is stale | NOTE 10, Style 8 (`unsure`) | Style Q5 | BILATERAL, NOTE vs Style |
| 15 | Off-unit: a finished spindle body turned rigidly stops being finished (main's) | NOTE 11 | never raised | UNILATERAL-A, NOTE |
| 16 | `face_cover_unproven` is a third spelling of the face-region rule; the superset property is held by hand | Style 1 (Q1 `sure`), C8 | MINOR 5 | BILATERAL, Style vs MINOR |
| 17 | `cover.rs:7` says geom's readings "hold the only kind matches"; the file now holds three | Style 3 | never raised | UNILATERAL-A, Style |
| 18 | `reach_of`'s closure count; `body_reach` is test-only | Style 6 | Style Q7, Idiom rating | BILATERAL, Style vs Style |
| 19 | The filed row's front matter reads `pr: 7140`, not 7225 | "The brief" (unlabelled) | never raised | UNILATERAL-A, unlabelled |
| 20 | `boxes.rs`'s "never admits one past it" overclaims while the clockwise-window row is open | not raised | NOTE A (`likely`) | UNILATERAL-B, NOTE (doc) |
| 21 | Two inside-read arms (torus compass points, cone apex candidate) do nothing in practice and have no row | not raised | NOTE B | UNILATERAL-B, NOTE |
| 22 | Stage 2 runs every face's `face_cover` after the first uncovered face; one log name serves both stages | not raised | NOTE D | UNILATERAL-B, NOTE |
| 23 | The door's stages 2 and 3 run on a different path from the gate's; the to-the-bit row pins only stage 1 | not raised | NOTE A, Q7 | UNILATERAL-B, NOTE |
| 24 | C6's "costs what main's did" is false of time on the common path | not raised | NOTES G and H | UNILATERAL-B, NOTE (claim) |

Notes on the rows where the coding is not obvious:

- **Row 1.** B's nearest text is row 2, which stays inside the spline kind. B's doc rating says the PR body "discloses the spline, spindle and torus-window gaps", and B says it did not re-measure the differential behind C5(c).
- **Row 5.** Shared survivors: the placement filter, stage 2 never admitting, the Uncovered arm dropped, a Zero read as past, the stage-1 cone apex, the torus compass points. A-only: mutant 15 (an uncovered face counted as within at stage 1); B says only "no row reaches `SessionBoxUncovered` at any door". B-only: stage 3 reading no edge points, and two rows that kill three of the survivors.
- **Row 7.** A-only sub-item: the family heading reads "reaches past the session box" over a limit.
- **Row 9.** B says the contrary in two places: C6 "holds for the logs" (NOTE H) and "every cone fixture's apex is a vertex" (NOTE B).
- **Row 10.** B timed only stage-1-admitted bodies and agrees the ask is "1-3% of the gate" (NOTE E).
- **Row 11.** B-only sub-items: a spline face behaves the same way; no `work/` row schedules an inside read; the spline row's "read off points on it" misleads.
- **Row 12.** A's C2 remark names the regime "between nearly along `e` and along `e`" and says it "can only turn a past-the-box refusal into a limit", by a different mechanism (two points kept instead of sixteen). B's cancellation mechanism and threshold are B-only. If the mechanism is taken as the defect, this is UNILATERAL-B at MINOR. The reports cannot show whether A's 13 cases share B's cause.
- **Row 15.** B reports "no other tier-3 finding on any image" across 22 540 asks, football included.
- **Row 16.** B adds a `[one-way.places]` breach (`likely`). Both note the repeated `bounded` test. A-only (Style 2): the fold over faces is written twice.
- **Row 17.** B says `face_extreme_points`' in-file kind match "is allowed" and does not flag the doc line.
- **Row 18.** A-only: `cover_of_edge`.
- **Row 20.** A's C1 takes the clockwise row's "latent" "on its word" and does not flag the sentence.
- **Row 21.** A's mutant 11 survives (row 5), but A does not call the arm dead.
- **Row 23.** A reports contrary evidence: its mutant "the door reading stages 2 and 3 off the unmoved body" went red on 12 rows, and the door agreed with the gate on 756 of 756.
- **Row 24.** A re-timed the same bodies ("the author's figures are inside my noise") and did not flag the wording.

Not coded as findings: both reports' confirmations of C1, C2 and C4 (no admission past the box, the door agreeing with the gate), the re-timings that agree with the author, C9's placement of the variants, and C10's two corrections, which both confirm. Ratings: Idiom A 4 / B 3; Tests 3 / 3; Docs 4 / 4.

## 2. Rule 6 on the one UNILATERAL MAJOR (row 1, A's MAJOR 1)

**(a) UNILATERAL and raised as MAJOR: met, on the reading I picked.**
- Reading 1 (picked): the defect is that non-spline in-box bodies newly refuse and the PR does not say so. A's headline: "The class is not 'spline nets': sphere faces through their poles, torus faces with no window, and every spindle face refuse as a limit inside the box". B never says an in-box sphere, torus or spindle body refuses. B's own rows show the opposite neighbour (a windowed zone admitted) and truly-past bodies only (rows 11 and 12).
- Reading 2: the defect is "C5(c) under-reports the newly refused valid class". B's NOTE F touches that (one fixture "is not a fair measure", P3 "too low", the row should name more), which would make it BILATERAL at MAJOR vs NOTE and fail (a).
- Why reading 1: all of A's measured evidence and its headline concern the kinds B never mentions. The shared strand is coded separately as row 2.

**(b) Defect class: not met; coded claim/doc-only, so excluded.**
- Evidence for claim-only: "The fixes owed are to what the PR tells Ev and to what its rows pin, not to the fold"; "The report back to Ev understates which valid bodies now refuse (MAJOR 1)"; "What is owed before Ev reads it: (c) in the PR body states the class, not the two fixtures"; "The filed row names splines only"; "P3 looks low ... That is the orchestrator's call"; C10: "The rows are true of what they measured and too narrow (MAJOR 1)"; ratings: "the PR body's report to Ev is the weak part (MAJOR 1, MINOR 2)".
- Evidence for the contract/code reading: "P3 looks low for a refusal of valid bodies that main admits"; "Of the 85, 77 were fully admitted at the merge base". On this reading a measured behavioural regression on valid input is contract-class and would count.
- Why claim-only: A asks for no code or test change under MAJOR 1. Every remedy is PR-body text, a tracker row or a priority. A's verdict opens "The predicate is sound as far as I could attack it" and treats the limit refusal as the designed outcome whose reporting is wrong. The "filed row" is a `work/boxes/` tracker row, not a test, so it is not a test-gap either.

**(c) DEDUP: met.** One defect, counted once. Rows 2 and 3 are strands of the same MAJOR and add nothing to the count.

**(d) DEMONSTRATED BY EXECUTION: met for the measurement.** "DEMONSTRATED, sure"; "Measured, same 756 placements at both commits"; "inside the box | 391 pass the box | 306 pass the box, **85 refused as a limit**"; the radius-300 ball wedge at `x = 900` with the refusal text quoted, and "The merge base admits it." These are measured values of behaviour. That the PR body's wording is too narrow is A reading the PR body.

**(e) FAIR PAIR: nothing in either report shows interruption or truncation.** Both end with a completed verdict and a statement that the other reviewer's branch was not read. The scopes differ by each reviewer's own account: A ran three mutants over the whole fast set and a merge-base differential; B lists "the whole fast set or slow set at any ε" as not done. I cannot tell from the reports whether that was choice or constraint, and I did not read rule 4, so the orchestrator must confirm (e).

**Result: does not enter the tally.** I am moderately sure. It is excluded two ways: (b) under reading 1, (a) under reading 2. It enters only if it is held both unilateral and contract-class, meaning the in-box refusals themselves are the defect, not their reporting. That is the one judgement the orchestrator should check.

## 3. Headline, unilaterals, tally

**Bilateral headline.** Both reviewers reach APPROVE-WITH-FIXES. Both report no body past the box admitted, no in-box body called past it, and the placing door agreeing with the gate. They converge on most substantive items: the surviving mutants (the placement filter, stage-2 admission, the Uncovered arm, a Zero read as past, the cone apex, the compass points), the tier-3 deferral against the README, the wrong recourse for Uncovered, the `tcost_k3` rows losing their subject at 1e-9, the empty hosted slow-set step, the third spelling of the face-region rule, admission on escalation, the stale "reported alone" doc, and truly-past spindle and torus bodies reading as a limit. Severity diverges on five rows: A rates higher on row 2 (MAJOR-part vs NOTE) and row 4 (MINOR vs NOTE); B rates higher on rows 11 and 12 (MINOR vs NOTE) and row 16 (MINOR vs Style).

**Unilateral, A only:**
- MAJOR: row 1.
- MINOR: row 9.
- NOTE: row 15 (off-unit).
- Style: row 17.
- Unlabelled: rows 10 and 19.
- Sub-items: mutant 15, the family heading, the fold written twice, `cover_of_edge`.

**Unilateral, B only:**
- MAJOR: none.
- MINOR: none as whole findings. Row 12's mechanism would be one on the alternative reading.
- NOTE: rows 20, 21, 22, 23, 24.
- Sub-items: the stage-3 edge-point mutant and the two killing rows; the spline inside read, its missing `work/` row and the row's "read off points" sentence; the `[one-way.places]` breach; imported trimmed NURBS.

**Tally: 0.** The one candidate is A's MAJOR 1 (row 1): unilateral on the picked reading and demonstrated by measurement, but coded claim/doc-only under (b), so it is recorded and excluded. Fair-pair (e) is unconfirmed from the reports alone.

---

Orchestrator's note (2026-11-21, after the coding was returned). Mapping by the recorded byte 47 (odd): **A is r2, B is r1**. The coder was a subagent of the orchestrator's session on the session's default model, handed the two reports with the reviewer labels and branch names stripped, the protocol's rules 5 to 7, and nothing else.

- **Tally: 0**, as coded. The pre-note's one candidate (r2's MAJOR 1) is excluded under rule 6(b) as claim-only: the reviewer asks for no code or test change under it, and treats the limit refusal as the designed outcome whose reporting is wrong. The coder flags this as the one judgement to check; the orchestrator's pre-note had left the same point open, and its adjudication took the finding as a report-and-rows fix (F1), which agrees with the claim-only reading. Recorded, not counted.
- **Fair pair (e): confirmed.** One brief for both, no relaxation to either, neither interrupted or truncated. The difference in scope the coder notes (one ran mutants over the whole fast set and a merge-base differential, the other did not) was each reviewer's own choice under the same brief.
- Severity diverges on five bilateral rows, as the coding lists them.
