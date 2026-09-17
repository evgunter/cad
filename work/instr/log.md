# INSTR log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/instr/plan.md`.

## Opened (2026-09-08)

Opened by METER's exit walk, `docs/METER-EXIT-WALK.md` §4, ratified by Ev
on 2026-09-08 at PR #2212 (*"1. open"*; then *"the new 3, instr, and your
plan all sound good!"*). This is step 3 of the walk's four-step close: the
walk merged, the discipline diff rides its own `[ev]` PR, this PR opens
`instr` and moves the residue, and the sweep after it deletes
`work/meter/` and the walk itself and records both in
`docs/DOC-LEDGER.md`.

**Twenty rows moved by `git mv` from `work/meter/`**, ids unchanged, each
carrying a `## Moved to INSTR (2026-09-08)` record in its body. The list
was re-derived from the tree rather than taken from the walk: the open
rows in `work/meter/` at `origin/main` `b36a8d4b8`, less the two §5 rows
that go elsewhere, is exactly the walk's nineteen instrument rows plus
`C15` — twenty, diffed line for line against §3's enumeration with no
difference. METER's slate was 37 items, 15 closed and 22 open, at that
SHA; the sweep that counted them reads the `status:` line of the files in
that directory and cannot see an item on an unmerged branch, an item
METER filed onto another program's slate, or a residue disclosed in a
merged PR body and never given a file.

A/B band **3300–3399** claimed in `docs/MODEL-AB-LOG.md`'s banding entry
in this same commit, per the rule that entry states. No A/B protocol runs
on this program until Ev says one does; the band is bookkeeping, METER's
posture inherited.

`work/meter/` is NOT deleted here and neither is the walk — that is the
sweep, a separate PR. What remains in that directory after this one is
its fifteen closed rows, the row this PR closed by deleting
`.cert1-notes/pr-body.md`, and the three program files.

**One hazard found while executing, and it belongs to the sweep, not
here**: six of the twenty carry `refs:` naming closed METER rows
(`D201`, `cut-prefix-three-unpinned-spellings`,
`k-lint-predicate-roster-unpinned` twice, `k-report-baseline-fold-cert1-roster`,
`tess-lint-twinned-csv-fixture`). `scripts/work.py lint` requires every
`refs:` id to resolve, so deleting `work/meter/` reds `main` for rows this
program owns and METER's sweep PR cannot touch — the cross-program cost
`work/README.md` already names for `parked` rows, in a second shape. It is
filed as an issue on METER's slate so the sweep sees it before it fires
(`work/meter/sweep-deleting-work-meter-dangles-six-refs-on-instr-rows.md`),
and the form is not a new decision: GATES answered the same hazard at its own
sweep the same day (`3a8dd05fe`) by putting the closing PR's number in the
`refs:` list, where lint does not check it, and saying so in a section at the
citing row. The five ids map to PRs 2167, 2151, 2115, 2140 and 2179, derived
twice; the row carries the mapping and the derivation. The sweep makes the
substitutions in its own commit, as GATES' did across three programs' rows.

## Orchestrator handover and the plan re-cut (2026-09-16)

New orchestrator on the program; nothing had been dispatched since it
opened, so the resting state was twenty-five rows all `open`, no
`instr/*` branch on origin and no PR. Four rulings from Ev on the
handover, all recorded in `plan.md` rather than here, because they bind
the lanes and this file is narrative:

1. **No A/B protocol, style review per unit, three full reviews** —
   units 7, 15 and 18. The question METER's walk §8 left open is now
   answered rather than inherited. Unit 5 was weighed for a fourth full
   review and ruled style-only: *"no need to include
   tess-lint-ungated-columns-fold-silently"*.
2. **S-MESH is reachable but not in active development**, so unit 10's
   producer-side column may be written here: *"for s-mesh you may be
   able to just do it since it won't collide with anything"*. The
   crossing is announced, not claimed — `crates/mesh/*` stays S-MESH's
   `paths` and `program.md`'s `keep_out` clause says so in its new form.
