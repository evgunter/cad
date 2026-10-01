# Dual Opus review — readout 2

Off-file (rule 10 of `docs/DUAL-REVIEW-PROTOCOL.md`). Owed under rule 9
as Ev re-set it on PR 3342 (commit `cb47d67c4`): "a readout is owed when
TWELVE fair pairs have found any MAJOR, or when the tally reaches EIGHT,
whichever comes first". The twelfth such pair is DR-30. The tally is 5,
so the first trigger was not reached; the second was.

Source: `docs/DUAL-REVIEW-LOG.md` on `origin/main` at `b9df52747`
(DR-1..DR-30, the DR-11 later-escape line and the foot line). DR-1..DR-12
are byte-identical to the rows readout 1 read at `17cbcdaa8`. The rows
are read as recorded. Where a cell contradicts another cell, this readout
says which one it used.

## 1. Population, exclusions and flags

**Which rows, and why.** Rule 9 counts *fair pairs that found any
MAJOR*, and that count started at DR-1: the four pairs readout 1 already
covered (DR-5, DR-9, DR-11, DR-13) are among the twelve. A rate also
needs the pairs that found nothing. This readout therefore covers **every
row DR-1..DR-30**, in three nested sets:

- **all rows, 30.** Excluded from every count: **DR-2** (method
  divergence, rules 4 and 6(e), as in readout 1) and **DR-22** (6(e):
  R2 was truncated by two harness hand-backs, so its executions never
  ran; the row marks it not fair). DR-22 carried no MAJOR on either side.
- **fair pairs, 28**: DR-1, DR-3..DR-21, DR-23..DR-30. These are the
  denominator for every rate below.
