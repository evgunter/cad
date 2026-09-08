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

## Unit 0 CLOSED and merged (2026-09-07)

PR 2111 merged at `876bb74bd`, green on the full code tier (34 checks,
12 `test (…)` jobs, 5 `k-lint (gate, …)` unifications). The first unit
off this slate. `tess-lint-face-ordinal-join` is closed; the decision
stands as **an ungated re-key is a NOTE**, with the rule-5 disanalogy
and the accepted cost written at the site.

The fix pass answered all 19 style findings and improved on two of
them rather than patching what was reported:

- The dead `noted.len() == 60` was **deleted, not repaired**, on the
  right ground: 60 is `72 − 12` by construction, so no assertion over
  it is reachable. Its replacement asserts the **twelve gating scenes
  by NAME**, which is the stronger pin the sibling census in the same
  file already used, and which answers the "named after a conclusion"
  and "names computed and discarded" findings at the same time.
- `Row::is_sized()` gives the four spellings of "carries a sized face"
  one home — after the sweep the only `nurbs.is_some()` in `tools/` is
  inside its body. The scene-level spelling genuinely cannot share it
  (summed cell counts), and its agreement rested only on `parse`'s
  cell-count floor, so that agreement is now ASSERTED over the whole
  committed corpus instead of written down in a comment. That is the
  class fix the reviewer asked for, done as a class.
- The mutation counts were **re-derived rather than copied from the
  review**: 5/6/2/9 under `--no-fail-fast`, against the shipped
  4/5/1/8; each undercount was the `cli_contract` red that fail-fast
  never reaches.

Two residues filed as files, not sentences —
`tess-budget-doc-note-finding-rule` (deferred to unit 2's lane, which
holds `docs/TESS-BUDGET.md`) and `tess-lint-twinned-csv-fixture`.

**A fourth orchestrator error, on the ledger fix.** The hosted matrix
had gone red on unit 5 because `crates/test-utils/tests/reader_census.rs`
gates every site reading Rust source; the orchestrator added the ledger
line, took green, and did not read what the row was FOR. Its own doc
says *"it is a new hand-rolled Rust reader — do not add the line. Use
the shared lexer."* `predicate_roster.rs` does both: it reaches the
shared lexer AND hand-rolls three helpers copied byte-for-byte from the
ledger row two lines beneath it, so `Shared` is one level too coarse.
The guard meant to catch that tests `contains("test_utils::source")`,
which the bare `use` import satisfies with no call. Same shape as the
other three: a result consistent with the framing, and no second
question. Relayed to unit 5's fix pass, where hoisting the helpers into
`crates/test-utils/src/source.rs` resolves it properly.

Unit 1 (`D213` + `D214`) dispatched on the freed `lib.rs`, briefed that
both item files' citations are stale against unit 0's merge.

## Unit 2 CLOSED and merged; the fix pass overturned its own reviewer (2026-09-08)

PR 2114 merged at `1c67277e9`, green on the full code tier.
`tess-budget-doc-finding-block-stale` is closed, and the closure is
**not the one the item, the orchestrator, or the style review
proposed** — which makes this the clearest thing that happened on this
program so far.

**The 146 was never stale.** The style review's S1 found
`docs/TESS-BUDGET.md` asserting that no count over the committed file
is written into the document and then writing five, with `benchlayout`
at 30 against a committed 18, supported by the claim that it "has been
18 in every committed version of the CSV". **That support is false**,
and the fix pass proved it by walking every committed blob rather than
today's: at `48559d61` (VERBS-TESSFOLD) `benchlayout/benchlayout`
carried 30 rows and the five summed to exactly 146. The scene was
re-modelled and re-swept as `bench/benchlayout` at 18 rows in the
montage-v3 tranche-2 renames (`6d957702`, 2026-09-01), taking the sum
to 134. So the passage is a **frozen record that is exact for the
corpus it describes**, now labelled as one, with the document's
universal narrowed to "as a CURRENT reading". Re-verified here against
all three commits before the reversal was accepted.

That is three levels of correction on one item: the orchestrator was
wrong about the block being stale, the style review corrected that and
was itself wrong about `benchlayout`, and the fix pass corrected the
review by going to the artefact instead of to today's copy of it. The
common cure each time was the same — **read the history of the
artefact, not its current state.**

Two other findings settled rather than patched. The mis-pairing
multiplier is **two figures, 3.52x and 3.35x**, not a "3–8x" range: the
8.5x is the pre-fix whole-patch numerator over today's per-cell
denominator, two column definitions apart, describing no schedule that
ever shipped. And the 83-cell gap between `MODEL-AB-LOG`'s 46,102 and
the file's 46,019 was **neither party's slip**: it moved at `a4eb03ae`
(CERT-10 fix pass) when four faces' certified bounds changed, so a
CERTIFICATE change is now named as the third thing a re-cut can be,
beside corpus growth and a schedule change.

Residue rowed rather than disclosed: plan unit 9
(`report-header-column-phrases-unqualified`) and unit 10
(`fold-the-two-baseline-census-files`, gated on both census-touching
PRs landing).

Unit 1's PR is 2125, style review dispatched; unit 5's fix pass is
still running.

## Unit 5 CLOSED and merged (2026-09-08)

PR 2115 merged at `c7dc5eb2d`, green on the full matrix. Three of the
eight units are now closed (0, 2, 5) and the slate's cheap end is
nearly spent.

This is the unit the style lane earned its place on. The first version
of the pin claimed to close the one genuinely silent direction — a
predicate that stays rostered and stops being ε-coupled — and **the
reviewer broke it green twice**: `margin.contains("target_len")` is a
substring, so a `let fixed_target_len` sails through; and a mint whose
`target_len` arrives as a function PARAMETER was never checked. A third
mutation dropped a whole mint site from the parse by respelling
`classify_len:: <T>(`, still green. The fix pass closed all three
(whole-identifier match, the binding sought in the mint's own enclosing
`fn` with a parameter REFUSED rather than answered, and a coverage
floor), and added the converse check the review found open.

`props_quad_last_round` is **excused by name rather than rostered**:
ε-coupled by this unit's own criterion, but rule (4)'s floor is the
minimum of 108 draws of `props_quad_converged`'s statistic and this
predicate has contributed none across all nine committed baselines, so
rostering it would be a distribution ruling with no distribution.

