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