- **fair pairs that found any MAJOR, 12** (rule 9's set): DR-5, DR-9,
  DR-11, DR-13, DR-15, DR-16, DR-17, DR-21, DR-24, DR-28, DR-29, DR-30.
  This matches the log's foot line. Section 2 covers these twelve in
  full.

**New since readout 1**: DR-14..DR-30, which is 17 rows and 16 fair
pairs. 8 of the 16 found a MAJOR. DR-20 and DR-21 carry the date
2026-09-24 and protocol `c3129311bd`: they ran before readout 1 was
written, but they were recorded on main on 2026-09-29, after it.
Readout 1's population could not contain them.

**Protocol.** There is one method throughout. DR-1..DR-14 ran under
`c3129311b`, and DR-2, DR-7, DR-20 and DR-21 write it `c3129311bd`.
DR-15 onward record `cb47d67c4`, the ruling commit, which changed only
rule 9's readout point. DR-17, DR-20, DR-21 and DR-28 say they were
dispatched under `c3129311b` and recorded under `cb47d67c4`. DR-29
records `29ebcc63e6` and DR-30 records `35d8ddab9`. Those are merge
commits on main, not the last commit that touched the protocol, but the
protocol text at both is identical to `cb47d67c4`'s (empty diff). There
is no method seam inside the population.

**Model era.** All rows fall in the Opus 5.5 era. The protocol records
no model change.

**Flags inside the fair set (not exclusions).** Readout 1's flags stand
(DR-7, DR-9, and name-only glimpses in DR-1, DR-3, DR-4, DR-6, DR-10,
DR-12, DR-13). New flags:

- **Name-only glimpses, no findings:** DR-15 (R1 listed scratchpad file
  names), DR-16 (R1 saw R2's lane directory names), DR-20 (R2 saw R1's
  nextest command line), DR-23, DR-24 and DR-25 (commit-trailer glimpses,
  plus other units' process-log hits), DR-26 and DR-27 (R1 saw the other
  lane's directory or worktree name). In DR-26 and DR-27 the coder's
  header grep also showed another unit's row (DR-1).
- **DR-28: sequential, not concurrent.** R2 ran after R1 delivered,
  under rule 2's late-trigger fallback, on the same frozen head. Disk
  space allowed one build at a time. The row rules it fair, and its
  elapsed time is the sum of both reviews, not the longer one.
- **Blinding weaker than rule 7 asks:** the orchestrator, not a separate
  coder, did the attribution-stripped coding for DR-23, DR-24 and
  DR-25. The DR-18 pre-note was "coded by the orchestrator with
  attributions visible", and the row names no separate coder for the
  coding itself. DR-28 records a byte (35) but no coder.
- **Re-tiered at delivery:** DR-25 and DR-27 were SINGLE at dispatch and
  became DUAL on the delivered scope. Both rows say this was logged
  before either review ran. It bears on population selection (rule 1),
  not on fairness.
- **Running counts that disagree with the foot line.** DR-17 writes
  "Running: 1" (the tally was 2 after DR-16; the row was renumbered
  behind DR-16). DR-28 writes "Running: 0" (it was recorded from a
  branch based on 2026-09-24). DR-18 says its severity divergence is
  "on the acyclicity prose only", but its own coding shows three
  divergent pairs. This readout counts from each row's coded cells, not
  from its running figure.
- **DR-30:** R2 copied the implementer's target dir to warm its cache,
  read nothing from it, and disclosed this. It was R2's own choice
  within one brief.
- **A flag for a future row (PR 3678, CLEAVE).** In the session that
  recorded DR-30, the CLEAVE orchestrator read readout 1's contents. At
  that time its next dual, PR 3678, already had both reviews delivered
  and adjudicated, but its row was not yet recorded. Rule 10 says an
  orchestrator with a dual in flight should not read a readout. Both
  reviews and the adjudication came before the reading. What the reading
  can still reach is the row's recording: its correspondence pre-note,
  its tally disposition and any severity calls made while writing it.
  That row should carry this flag. The rows do not say when in that
  session the reading fell relative to DR-30's own recording. DR-30 was
  coded by a separate blind coder, and its tally disposition (a
  bilateral MAJOR, which fails 6(a)) is mechanical.

## 2. The measurement (rule 6)

Every MAJOR raised in the twelve pairs, grouped by underlying defect as
the coders deduplicated them:

| row | finding | disposition |
|---|---|---|
| DR-5 | poisoned `Segment::Arc` carrier at b = 0 | MAJOR/MAJOR, bilateral — fails (a) |
| DR-9 | `Strand` on a `Declare` answered `Again` | R1 MAJOR / R2 MINOR, bilateral — fails (a) |
| DR-11 | line×torus silent miss (G1) | MAJOR/MAJOR, bilateral — fails (a) |
| DR-11 | R2's `ops.rs` face-interior-oval fallback | (a) unilateral MAJOR, (b) code, (c) distinct from G1, (d) mutant executed, (e) fair — **tallied** |
| DR-13 | one-outer-shell premise | MAJOR/MAJOR, bilateral — fails (a) |
| DR-13 | declared-only row a demonstrated wrong clear | MAJOR/MAJOR, bilateral — fails (a) |
| DR-15 | `axis_pose` origin pivot (a far origin reclassifies a carrier line; R2's W0 clear at 1 km) | R2 MAJOR / R1 MINOR, both executed, bilateral — fails (a) |
| DR-16 | the first-Out / first-root mutants survive the suite | MAJOR/MAJOR, both executed, bilateral — fails (a) |
| DR-16 | the parallel arm certifies phantom roots (in-band tilt read as exact) | R2 MAJOR executed / R1 MINOR argued, bilateral — fails (a) |
| DR-16 | R1's half-angle pole conditioning (wrong certified counts against a bisection oracle; a silent miss in `sweep_traces`) | (a) unilateral MAJOR, (b) code, (c) held distinct from R2's lever-scaling over-refusal, (d) executed against an oracle, (e) fair — **tallied** |
| DR-17 | an edge collinear with the probe ray behind `p` refuses in band | MAJOR/MAJOR, both executed with a fix mutant, bilateral — fails (a) |
| DR-21 | R2's MINT-ORDER: the refusal-order move is observable at `Dual64` through a public-API mint | (a) unilateral MAJOR (R1 asserted the opposite by inspection), (b) code, (c) one group, (d) base-vs-head probe with measured values, (e) fair — **tallied**. R2 itself rated the reach as public kernel API only; the orchestrator upheld it as blocking |
| DR-24 | R2's torn-body panic: `kev_describing`'s write-loop `unreachable!` is reachable | (a) unilateral MAJOR, (b) code, (c) one defect (the killed-edge naming has the same root), (d) red probe plus seeded search (22 panics per 96,000 kills), (e) fair — **tallied** |
| DR-28 | a `FromFace` side refuses `Unpinned` on every analysis lane | R1 MAJOR / R2 NOTE, both executed, bilateral — fails (a). The fix pass ruled the refusal honest and disclosed it at every door, rather than changing it |
| DR-29 | R1's aux-key MAJOR: `aux_partner` is keyed by partner face, so a radical plane is reused for a second sphere pair | (a) unilateral MAJOR, (b) code, (c) one defect (a doc item and the plane×sphere extension fold in), (d) red probe plus a planted differential to an axisymmetric oracle, (e) fair — **tallied** |
| DR-30 | the column-gap predicate refuses `OrderEscalated` across different faces a few ε apart, where main answers | MAJOR/MAJOR, both executed on head and main, bilateral — fails (a) |

**Recorded near misses, new rows:**

- DR-14: R2's corner-bar lens is a unilateral MINOR, demonstrated with a
  probe. It fails (a). The stopgap PR 3336 covered it.
- DR-15: each reviewer has unilateral MINORs executed by surviving
  mutants. They fail (a).
- DR-27: R1 flagged two findings "MAJOR by the brief's letter" but
  graded them MINOR, and the orchestrator ruled neither MAJOR. Both are
  bilateral. They fail (a).
- DR-29: R2's spun ball, which refuses at TANG's pierce-ring door, is a
  unilateral MINOR, executed. It fails (a).

Readout 1's near misses for DR-4 and DR-12 stand.

**Tally: 5 over 28 fair pairs** (DR-11, DR-16, DR-21, DR-24, DR-29).
Each tallied pair holds exactly one tallied finding. By dispatch
position: R1 2 (DR-16, DR-29), R2 3 (DR-11, DR-21, DR-24). Positions are
labels only (rule 2).

**Rates, with exact (Clopper–Pearson) 95% intervals:**

| measure | readout 1 (DR-1..13) | new (DR-14..30) | cumulative |
|---|---|---|---|
| fair pairs with a tallied finding | 1/12 = 0.083 (0.002–0.385) | 4/16 = 0.250 (0.073–0.524) | **5/28 = 0.179 (0.061–0.369)** |
| fair pairs that found any MAJOR | 4/12 = 0.333 (0.099–0.651) | 8/16 = 0.500 (0.247–0.753) | **12/28 = 0.429 (0.245–0.628)** |
| tallied, given the pair found a MAJOR | 1/4 = 0.25 (0.006–0.806) | 4/8 = 0.50 (0.157–0.843) | **5/12 = 0.417 (0.152–0.723)** |

**MAJOR groups across the twelve pairs: 16.**

- 7 were raised MAJOR by both reviewers: DR-5, DR-11 G1, DR-13 ×2,
  DR-16 first-root, DR-17, DR-30.
- 4 were raised MAJOR by one reviewer and lower by the other: DR-9,
  DR-15, DR-16 parallel arm, DR-28.
- 5 were raised MAJOR by one reviewer and not mentioned by the other.
  These are the five tallied findings.

Every unilateral MAJOR raised in a fair pair met (b) through (e). In
these rows, criterion (a) is the only one that removed a MAJOR from the
tally.

## 3. The plentiful signal

The counting convention is readout 1's. Severity counts (MAJ/MIN/NOTE)
come from the R1 and R2 cells. The unilateral columns count items the
correspondence cell labels MAJOR or MIN as stand-alone findings. Items
that are unlabelled, NOTE, NIT or style are not counted. Rows DR-1..13
are in readout 1. The new fair rows follow.

| row | R1 | R2 | bilateral MAJOR | R1→ MAJ / MIN | R2→ MAJ / MIN |
|---|---|---|---|---|---|
| DR-14 | 0/3/2 | 0/2/1 | 0 | 0 / 0 | 0 / 1 |
| DR-15 | 0/4/2 | 1/5/3 | 0 (1 split MAJOR/MINOR) | 0 / 2† | 0 / 4† |
| DR-16 | 2/4/2 | 2/2/2 | 1 (+1 split MAJOR/MINOR) | 1 / 2 | 0 / 1 |
| DR-17 | 1/1/4 | 1/2/2 | 1 | 0 / 0* | 0 / 0* |
| DR-18 | 0/3/1 | 0/3/3 | 0 | 0 / 1 | 0 / 2 |
| DR-19 | 0/1/6 | 0/1/3 | 0 | 0 / 0 | 0 / 0 |
| DR-20 | 0/5/4 | 0/6/3 | 0 | 0 / 0 | 0 / 1 |
| DR-21 | 0/2/6 | 1/2/2 | 0 | 0 / 0 | 1 / 0 |
| DR-23 | 0/6/8 | 0/5/6 | 0 | 0 / – | 0 / – |
| DR-24 | 0/5/8 | 1/4/4 | 0 | 0 / – | 1 / – |
| DR-25 | 0/2/7 | 0/3/7 | 0 | 0 / – | 0 / – |
| DR-26 | 0/4/1 | 0/4/2 | 0 | 0 / 0 | 0 / 0 |
| DR-27 | 0/4/1 | 0/5/4 | 0 | 0 / 0 | 0 / 3 |
| DR-28 | 1/6/3 | 0/4/4 | 0 (1 split MAJOR/NOTE) | 0 / 1 | 0 / 0 |
| DR-29 | 1/4/4 | 0/2/3 | 0 | 1 / 2 | 0 / 1 |
| DR-30 | 1/2/5 | 1/2/6 | 1 | 0 / 1 | 0 / 1 |
| **new total** | 6/56/64 | 7/52/55 | **3** (+3 splits) | **2 / 9** | **2 / 14** |
| **cumulative, 28 fair** | 11/98/103 | 12/92/113 | **7** (+4 splits) | **2 / 19** | **3 / 20** |

Notes on the table:

- † DR-15's correspondence cell gives no severities for unilateral
  items. The counts are inferred from the reviewer cells. A MINOR whose
  mutants split across bilateral and unilateral parts counts once in
  each direction where a unilateral part exists.
- \* DR-17: every MINOR in either reviewer cell is accounted for by a
  bilateral group, so 0 by inference.
- – DR-23, DR-24 and DR-25 code their unilateral items without
  severities, and the counts cannot be inferred. They are missing, not
  zero. The cumulative MIN totals exclude them.
- DR-27's R1→ test-gap (no fast-CI row saves and loads at depth) is
  unlabelled and not counted.
- DR-22 (excluded): R1→ 0 MIN, R2→ 0 MIN, with five unilateral
  NOTE/style items between them.

In the labelled rows, unilateral MINORs run at roughly 1.4 per fair pair
cumulatively (39 over 25 rows with labels). Unilateral MAJORs number 5
over 28 pairs. As in readout 1, any difference between the R1→ and R2→
columns is noise or position, not a reviewer property.

## 4. Severity divergence

Bilateral findings that drew different severities on the
MAJOR/MINOR/NOTE/style ladder, new fair rows: **30**.

- DR-14: 2
- DR-15: 1
- DR-16: 1
- DR-17: 1
- DR-18: 3
- DR-20: 4
- DR-21: 1
- DR-23: 1
- DR-24: 3
- DR-26: 4
- DR-27: 3
- DR-28: 1
- DR-29: 2
- DR-30: 3

DR-19 and DR-25 have none. Two more are off the ladder and not counted:
DR-20 (NOTE vs "obs.") and DR-27 (unrated vs MIN).

**Cumulative over 28 fair pairs: 50.**

**Divergences across the MAJOR line: 3 new, 4 cumulative.**

- DR-9: MAJOR vs MINOR.
- DR-15: R2 MAJOR vs R1 MINOR. Both executed independent probes of the
  same origin pivot.
- DR-16: R2 MAJOR executed vs R1 MINOR argued, on the parallel arm.
- DR-28: R1 MAJOR vs R2 NOTE. Both executed. It is the widest split in
  the log, and the fix pass resolved it by disclosure, not by a code
  change.

Bilateral coding takes all four out of the tally, whatever the
severity. Of the 16 MAJOR groups, 9 were graded MAJOR by only one of the
two reviewers. Five of those were unilateral and four were split.

## 5. Cost

**Review tokens, new fair pairs:**

| | readout 1 (12 fair) | new (16 fair) | cumulative (28 fair) |
|---|---|---|---|
| R1 total | 2,548,094 | 4,689,827 | 7,237,921 |
| R2 total | 2,617,227 | 4,613,608 | 7,230,835 |
| mean per review | ~215k | ~291k | ~258k |
| R2's share of the pair's review tokens | 51% | **49.6%** | **50.0%** |

R2's share in the new rows ranges from 45.7% (DR-16) to 58.1% (DR-30).

**Against the implementer's tokens**, where the row records them:

| row | R2 tokens | implementer tokens | ratio |
|---|---|---|---|
| DR-20 | 321,390 | 421,544 | 0.76× |
| DR-27 | 304,695 | 653,216 | 0.47× |
| DR-28 | 274,648 | 603,047 (across both legs) | 0.46× |

The other new rows record fix-pass lanes but no implementer figure.

**Fix passes** run off the union, so they are not attributable to the
second review alone. Where recorded they ran 315k–609k tokens: DR-14
609k, DR-21 454k, DR-23 478k, DR-24 448k, DR-25 315k, DR-26 605k, DR-27
540k. DR-22's implementer is ~1.26M across all phases.

**Delta reviews**, an added cost that is not part of the dual:

- DR-16: two single deltas.
- DR-17: one.
- DR-29: one.
- DR-30: both reviewers ran deltas.

**Blinded coders** add 51–91k tokens per row where a separate coder is
recorded: DR-14, DR-15, DR-16, DR-17, DR-19, DR-26, DR-27, DR-29,
DR-30.

**Wall-clock per review** (harness, minutes, R1/R2):

| row | R1/R2 | row | R1/R2 |
|---|---|---|---|
| DR-14 | ~43/~40 | DR-23 | 86/46 |
| DR-15 | ~65/~40 | DR-24 | 28/32 |
| DR-16 | ~109/~110 | DR-25 | 115/163 |
| DR-17 | 124/119 | DR-26 | 87/76 |
| DR-18 | 19/17 | DR-27 | 58/79 |
| DR-19 | ~65/~58 | DR-28 | 15/20, sequential, so the pair took ~35 |
| DR-20 | 25/26 | DR-29 | 17/12 |
| DR-21 | 28/61 | DR-30 | 50/62 |

Elapsed cost is still dominated by build-slot and disk contention, which
the rows mention but do not measure:

- DR-22's truncation came from two concurrent duals on a 4-core box.
- DR-28 was serialised by disk.
- DR-19's R2 argued the tour instead of running it because the slots
  were congested.

## 6. What the rows cannot tell

- **Missing data (6(e)).** Readout 1's list stands. New gaps:
  - unilateral severities are unlabelled in DR-23, DR-24 and DR-25, and
    inferred in DR-15 and DR-17;
  - DR-20, DR-21 and DR-28 record no coder tokens;
  - DR-28 names no coder;
  - DR-23, DR-24 and DR-25 were coded by the orchestrator;
  - DR-26's implementer report was lost to a container restart;
  - implementer tokens are absent for most new rows.

  None is counted as a zero.
- **Escapes (rule 11).** One later-escape line exists: DR-11 (PR 3265).
  A torus face met a partner only in a face-interior oval while
  crossings existed elsewhere, and `union` counted the lens twice.
  Neither review raised that instance. Both raised the no-crossings
  instance, which is R2's tallied MAJOR. That is one joint miss in 28
  fair pairs as recorded, too few to bound what a second review buys.

  Beyond rule 11, fix passes and delta reviews found defects that
  neither reviewer had raised. They sit in the fix-pass cells and are
  not coded as escapes:
  - DR-16: a single delta review found a new MAJOR, f64 noise certified
    at large circle radius. It was pre-existing and made silent by the
    new lane.
  - DR-29: a delta found that the cross-arm row never reached its
    subject.
  - DR-30: the reviewers' own deltas found three more items.
  - DR-14: the implementer lane's ∖/∩ measurement surfaced DR-11's
    escape.
  - DR-15: the `:833` coplanar-conic premise gap was filed as
    pre-existing.
  - DR-24: the orchestrator filed `mev_fan_plan`'s same orbit gap.

  A second measure of the kind readout 1 listed would have something to
  count.
- **Selection.** Hard units only, and the tier is chosen per
  `memories/orchestration-model.md`. Two rows were re-tiered at
  delivery. The size labels are not uniform: some rows use S/M/L, some
  use H. Read as written, MAJOR-finding and tallied pairs by size are:

  | size | fair pairs | found a MAJOR | tallied |
  |---|---|---|---|
  | M | 10 | 1 (DR-11) | 1 (DR-11) |
  | H, incl. DR-21's "H by breadth" | 10 | 6 | 3 (DR-16, DR-21, DR-24) |
  | L | 8 | 5 | 1 (DR-29) |

  The six H/SOUNDNESS rows found a MAJOR four times and tallied once.
  By program, GERM holds 3 of the 12 MAJOR pairs and 2 of the 5 tallied
  findings. These cells are too small to rank classes, and the rows say
  nothing about units that get one review.
- **Drift across the two windows.** The tallied rate rose from 1/12 to
  4/16, and the MAJOR-finding rate from 4/12 to 8/16. The intervals
  overlap widely. The rows cannot separate harder units, changed triage,
  reviewers or briefs, or orchestrator expectancy (readout 1 existed on
  its branch from 2026-09-28) as causes. The rows do not record whether
  any orchestrator who adjudicated DR-14..DR-30 had read readout 1. The
  one known reading is the PR 3678 flag in section 1.
- **Instrument.** The tally cannot see:
  - a demonstrated defect graded below MAJOR by its only raiser (DR-14's
    lens, DR-29's spun ball);
  - a MAJOR the other reviewer saw at a lower grade (DR-28: MAJOR vs
    NOTE, where a single NOTE review might not have blocked).

  Whether a split finding would have been fixed under one review is not
  recorded. 16 of 28 fair pairs carry no MAJOR from either reviewer.
- **Seams.** One model era and one protocol method, as section 1 sets
  out. Not comparable with the A/B log's duals.

## 7. The decision it informs

For Ev under rule 9. Duals continue until a ruling. The options the data
bears on, not ranked:

- continue duals on the current tier unchanged, and read again at a
  later threshold (another count of MAJOR-finding pairs, or the tally of
  eight);
- narrow the dual tier, for example to the size or task classes in
  section 6, accepting the small cells;
- change the triage that sends units to a dual, including the
  re-tier-at-delivery path;
- change the instrument, for example by:
  - how splits across the MAJOR line are scored;
  - adding a second measure for fix-pass, delta-review and later-escape
    discoveries;
  - requiring a separate blind coder;
  - requiring unilateral severities in every correspondence cell;
- stop the stream.