**The orchestrator's ledger error is repaired properly rather than
papered over.** The three helpers that made the `Shared` disposition
half-true are hoisted into `crates/test-utils/src/source.rs`, deleted
from `tools/tess-meter/tests/derivations.rs` and never written in
`k-lint`. The class behind it is filed
(`reader-census-shared-disposition-survives-partial-reversion`):
`every_shared_entry_actually_reaches_the_shared_lexer` tests one
substring that a bare `use` import satisfies, so all 44 `Shared` rows
carry the same hole, and strengthening it means ruling on every row.

### A near-miss the orchestrator owns

Verifying the fix pass's "the pin now reds" claim, the orchestrator's
first mutation script **failed its own assertion before writing the
file**, and the suite came back green. Green on an unmutated tree,
which is evidence of nothing. Reading that as verification would have
merged an unproven pin into the PR whose whole subject is assertions
that pass for the wrong reason. Caught, redone against the real anchor,
and the answer was the true one: RED under the substring break, green
reverted, `crates/` clean. **The lesson is the same one this program
keeps producing — a green result is only evidence once you have checked
that the thing you meant to break actually broke.**

Three new items filed by the lane:
`k-lint-last-round-is-eps-coupled-but-unrostered`,
`k-lint-roster-wants-a-kernel-side-vocabulary` (a `const` slice from
`geom_core::k_stats` would close the criterion issue in the same PROPS
edit), and the reader-census class above.

**Unit 1 (`D213` + `D214`) landed the sizing block as one decision.**
The two items pointed at the same seam from opposite sides and the
answer was one edit: `parse` refuses the lane pairing in both
directions (`SIZED_CHART_TAGS`, the consumer's mirror of
`tess_meter::FaceRow::csv_row`'s 2x2), which makes `Row::is_sized` a
function of `chart` on every parsed row, which retires
`IDENTITY_COLUMNS`' block-presence entry rather than re-justifying it —
`chart` is entry zero, so the block entry could never have been the
first disagreement rule 4 reports. `D213`'s mesh chain was re-walked
against the merge base, `per_cell_candidates`'s `?` included, and
holds. The census goes six-of-eight to five-of-seven.

Two things the wave should know. The pairing test derives its
expectation from `SIZED_CHART_TAGS`, so a member gained or lost moves
the expectation with it and passes in silence — the roster needs the
written-out literal pin `CHART_TAGS` already carries, and that is the
only guard on its membership; a cross-root pin against
`Chart::sized_lane` would want `tools/tess-meter/tests/derivations.rs`,
which is the meter's file and its ground, exactly as `CHART_TAGS`'s own
doc says of the ADDED direction. And `docs/TESS-BUDGET.md` enumerates
the identity list twice, both copies still eight entries — filed as
`tess-budget-doc-identity-column-list` rather than edited, because unit
2 holds that file for the wave.

**Unit 1, fix pass.** The review found the unit had disarmed a guard it
did not touch: `an_unknown_chart_tag_is_harness_breakage_not_a_re_key`
asserted only `text.contains("chart")` and `contains("hessian")`, and
the new pairing arm's `(false, true)` message names both — so deleting
`CHART_TAGS`' roster check left all 65 tests green on this branch while
the same deletion reds on `main`. Reproduced in both directions before
the fix. The refusal is now compared WHOLE and on both row shapes (the
unsized one, where the pairing has no opinion, is what the fixture was
missing), the two lane-pairing tests compare whole messages for the
same reason, and the vacuous `if let Err(e) { assert!(!contains(..)) }`
loop is gone — its positive content is
`every_chart_tag_owes_the_sizing_block_or_refuses_it`, which reads both
shapes per tag.

The sweep behind it is mechanical and is the instrument this class
wants: **delete each refusal in `parse`, one at a time, and record
which tests red.** Ten mutations over nine guards. Before the fix, one
guard — the roster — had no red; every other guard reds at least one
row, and the attribution is right in each. After, the roster reds two.
Refusal ORDER is now pinned too (roster before pairing, pairing before
`face`), proved by swapping the two blocks; nothing pinned it before.

`SIZED_CHART_TAGS`' membership is no longer guarded only by a literal
inside the test that reads it: the cross-root pin was WRITTEN rather
than filed —
`tess-meter`'s `the_lints_sized_roster_answers_sized_lane_for_every_tag_this_crate_emits`
reads the declaration out of `tess-lint`'s source and asserts
membership equals `Chart::sized_lane` per tag, with a falsification
guard covering a roster short a sized tag and one carrying an unsized
one. Per-tag biconditional rather than equality, for the same reason
`CHART_TAGS`' pin is containment: the lint parses baselines cut from
older trees, so a retired tag must stay in both rosters.

Also this pass: `CHART_TAGS`' doc said closing its one-way asymmetry
was "`tess-meter`'s ground" when `the_lints_roster_admits_every_tag_this_crate_emits`
had already closed it — a stale sentence that steered the deferral
above. `D214`'s history figures did not reproduce and were re-taken (20
blobs, not 127; no pre-`triangles` header shape exists; the conclusion
is unchanged). A third copy of the identity-column enumeration turned
up in `local-scripts/ci-local.sh`'s hosted-mirror comment and is fixed
here; `C15` and `D201` restated it too and are corrected. `D213`'s
close is rewritten to lead with the argument that carries it — a
property of `parse` and `first_disagreement`, no kernel content — with
the mesh chain demoted to what it is actually for.

## Unit 3 — `D203`, the cross-column rule (2026-09-08)

The rule gets one home: `tools/tess-lint/README.md`, clauses
`CC1`–`CC5`, cited by path from `k-lint`'s `Admissible` and from the
band check in `lint_csv`, and by clause from four sites in
`tess-lint`. First `tools/*/README.md` in the tree; the precedent is
`scripts/gates/README.md` — a design page beside the code it governs,
where the governed code is not one crate.

**The unit was briefed with two instances and the sweep found seven.**
The lane pairing unit 1 landed four commits earlier is a genuine third
— checked against the merged source, not assumed: `parse`, harness
voice, two columns `Admissible` polices singly, and a comment already
calling itself *"the second cross-column rule"*. The others: the
all-or-none sizing tail, `worst_dev` against `dev_samples`, `k-lint`'s
`Margin` against `outcome` (which resolves a cross-column invariant by
WIDENING the table's signature rather than checking beside it, and is
the reason `CC2` exists as a separate clause), and `Extent`'s trim-box
non-degeneracy, which is checked nowhere because `span_opt_cells`'
geometric floor already refuses every violating row.

