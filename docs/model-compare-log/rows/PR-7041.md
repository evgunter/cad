---
pr: 7041
protocol: e1c428efbb2
program: tesslane
unit: a-curved-face-is-routed-by-its-domain-not-its-carriers
letter: M
tier: dual
arm: sequential
implementer: Opus 5.5
r1: Fable 5.2
r2: Opus 5.5
---

# PR 7041: a curved face is routed by what its domain is, read once per run

**Unit:** TESSLANE, `a-curved-face-is-routed-by-its-domain-not-its-carriers`: `tessellate::lane_of` chose a curved face's lane by three tests on its carriers' kinds, and the rectangle lane then asked the two shape doors and refused what they did not pass, so a face bounded by rims and rulings that is no rectangle (a pipe wall notched through its seam) reached no lane. The route is now one value per face, made once per run from the doors' answer and the face's stored images, and the rectangle lane takes a token only that answer mints.

**Letter and tier:** M, dual (sequential): every curved face's route moves from three kind tests to one door, so the impact is as broad as the lane, and the logic is a read of one predicate and a table of its causes. Both were logged in the unit's file (`0b544863e8`) before the first draw.

## Draws
All bytes are on `analysis/model-compare/tesslane-2026-11-14`.

| Draw | Byte | Result |
|---|---|---|
| Implementer | 10 | Opus |
| First reviewer | 53 | Fable |

**The second review ran**, by the other model (Opus), on the first fix pass's head, of the whole unit: the first raised a MAJOR. No draw: the protocol names the model.

## Agents
| Role | Model | Where |
|---|---|---|
| Implementer, both fix passes, the integration of PR 7049's route and the closing pass | Opus 5.5 (`claude-opus-5-5`, configured and served) | cloud session |
| First reviewer | Fable 5.2 (`claude-fable-5-2`, configured and served) | cloud session |
| Second reviewer, and its delta re-review | Opus 5.5 (`claude-opus-5-5`, configured and served) | cloud session |
| Orchestrator | Fable 5.2 | cloud session |

## Quality of the implementation
| | First review (frozen head `a55cd5809e`) | Second review (fix head `3494faec95`) | Delta (fix head 2 `68873a44b5`) |
|---|---|---|---|
| Verdict | NOT-MERGEABLE-AS-IS | APPROVE-WITH-FIXES | APPROVE |
| MAJOR / MINOR / NOTE | 1 / 7 / 3, plus 13 style | 0 / 6 / 3, plus 9 style | 0 / 0 / 4 |
| Rubric: idiom, tests, docs | 3 / 2 / 2 | 4 / 3 / 3 | not scored |

**The first review's MAJOR, DEMONSTRATED:** a ball cut by a brick tilted a few billionths of a radian meshes on main (through the sphere face's projected image) and refused `UnsupportedCurvedShape { Escalated }` at the frozen head, eight cells, because the route asked a banded question (is this circle's axis parallel?) ahead of a decided fact the body already stores (the image's kind). The PR had named the case and answered the spec's "pinned on a body if one can be built" with "none". The orchestrator's ruling: the route may read a stored image kind ahead of a banded question, and never defaults on an escalation.

The first review's MINORs, by defect:

- **A refusal class still mixing three things, DEMONSTRATED at the door:** `NotIsoRectangle` answered for off-surface circles, kernel-defect states and decided "no rectangle" alike, at sites the PR's table omitted.
- **The K stream of bodies that mesh on main moved, and memo hits recorded questions, DEMONSTRATED** (24 to 30, 20 to 26, 28 to 32 questions on three bodies).
- **Three behaviours with no row, DEMONSTRATED by mutants:** the kind pre-pass (a mutant red in 0 of 3,205 rows); `Escalated` and `OffSurface` through `tessellate`; the rectangle lane's own shape guard, removed silently against the spec and caught only by incidental guards.
- **A stated cause that was wrong, DEMONSTRATED:** "refuses for want of stored images" for the four D9 witnesses; with images minted they still refuse, for other reasons, and main's recourse text for one was lost.
- **A filed row resting on a false premise, DEMONSTRATED.**

**Spec deviations at the frozen head.** Reported: four D9 refusal digests re-baselined where the spec called a moved byte a stop (the implementer reported and re-baselined instead of stopping; both reviews called that a deviation that owed the stop); three names in the spec that do not exist in the tree; the doors do not read rings. Silent: the removed shape guard.

**The second review found no MAJOR** on the fix head and held the first fix pass's claims with a 340-row differential against main. Its MINORs: off-surface data still classed as a valid "no rectangle" at four door sites, DEMONSTRATED; an escalation in the torus classification read as a class, BY-INSPECTION; docs asserting a K-stream property of memo hits that the code did not have; the D9 file still re-baselined against the spec, DEMONSTRATED; the memo-hit path 1.5 to 2.5 times slower than main's, DEMONSTRATED; a row that cannot go red on its subject, DEMONSTRATED.

