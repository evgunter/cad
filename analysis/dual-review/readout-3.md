# Dual Opus review — readout 3

Off-file (rule 10 of `docs/DUAL-REVIEW-PROTOCOL.md`). Owed under rule 9
as Ev re-set it on PR 3704 (protocol commit `7cb05367ef`, unchanged on
main since): "a readout is owed when TWENTY M-tier units have been
recorded under rule 1's arms (holdout and sequential together), or at
the first M-TIER MISS, whichever comes first". The trigger is the first
M-tier miss, DR-104. Five M-tier units are recorded, so the count of
twenty was not reached.

Source: `docs/DUAL-REVIEW-LOG.md` on `origin/join/dr104-holdout-miss` at
`7eefb359d`. That branch holds DR-1..DR-104, the DR-11 later-escape line,
the foot line and the dated note of Ev's readout-2 ruling. Main at
`7266fa335` holds DR-1..DR-103. The branch adds only the DR-104 row and
the new foot line. The protocol is read at `origin/main`. The rows are
read as recorded. Where a cell contradicts another cell or the foot
line, this readout says which one it used.

## 1. The trigger

**Rule 9's definition, verbatim:** "An M-tier miss is either

- a tallied finding in a holdout pair whose other review raised no
  MAJOR at all (taken first, the sequential arm would have shipped it),
  or
- a later escape (rule 11) traced to a sequential unit that meets
  rule 6(b) and (d) and that no review of the unit raised as MAJOR."