`cap_bands` / `snap_bands` against `bands` was the one instance
closable from the consumer's side and is now refused at `parse`. The
report prints those counts `of {bands}`, so an unrefused one reached
the reader as a reading. Mutation-checked in both directions: red with
the guard removed, red with `>` widened to `>=`.

`CC5` is the clause that did the most work in the sweep — an admission
refuses what the producer could not have written, never what the
instrument exists to measure. `grid_cells` against `span_opt_cells`
has exactly the shape of an invariant and is the `split` ratio the
report is for; the committed baseline carries rows on both sides of
it.

Filed: `tess-lint-zero-certificate-two-meanings` — `worst_cert = 0`
has two meanings in `mesh::budget` and one in `Admissible::Certificate`,
and the discriminant the kernel names is `0` on every row of the
`--sizing-only` sweep CI gates on. Producer-side fix, outside `D203`'s
fence.

### Unit 3, fix pass (2026-09-08)

**`CC4` was false and the style review broke it by construction.** It
claimed the trim box's non-degeneracy was a property *"no row
surviving the per-column table can violate"*. Reproduced: a row with
`u0 = u1 = 0e0` beside `span_opt_cells = 2.5e1` parses, and the
collapsed box reaches rule 4's `identity` as the face-identity reading
`["nurbs", "0.0", "0.0", "0.0", "1.0", …]`. The true criterion was "no
row THIS PRODUCER writes can violate" — producer-correctness, which is
the one thing the instrument exists not to assume and which
`tess-meter`'s own header says outright.

The clause is not rewritten as a conditional exemption; it is
**inverted**. `CC4` now says there is no fourth disposition: a
producer-side entailment is not one, because the boundary cannot see
the producer's code. `Extent` moves to `CC3` and is checked. That
disposes of the cross-crate entailment nobody could invalidate, and of
the exemption that was facing the wrong way.

**The page moved to `tools/README.md`.** The cited precedent
(`scripts/gates/README.md`) is a DIRECTORY page, the rule's subject is
two instruments, and hosting a shared rule inside one of its two
consumers is the drift shape. `tools/tess-lint/README.md` held nothing
else and is deleted; eight citations across the two crates now name
the new path. The `CC3` instance roster is deleted with it — each site
cites the clause where it stands, and a census on a clause page rots.

**Citations rot loudly now.** Each citing crate `include_str!`s the
page (a moved page stops both crates compiling) and asserts that every
clause id it cites is a heading there and that the page carries no
clause it has not seen (a `CC6` reds both). Negation-checked in all
three directions.

**Two more instances by the unit's own criterion**, both checked at
`parse` in the harness voice: `patch_cells = nu · nv`, stated in
`tess_meter::columns` and in the header and printed by the report; and
`opt_cells ≤ patch_cells`, since `best_split_scan` seeds its running
minimum with the same whole-patch schedule. The blanket "optimality
relations are the report's subject" that swept the second one out had
been verified only for `grid_cells`/`span_opt_cells`; it does not
generalise, and the distinction is now written into `CC5`. Both are
exact on all 64 sized rows of the committed baseline, `opt_cells`
equal on six of them.

**Three claims corrected.** `CC1` no longer says the row type is the
contract past the boundary — `tess_lint::Row` says the opposite at
itself, and `k-lint` has no row type at all and exports `lint_sample`
over raw scalars its own tests call with hand-written bands.
`tess-meter`'s "diagnostics no rule reads" is retired: the consumers
arrived. And `tess-lint-zero-certificate-two-meanings` rested on a
false premise — the gate reads TWO CSVs through the same `parse`, and
`dev_samples` is 2464–201096 on all 64 sized rows of the committed
baseline, so the discriminant is live on that side.

**The page is unratified and now says so.** Six code sites cite
`CC1`–`CC5` as clause law and `docs/DESIGN.md`'s companion table
carries no row for the page. The criterion for such a row is Ev's
ratification, not a program close: `scripts/gates/README.md`'s row
landed in the commit recording Ev's ratification while `work/gates`
was still open. No row is added here.

## Unit 4 — the cut line's two halves, pinned (2026-09-08)

`cut-prefix-three-unpinned-spellings` closed on `meter/cut-prefix-pin`.
`tools/tess-lint/tests/cut_line_pin.rs` reads
`scripts/tess_budget_cut.sh` as text and holds its three spellings of
the prefix to `CUT_PREFIX`, each located separately so each reds
alone.

**The finding was the shape, and it had moved under the filing.**
`split_cut` grew a shape check between the filing and the lane, so the
open question became whether the two constraints are the SAME. They
were not: this crate admitted uppercase hex, an over-long object name
and arbitrary whitespace after the prefix — three spellings
`tess_budget_cut.sh` cannot emit and its `CUT_RE` does not match. The
asymmetry costs in one direction only, and it is the bad one: the
script's already-stamped arm would not recognise such a line, so it
re-stamps a file that already carries a cut. `split_cut` is tightened
to the script's language and a truth table now runs both readings
side by side, the script's own regex extracted from its text and run
by `grep -E`. Cited to `tess_lint::Report`, whose test decides both
that a check is owed and that its voice is the harness voice;
`tools/README.md`'s `CC1`–`CC5` were cited at first and are not any
more, since they are stated over cross-column admissions and hand the
general test back to `Report` themselves. The separate finding about
that page's scope stands and is with Ev on #2147.

**The fix pass, same PR.** Two assertions that could not fail went:
the containment `!(reads && !recognises)` read the table's own
constants rather than the computed answers, and a coverage predicate
over the const table was subsumed by the per-row asserts. The
containment now runs on the computed pair and runs FIRST, so a real
inversion reds with what it costs rather than with a table mismatch.
The stem sweep was narrowed to EXECUTABLE spellings — a comment line
is skipped, proved by a decoy fixture — because
`scripts/tess_budget_cut.sh` is CIW's and an ordinary sentence there
was reddening a suite in a cargo root outside the workspace, with a
message naming neither the rule nor the fix; every failure the sweep
can produce now carries both. The floor moved from `>= 4` over five
mentions to `>= 4` over exactly four executable ones.

**Residue, filed on CIW's slate** (the fixes are edits to
`scripts/tess_budget_cut.sh`, so they go where the owner will see them
rather than waiting on this program's pre-close sweep):
`cut-regex-unanchored-admits-a-line-the-lint-refuses`, `CUT_RE`
lacking an end anchor, pinned meanwhile as the one row of the truth
table where the two halves disagree; and
`cut-script-header-claims-no-cross-language-gate-exists`, the script
header sentence this unit falsified. On METER's own slate,
`baseline-census-partition-assert-cannot-fail` — the second and only
other `tools/` instance of the cannot-fail assertion class.

