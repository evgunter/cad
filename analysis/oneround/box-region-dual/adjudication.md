# box-region (PR 7225): adjudication of the dual review, frozen head 5c3bb9b9

Both reviews delivered (r1 at 04:37Z, r2 at 05:00Z, 2026-11-21) before either was read against the other. Verdicts: r1 APPROVE-WITH-FIXES (no MAJOR, 6 MINOR, notes A–I); r2 APPROVE-WITH-FIXES (1 MAJOR, 6 MINOR, notes 8–11). The orchestrator's adjudication is UNBLINDED (it dispatched both and knows the slot order).

## Correspondence pre-note (before the blind coding)

BILATERAL:
- Surviving mutants in the box rows (r1 MINOR 3, r2 MINOR 3): the placement filter, stage 2 never admitting, the gate dropping `SessionBoxUncovered`, a Zero read as past, the cone's apex dropped from stage 1, the torus compass points. Same severity. r1 alone: stage 3 reading no edge points, and two rows supplied. r2 alone: an uncovered face counted within at stage 1; survival shown over the whole fast set.
- The deferral of `SessionBoxUncovered` behind tier 3 against the README's "before anything is decided" (r1 MINOR 4, r2 MINOR 4), with the wrong recourse for it (r1 inside MINOR 4, r2 MINOR 5). Same severity.
- `face_cover_unproven` as a third statement of one rule, the superset held by hand (r1 MINOR 5, r2 Style 1). Severity diverges: MINOR against a style finding.
- The two `tcost_k3_certificate` rows lose their subject at the default ε (both MINOR 6).
- Hosted CI and the edited slow rows (r2 MINOR 2, demonstrated from the log: 0 tests; r1 NOTE I: could not read the step's output). Severity diverges: MINOR against NOTE.
- Bodies truly past the box refused as a limit, not as past (r1 MINOR 2, spindle and spline, with the unscheduled inside read; r2 NOTE 8, 36 of 365). Severity diverges: MINOR against NOTE.
- An undecided box margin admits (r1 NOTE C, r2 NOTE 9).
- Stages 2 and 3 decide on a body the box has not admitted; `OutsideSessionBox`'s doc now false (r1 Style Q5, r2 NOTE 10 and Style 8).
- The cost of the ask re-measured, in agreement with the author on the common path (r1 NOTE E, r2 C5(b)); r2 alone measured the path where stage 1 does not admit (10–30×).
- How wide the spline class is and whether P3 is right (r1 NOTE F; r2 inside MAJOR 1). See the unilateral MAJOR below for what r1 did not raise.

UNILATERAL, r2 only:
- **MAJOR 1**: valid in-box bodies with a windowless sphere or torus face or a spindle newly refuse as a limit (85 of 391 in-box placements, 77 admitted at the merge base); the PR's report names two spline fixtures. r1's NOTE F discusses the spline class's width and P3 only; it does not raise sphere, torus or spindle faces refusing in-box bodies.
- MINOR 7: a suite fixture reaches stage 2; the far-apex cone class at the origin, its cost and its logging; the unsure seamless-frustum case.
- NOTE 11: a finished spindle body turned rigidly is refused by tier 3 at both commits (main's).
- Brief correction: the "707 m from the origin" premise.

UNILATERAL, r1 only:
- MINOR 1: a torus tilted 1e-6 to 1e-7 rad off a world axis loses its inside read (cancellation in `e − a(a·e)`).
- NOTE A: `boxes.rs`'s "never admits one past it" against the open clockwise-window row, now on a soundness-bearing path.
- NOTE B: the compass points and the cone-apex candidate of the inside read are needed by no body the kernel mints.
- NOTE D, NOTE G (the ledger's increments explained), NOTE H.

TALLY CANDIDATE (rule 6: unilateral, MAJOR, class code/test-gap/contract, deduped, demonstrated, fair pair): **r2 MAJOR 1**. Unilateral as far as sphere, torus and spindle faces go; MAJOR; demonstrated by execution (the 756-placement differential at both commits). Its class is the open point for the blind coder: the reviewer frames the fix owed as "text and rows" (the report to Ev understates the class), which reads as doc- or claim-only (excluded by 6(b)), while what it demonstrates is a behaviour (valid bodies main admits refuse), which is contract. Fair pair: one brief, no relaxation to either reviewer, neither interrupted; r2's review branch was created at main's head rather than the frozen head and it worked in a detached worktree of the frozen head (a setup difference, not a method difference).

## Adjudication (each finding: TAKEN / ARGUED DOWN / DUPLICATE)

Sent to the implementer as F1–F13 in one message at 05:06Z.

| finding | call | fix-pass item |
|---|---|---|
| r2 MAJOR 1 | TAKEN (report rewritten to the class, rows per kind at P2; tighter covers not built in this PR) | F1 |
| r2 MINOR 7 | TAKEN; the seamless frustum is built and, if it refuses at the origin, fixed here | F2 |
| r1 MINOR 4, r2 MINOR 4, r2 MINOR 5 | TAKEN, ruled: the code follows the README (box first and alone); the uncovered refusal carries the cover's reason and its own recourse | F3 |
| r1 MINOR 3, r2 MINOR 3, r1 NOTE B | TAKEN (rows for every survivor; an arm no body needs is deleted or pinned) | F4 |
| r1 MINOR 1 | TAKEN | F5 |
| r1 MINOR 2, r2 NOTE 8 | TAKEN (row; the limit's sentence made true) | F6 |
| r1 MINOR 5, r2 Style 1, 2, 3 | TAKEN (one statement by construction) | F7 |
| r1 MINOR 6, r2 MINOR 6 | TAKEN | F8 |
| r2 MINOR 2, r1 NOTE I | TAKEN as far as the PR can (re-check the step on the new head); the selector's blind spot already has a row on CIW, evidence added | F9 |
| r2 C5(b), r1 NOTE E/G | TAKEN (cost table completed) | F10 |
| r2 NOTE 10, Style 8; r1 Q5, NOTE A | TAKEN (the argument owed, the lever named; docs corrected) | F11 |
| r1 NOTE C, r2 NOTE 9 | TAKEN as a row (P3) | F12 |
| r2 NOTE 11 | TAKEN as a row on the owning slate (P1), main's defect | F13 |
| r1 NOTE D, r2 Style 6, 7 | left to the author | |
| r2's brief correction (707 m) | TAKEN: the orchestrator's premise was wrong; F1 carries the right statement | F1 |
