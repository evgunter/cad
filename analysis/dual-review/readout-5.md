# Dual review — readout 5

Off-file (rule 10 of `docs/DUAL-REVIEW-PROTOCOL.md`). This is the second
**full readout** under rule 9 as Ev wrote it on PR 4283. It is owed on
the miss leg. Rule 9 says, verbatim:

> a **full readout** when TWENTY M-tier units have been recorded under
> rule 1's arms (holdout and sequential together), or at the second
> M-TIER MISS after the last full readout's trigger, whichever comes
> first

The last full readout's trigger was DR-144 (readout 4, ruled on PR
4555). The two M-tier misses since are **DR-147** (GERM #4530) and
**DR-165** (INTENT #4618). Each is a holdout pair in which one review
raised a tallied MAJOR and the other raised no MAJOR. DR-147 got its
short readout (`analysis/dual-review/short-readout-dr147`). DR-165 is
the second miss, so this full readout is owed. By the rows, 12 M-tier
units have been recorded since DR-144, so the twenty-unit leg was not
reached.

**Source.** `docs/DUAL-REVIEW-LOG.md` at `1c4f9960d9`, the merge of PR
4618, which carries the DR-165 row and the foot line it rewrites. One
row's lost text is read at `24e28f5bdf` (section 0). The protocol is
read at the same commit. It is byte-identical to its text at
`4b5880058a`, and no commit on main since the checkout's graft has
touched it. The four earlier readouts are read on
`analysis/dual-review/readout-1` to `-4`, and the short readout on
`analysis/dual-review/short-readout-dr147`. Ev's rulings on readouts 2,
3 and 4 are the three dated bullets under the rows table. Every count
below comes from the row cells. Where a cell contradicts another cell
or the foot, this readout names the cell it used and lists the
disagreement in section 6.

Position labels: in a holdout pair, R1 and R2 are dispatch order only
(rule 2). In a sequential unit, "first" and "second" are the arm's own
order. AWF = APPROVE-WITH-FIXES, NMAI = NOT-MERGEABLE-AS-IS.

## 0. Data hygiene: which rows were read, and how

The log was edited concurrently by many programs between DR-144 and
DR-165. At `1c4f9960d9`:

**Rows inside the table after DR-144 (14):** DR-145, DR-146, DR-147,
DR-148, DR-149 (INTENT #4583), DR-150 (NURBS #4540), DR-151, DR-152,
DR-153, DR-156, DR-158, DR-160, DR-161, then the foot line.

**Rows displaced below Ev's three ruling bullets (7).** They sit after
the dated bullets, outside the table, so Markdown does not render them
as table rows:

- DR-149 (FLUX #4554) and DR-150 (FLUX #4557). These numbers collide
  with the in-table DR-149 (INTENT #4583) and DR-150 (NURBS #4540).
  Pending PR #4602 renumbers the FLUX rows to DR-155 and DR-159 and
  moves them up.
- DR-157 (GERM #4592), DR-162 (INTENT #4608), DR-163 (GERM #4600),
  DR-164 (CONTACTHOLD #4575) and DR-165 (INTENT #4618).

**Numbers never used on main:** DR-154, DR-155 and DR-159. `git log -S`
finds no row ever written under them. After PR 4602 only DR-154 stays
unused.

**How this readout counts.**

- **Deduplicated by PR number, not DR number.** The 21 rows name 21
  distinct PRs, and no PR appears twice. That matches main's
  first-parent merge order: 21 unit merges from #4525 to #4618 after
  #4518 (DR-144), so the last is the 165th.
- **Naming.** The FLUX rows are named by PR in this readout: "FLUX
  #4554 (DR-155 after PR 4602)" and "FLUX #4557 (DR-159 after PR 4602)".
  "DR-149" and "DR-150" mean the in-table INTENT #4583 and NURBS #4540
  rows.
- **Merge order vs row numbers.** Most row numbers do not follow main's
  merge order (rule 8). For example, by merge order #4560 is the 151st
  and #4554 the 152nd. No row is counted twice or dropped because of
  this, since counting is by PR.

**A row truncated on main.** DR-150 (NURBS #4540, a holdout) ends
mid-cell at `1c4f9960d9`, at "BILATERAL 11:". Its row was written over
several lines (`24e28f5bdf`, 21 lines). The merge of PR 4560
(`1f0d8d040e`) kept the first line and dropped the other 20, which hold
the rest of the correspondence cell and the whole tally, fair and
fix-pass cells. This readout reads that row's lost cells at
`24e28f5bdf`. There they read: tally "0 (no MAJOR on either side).
Nearest miss: R1's `chart_stretch_inf`… fails (a) as MINOR"; fair
"identical briefs, dispatched together … no glimpse"; and a one-pass
fix by the implementer.

## 1. The population since DR-144

21 dual-tier units, 2026-10-10 to 2026-10-11:

- **12 M-letter units under rule 1's arms:** 4 HOLDOUT, 8 SEQUENTIAL.
- **8 H-letter concurrent pairs.**
- **No L-letter and no S-letter units.**
- **No unit outside the arms.** No M unit ran without a draw.

The M-tier units, with every byte checked against its arm:

| # | row | program, PR | class | byte (mod 3) | arm |
|---|---|---|---|---|---|
| 1 | DR-145 | SHELL #4525 | M / SOUNDNESS | 37 (1) | SEQUENTIAL |
| 2 | DR-147 | GERM #4530 | M / TRICKY | 222 (0) | HOLDOUT |
| 3 | DR-149 | INTENT #4583 | M / SOUNDNESS | 91 (1) | SEQUENTIAL |
| 4 | DR-150 | NURBS #4540 | M / SOUNDNESS | 18 (0) | HOLDOUT |
| 5 | DR-151 | INTENT #4589 | M / SOUNDNESS | 176 (2) | SEQUENTIAL |
| 6 | DR-156 | INTENT #4596 | M / SOUNDNESS | 149 (2) | SEQUENTIAL |
| 7 | DR-157 | GERM #4592 | M / NUMERIC | 186 (0) | HOLDOUT |
| 8 | DR-158 | NURBS #4553 | M / SOUNDNESS | 182 (2) | SEQUENTIAL |
| 9 | DR-161 | INTENT #4585 | M / SOUNDNESS | 157 (1) | SEQUENTIAL |
| 10 | DR-162 | INTENT #4608 | M / STRUCTURAL | 197 (2) | SEQUENTIAL |
| 11 | DR-163 | GERM #4600 | M / NUMERIC | 31 (1) | SEQUENTIAL |
| 12 | DR-165 | INTENT #4618 | M / STRUCTURAL | 138 (0) | HOLDOUT |

**The rows hold 12 arm units. The foot counts 11.** The foot's
"toward the next full readout" list leaves out DR-161 (INTENT #4585),
whose class cell carries an M letter, byte 157 and SEQUENTIAL.

**Mix.**

- Holdout share 4/12 = 0.333 (0.099–0.651). The draw's own rate is
  86/256 = 0.336.
- By program: INTENT 6 (DR-149, 151, 156, 161, 162, 165), GERM 3
  (DR-147, 157, 163), NURBS 2 (DR-150, 158) and SHELL 1 (DR-145).
- Five of the six INTENT units are the `from_f64` audit (U1 is H; U2,
  U3, U4 and the generic-crates unit are M) or its sequel (#4608).

**Cumulative, readouts 4 and 5:** 35 arm units, 9 HOLDOUT and 26
SEQUENTIAL. The holdout share is 9/35 = 0.257 (0.125–0.433). DR-101 (M,
no draw) stays outside the arms, as before.

## 2. What the M-tier arms show

Every arm unit is letter M. No S-letter unit has been recorded, so the
per-letter tables below have one populated row. Task class is given as
a secondary split.

### 2.1 Holdout pairs since DR-144 (4)

**DR-147** (GERM #4530, M / TRICKY, head `102aa89a7`). The first miss
(section 2.3).

- R1 (AWF, 0/3/1): no MAJOR, no wrong answer. It used an independent 3D
  tracer over 2,803 poses and 9,000 end-to-end runs. It ran the slow
  search at the default ε only.
- R2 (AWF, 2/0/2): the new slow-set rows fail at the nightly's ε rows.
  This was coded as one defect.
- 1 tallied finding (R2's).

**DR-150** (NURBS #4540, M / SOUNDNESS, head `5ec41ae290`). No MAJOR on
either side.

- R1 (AWF, 0/4/3) and R2 (AWF, 0/3/4). Each executed thousands of
  stalls, and neither found an unsound reading.
- Blinded coding by a separate coder, byte 201: 11 bilateral groups,
  R1→ 6, R2→ 8.
- Tally 0. Nearest miss: R1's `chart_stretch_inf`, unilateral, code,
  executed, but a MINOR. The fix pass ran it: "every wall at a ≥ 0.5
  refuses `ArmUnbounded`", which is a loud refusal. It was filed on
  PCERT.

**DR-157** (GERM #4592, M / NUMERIC, head `c801c98a4`). No MAJOR on
either side.

- R1 (AWF, 0/1/2) and R2 (AWF, 0/2/1). Both found 0 wrong bodies
  against independent mpmath oracles (60k and 15k poses).
- Bilateral MINOR, both executed: the boolean consumers are reached and
  right bodies are lost to the apex-read bound. The fix pass re-banded
  the filed item P3 → P1.
- R2→ MINOR: the both-nappe cost (executed).
- Coded by the orchestrator, byte 29. Fair, flagged: R2 saw the filename
  `brief-a.md` in a shared scratch directory and did not open it.

**DR-165** (INTENT #4618, M / STRUCTURAL, head `aee8cbea05`). The second
miss (section 2.3).

- R1 (AWF, 0/3/3): no MAJOR.
- R2 (AWF, 1/5/3): M1, a measure across one split's two halves resolves
  both against one half.
- 1 tallied finding (R2's), confirmed by a blinded coding.

**Shape of the period's holdouts:** two pairs with a MAJOR, each a
miss; two pairs with no MAJOR on either side. No pair in the period
raised a MAJOR from both reviews, and none split a MAJOR at a severity
(the DR-48 shape).

### 2.2 Sequential units since DR-144 (8)

**The first review raised a MAJOR in 3 of 8 units, and a second review
ran in exactly those 3:** DR-156, DR-158 and DR-162. The other 5
(DR-145, DR-149, DR-151, DR-161 and DR-163) had one review. No unit
skipped a second review it was owed, and none ran one it was not owed.

What each second review added:

- **DR-156** (INTENT #4596, M / SOUNDNESS).
  - First (`4b7726ea0`): NMAI, 3/4/2.
    - M1, executed: parting at a sign flip cut plate_param's row by 7603
      decisions.
    - M2, by inspection: the mate `Arm::of` classed `constant`.
    - M3, by inspection: the pinned profile lift classed `constant`.
  - Second (`4fce98dcc`, whole unit): AWF, 0/4/3. M1–M3 confirmed fixed
    (M1 executed). **No new MAJOR.** New MINORs:
    - no pin for a decision that flips itself (executed: a mutant stays
      green);
    - the P0 left open with its premise unretracted;
    - the unit's item not closed;
    - die's census covers a prefix only.
- **DR-158** (NURBS #4553, M / SOUNDNESS).
  - First (`e32d58c62d`): NMAI, 1/5/2. The MAJOR (executed): dropping
    `localized` lost `Interval` verdicts. At ε 1e-12, d3 went from 15 to
    6 certified rulings.
  - Second (`ac09740216`, whole unit): AWF, 0/3/2. The MAJOR confirmed
    fixed by execution: 8 families × 19 rulings × both scalars × three
    ε, and 170,601 exact-rational containments. **No new MAJOR.** New
    MINORs:
    - the `compile_fail` rows also emitted E0599;
    - retiring a probe left an escalation payload unguarded (this
      concerns the fix pass's change);
    - a stale doc.
- **DR-162** (INTENT #4608, M / STRUCTURAL).
  - First (`c7533a2d6`): NMAI, 1/3. M1 (executed, `cargo check
    --workspace`): the new `SplineError` arm broke an exhaustive match
    in geom-brep, so the workspace did not build. It also raised an
    executed MINOR: the knot gaps were still taken in `f64`, so the
    re-pin's soundness claim was false.
  - Second (`f7843f26e`, whole unit): AWF, 0/4. M1, M2 and M4 confirmed
    fixed (an exact-rational oracle over 7,200 entries, 0 misses); M3
    half-fixed. **No new MAJOR.** Four new MINORs. The row says "two on
    code the fix pass added":
    - `map`'s check was debug-only;
    - a multiplicity-0 run was accepted and dropped;
    - pin text in the present tense;
    - `HeldArc` with `m = 0` panics.

**Single-review units.** For each, the closest the review came to a
blocking defect, as recorded:

- **DR-145** (SHELL #4525), 0/1/2. A MINOR (executed): a mixed-sign net
  answers `holds_the_move` true while its moved row lies 0.1 off the
  plane. "Reach through the door unsure (a near-singular chart)". The
  fix pass took it, and the row is red without the check.
- **DR-149** (INTENT #4583), 0/1/2. Every one of 14 site reverts reds a
  pin. The MINOR is a misplaced doc.
- **DR-151** (INTENT #4589), 0/3/2. A MINOR (executed): both re-value
  pins pass vacuously on a carrier built outside the session. The fix
  pass added positive controls.
- **DR-161** (INTENT #4585), 0/4. A MINOR (executed): the cell pin
  reached 3 of 8 sites, and 5 single-site reverts stayed green.
- **DR-163** (GERM #4600), 0/3/2. 18,900 poses against a 60-digit
  oracle with 0 wrong answers. Style: four mutants pass the new rows,
  and one (Mwidth) is killed by no row (filed).

No later escape is recorded against any of the 5 (section 3).

### 2.3 The two M-tier misses

**DR-147** (GERM #4530, 2026-10-10). The section certificate answers
torus × a cylinder square to the torus axis.

- **Arm:** HOLDOUT, byte 222. **Class:** M / TRICKY. **Head:**
  `102aa89a7`.
- **The finding.** R2's MAJOR: the PR's new slow-set rows fail at the
  nightly's ε rows.
  - The square-wall search fails at ε 1e-12 on 4 of 4 seeds, because
    its "in-band" tilts (1e-18..1e-13) fall outside the band at that ε.
  - It fails at 1e-6 on one seed: an R-tan window judged on the
    family's own margin only.
  - `germ_tilted_rod`'s new row fails at 1e-6: a fixed 1e-9 volume
    tolerance against a 4e-9 error.
  - Each failure was shown by a red run at a named seed.
  - The defects are in test code. Neither review found a wrong answer
    in the arm. The per-PR CI was green because its ε rows skip the slow
    set.
- **The other review.** R1 raised no MAJOR. It "ran the slow search at
  the default ε only" and never mentions the ε rows.
- **Coding.** "Coded by the orchestrator, byte 115 (odd: A = R2)". The
  three failures were counted as one defect under 6(c), "a close call
  since the mechanisms differ". The short readout calls the coding
  blinded. The row names the orchestrator as coder (section 6).
- **Fair:** yes. Identical briefs, dispatched together as local
  subagents, no glimpse, no interruption.
- **What became of it.** A fresh lane took the union, swept every
  numeric literal in the diff, and made the tolerances ε-aware. Every
  touched row is green at three ε.

**DR-165** (INTENT #4618, 2026-10-11). A split's half is its port, and
`Part { SplitHalf }` retires.

- **Arm:** HOLDOUT, byte 138. **Class:** M / STRUCTURAL. **Head:**
  `aee8cbea05`. Taken over from a blocked lane and merged over DM4
  unit 1.
- **The finding.** R2's M1, executed: a measure whose two references
  read the two halves of one split resolved both against one half.
  `reads_projected` is keyed by the split, so the later read overwrites
  the earlier.
  - Red at the head in both orders.
  - Green at the base through two `Part`s. The row: "retiring `Part {
    SplitHalf }` removed the only spelling that worked".
- **The other review.** R1 raised no MAJOR. It raised three MINORs,
  including one executed code defect: a revolve's viewport pick seated
  as a half. The coder noted that "R1 listed claim 2 as sound by
  inspection: affirming a claim is not raising its defect".
- **Coding.** The implementer's unblinded pre-note was confirmed by a
  blinded coding: a separate coder session, byte 34, reports stripped of
  labels, probes and branch names, kept off the PR body and the row.
  6(a)–(e) all pass.
- **Fair:** yes. Identical briefs, separate worktrees, no glimpse.
- **What became of it.** The implementer took the union. A measure's
  references are now projected read by read, and a node reading two
  bodies of one operation into one map fails loud. R2's probe is kept
  as a row that is red without the fix.

**What the two share, as recorded.**

- In both, the review that raised no MAJOR did not exercise the region
  that held the defect. DR-147's R1 ran the default ε only. DR-165's R1
  affirmed the relevant claim by inspection.
- Both MAJORs are R2's. These are dispatch labels only. DR-104's MAJOR
  was R1's.

**A related observation, not a finding.** Readout 4 (section 3) traced
an overlay "keyed by node id" that "served `Union[split.0, split.1]` one
half twice" to DR-119 (unit B). DR-123 fixed it for booleans and
unions. DR-165's M1 is a projection keyed by the split that serves one
half for both reads of a measure. The rows do not say whether the two
share a cause, or whether a measure reading both halves of a split by
port was already wrong before #4618.

### 2.4 Counts and rates per difficulty letter (exact 95% intervals)

**Letter M** (the only letter populated). S: 0 units in either window.

| measure | since DR-144 (12 units) | cumulative arms (35 units) |
|---|---|---|
| holdout pairs | 4 | 9 |
| holdout pairs with any MAJOR | 2/4 = 0.500 (0.068–0.932) | 5/9 = 0.556 (0.212–0.863) |
| holdout pairs with a tallied finding | 2/4 (DR-147, DR-165) | 4/9 = 0.444 (0.137–0.788): DR-104, DR-137, DR-147, DR-165 |
| M-tier misses per holdout pair | 2/4 = 0.500 (0.068–0.932) | 3/9 = 0.333 (0.075–0.701) |
| holdout reviews with no MAJOR whose partner raised a tallied MAJOR | 2/6 = 0.333 (0.043–0.777) | 3/12 = 0.250 (0.055–0.572) |
| sequential units | 8 | 26 |
| sequential first reviews raising a MAJOR | 3/8 = 0.375 (0.085–0.755) | 7/26 = 0.269 (0.116–0.478) |
| second reviews raising a new MAJOR | 0/3 (0.000–0.708) | 2/7 = 0.286 (0.037–0.710): DR-125, DR-131 |
| initial reviews raising a MAJOR (holdout reviews + sequential firsts) | 5/16 = 0.312 (0.110–0.587) | 13/44 = 0.295 (0.168–0.452) |

The fifth line is the closest the arms come to the sequential arm's
blind spot: a first review with no MAJOR, so no second review. Among
holdout reviews with no MAJOR, it counts how often the concurrent other
review raised a tallied one. The no-MAJOR reviews are:

- since DR-144: DR-147 R1, DR-150 ×2, DR-157 ×2, DR-165 R1;
- before: DR-48 R1, DR-104 R2, DR-122 ×2, DR-143 ×2.

DR-48 R1's partner raised a MAJOR that was coded bilateral at a
severity split. Counting it gives 4/12.

**By task class** (all letter M):

| class | holdout (MAJOR / miss) | sequential (first MAJOR → second ran / new MAJOR) |
|---|---|---|
| SOUNDNESS | DR-150 (0 / 0) | DR-145, 149, 151, 161 (0); DR-156, DR-158 (MAJOR → ran / none) |
| TRICKY | DR-147 (1 / miss) | — |
| NUMERIC | DR-157 (0 / 0) | DR-163 (0) |
| STRUCTURAL | DR-165 (1 / miss) | DR-162 (MAJOR → ran / none) |

**Reviews run in the arms since DR-144: 19.** That is 8 holdout, 8
sequential first and 3 second, for 1.58 per unit. Cumulative: 32 + 19 =
51 over 35 units = 1.46.

### 2.5 What the sample allows

- **The DR-104 shape is now shown three times in nine holdouts.** Each
  time, a complete, fair, unrelaxed review raised no MAJOR while its
  concurrent twin raised a demonstrated, tallied one. Taken first, the
  sequential arm would have shipped each one. The per-holdout rate is
  3/9, bounded to 0.075–0.701.
- **Of the three defects:**
  - DR-104: a code defect, a build-path wrong classification;
  - DR-147: test code that would have turned the nightly red at two ε
    rows, with no wrong answer in the kernel;
  - DR-165: a code and contract-API defect, a wrong measured value.
- **The conditional second review fired three times in this period and
  raised no new MAJOR.** Each confirmed the first review's MAJORs fixed
  and added MINORs. At least three of those MINORs concern code a fix
  pass wrote (DR-158's unguarded payload; two in DR-162). Across the
  seven second reviews to date, 2 raised a new MAJOR (DR-125 spec text,
  DR-131 code). Neither is in this period.
- **What the arms cannot see.** Five sequential units in this period
  (and 19 of 26 cumulative) shipped on one review with no MAJOR. Only
  holdouts speak to them, and holdout reviews with no MAJOR had a
  partner with a tallied MAJOR 3 times in 12.
- **Confounds.**
  - Program, task class and orchestrator still vary with arm. Both
    misses in the period come from units built on hard-to-reach regions
    (non-default ε rows; a multi-read measure).
  - The sequential units are concentrated in one INTENT audit series.
  - The rows cannot separate program from arm.

## 3. Escapes (rule 11)

**Recorded later-escape lines.** Still two, DR-11 and DR-54, both
covered by earlier readouts. **None was added since DR-144**, and none
names an arm unit.

**Traceable from the repository, not recorded as a rule-11 line.**

**DR-144** (NURBS #4518, M / NUMERIC, SEQUENTIAL, a single review with
no MAJOR) is readout 4's trigger unit.

- **What was found.** After it merged (10:27 local, 2026-10-10), PR
  4580 (merged 13:20) re-pinned `crates/viewer/tests/review_pick_r2.rs`,
  "which #4518 moved". The pinned tally changed from `(441198, 141249,
  14088, 7230)` to `(441198, 141261, 14088, 7230)`: "12 more rays are
  answered at the aimed vertex … No genuine crossing is refused either
  way".
- **Who saw it.** DR-160's and DR-152's fix-pass cells record it as one
  of "two slow-set rows red on main, each bisected and fixed in its own
  PR".
- **On the base.** The test file was on #4518's merge base (`effba4bf`).
  This is not a crossed-merge pin like readout 4's DR-118 case.
- **What DR-144's row says.** The review found "with the vertex change
  reverted every fence reads main's digest". The fix pass (`fd02e1a594`)
  then moved far-cap and half-turn vertices further. The row ends
  "nothing in the corpus moved". It does not say whether the viewer
  tally moved at the reviewed head or only after the fix pass.
- **What it is.** A stale pinned count in a slow-set row, red on main,
  with no wrong answer. No review of DR-144 raised it.
- **Whether it is an M-tier miss** depends on one reading. Rule 9's
  second branch needs an escape traced to a sequential unit that meets
  6(b) and 6(d) and that no review raised as MAJOR.
  - DR-137's `SAID_DIGEST` not re-pinned (red in the slow set) was
    raised as a MAJOR and listed as a tally candidate.
  - DR-147's miss is red test rows that only the nightly would have
    shown.
  - If a moved golden is a test-class defect under 6(b), demonstrated by
    the red row under 6(d), this is a third M-tier miss since DR-144.
  - If a pin that an improvement moved is not a defect, it is not an
    escape at all.
  - The protocol does not decide this. It does not move this readout's
    trigger.

**DR-119's overlay** (readout 4) is still not recorded as a line under
DR-119.

**Examined and not escapes.**

- `work/germ/pn-apex-line-outcomes-stand-in-for-a-section-off-the-apex`
  (P2, fixed by DR-157's unit). It was found by a sweep owed by DR-136's
  unit. Its `ApexLinePair` lane predates the checkout's graft
  (2026-10-09), and so predates DR-136's unit.
- `work/intent/the-plate-verdict-pin-holds-only-at-the-default-eps`
  (found in DR-145's lane): "identically on merge base". Pre-existing.
- `work/tint/silent-fixed-predicates-probe-reds-under-all-features-on-main`
  (found in DR-158's lane): "Not this PR's".
- `work/cleave/main-reds-on-a-recharted-residual-in-the-band-since-4609`
  (found in DR-162's lane): PR 4609, which has no row in this log.
- Main's red `assemble_one_local_battery` (in DR-165's fix-pass cell)
  is not attributed to any unit in the rows.

**Defects found after review inside an arm unit's own cycle.** These are
not escapes, but they bear on what one review sees. DR-158's and
DR-162's second reviews found MINORs in code their first fix passes
wrote.

## 4. The concurrent tier (H/L) since DR-144

Holdouts are excluded, as the Seams require.

**Population: 8 H-letter pairs, no L.** DR-146 (INTENT #4543), DR-148
(SHELL #4524), DR-152 (INTENT #4560), DR-153 (INTENT #4527), FLUX #4554
(DR-155 after PR 4602), FLUX #4557 (DR-159 after PR 4602), DR-160
(INTENT #4566) and DR-164 (CONTACTHOLD #4575).

- Every fair cell reads yes. DR-164 is flagged: R1 glimpsed R2's command
  line via `pgrep` (method only), and R2's census was not run. Counted.
- No mixed-model pair and no truncation is recorded. **8 fair pairs.**

**Fair pairs that found any MAJOR: 7.** All but DR-164.

| pair | R1 | R2 | MAJOR coding | tally |
|---|---|---|---|---|
| DR-146 | 1/7/0 | 1/5/0 | block arguments exempt: MAJ/MIN, bilateral; R2's `Margin::lift` by inspection | 0 (6(a); 6(d)) |
| DR-148 | 1/3/2 | 1/2/1 | G1-corner NaN escalation, MAJ/MAJ | 0 |
| DR-152 | 1/5/4 | 1/5/7 | the read door's world-copy selection, MAJ/MAJ | 0 |
| DR-153 | 1/3/4 | 1/3/6 | `Subtract` over one family refuses, MAJ/MAJ | 0 |
| FLUX #4554 | 1/3/4 | 1/4/2 | endpoint census CI red, MAJ/MAJ | 0 |
| FLUX #4557 | 0/2/3 | 1/2/2 | R2's census MAJOR unilateral (red CI row); R1 "read CI while the run was in progress" | **candidate, undecided** |
| DR-160 | 2/5/4 | 3/7/7 | two distinct split-crossing MAJORs (one each, executed); door/persist MAJ/MAJ; `Unevaluated` MIN/MAJ | **2** |

**Tallied findings: 2, in 1 pair (DR-160).** Both are split-crossing
defects:

- R1: through a placement of another body, a measure 5.0 → 0.0;
- R2: through a posed cut placement, 1.0 → 9.0.

"Each restores a different guard main held." By position, R1 1 and R2 1.

**FLUX #4557's candidate is unresolved in the row.** The tally cell
reads "flag for the blinded coding to decide whether the pair is fair
on that item". No blinded coding is recorded, and the foot does not
count it. Counted, it makes 3 findings in 2 pairs.

**Rates:**

| measure | since DR-144 (8 fair) | cumulative concurrent (123 fair) |
|---|---|---|
| pairs that found any MAJOR | 7/8 = 0.875 (0.473–0.997) | 64/123 = 0.520 (0.428–0.611) |
| pairs with a tallied finding | 1/8 = 0.125 (0.003–0.527); 2/8 with FLUX #4557 | 26/123 = 0.211 (0.143–0.294); 27/123 with FLUX #4557 |
| tallied, given the pair found a MAJOR | 1/7 = 0.143 (0.004–0.579) | 26/64 = 0.406 (0.285–0.536) |
| tallied findings | 2 (3) | 35 (36) |

The cumulative column is readout 4's 115 fair pairs (57 with a MAJOR,
25 tallied, 33 findings) plus these 8.

**What the period shows.** Six of the seven MAJOR pairs carry a
MAJOR/MAJOR or MAJOR/MINOR bilateral. In this window, the H tier's
MAJORs were mostly found by both reviews.

## 5. Costs

**Arm reviews:** 19 over 12 units (section 2.4).

- Holdout: 2 per unit.
- Sequential: 11 over 8 units = 1.375.

**Fix passes on arm units: 15 over 12 units.** Every sequential unit
with a second review took two passes (DR-156, DR-158, DR-162). Every
other arm unit took one. Executors:

- a fresh lane on DR-147 and DR-163;
- the build lane on DR-158;
- the implementer elsewhere.

**Extra lanes outside the arms' count.**

- Arm units: "orchestrator checked the delta" on DR-145 and DR-158.
- Concurrent units:
  - independent verifiers on FLUX #4554 and #4557;
  - R1's verification of DR-164's fix pass;
  - DR-146's whole-corpus run by the orchestrator.

**Tokens and wall clock.** Recorded for few rows, in mixed units.

| row | arm | recorded |
|---|---|---|
| DR-162 | sequential | first 144,340 tokens, 56 min; second 166,822 tokens, 60 min |
| DR-165 | holdout | about 254k tokens, 26 min; about 249k tokens, 47 min |
| DR-153 | concurrent | about 290k tokens, ~60 min; tokens not instrumented, ~90 min |
| DR-160 | concurrent | about 190k, 1h45m; about 300k, 2h15m |
| DR-146, DR-152 | concurrent | "not recorded" |
| every other row since DR-144 | — | nothing |

On the rows that record tokens, one review runs 140k to 300k tokens.
That is consistent with readout 4's 120k–300k.

## 6. Instrument seams and data quality

**The log's shape.**

- **Seven rows sit outside the table** (section 0). DR-165, the
  triggering row, is one of them.
- **DR-150's (NURBS #4540) row was truncated** by the merge of PR 4560,
  losing its tally, fair and fix-pass cells on main. No other row in the
  period is shorter than its first recording. Each in-table row since
  DR-144 was checked to end with a closing cell.
- **Duplicate numbers:** DR-149 and DR-150 (pending PR 4602). Unused:
  DR-154, DR-155 and DR-159. Most row numbers in the period do not
  follow merge order (rule 8 says they are assigned at merge in main's
  merge order).

**Foot versus rows.** The foot reads Tally 40, MAJOR pairs 66, toward
the next 11, misses DR-147 and DR-165.

- **M-tier count.** The foot gives 11 and the rows give 12; DR-161 is
  missing. The foot's line for readout 4's window still reads 20, where
  the rows give 23 (readout 4: DR-123, DR-129 and DR-132).
- **Tally.** The foot's 40 is the 36 at readout 4 plus DR-147 (1),
  DR-160 (2) and DR-165 (1).
  - It folds in holdout tallies: DR-137's 3, DR-147's 1 and DR-165's 1.
    It still keeps DR-104's 1 apart.
  - By the rows the concurrent tally is 35, plus 1 if FLUX #4557's
    candidate stands. Holdout tallied findings total 6 (DR-104 1, DR-137
    3, DR-147 1, DR-165 1).
  - Running figures in the period are not monotone in row order: DR-153
    reads 36 after DR-147's and DR-152's 37. The pairs FLUX #4554 and
    #4557, DR-148, DR-150 and DR-164 give no running figure.
- **Fair pairs that found any MAJOR.** The foot gives 66 and the rows
  give 64 concurrent.
  - The foot adds DR-146, 147, 148, 152, 153, 160 and 165 to its 59.
  - It includes the holdouts DR-147 and DR-165, as it did DR-48 and
    DR-137.
  - It omits FLUX #4554 and #4557, both of which found a MAJOR, and the
    holdout DR-104.
  - It still carries DR-57 and omits DR-66 (readouts 3 and 4).
  - 66 − (DR-48, DR-137, DR-147, DR-165) − DR-57 + DR-66 + FLUX #4554 +
    FLUX #4557 = 64.

**Coding weaker than rule 7.**

- **Holdouts:**
  - DR-147, the first miss, was "Coded by the orchestrator, byte 115".
    The short readout calls this coding blinded, but the row records no
    separate coder.
  - DR-157 was coded by the orchestrator.
  - DR-150 and DR-165 were coded by a separate coder, blind.
- **Concurrent:**
  - Orchestrator's unblinded pre-note, blinded coding not run: DR-146,
    DR-152 and DR-160. DR-160's two tallied findings rest on this
    pre-note.
  - No coder or byte recorded: FLUX #4554, FLUX #4557 and DR-164.
    FLUX #4557's candidate is explicitly left for a blinded coding that
    is not recorded.
  - Separate blinded coder: DR-148 (byte 0) and DR-153 (byte 80).

**Fairness flags and disclosures** (inside the fair set):

- DR-157: R2 saw a brief's filename.
- DR-164: R1 saw R2's command line via `pgrep`.
- FLUX #4557: R2 saw the title of a newer commit. R1 checked CI before
  its census result existed. The row asks whether that makes the item
  unfair.
- DR-153: "Coverage, not method": R2 ran neither the viewer, the Python
  suite nor `ty`.
- DR-148 records that its brief forbade system-wide process listings
  "so the DR-138 leak did not recur".

**Protocol hashes.**

- Rows record `da5dbd1532`, `5dedb0a41a` (DR-150, DR-158), `4b5880058a`
  (DR-149, DR-156, DR-161) and `09b01acd8` (DR-146).
- `4b5880058a` is a STATUS render commit, not one that touched the
  protocol. Its protocol text equals main's at `1c4f9960d9`.
- `09b01acd8`, `da5dbd1532` and `5dedb0a41a` do not resolve in this
  checkout, which is grafted at 2026-10-09.
- No commit on main after the graft touches the protocol, so every row
  since DR-144 ran under one protocol text.

**Model era.** No model change is recorded in the protocol's Seams, and
every review in the period is recorded as the same model or is
unlabelled. One era.

**Draw timing.** No row in the period says when its byte was drawn. No
late draw and no missing draw is recorded.

## 7. The options for Ev

The arms continue until Ev rules (rule 9). The options are not ranked,
and none is recommended.

Costs are in reviews per M-letter dual unit. The observed figures are
1.58 since DR-144 (1.46 cumulative), a holdout share of 0.257
cumulative, and a sequential first-review MAJOR rate of 7/26 cumulative
(3/8 in the period). The draw's holdout rate is 86/256 = 0.336.

- **Continue the arms unchanged.** The next full readout comes at
  twenty more M-tier units or two more M-tier misses after DR-165.
  - **Cost:** 2 × 0.336 + 0.664 × (1 + p) per unit. That is **1.51** at
    p = 7/26 and 1.58 at p = 3/8.
  - **Measures:** about 7 more holdouts per twenty units, and more
    second reviews.
  - **Risks:** at the cumulative rate (3 misses in 9 holdouts), a
    sequential unit whose first review raises no MAJOR carries a
    demonstrated, blocking defect that a second review would have
    raised in about a quarter to a third of cases (0.25 of no-MAJOR
    holdout reviews; interval 0.055–0.572). Such a defect ships unless
    it surfaces later as an escape. Misses at this rate would trigger
    readouts often.
- **Change the arm split.** For example, byte mod 2 = 0 for a 0.5
  holdout share, or all holdout for a fixed window.
  - **Cost:** about 1.63 per unit at 0.5 with p = 7/26; 2 per unit all
    holdout.
  - **Measures:** the miss-per-holdout rate narrows faster. Second-review
    data accrues more slowly.
  - **Risks:** fewer sequential units and second reviews. There are
    seven second reviews so far.
- **Make the sequential second review unconditional.** Every sequential
  unit gets a second review on the fix-pass head, MAJOR or not.
  - **Cost:** 2 reviews per sequential unit, plus a second fix pass where
    the second review finds anything.
  - **Measures:** what a whole-unit review on a fixed head adds when the
    first raised no MAJOR. Today only holdouts speak to that.
  - **Risks:** the conditional trigger is no longer measured. Wall clock
    is longer than a concurrent pair at the same review count. The
    period's three second reviews raised only MINORs, some in code the
    fix pass wrote.
- **Return M units to concurrent pairs.** The arms end, and M pairs join
  the tally and the pair count.
  - **Cost:** 2 per unit.
  - **Measures:** the unilateral-MAJOR rate at M directly, comparable
    with the 9 holdouts and the 10 pre-arm M pairs.
  - **Risks:** gives up the sequential arm's data on a reviewer who sees
    the first review's adjudicated findings and a fixed head.
- **Make the second review conditional on something other than a
  MAJOR.** For example, fire it on any executed MINOR or on a
  coverage-limited claim.
  - Both misses in the period sat where the no-MAJOR review had not
    exercised a region: the default ε only, or a claim affirmed by
    inspection.
  - **Cost:** between 1 and 2 per unit, depending on how often the new
    trigger fires. The rows do not give that rate.
  - **Measures:** whether the new trigger catches the DR-104/147/165
    shape. Holdouts would still be needed to see what it misses.
  - **Risks:** a new trigger needs a definition the arms can apply
    mechanically, and the rows record coverage limits unevenly.
- **End the sequential arm.** M units take a single review with no
  conditional second, and leave this log.
  - **Cost:** 1 per unit.
  - **Measures:** nothing in this log. Only later escapes would speak.
  - **Risks:** none of the period's MAJORs came from a second review,
    but a single-review regime would also have shipped the three holdout
    misses' shape whenever the no-MAJOR review was the one run.
- **Stop the experiment.**
  - **Cost:** none further.
  - **Measures:** none further. The record stands at:
    - 35 arm units: 9 holdouts, 26 sequential, 7 second reviews, 3
      M-tier misses;
    - 123 fair concurrent pairs and 35 or 36 concurrent tallied findings
      by the rows (40 by the foot).
  - **Risks:** review tiering then rests on the record as it stands.

Further options the rows suggest. Each can go alongside any of the
above.

- **Rule on DR-144's re-pinned viewer tally** (section 3). Say whether a
  pinned count that a unit moved and left red on main is a rule-11
  escape under 6(b) and (d), and so an M-tier miss when no review raised
  it.
  - **Cost:** a sentence, and a line under DR-144 if it counts.
  - **Measures:** whether escapes of this kind, which only the
    sequential arm's single-review units can produce, enter the count.
- **Restore DR-150's (NURBS #4540) lost cells and move the seven
  displaced rows into the table.** PR 4602 does part of the second.
  - **Cost:** one docs change.
  - **Risks:** none to the method. Without it, the row that triggered
    this readout and six others do not render as rows.
- **Make the foot follow from the rows.** For example, a lint or
  `scripts/` check that counts M-letter arm rows, MAJOR pairs and tally
  cells, and refuses a row that does not close its last cell.
  - **Cost:** a small script.
  - **Risks:** none to the method. In this period it would have caught
    DR-161's omission, the FLUX pairs' omission from the MAJOR count,
    and the truncation.
- **Say how holdout tallies and holdout MAJOR-pairs enter the foot.**
  DR-104 is kept apart, while DR-48, DR-137, DR-147 and DR-165 are
  folded in.
  - **Cost:** a sentence and a one-time re-count.
- **Settle FLUX #4557's candidate.** Run the blinded coding its row
  asks for, or record that a review's check made before the CI result
  existed is or is not a method difference under rule 4.
  - **Cost:** one coder lane.
- **Require blinded coding by a separate coder before a finding is
  counted.**
  - In this period, these rest on weaker coding: DR-160's 2 tallied
    findings (unblinded pre-note) and DR-147's miss (coded by the
    orchestrator).
  - **Cost:** one coder lane per pair that raised a MAJOR.
- **Name the "first" review in each holdout pair** by a second recorded
  byte, raised in readouts 3 and 4 and not adopted.
  - **Cost:** one byte per holdout.
  - **Measures:** rule 9's counterfactual for one review rather than
    "the other review".
- **Record the draw time and the cost of every arm review in one unit.**
  10 of the 12 arm units since DR-144 record no review cost.
- **Have the orchestrator of the fixing unit append the rule-11 line**
  whenever a fix names a merged unit, as PR 4580 named #4518.
  - **Cost:** a line per traced defect.
  - **Measures:** escapes, the only signal the single-review units can
    give.