**Left, deliberately:** `cut-line-commit-names-no-baseline-change`.
Same seam, different defect, and its substance is what the verdict
PRINTS rather than what the parser admits. A doc-only rider would
half-close it.

## Unit 6 — `D206` + `D201`, one re-cut (2026-09-08)

Branch `meter/split-scan-and-face-name`, three separable commits and
one sweep.

`SPLIT_SCAN_SAMPLES` is 379: the one-sided envelope reads 5.0075% at
378 and 4.9939% at 379 against the gate's 5%, so the threshold the row
carried is the right one and was re-derived rather than inherited. On
the `ceil`'d column the family's worst falls 5.88% → 2.94%. Every
sample-count-dependent percentage in `tess-meter` moved with it,
including two suprema that would have read ABOVE the new bound had they
been left.

The budget CSV carries a `name` column at position 2 — a `StableName`
in its ratified serialization with `,` swapped for `;`. **6 of the
tour's 72 scene bodies can hand over an evaluation** (the three
heatsinks and the three die stops), so 286 of 1353 rows carry a name
and the rest are honestly empty, which is what Ev's second clause
scopes as an outcome. **The coverage is disjoint from the defect**:
none of the 64 sized rows and none of the 14 rows in the seven
same-shape pairs is named, so `C15` cannot be discharged by reading
the column.

**The fix pass corrected what the unit claimed, not what it did.** The
sentence licensing 379 said the one-sided envelope was "the largest
factor a `ceil`'d count can inherit from where a bound happens to sit
relative to the lattice". It is not: the envelope bounds the
CONTINUOUS excess, and a `ceil`'d miss costs a whole division —
`muu = 100, mvv = 0.1` over a `1 × 10` box at `δ_s = 1` admits 65 cells
at `t = 26` and the shipped scan reports 70, **7.6923%** against a
4.9939% envelope and against the gate's whole 5% margin. One missed
division out of `n` costs `1/n`, and the corpus's median
per-analysis-cell optimum is 44.4 cells — near seven divisions an
axis, where one whole division is 14%. So `D206`'s premise
survives its own closure, re-filed as
`tess-lint-growth-margin-unprotected-from-ceil-quantisation`; what 379
bought is the continuous half, now re-derived by a row instead of
asserted in prose. Two range-claim figures went the same way — a
scan-to-true ratio quoted as a ceiling is a sample at an unstated
density, and denser sampling of the same range beats both.

One sweep, with the deviation pass, at `3f55f361b22e`. Two columns
moved for this unit's reasons (`opt_cells` −1.16%, `span_opt_cells`
−0.64% over the sweep); seven more moved because the previous cut was
four days and 442 `crates/` commits stale, which nothing reports —
filed as `tess-lint-ungated-columns-fold-silently`.
## Units 7 and 11 closed; and the slate is now larger than it opened (2026-09-08)

Both lanes deliberately left this file alone to avoid conflicting with a
live sibling, so their entries are written here.

**Unit 7 — `k-report-baseline-fold-cert1-roster`** (PR 2140). The
CERT-1 roster fold was READ and **no committed CSV was re-cut**, which
is the unit's judgement rather than an omission: `k-report-data`'s rule
1 cuts a new era when the DISTRIBUTION moves, and the three witnesses
were pointwise identical to M7. The full falsification review ran the
attack the lane had not — smallest ambient definite margin over every
non-ε-coupled name, both eras — and could not break it.

What the review DID break was **when** the measurement was taken: at
the branch's merge base rather than the PR's base, with
`props/curved.rs` moved 129+/74− in between, adding a refusal door that
runs *before any margin is formed* and two recorded names. Re-swept at
`c39a904e`: roster 279 → **281**, verdict unchanged. The lane also
caught an arithmetic slip in its own first pass (the decade gap is
14.8, not 15.8). The reviewer predicted ≥282; `interval_span_winding`
turned out to exist at both tips, so 281 is right — checked here.

**Unit 11 — `k-report-era-witnesses-have-no-guard`** (PR 2158), filed
by unit 7's fix pass and closed the same night. `docs/K-REPORT.md` now
states as a merged claim that M7 remains the era because the
distribution did not move, and **nothing computed with the three values
that claim rests on**, while `threshold_provenance.rs` re-derives four
shipped constants against `M7` on every gate run. The guard asserts
each witness's PREDICATE as well as its value — a floor that stayed at
4.79652e-5 under a different name is a moved distribution wearing the
old number — and it EXHIBITS the string-comparison trap rather than
asserting it, running the selection both ways over the committed rows.

Verified here by corrupting the committed `.gz` (renaming one predicate
occurrence) rather than by editing a literal: RED with *"M7 carries 234
distinct predicate names at eps=1e-9, not the 233"*, then restored
byte-identical. That is the strong form — it catches corruption of the
artefact, not merely an edited expectation.

### The slate opened at eleven items and now stands at twenty-nine

Eight of the eleven are closed and the directory is bigger than when it
started. That is not drift: every one of the eighteen new files came
out of a sweep a unit was required to run, or a reviewer's class
finding, and each names a real defect with a citation. The program's
own instruments kept finding siblings of what they were sent to fix —
`EXPECTED_HEADER` hand-copied at five producers, a `Shared` ledger row
a bare import satisfies, a partition assert that cannot fail, two sites
claiming the k-lint gate diffs committed baselines when nothing does.

**What that means for closing.** METER cannot exit by draining the
slate to zero: at the current rate each closed unit files two more.
The exit walk will have to rule on which of the twenty-nine are METER's
to finish and which are re-homed — three already went to `work/ciw/`
on unit 4, which is the pattern. Recording it now because the decision
belongs in the walk and the number will be larger by then.

## Unit 6 CLOSED and merged; unit 8 re-planned from a measured reading (2026-09-08)

Merged at `c8136adf2` (PR 2167) with all 37 checks green. Before
merging, the four things green CI cannot see were checked here: the two
cannot-fail assertions really are gone from the tree (`total_ceiling`,
`every_token_is_one_non_empty_csv_field` — no match anywhere under
`tools/`), `tess-lint-growth-margin-unprotected-from-ceil-quantisation`
is filed and open, `work.py lint` is clean, and the counterexample
licensing the rewrite reproduces exactly: `muu = 100, muv = 0,
mvv = 0.1` over a `1 × 10` box at `δ_s = 1` gives an admissible 13 × 5
grid at `t = 26` (65 cells) that the lattice does not contain, against
which the shipped 379-sample scan reports 70 — **7.6923%** over a
4.9939% envelope. The rewritten licence does not merely drop the false
sentence; it names it false and carries the exhibit, which is the form
a corrected claim should take.

