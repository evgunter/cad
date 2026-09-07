# METER log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/meter/plan.md`.

## Opened (2026-09-06)

Opened in the tracker-wide cut of 2026-09-06 (Ev's direction,
in-chat; `docs/WORK-TRACKS-2026-09.md` addendum 2), with GATES, the
two halves of code-quality Track K claimed whole. Eleven rows moved
by `git mv` with a `## Claimed by` record each: eight with `track: K`
and three unlettered rows on K's `tools/*` fence. `S115`(a) (the inert
`agree` column) and `S29` (the sizing policy) stay in
`work/code-quality/` — the first as a member of a roll-up this
program will take as a rider on unit 1 if the roll-up is not split
first, the second as S-MESH's design PR. No branch exists yet; the
first dispatch is unit 1.

## Orchestrator seated; review posture set; slate re-read (2026-09-07)

Ev seated an orchestrator on this program in chat and set its review
posture in the same breath: **no A/B protocol** — most units are easy —
**a style review each, and a full falsification review only for the
units that are genuinely tricky.** `plan.md`'s *Review posture* now
carries that, with the two units that qualify named and the reason each
qualifies stated: `D206` and `k-report-baseline-fold-cert1-roster`, both
because they move committed measurements other lanes read. The A/B band
3200–3299 stays claimed for bookkeeping and no row will be written to
`docs/MODEL-AB-LOG.md` from this program.

The slate was read end to end against the tree before the first
dispatch, and three things came back that the tracker did not know.

**The join item's fix has landed and the item does not know it.**
`tools/tess-lint/src/lib.rs`'s `Kind::Rekeyed` / `Rekey` under
`compare`'s rule-4 precondition closes both branches
`tess-lint-face-ordinal-join` names. `plan.md`'s step 8 still described
that tripwire as unlanded, under the name `Kind::Reordered` it was
proposed with; the sentence is corrected and the item becomes **unit
0**, because reading it as open mis-states what `C15` has left to do.
The unit is not pure bookkeeping: `compare`'s `gated` split routes a
re-key on a scene with no Hessian-sized face to `notes` rather than
`findings`, which is #738's `diefillet` exactly, so the permutation the
item was written around is reported in the lint's quietest voice and
nobody has recorded whether that is right.

**The stale-doc item's own table has gone stale.** Re-derived against
the baseline as committed today (cut `aba2625f8f84`, 2026-09-04, later
than the `a4eb03a` the item read): 1353 rows, 1,552,822 triangles, the
64 NURBS faces at 4.7% carrying 10.6%. The item said 1306 / 1,416,410 /
4.9% / 11.6%. That is the item's own mechanism firing on the item, four
days after it was filed, and it settles both open questions in the
file — the block is stale against re-cuts rather than describing an
older sweep, and the fix is to CITE `tess-lint`'s report header, never
to commit a fourth transcription. Recorded on the item.

**A pin cannot close `k-lint`'s roster in the direction that matters.**
`k-lint-predicate-roster-unpinned` reaches for `D204`'s `include_str!`
remedy, which worked there because `CHART_TAGS` and `Chart::tag` are
the same finite set. `EPS_COUPLED_PREDICATES` is a SUBSET selected from
an open vocabulary by a property that is written down nowhere, so a pin
closes the RENAMED direction and cannot close the ADDED one at all.
Filed as `k-lint-eps-coupled-criterion-unwritten` so the pinning lane
is dispatched knowing it, rather than discovering at review that its
claim is narrower than the item it cites.

Dispatch order from here: unit 0 and unit 1 both sit in
`tools/tess-lint/src/lib.rs` and are sequenced rather than run
together; unit 2 is disjoint (`docs/TESS-BUDGET.md`) and runs beside
whichever of them is live. `D201` goes out as an `[ev]` PR early rather
than at step 8 — it is the slate's one design fork and its long pole,
and the answer wants to be in hand before the cheap end is exhausted.

## Unit 0 landed and adjudicated; two orchestrator errors (2026-09-07)

