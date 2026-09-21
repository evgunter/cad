# VNEWS — the viewer's news vocabulary (plan)

**STATUS: OPEN (2026-09-17).** Opened in VIEW's re-scope, on fourteen
rows that arrived by `git mv` with their bodies unchanged. Live state
is `work/vnews/log.md`'s tail and the item files beside this plan,
never this file.

Branch prefix (the #396 convention): **`vnews/`** — unit branches
`vnews/<unit>-<slug>`, orchestrator branch `vnews/orchestrator`.
Away-channel tag `(VNEWS orchestrator)`. A/B ordinal band
**VNEWS = 5200–5299**.

## Charter

**Every row here is a defect in the vocabulary a fact travels in on
its way to a reader — never in the fact.** The viewer works out
something true: this control cannot be used, and why; this frame
refused, and what else it had to say; this seat list has three
entries; this evaluation is outstanding. Then it spells that fact as a
bare `bool` with the reason kept somewhere else
(`environmental-facts-answer-usable-as-a-bool-…`,
`is-instance-collapses-absent-and-wrong-kind`), as a literal beside a
shared constant that exists (`seat-line-spells-the-list-mark-as-a-
literal`, `converged-recourse-has-no-home`,
`the-new-document-button-states-its-refusal-twice`), as a raw variant
identifier where a word was owed
(`viewer-preview-names-a-verb-by-its-variant-identifier`), as one of
two enums that are the same enum
(`outstanding-and-progress-are-two-three-state-enums-one-hop-apart`,
`ranked-and-unranked-verdicts-are-one-type`), as a value in one file
and a comment in two others
(`tone-is-a-value-in-frame-and-a-comment-in-two-panes`), or as a rank
that silently drops its siblings
(`rank-one-discards-the-frames-other-news`,
`one-line-one-subject-loses-a-mixed-frames-expiry`).

**The test that separates this program from its three siblings.** A
VNEWS fix changes the TYPE or the DOOR a fact arrives through; it does
not change the fact, and nothing it touches survives the frame that
produced it. That is false of VGEOM, whose rows are about the VALUE
being wrong rather than the word carrying it; false of VSEAM, whose
rows are about state that outlives its frame and the boundary that
owns it; and false of VDOC, whose rows change what the tree says about
itself and change no viewer behaviour at all.

Applying it the other way, as this program's own rule about splits
demands: **a row belongs here only if a reader would see the
difference.** A rename nobody reads is not news.

## Order

E-first. Nothing here is blocked, nothing here waits on Ev, and the
three sibling successors are file-disjoint from this one except at the
shared files `program.md`'s `keep_out` names — so this program is
dispatchable in parallel with VGEOM and VSEAM from its opening day.

1. **The literals that have a home already** —
   `seat-line-spells-the-list-mark-as-a-literal` (`frame::LIST_SEPARATOR`
   exists and `seat_line` does not use it),
   `the-new-document-button-states-its-refusal-twice` (a literal beside
   a comment naming the `Refusal` variant that should back it, the two
   sentences differing), `converged-recourse-has-no-home` (two literals
   in two crates held in step by a test). Three rows, one class, one
   shape of fix; the third crosses a crate and announces.
2. **The collapses** — `is-instance-collapses-absent-and-wrong-kind`
   and `environmental-facts-answer-usable-as-a-bool-with-the-reason-
   elsewhere` are the same defect at two doors: a `bool` answered
   because one caller wanted one, with the reason kept elsewhere or
   nowhere. Take them together or the second re-mints the first.
3. **The two-enums rows** —
   `outstanding-and-progress-are-two-three-state-enums-one-hop-apart`
   and `ranked-and-unranked-verdicts-are-one-type`. Both carry an
   undecided fork (collapse, or name the difference), and the second's
   fork decides what the first collapses INTO, so they are one
   conversation. `outstanding-…` reaches `session.rs` and `pickcache.rs`,
   which are VSEAM's: announce.
4. **The discards** — `rank-one-discards-the-frames-other-news` and
   `one-line-one-subject-loses-a-mixed-frames-expiry`. Both are about
   news the ranking throws away rather than spells; the first names a
   discarded free-move placement as unrecoverable, which is the row's
   own argument for taking it before the other.
5. **The rest, unordered** —
   `a-fold-row-composes-a-producer-with-a-dead-door`,
   `document-news-has-no-home`,
   `tone-is-a-value-in-frame-and-a-comment-in-two-panes`.

**Not scheduled, and why.** `document-news-has-no-home` is a census of
a class before it is a fix, and the VIEW register's rule about a
population applies: re-derive the population by subject before naming a
door, because the count in the row is evidence only as of its filing.
`a-disabled-control-says-why-in-four-shapes` was the other, and it has
now RUN — see the group below.

### 6. The disabled-control family, after the census (2026-09-19)

`a-disabled-control-says-why-in-four-shapes` ran first of all the rows
here, because its rule decides the disposition of three others and
deciding any of them alone would settle the class from its easiest
instance. It is `review` at PR #2908, with a style review, a
correctness review and a fix pass against their union.

**What it established** (subject to the fix pass landing): a control a
reader cannot use owes the sentence a click would have been answered
with, **where there is such a sentence** — and the tree has a third
case the rule as first written did not name, a chrome-policy gate where
the operation would SUCCEED and the control declines anyway
(`pane/create.rs`'s bore-against-radius arm, whose own literal says *"a
larger bore would swap the roles rather than refuse"*).

**It overturned both predictions its own row made.** Neither
`pane/create.rs`'s `blocked: Option<&'static str>` field nor
`platform::NO_CHOOSER_BACKEND` is a hit — the first gates a draft, the
second sits on buttons that push no op — and the genuine hits were
mostly elsewhere. That is the argument for running a census before a
fix, stated by the one case where it paid.

**The family's order now:**

1. `undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words`
   — the sharpest of the three hits: `Session::step` refuses
   `Refusal::NothingToDo` on exactly the conditions the two buttons
   gate on, the buttons carry no words at all, and the review
   established that those buttons are the only hand — no keyboard
   route pushes `Undo`/`Redo` — so the refusal's sentence is
   unreachable from the chrome entirely. `app.rs` is VSEAM's: announce.
2. `the-range-button-re-mints-the-ratified-affordance` — a third
   spelling of `Refusal::affordance`, whose doc says it has one home,
   in a panel that calls the helper fifty lines up. Its second conjunct
   hands a *"computed slot"* sentence to a slot that is not computed;
   whether that state is reachable is open and the row says so.
3. `the-new-document-button-states-its-refusal-twice` — **answer 1,
   read the refusal**, which is what the census's rule gives it. Its
   stated cost (*"the refusal's sentence is written for a status line"*)
   is the objection `Refusal::exists_wording` exists to answer.

**Not in the family, though the row guessed it was.**
`environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere`
is a different class — a value that knows a fact and carries none of
its words — and it stands on its own argument. It also reaches
`platform.rs` and `prefs.rs`, which are in no program's territory
(`work/view/viewer-src-files-no-successor-claims`), so it is not
dispatchable here until that is sorted.

**Filed out of the census and not this program's:**
`two-pickers-spell-one-not-well-typed-sentence-twice` (the words are
vnews' by VGEOM's cession, the `pane/profile.rs` half is nobody's) and
`a-disabled-controls-reason-has-one-home` on VDOC — which asks for a
GENERALISATION, not a first statement: `crates/viewer/README.md`
already carries the clause for two families and names the test that
holds it.

### What re-deriving the order against the tree changed (2026-09-19)

The order above was written at the re-scope and every row in it was
written earlier still. Re-deriving each row's fix SITE against the tree
at this program's first dispatch moved four of them. Recorded here
rather than fixed silently, because the order is the thing a later
session reads first.

- **`converged-recourse-has-no-home` left group 1 and then left this
  program — it is EDIT's now** (moved 2026-09-19; Ev granted the move
  and the standing authority to re-home a unit between tracks without a
  ruling). The group called it *"the third crosses a crate and
  announces"*; announcing is not what it needed. Both shapes the row
  states — a `pub const` beside `EditError`, or a `recourse()` method on
  it — **add API surface to `crates/editor-core`**, and the
  authorisation this program inherits is scoped to `EditError`'s
  `Display` WORDING (Ev, in-chat, 2026-09-04). `EditError` is declared
  in `crates/editor-core/src/edit.rs`, which is EDIT's territory. Only
  the viewer arm stays here: a forward at one site in
  `session/refuse.rs` once `editor-core` exposes the recourse, landing
  with the EDIT unit that exposes it.
- **`viewer-preview-names-a-verb-by-its-variant-identifier` is CLOSED
  — it was already discharged when this program inherited it.**
  `profile::path::Verb` has had a `Display` since
  `work/fix/verb-and-dimension-render-through-debug` (FIX, PR 2347,
  2026-09-11), and `PreviewError`'s arm in `crates/viewer/src/sketch.rs`
  forwards to it — `{verb}`, not `{verb:?}`. **This entry first said the
  row "cannot land from here alone" and routed it to PATHS**, which was
  wrong: the orchestrator re-derived the row's citation and not its
  premise. The correction is kept visible rather than overwritten
  because the rule it breaks — *a row's premise ages against the tree
  exactly like a citation does* — is the one handed to every lane this
  program dispatches, and the register's own instances of it are mostly
  the orchestrator's.
- **`tone-is-a-value-in-frame-and-a-comment-in-two-panes` keeps its
  place and loses a citation.** `pane/features.rs`'s hand-picked
  `ui.weak` / `ui.colored_label` pair is there as described, with the
  rule in a comment; `tree::RowStatus::badge()` is there and takes no
  tone. The row's THIRD copy at `pane/create.rs:582-586` is not: those
  lines are the `ShapeKind::Path` notation block today. The subject is
  re-derived by the lane, not repointed by arithmetic — this register's
  own rule.
- **`the-new-document-button-states-its-refusal-twice` waits on the
  census, one group later than the order puts it.** Its two answers are
  *"read the refusal"* and *"keep the literal and delete the claim"*,
  and the row says which is right is what
  `a-disabled-control-says-why-in-four-shapes` asks generally. The
  button is a genuine member of that general question — it IS gated on
  the condition `NewDocument` refuses — so deciding it alone decides
  the class from its easiest instance. The census goes first and this
  row applies its rule.

**And the frame.rs cluster is serialized, which the order does not
say.** Groups 3, 4, part of 5 and `document-news-has-no-home` all edit
`crates/viewer/src/frame.rs`. Under merge-only rules two lanes in that
file at once is a conflict bought for nothing, so **at most one
`frame.rs` lane runs at a time**, whatever the group order allows in
parallel elsewhere.

### 7. The frame.rs cluster, adjudicated (2026-09-20)

Groups 3 and 4, two of group 5's three, and two rows the order never
placed all edit `crates/viewer/src/frame.rs`, which is serialized. This
section is the order for that file, derived against the tree on
2026-09-20 rather than from the rows. It supersedes groups 3 and 4 for
the rows it names. Every claim below is a claim; the evidence is in
each row's own `## Adjudicated 2026-09-20` section.

**The enumeration rule: an OPEN row on this slate whose fix edits
`crates/viewer/src/frame.rs`.** The table below is the population of
record and its size is not restated here — the register's own rule
about a tabulated population, which it says its own count got wrong
twice.

**The instrument, and the member it cannot see.** The first pass was a
grep for the literal `frame.rs` across `work/vnews/*.md`, sixteen hits:
three are `plan.md`, `program.md` and `log.md`; **four are closed rows**
(`is-instance-collapses-absent-and-wrong-kind`,
`seat-line-spells-the-list-mark-as-a-literal`,
`tone-is-a-value-in-frame-and-a-comment-in-two-panes`,
`tone-to-chrome-mapping-is-spelled-twice`); two are open rows that name
the file without editing it —
`seat-lines-item-mark-has-no-name` says so itself (*"`frame.rs` is not
in scope"*) and
`undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words`
cites it only as evidence about reachability, its home being `app.rs`.

**That leaves seven, and the population is the table below.** The
eighth, `document-news-has-no-home`, **contains no occurrence of the
string `frame.rs`** — it names `frame::tool_news`, whose declaration is
in that file. It was found by reading every open row's cited fix SITES
and resolving each symbol to its file, not by the grep; the grep was
only how the other seven were found. So **the rule and the instrument
do not agree, and the rule is the one to re-run**: a row can edit this
file while naming only `frame::` types. A later session re-deriving
from the grep alone gets seven.

#### What each row is

| row | what it is | evidence |
|---|---|---|
| `ranked-and-unranked-verdicts-are-one-type` | **a build** this program decides | its sequencing gate (`frame-module-has-eight-concerns-and-no-holds-row`) is closed; doors and citations re-derived |
| `outstanding-and-progress-are-two-three-state-enums-one-hop-apart` | **a decision this program makes**, then a build | no clause in `GUI-DESIGN.md`; its only cited ratification is `crates/viewer/README.md:368`, the implementation record |
| `rank-one-discards-the-frames-other-news` | **a decision this program makes** | rank 1 is NOT ratified — see below |
| `one-line-one-subject-loses-a-mixed-frames-expiry` | **a decision this program makes** | same finding; arm 1 is the status quo and not a design change at all |
| `a-fold-row-composes-a-producer-with-a-dead-door` | **CLOSED 2026-09-20**, a negative result | its question is applied in the tree at `frame.rs:2337-2349` and stated nowhere; both residues re-homed as files |
| `document-news-has-no-home` | **a build**, not dispatchable yet | its fourteen sites include `pane/profile.rs`, which no dispatching program claims |
| `folded-moved-true-arm-covers-a-fold-that-did-not-move` | **a rider** (one doc line) **plus a rename the lane judges** | doc line verified at `frame.rs:2082`; the rename's churn is nine sites in four files, one across VDOC's fence |
| `tone-doc-argues-from-a-site-that-now-reads-the-value` | **two of its four members ride**; the row stays open | both `frame.rs` members verified false at `:1166-1171` and `:1764-1768`; its `theme.rs` member is in an unclaimed file |

**Nothing in this cluster is a decision for Ev.** Nothing here is
`needs_ev`, and none of it should become so.

#### The finding that decides the two ranking rows

Rank 1's *"a refusal wins, alone"* was called ratified by two rows and
**is not**: no clause in `crates/viewer/GUI-DESIGN.md`, none of
`docs/DESIGN.md`'s Q1–Q9, no `[ev]` commit ever on
`crates/viewer/src/frame.rs`, and one agent-written doc comment as its
only normative statement.

**The finding, the searches and their corrections live once**, in
`work/vnews/rank-one-discards-the-frames-other-news`'s adjudication
section. Five sibling rows and this section cite it and do not restate
it, for the reason §The register gives about the lane register: a claim
fixed in one place and stale in another contradicts itself, and six
copies of one search guarantee divergence the first time a line moves.

**It is an instance of a wider class, and the class is now a row.**
`crates/viewer/src` carries thirty-five `ratif*` assertions and at
least two more in this file alone have the same provenance —
`frame.rs:1191-1192` (*"the ratified argument the checks badge
carries"*) and `frame.rs:2040` (*"The ratified pattern is
refuse-then-offer"*, already spread to four sites) — while
`frame.rs:565`'s *"ratified expression-driven affordance"* is genuine
(`crates/viewer/GUI-DESIGN.md:93-94`, G4). Mixed, so it needs a census:
`work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`.

**One sentence in the source asserts the gate, and it now has an item.**
`crates/viewer/src/frame.rs:656-658` says the per-subject line *"is a
design question for Ev"*. That is
`work/vnews/frame-rs-says-the-per-subject-line-is-a-question-for-ev`,
group A's carrier — an item rather than a plan paragraph, because a
sentence that tells every reader a wall exists is the thing this
adjudication is about, and it needs an id for the riders to name.

#### Three claims the order and the brief carried that do not hold

1. **Group 3's *"the second's fork decides what the first collapses
   INTO, so they are one conversation"* is false.**
   `Outstanding`/`Progress` feed the badge channel and `StatusUpdate`
   feeds the status line; no value of either family is convertible to
   the other. They share a SHAPE, not a dependency, and are
   independently decidable. (Both halves of that sentence — the
   collapse and the *"one conversation"* — are group 3's. Group 4 says
   only *"Both are about news the ranking throws away rather than
   spells"*, which is true, and argues `rank-one-discards-…` first,
   which the order below agrees with.)
2. **Group 4's rows are adjacent, not the same question**, and that is
   a correction to how the cluster reads rather than to a sentence it
   wrote. A refusal and a supersession are both `Subject::Document`
   (`crates/viewer/src/frame.rs:583-585` and `:2502-2507`), so
   `one-line-one-subject-…`'s arm 2 leaves `rank-one-discards-…`'s case
   exactly as it is — the per-subject line still holds one message for
   that pair. The interlock runs one way only, and it is group 4's own
   order: `rank-one-discards-…` first makes arm 2 cheaper, not the
   reverse.
3. **`a-fold-row-…`'s *"nothing states it anywhere"* is nearly right
   and was over-corrected.** `frame.rs:2337-2349` APPLIES the rule at
   one site and never states it; the row's sentence is right about the
   rule's homelessness and wrong only in implying the tree offers
   nothing to read. The row is closed either way, its residues filed.
   And `ranked-and-unranked-…` may delete the instances outright, on
   one of its two arms — see the order below.

#### The order

**A. The prose pass — one lane.** Its carrier is
`work/vnews/frame-rs-says-the-per-subject-line-is-a-question-for-ev`
(the false Ev gate at `frame.rs:656-658`). Every other member is a row
of its own with `rides_with:` naming that carrier, so the dispositions
are on the board and visible to `lint` rather than living in this
paragraph:

| member | site | row |
|---|---|---|
| the false Ev gate | `frame.rs:656-658` | the carrier itself |
| *"actually moved the camera"* | `frame.rs:2082` | `folded-moved-true-arm-…` (doc-line half) |
| the two tone attributions | `frame.rs:1166-1171`, `:1764-1768` | `tone-doc-argues-…` (two of its four members) |
| the undisclosed dead composition | `frame.rs:2182` | `a-dead-composition-sets-up-a-fixture-without-saying-so` |
| the hand-maintained counts | `frame.rs:1345` and four siblings | `hand-maintained-counts-in-frame-rs-prose-have-no-guard` |

One file, no value change, one serialized slot instead of five.

**The tier is a style review, not the orchestrator's read.** An earlier
draft claimed the third tier by quoting half its definition; §Review
posture says *"neither correctness NOR STYLE is meaningfully at
risk"*, and six prose edits in a serialized file — one of them
retracting a sentence that told readers a design gate existed, one
deciding whether to remove a count or restate it — are a style
judgement in exactly the sense that tier excludes. Correctness is not
at risk, so no correctness arm.

Three things a lane taking it owes. **`frame.rs:1345` loses its number
rather than gaining a new one** — the rider says why. **`:656-658` and
`:704` are also in
`work/vdoc/crates-cite-work-view-rows-that-moved-in-the-rescope`'s
table**, so either the crossing is announced and those two lines
struck, or the lane leaves the path alone and repairs only the gate
claim; not both. And **`theme.rs` is out of fence and stays out**, so
`tone-doc-…` does not close with this lane.

**B. `ranked-and-unranked-verdicts-are-one-type`.** First of the
builds, because on its live arm it deletes other rows' subjects
outright: `apply` taking a ranked verdict makes **seven** sites stop
compiling — `frame.rs:2172`, `:2188`, `:2191`, `:2231`, `:2239`,
`:2370` and the production call at `pane/viewport.rs:473`. (On its
other stated arm, `apply` made private to `frame`, none of them moves;
that arm is not available while `apply` has two production callers
outside the module, which is the argument, not a certainty.) It also
decides the type every later row in this file writes against.
**Announce**: `pane/viewport.rs` is VGEOM's and VSEAM's too, and the
`frame::apply` call sites at `crates/viewer/tests/frame_policy.rs:826`,
`:904` and `:929` put it in VDOC's, S-TCOST's, S-TINT's and Track W's
ground.

**C. `rank-one-discards-the-frames-other-news`.** P0, and a decision
this program now knows it may make. Ahead of `one-line-one-subject-…`
because a rank-2 list that must already carry a second message beside a
refusal is the shape that row's arm 2 wants. **Announce**: the ranking
is pinned by `crates/viewer/tests/frame_policy.rs`, which is VDOC's,
S-TCOST's, S-TINT's and Track W's, and the row's own *"where to look"*
names `crates/viewer/src/app.rs`, which is VSEAM's. (The band is worth
a look: `work/README.md` P0 is *a live wrong answer* or *a defect Ev
reported*, and this is a silence about a lost hand-placed pose. Left
as filed; the re-band is the orchestrator's call, not an
adjudication's.)

**D. `one-line-one-subject-loses-a-mixed-frames-expiry`.** Its three
arms, costed against the tree rather than the row — arm 1 is the
status quo and costs one recording sentence.

**E. `outstanding-and-progress-are-two-three-state-enums-one-hop-
apart`.** Independent of B–D, so it goes wherever the serialized slot
is free. It reaches `session.rs` and `pickcache.rs`, which are VSEAM's:
announce.

**Not scheduled: `document-news-has-no-home`.** Its fix edits
`crates/viewer/src/pane/profile.rs`, which
`work/view/viewer-src-files-no-successor-claims` reports as claimed by
no re-scope successor while VIEW is `NOT DISPATCHING` — the same
blocker this plan already applies to
`environmental-facts-answer-usable-as-a-bool-with-the-reason-
elsewhere`. Left `open` rather than `parked`, matching that row.

**And `a-fold-row-composes-a-producer-with-a-dead-door` is CLOSED**
(2026-09-20), a negative result: its question is applied in the tree
and stated nowhere, and neither half is a decision this slate has to
make. Both residues are files, filed in the commit that closed it —
`work/vnews/a-dead-composition-sets-up-a-fixture-without-saying-so`
(group A) and
`work/vdoc/a-fixture-may-compose-a-dead-door-and-nothing-says-when`.

**Filed alongside, and not in this cluster's order:**
`work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`
(the class behind the headline finding — a census before any fix, and
it reaches files this program does not claim) and
`work/view/tier-rule-says-twenty-one-jobs-and-a-docs-run-shows-twenty-two`
(the register's docs-tier marker, measured stale by this lane's own
receipt).

#### One cross-program hazard on this file

`work/vdoc/crates-cite-work-view-rows-that-moved-in-the-rescope` edits
`crates/viewer/src/frame.rs:657`, `:704` and `:1895`, and
`work/vseam/projection-fault-has-no-sweeper` names `:1895` too. The
serialization clause above says *at most one `frame.rs` lane at a
time* and reads as a rule about this program's lanes; it is not one a
sibling program's dispatch can see. Whoever dispatches group A
announces it.

## Inbound

`joined-notices-nest-their-own-separator` is this program's row and is
still on VIEW's slate: its lane is in flight at PR #2665 and a rename
mid-review is a merge conflict for nothing. It arrives here when that
PR merges, or with VIEW's exit walk, whichever is first.

**Correction (2026-09-20, `vnews/frame-cluster-order`): the trigger
fired five days ago and nothing moved.** PR #2665 merged
2026-09-15T15:28:37Z (`bf79ece0e`, *"VIEW: a notice boundary is a mark
no notice can contain"*); its fix is on `main` —
`frame::NOTICE_SEPARATOR` is `" • "` built from `frame::NOTICE_MARK`,
with `frame::LIST_SEPARATOR` beside it. The row still reads
`status: review`, `pr: 2665` in `work/view/`, and its own `## Answered`
section records the work as done with the inner-level residue filed as
`withdrawal-causes-join-on-a-mark-a-fault-may-contain`. So the row
reads as an in-flight `frame.rs` lane on the board and is not one —
which matters here, because the serialization clause above is read off
exactly that. **The VNEWS orchestrator is moving and closing it**; this
section re-points when that lands. Either way **no `frame.rs` lane is
in flight today**, which is what group A's slot depends on.

(The first draft of this paragraph declined to act because the file is
VIEW's. That was wrong on `work/README.md`'s own terms — *"a lane does
not need the owner's permission to put a finding where it belongs"* —
and it is recorded rather than overwritten because declining to cross a
fence that the contract says is open is the mirror image of the
false-Ev-gate defect this cluster is about: a wall read where none was
written.)

## Dispatch rules this program pays for

**Do not ask a lane for `cargo nextest run --workspace` locally.**
Learned 2026-09-19/20, at a cost: wave 1's four lanes each carried that
instruction, and on this box it meant four concurrent workspace builds.
One lane's run was SIGTERM'd after **4h33m** stuck on a single
unrelated test with 1,427 rows still unrun; the same lane had
`scripts/doc-gate.sh --skip-viewer-toolkit` killed by the machine three
times, once after a 94-minute rustdoc phase. Measured at the end of
that wave: load average **37**, 0 GB of 9 GB free, 17 `rustc`/`cargo`
processes.

The instruction was also redundant, which is the part that makes it a
mistake rather than a trade. `docs/prompts/implementer-discipline.md`
§2 says hosted CI is the **verification of record** and that a local
run is an iteration tool which does not replace it — and CI's twelve
`test (…)` jobs ARE the workspace coverage the local run was asked for.
The register's rule that produced the instruction (#2293: *the viewer's
own suite cannot see rosters that live in other crates*) is about what a
**receipt** must cover, and CI covers it. So the fix is not to narrow
the method; it is to stop taking a second, slower copy of a receipt CI
already takes.

What a viewer lane owes locally, and nothing beyond it:
- `cargo test -p viewer --features app --no-fail-fast`, asserted by
  SHAPE — every `--test all` row passing, `--lib` one row red (the WGPU
  adapter) and no other — never against a row count, which moves.
- the two `scripts/doc-gate.sh` passes **when the diff touches a doc
  comment**, and only then.
- anything a tight edit-compile loop genuinely needs.

**And weigh the doc-gate exposure before demanding the skip pass.** The
skip-mode hole (#2320) is about a link from the renderer-free half INTO
the `app`-gated half. A diff whose new link is renderer-free to
renderer-free — both modules ungated `pub mod` in `lib.rs` — has no
exposure the full pass does not already judge, and the full pass is the
one CI runs on any branch that touches this crate. Check the gating
before treating a missing skip-pass receipt as uncovered.

**A named `keep_out` carve-out beats a general fence ruling, and an
orchestrator's ruling is not a territory amendment.** Adjudicating two
units on 2026-09-20 this orchestrator ruled that *prose your own diff
falsifies is yours to fix, and lands with the change that caused it;
prose that was already wrong is filed across the fence.* That rule is
**fence-local**. Where `program.md`'s `keep_out` names a file
specifically — `crates/viewer/README.md`, whose clause says a prose,
census or citation defect there *"is filed on vdoc and never fixed
across that fence"* — the carve-out wins and the sentence is filed even
when this program's own diff falsified it.

Three reasons, so it is not re-litigated. A `keep_out` is a territory
contract between two live programs, written on both sides; a ruling
issued in a dispatch message is not an amendment to one. VDOC's whole
charter is prose, census and citation on that file, so a rule letting
each of four successor programs fix the sentences its own diffs
falsified would put four programs in one file — the conflict
one-file-one-item exists to prevent. And the general rule's
justification is that such prose *lands with the change that caused
it*, which holds where the prose sits in the program's own ground and
not where it does not.

**The same ruling does not license code.** Told to repair a sentence
in `crates/viewer/src/app.rs`, the tone lane instead made the sentence
true by adding `app::toned` — better than the letter, and it closed a
residue row outright. But `app.rs` is **VSEAM's**, not a shared claim,
and a prose ruling never licensed a new door in another program's file;
the lane had itself declined to cross for that reason a day earlier and
was talked out of it by an instruction that did not reach that far. The
crossing stands and owes a row on the owner's slate naming the door,
its callers on both sides of the fence, and what its visibility
forecloses. **Fixing a sentence and adding a function are different
acts and a fence ruling about one says nothing about the other.**

**Apply the charter's reader test at SPEC time, not at review.** The
Charter's *"a row belongs here only if a reader would see the
difference; a rename nobody reads is not news"* is a scoping test —
does this row belong to a NEWS program at all, rather than to a sibling
— and it is cheap to run before a lane is dispatched and expensive
after. `is-instance-collapses-absent-and-wrong-kind` reached its style
review before anyone asked it: the split it makes is real, its one
caller draws nothing either way, and `DisplayState::prune` had already
told the two states apart. The unit is worth having — one decision site
where there were two, and a `pub` door whose next caller cannot get it
wrong — but it is NOT news, and nothing on this slate said so until a
reviewer read the charter back at it.

The consequence is about ORDER, not merit. Three rows in group 6 change
what a reader is actually told — Undo and Redo disabled in silence over
a refusal that has words, the range button's third spelling, the New
document button — and any of them would have been a better wave-1 unit
than a door nobody can currently misuse. **A row whose fix no reader
sees goes behind every row whose fix one does.**

**And when a lane refuses the orchestrator's recommended shape, the
burden is the orchestrator's.** Adjudicating that same unit I proposed
the panel SAY the absent-node sentence, to make the change news. The
lane refused and was right on four independent grounds, the decisive
one being that `pane/properties.rs`'s `standing_ui` already draws
*"deleted"* three lines earlier from the same lookup on the same
document — so the sentence would have been a second spelling of news
already on screen, minting the node-side twin of
`three-spellings-say-a-parameter-is-not-declared`, which is open on
this slate against the parameter half of the same function. Two further
grounds I had not weighed: `session::Standing`'s doc makes the section
not drawing the SECOND CLAUSE of its own rule (*"the affordances that
need a live entity switch off"*), and that clause is **GQ7's recorded
constraint** — ratified design in `crates/viewer/GUI-DESIGN.md`, so the
shape I recommended would have needed an `[ev]` PR to implement. An
orchestrator's fix shape is a claim like any other and gets checked
against the tree before it is issued.

**Read a row's STATUS before you build on its premise — a citation
check is not a premise check.** Three rows on this slate had premises
that the tree had already falsified, and the sessions that dispatched
them, including this one, checked line numbers instead:

- `viewer-preview-names-a-verb-by-its-variant-identifier` had been
  discharged eight days earlier by a CLOSED row on FIX's slate. The
  orchestrator re-derived its citation correctly — the render site
  really had moved — and routed it to PATHS on a premise that was dead.
- `seat-line-spells-the-list-mark-as-a-literal` told its lane the line
  *"reaches the chrome as a lost-pick notice's text"*. It does not, and
  the lane falsified it — then carried the correction into its report
  and not into its decision, which is the same miss one step in.
- That same row's central argument — that a `LIST_SEPARATOR` change
  *"moves the withdrawal join and the preferences join and leaves this
  one behind"* — rested on a consumer that #2710 had removed **the day
  after the row was filed**, for failing the very test the row's
  subject also fails. The unit closed as a negative result.

The instrument is cheap and is not the one anybody ran: **grep the
subject across `work/` and read the `status:` of every hit**, and read
the doc comment on the TYPE a row wants to reuse, not only the rule the
README states about modules. A closed row naming your site is the
answer to your unit. Both halves of that are already in the register —
*a sweep owes a TRACKER pass as well as a tree pass*, and *when the
type's own doc already states the invariant, satisfying it is not a
preference* — and this program has now paid for both twice in two days.

**A negative result is a deliverable.** Two of the three above closed
with no diff, and the argument for why is the thing that stops the next
sweep re-minting them. Write it into the row rather than the PR body,
which stops being read at merge.

**Concurrency.** Four lanes is the right number for READING work and
too many for four simultaneous viewer builds. Prefer a wave that mixes
one or two code lanes with census and adjudication work, which is what
wave 1 accidentally got right with its fourth lane.

## The register

**`work/view/plan.md`'s rule register binds every lane dispatched from
this program, inherited BY REFERENCE and not copied.** Read it in full
before writing a dispatch.

The reason it is not copied is the register's own: a claim fixed in one
place and stale in another contradicts itself, and four copies of a
register that is re-derived every wave guarantee four divergent copies
within a week. The register is also evidence — every rule in it is a
named failure at a named PR — and a copy detached from the program that
paid for it reads as a rule without its receipt.

**What that costs, said plainly:** `work/view/plan.md` goes when VIEW's
directory goes at its exit walk, and this reference dangles that day.
The register's permanent home is
`work/view/the-lane-register-has-no-home-after-views-directory-goes`,
open on VIEW's slate, and it is a precondition of VIEW's exit walk
rather than a follow-up to it. This section re-points when it lands.

## Review posture

**Inherited from VIEW unchanged (Ev, in-chat, 2026-09-04, reaffirmed
2026-09-04 evening; `docs/MODEL-AB-LOG.md`'s roster line).** No A/B
duals, no row in `docs/MODEL-AB-LOG.md`; the band stays claimed and
empty. The default is a style review against
`docs/prompts/reviewer-style-lane.md`, with a correctness arm added
only where a unit's failure mode is a confident wrong answer rather
than a refusal, and the dispatch says which it chose and why.

**A third tier, from protocol v7 (Ev, in-chat, 2026-09-19;
`docs/MODEL-AB-LOG.md`, the v7 entry, item 2).** v7 triages units into
and out of the A/B protocol, and its out-of-protocol half states a tier
below the style review that this program's posture predates and does
not mention: **a mechanical change, where neither correctness nor style
is meaningfully at risk, merges on green CI and the orchestrator's own
read — no review lane, no row.** v7's out-of-protocol clause governs
every unit that is not triaged in, and under the posture above no unit
here is ever triaged in, so the tier is available on this slate.

Nothing else about the posture moves. v7's triage question — is the
logic especially tricky, or is this an architectural decision whose
impact is broad or hard to reverse — is the question this program
answers with a **correctness arm** rather than with a dual, because
Ev's posture ruling is the more specific instruction and is not
withdrawn by v7. So the three tiers here are: orchestrator's read;
style review; style review plus a correctness arm. **The dispatch says
which tier and why, and the reason is recorded in `log.md`** (v7 item
5), so an un-reviewed unit is auditable rather than invisible.

## Exit shape

Every row above landed or ruled out, and the vocabulary the fixes
converge on stated once in `crates/viewer/README.md` with the sweep
rule that produces its population beside it — that last clause because
this program's output is exactly the kind of universal the VIEW
register says is where the next defect hides. The README clause is
VDOC's territory and lands as an announced crossing. The walk
convention applies; residue re-homes per `work/README.md`.
