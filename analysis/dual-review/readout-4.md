# Dual Opus review — readout 4

Off-file (rule 10 of `docs/DUAL-REVIEW-PROTOCOL.md`). This is the first
**full readout** under rule 9 as Ev rewrote it on PR 4283 (2026-10-08,
protocol commit `da5dbd1532`, unchanged on main since). Rule 9 says, verbatim:

> a **full readout** when TWENTY M-tier units have been recorded under
> rule 1's arms (holdout and sequential together), or at the second
> M-TIER MISS after the last full readout's trigger, whichever comes
> first

The foot line puts the twentieth unit at DR-144. Counted from the rows,
the twentieth was DR-136 (section 1).

**Source.** `docs/DUAL-REVIEW-LOG.md` at `3122531ba2` (branch
`nurbs/revolved-point-axis-anchored`). That commit is main at `effba4bf2f`
plus the DR-144 row and the foot line it rewrites. It holds DR-1..DR-144,
the two later-escape lines (DR-11, DR-54), the foot and Ev's two dated
rulings. The protocol is read at `origin/main`. Readouts 1–3 are read
on `analysis/dual-review/readout-1`, `-2` and `-3`. Every count below comes from the row cells. Where a
cell contradicts another cell or the foot, this readout names the cell
it used and lists the disagreement in section 6.

Position labels: in a holdout pair, R1 and R2 are dispatch order only
(rule 2). In a sequential unit, "first" and "second" are the arm's own
order. AWF = APPROVE-WITH-FIXES, NMAI = NOT-MERGEABLE-AS-IS.

## 1. The count, and when it was reached

These rows carry an M letter and a recorded rule-1 draw. Every byte
checks against its arm:

| # | row | program, PR | class | byte (mod 3) | arm |
|---|---|---|---|---|---|
| 1 | DR-48 | CLEAVE #3866 | M / TRICKY | 189 (0) | HOLDOUT |
| 2 | DR-53 | CLEAVE #3892 | M / TRICKY | 238 (1) | SEQUENTIAL |
| 3 | DR-60 | CLEAVE #3924 | M / TRICKY | 137 (2) | SEQUENTIAL |
| 4 | DR-65 | CLEAVE #3981 | M / TRICKY | 74 (2) | SEQUENTIAL |
| 5 | DR-104 | JOIN #4274 | M / TRICKY | 81 (0) | HOLDOUT |
| 6 | DR-105 | PATHS #4276 | M / TRICKY | 208 (1) | SEQUENTIAL |
| 7 | DR-108 | INTENT #4295 | M / STRUCTURAL | 242 (2) | SEQUENTIAL |
| 8 | DR-110 | JOIN #4317 | M / STRUCTURAL | 19 (1) | SEQUENTIAL |
| 9 | DR-118 | GERM #4375 | M / STRUCTURAL | 88 (1) | SEQUENTIAL |
| 10 | DR-122 | INTENT #4405 | M / STRUCTURAL | 180 (0) | HOLDOUT |
| 11 | DR-123 | INTENT #4455 | M / CORRECTNESS | 79 (1) | SEQUENTIAL |
| 12 | DR-125 | INTENT #4464 | M / DESIGN-SPEC | 175 (1) | SEQUENTIAL |
| 13 | DR-128 | GERM #4418 | M / STRUCTURAL | 142 (1) | SEQUENTIAL |
| 14 | DR-129 | INTENT #4463 | M / STRUCTURAL | 20 (2) | SEQUENTIAL |
| 15 | DR-130 | JOIN #4415 | M / CORRECTNESS | 178 (1) | SEQUENTIAL |
| 16 | DR-131 | INTENT #4359 | M / STRUCTURAL | 128 (2) | SEQUENTIAL |
| 17 | DR-132 | INTENT #4355 | M / STRUCTURAL | 62 (2) | SEQUENTIAL |
| 18 | DR-133 | SHELL #4467 | M / SOUNDNESS | 178 (1) | SEQUENTIAL |
| 19 | DR-134 | INTENT #4506 | M / RECORD | 97 (1) | SEQUENTIAL |
| 20 | DR-136 | GERM #4478 | M / TRICKY | 109 (1) | SEQUENTIAL |
| 21 | DR-137 | INTENT #4508 | M / RENAME + one decision arm | 144 (0) | HOLDOUT |
| 22 | DR-143 | GERM #4522 | M / TRICKY | 105 (0) | HOLDOUT |
| 23 | DR-144 | NURBS #4518 | M / NUMERIC | 52 (1) | SEQUENTIAL |