`tess-lint-face-ordinal-join` is closed on `meter/join-gated-voice`
(PR 2111), style review dispatched. **The decision: an ungated re-key
stays a NOTE**, argued from the disanalogy with rule 5 — an uncovered
scene carries a coverage claim that is asserted and false, while a
scene with no sized face gives rule 2 no claim there to be false, and
`gated` reads both sides so the note CONVERTS to a finding the day the
scene gains a sized face, which rule 5's case never does. The accepted
cost is written at the site and its cure named (`D201`/`C15`, both
live here).

Two loads the lane carried that the plan did not anticipate. **#738's
`diefillet` permutation no longer reproduces** — the lane ran the
sweep rather than inheriting the claim, and got 1353 rows over 72
scenes with every ordinal agreeing on all eight identity columns, 0
findings and 0 notes at exit 0. A later re-cut absorbed it, which is
what the item's own body predicted would happen. So the decision was
made on the merits and not under a live red. And **a third instance of
the transcribed-census class** turned up in the sweep: `lib.rs` claimed
"58 of the committed baseline's 70 scenes" where the truth is 72
scenes, 12 sized and 60 not. It moved to `tests/baseline_census.rs`,
the standing one home. That class has now fired three times in a week
on this program's ground.

### Two things the orchestrator got wrong, recorded because the board should carry them

**The dispatch asserted an unrecorded decision that was recorded.** The
brief for unit 0 said nobody had written down whether an ungated re-key
should be a note or a finding. It was on main at
`tools/tess-lint/src/lib.rs:177`, with the principle on `Report` and
the ungated case in `main.rs`'s recourse item 4. The lane checked the
premise instead of building on it — which is what
`docs/REVIEW-STYLE-DISPATCH.md` §3 asks of a dispatch and what the
brief explicitly invited — and the unit became an EXTENSION of a
recorded decision rather than a first statement of an unrecorded one.
Re-verified before the correction was accepted onto the item.

**The dispatch did not read `memories/agent-lane-operations.md` before
sending lanes out, and that memory already carried the hazard that
bit.** Its *"Where subagents share the orchestrator's checkout (no
per-lane worktree — the remote-session default)"* clause names exactly
what happened: three lanes in `/home/user/cad`, one of them
(`tess-budget-doc-finding-block`) accumulating uncommitted work on
top of `meter/d201-ev-question` — the open `[ev]` PR's branch — because
another lane's `git checkout` had silently re-pointed the shared
directory under it. Caught before any commit landed and the lane moved
to a private worktree. No memory amendment is owed: the text was
already right and the orchestrator had not followed `MEMORY.md`'s
pointer to it. **Every dispatch from here carries the worktree
instruction explicitly.**

Also filed from unit 0's out-of-fence report:
`cut-line-commit-names-no-baseline-change` — the baseline's cut line
stamps the sweeping tree's HEAD, so the commit it names need never have
touched the baseline (verified: `aba2625f8f84` does not). Honest by
`tess_budget_cut.sh`'s third arm and unstated where `tess-lint` READS
and prints it. Adjacent to unit 4's seam and left separate, because the
two want different edits and only one of them is a pin.

## Wave 1 in; three dispatcher readings falsified by the lanes (2026-09-07)

Units 2 and 5 are in review (PRs 2114, 2115) and unit 0 is in a fix
pass off its style review (PR 2111, 19 findings). **All three lanes
falsified something the orchestrator handed them as settled**, which is
the dispatch contract working, and is also a pattern worth naming
rather than filing three times as an isolated slip.

**Unit 2 — the biggest one, and it was published.** The orchestrator
told the lane that `docs/TESS-BUDGET.md`'s finding block had gone stale
against a second re-cut, that this "settles the judgement the item
declined to make", and that only the form of the fix was open. That is
false. **The block is the #547 pre-fix sweep and every figure in it is
correct for the tree it was taken from.** The block has FOUR cell lines
and both the item's table and the orchestrator's re-derivation compared
only three: the fourth, `44,457 with both`, is today's `span_opt_cells`
of **44,446** — 0.02% apart — and `95,090 at the cheapest split` is
`opt_cells` **94,154**, 1.0% apart. The two schedule-INDEPENDENT optima
have not moved; the schedule-DEPENDENT columns moved several-fold,
which is a FIX's signature, not a re-cut's. `docs/MODEL-AB-LOG.md:1578`
corroborates independently, and the document already said so 155 lines
above the block.

