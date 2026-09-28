# Dual Opus review — readout 1

Off-file (rule 10 of `docs/DUAL-REVIEW-PROTOCOL.md`). Owed under rule 9
at the twelfth fair pair (DR-13).

## 1. Population

- **Sources.** Rows DR-1..DR-13 from `contact/land-5:docs/DUAL-REVIEW-LOG.md`.
  `origin/main` (at `17cbcdaa8`) holds DR-1..DR-12, byte-identical to
  the branch's; the branch adds only DR-13 and the foot line. No row on
  main is missing from the branch.
- **Protocol.** All thirteen rows ran under `c3129311b` (DR-2 and DR-7
  write it `c3129311bd`, the same commit), the last commit touching the
  protocol on main. One protocol version; no seam inside the population.
- **Rows 13, fair pairs 12**: DR-1, DR-3..DR-13.
- **Excluded: DR-2** — method divergence (rules 4, 6(e)): R2's mutation
  reverts were refused by the session's permission classifier and it
  substituted compile probes and differential runs; R1 ran its reverts.
  Recorded in full, counted nowhere below except where marked.
- **Flags inside the fair set, not exclusions**: DR-7 (a disk note sent
  to both mid-review, ruled a box constraint); DR-9 (no brief sha256
  stored; tier chosen after spec, not at it); glimpses disclosed as
  names only in DR-1, DR-3, DR-4, DR-6, DR-10, DR-12, DR-13, none
  holding findings.

## 2. The measurement (rule 6)

Every MAJOR raised in the fair pairs, and each recorded near miss:

| row | finding | disposition |
|---|---|---|
| DR-5 | poisoned `Segment::Arc` carrier at b = 0 | MAJOR/MAJOR, bilateral — fails (a) |
| DR-9 | `Strand` on a `Declare` answered `Again` | R1 MAJOR / R2 MINOR, bilateral — fails (a) |
| DR-11 | line×torus silent miss (G1) | MAJOR/MAJOR, bilateral — fails (a) |
| DR-11 | R2's `ops.rs` face-interior-oval fallback | (a) unilateral MAJOR, (b) code, (c) distinct from G1, (d) executed with a mutant, (e) fair — **tallied** |
| DR-13 | one-outer-shell premise | MAJOR/MAJOR, bilateral — fails (a) |
| DR-13 | declared-only row a demonstrated wrong clear | MAJOR/MAJOR, bilateral — fails (a) |
| DR-4 | R1's executed `mass_properties` panic | raised as NOTE, out of fence — fails (a) |
| DR-12 | R1's red CI gate | unilateral MINOR — fails (a) |

DR-1, DR-3, DR-4, DR-6, DR-7, DR-8, DR-10, DR-12 carry no MAJOR from either
reviewer. DR-2 (excluded) carried none either.

**Tally: 1 over 12 fair pairs** (DR-11). Rule 9's first trigger (tally
8) was not reached; the second (12 fair pairs) was.

**Rate.** Tallied findings per fair pair: 1/12 = 0.083. Exact
(Clopper–Pearson) 95% interval for the per-pair probability of at
least one tallied finding: **0.002 to 0.385**. The point estimate rests
on one event.

## 3. The plentiful signal

Severity counts (MAJ/MIN/NOTE) are the rows' R1 and R2 cells. The
unilateral columns count only items the correspondence cell labels
MAJOR or MIN as stand-alone findings; items marked "inside MINOR",
unlabelled, NOTE or style are not counted.

| row | R1 | R2 | bilateral MAJOR | R1→ MAJ / MIN | R2→ MAJ / MIN |
|---|---|---|---|---|---|
| DR-1 | 0/6/6 | 0/6/4 | 0 | 0 / 1 | 0 / 0 |
| DR-3 | 0/7/4 | 0/5/5 | 0 | 0 / 2 | 0 / 2 |
| DR-4 | 0/4/4 | 0/4/5 | 0 | 0 / 1 | 0 / 1 |
| DR-5 | 1/2/3 | 1/3/7 | 1 | 0 / 0 | 0 / 1 |
| DR-6 | 0/3/3 | 0/2/3 | 0 | 0 / 0 | 0 / 0 |
| DR-7 | 0/6/1 | 0/3/3 | 0 | 0 / 1 | 0 / 0 |
| DR-8 | 0/2/4 | 0/2/9 | 0 | 0 / 0 | 0 / 0 |
| DR-9 | 1/2/3 | 0/3/5 | 0 (1 split MAJOR/MINOR) | 0 / 0 | 0 / 0 |
| DR-10 | 0/3/5 | 0/2/7 | 0 | 0 / 2 | 0 / 0 |
| DR-11 | 1/3/1 | 2/3/3 | 1 | 0 / 2 | 1 / 2 |
| DR-12 | 0/2/4 | 0/3/6 | 0 | 0 / 1 | 0 / 0 |
| DR-13 | 2/2/1 | 2/4/1 | 2 | 0 / 0* | 0 / 0* |
| **total** | 5/42/39 | 5/40/58 | **4** | **0 / 10** | **1 / 6** |