**DR-104** (JOIN, PR #4274, 2026-10-07, head `4d985b572d`) meets the
first branch:

- **Unit.** A nested pairing at a shared vertex builds through one
  laminar reading of a plan's holders. `hang_at_shared` replaces
  `hang_in_turned`, which is the code DR-101 added (PR #4249). The
  up-front nested-plan refusal is removed.
- **Class.** M / TRICKY.
- **Arm: HOLDOUT.** The draw byte was 81, and 81 mod 3 = 0. Per the
  class cell, the byte was "drawn at implementer dispatch before
  triage; triage to DUAL was made on the merits". The arm and the byte
  are recorded in the row's class cell, as rule 1 asks. As a holdout,
  the unit ran as a concurrent pair under rule 2.
- **R1:** APPROVE-WITH-FIXES, 1/2/4. M1, executed: on
  `wedge343×wedge330` plus a cube in `ba` U/S, at 42 of 48 poses, two
  turned runs leave a strut-only chain outside every fan, so the inner
  strut mints at the shared vertex. 84 probe lines move from
  `SharedVertexCrossings` to `ClassificationInvariant`. The evidence is
  a depth-two probe of 576 lines plus a holder trace.
- **R2:** APPROVE-WITH-FIXES, 0/2/3. It ran 4- and 5-cube many-pairs
  families and found refusal→BAD 0 and SOUND→refusal 0. It marked depth
  above one at a shared vertex UNSURE, because it never built a
  depth-two pose.
- **Coding.** A separate Opus coder coded the pair blind, byte 57
  (parity 1: A = R2). R1's M1 is UNILATERAL and tallied: (a)
  unilateral MAJOR, (b) code, (c) one defect with S2 merged, (d)
  executed, (e) fair. The tally cell reads it "apart from the concurrent
  tally (rule 1 Seams)" and marks the M-tier miss.
- **Fair.** Both reviews completed, with no relaxation and no glimpse.
  Each reviewer chose its own probe family.
- **Fix pass.** The cell records only what was sent to the implementer:
  root-cause M1, pin depth above one and `by_strut`, and tell the
  cover and crossing refusals apart. The row records no completed fix
  and no merge.

Rule 9's definition is symmetric: it reads "the other review". In
DR-104, R2 raised no MAJOR. A sequential unit whose single review was
R2's would have had no second review, under rule 1's sequential arm.

## 2. The M-tier arms so far

The foot line counts five units under rule 1's arms: DR-48, DR-53,
DR-60, DR-65 and DR-104. All are M / TRICKY. All draws check: 189 mod
3 = 0, 238 mod 3 = 1, 137 mod 3 = 2, 74 mod 3 = 2, 81 mod 3 = 0.

| row | program, PR | arm (byte) | first review (R1) | second review | what the second added |
|---|---|---|---|---|---|
| DR-48 | CLEAVE #3866 | HOLDOUT (189) | AWF, 0/2 (+7 style) and NOTEs; no MAJOR. 197,181 tokens, 34 min | concurrent R2: AWF, 1/2 (+7 style). MAJOR: a pre-existing wrong answer, executed (`point_in_solid` reads `In` 5e-7 outside; `cast_ray`'s parallel-ray skip on a false premise). 207,216 tokens, 35 min | Coded BILATERAL: R1 raised the same skip premise as a MINOR, by reading. Tally 0. The coder notes that this severity call alone decides 0 vs 1. No unilateral MINOR in either direction |
| DR-53 | CLEAVE #3892 | SEQUENTIAL (238) | AWF, 0/3 (+7 NOTE, +8 style); no MAJOR; no wrong answer in ~150 split poses at f64 and Interval. MINORs: the concave guards cannot see the flip they guard (executed by plant); convex grazes refuse at some cone/fillet azimuths (executed); the audit row (read). 4.64 USD, 25 min | did not run (no MAJOR) | — . The fix pass corrected one part of the reviewer's diagnosis |
| DR-60 | CLEAVE #3924 | SEQUENTIAL (137) | AWF, 0/3 (+4 NOTE, +8 style); no MAJOR; no wrong answer in ~20k fuzz reads and 4,800 face probes. MINORs: global bounds exhaust the piece budget (executed); exhaustion escalates as an in-band reading ("likely"); halving A turns nothing red (executed). 3.49 USD, 27 min | did not run (no MAJOR) | — . The fix pass took far refusals from 103/14.5k to 0 |
| DR-65 | CLEAVE #3981 | SEQUENTIAL (74) | AWF, 0/2 (+6 NOTE, +7 style); no MAJOR; the clamp sound at all six callers. MINORs: stale pin prose (read); the tour's tip ratio drifts with link count (executed). 3.03 USD, 45 min | did not run (no MAJOR) | — |
| DR-104 | JOIN #4274 | HOLDOUT (81) | AWF, 1/2/4: M1 above (executed); cost not recorded | concurrent R2: AWF, 0/2/3; no MAJOR; depth above one UNSURE; cost not recorded | Coded UNILATERAL R1→ M1, **tallied (holdout)**. M-tier miss |

AWF = APPROVE-WITH-FIXES. For a holdout pair, "first" and "second" are
R1 and R2, which are dispatch-order labels only (rule 2). Both reviews
of a holdout pair ran concurrently on one frozen head.

**Counts.**

- **Units: 5.** 2 HOLDOUT (DR-48, DR-104) and 3 SEQUENTIAL (DR-53,
  DR-60, DR-65). That is 5 of the 20 rule 9 names.
- **Sequential arm:**
  - 0 of 3 first reviews raised a MAJOR, so 0 second reviews ran.
  - No later escape (rule 11) is recorded against any of the three.
- **Holdout arm:**
  - 2 of 2 pairs had a MAJOR from exactly one review. In both pairs the
    other review raised none.
  - Tallied findings: 1 (DR-104). DR-48's MAJOR was coded bilateral at
    a severity split (MAJOR against MINOR) and fails 6(a).
  - M-tier misses: 1 (DR-104).
- **Every review in the arms, counted separately:** 7 reviews (3
  sequential plus 4 holdout), of which 2 raised a MAJOR. Both were in
  the holdout pairs: DR-48's R2 and DR-104's R1.

**Rates, with exact (Clopper–Pearson) 95% intervals:**

| measure | value | interval |
|---|---|---|
| holdout pairs with a tallied finding | 1/2 | 0.013–0.987 |
| holdout pairs with any MAJOR | 2/2 | 0.158–1.000 |
| sequential first reviews raising a MAJOR | 0/3 | 0.000–0.708 |
| reviews in either arm raising a MAJOR | 2/7 | 0.037–0.710 |
| M-tier misses per holdout pair | 1/2 | 0.013–0.987 |

**Pre-arm M-letter context.** These rows are concurrent pairs recorded
before rule 1's arms, so they are not part of the arms. Readout 2
(section 6) read 10 fair M-letter pairs in DR-1..DR-30 (DR-1, DR-3,
DR-6, DR-7, DR-10, DR-11, DR-14, DR-19, DR-20, DR-25):

- 1 of the 10 found a MAJOR (DR-11), and 1 was tallied (DR-11).
- In DR-11 both reviewers raised a MAJOR, the bilateral G1, so neither
  review alone was MAJOR-free.
- DR-31..DR-103 hold no pre-arm M-letter pair.

Adding the two holdouts gives 12 fair M-letter concurrent pairs, of
which 3 found any MAJOR (0.250, 0.055–0.572) and 2 hold a tallied
finding (0.167, 0.021–0.484). DR-104 is the only one of the 12 in which
the tallied finding's other review raised no MAJOR.

**What the sample allows.**

- **Exists, not how often.** The rows show that the shape rule 9 names
  exists in this population. DR-104 is one demonstrated, executed
  defect (6(d)) that one complete, fair, unrelaxed review did not raise
  at any severity. In DR-48, a MAJOR-severity wrong answer was seen
  only as a MINOR by the other review.
- **No frequency estimate.** Two holdouts bound the per-pair miss
  probability only to 0.013–0.987.
- **The sequential trigger.** 0/3 sequential first reviews with a
  MAJOR bounds the probability that a first review raises one to below
  0.708. No row yet shows what a sequential second review adds,
  because none has run.
- **Program mix.** Four of the five arm units are CLEAVE and one is
  JOIN. The sequential units are all CLEAVE. The rows cannot separate
  program, difficulty or orchestrator from arm.
- **What the rows cannot say:**
  - whether a sequential first review of DR-104 or DR-48 would have
    been R1's or R2's;
  - whether DR-48's MINOR-graded skip premise would have been fixed in
    a single-review fix pass;
  - whether DR-104's M1 is fixed (its fix pass is not recorded as done).
- **How the next ones accrue.** Fifteen more arm units reach twenty. A
  draw puts a unit in HOLDOUT with probability 86/256 = 0.336, so about
  5 of the 15 would be expected to be holdouts. The window DR-31..DR-104
  (row dates 2026-09-30 to 2026-10-07) holds 6 M-letter dual units in
  74 rows: the 5 arm units plus DR-101.

## 3. The concurrent stream since readout 2

The Seams say "a readout reports holdout pairs apart from H and L
pairs". This section therefore excludes DR-48 and DR-104 (holdout),
DR-53, DR-60 and DR-65 (sequential), DR-101 (excluded, section 4) and
DR-51 (excluded, section 4).

**Population.** DR-31..DR-103 is 73 rows. Removing the six rows above
leaves **67 fair H- and L-letter concurrent pairs**: 62 H and 5 L
(DR-31, DR-32, DR-34, DR-58, DR-88). All carry a fair cell of "yes",
except DR-84 and DR-93, which have no separate fair cell (section 5).
The foot line counts DR-93 as fair, and this readout does the same for
both.

**Tallied findings, new rows: 22, in 15 pairs.**

| row | letter | tallied | finding (raiser) |
|---|---|---|---|
| DR-34 | L | 5 | R1's split-anchor and flush MAJORs; R2's own-space gate, STEP seam and left-placing-mate MAJORs. All executed. The coder notes that merging the two split findings would make it 3 |
| DR-35 | H | 1 | R1: refusals become wrong ∩ bodies (Monte Carlo oracle) |
| DR-37 | H | 1 | R1: the pointed cone splits into wrong halves (62 of 144 cuts) |
| DR-52 | H | 2 | R1: the twin refusal at `Interval` (red head, green base); R2: the sidecar red CI (attributed by mechanism; base not run) |
| DR-54 | H | 1 | R1: the lever MAJOR (seeded fuzz plus a mutant) |
| DR-58 | L | 2 | R1: `Fold` leaves a dangling input; R2: a round trip fails on a shape split admits (predates the PR) |
| DR-75 | H | 1 | R2: four editor-core slow-set rows red at the head |
| DR-76 | H | 2 | R2: `at_infinity_side` misread; R1: check 10 skips a straddling solid |
| DR-77 | H | 1 | R2: an inside-out operand at `Dual64` ships its complement |
| DR-86 | H | 1 | R1: the `stretch_sup` regression. Under the coder's alternative reading it is 0 |
| DR-89 | H | 1 | R1: a concave cut-off swallows end-face features |
| DR-90 | H | 1 | R1: the carrier-circle refusal. If R2's style item is read as a mention, it is 0 |
| DR-93 | H | 1 | R1: `split_cones` reads one cone as two (12 lines refuse) |
| DR-97 | H | 1 | R1: the document path refuses the seamed band |
| DR-98 | H | 1 | R1: the `SetExpression` re-mint |

By dispatch position, the 22 split R1 14 and R2 8. These are position
labels only.

The foot line's tally is 27: readout 2's 5 (DR-11, DR-16, DR-21, DR-24,
DR-29) plus these 22. It does not include DR-104's holdout finding.

**Fair pairs that found any MAJOR, new rows: 36.** Read from the R1 and
R2 cells:

DR-32, DR-33, DR-34, DR-35, DR-37, DR-38, DR-39, DR-42, DR-43, DR-46,
DR-50, DR-52, DR-54, DR-56, DR-58, DR-59, DR-62, DR-66, DR-67, DR-70,
DR-72, DR-74, DR-75, DR-76, DR-77, DR-78, DR-86, DR-88, DR-89, DR-90,
DR-93, DR-95, DR-96, DR-97, DR-98, DR-102.

This set differs from the foot line's list in three places (section
5): the foot line lists DR-57 and DR-48 and omits DR-66.

**Rates, with exact 95% intervals:**

| measure | new (DR-31..103, 67 fair) | cumulative concurrent (95 fair) |
|---|---|---|
| pairs with a tallied finding | 15/67 = 0.224 (0.131–0.342) | 20/95 = 0.211 (0.134–0.306) |
| pairs that found any MAJOR | 36/67 = 0.537 (0.411–0.660) | 48/95 = 0.505 (0.401–0.609) |
| tallied, given the pair found a MAJOR | 15/36 = 0.417 (0.255–0.592) | 20/48 = 0.417 (0.276–0.568) |

The cumulative column adds readout 2's 28 fair pairs (12 with a MAJOR,
5 tallied) and excludes the holdout pairs.