**The rows hold 23 arm units. The foot line counts 20.** The foot's
list leaves out three rows: DR-123, DR-129 and DR-132. Each has an M
letter, a byte and a SEQUENTIAL arm in its class cell. Row numbers
follow main's merge order (rule 8), so by the rows the twentieth arm unit
is **DR-136**, and the full readout was owed when DR-136 was recorded.
DR-137, DR-143 and DR-144 were recorded after that point. This readout
covers all 23 units and gives the twenty-unit figures where they differ.
DR-101 (M / TRICKY, no draw) stays outside the arms, as readout 3 and
the log record it.

**Mix.** 5 HOLDOUT and 18 SEQUENTIAL. The holdout share is 5/23 = 0.217
(Clopper–Pearson 95%: 0.075–0.437). The draw's own rate is 86/256 =
0.336.

- By program: INTENT 9 (DR-108, 122, 123, 125, 129, 131, 132, 134,
  137), CLEAVE 4, GERM 4 (DR-118, 128, 136, 143), JOIN 3 (DR-104, 110,
  130), and one each for PATHS (DR-105), SHELL (DR-133) and NURBS
  (DR-144).
- Of the 18 arm units recorded since readout 3, 9 are INTENT. Every
  CLEAVE unit predates readout 3.

## 2. What the M-tier arms show

### 2.1 Holdout pairs (5)

For each pair: what each review alone would have shipped, and what the
sequential arm would have done with that review as its first.

