# Dual Opus review — readout 6

Off-file (rule 10 of `docs/DUAL-REVIEW-PROTOCOL.md`). A **full readout**
under rule 9 as it now reads: "a full readout only when Ev requests
one". Ev requested this one on 2026-11-10. The last full readout was
readout 5 (trigger DR-165), after which Ev ruled on PR 4645: "keep the
experiment running but switch the readout to just do the 'let me know
what the miss was' readouts and not do any more full readouts until i
request one".

## Summary (phone)

- **The log:** 457 rows, DR-1..458 (2026-09-24 to 2026-11-09). Since
  readout 5: 293 rows. 165 are H-letter concurrent pairs, one is an M
  pair run without a draw, and 126 are M/S units under the arms (39
  holdout pairs, 87 sequential). There are no L-letter units. Every reviewer was Opus. No model seam falls
  inside the data.
- **What a second review buys, at H (the clearest answer):** in 21.7%
  of fair H pairs (56/258, 95% CI 17–27%), exactly one review found a
  demonstrated, blocking defect (a "tallied finding"). That is 0.28
  findings per pair, so a lone review would have shipped about 0.14 per
  unit. The rate has been flat across five blocks of the log
  (18%, 23%, 19%, 20%, 24% of pairs). One more Opus/Opus H pair now
  adds almost nothing to the estimate.
- **At M, the arms:** since readout 5 the holdout miss rate fell, to
  4 misses in 39 holdout pairs (10%, CI 3–24%), against 3 in 9 before.
  Cumulatively it is 7 in 48 (15%, CI 6–28%). Since readout 5, a holdout
  review that raised no MAJOR had a twin raising a tallied MAJOR in 4 of
  55 cases (7%, CI 2–18%).
- **The sequential arm** costs 1.28 reviews per unit against 2 for a
  pair. Its conditional second review fired in 24 of 87 units, and 7 of
  those 24 raised a new MAJOR (29%). On the holdout evidence, the
  sequential arm ships about 0.05–0.07 demonstrated blocking defects per
  M unit that a pair would have caught. That is roughly one per 10–14
  reviews saved, or about 3M tokens.
- **Escapes are badly under-recorded.** The log has 5 later-escape
  lines. Work items attribute a red on main or a regression to 28
  logged units since readout 5:
  - 10 of 87 sequential units;
  - 16 of 166 concurrent pairs;
  - 2 of 39 holdouts.

  These rates are statistically indistinguishable. Most of these
  escapes sit in rows the per-PR gate does not run (the slow set, the
  1e-6 and 1e-12 rows, feature-gated rows), so both reviewers share
  the blind spot. A second review does not buy their removal. CI
  selection would.
- **Data problems.**
  - One holdout is mislabelled concurrent (DR-410).
  - One holdout pair is logged as two rows (DR-207 and DR-208).
  - 127 rows have the wrong number of table cells.
  - The foot's tally (91) mixes in holdouts; the concurrent-only tally is
    82.
  - Readout 5's ruling is missing from `rulings.md`.
  - Tokens are recorded for only 23 of 526 reviews since readout 5.
- **For the Opus+Fable question:** the Opus/Opus H baseline is now
  precise enough to serve as the reference a cross-model stream would
  be measured against. Ending this stream's H pairs costs little
  information. The M-tier estimate is the weaker one. If dual pairs go
  cross-model, one way to keep the M question answerable is to keep the
  holdout and sequential arms Opus/Opus, or to stop them in one
  recorded step.

## 0. Sources and method

- **Rows:** `docs/dual-review-log/rows/PR-*.md` on `origin/main` at
  `677f7d2eb8` (2026-11-10). `scripts/dual-review-log.py lint` passes
  there. The foot is quoted from `docs/DUAL-REVIEW-LOG.md` at the same
  commit.
- **Protocol:** `docs/DUAL-REVIEW-PROTOCOL.md` at `24b76fff0e`. Its text
  is byte-identical on main today; later commits that touch the path
  are merges with no net change. A pending branch adds a model-version
  rule and a "Fable 5.2 era" seam (section 6). It is not on main and no
  row ran under it.
- **Previous readouts:** read on `analysis/dual-review/readout-4` and
  `-5`. **Ev's rulings:** read in `docs/dual-review-log/rulings.md`,
  plus PR 4645's comments for readout 5's ruling, which `rulings.md`
  does not carry.
- **Reproducibility.** Every number below is printed by
  `analysis/dual-review/readout-6-numbers.py`, which uses
  `readout-6-rows.py` and is stdlib only. Run it from a main checkout:
  `python3 analysis/dual-review/readout-6-numbers.py .`. Counts come from
  the rows' front matter, except where the script names a correction
  and the row cell it is read from.
  - Per-review MAJOR counts come from the `MAJ/MIN/NOTE` triple in the
    R1 and R2 cells, with five rows read by hand.
  - Second-review outcomes and escape attributions were read by hand.
    The script lists them as constants with a one-line reason each.
