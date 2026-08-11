# Readouts archive — moved off the protocol file

`docs/MODEL-AB-LOG.md` is read by every orchestrator before dispatch, so
under the results-off-file rule (Evan, 2026-08-11) it carries no
arm-comparison results. Text moved out of it lands here, unaltered, so
the record survives outside git history.

Current readout: `report.html` in this directory (second readout,
2026-08-11). Below: the earlier text, verbatim as it stood on main.

---

## M5-close readout (2026-08-03, PR 14)

Scope: rows 11–40 are the M5 dispatches (rows 1–10 were M4; the
reference rows in the footer are pre-experiment). Thirty M5 rows,
plus the two unnumbered no-blinded-lane units above.

**Arm balance.** M5 rows 11-40: **fable 15, opus 15.** The blocked
randomization held — every block after M4's block 1 drew its order
from `/dev/urandom`, and the pairing landed the milestone exactly
even without any further override.

**Stratified by pre-logged difficulty** (difficulty was logged before
the flip or before assignment in every M5 row; the one ordering slip,
row 29, is recorded in its own cell):

| difficulty | fable rows | opus rows | fable MAJ | opus MAJ | fable silent | opus silent |
|---|---|---|--:|--:|--:|--:|
| **L** (9) | 12, 14, 19, 28, 32 | 25, 29, 31, 40 | 6 | 5 | 2 | 3 |
| **M** (12) | 15, 20, 26, 34, 36 | 11, 13, 18, 21, 33, 35, 37 | 4 | 1 | 1 (+1 unrecorded, row 36) | 1 |
| **S** (9) | 17, 23, 30, 38, 39 | 16, 22, 24, 27 | 1 | 0 | 0 (+1 unrecorded, row 38) | 0 |
| **total** | 15 | 15 | **11** | **6** | **3 recorded, 2 unrecorded** | **4** |

Both arms are within one row of each other at every difficulty level
except M, where opus drew seven to fable's five.

**MAJOR findings — read the classifications, not the counts.** The
raw totals (fable 11, opus 6) are not a quality signal, because the
review record classifies a large share of them as something other
than implementation defects:

- **Design forks ruled by Evan, not defects**: row 15's two MAJs.
- **Ruled ACCEPT-AND-BANK** (the finding became a scheduled unit,
  PR 7b): row 25's M2.
- **Claim- or proof-text scope, not code**: row 31's MAJ; row 33's
  was a premise refutation returned as a MAJOR against the *spec*.
- **Real defects outside the unit's own acceptance target**: row
  40's octant `e0` pick (tier-3 lost on non-square prisms; the die,
  which the unit shipped, is unaffected).
- **Real, consequential, on the unit's own geometry**: row 19's
  MAJ-1 — an even-crossing silent one-sided split — the project's
  only REJECT. Its fix pass exposed and fixed two further latent
  defects and re-reviewed at APPROVE 5/5/5. Row 28's three (two
  silent) and row 20's one (a silent corrupt STL via a hole-creating
  merge role inversion) are the other members of this class.

Counting only that last class, the milestone's genuinely
consequential implementation MAJORs are rows 19, 20, 28 (fable) and
row 40 (opus) — four across thirty dispatches, and present on both
arms.

**Silent deviations** — the metric the protocol weights worst, and
the one where the arms are closest to indistinguishable. M5 total:
**fable 3** (row 26's center-shift ring-fallacy; row 28's two),
**opus 4** (row 11's stale-claims sweep leaving live rustdoc inari
mentions; row 29's two node-layer sweeps; row 40's Band-4 scope
gap). Two fable rows (36, 38) have no silent-deviation datum
recorded at all, so fable's true count is 3-5. Every other M5 row
recorded 0 silent alongside a nonzero count of *reported*
deviations — the reporting discipline itself held well on both arms,
which is the outcome the protocol most wanted.

**Fix-pass size distribution.** Rows 36, 38 and 40 were described
narratively and never classified; they are counted as unclassified
rather than folded into a bucket.