**By letter, new rows:**

| letter | fair pairs | found a MAJOR | pairs tallied | findings tallied |
|---|---|---|---|---|
| L | 5 | 4 (DR-32, DR-34, DR-58, DR-88) | 2 | 7 |
| H | 62 | 32 | 13 | 15 |

**Why MAJORs fell out, new rows.** These are the cases where the
reason is criterion (a) on a severity split or (b)/(d):

- **Bilateral at a severity split (MAJOR against MINOR or NOTE)** fails
  (a) in DR-42, DR-43, DR-46 (one of two), DR-56, DR-62, DR-66 (both),
  DR-67, DR-72, DR-74 and DR-96. The coder flagged DR-66 and DR-96 as
  close calls that would each tally 1 under the other reading.
- **(b) and (d):** DR-33's only MAJOR fails both, as a doc-only finding
  accepted by inspection.
- **(d) alone:** DR-38's R2 stale-Python MAJOR fails (d), on a predicted
  value rather than a red test. The fix pass later confirmed the reds.
- **Ruling question:** in DR-39, (b) also fails on the C4 overlap,
  which went to Ev on PR 3662.

## 4. Exclusions

- **DR-101** (JOIN, PR #4249, M / TRICKY). "Arm not drawn:
  orchestrator's error. Rule 1 asks an M unit for a draw byte at
  dispatch; none was drawn, and the unit ran as a concurrent pair."
  The row excludes it from the tally and the pair count as neither a
  random HOLDOUT nor a SEQUENTIAL unit, and keeps its findings:
  - Its one MAJOR is BILATERAL: three pairs at one vertex move
    refusals to `OK BAD` (R1: 22 of 168 poses; R2: 18 on `tripod`),
    both executed. Tally 0.
  - After the first fix pass, a second FULL review ran on the fix head
    "(the sequential arm's shape, though this row is excluded)". It
    found 0/2/4 and confirmed every prior finding fixed.
  - It is the one recorded instance of a sequential-shaped second
    review. It cannot enter the arm counts.
- **DR-51** (PCERT, PR #3759, H / SOUNDNESS). Excluded under 6(e):
  R1 was truncated. Its review is a draft summary held for a human
  go-ahead, with three checks pending and no severity labels. The count
  is missing data, not 0. R2's only MAJOR was coded bilateral.
- **Earlier exclusions, carried for the cumulative column:**
  - DR-2: method divergence (rules 4 and 6(e)).
  - DR-22: 6(e), R2 truncated.
- **Not exclusions, but outside the pair count by rule 1:** the three
  sequential units (DR-53, DR-60, DR-65).

## 5. Seams and caveats

**Model era.** Every row is dated 2026-09-24..2026-10-07, after the
2026-09-22 Opus 5.5 release. The protocol records no later model change,
so the population spans one era. The Seams' instruction to report eras
separately does not split anything here.

**Protocol versions inside the new rows.**

- DR-31 records `35d8ddab9`.
- DR-32 and DR-33 record `cb47d67c4`.
- DR-34 onward record `7cb05367e(f)`, the arms ruling.
- DR-34 was dispatched under `cb47d67c4`. DR-35, DR-37 and DR-39 were
  dispatched under `29ebcc63e6`.

All of these are H or L, which are concurrent under every version. Rule
1's arms apply from DR-48, the first M-letter unit recorded under
`7cb05367e`.

**Venue and cost units changed within the stream.**

- **Venue.** Reviews ran in the harness, as cloud sessions, or as local
  subagents (DR-50, DR-90).
- **Cost.** Recorded as total tokens, as output tokens only (DR-56,
  DR-59, DR-75), as USD (the three sequential reviews, DR-53, DR-60 and
  DR-65, 3.03–4.64 USD), or not at all (most cloud-session pairs from
  DR-42 on, DR-101 and DR-104).
- **Consequence.** The rows do not give the sequential and holdout arms'
  costs in one unit. DR-48's two holdout reviews ran 197,181 and
  207,216 tokens.

**Arm draws and selection.**

- **DR-48.** The unit was "escalated from a planned single review at
  report time". The row does not say when its byte was drawn relative
  to that escalation.
- **DR-104.** The byte was drawn at implementer dispatch, before triage.
- **DR-53, DR-60, DR-65.** The byte is recorded, but the time of the
  draw is not.
- **DR-101.** No byte was drawn.
- **DR-80.** The tier letter was "recorded at this sync: the dispatch
  log names no letter".

**Rule-10 readings recorded in rows.**

- **DR-31.** The CLEAVE orchestrator read readout 1 after both reports
  and the adjudication, and before the coding. The coding was done by
  a separate blind coder. This is the flag readout 2 asked PR 3678's
  row to carry.
- **DR-34.** The orchestrator read readout 2, at Ev's request, after the
  pre-note, the coding and the adjudication, and before recording.
- **Possible reach.** All four CLEAVE arm units (DR-48, DR-53, DR-60,
  DR-65) post-date the DR-31 reading. The rows do not record which
  orchestrator session dispatched or adjudicated them.

**Coding weaker than rule 7, new rows.**

- **Coded by the orchestrator:** DR-41, DR-42, DR-57 and DR-62.
- **Coding pending, never recorded as done:** DR-56 and DR-59 carry
  "Pre-note by the orchestrator at merge (blinded coding pending)".
  DR-59's MAJOR is MAJOR/MAJOR. DR-56's MAJOR is MAJOR/MINOR, coded
  bilateral in the pre-note.
- **Coder not named** (the cell reads "Coded attribution-stripped, byte
  N"): DR-36, DR-67, DR-68, DR-69, DR-70, DR-71, DR-73, DR-76, DR-77,
  DR-80, DR-83, DR-87 and DR-102. Two of these, DR-76 (+2) and DR-77
  (+1), carry tallied findings.
- **Sequential units** have no correspondence coding, because there is
  no pair.

**Fairness flags inside the fair set (not exclusions).**

- **Name-only glimpses, no findings:**
  - process command lines or directory names: DR-31 (both), DR-32,
    DR-33 (both), DR-34, DR-36, DR-48 (R1 saw R2's cargo line), DR-50
    (plus an all-refs fetch, unread), DR-55, DR-61, DR-86 (both; the
    A/B mapping file was one listing away) and DR-90;
  - past log rows: DR-45, DR-68 and DR-80;
  - post-freeze commit titles: DR-67 and DR-72;
  - a test name already on main: DR-94.
- **DR-70:** sequential under rule 2's late-trigger fallback, because
  of disk space.
- **DR-73:** R2's first session stalled. A fresh session replaced it,
  with a clarifying prompt. The wall clocks do not overlap.
- **DR-34:** the two reviewers reviewed different trees by
  circumstance. R1's merge of main was clean. R2's later merge
  conflicted, and R2 aborted it.
- **DR-62:** R2 ended a turn with a background job against the brief.
- **DR-95:** two review rounds were recorded as one pair. In round 2,
  R2 read earlier-round comments and R1 read none, which was ruled a
  reviewer's choice.
- **DR-91:** R1 read the implementing lane's evidence comments, which
  the brief permitted to both.
- **DR-87:** both reviewers measured on a local merge of PR 4046.
- **DR-33:** a brief inaccuracy, identical in both briefs.

**Orchestrator errors and deviations recorded in rows.**

- DR-101: the arm was not drawn.
- DR-66: "a `design: true` row decided in the orchestrator's brief
  without the designer weighing, recorded as a process deviation".
- DR-33: the C5 count was wrong in both briefs.
- DR-80: no tier letter at dispatch.
- DR-56 and DR-59: blinded coding pending.

**Coverage limits and budget-limited readings recorded in reviews.**

No row records a review cut short by a token budget, apart from the
truncations already excluded. Several record narrower coverage:

- **DR-104, the trigger.** R2 never built a depth-two pose and marked
  the claim UNSURE. That is the region where R1's M1 lies.
- **DR-75.** R1 "ran a narrower set, disclosed". That is the set
  holding R2's tallied red rows.
- **DR-77.** R1 ran no tour or k-lint. R2 skipped Q8 and three crates.
- **DR-33.** Both declined to re-measure C8 under mutex contention.
- **DR-73.** Both had the gate-bypass probe denied.
- **Readings without execution:**
  - DR-88: R2 inspected the line×circle sibling only;
  - DR-92: R2's MINORs are by reading;
  - DR-64: R1's MINORs are read;
  - DR-60: one MINOR is marked "likely";
  - DR-53 and DR-65: one MINOR each is by reading.
- **DR-95.** A usage cap stalled the unit between its two rounds.

**Foot-line discrepancies.** This readout counts from the row cells.

- **DR-57 and DR-66.** The foot line's list of 49 includes DR-57,
  whose R1 and R2 cells both say "No MAJOR". It omits DR-66, whose
  cells read 1/2/6 and 1/2/5. The count of 49 is unchanged by the swap.
- **DR-48 and DR-104 are treated differently.** The 49 include the
  holdout DR-48. The tally of 27 keeps DR-104's holdout finding apart.
  Without DR-48 the concurrent count is 48. Section 3 uses 48.
- **DR-84 and DR-93** merge their correspondence, tally and fix-pass
  text into one cell and have no fair cell.
- **Running figures** in the tally cells are not monotone in row order.
  For example, DR-86 writes "Running: 21" after its own +1, and DR-98
  writes 25 after DR-97's 26. The per-row tallies sum to the foot
  line's 27.

**Escapes.** The only later-escape line is still DR-11's. None is
recorded against an arm unit. Fix passes and delta reviews again found
defects neither reviewer raised. For example:

- DR-34: the eps step found `admit_mate`'s document-order comparison.
- DR-36: a delta review found a false `Miss` at K = 1.5.
- DR-50: a delta review found that fix pass 1 masked the MAJOR.
- DR-57: a delta review found that MI3 does flip.

These are not coded as escapes, and none touches an arm unit.

**A related observation, not a finding.** DR-84 (H, JOIN, PR #4050)
records a bilateral MINOR: "a strut held by a strut held by a fan mints
at the original vertex and refuses `ClassificationInvariant`... the fan
holder read one link deep". DR-104's tallied M1 also reaches
`ClassificationInvariant` through a strut-only chain at a shared
vertex. The rows do not say whether the two share a cause.

## 6. The options for Ev

The arms continue until Ev rules (rule 9). The options are not ranked.
Each gives its cost in reviews per M-letter dual unit, and what it would
measure.

- **Continue the arms unchanged**, and read again at twenty units or at
  the next miss.
  - Cost: 2 × 0.336 + 1 × 0.664 = **1.34 reviews per unit** while no
    sequential first review raises a MAJOR, rising to 2 if every one
    does. The observed rate so far is 0/3.
  - Measures: about 5 more holdouts in the next 15 units, which would
    narrow the miss-per-holdout interval from 2 pairs to about 7. It
    also gives the first data on what a sequential second review adds,
    once one fires.
- **Change the arm split.** For example, byte mod 2 = 0 gives a 0.5
  holdout share; all holdout for a fixed window is another split.
  - Cost: 1.5 reviews per unit minimum at 0.5. 2 per unit all-holdout.
  - Measures: holdouts accrue faster (about 0.5 or 1 per unit), so the
    miss rate narrows sooner. Fewer sequential units, and second-review
    data comes later.
- **Return M units to concurrent pairs.** The arms end, and M pairs
  join the tally and the pair count.
  - Cost: 2 reviews per unit.
  - Measures: the unilateral-MAJOR rate at M directly, comparable with
    the 12 M-letter concurrent pairs above. It gives up measuring the
    sequential arm, which has produced no second review yet.
- **End the sequential arm.** M units take a single review, with no
  conditional second, and leave this log (rule 1: single-review units
  do not enter).
  - Cost: 1 review per unit.
  - Measures: nothing in this log. Only later escapes would speak
    (rule 11), and then without an arm to count against.
- **Stop the experiment.**
  - Cost: none further.
  - Measures: none further. The record stands at 95 fair concurrent
    pairs, 27 tallied findings, 2 holdouts and 3 sequential units.

Further options the rows suggest:

- **Name the "first" review in each holdout pair** with a second
  recorded byte, so that rule 9's counterfactual concerns one review
  rather than "the other review".
  - Cost: one byte per holdout.
  - Measures: what the sequential arm would have shipped on that unit,
    as distinct from what either review alone missed.
- **Score a holdout severity split at the MAJOR line**, the DR-48
  shape: one review's MAJOR is seen at MINOR by the other. Rule 9 does
  not count it, but the sequential arm's second review fires only on a
  MAJOR.
  - Cost: a definition change. It applies retroactively to one row.
  - Measures: how often a single review's grade, rather than its
    coverage, decides whether the second review runs.
- **Fix when the draw happens.** For example, require it at
  implementer dispatch for every M-letter unit, before triage, as in
  DR-104. This addresses DR-101's missing draw and DR-48's report-time
  escalation.
  - Cost: procedure only.
  - Measures: removes a selection seam between the draw and the tier
    decision.
- **Record arm costs in one unit**, as tokens and wall-clock for every
  review including the sequential ones.
  - Cost: bookkeeping.
  - Measures: what the sequential arm saves per unit, which the rows
    cannot give now.