### Unit 8 cannot be what the plan said, and the replacement is sharper

The plan had `D201` → the join → `C15`, with `C15` discharged by
reading the new column. It cannot be: `D201` landed the column and
**0 of the 64 sized rows carry a name**, all 14 rows of the seven
indistinguishable pairs among them. Re-derived here from the committed
baseline — 7 pairs in `lily/lily_leaf_b`, `lily/lily_leaf_c`,
`lofts/loft_prism`, `lofts/nonuniform_loft`, `s_duct/s_duct`, every
name field empty.

So the question became what an undetected swap actually COSTS, and the
answer is worth the unit. Two readings of mine were wrong on the way to
it and both were caught by reading the rules rather than the docs:

- *"every gated column is bit-identical within each pair"* — I wrote
  this before checking, and `worst_dev` differs within five of the
  seven pairs, by 1.4652% on `lofts/loft_prism`. Had `worst_dev`
  gated, the claim would have been backwards.
- *"`worst_dev` gates, so a swap spends 29% of the total rule's
  margin"* — also wrong. The module docs list `total = delta /
  worst_dev` among the three ratios, but the gate emits five kinds
  (`Vanished`, `Triangles`, `Uncovered`, `Slack`, `Rekeyed`) and none
  reads `worst_dev`. The total ratio is REPORTED, not gated. A
  ratio named beside two gated ones in a doc list is not thereby
  gated, and the enum is what settles it.

**The checked statement.** Rule 1 is per-SCENE triangle totals, so a
swap WITHIN a scene cannot move it — that half is a theorem, not a
reading. Rule 2 compares `recoverable()` = `grid_cells /
span_opt_cells`, and both are bit-identical within all seven pairs on
the committed baseline — that half IS a reading, and could stop being
true at any re-cut. So today an undetected swap moves no finding and
moves the reported `total` ratio by up to **1.4652%** and `worst_cert`
in its last digits: **invisible in the gate, visible in the report.**

That is unit 8: the reading gets an executable home in
`baseline_census.rs` beside the pair census that already stands there,
so that a re-cut which makes a swap GATE-visible fails and names what
moved — and the name column's disjointness from the pairs gets pinned
in the same place, so that a scene becoming document-built fires and
says `C15` became dischargeable. `C15` stays OPEN: the defect is live,
what closes it is corpus-side, and Ev's second clause on `D201` scopes
that as not urgent rather than as done.

## Unit 8 reported; two corrections to the orchestrator's brief (2026-09-08)

PR 2177, green. The lane checked all four of my claims against the
committed CSV and none came back false — the seven pairs, the five
scenes, the ordinals, 0 of 64 sized rows named, the five `Kind`
variants, the bit-identical `grid_cells`/`span_opt_cells`. Two things
it corrected are MINE, and both are the same mistake in different
sizes: a claim stated more widely than what was measured.

- **`worst_cert` "in its last digits" over-generalised.** It moves
  within four of the seven pairs and is bit-identical in the other
  three (`loft_prism`, `nonuniform_loft`, `s_duct`). The full row of
  total-ratio movements is 1.4652 / 0.8788 / 0.1304 / 0.0182 / 0.0146
  / 0 / 0 — so three of the seven pairs cost a swap nothing on the
  reported side either, which the single worst-case figure hid.
- **The module docs are not defective, and my brief said they were.**
  I wrote that `tools/tess-lint/src/lib.rs`'s three-ratio list misled
  me into thinking `worst_dev` gates. It does not: the list is at
  `:22-38` under *"# 1. The report"*, the gate is a separate section
  from `:52`, and the `held` bullet explicitly flags the one ratio that
  DOES reach the gate while nothing of the kind sits on `total`. The
  docs already draw the distinction. I misread a correctly sectioned
  list and then blamed the list. Nothing was filed, which is right.

The lane also declined to assert the reported-side figure, and its
reason is better than my leaving it open: **an assertion that goes red
when the corpus IMPROVES is not a guard.** A pin on `worst_dev`'s
movement would fire the day a pair's two members came to agree. That
is a general test worth carrying — it is the cannot-fail defect's
mirror image, an assertion that CAN fail but only in the wrong
direction.

### Operations: the shared working tree, and a stale `main`

Three lanes were dispatched into what turned out to be ONE working
tree at `/home/user/cad`, and the orchestrator's own post-merge check
staged `work/` from `origin/main` onto a lane's checked-out branch
before anyone noticed. Restored, and every lane now has its own
`git worktree`; the orchestrator works only in `/home/user/cad-orch`.

Then the fix made a second mess: moving a lane out, I told it to
`git checkout main` first — and this container's local `main` is a
pointer from 2026-09-04, **11,510 commits behind `origin/main`**. That
parked the shared tree on an ancient snapshot, where unit 8 read
`r.nurbs.is_some()` and an eight-column identity list and correctly
reported it as a possible sibling-lane conflict. It was mine. The tree
is re-parked detached on `origin/main`.

Both are worth carrying: **concurrent lanes need one worktree each, and
`main` in an ephemeral container is not necessarily `origin/main`.**

## Unit 8's style review returned a MAJOR against the orchestrator (2026-09-08)

The tests survived — the reviewer could not break them, and confirmed
both assertions independently reachable exactly as the lane argued,
every assertion able to fail, and the vacuity guard closing its hole.
What did not survive is the sentence I wrote into the brief.

**"Invisible in the gate, visible in the report" is false in its
second half.** I conflated two functions with one name:
`Row::total_slack` (`tools/tess-lint/src/lib.rs:461`) IS
`delta / worst_dev` and the report never prints it — it is only an
addend inside `SceneTotals::add`; what `main.rs:354` prints is
`SceneTotals::total_slack` (`:1272`), `measured_triangles /
extrapolated_triangles`. That function's own doc at `:1266-1270`
names and REJECTS the identification I made. `worst_cert` is printed
by nothing at all.

Verified here rather than relayed, and the verification is the lesson.
Swapping each pair's `face` field by minimal textual edit and diffing
`tess-lint --top 100`: **all seven identical.** My first run said the
opposite — because I had rebuilt the CSV through `csv.writer`, which
changes the report on its own with no swap at all. The control caught
it: a no-op round-trip already differed. A second confound was the
report echoing its input path in line 1. Only with a byte-identical
control passing and that line excluded does the measurement mean
anything.

**Three errors in one family, and this is the largest.** I checked
which COLUMNS differ within a pair and never checked what the report
PRINTS. Reading the enum told me `worst_dev` does not gate; I stopped
there and assumed the complement — that it therefore reaches the
report — instead of grepping `main.rs`, which takes one command and
returns zero. *Establishing that a thing is not in set A is not
evidence that it is in set B.*

**The corrected claim**, which is smaller and better founded: a swap
exchanges two whole rows, so every aggregate the report folds sees the
same multiset. What an undetected swap costs is a **wrong-face
attribution in a column nothing reads** — not a moved number anywhere.
The 1.4652% is real but it is the gap between the two ROWS'
`delta / worst_dev`; a swap does not change either value, only which
ordinal carries it.

That makes `C15`'s live blast radius smaller than this program has
said all along, and it strengthens rather than weakens the case for
the census guard: gate-invisibility is now established by EXECUTION
over the committed corpus, and the guard is what fires when it stops
being true.

The reviewer also ruled the question I put to it (S2): dropping the
two numbers from the row was right, leaving the PREDICATE unguarded is
the over-correction — a reading of a committed artefact sitting as
prose is the shape the row itself forbids. Sent back with the MAJOR.

## Three reviews, three instances of one defect: the claim of uniqueness (2026-09-08)

Units 8, 9 and 12 were reviewed within a few hours of each other, and
each review's MAJOR was the same sentence in different words: **a claim
that some site is the only one of its kind, made without sweeping for
the others.**

- **Unit 8** — a doc identified `Row::total_slack` (`delta / worst_dev`,
  printed by nothing) with what the report prints,
  `SceneTotals::total_slack` (triangle-weighted). Two functions, one
  name. The orchestrator wrote that one.
- **Unit 9** — the same shape survived into the unit that exists to fix
  ambiguous naming: `Row::recoverable` and `SceneTotals::recoverable`,
  differing by aggregation, and the unit attached the column formula to
  the sweep-level one only. So `grid_cells / span_opt_cells` now has
  THREE English names in one report.
- **Unit 12** — `csv_fixture.rs:36-38` says the fixture's own test is
  *"the only thing in either root that can see its token"*, while
  `src/lib.rs:1768-1771` says it is *"the only thing that can catch
  `parse` reading it out of the wrong column"*. Both about the same
  value; the second is unit 6's. The tree carried two sole-reader
  claims about one field, and blanking the token reds both.

**And the sibling shape, also three for three: a PR disclosing a blind
spot in its own sweep and claiming or implying it was empty.** Unit 9's
*"there are none in this file today"* — `main.rs:233`'s `line()` helper
is one, and the figure it hides is the very quotient the unit renamed
elsewhere. Unit 12's blind spot — a second full-width sized-row literal
at `src/lib.rs:1983-1990`, inside the fence, which reds for the WRONG
reason when the header moves. Unit 8's was the orchestrator's rather
than the lane's: an experiment with a negative control and no positive
one, which could show the harness was not lying but never that it could
see.

**The rule these three suggest**, and it is cheap: *a uniqueness claim
is a sweep result, so it is not writable without running the sweep —
and "my pattern could not match X" is a work order to find X another
way, never a boundary.* Every one of the six instances was one grep or
one control run from being caught, and none of them was caught by the
lane that wrote it.

Worth carrying to the exit walk: this is a candidate for the standing
discipline docs rather than a METER row, since nothing about it is
specific to the instruments.

## The third fake green, and it was in the verification harness (2026-09-08)

Unit 9's fix pass caught one in itself and reported it unprompted. Its
verification chain was

    cargo test … | grep -E "^test result|FAILED" && …

and `grep` exits 0 on a MATCH — so a run containing `FAILED` satisfies
the chain and the `&&` fires. **Three failing tests passed straight
through into a commit.** The failures were real: four tests shared one
`{pid}` fixture path, `fs::write` truncates before writing, and a
reader could take the file inside that window.

That is three fake greens in this program, and they have nothing in
common except their shape:

1. Unit 5 — a mutation script that failed its own assertion BEFORE
   writing the file, so the suite ran on an unmutated tree.
2. Unit 11 — a mutation that landed on a doc comment 110 lines above
   the executing literal.
3. Unit 9 — a verification chain that cannot fail.

**Every one produced a green that meant nothing, and in all three the
green was believed until someone asked what would have made it red.**
The cure is the same each time and it is not more care: it is to break
the thing on purpose first and watch it go red, then fix it and watch
it go green. A green you have not earned by first producing a red is a
measurement of your harness, not of the tree.

Also settled here: unit 9's reviewer filed the fixture-path race at
`unsure`, saying it could not reproduce it in 40 consecutive runs. The
lane reproduced it at **1 in 25** on the shared path and **0 in 25**
with a path per call. A finding a reviewer cannot reproduce is still
worth filing — this one was live, and dropping it for want of a
reproduction would have shipped an intermittent pin, which reds for a
reason that is not the reason it exists.

## Unit 8 CLOSED and merged; and a nastier variant of the day's pattern (2026-09-08)

Merged at `9ad3b0bc7` (PR 2177), head `4ea426d08`, 37 checks green,
after three rounds: the MAJOR, a style pass, and three prose fixes.
`C15` stays OPEN with its residue now guarded by execution rather than
argument, and `D206`'s neighbour `baseline_census.rs` carries six tests
where it carried three.

**The lane's diagnosis of its own N1 is better than the one I gave it,
and it names a variant worth separating from the rest.** I filed the
"harder permutation" claim as another instance of the day's
uniqueness-claim pattern. It is not, quite. The lane's
`extrapolated_triangles` measurement was REAL and survives — the
permutation does move it in the last bit on 2 of 7 pairs. What was
invented was the ATTRIBUTION: that only one of the two spellings
produced that movement. In the lane's words, **"the measurement was
sound and the attribution was invented, which is a nastier variant
than an unmeasured claim: it arrives wearing evidence."**

That is the shape to watch for, because an unmeasured claim can be
caught by asking "did you measure this?" and this one cannot. It
answers yes. The question that catches it is *"what did you measure,
exactly, and what else would have produced the same number?"* — here,
one `grep` for `FaceIndex` shows `totals` folds by ordinal through a
`BTreeMap`, so relabelling and re-texting produce the identical addend
sequence and there was never a weaker spelling to be stronger than.

It also travelled furthest of any error today: the doc comment, the
lane's report, the PR body, my message to the reviewer, and my summary
to Ev. Only the reviewer stopped it, by implementing both versions and
comparing them. **Evidence-shaped claims propagate faster than bare
ones, because each reader takes the evidence as having been checked by
the last.**

`D103` (5 sites) and `D107` (1 site) — dead `work/code-quality/*.md`
pointers the lane found outside METER's fence and correctly reported
rather than filed — are the orchestrator's to route.

## Unit 9 CLOSED and merged; and the ordering rule that prevents the citation defect (2026-09-08)

Merged at `2d7b39809` (PR 2180), head `18c314c7b`, 37 checks green,
after three rounds: three MAJORs, then the `_cells` and gloss fixes,
then the citation re-take. The report now names its four cell totals by
column with the two factors as formulas, and `report_columns_pin.rs`
gates the names, the order, the qualifiers and the completeness
converse against the parser's own roster rather than against a
spelling.

**The citation defect recurred INSIDE the pass that was fixing it, and
that is what produced the rule.** Re-taking the ten line numbers, the
lane then fixed the gloss-pin overclaim — which inserted lines above
two assertions and shifted `:351`→`:358` and `:385`→`:392`. It caught
that only because it had just been told the shape of the failure. The
rule it derived and now follows:

> **Commit every edit first, then take citations from the committed
> tree, then write them up.** Any other order re-creates the defect by
> construction.

That is stronger than "check your citations", because it removes the
window rather than asking for vigilance inside it. The durable half is
the same lesson one level up: the table now carries an ENCLOSING TEST
NAME beside each line, and the re-filed item cites symbols with the
number as a hint, because **a test name survives an edit and a line
number does not.**

The lane also corrected the table's PREAMBLE rather than only its rows
— saying plainly that the previous version was stale by −10, taken
before the race-fix commit, under a claim that it had been re-taken.
Without that, the new table is indistinguishable from the old one to
anyone who does not re-run it, which was the entire complaint.

Verified here before merging: `:358`, `:392` and `:263` are
`assert_eq!`/`assert!` sites, `main.rs:237` is `fn line`, and the
`SceneTotals` triple `:1285`/`:1292`/`:1330` is `recoverable`,
`span_held`, `total_slack` — the corrected order, which the first
version had inverted.

## Unit 12 CLOSED and merged; the scope of a figure is not the scope of the command (2026-09-08)

Merged at `51c5aebda` (PR 2179), head `647726cf3`, CI green, after
three rounds. `tools/tess-lint`'s CSV fixture now has one home,
`src/tests/csv_fixture.rs`, mounted into the crate's test module and
into `tests/cli_contract.rs` with `#[path]`, so a value drift between
the two sides can no longer be WRITTEN. The unit also overturned the
finding that promoted it: blanking the token was not silent on CI
(clippy's `dead_code` caught it, by accident of reachability rather
than by any content check), and the failure mode nobody had measured —
drifting the constant's VALUE on one side — was invisible to tests and
clippy on both.

**The unit's two self-caught errors have one mechanism, and the lane
named it better than I did.** It reported a clippy grid it had not run,
and a drill count scoped to the lib binary as though it were the
tree's. The second has a concrete cause worth keeping: **`cargo test`
fail-fasts at the lib binary, so a drill run without `--no-fail-fast`
never sees the rest.** Every drill the lane ran was `--lib` or a
fail-fasting `cargo test`, so it reported a number whose scope came
from the command it happened to type rather than from the claim it was
making. The true figure is 5 crate-wide, not 1: the other four are the
two census files reading a committed baseline whose header no longer
matches.

**And the S3 argument, which the orchestrator relayed onward and
praised.** The lane argued that a pin on the reader census would
manufacture the finding it was checking for, since it would name a
`.rs` file without mounting it. False: **owing a ledger line is not a
finding** — `reader_census.rs`'s own header says so in plain words
eight lines in, *"not 'you did something wrong'"* — and
`tools/k-lint/tests/predicate_roster.rs` and
`tools/tess-meter/tests/derivations.rs` both carry `Shared` lines and
dev-depend on `test-utils` to earn them. The real blocker is narrower:
`tools/tess-lint`'s manifest declares NO DEPENDENCIES AT ALL as a
stated property, so reaching `test_utils::source` changes that
property; and the ledger line is outside METER's fence. Conclusion
right, reason wrong. It never reached the tree — only the PR body, the
lane's report, my relay to the reviewer, and my summary to Ev — and the
body now carries the correction as a correction rather than a silent
rewrite.

The lane's own diagnosis ties the day together: *"both were conclusions
I reached without opening the thing I was reasoning about. I inferred
the ledger's semantics from the shape of a red instead of reading its
header — the same move as citing a line without opening it."*

**That is the day's finding in one sentence.** Every error today, the
orchestrator's six included, was an inference from the shape of a thing
in place of a reading of it: which function a doc list implied was
gated, which spelling produced a measured movement, which file a quoted
figure lived in, what a census red means, how many tests a drill reds.
None needed cleverness to catch. Each needed one `grep`, one control,
or one header read.

## Unit 10 CLOSED and merged — METER is out of planned units (2026-09-08)

Merged at `9678901b6` (PR 2187), head `8f6b7fafd`, four full-matrix
greens on the branch. `tools/tess-lint/tests/baseline_sizing_census.rs`
is gone; its census lives in `baseline_census.rs` — one `BASELINE`
const behind one `include_str!`, one header, seven `#[test]`s. Verified
here before merging that the final push touched `work/` markdown ONLY
and `tools/` is byte-identical to the reviewed head, which is what
licensed merging without a third review round.

**The name was the lane's call and it overruled both the item and the
plan.** Both proposed `tests/baseline.rs`; the lane kept
`baseline_census.rs` on a pointer census — 24 live mentions across 14
files name it against 7 across 6 naming the deleted file, so a third
name would have staled all 31. Choosing the name that costs the fewest
stale pointers is the right instinct for this program, and the
reviewer confirmed the winning side exact to the number.

**Evidence of the right shape for the risk this unit actually carried.**
A fold that drops a `#[test]` still compiles and still goes green, so
a total is worthless: all seven were confirmed BY NAME from
`cargo test -- --list`, and the reviewer went further with a PER-TEST
mutation sweep — one targeted mutation per test, seven separate runs,
all seven red individually.

### The last two rounds were both the same defect at one remove

Unit 10's two MAJORs were a missed sweep site and a false "no live
figure" claim. Both were fixed with CAUSES rather than corrections, and
both causes were tooling the lane had written and not checked:

- The missed site came from re-sweeping `work/ docs/` instead of the
  root, against a file the same merge had just created. The reviewer
  reproduced the arithmetic exactly — 9 hits under those two
  directories minus the item's own 3 = the 6 first reported; 10 from
  the root minus 3 = 7. **A sweep result is only as wide as the command
  actually typed.**
- The literal-collision accounting was wrong TWICE, and each time the
  cause was in the checker: first a function-body splitter that broke
  on `///` lines rather than braces, so literals leaked between
  neighbours; then a tokenizer that stopped at `5e-` and read the
  exponent as its own token, manufacturing a `4`. Under Rust's
  tokenization exactly one literal is shared, `72`. The item now states
  the tokenization BEFORE the list, because **a shared-literal list is a
  function of how you tokenize, so the tokenization is part of the
  claim.**

The lane kept both wrong passes on the record rather than tidying them
away, under a heading that says "verified rather than assumed". That is
the right call: a corrected paragraph that hides having been wrong is
indistinguishable from one that was right the first time.

**And a live site nobody had found**: `tools/tess-lint/src/lib.rs:2682`
says *"Six of the 64 sized rows of the committed baseline sit exactly
there."* `64` is asserted at `baseline_census.rs:437`; **"Six" is
asserted nowhere at all** and is spelled as a word, so no figure sweep
finds it. A re-cut moves it and nothing fires — worse than the document
copies, which at least red a test somewhere. Found by reading, not by
pattern.

### Program state

Eleven units merged (0-12 less the two that became other things), four
of them today. **The slate stands at 40 files, 24 open** — it opened at
eleven. Every one of today's four units closed on a MAJOR that was a
false CLAIM rather than broken code; not one shipped a defect in an
assertion. What is left is the exit walk, and it must RULE rather than
drain.

## The dead pointers routed, and the exit walk planned (2026-09-08)

**Routing.** Unit 8's dead-pointer sweep found `D103` and `D107` cited
at `work/code-quality/` paths that no longer resolve — both rows moved
in the 2026-09-06 cut, to `work/gates/D103.md` and `work/topo/D107.md`.
Verified here: five files for `D103` (eight occurrences), one for
`D107` (two). The sharpest is that the `D107` citations are inside
`work/code-quality/` itself — a row citing a sibling that is no longer
its sibling. `work/code-quality/` still exists and still holds other
rows, so the directory being present is not evidence a path in it
resolves, which is presumably how these survived.

**Filed to `work/meta/`, and I filed it in the wrong place first.** I
put it in `work/issues/` reasoning that no single program owns all six
sites. That is wrong on `work/README.md`'s own terms: `issues/` is for
an owner *undecided or disputed*, not *plural*, and the page says an
unsorted pile of related items there is what the sweep exists to
prevent. The right home was findable rather than inferable — META's
`keep_out` says in as many words that *"a stale citation in another
program's slate is routed to its owner and never fixed across the
fence"* and NAMES the standing instance,
`stale-track-t-citations-in-fillet-and-cert`, which is the same shape
held in `work/meta/`. Moved. The day's rule applies to conventions too:
the precedent was written down and I reasoned about the shape instead
of reading it.

**The walk is planned in `plan.md` and deliberately not started.** The
slate is 23 open items against eleven at opening, and it grew from real
findings rather than drift. The central ruling: **roughly nineteen of
the twenty-three are `tools/*`** — six k-lint, nine tess-lint/tess-meter,
four `docs/TESS-BUDGET.md` — and `work/README.md` says a dozen items on
one territory are a successor's opening slate, which the closing
program opens. So the walk opens a successor for the instruments rather
than scattering them; nineteen coherent rows in `work/issues/` are
exactly the pile that page forbids.

Four are not the successor's and want their owner checked against that
program's `paths` rather than guessed: `C15` (corpus-side, `demos/`,
Track X), the reader-census disposition (`crates/test-utils/`),
`cert1-notes-pr-body-tracked-on-main` (hygiene on `main`), and the
`tools/README.md` ratification, which cannot be re-homed until Ev
rules on #2147.

**Opening a program is not the orchestrator's call alone.** The walk
should propose the successor — charter, `paths`, fence, opening slate —
and go to Ev, the way `D201` and the `CC1`–`CC5` ratification did.

## Ev asked what the CC scope question costs each way (2026-09-08)

He asked for concrete consequences rather than more argument, and the
delay helped: two lanes have used the page since, and one of them is
the case that decides it.

**The tension is textual and sits inside `CC5` itself.** Its first
paragraph is reading-general — every clause exits in the harness
voice, each lint spells that constant privately in its own `main.rs`,
*"the voice is one thing and the constant is two, for the same reason
nothing else crosses these cargo roots."* Its second paragraph scopes
the page: *"What this page owes is what falls out of it for a
cross-column reading."* One clause, two scopes.

**The live instance is `tools/k-lint/src/main.rs:44`** — `EXIT_FINDINGS`,
an exit-code convention with no column in it, citing `CC5` across
roots for the voice rule. Its neighbour at `:51` (`EXIT_HARNESS`,
*"Every cross-column admission `lint_csv` refuses leaves here"*) is
column-scoped and correct under either reading. So the two readings
are not equally supported by the tree as it stands: one site already
depends on the broad one.

What each costs, as sent: narrow makes `:44` a mis-citation and leaves
the next non-column check in `k-lint` or `tess-meter` with no route —
it cannot reach `tess_lint::Report` across the root, so it writes a
second page, which is the failure the page exists to prevent. Unit 4's
`cut_line_pin.rs` survives narrow only because it is in the same crate
as `Report`. Broad makes `:44` correct and costs one editing pass to
label `CC2`/`CC3`/`CC4` as per-column, since those three genuinely do
not generalise.

**Recommended broad, on the asymmetry rather than the elegance:** the
narrow reading's cost is paid by a future lane, is invisible until
someone writes the second page, and is the exact failure mode the page
was written against; the broad reading's cost is paid now, by me, and
is bounded. Flagged that if he takes narrow, `CC5`'s voice paragraph
should be split out in the same PR rather than left to be found later.

Fifth comment from me on that PR, and the restraint I had set myself
("four is enough") was against volunteering more, not against
answering a direct question from Ev.