| size | fable | opus |
|---|---|---|
| none | 30, 39 | 22, 27 |
| light / tiny | 23, 34 | 24, 33, 35, 37 |
| moderate | 14, 17, 20, 26, 32 | 11, 13, 18, 21, 31 |
| substantial / heavy | 15, 19, 28 | 25, 29 |
| unclassified | 36, 38 | 40 |

Several cells carry a qualified size in the row itself
("moderate+", "light + one gate red", "moderate, in flight");
collapsing those into buckets loses information the row cells keep,
and the row cells are authoritative. Read directionally: the
distributions overlap heavily, with the heavy tail populated by both
arms and driven by unit scope rather than arm.

**What the milestone shows, honestly.**

1. **No arm-level quality difference is visible at this n.** Both
   arms produced clean rows and both produced the milestone's
   heaviest fix passes. Both arms carried silent deviations (fable 3
   recorded plus 2 rows with no datum, opus 4) — the metric the
   protocol weights worst, and it does not separate them. Both arms had a row where the review found a real,
   consequential defect that shipping would have carried (row 19
   fable, row 40 opus). The M4-close reading — "no evidence Opus
   implementation is worse at this scale; suggestive that it's
   comparable" — is unchanged by thirty more rows, and it is now
   supported by a difficulty-stratified sample rather than a skewed
   one.
2. **The confounds have NOT gone away and are not small.** Reviewer
   variance is still unmeasured — the same orchestrator-model
   reviewed both arms, and review depth demonstrably varied across
   the milestone (row 19's review found three MAJORs on geometry that
   three earlier reviews of comparable units did not probe as hard).
   Difficulty labels are one orchestrator's pre-flip guess, not a
   calibrated scale. Unit scope varied by more than an order of
   magnitude within the same difficulty letter. Fix passes were
   sometimes run by the implementer's own agent and sometimes
   orchestrator-applied.
3. **No significance is claimed, and none is available.** n = 40 with
   a binary arm, an unblinded orchestrator, a subjective outcome
   scale, and multiple uncontrolled confounds does not support a
   significance claim, and no test is reported here. The honest
   summary is the same shape as M4's: *the experiment has produced no
   evidence that either model is worse at this work, and the sample is
   now large enough that a large effect would probably have shown.* A
   small effect would not have, and this design cannot find one.
   Arm balance (15/15) and difficulty balance are the two things this
   milestone did materially improve over M4's 4-0 opening skew.

**Data-quality findings this readout is obliged to state.**

- **The table was five rows stale at milestone close** and rows
  36–40 had to be reconstructed from prose. The reconstruction is
  faithful but lossy — see the `—` cells.
- **The rubric (idiom/tests/docs) is missing for rows 36, 38, and
  40** and was never recorded. Row 40 is an L-difficulty row, so the
  most informative single rubric of the milestone's end is absent.
- **Tokens and wall-clock are absent for every row from 13 onward**
  ("(in log)" was written in place of a figure and the figure was
  never carried across). The protocol lists them as per-row objective
  companions; in practice the experiment collected them for twelve
  rows and then stopped. Any future cost comparison between arms is
  therefore not available from this log.
- **Two rows (36, 38) lack a silent-deviation count**, the
  protocol's most heavily weighted metric.
- Recommendation for the next milestone, if the experiment
  continues: record the row AT MERGE rather than at next-touch, and
  treat a missing rubric or silent-dev count as a merge blocker for
  the row — the cheap discipline that would have prevented every gap
  above.


---

## Protocol v3's original rationale (moved 2026-08-11)

Verbatim as it stood before the strike:

> rationale: the readout shows no quality separation, a consistent lean
> toward opus on findings and cost, and modest power loss at 2:1 (~12%
> contrast-variance inflation), so allocation shifts toward the cheaper
> arm while keeping a live fable stream for drift detection.

Note for anyone citing it: the **cost** half of that clause was retired
by the second readout (roughly double the cost data; the token lean
shrank and wall-clock reversed, neither separating). The findings half
strengthened.