\* DR-13's unilateral items carry no severity label; each reviewer's
MINOR count is fully accounted for by bilateral groups, so 0 by
inference. DR-2 (excluded): R1→ 3 MIN, R2→ 1 MIN.

R1/R2 are dispatch-order labels only (rule 2). Any difference between
the R1→ and R2→ columns is noise or position, not a reviewer property.

## 4. Severity divergence

Bilateral findings drawing different severities on the
MAJOR/MINOR/NOTE/style ladder, fair pairs: **20** — DR-1 (1), DR-3 (2),
DR-4 (3), DR-5 (1), DR-6 (2), DR-7 (1), DR-9 (3), DR-10 (2), DR-11 (3),
DR-12 (1), DR-13 (1). DR-8 records two more off the ladder ("remark",
"none vs NOTE"), not counted. **One divergence crosses the MAJOR line:
DR-9** (`Strand`/`Declare`, R1 MAJOR vs R2 MINOR, both executed with
identical results; the verdict split tracks it). Bilateral coding
removes it from the tally whatever the severity.

## 5. Cost

Review tokens, fair pairs: R1 total 2,548,094, R2 total 2,617,227; mean
~215k per review. **R2's share of the pair's review tokens: 51%**
(per-row range 45% DR-1 to 58% DR-11). Against the implementer figure
where the row records one (scopes differ: DR-3 first pass only, DR-11
three phases, DR-9 approximate), R2 alone ran 0.19× (DR-11) to 0.88×
(DR-1) of the implementer's tokens; DR-7, DR-8, DR-12, DR-13 record no
implementer figure. Blinded coders add 43–73k per row where recorded
(DR-3..DR-6, DR-8..DR-13).

Wall-clock per review (harness, minutes, R1/R2): DR-1 31/52, DR-3
52/51, DR-4 ~42/~26, DR-5 93/71, DR-6 ~130/~189 (mostly build-slot
waits), DR-7 35/28, DR-8 12/28, DR-9 77/113, DR-10 37/30, DR-11 20/25,
DR-12 23/18, DR-13 45/38. The reviews ran concurrently, so a pair's
elapsed time is near the longer of the two; the second review's
elapsed cost is build-slot contention, which the rows do not measure.

## 6. What the rows cannot tell

- **Missing data (6(e)).** DR-7 records no attribution-stripped coding
  byte or coder; DR-1 no coder tokens; DR-9 no brief sha256; DR-5's
  coder lost R2's verdict line; DR-4, DR-6, DR-10 were coded from
  orchestrator-condensed transcriptions; unilateral severities are
  unlabelled in parts of DR-4, DR-6 and DR-13; implementer tokens are
  absent for four rows. None is counted as a zero.
- **Escapes (rule 11).** No row carries a later-escape line yet, so the
  upper bound that joint misses place on a second review is unmeasured.
  Several fix passes and delta reviews found further defects neither
  reviewer raised (DR-6's wall arm, DR-8's third and fourth lenient
  levers, DR-13's refused hollow seats); these are recorded in the fix
  pass cells, not coded as escapes.
- **Seams.** One era: every pair post-dates the 2026-09-22 Opus 5.5
  release and the protocol records no model change. Not comparable with
  the A/B log's cross-model or fable/fable duals.
- **Population.** Hard units only, as the review tiers select them; three
  of twelve fair pairs are CONTACT (DR-8, DR-12, DR-13). The rows say
  nothing about what a second review buys on units that get one review.
- **Instrument.** v6 item 2's thresholds were set because the
  informative unit is the tallied unilateral MAJOR and zero-MAJOR pairs
  are nearly uninformative; eight of twelve fair pairs carry no MAJOR
  from either reviewer. The tally also cannot see a defect a reviewer
  demonstrated but graded below MAJOR (DR-4's panic).

## 7. The decision it informs

For Ev under rule 9; duals continue until a ruling. Options the data
bears on, not ranked:

- continue duals on the current tier unchanged, and read again at a
  later threshold;
- narrow the dual tier (for example to a subset of the hard classes);
- change the triage that sends units to a dual;
- change the instrument (for example what counts toward the tally, or
  recording escapes and fix-pass/delta discoveries as a second
  measure);
- stop the stream.