- **Intervals** are exact (Clopper–Pearson) 95%.
- **Windows:**
  - pre-arms: DR-1..47;
  - arms through readout 5: DR-48..165;
  - since readout 5: DR-166..458. DR-166..172 were recorded on
    2026-10-10/11, in parallel with DR-165; numbering follows main's
    merge order.

**Corrections applied, each from a row cell:**

1. **DR-410** (RESTCURVE #6252) is a HOLDOUT. Its class cell reads "arm:
   HOLDOUT (byte 198, mod 3 = 0), a concurrent pair", while its front
   matter says `arm: concurrent`. It is counted as an M-letter holdout
   with no MAJOR on either side. The foot therefore counts it among the
   fair concurrent pairs and omits it from the M-tier units.
2. **DR-207** (CONTACTHOLD #4809) and **DR-208** (#4820) are one pair,
   "reviewed with PR 4820 as one stacked unit at `af3aa3d`". The
   findings, coding and tally are on DR-208's row. The two rows are
   counted as one holdout pair. The foot counts two M-tier units.
3. **DR-104** is counted as a fair holdout pair with one tallied finding,
   as readouts 3–5 counted it. Its `excluded` field is bookkeeping ("read
   apart from the concurrent tally"), not a rule 4 or 6(e) exclusion. Ev
   was asked about it on PR 4645, and the question was dropped
   unanswered.

With these corrections the M-tier unit count since readout 5 is 126 (39 +
87). The foot also says 126, by two offsetting errors.

**Glossary.**
- *Tallied finding*: rule 6. Unilateral (one review raised it, the
  other never mentioned it), raised as MAJOR, code / test-gap /
  contract-API class, deduplicated, demonstrated by execution, in a fair
  pair.
- *Fair pair*: rule 4 and 6(e). Both reviewers ran under the same method
  and neither was interrupted.
- *M-tier miss*: rule 9. Either a tallied finding in a holdout pair
  whose other review raised no MAJOR, or a later escape from a
  sequential unit that no review raised as a MAJOR.
- *Holdout / sequential*: rule 1's two M-tier arms. A byte mod 3 = 0
  gives a concurrent holdout pair. Otherwise the unit gets one review,
  and a second only after the fix pass if the first raised a MAJOR.

## 1. The population

| window | concurrent H | concurrent L | concurrent M | holdout | sequential |
|---|---|---|---|---|---|
| DR-1..47 (pre-arms) | 25 | 12 | 10 | — | — |
| DR-48..165 | 79 | 2 | 1 (DR-101, no draw) | 9 M | 26 M |
| DR-166..458 | 165 | 0 | 1 (DR-420, no draw) | 38 M + 2 S | 86 M + 1 S |
| whole log | 269 | 14 | 12 | 47 M + 2 S | 112 M + 1 S |

(After correction 1. Holdouts count rows; correction 2 folds DR-207 into
DR-208, so the holdout pair counts below are 48 overall and 39 since
readout 5.)

- **Letters.**
  - **L is extinct.** The last L unit is in DR-48..165.
  - **S is nearly so:** three units, DR-243 (sequential) and DR-249 and
    DR-253 (holdouts, neither with a MAJOR).
  - The per-letter tables therefore have two populated rows, H for
    concurrent and M for the arms. Every S and L figure below is too
    small to read.
- **Draws.**
  - Every arm row's recorded byte agrees with its arm.
  - Since readout 5, 38 of 120 parseable draws gave mod 3 = 0. The
    observed rate is 0.317; the draw's rate is 86/256 = 0.336.
  - Two M units ran as pairs without a draw, DR-101 and DR-420. Both are
    excluded.
- **Programs in the arms since readout 5:**
  - PCERT 22, CONTACTHOLD 17, FLUX 17, INTENT 14, GERM 13;
  - 14 others with 7 or fewer each.

  That is a much broader spread than readout 5's INTENT-heavy window.

## 2. The concurrent tier: what a second review buys at H

Rule 6's tally over concurrent pairs only, as the Seams require. Holdouts
are kept apart.

| measure | DR-1..47 | DR-48..165 | since readout 5 | whole log |
|---|---|---|---|---|
| fair H pairs | 24 | 76 | 158 | 258 |
| pairs with any MAJOR | 14 (0.58) | 40 (0.53) | 83 (0.53) | 137 = 0.531 (0.468–0.593) |
| pairs with a tallied finding | 5 (0.21) | 17 (0.22) | 34 (0.22) | 56 = 0.217 (0.168–0.272) |
| tallied, given a MAJOR | 5/14 | 17/40 | 34/83 | 56/137 = 0.409 (0.326–0.496) |
| tallied findings | 5 | 21 | 47 | 73 (0.283 per pair) |

- **All concurrent letters (H + L + M):** 281 fair pairs, 147 with a
  MAJOR, 60 with a tallied finding, 82 tallied findings.
- **L alone:** 13 fair pairs, 3 with a tallied finding, 8 findings. The
  numbers are consistent with H, and too few to say more.

**Trend.** The table groups the fair concurrent pairs into blocks of
the log.

| block | dates | fair | any MAJOR | tallied pairs | findings |
|---|---|---|---|---|---|
| DR-1..47 | 09-24..10-02 | 45 | 0.49 | 8 = 0.18 (0.08–0.32) | 12 |
| DR-48..165 | 10-02..10-11 | 78 | 0.54 | 18 = 0.23 (0.14–0.34) | 23 |
| DR-166..260 | 10-10..10-14 | 31 | 0.35 | 6 = 0.19 (0.08–0.38) | 7 |
| DR-261..360 | 10-14..10-27 | 51 | 0.57 | 10 = 0.20 (0.10–0.33) | 15 |
| DR-361..458 | 10-27..11-09 | 76 | 0.57 | 18 = 0.24 (0.15–0.35) | 25 |

- **No trend.** Every block's interval contains every other block's
  point estimate.
- **The second review's yield is stationary,** at about one pair in
  five yielding a demonstrated blocking defect that one review would
  have let through.
- **The dip in DR-166..260's MAJOR rate (0.35) is the one feature.** It
  coincides with the window's large INTENT and PCERT audit series. The
  tally rate did not move with it.

**What the tallied findings are, since readout 5.** Reading the 34 pairs'
tally cells:

- **Most are code-class findings** with a measured wrong value or a red
  probe. Examples: a certified overlap with a wrong volume (DR-184); the
  bisector serving `inf` (DR-308); an `f64` false pass of a 1.038e-12 m
  miss at ε = 1e-12 (DR-363); the complement inversion and the fold-over
  (DR-379, 3 findings).
- **A minority are test-class:**
  - DR-170's red row at head and base;
  - DR-254's `recognize_pins` red in a slow set the other reviewer did
    not run;
  - one of DR-317's three (ε-row reds);
  - DR-340's surviving mutant.
- **The recurring shape** is the one readout 5 named for the misses:
  the silent review did not exercise the region. The rows say so in
  words:
  - DR-254: "R2 ran no step-import slow set, which is where it hid";
  - DR-406: "R1 probed Interval on spheres and the torus at f64";
  - DR-411: "R2 read five of the six fixture edits".

**Coding behind the tally.** Since readout 5, 30 of 38 tallied
pairs (34 concurrent and 4 holdout) rest on a coding the row describes as
attribution-stripped by a separate coder. The rest are weaker:

- an unblinded pre-note: DR-166;
- orchestrator-coded or unstripped: DR-171, DR-208, DR-254, DR-304 and
  DR-308;
- blind, but working from orchestrator condensations: DR-411;
- a byte recorded with no coder named: DR-353.

These carry 9 of the 51 findings.

**Severity divergence.** Keyword counts over the correspondence and
tally cells give a lower bound:

- Since readout 5, 38 of 98 fair pairs with a MAJOR record a MAJOR met by
  a MINOR or NOTE (0.39, CI 0.29–0.49). The figure was 11 of 47 (0.23)
  in DR-48..165 and 6 of 22 (0.27) before the arms.
- MAJOR/MAJOR bilaterals hold steady at about 0.38.
- The rise in divergence may be recording rather than reviewing: rows
  since readout 5 spell out the severities more often.
- Either way, a large share of "one review's MAJOR" is the other
  review's MINOR on the same defect. That share fails 6(a) and is not
  tallied. It is the other half of what a second review buys: the
  severity, not the sighting.

**Unilateral MINOR variance.** The rows do not give unilateral MINOR
counts in a parseable form, so this readout uses a coarser proxy, the
gap in each pair's MINOR counts.

- **Per review:** a mean of 3.2–3.5 MINORs in every window.
- **Within a pair:** the counts differ by a mean of 1.1–1.2, and are
  equal in about 30% of pairs. This is unchanged since the pre-arms
  window.
- **Bilateral group counts** (25 + 40 pairs that state one): a median of
  8 groups per pair.

The plentiful MINOR variance is stable. Nothing in it says the reviewers
have converged or drifted apart.

**Excluded concurrent pairs: 14.**

- **Since readout 5: 8.**
  - Six by 6(e), interrupted reviews: DR-348, DR-360, DR-373, DR-413,
    DR-415 and DR-431. The causes were container restarts, the weekly
    usage limit and a worker restart.
  - One by a disk filling mid-review: DR-442.
  - One M unit run without a draw: DR-420.
- **Earlier: 6.**
  - DR-2: method divergence.
  - DR-22 and DR-51: truncated.
  - DR-101: no draw.
  - DR-112 and DR-119: mixed Fable + Opus pairs.
- Infrastructure interruptions are now the dominant reason for
  exclusion: 7 of the 8 since readout 5. Five of the 8 found a MAJOR
  (DR-348, 413, 420, 431 and 442). Their tallies are missing data, not
  zeros.

## 3. The M tier: holdouts against the sequential arm

### 3.1 Holdout pairs

| measure | DR-48..165 | since readout 5 | all arms |
|---|---|---|---|
| holdout pairs | 9 | 39 (37 M, 2 S) | 48 |
| no MAJOR on either side | 4 | 24 | 28 |
| MAJOR from both (bilateral MAJOR/MAJOR) | 1 (DR-137, 3 tallied anyway) | 8 | 9 |
| one side MAJOR, other MINOR (severity split, not tallied) | 1 (DR-48) | 3 (DR-182, DR-318, DR-430) | 4 |
| one side MAJOR, other silent, tallied = **M-tier miss** | 3 (DR-104, 147, 165) | **4 (DR-185, 208, 321, 353)** | 7 |
| pairs with any MAJOR | 5/9 = 0.56 | 15/39 = 0.385 (0.234–0.554) | 20/48 = 0.417 (0.276–0.568) |
| pairs with a tallied finding | 4/9 | 4/39 = 0.103 (0.029–0.242) | 8/48 = 0.167 (0.075–0.302) |
| M-tier misses per holdout pair | 3/9 = 0.333 (0.075–0.701) | 4/39 = 0.103 (0.029–0.242) | 7/48 = 0.146 (0.061–0.278) |
| holdout reviews raising a MAJOR | 6/18 | 23/78 = 0.295 | 29/96 = 0.302 |
| no-MAJOR reviews whose twin raised a **tallied** MAJOR | 3/12 = 0.250 (0.055–0.572) | **4/55 = 0.073 (0.020–0.176)** | 7/67 = 0.104 (0.043–0.203) |
| no-MAJOR reviews whose twin raised **any** MAJOR | 4/12 | 7/55 = 0.127 | 11/67 = 0.164 |

**The four misses since readout 5.** Each got its short readout.

| row | unit | class | the tallied MAJOR (raiser) | the silent review |
|---|---|---|---|---|
| DR-185 | FLUX #4696 | M / SOUNDNESS | the cone flux lane dropped the on-surface premises, so a chord-rimmed rectangle MEASURES 0.6051 against 0.7191 where the base refused (R2) | R1: AWF 0/2/2, a 1,120-case fuzz against an independent closed form that never built an off-surface rim |
| DR-208 | CONTACTHOLD #4809+#4820 | M / STRUCTURAL | a subtract digging a pit under a carried rest builds on #4809's head and refuses `JoinDesync` on #4820's (R1) | R2: AWF 0/3/4, mutants over the topo suite; no pit pose |
| DR-321 | FLUX #5576 | M / SOUNDNESS | the backstop's Bound arm reads an f64-rounded margin as exact, so a violated `vol(A ∩ B) ≤ vol(A)` passes (R1) | R2: AWF 0/3/7; its related gate finding was coded bilateral with R1's other MAJOR |
| DR-353 | SPLICE #5805 | M / SOUNDNESS | on a closed chart, parity ignores the gauge between loops, so a hole lifted a period reads In (R1) | R2: AWF 0/3/7; "closed charts NOT exercised" (its own NOTE) |

All four are code-class wrong answers shown by execution: a wrong
measure, a wrong refusal, a wrong pass and a wrong classification. Not
one is a test-only defect like DR-147.

- **DR-208's coding is the weakest.** It was "Coded by the orchestrator",
  not by a separate blind coder.
- **DR-208's fair cell records two disclosures:**
  - both reviewers saw the other's build lines in a shared scratch log;
  - R1 force-pushed its own review branch.

  The row still counts it fair, under rule 4's test.
- **In three of the four, the row itself says the silent review never
  built the input that broke.** The exception is DR-321, whose silent
  review was near the defect and coded a different one. This is the same
  shape as DR-147 and DR-165.

**The severity-split holdouts** are the DR-48 shape:

- DR-182: R1's MAJOR, R2's MINOR, both executed;
- DR-318: R2's MAJOR, R1's MINOR "argued unreachable";
- DR-430: B's MAJOR, A's first MINOR, with "a fix asked before merge".

None is tallied. Under the sequential arm with the MINOR-side review
first, no second review would run. Whether a fix pass would take the
MINOR depends on the orchestrator. The rows show that it usually does:
the fix passes take the adjudicated union.

**Expected loss per M unit.** Suppose the sequential arm's one review is
either review of the holdout pair with probability ½. Then the expected
number of tallied defects it ships with no second review to catch them
is misses / (2 × pairs):

- **0.051 per unit since readout 5;**
- **0.073 per unit cumulatively.**

This ignores whatever the fix pass and later work catch anyway (section
4). Rule 9's count names "the other review" and does not randomise. The
½ factor is this readout's.

**What changed since readout 5.** Readout 5 read 3 misses in 9 holdouts
and warned that "misses at this rate would trigger readouts often". The
next 39 holdouts gave 4.

- Readout 5's rate (0.333) lies above this window's interval
  (0.029–0.242). The two windows are unlikely to share one rate, but the
  early window is 9 pairs.
- The likelier reading is that the early rate was a small-sample high.
  The pooled 0.146 (0.061–0.278) is the best estimate, and it is still
  wide.
- The holdout MAJOR rate also fell, from 5/9 to 15/39.

### 3.2 Sequential units

| measure | DR-48..165 | since readout 5 | all arms |
|---|---|---|---|
| units | 26 | 87 (86 M, 1 S) | 113 |
| first review raised a MAJOR | 7 = 0.269 | 24 = 0.276 (0.185–0.382) | 31 = 0.274 (0.195–0.366) |
| second review owed and run | 7/7 | 24/24 | 31/31 |
| second review raised a MAJOR the first did not | 2/7 (DR-125, DR-131) | **7/24 = 0.292 (0.126–0.511)** | 9/31 = 0.290 (0.142–0.480) |
| reviews per unit | 1.27 | 1.28 | 1.27 |

- **The arm ran as written.** Every owed second review ran, and none ran
  unowed.
- **Three second reviews deviated from rule 1:**
  - DR-246 ran on the SAME head as review 1, with no fix pass between,
    and its brief was relaxed by one phrase;
  - DR-263 ran on the same head while the first MAJOR went to Ev as a
    fork;
  - DR-316's was briefed blind to review 1's report, where rule 1 briefs
    it with the adjudicated findings.

  None of the three raised a new MAJOR.

**What the seven new second-review MAJORs were** (read from the rows):

- **A regression in the original diff that the first review missed:**
  - DR-226: the rim pre-gate's `same_circle`, "a regression from the
    base";
  - DR-278: an in-flight gesture re-checked by its root only, "a
    regression".
- **The first MAJOR's fix did not close its class:**
  - DR-295: moved-then-joined cells read Kept;
  - DR-355: the bundle doors still zero-fill, "closed only at the site it
    cited".
- **Rows missing for code the first fix pass added:** DR-273, five
  surviving mutants.
- **Origin not stated in the row:**
  - DR-297: two MAJORs, the rebuilt line and the arc one-point overlap;
  - DR-455: a refusing take lost its operand's name, among items "review
    1 did not" raise.

So a conditional second review on the fix head finds a new MAJOR about
three times in ten. Some of those are pre-existing in the diff, which
is the holdout's question, and some are created or left by the fix pass,
which a concurrent pair cannot see at all.

**The 63 single-review units since readout 5.** Only holdouts speak to
these, and only escapes test them (section 4).

### 3.3 The sequential arm's cost, weighed

- **Reviews per M unit since readout 5.** Holdout pairs 2; sequential
  1.276. With a holdout share of 0.310 the arms average 1.50. Always
  pairing would cost 2 reviews; never pairing, 1.28 (the sequential
  arm alone).
- **Saving and cost.** Running sequential instead of a pair saves 0.72
  reviews per M unit. Section 3.1 puts the cost at 0.05–0.07 tallied
  defects shipped per unit, before later catches. That is one
  demonstrated blocking defect shipped per 10–14 reviews saved.
- **At H, by the same arithmetic,** a lone review would ship
  0.283 / 2 = 0.14 tallied defects per unit, one per 7 reviews saved.
- **Tokens.** At the observed median of about 250k tokens per review
  (section 5), the M trade is about 2.5–3.5M tokens per shipped M-tier
  defect, and the H trade about 1.8M.
- **So the second review is worth about twice as much per token at H as
  at M.** That fits Ev's tiering. The intervals are wide: the M figure's
  numerator is 4 to 7 events.

## 4. Later escapes (rule 11)

**Recorded lines: 5.**

- DR-11 (M, concurrent, pre-arms). Covered by earlier readouts.
- DR-54 (H, concurrent). Covered by earlier readouts.
- **DR-200** (PCERT #4791, M, HOLDOUT). Seven probe-feature
  rows went red. They run only in the nightly's `--all-features` sweep.
  "Neither review raised it."
- **DR-382** (H, concurrent). A stale slow-set fixture. Neither review
  raised it, and the lane did not run it.
- **DR-438** (H, concurrent). The editor-core resolver red was caught by
  CI at the fix pass's head, not by either review. That is before merge,
  so arguably not an escape.

None names a sequential unit, so the recorded escape-kind M-tier misses
remain zero.

**Traced from the repository but not recorded.** A pattern scan of
`work/**` finds items whose title, or whose bisection, attributes a red
on main or a regression to a logged unit's PR. Each was read and
classed:

- **pin:** a pinned value, golden, digest or census registration moved,
  with no wrong answer;
- **build:** a compile, doc or feature build broke;
- **behaviour:** a refusal, escalation, wrong value or slowdown where
  main had answered.

The list is in the script (`ESCAPE_CANDIDATES`), with the item each
comes from. It is not adjudicated under 6(b) and (d), and the scan is
surely incomplete: it finds only items that name the PR.

| arm, since readout 5 | units with a traced red | kinds |
|---|---|---|
| sequential (M) | 10/87 = 0.115 (0.057–0.201): DR-174, 176, 177, 259, 266, 284, 343, 392, 435, 455 | pin 6, build 2, behaviour 2 |
| holdout (M) | 2/39 = 0.051 (0.006–0.173): DR-200 (recorded), DR-345 | build 1, pin 1 |
| concurrent (H) | 16/166 = 0.096 (0.056–0.152): DR-237, 264, 279, 288, 308, 331, 335, 348, 360, 363, 368, 377, 382, 390, 397, 405 | pin 7, behaviour 9 |

What this shows:

- **The upper bound on what a second review can buy.** Escapes from
  pairs are defects both reviews missed. They run at about 0.10 per H
  unit, against 0.28 tallied findings per H pair. So for every three
  demonstrated blocking defects a second review adds, about one more
  gets past both. About half of those are behaviour regressions:
  - DR-264, 308, 331, 363 and 390: refusals at 1e-6, 1e-12 or Interval;
  - DR-279: a fuzz reading OnBoundary where In;
  - DR-348: a far union 3.2e-5 off its closed form;
  - DR-368: a section exhausting its cell budget;
  - DR-377: a 15× slowdown that cancelled the nightly's 1e-12 job.
- **The sequential arm does not escape visibly more.** 0.115 against
  0.096 for the H pairs. The intervals overlap almost entirely, and the
  letters differ, so this is not a clean comparison. Nothing here says
  single-reviewed M units leak more than paired H units.
- **What they share is the CI's blind spot, not the reviewer's.**
  - Almost every traced escape is a row the per-PR gate does not select:
    the slow set, the 1e-6 and 1e-12 rows, `--features probe`, the
    rustdoc and demos nightly jobs, the step-import or test-utils
    suites.
  - The items say so: "Its own CI never selected test-utils" (#4677);
    "no per-PR row runs them" (#4791).
  - Two independent reviewers who share the habit of not running the
    slow set at every ε miss these together. A second review buys
    little against this class; CI selection would.
- **Candidate escape-kind M-tier misses (rule 9's second branch).** The
  ten sequential units, if their escapes are confirmed as defects under
  6(b) and (d) and no review raised them. Spot checks of DR-176, DR-392
  and DR-455 find no mention in any review cell.
  - **Behaviour regressions:** DR-392 (#6053), where a prism tilted
    within the zero now escalates where it built (bisected); and DR-343
    (#5529), CI-profile rows refusing flush copies, which the item
    suspects of being a crossed base.
  - **The rest are pins, censuses and builds:** DR-174's compile break is
    a semantic conflict with a concurrent commit.
  - **Readout 5 asked whether a moved pin counts** (its DR-144 case) as a
    test-class defect under 6(b), demonstrated by the red row under
    6(d). Ev was never asked to rule; PR 4645 dropped the question. If it
    counts, the escape kind supplies up to about 9 M-tier misses since
    readout 5, more than the holdout kind's 4. If it does not, about 1–2.

## 5. Cost

| window, arm | reviews | tokens recorded | median tokens (IQR) | minutes recorded | median minutes (IQR) |
|---|---|---|---|---|---|
| DR-48..165, all | 217 | 63 | 250k (207k–290k) | 70 | 42 (26–65) |
| since R5, concurrent | 332 | 15 | 279k (240k–295k) | 83 | 105 (45–170) |
| since R5, holdout | 80 | 4 | 200k (182k–235k) | 14 | 112 (86–150) |
| since R5, sequential | 114 | 4 | 173k (150k–235k) | 35 | 135 (85–170) |
| whole log, all | 837 | 166 | 250k (212k–292k) | 296 | 61 (31–115) |

- **Tokens.** One Opus review costs about 200–300k tokens. This is
  unchanged from readouts 4 and 5. Since readout 5 tokens are recorded
  for only 23 of 526 reviews, so per-arm token figures are anecdotes.
- **Wall-clock.** The median recorded review time rose from about 42 to
  about 115 minutes. The cells mix harness time, wall time across usage
  pauses and container restarts ("~7 h, spanning a container restart",
  DR-403), and estimates. The rise is real in the cells, but the cause
  may be heavier batteries (three ε, slow sets), queueing for the build
  mutex, or recording style.
- **A pair's second review costs about 1–2 hours of wall clock.** It
  costs nothing in latency when it runs concurrently. The sequential
  arm's second review adds a full review's latency after the fix pass.
- **Fix passes and verifiers are outside these counts.** Independent
  verifiers and delta reads are common since readout 5, for example
  DR-185, DR-273 and DR-297.

## 6. Data quality, seams and confounds

**Data problems (to file):**

1. **Rule 11 is under-recorded.** There are 5 later-escape lines, but
   work items attribute a red or regression to 28 logged units since
   readout 5 (section 4). Fixing PRs name the unit, often by bisection,
   and nobody appends the line. This is the largest gap in the
   experiment. Rule 9's escape-kind miss and rule 11's upper bound are
   both blind without it.
2. **DR-410's `arm: concurrent` contradicts its class cell (HOLDOUT,
   byte 198).** The foot counts it as a concurrent fair pair and omits
   it from the M-tier units.
3. **DR-207 and DR-208 are one pair recorded as two holdout rows.** The
   foot counts two units.
4. **Table shape.**
   - 127 rows do not have the header's 11 cells.
   - 74 have fewer, 71 of them since readout 5, mostly with 10 cells
     from about DR-199 on. Their tally and fair contents shift one column
     left in the render, so the "tally" column shows fair text and the
     "fair" column shows the fix pass.
   - 53 have more than 11 because of unescaped `|` inside cells
     (`|axis·A⃗|`, `|h|`).
   - DR-150's row is still truncated, ending mid-cell (readout 5).
   - `lint` checks none of this.
5. **The foot's tally mixes in holdouts.**
   - The foot reads "Tally: 91". That is 82 concurrent plus 9 holdout
     findings, with DR-104's tenth held out by its `excluded` field.
   - The Seams say "the tally and the pair count read concurrent pairs
     only". Its 166 fair-MAJOR pairs likewise include 19 holdouts (DR-104
     is excluded).
   - The script computes `pair` as concurrent or holdout.
6. **`major` contradicts the cells in three rows.**
   - DR-66 says false, though both reviews raised a MAJOR, coded
     bilateral by severity split.
   - DR-57 says true, though neither review raised a MAJOR.
   - DR-376 says true for a MAJOR from a delta read outside the pair.
   - Readouts 3–5 flagged DR-57 and DR-66, and they are still
     uncorrected.
7. **Readout 5's ruling is not in `rulings.md`.** Ev ruled on PR 4645,
   2026-10-11, as quoted at the top. Two questions readout 5 and PR 4645
   raised were never answered: the DR-144 moved-pin escape, and whether
   DR-104 counts.
8. **Protocol hashes since readout 5.**
   - 164 of 293 rows name `24b76fff0e`, the last commit that touched the
     protocol.
   - 124 name a later main commit whose protocol text is identical. That
     is harmless, but not rule 8's wording.
   - DR-166, 167, 169 (`da5dbd1532`) and DR-174 (`4b5880058a`) name the
     pre-PR-4645 text. It differs only in rule 9's readout trigger.
   - DR-455's `1bbfcda6` does not resolve.
9. **Cost fields are sparse.** Tokens appear on 23 of 526 reviews since
   readout 5, against 63 of 217 before. Minutes appear on 132, in mixed
   units.
10. **Coding is weaker than rule 7 on 8 of 38 tallied pairs since
    readout 5** (section 2), including the M-tier miss DR-208.
11. **Sequential deviations:** DR-246 and DR-263 ran the second review on
    the first's head, and DR-316's second review was briefed blind.
12. **Draw timing** is recorded on 59 of 126 arm rows since readout 5.
13. **Stale hand-kept "Running:" counters** in tally cells, for example
    DR-366's "Running: 2" and DR-448's "Running: 1". They are left over
    from the hand-kept foot.
14. **The reviewer's model version is not recorded beyond "Opus".** The
    Seams' Opus 5.5 era rests on dates, not on the rows.

**Seams.**

- **Arms:** concurrent and arm rows are reported apart throughout.
- **Model:** all 457 rows are Opus-reviewed and post-date the Opus 5.5
  release. The Fable 5.2 era note and the implementer-model draw
  (`docs/MODEL-COMPARE-PROTOCOL.md`) are on a pending branch dated
  2026-11-10, after the last row (DR-458, 2026-11-09). There is one era.

**Confounds.**

- **Letter and arm are confounded.** H is always concurrent and M is
  always in the arms. So H against M comparisons (section 3.3) mix the
  review regime with unit difficulty.
- **Program and orchestrator vary with time and arm.** The holdout miss
  rate's fall coincides with a broader program mix: since readout 5,
  FLUX and CONTACTHOLD supply three of the four misses.
- **Fix passes take the adjudicated union.** So "what one review would
  have shipped" is a counterfactual this log estimates from coding, not
  from shipped code. Escapes are the only direct evidence, and they are
  under-recorded.

## 7. Options for Ev

The arms continue until you rule (rule 9). The options are unranked.
The figures behind them are:

- **H:** the second review yields a tallied finding in 0.22 of pairs,
  0.28 findings per pair, at about 250k tokens and 1–2 h.
- **M:** misses run at 0.10 (since readout 5) to 0.15 per holdout pair,
  0.05–0.07 shipped per unit. Sequential second reviews add a new MAJOR
  in 0.29 of those that run, and the arm costs 1.28 reviews per unit.

The options:

- **Continue unchanged.**
  - **Cost:** 2 reviews per H unit, about 1.5 per M unit.
  - **Measures:** the M miss rate narrows slowly. Halving its interval
    takes about four times the sample, roughly 150 more holdouts. At
    about 40 holdouts a month, that is four months.
  - **Risk:** the H estimate is already stationary (five blocks, one
    rate), so H pairs now buy defects, not information. That may be
    reason enough to keep them, as review policy rather than as an
    experiment.
- **End the H experiment, keep H pairs as policy.**
  - Stop coding and tallying H pairs: drop the blinded coder, keep both
    reviews.
  - **Saves:** one coder lane per MAJOR pair, and the row-keeping.
  - **Loses:** nothing the estimate needs, at 258 fair H pairs.
- **Change the M arms.**
  - **Raise the holdout share** (mod 2 for 0.5): holdouts accrue 1.5×
    as fast, at about 0.12 more reviews per M unit (1.64 against
    1.52).
  - **Make the second review unconditional:** it measures what a
    fix-head review finds when the first raised no MAJOR. That is the
    sequential arm's actual blind spot, 63 units since readout 5. It
    costs 2 reviews per unit, with no concurrency.
- **End the M arms.** All M units get one review, or all get pairs.
  - **Single review saves** 0.28 reviews per unit against today's
    sequential arm, at the cost of the conditional second review's
    findings: 7 new MAJORs in 24 runs since readout 5. That is the
    better-measured arm's best product.
  - **All pairs** cost 0.5 more per unit than today's mix (0.72 more than
    a sequential unit) and buy back the ~0.05–0.07 misses per unit.
- **End the stream.** The record stands:
  - 281 fair concurrent pairs, 60 with tallied findings, 82 findings;
  - 48 holdouts with 7 M-tier misses;
  - 113 sequential units with 31 second reviews and 9 new MAJORs.
- **Whatever else is chosen** (each is cheap):
  - **Record escapes:** have the orchestrator who files or fixes a main
    red that bisects to a logged unit append the rule-11 line. Then rule
    on whether a moved pin or census registration is an escape.
  - **Fix the log's bookkeeping:**
    - DR-410's arm; DR-207 and DR-208 as one pair;
    - the 10-cell rows, the escaped pipes and DR-150;
    - `major` on DR-57, DR-66 and DR-376;
    - concurrent-only foot tallies, with holdouts reported beside;
    - readout 5's ruling into `rulings.md`.

    A lint for the cell count would have caught most of these.
  - **Spend on CI selection rather than review.** The escapes both
    reviewers miss are mostly unselected rows. Running the slow set and
    the 1e-6 and 1e-12 rows for the crates a PR touches would remove a
    class of escape that no number of reviewers removes.

**On the Opus-vs-Fable question (cross-model dual pairs).** Making
dual-tier pairs one Opus and one Fable ends this Opus/Opus stream. What
this readout implies:

- **The same-model baseline is ready to be a reference.** At H, the
  Opus/Opus second review's yield is measured to about ±5 points
  (0.217, 0.168–0.272). It has been stationary for six weeks. A
  cross-model stream can be read against it directly. If an
  Opus+Fable pair's tallied-pair rate is clearly above 0.22, the
  difference is what model diversity adds. This stream supplies the
  "same model" control a cross-model stream needs. Ending it now loses
  little.
- **Under today's protocol a cross-model pair is excluded.** DR-112 and
  DR-119 were excluded under rule 2 as mixed-model pairs, and the Seams
  call the A/B log's cross-model duals "not comparable". A cross-model
  stream needs:
  - its own protocol, or a dated seam here;
  - a recorded rule for which reviewer is which model, by draw, so that
    model is not confounded with dispatch order;
  - the model version per reviewer, as the pending branch's rule 8
    addition asks.
- **The M question is the one a switch would muddle.** The holdout
  counterfactual ("what would one review have shipped") changes meaning
  if the one review could be either model.
  - **Option (a):** keep the M arms Opus/Opus until their question is
    answered or stopped. Halving the interval takes about 150 more
    holdouts.
  - **Option (b):** stop the M arms at the switch, recorded as the end
    of the stream.
  - **Option (c):** draw the sequential arm's single reviewer's model
    too. That turns the M tier into a model comparison and gives up the
    second-review question there.
- **What cross-model pairs would and would not fix.** The escapes both
  reviewers miss are dominated by shared method gaps: unrun slow sets
  and unexercised ε rows. A different model running the same brief would
  likely share them. If the hope is that Fable catches what Opus misses,
  this readout's data suggests the brief's coverage, not the model, is
  the binding constraint for that class. For the tallied class, where
  the silent review "did not exercise the region", model diversity could
  plausibly help, and only a cross-model stream can say.