3. **PROPS is live.** Ev can carry a message, and *"if the item would be
   better sited in props itself then you can just move it there"* — so
   unit 17's ask either rides an `[ev]` message or `git mv`s onto
   PROPS' slate, and does not park.
4. **`[ev]` PRs open early and the rest of the work does not wait on
   them**: *"i'd suggest you open them sooner and just work on
   everything that doesn't wait on them until i reply."* Unit 16's
   roster-membership question is the first one that qualifies.

**The five rows that arrived after the plan was written are placed.**
`pin-table-has-no-whitespace-tail-row` joins unit 9 (both read the cut
line, and the second is one truth-table row in a file the first is
already editing); `k-report-bit-identity-claim-has-no-citation` joins
unit 13 (one document lane against `docs/K-REPORT.md`);
`no-guard-reds-a-decide-name-missing-from-the-k-report` and
`frame-mint-funnel-names-outside-every-sweep-corpus` become units 15 and
16 of lane C; `c15-is-dischargeable-now-that-a-sized-scene-carries-names`
joins `C15` itself.

**`C15` stops being "last, and not in a lane".** Its own evidence row
says the corpus change it waited on has landed: the teapot's spout is a
lofted canal, the first sized scene carrying durable per-face names, and
four pairs of its laterals are indistinguishable by every
`IDENTITY_COLUMNS` entry and separated by their names. So the charter
sentence in `program.md` and `plan.md` saying it *"waits on a corpus
change no lane here makes"* was false on the tree and is rewritten in
this commit. It is lane D, with the full review.

## The PROPS pair splits, and one half leaves the slate (2026-09-16)

Ev's handover ruling gave two options for the pair INSTR had held since
opening — a message carried to PROPS, or a move — and the two rows
turned out to want different ones.

`k-lint-roster-wants-a-kernel-side-vocabulary` **moved to
`work/props/`**. Its whole content is a kernel-side ask: a `#[test]`
inside `geom-brep` over its own minted names, or the better shape, a
`const` slice exported from `geom_core::k_stats`. Both are edits under
`crates/`, both on PROPS' seam, and neither is INSTR's to make. The row
was here because METER's fence chose the instrument — which is the
finding the row exists to record, not a reason for it to live here. A
message would have described the ask from a slate that cannot act on
it; the move puts it where it is dispatchable, and reaches PROPS'
board through `STATUS.md` rather than through anyone's inbox.

`k-lint-eps-coupled-criterion-unwritten` **stays, now `parked` on it.**
Its title is already the instrument's half — *"so k-lint's roster cannot
be pinned against the kernel in the ADDED direction"* — and the pin is
`tools/k-lint`'s, this program's ground. So the pair did not need a
third row for the follow-up: this row IS the follow-up, and it was
mis-shelved as `open` rather than parked while its trigger sat on the
wrong slate.

Net: unit 17 stops being two rows describing one PROPS unit, and
becomes one row waiting on one. The plan's unit 17 is rewritten to say
so.

## Unit 4 lands its review; two corrections travel further than the unit (2026-09-16)