**The integration of PR 7049.** PR 7049 merged a second route (a ruled face with a split straight side goes to the chart lane) into main's `lane_of` while this PR was in review. The implementer made it the fourth fact of the one route value, removed main's plumbing for it, and re-pinned three K-stream digests on ruled bodies (main read the split test about nine times per face per run; the route reads it once). Nobody had reviewed that merge, so the orchestrator added a delta re-review the tier did not owe.

**The delta found no MAJOR and no MINOR.** Its 764-row differential against main, 391 of the rows new split-side bodies, matched main in every mesh and every refusal but the four witnesses. It counted the K stream on three trees: on a ruled body with no split side the head's is main's before PR 7049 plus pure insertions; on a split-side body it is a sub-multiset of main's, reordered (80 reads against 720 on one cone). Its four notes: the PR body's K bullet did not cover split-side bodies; two of the moved D9 rows digest alike; a face with no stored image refuses differently by its route, BY-INSPECTION; three door-class mutants red rows of `geom-brep` and none of `mesh`, DEMONSTRATED. It also weighed the orchestrator's D9 ruling: the reason given ("the refusal says what is missing") holds for all four bodies, but minting leads to a mesh for one of the four only.

**The closing merge of main did not compile.** Its one textual conflict was a test list; PR 7054's code named a type through an import this branch had removed, and its unit row called the chord pass by main's signature. The orchestrator found both by a type-check in the closing worktree and fixed them in one commit; the implementer then ran the suite on that head. On it `mesh` whole with the slow set is 528 of 528 at ε 10⁻⁹, 10⁻⁶ and 10⁻¹², PR 7054's rows among them, and no digest moved; the viewer's slow-set rows and `sweep`'s meshing modules were still running when this row was written, and their result is in the PR body.

**Hosted CI:** green on the frozen head, on both fix heads and on the closing head. The closing head's first run was cancelled by the next push. Three in-progress heads of the second fix pass were red on the branch's own code (a `from_f64` gate, a clippy lint in a new test file, a new row over the slow-set threshold), each fixed by the next push.

**Fix passes.** Two, and the integration, all by the implementer's own agent, continued.
- **The first**, nine items, all answered: the route reads stored non-iso image forms before the doors (the eight cells mesh to main's bytes, pinned); `NotIsoRectangle` split into `OffSurface`, `Defect` and `SpanUncertified`; a door-decided face goes to the chart lane only where the lane can read its domain; the route keeps what the doors asked apart from what the domain read cost, and a row holds main's K stream on nine bodies on a miss and on a memo hit; the rectangle lane requires a token only the doors' answer mints; rows for the kind pre-pass and for both refusals through `tessellate`; four issues filed.
- **The second**, ten items, all answered: a nine-point sample decides off-surface, escalated or class at the doors; the branch door's `Ok` means decided; a curved door asked of a plane answers `Defect`; D9 shows the four unminted witnesses with their refusal constants moved and stated, their minted forms as rows of their own, and the PR body carries a "Spec deviations" paragraph; the memo-hit cost measured and filed, not cured; the route's two trimmed arms are a type (`Trim::{Structural, Read}`) and not an `Option` with an `unreachable!`.
- **Closing.** The orchestrator's merge of main and its fix; one commit by the implementer (a sentence in D9 on the two rows that digest alike, and the filing of the delta's note on two readers of image presence).

## Quality of the review
The orchestrator's adjudication is unblinded.