**DR-48** (CLEAVE #3866). This is readout 3's reading; nothing in the
row has changed since.

- R1 (AWF, 0/2) raised the parallel-ray skip premise only as a MINOR,
  by reading.
- R2 (AWF, 1/2) raised it as a MAJOR. It executed a pre-existing wrong
  answer: `point_in_solid` reads `In` 5e-7 outside.
- Coded BILATERAL at a severity split, so tally 0.
- R1 alone: no MAJOR, so no second review. The row cannot say whether
  a single-review fix pass would have taken the MINOR.
- R2 alone: the MAJOR fires the second review.

**DR-104** (JOIN #4274). The M-tier miss, readout 3's trigger.

- R1 (AWF, 1/2/4): M1, executed. A strut-only chain mints at the shared
  vertex: 84 probe lines go to `ClassificationInvariant`.
- R2 (AWF, 0/2/3): depth above one was UNSURE. R2 never built a
  depth-two pose.
- R2 alone ships M1, and with no MAJOR no second review runs.
- **Update since readout 3.** The row now records the fix and the merge:
  - fix pass `7464d6c0` root-caused M1 to `mint_plans`' sort, and the 84
    lines build SOUND;
  - a full review of the whole unit on that head (the orchestrator's
    call, since a holdout owes none) gave APPROVE 0/1/4;
  - its MINOR went in at `539242ee`, and the unit merged as `53818ba7`.

**DR-122** (INTENT #4405). Agreement.

- R1 (AWF, 0/2/4) and R2 (AWF, 0/2/5) each raised the same MINOR by an
  executed probe of its own: a same-carrier decided joint is recorded
  `SameOriented` where the PR and spec §8 say `Tangent`, and no row pins
  it.
- R1 alone would have shipped R2's unilateral MINOR-2: the PR's
  vocabulary sweep is not clean by its own pattern, with about 30
  surviving lines in `crates/profile/src`.
- R2 alone would have shipped R1's unilateral items:
  - the refusal count misstated;
  - the fillet golden's history deleted;
  - the escalation text still offering a declaration.
- Neither review raised a MAJOR, so either one alone is the unit's only
  review.
- The row records no coding byte and no coder.

**DR-137** (INTENT #4508). Both reviews raised a MAJOR, and the MAJORs
are disjoint.

- R1 (AWF, 1/3/3): `name_words_corpus::SAID_DIGEST` was not re-pinned.
  It is red in the slow set at the frozen head (executed).
- R2 (AWF, 2/2/3):
  - `demos/tour` fails rustfmt, so CI is red (executed);
  - the PR's spec-test-3 deviation is false. `a − b = 0` over two
    toleranced twins bisects to `Budget` (executed). R1 had judged the
    same claim TRUE by inspection.
- Either review alone raises a MAJOR, so the sequential arm would have
  run a second review on the fix head. The rows cannot say whether that
  review would have raised the other review's MAJORs.
- The tally cell lists 3 candidates with (a)–(e) dispositions, and adds:
  "All three were also red on the implementer's own hosted CI or caught
  by it, two before the reports arrived".
- Not an M-tier miss: each review raised a MAJOR.
- Coded by the orchestrating implementer, byte 63.

**DR-143** (GERM #4522). No MAJOR either side. Each review raised its
own MINOR, both executed.

- R1 (AWF, 0/3/3): τ₁ = τ₂ parallel cones refuse R-tan, mislabelled as a
  tangency.
- R2 (AWF, 0/3/1): the search never enforces its "must answer" (the
  skip-first-fold mutant stays green).
- Each review alone would have shipped the other's MINOR, and neither
  would have fired a second review.
- Bilateral: the witness classes are guarded only by the search, with
  each review using a different mutant.
- Coded by the orchestrator, byte 39.

**Findings the sequential arm would have missed** (as the first review
of a sequential unit):

- **DR-104's M1**, if R2 had been the first review. This is the only
  tallied finding in a holdout pair whose other review raised no MAJOR.
- **DR-48's wrong answer**, if R1 had been first. It would not have been
  missed, but it would have been graded below the line that fires the
  second review.
- **DR-137's MAJORs**: undetermined. Both reviews would have fired a
  second review, and the implementer's CI also caught all three.
- **DR-122 and DR-143**: unilateral MINORs only, listed above.

### 2.2 Sequential units (18)

**The first review raised a MAJOR in 4 of 18 units, and a second review
ran in exactly those 4:** DR-125, DR-130, DR-131 and DR-133. In the
other 14 the first review raised no MAJOR, so it was the only review:
DR-53, DR-60, DR-65, DR-105, DR-108, DR-110, DR-118, DR-123, DR-128,
DR-129, DR-132, DR-134, DR-136 and DR-144. No unit skipped a second
review it was owed, and none ran one it was not owed.

What each second review caught:

- **DR-125** (INTENT #4464, M / DESIGN-SPEC, docs and tracker only).
  - First review (`b84fa0c01e`): AWF, 1/11. The MAJOR: D's migration
    placed each construction on an absolute datum against the world.
  - Second review (`0af7f5c39b`, after fix pass 1): NMAI, 2/6. **Two new
    MAJORs**, both spec text:
    - B's kind walk made today's combines of datum-built roots refuse
      before D;
    - test 11 and "After D" overclaimed while `Pattern`/`PlacedUnion`
      place by rule until G.
  - The second review confirmed all eleven of the first review's fixes,
    with one residue.
  - The row: "R2's MAJORs were spec text an implementer would have built
    wrong … neither was raised by R1".
  - In a pair, rule 6(b) would exclude doc-only findings. The row does
    not say whether the two MAJORs were already in the first head or
    came from fix pass 1.
- **DR-130** (JOIN #4415, M / CORRECTNESS).
  - First review (`24aa90e71a`): AWF, 1/2/6. The MAJOR (executed): with
    two in-band shells the quoted tolerance was the first finding's, so
    it was false and depended on operand order.
  - Second review (`a5f10476b7`, the first fix pass's head): AWF, 0/2/3.
    It confirmed the MAJOR resolved by mutant. **No new MAJOR.** Two new
    MINORs, both executed:
    - the first fix pass's revert of the first review's MINOR-2
      re-opened D10's typing: a certified wholly-in-band shell whose
      walk read zero was refused `ResultInvalid` (2 of ~380 in-band
      shells at ε 1e-9; "not a regression against main");
    - the lever quoted a false threshold.
  - New NOTE: the refusal locates no piece.
  - So the second review's new MINOR concerns a hole created by the fix
    pass.
- **DR-131** (INTENT #4359, M / STRUCTURAL).
  - First review (`1ed6477f2`): AWF, 2/5/7. Two MAJORs:
    - M1 (executed): a construction reads the world;
    - M2: no row evaluates a non-identity pose (a test gap; the
      pose-blind mutant survives).
  - Second review (`23bad579f`, after fix pass 1): NMAI, 2/2/2. **Two
    new MAJORs**, both executed by probe:
    - MAJOR-A: split re-points a remainder reader at the part's whole
      posed world;
    - MAJOR-B: inline of an instance placed twice drops a copy and moves
      the product.
  - Also two new MINORs.
  - The second review confirmed APPROVE at `233d312b3` after fix pass 2.
  - The row does not say whether MAJOR-A and MAJOR-B were in the first
    head or came from fix pass 1. Fix pass 1 did change split ("split's
    copy arm dropped").
- **DR-133** (SHELL #4467, M / SOUNDNESS).
  - First review (`fd80908701`): REJECT, 1/1/1. The MAJOR (executed):
    the arc refinement ignored half-edge orientation, so `walls_cross`
    read a 0.06 m overlap as clear.
  - Second review (`1b400e68f4`): AWF, 0/4/1. It confirmed the MAJOR
    fixed. **No new MAJOR.** New MINORs:
    - a contact on `L` was read from one side only, so
      `walls_cross(a, b)` and `walls_cross(b, a)` disagree, at a depth
      no greater than the zero band;
    - the weight term and the hull ball are unpinned (mutants green);
    - the over-coverage text is false past the split budget.

**Summary.**

- Second reviews that raised a new MAJOR: 2 of 4 (DR-125, DR-131).
  Rows: 4 new MAJORs.
- Of those 4: 2 are spec text in a docs-only unit (DR-125), and 2 are
  executed code defects (DR-131).
- Second reviews that raised new MINORs only: 2 of 4 (DR-130, DR-133).
  One of DR-130's MINORs concerns a hole its first fix pass created.

**Sequential units since readout 3 with no MAJOR** (single review). The
closest a first review came to a blocking executed defect, as recorded:

- DR-105: mutant M4 (an in-band end read as no-touch) survives every PR
  row, with 155 silent misses. Graded MINOR, test-gap.
- DR-123: the uniqueness branch is unpinned, and its mutant is green on
  every row. Graded MINOR.

No row records a later defect in any of the 14 no-MAJOR sequential
units (section 3).

### 2.3 M-tier misses

**One: DR-104** (holdout; R1's M1, and R2 raised no MAJOR). It is
readout 3's trigger, ruled on in PR 4283. Since then there have been
none:

- no holdout pair has a tallied finding whose other review raised no
  MAJOR (DR-137's other review raised MAJORs; DR-122 and DR-143 raised
  none on either side);
- no rule-11 escape is recorded against a sequential unit.

So no short readout was owed after PR 4283, and the "second miss" leg
of rule 9 was not reached.

### 2.4 Counts and rates (exact 95% intervals)

| measure | 23 units (rows) | first 20 (to DR-136) |
|---|---|---|
| holdout pairs | 5 | 3 (DR-48, DR-104, DR-122) |
| holdout pairs with any MAJOR | 3/5 = 0.600 (0.147–0.947) | 2/3 |
| holdout pairs with a tallied finding | 2/5 = 0.400 (0.053–0.853): DR-104, and DR-137 per its cell | 1/3 |
| M-tier misses per holdout pair | 1/5 = 0.200 (0.005–0.716) | 1/3 |
| sequential units | 18 | 17 |
| sequential first reviews raising a MAJOR | 4/18 = 0.222 (0.064–0.476) | 4/17 = 0.235 |
| second reviews raising a new MAJOR | 2/4 = 0.500 (0.068–0.932) | 2/4 |
| second reviews raising a new executed code MAJOR | 1/4 (DR-131) | 1/4 |
| initial reviews raising a MAJOR (10 holdout + 18 sequential-first) | 8/28 = 0.286 | — |

- **Reviews run in the arms: 32.** That is 10 in holdout pairs, 18
  sequential first reviews and 4 second reviews.
- **Per unit:** 32/23 = 1.39. Over the first twenty it is 6 + 17 + 4 =
  27/20 = 1.35.

### 2.5 What the sample allows

- **One shape stands, shown once.** Five holdouts contain one case
  (DR-104) in which a complete, fair, unrelaxed review let a
  demonstrated MAJOR through and would not have fired the second
  review. The per-pair rate is bounded only to 0.005–0.716.
- **The conditional second review has now fired four times.**
  - Twice it raised MAJORs the first review had not: DR-125 (spec text)
    and DR-131 (code).
  - Twice it confirmed the fixes and added MINORs: DR-130 and DR-133.
  - The rows do not separate, for DR-125 and DR-131, defects present at
    the first head from defects a fix pass introduced.
- **What the arms cannot see.** No measure in the log says what a
  second review would have found in the 14 units whose first review
  raised no MAJOR. Only holdouts speak to that, and the 3 holdouts with
  a no-MAJOR review (DR-104's R2, DR-122, DR-143) give one MAJOR the
  other review found (DR-104).
- **Confounds.** Program, task class and orchestrator vary with arm.
  - Both sequential units whose second review raised MAJORs are INTENT
    stage-2 or spec units.
  - The three holdouts that raised any MAJOR are CLEAVE (DR-48), JOIN
    (DR-104) and INTENT (DR-137).
  - The rows cannot separate program from arm.

## 3. Escapes (rule 11)

**Recorded later-escape lines.** There are two, and neither is against
an arm unit:

- **DR-11** (M, concurrent, pre-arm). This is readout 2's.
- **DR-54** (H / TRICKY, concurrent). Added since readout 3, found
  2026-10-08, fixed by PR 4357.
  - The escape was created by the unit's first fix pass
    (`31116eeaee`), after the pair had reviewed. The pair reviewed the
    head before that fix pass, and R2's fuzz found 0 misplaced roots
    there.
  - A delta review on the fix head did not meter the roots against the
    truth along the carrier.
  - The line reads it as raised by no review.

**Traceable from the rows and `work/`, not recorded as a rule-11 line.**

- **DR-123** (INTENT #4455) fixed "a silent wrong body on main from a
  merged unit": "B's one overlay keyed by node id served
  `Union[split.0, split.1]` one half twice: volume 0.5, silent".
- `work/intent/a-union-member-is-keyed-by-its-read.md` names unit B as
  `operands-are-reads`, PR 4342, which is **DR-119**.
- `crates/editor-core/tests/intent_s2_b_reads.rs` is stage 2 unit B's
  test file. It carries the fix's row ("where one projection per node
  read one half twice").
- DR-119 is H / STRUCTURAL and excluded as a mixed-model pair. Its R2
  (Opus) raised a related but different MAJOR: the content and naming
  keys ignore a read's port, so the memo serves the wrong half after a
  re-point. Its R1 (Fable) raised none.
- No line under DR-119 names the overlay defect. The rows do not say
  whether the overlay existed at DR-119's frozen head or arrived in its
  fix pass. It is an H-letter, concurrent-tier unit, so it is not an
  M-tier miss under either reading.

**Examined and not escapes.**
`work/germ/main-red-cone-join-lane-guard-passes-after-cone-section-rows.md`
records a red main on 2026-10-09:

- DR-118 (GERM #4375, sequential) pinned three cone-join rows as
  refusing.
- DR-117 (#4352), merged 17 minutes earlier, made them build.
- The issue was closed after measurement: every pose built sound against
  its closed form, the rows were re-pinned, and "No pose built a wrong
  body".
- It is a crossed-merge stale pin with no defect under 6(b)/(d).

Other `work/` items that cite arm-unit PRs are filings the units made
themselves, or notes of what they changed.

**Defects found after review inside an arm unit's own cycle** are not
escapes, but they bear on what one review sees:

- DR-130: a fix-pass-created typing hole, found by the second review;
- DR-104: a post-fix MINOR, found by the extra full review.

## 4. The concurrent tiers (H/L), DR-105..DR-144

**Population.** There are 22 H-letter concurrent rows and no L-letter
rows:

DR-106, 107, 109, 111, 112, 113, 114, 115, 116, 117, 119, 120, 121,
124, 126, 127, 135, 138, 139, 140, 141 and 142.

Excluded: **DR-112** and **DR-119**, mixed-model pairs (Fable + Opus,
against rule 2). That leaves **20 fair pairs.**

**Fair pairs that found any MAJOR: 9.**

- DR-107 (R1 1, R2 2);
- DR-111 (both);
- DR-114 (R1);
- DR-115 (R2);
- DR-116 (both);
- DR-117 (both);
- DR-138 (R1);
- DR-140 (both);
- DR-141 (both).

The other 11 found none: DR-106, 109, 113, 120, 121, 124, 126, 127, 135,
139 and 142.

**Tallied findings: 6, in 5 pairs.**

| row | tallied | finding (raiser) |
|---|---|---|
| DR-107 | 1 | R2: a toleranced dimensionless number loses its distribution through save/load (re-verified red by the coder) |
| DR-115 | 1 | R2: the apex window reads transported rims, so a derived rim lands on the mirror nappe |
| DR-138 | 1 | R1: a derived section's root decision reads `extent_of` at the old edge's parameters, so the graze lever is 2–4.5× weak (measured) |
| DR-140 | 1 | R1: `DerivLadder`'s `Level::next` zeroes orders 2–3 below a discontinuous first derivative. Predates the PR |
| DR-141 | 2 | R1: split breaks a named selection's sharing. R2: inline strands a downstream selection. Distinct defects |

By position, R1 has 3 and R2 has 3.

**Pairs with a MAJOR and no tally.** The MAJOR was bilateral:

- DR-111: MAJOR/MAJOR (the steep-ellipse root);
- DR-114: MAJOR/MINOR (bodies that never tessellate);
- DR-116: MAJOR/MAJOR (limbs 1–2 dropped);
- DR-117: MAJOR/MAJOR (the naive axis projection).

DR-107 and DR-140 also each carry a bilateral MAJOR beside their tallied
one: `INT/INT` re-association (MAJOR/MAJOR), and refine count-pairing
(MAJOR/MINOR).

**Excluded pairs' MAJORs.** These are recorded and not counted. DR-119's
R2 raised two unilateral MAJORs, both executed: the slot door skips the
bound check, and the keys are blind to a read's port. The row disposes
them at 6(e).

**Rates:**

| measure | DR-105..144 (20 fair) | cumulative concurrent (115 fair) |
|---|---|---|
| pairs that found any MAJOR | 9/20 = 0.450 (0.231–0.685) | 57/115 = 0.496 (0.401–0.590) |
| pairs with a tallied finding | 5/20 = 0.250 (0.087–0.491) | 25/115 = 0.217 (0.146–0.304) |
| tallied, given the pair found a MAJOR | 5/9 = 0.556 (0.212–0.863) | 25/57 = 0.439 (0.307–0.576) |
| tallied findings | 6 | 33 |

The cumulative column is readout 3's 95 fair pairs (48 with a MAJOR, 20
tallied, 27 findings) plus these 20. Holdouts are excluded, as the Seams
require.

## 5. Costs

**Reviews.** As counted in section 2.4, the arms ran 32 reviews over 23
units:

- holdout: 10, i.e. 2 per unit;
- sequential: 22 over 18 units, i.e. 1.22 per unit.

Reviews and verifications **outside** the arms' count, on arm units:

- DR-104: one full review after the fix pass (orchestrator's call);
- DR-130: an orchestrator-run Opus verification, APPROVE;
- DR-131: the second reviewer re-confirmed after fix pass 2;
- DR-133 and DR-144: "orchestrator checked the delta".

On concurrent units, extra review lanes are recorded in:

- DR-109 (an independent verifier);
- DR-116 (a confirming review and two delta reviews);
- DR-120 and DR-127 (verification subagents);
- DR-140 (two delta reviews).

**Fix passes on arm units: 27 over 23 units.**

- Every sequential unit with a second review took two passes: DR-125,
  DR-130, DR-131 and DR-133.
- Every other arm unit took one.
- Executors:
  - a fresh Opus lane on DR-105, DR-118, DR-128, DR-136 and DR-143;
  - the implementer or author elsewhere, where the row names one.

**Tokens and wall clock.** These are recorded in mixed units, and for
most rows not at all.

| row | arm | recorded |
|---|---|---|
| DR-48 | holdout | 197,181 / 207,216 tokens; 34 / 35 min |
| DR-53, DR-60, DR-65 | sequential | 4.64 / 3.49 / 3.03 USD; 25 / 27 / 45 min |
| DR-105 | sequential | review 232,349 tokens, 34 min; fix pass 277,935 tokens, 101 min; implementer 327,520 tokens, 104 min |
| DR-108 | sequential | ~252k context tokens, ~97 min |
| DR-110 | sequential | ~203k context tokens |
| DR-123 | sequential | 121,725 tokens, 31 min |
| DR-130 | sequential | first 249k (peak context), 20 min; second 298k, 41 min |
| DR-134 | sequential | ~150k tokens, ~45 min |
| DR-104, DR-122, DR-137, DR-143 | holdout | not recorded |
| DR-118, 125, 128, 129, 131, 132, 133, 136, 144 | sequential | not recorded (DR-129 and DR-132 say so explicitly) |

On the rows that record tokens, a single sequential review ran 120k to
300k tokens, and DR-48's two holdout reviews ran about 200k each. The
rows do not give one unit of cost across the arms. Concurrent rows from
DR-105 on record tokens for about half the reviews, again in mixed
units: total, peak context, or "agent".

## 6. Data quality caveats

**Exclusions.**

- In this period: DR-112 and DR-119, mixed-model pairs.
- Carried: DR-2 (method), DR-22 and DR-51 (6(e)), and DR-101 (M with no
  draw).
- No 6(e) truncation is recorded in DR-105..DR-144.

**Mixed-model pairs.** DR-112 has an R1 from Fable. DR-119 has an R1
from Fable, and its two unilateral R2 MAJORs fall out at 6(e). Both
were "orchestrator error at dispatch". Every other review in the period
is recorded as Opus or is unlabelled. No model change is recorded in
the protocol, so the population is one era.

**Draw timing.**

- Of the 18 arm rows since readout 3, three say the byte was "drawn at
  dispatch": DR-108, DR-110 and DR-131. The other 15 record the byte but
  not when it was drawn.
- No row in the period records a late draw. DR-48's report-time
  escalation and DR-101's missing draw predate it.
- DR-130 and DR-133 both record byte 178.
- DR-119, an H unit, records a byte ("drawn at dispatch") that its arm
  does not use.

**Coding weaker than rule 7.**

- **Holdouts:**
  - DR-137 was coded by the orchestrating implementer. Its three
    candidates are counted in the foot.
  - DR-143 was coded by the orchestrator.
  - DR-122 records no coder and no byte.
- **Concurrent:**
  - **Orchestrator's unblinded pre-note, blinded coding not run:**
    DR-139, DR-141 and DR-142. DR-141's two tallied findings rest on
    this pre-note. DR-139 and DR-142 have no MAJOR, so they are
    unaffected.
  - **Coded by the orchestrator:** DR-117 and DR-135.
  - **Blinded coding by an unnamed coder:** DR-115 (+1), DR-116, DR-126
    and DR-138 (+1).
  - **No coding recorded at all:** DR-121's correspondence cell carries
    no byte and no coder.
  - **Attributed read before blinded coding:** in DR-124 the
    orchestrator read both reports attributed before the coding
    (disclosed).

**Disclosures** (flags inside the fair set, not exclusions):

- **Process names or command lines of the other lane seen:**
  - DR-109: R1 saw the other lane's directory name;
  - DR-117: both reviewers, via `ps`, and R2 also listed the shared locks
    dir;
  - DR-126: both, through a shared build-log name;
  - DR-138: both, via `pgrep`;
  - DR-143 (holdout): R2 saw one process name of R1's lane.
- **File names:** DR-115, both reviewers.
- **Read-only, ruled not a glimpse:** DR-111's R2 ran `git remote -v`
  and listed directory names.
- **A commit subject:** DR-124's R1 saw an orchestrator commit's subject
  line.
- **DR-116.** R2's report was retrieved by request after a stop-hook
  turn buried it.
- **DR-127.** A post-freeze push landed after the freeze instruction.
  The reviews stayed on the frozen head.
- **DR-140.** The first dispatch died at init and was re-dispatched
  identically.
- **DR-142.** The pair reviewed before E merged, on the diff against E's
  head, by the orchestrator's leave.

**Rows rendered from a template.**

- DR-135 and DR-143 read "Running: {tally}", an unfilled placeholder.
- DR-135 also carries "Enclosure {{ lo: …", and DR-143 "e^{{iθ}}".
- DR-115 and DR-140 give no running figure.
- DR-111, DR-113 and DR-114 read "Running: 0", and DR-124 reads
  "Running: unchanged".

**Protocol hashes.**

- `7cb05367ef` is recorded in DR-104..DR-109, DR-112 and DR-116.
- `da5dbd1532` is recorded from DR-110; DR-123 writes it `da5dbd153`.
- `5dedb0a41a` is recorded in DR-124, DR-140 and DR-144. It is PR 4317's
  merge, not a commit that touched the protocol, but the protocol text
  there equals `da5dbd1532`'s.
- `7cb05367ef` and `da5dbd1532` differ only in rule 9's readout point.
  The arms and the instrument are identical across the period.
- DR-108 and DR-112 (dated 2026-10-08) and DR-116 (2026-10-09) record
  `7cb05367ef`, though `da5dbd1532` landed on 2026-10-08 at 07:39Z. The
  rows do not give their recording times.

**A docs-only arm unit.** DR-125 (M / DESIGN-SPEC) is a spec with no
code. Its four MAJORs across two reviews are all spec text. In a pair,
rule 6(b) would record them and exclude them. The protocol does not say
how such a unit counts toward the arms.

**Foot versus rows.** Two foot lines stand at the end of the rows table:

- the first reads "toward twenty: 20" (ending DR-144);
- the second, left from an earlier recording, reads 18 (ending DR-137).

Main (`effba4bf2f`) also carries two, reading 19 and 18. Against the
rows:

- **M-tier count.** The foot gives 20; the rows give 23. DR-123, DR-129
  and DR-132 are missing from the foot (section 1).
- **Tally.** The foot gives 36; the rows give 33 for the concurrent
  tier: readout 3's 27 plus 6.
  - The foot's 36 includes DR-137's three holdout candidates, but not
    DR-104's one, which readout 3 kept apart.
  - The running figures agree. The cells read 28 after DR-107, and 29
    from DR-117 (after DR-115's +1). Then 29 + DR-137's 3 + DR-138's 1 +
    DR-140's 1 + DR-141's 2 = 36, which DR-141 and DR-142 read.
  - The figures are not monotone in row order: DR-139 reads 29 after
    DR-137 and DR-138, and DR-116 reads 28 after DR-115 (it was
    renumbered at merge).
- **Fair pairs that found any MAJOR.** The foot gives 59; the rows give
  57 concurrent.
  - The foot's list includes the holdouts DR-48 and DR-137, but omits
    the holdout DR-104, whose R1 raised M1.
  - It still includes DR-57 (both cells "No MAJOR") and still omits
    DR-66 (1/2/6 and 1/2/5), as readout 3 noted.
  - 59 − DR-48 − DR-137 − DR-57 + DR-66 = 57.
- **Exclusions.** The foot's exclusion notes name DR-51, DR-2, DR-101,
  DR-112 and DR-119, but not DR-22 (readout 2's 6(e) exclusion).

## 7. The options for Ev

The arms continue until Ev rules (rule 9). The options are not ranked,
and none is recommended. Costs are in reviews per M-letter dual unit.
The observed figures are 1.39 over 23 units, a holdout share of 0.217,
and a first-review MAJOR rate of 4/18.

- **Continue the arms unchanged.** The next full readout comes at
  twenty more units or two more M-tier misses.
  - **Cost:** expected 2 × 0.336 + 0.664 × (1 + p) per unit, where p is
    the first-review MAJOR rate. At p = 4/18 that is **1.48**. The range
    is 1.34 (p = 0) to 2.
  - **Measures:** about 7 more holdouts per twenty units, narrowing the
    miss-per-holdout interval from 5 pairs to about 12, plus about 3
    more second reviews.
  - **Risks:** a sequential unit whose first review raises no MAJOR
    ships on one review, as 14 of 18 did. A miss there is visible only
    as a later escape (rule 11), and none is recorded yet.
- **Change the arm split.** For example, byte mod 2 = 0 gives a 0.5
  holdout share; another choice is all-holdout for a fixed window.
  - **Cost:** about 1.6 per unit at 0.5 with the observed p, or 2 per
    unit all-holdout.
  - **Measures:** holdouts accrue faster, so the miss-per-holdout rate
    narrows sooner.
  - **Risks:** fewer sequential units, so second-review data accrues
    more slowly. Four have run so far.
- **Make the sequential second review unconditional.** Every sequential
  unit gets a second review on the fix-pass head, MAJOR or not.
  - **Cost:** 2 reviews per sequential unit, plus a second fix pass
    where the second review finds anything.
  - **Measures:** what a second review on a fixed head adds when the
    first raised no MAJOR. Today only holdouts speak to that.
  - **Risks:** the conditional trigger itself is no longer measured. The
    cost per unit equals concurrent pairs, but in sequence, so wall
    clock is longer.
- **Return M units to concurrent pairs.** The arms end, and M pairs join
  the tally and the pair count.
  - **Cost:** 2 per unit.
  - **Measures:** the unilateral-MAJOR rate at M directly, comparable
    with the 5 holdouts and with the 10 pre-arm M pairs (DR-1..DR-30).
  - **Risks:** gives up the sequential arm, whose four second reviews
    are the only data on a reviewer who sees the first review's
    adjudicated findings and a fixed head.
- **End the sequential arm.** M units take a single review with no
  conditional second, and leave this log (rule 1).
  - **Cost:** 1 per unit.
  - **Measures:** nothing in this log. Only later escapes would speak,
    with no arm to count them against.
  - **Risks:** in this period that would have shipped DR-131's two
    executed code MAJORs and DR-125's two spec MAJORs, which only the
    second review raised. A DR-104-shaped miss would go unmeasured.
- **Stop the experiment.**
  - **Cost:** none further.
  - **Measures:** none further. The record stands at 23 arm units (5
    holdouts, 18 sequential, 4 second reviews, 1 M-tier miss), 115 fair
    concurrent pairs and 33 concurrent tallied findings by the rows (36
    by the foot).
  - **Risks:** none for the stream. Review tiering would then rest on
    the record as it stands.

Bookkeeping options the rows suggest. Each can go alongside any of the
above.

- **Make the foot follow from the rows.** For example, a lint or
  `scripts/` check that counts M-letter arm rows, MAJOR pairs and tally
  cells.
  - **Cost:** a small script, plus fixing the current foot (two lines,
    three missing M units, the holdout inclusions).
  - **Risks:** none to the method. Without it, a trigger can pass
    unseen, as the twentieth unit (DR-136) did.
- **Say how holdout tallies and holdout MAJOR-pairs enter the foot.**
  DR-104 is kept apart; DR-48 and DR-137 are folded in.
  - **Cost:** a sentence in rule 9 or the Seams, and a one-time
    re-count.
  - **Risks:** none to the method. It changes the reported totals
    (36 → 33 for the tally).
- **Require blinded coding by a separate coder before a finding is
  counted.** In this period four concurrent tallied findings rest on
  weaker coding: DR-141 (+2, unblinded), DR-115 and DR-138 (unnamed
  coder). So do DR-137's three holdout candidates, coded by the
  implementer.
  - **Cost:** one coder lane per pair that raised a MAJOR.
  - **Risks:** more rows held open at merge for the coding.
- **Name the "first" review in each holdout pair** by a second recorded
  byte. This was raised in readout 3 and not adopted.
  - **Cost:** one byte per holdout.
  - **Measures:** rule 9's counterfactual for one review rather than for
    "the other review" (DR-48, DR-137).
- **Record the draw time and the cost of every arm review in one
  unit**, as tokens and wall clock.
  - **Cost:** bookkeeping.
  - **Measures:** what the sequential arm saves per unit. The rows
    cannot give that now, because 13 of 23 arm units record no review
    cost.
- **Say how a docs-only M unit counts** (DR-125).
  - **Cost:** a definition.
  - **Risks:** none to the method. It moves one of DR-125's or a future
    unit's MAJORs in or out of the second-review count.
- **Have the orchestrator of the fixing unit append the rule-11 line**
  whenever the fixing unit names a merged unit, as DR-123 did for unit
  B (DR-119).
  - **Cost:** a line per traced defect.
  - **Measures:** escapes, which are the only signal the sequential
    arm's single-review units can give.