**The unit.** `baseline_census`'s `constant.len() + discriminating.len()
== IDENTITY_COLUMNS.len()` is deleted rather than replaced. The item
offered a replacement — *"that `distinct` has no zero entry"* — and the
lane established on the code that the same `assert!(!sized.is_empty())`
guard makes BOTH unfailable, so the replacement would have been the
defect with a new message. Style review upheld both claims `sure`.

**The review caught the trap in the assertion the unit KEPT.** Given
`distinct[i] >= 1`, once the `constant` assert passes the
`discriminating` one is forced — `IDENTITY_COLUMNS` minus those five, in
order — so no re-cut can red it alone. It stays reachable under a source
edit adding an eighth column, so it is not dead; what it owed was the
disclosure this file already gives three times over for exactly this
shape. The PR body's *"say strictly more"* was half-false. This is the
precise shape `docs/REVIEW-STYLE-DISPATCH.md` §2 names and the reviewer
brief says only a non-author has ever caught, and it was worth the
review on its own.

**Two findings reach past this unit.**

`cut_line_pin.rs`'s Class-section precedent is half wrong, and the
review verified the correction independently: PR 2151's deletion is
real, but a DIFFERENT `TABLE.iter().any(…)` survives today in
`the_committed_baselines_own_cut_line_is_a_row_of_the_table`, reading
`BASELINE`'s first line at runtime and failable on a re-cut. **Unit 9
is briefed on this as fact** — reading the precedent as "none survives
there" would have deleted a live guard in the file unit 9 edits.

The item's stated sweep pattern cannot match the item's own instance:
two passes written faithfully to *"operands are `const` items or derived
from them alone"* returned the defect not at all, because
`constant`/`discriminating` descend from `parse(BASELINE)` at runtime
and the defect is a tautology rather than a compile-time constant. A
lane running the stated pattern gets a clean result over the very defect
it was dispatched for, and a clean sweep reads as a negative result
rather than as a wrong instrument. Filed on META's slate as
`an-items-stated-sweep-pattern-may-not-match-its-own-instance`, because
`docs/prompts/*` is META's and the two fixes that would close it are
`[ev]` conversations.

**Unit 12's plan entry was wrong and is corrected here.** It called the
row *"one name … the cheapest evidence that the roster's ADDED direction
is unpinned"*. It is not: the omission is already loud via
`NOT_ROSTERED` plus a guard, and what is left is a ruling whose sample
branch has no corpus — `props_quad_last_round` emits zero rows in all
nine committed baselines, measured before dispatch. The lane was
briefed on the corrected shape rather than the plan's.

## Unit 0 closes; the sweep found more than the row it closed (2026-09-16)

`baseline-sizing-census-pointers-stale` is CLOSED. Four of its seven
cited mentions had resolved **by deletion** — every `work/meter/` row
went with METER's closing sweep — so the live list was three, and the
lane re-swept rather than trusting the row. The headline count is kept
as the figure the row reported on the day it reported it, with a dated
re-sweep section carrying the live list; the repair for the four is a
`docs/DOC-LEDGER.md` sweep-10 citation plus the SHA the ledger
publishes, so the line numbers resolve at the tree they were taken from
instead of nowhere.

**The review found the unit breaking its own rule.** The lane edited
one dated record's body on the doctrine *"the FIGURES are frozen; the
PATHS are not"* and left five of that same record's pointers dangling,
one of them pointing at the lane's own row. Fixed on the rule rather
than around it.

**Three things outlived the unit.**

`docs/TESS-BUDGET.md`'s four sizing figures are not merely a second
copy — they are WRONG. The baseline was re-cut on 2026-09-15 and the
prose did not follow. Summed from the committed CSV at adjudication:
`grid_cells` 88,036, `patch_cells` 147,960, `opt_cells` 127,966,
`span_opt_cells` 76,599, 1605 rows, 362,154 triangles, against the
document's 46,019 / 110,811 / 93,066 / 44,162 / 1353. This is the
failure `baseline-sizing-census-second-copy` PREDICTED in as many words
— *"the alarm fires and the document stays wrong"* — and that row does
not know it has happened; the row is also stale about itself, naming
the superseded sums and greping for them. **Unit 1 goes next and carries
the verified figures rather than the item's.**

The dangling-`work/meter/` class is live in `tools/` **code doc**, not
just in the tracker: thirteen citations across five files, one of them
inside an assertion string. Filed as
`tools-doc-prose-cites-thirteen-dead-work-meter-paths`. The crates-side
half went as evidence onto the existing
`work/issues/dead-work-citations-from-shipped-code-and-docs` rather than
a new row, and carries two things that row needed — its proposed gate
is scoped to `crates/**`, so it would have caught none of the thirteen,
and `tools` is `exclude`d from the workspace, so a gate written as a
workspace test cannot see them even with the glob widened.

A `mergeable_state: dirty` PR gets **no Actions run at all** — no
failing check, no error, twenty minutes of polling a silence. CIW
already owns it (`dirty-pr-gets-no-actions-run`); evidence added there
rather than as a fourth row, and the duplicate open row CIW opened the
same day is named on it for its owner to judge.

**Two corrections to the orchestrator, both upheld.** The tools/ file
count was five, not the four I briefed (`tools/README.md` missed), and
the crates-side finding belonged on an existing row rather than the new
one I asked for — the discipline's grep-first rule pointed at a row
open since 2026-09-13 with a live design argument the evidence bears on.

## Unit 4 closes; the review caught the fix reproducing its own defect twice (2026-09-16)

`baseline-census-partition-assert-cannot-fail` is CLOSED. The
partition assert is DELETED rather than replaced: the item offered
*"assert that `distinct` has no zero entry"*, and the same
`!sized.is_empty()` guard three lines above makes both unfailable, so
the replacement would have been the defect with a new message.

**The style review earned its dispatch twice over, and both times on
the assertion or the prose the unit KEPT rather than on what it wrote.**

First: given `distinct[i] >= 1`, once the `constant` assert passes the
`discriminating` one is forced to be the rest of `IDENTITY_COLUMNS` in
order, so **no re-cut of the baseline can red it alone** — the pair is
one assertion against a change in the DATA, not two. The PR body's
*"say strictly more"* was half-false for exactly the reason the deleted
assert said nothing. It stays (a source edit adding or removing a
column still reds it), and it now carries the disclosure this file
already gives at three other sites.

Second, and this is the one worth remembering: the fix pass's FIRST
reword of the doc comment moved the partition claim from the assert
into prose — *"between them they name the whole of `IDENTITY_COLUMNS`"*
— a static claim about two literals that no code computes, in a file
whose own module docs say *"a number transcribed into prose is a number
nothing can check"*. The lane did not catch that re-reading its own
diff. `docs/REVIEW-STYLE-DISPATCH.md` §2 predicts exactly this and the
reviewer brief says only a non-author has ever caught it; both held.

**Six rows filed, one of them a correction to this orchestrator's
adjudication.** I told the lane to add the `C15.md` evidence to
`tess-budget-doc-identity-column-list` rather than open a row; the lane
flagged that the subjects differ (membership of the identity list
versus the split among sized rows) and asked me to re-check. It was
right, and for a reason neither of us had named at first: **that row is
unit 2's and closes when unit 2 lands**, unit 2 will never touch
`C15.md`, and `work/README.md` holds that a residue inside a closed
item's prose *"reads as a record of work done, not as an open thread …
and dies with the directory"*. Split into
`c15-transcribes-the-sized-row-identity-split`, with a pointer left on
the membership row.

The lane also corrected a count I had passed through from the review
without checking: the scene-set derivation has THREE sites, not five —
`by_totals` derives recoverable-`SceneTotals` scenes and `scenes`
derives every scene, different sets in the same idiom. Filing my
framing verbatim would have put a wrong count in the tracker. The
dispatcher's own exposure, exactly as `docs/REVIEW-STYLE-DISPATCH.md`
§3 states it.

## Unit 12 closes, and the full review it was upgraded to caught two shipped falsehoods (2026-09-16)

`k-lint-last-round-is-eps-coupled-but-unrostered` is CLOSED.
`props_quad_last_round` stays off rule (4); the ground is that it has
**no draws**, and that is now measured rather than inferred.

**The plan called this unit "one name, the cheapest evidence", and the
posture called it style-only. Both were wrong, and the unit was
upgraded to a full falsification review mid-flight.** It earned it: the
review falsified two arguments the unit had already SHIPPED, one inside
a `const` string that goes out in the library and one in
`docs/K-REPORT.md`.

- **The second-floor asymmetry was false about the family it contrasts
  with.** The unit argued `props_quad_last_round` is distinguished by a
  refusal side unbounded below, where the rostered family's headroom is
  bounded by the target. `props_quad_converged` carries 12 positive and
  **24 negative** rows at `m7-eps-1e-9` and 12/**48** at 1e-12, reaching
  `|m| = 1.83e-4` ≈ 1.8e8·ε — and the P0 the floor is cut from is
  itself a negative row. The property named as distinguishing is one
  both families have. The ruling survives on the no-draws ground alone;
  the argument was deleted from all three homes.
- **The residue's justification was contradicted by the kernel in the
  kernel's own words.** The unit wrote that a purely-refusal population
  needs no guard because a definite negative refuses the face, so it
  would be a red suite long before a lint row.
  `crates/topo/src/props.rs` says a face refusing on BUDGET *"has an
  enclosure, and the sum keeps it — so the refusal rides on the
  certificate, and whether it is REPORTED is the caller's decision,
  taken by `last_word`"*, and `last_word` is asked only when `settle`
  never accepted. Negatives can accumulate on a fully green suite.

**The fix took the stronger branch than the one adjudicated.** Rather
than correcting the sentence and filing the hole, the lane made a row
for an `EPS_COUPLED_UNRULED` name a **finding that fails the run**,
riding with rules (2) and (3) on the demotable side so the E6 driver
row still works. Gating on the NAME is the only statement independent
of a row's sign or magnitude, so one change closes the refusal-side
hole and the unstated `last_round_len >= 0` premise together, and the
two rows that branch would have owed collapse into one narrower row on
PROPS' slate. The verdict names the ruling and says outright that
re-deriving `BASELINE_FLOOR_MARGIN` is NOT the recourse.

**"Three guards" was two guards and an unexercised printer.** The
review found `Scan::unruled` and its CLI note had no test anywhere, in
a crate where every other rule, cap and threshold is pinned. It is now
pinned twice, and the phrase is gone from every document. The roster
guard also read `EPS_COUPLED_UNRULED[0]`, so a second entry would have
had no guard at all; it iterates now.

**The gate inference and the measurement's framing were both narrowed.**
A green says something about the three gated `k-fresh` files and nothing
about the M2 dump (read by nothing) or the driver dump (demoted); and
the zero is structurally determined — the mint is reachable only from
the two patch lanes, and in the committed era the round-0 failures sit
in the lane without it — so 1.26M samples is the run's scale, not its
coverage.

**Bookkeeping this closed.** Unit 12 repaired six of the thirteen
dangling `work/meter/` citations unit 0 filed, withdrew its own
duplicate row, and contributed the argument that decides the repair for
the remaining seven: the ledger's *A note on inbound references*
settles DELETED files, while these name rows that moved and are open,
where the recovery recipe yields a superseded snapshot — a pointer that
resolves and lies.

## Ev rules the roster's membership rule (2026-09-16)

The first `[ev]` PR of this program (#2733) asked whether a test-support
name is K-roster material. **The question was wrong and Ev's first reply
said so**: *"isn't everything that reaches the funnel dependent on our
particular demos and tests?"* It is — every sample in the distribution
comes from running our demos and tests, for every rostered row — so
*"reaches the sweep's corpus?"* divides the corpus, not the names, and a
roster built on it is a property of what we ran last.

Re-framed on the axis the tree already states: **whose decision the name
records.** `sketch_plane_frame_norm`'s own doc says the pair reaches it
*"from a user's Python call"*; `crates/sweep/src/lib.rs` gates
`test_support` behind `#[cfg(any(test, feature = "test-support"))]` and
`#[doc(hidden)]`. So `fixture_frame_axis` is not a name the corpus
happens to miss — nothing outside a test build can reach it at all.

**Ruled (a): the roster records decisions the kernel can be ASKED to
make.** `fixture_frame_axis` leaves; `sketch_plane_frame_norm` stays
with its coverage gap as the finding; and — the part larger than the two
names — `docs/K-REPORT.md`'s stated rule (*"from anywhere the sweep can
execute"*) is reading (b) and is now wrong. The `chart_bound_*`
paragraph already practises (a). Unit 16 rewrites the stated rule and
keeps the coverage column as the behavioural half; `K-REPORT` is
`Reference`, so that edit is the ruling landing rather than a second
conversation.

**Worth keeping as process, not just as content**: the useful output of
this `[ev]` PR was not the answer to the question asked. It was Ev
rejecting the question's premise, which turned a one-name membership
query into a ruling on the roster's meaning and exposed two sentences in
the governing document that disagree. Opening it early — before unit 16
was anywhere near dispatch — is what made that cheap.

## Unit 1 closes: the document was not carrying a copy, it was carrying a different cut (2026-09-17)

`baseline-sizing-census-second-copy` is CLOSED, and the row's own
diagnosis was too kind to the tree. The four figures in
`docs/TESS-BUDGET.md` were not drifting copies of a live number — they
were a **coherent reading of one superseded cut**, stamped
`3f55f361b22e` and summable at `715977e6a`: 1353 rows, 64 sized, 286
named, 46,019 / 110,811 / 93,066 / 44,162. Every derived factor in the
document (3.52x, 3.35x, 1.042x, 8.5x) came from those operands. The
census was re-pinned five times between 2026-09-10 and 2026-09-15 while
the prose stood still. This is the failure the row PREDICTED —
*"the alarm fires and the document stays wrong"* — arriving while the
row slept.

**The fix shape, and why a pointer could not do it.** One executable
home; live claims cite the census tests; and the historical comparison
is FROZEN and labelled, because a comparison between two named trees
cannot be cured by a pointer — you cannot divide a pointer by 390,100.
The document's frozen-exception enumeration went from two to three.

**The `name` bullet's conclusion was backwards, not merely stale.** It
said coverage is disjoint from the case a join would fix.
`teapot/teapotspout` carries 8 sized rows, all 8 named, and four pairs
separated by `name` — which is `C15`'s own dischargeability evidence.
Two doc-comment leads stated the same falsehood directly above the
assertions refuting it.

**The orchestrator was wrong about this unit, in the most confident
possible way, and a lane refused the instruction.** A
`# tess-budget-cut:` line names the tree the sweep READ, not the commit
whose blob holds the rows. So `git show <cut>:…baseline.csv` returns
the PREVIOUS baseline — parsing cleanly, same shape, different sums, no
error. Checking the unit's label that way produced 94,154 / 44,446
against the document's 93,066 / 44,162, and the lane was instructed to
re-point at `448275c8d`. **That would have been wrong**: `448275c8d`
agrees on all four sums and carries 316 named against 286, so it cannot
be the `name` bullet's source. The lane re-summed, found the
stamp-versus-tree distinction, and refused. Recorded as measured
evidence on `cut-line-commit-names-no-baseline-change`, whose two known
failure modes both end with the reader KNOWING something is wrong; this
is a third that ends with them confidently holding wrong numbers.

**The repair for that trap is the unit's, and it is better than either
option the orchestrator offered**: every cut citation is two-part — the
stamp AND the commit to sum — with the re-sum command beside it and an
explicit warning that the stamp's own SHA holds the previous file.

**Then the fix minted its own defect, in the sentence explaining the
defect.** The diff wrote *"64 there, 80 now"* into the passage arguing
why the comparison had to be pinned. `80` is a current reading asserted
at `baseline_census.rs`'s face-identity census, unlabelled, citing
nothing — four hundred lines below the document's own rule that **no
count over this file is written as a CURRENT reading**. The unit's own
re-derived sweep declares it cannot see `sized == 80`, so the sweep
written to prove the class closed could not see it re-opened. Only the
style review caught it. Filed as
`no-guard-reds-on-an-unlabelled-figure-in-the-budget-doc`, which
records F1 as the hole's first measured instance, produced by the
commit that disclosed the hole.

**The file also stated a proposition and its negation** 170 lines
apart — the rewritten header saying the document carries no current
figure, `## What it is for` still saying it carries all four in
present-tense prose, which was that test's entire justification. Fixed,
and the class swept by reading all six named sites (five checked and
still true).

**The rename guard's reach was overclaimed and is now split.** The
grep found ELEVEN unguarded-by-prose citation sites, not the three the
review named — including two in code, one of them
`report_columns_pin.rs`, the citation unit 0 minted when it re-pointed
the dead path, in the same `tests/` directory as the guard. The two
arms are now documented as unequal: the rename arm (a path expression)
covers every citation anywhere; the prose arm reaches
`docs/TESS-BUDGET.md` alone. **A tracker row does not belong in a
test's guard list** — a row is deleted with its program, so wiring one
in reds on an ordinary tracker edit and again at program close. That
was the open question and the lane answered it with the reason.

And the lane declined to write the count of unguarded tracker rows into
the prose, because *"that count is exactly the kind of figure this unit
exists to remove, and F1 is what putting it there looks like."*

## Unit 5 closes: a garbage value reached the gate and the gate reported clean (2026-09-17)

`tess-lint-ungated-columns-fold-silently` is CLOSED, arm by arm.

**The constructed case is stronger than the row claimed, and the lane
built it before writing anything.** One sized row of the committed
baseline with `muu=banana`, linted as the fresh file against that same
baseline: `0 finding(s)`, *"clean — no scene grew and no face's sizing
got wastefuller"*, **exit 0**. The gate did not merely ignore the
column — it read the file, compared it, and reported clean. After the
fix: `malformed budget row (harness breakage)`, exit 1. The reviewer
reproduced both directions against the real gate.

**Arm 1 fixed** at the parse boundary; **arm 2** (`name` parsed and
read by nothing) ruled unit 18's, since what it lacks is a rule that
reads it and that is `C15`'s job; **arm 3** split out as
`tess-lint-re-cut-folds-uncompared-columns`, carrying a sharper first
move than "build the diff": `tess-lint` cannot today tell a re-cut from
a gate run — both arrive as `<fresh> --baseline <committed>` — so a
diff beside the verdict would print on every green run over what the
row's own table says is mostly last-digit noise. Settling who the diff
is for comes first.

**The style review produced this program's first DEMONSTRATED finding
rather than an argued one.** `Certificate`, `Sup` and `Count` were
three spellings of `finite && >= 0.0`; the reviewer scrambled
`BOUND_COLUMNS` across them and the whole suite stayed green, 59 + 31.
Nothing in the crate could tell them apart. **The fix pass then found
it was worse**: `Target`/`Aspect` were two more spellings of
`finite && > 0.0`, which neither the review nor the orchestrator had
named. The predicate is now the variant and the quantity is data —
`NonNegative(&str)` / `Positive(&str)` — with messages verified
byte-identical through the CLI, and the mutation's surviving
cross-policy form now reds four tests.

**The census asserted completeness it did not have.** Its failure
message claimed a *claimed* column does not reach the gate carrying
whatever the file says — false for `name`, which it claimed, `parse`
stores and no rule reads: arm 2, the arm the unit declined to fix. A
hand-maintained list asserting completeness about a hand-maintained
list, failing open. Now split into `Claim::Read` / `Claim::Carried`
with `assert_eq!(carried, ["name"])`, which fails in both directions.

The lane declined one design call and said so rather than taking it
silently: making the census REFUSE to claim a carried column collapses
the gap check into the read check, and the gap check must cover every
header column — including one nothing reads — or it stops being a
cover. Adjudicated as correct.

**Two more homes for "what the gate reads", filed on CIW.** The sweep
script's copy went stale *because of this diff*, and `ci.yml` asserts
the roster lives in `tools/tess-lint`'s module docs *"AND NOWHERE ELSE,
this file included"* and then states a copy in that same file — a
sentence false in its own file, which is the sharper half.
`work/ciw/gate-reads-roster-has-two-copies-in-ciw-files`, filed after
grepping 114 CIW rows and finding the nearest neighbours are the same
shape on the cut line.

**On the orchestrator's instruction about `C15.md`**: the note said
both "you were right not to edit another unit's file" and "check the
note lands where a unit-18 lane looks". The lane resolved the
contradiction by appending a dated, additive section that does none of
unit 18's work and leaves the re-pointing undone, and offered to revert
it. Kept — the instruction was ambiguous and this reading is the useful
one.

**Posture note.** `plan.md` names the census-asserting-completeness
shape as the reason unit 15 gets a full falsification lane; unit 5 was
ruled style-only and the style lane caught it anyway. That is the
second time a style review has caught something the full lane was
reserved for. Not a reason to reopen the posture, recorded as evidence
if it is ever revisited.