The orchestrator's stated evidence was also unsound where it was most
confident: *"stale against two successive re-cuts in the same
direction"* discriminates nothing, because under either hypothesis the
gap grows monotonically with the corpus. It read like evidence and was
not. Retracted on the item beside the original text, because the item's
own `## What` makes the same mis-mapping and a reader deserves to know
why two successive readers reached the wrong answer. **The claim also
went out in PR 2110's body, which is merged**; nothing rewrites that,
so this log entry and the item's retraction are the record.

**Unit 5.** The orchestrator filed
`k-lint-eps-coupled-criterion-unwritten` asserting a pin "cannot close
the ADDED direction at all". Half right: no evaluable criterion exists,
so that issue stands. But the ADDED direction is **not silent** — an
unrostered ε-coupled predicate keeps flagging under the metre rules,
and `docs/K-REPORT.md:645-658` is a ratified ruling saying exactly
that ("a roster omission therefore cannot silently weaken the gate").
The orchestrator's grep was the constant's name plus "ε-coupled" and
could not see a ruling that used neither. The same argument kills the
ITEM's own "both directions are silent". The genuinely silent direction
is a third one neither file named — a predicate that stays rostered and
stops being ε-coupled — and unit 5 closed it.

**Unit 0.** The dispatch called a recorded decision unrecorded; already
logged above, and the fix pass is softening the orchestrator's own
over-correction (the review's S3: IMPRECISE, not WRONG).

### The pattern, and what changes

Three for three, and the common shape is the same: **the orchestrator
re-derived a number or ran a grep, got a result consistent with the
item's framing, and stopped there rather than asking what else would
produce that result.** Unit 2 is the sharpest case — four columns were
available and three were read. The dispatch briefs did their job, since
each said the reading was a hypothesis and invited correction, and each
lane took the invitation. What did NOT do its job is the orchestrator's
own confidence, which in unit 2's case reached a merged PR body.

Concretely, from here: a dispatcher's re-derivation gets the same
treatment the discipline asks of a lane's sweep — **state what the
pattern could not match, before handing it over** — and a claim that an
artefact has DRIFTED names the alternative hypothesis (it was correct
for an older vocabulary) and says what would discriminate them.

## D201 ruled: arm A, and it merges into D206's re-cut (2026-09-07)

Ev on PR 2109: *"A works, but also it's a demo so it doesn't matter
much"*. **Arm A** — the sweep reaches the `StableName` that already
exists, `SceneBody` carries the evaluation, `tools/tess-meter` writes
the column. `D201`'s `kind` moves `issue` → `unit`, which its own kind
note said would happen once the design question was settled, and
`needs_ev` is cleared. The program now has **no open Ev question**.

The second clause is read as a scoping instruction: the unmeasured
count (how many tour scenes can hand over an evaluation) does not gate
the arm, an honestly ABSENT name on a scene that cannot is acceptable,
and the `demos/` seam with Track X is drawn in the landing PR rather
than asked about. No lane will be spent making tour scenes
document-built to fill the column.

**Sequencing decision, taken rather than asked.** `D201`'s column and
`D206`'s constant both force a re-cut of
`docs/tess-budget-data/tess-budget-baseline.csv`, and a re-cut is a
full release sweep over every tour scene plus a PROPS coordination
round. Landing them in plan order buys two of each for changes that fit
in one, so **they become a single unit 6**, separable in the diff, with
the sweep taken once after both are in. The column is also not additive
the way it looks: `tools/tess-lint` pins column POSITIONS against
`EXPECTED_HEADER`, so inserting one moves the blocks the parser
polices, and both census tests re-derive — which is a second reason to
pay that cost once. `D206` keeps the full falsification review the
posture assigns it; `D201`'s half takes the style lane. Unit 8 shrinks
to `C15` discharged by reading the new column.