- **TAKEN, first review:** everything: the MAJOR, MINORs 2 to 7; MINOR 8 and the three NOTEs filed, not fixed (a seam-cornered notch whose rectangle neighbour exceeds its certificate on both trees; a guard row red locally and green hosted; the branch door reading an escalation as one branch, which the second fix pass then cured).
- **TAKEN, second review:** all six MINORs. On the D9 finding the orchestrator ruled the form (unminted witnesses restored, constants moved and stated). Not taken: its suggested cure for the memo-hit cost (routing in the parallel stage), filed instead.
- **Wrong in the orchestrator's own notes to the implementer:** a premise about NaN in the nine-point sample (`Real::max` propagates it; the implementer corrected it and pinned the escalation).
- **What each executed.** First review: the eight cells on both trees; 24 notched bodies at four δ and three ε on both trees; five mutants over the suite; the door sites by hand-built carriers; the D9 witnesses with images minted. Second review: a 340-row differential against main; the door classes by probe; the memo-hit path timed on both trees; mutants; the hosted run. Delta: the 764-row differential; K dumps on three trees; nine mutants; door-class and tangent-line probes; warm memo-hit timing on both trees; `geom-brep`, `topo` and `step-import` whole and `mesh` whole at three ε. It did not run the viewer's slow-set rows (disk).
- **Claims.** First review (twelve): 1 falsified (the MAJOR); 2 falsified in part; 3, 4, 9 corrected; 5 held for the route and falsified for the K stream; 6 not sufficient; 7 held (96 of 104 notch pairs mesh right, the other 8 a pre-existing defect it filed); 10 to 12 held or corrected in a detail. Second review (twenty-one): 1 held; 2 falsified in part; 4, 10 corrected; 14 and 16 held with the MINORs; 20 a stale title; the rest held. Delta (twelve): all held but 5 (held, the ruling's reason partly true), 6 (sound, and a symptom of two readers of one fact) and 10 (two sentences of the PR body corrected).

## Correspondence and blind coding
None: the arm is sequential, and the two reviews read different heads. Read unblinded: the second review re-raised, on the fix head, two of the first's findings in narrower form (the door classes; the D9 re-baseline), and its other four MINORs concern code the first review also read and did not flag (the torus classification's escalation, the memo-hit docs and cost, the row that cannot go red).

## Time and tokens
The session records give usage by kind and a cost. The implementer's context was compacted during the second fix pass and its container restarted several times; its figures are the record's cumulative ones.

| Phase | Wall-clock | Tokens |
|---|---|---|
| Implementation (dispatch 18:11:13 to report 23:10:12 UTC, 2026-11-16) | 4 h 58 min 59 s | input 545, output 173,605, cache read 100,970,648, cache write 450,604 ($27.28) |
| First review (23:14:24 to 00:38:06) | 1 h 23 min 42 s | input 166, output 76,848, cache read 22,679,989, cache write 300,041 ($12.41) |
| First fix pass (00:40 to 07:29:15) | about 6 h 49 min | not read at its end |
| Second review (07:32:55 to 10:50:04, an interim at 08:44) | 3 h 17 min 09 s | input 238, output 74,896, cache read 28,095,175, cache write 245,635 ($9.08) |
| Merge of main with PR 7043, and the integration of PR 7049 (08:38 to 13:28:01) | about 4 h 50 min, beside the second review | in the implementer's cumulative figure below |
| Second fix pass (15:42 to 19:51:32) | about 4 h 10 min | implementer cumulative at its end: input 6,271, output 453,293, cache read 291,875,675, cache write 1,494,655 ($79.45) |
| Delta re-review (19:52:55 to 22:42:45, an interim at 20:34:32) | 2 h 49 min 50 s | reviewer cumulative at its end: input 864, output 122,364, cache read 51,735,239, cache write 606,163 ($17.65); the delta turn alone $8.49 |
| Closing pass (22:43 to the merge) | under an hour and a half | implementer cumulative as the record gave it at 23:14, its last turn still open: input 6,329, output 468,405, cache read 304,287,188, cache write 2,458,176 ($89.95) |
| Dispatch to merge | 18:11 UTC on 2026-11-16 to the PR's merge time | |

Waits the agents did not cause, not subtracted: each cloud session builds cold in its own container; the second fix pass's brief and the go-ahead to merge main sat undelivered for about 2 h 10 min because the implementer's turn stayed open after its integration report and the orchestrator's notes were queued behind it; main moved under the branch four times (PRs 7043, 7049, 7052, 7054); the implementer's container restarted during its long runs.

## Relaxations and fairness
- **A delta the tier did not owe.** The second review raised no MAJOR, so the protocol owed nothing more; the orchestrator added a delta by the second reviewer because the merge of PR 7049's route was a routing change nobody had reviewed. Its findings are in this row and in no tally the protocol defines.
- **The second review read a head that main had already moved under.** The implementer merged main (PR 7043) on top of the fix head while the second review ran; the review read the fix head detached.
- **The implementer was sent hosted-CI failures of its in-progress heads** (three, above) and, during the first fix pass, one `now` interrupt asking for a progress note and a freeze on the green head without waiting for its armed runs.
- **The first fix pass ran 6 h 49 min**, most of it armed whole-suite runs on a contended box; either arm would have been briefed the same.
- **The implementer was told not to open the PR and not to wait on hosted CI.** Either arm would have been briefed the same.
- **The orchestrator wrote code on the branch at closing** (the merge of main and the two-line fix that made it compile), where the protocol has the implementer's agent do fix passes. It is in no review's scope; the implementer read it and ran the suite on it.
- No other relaxation. The reviewers read the PR description and the check runs and nothing else of the thread.
