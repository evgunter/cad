# VNEWS — log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/vnews/plan.md`. A/B band 5200–5299
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-17) — opened in VIEW's re-scope

Opened by VIEW's orchestrator as one of four successors on VIEW's
ground, per `work/README.md`: *residue is re-homed before the sweep …
to a new program opened for it when the residue coheres into a track of
its own (a dozen items on one territory are a successor's opening
slate, and the closing program opens it)* (Ev, 2026-09-06).

**VIEW is not closed by that act and is not closed by this one.** Its
`Order`'s six units are done, deferred or handed off; its ninety-four
live rows were review accretion on one crate, and four of them cohere
into tracks. VIEW stays open with eight rows, its exit walk is a
separate ratified step, and `docs/DOC-LEDGER.md` records the sweep when
it happens.

14 rows arrived from `work/view/`, each by `git mv` with its body,
its id and its history unchanged — no row's prose was edited on the way
past, and the item schema carries no `program:` field, so a re-home is
the move and nothing else:

- `a-disabled-control-says-why-in-four-shapes`
- `a-fold-row-composes-a-producer-with-a-dead-door`
- `converged-recourse-has-no-home`
- `document-news-has-no-home`
- `environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere`
- `is-instance-collapses-absent-and-wrong-kind`
- `one-line-one-subject-loses-a-mixed-frames-expiry`
- `outstanding-and-progress-are-two-three-state-enums-one-hop-apart`
- `rank-one-discards-the-frames-other-news`
- `ranked-and-unranked-verdicts-are-one-type`
- `seat-line-spells-the-list-mark-as-a-literal`
- `the-new-document-button-states-its-refusal-twice`
- `tone-is-a-value-in-frame-and-a-comment-in-two-panes`
- `viewer-preview-names-a-verb-by-its-variant-identifier`

The charter that makes these one program, and the sentence that is true
of them and false of the other three tracks' rows, is `plan.md`
§Charter. The band 5200–5299 is claimed in `docs/MODEL-AB-LOG.md` in the
commit that opens this program, per that entry's own rule.

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`
if the posture ever changes; under the inherited posture it records no
row.

## 2026-09-19 — opened for work; wave 1 dispatched (four lanes)

First dispatch. The session read `work/README.md`,
`docs/prompts/implementer-discipline.md`,
`docs/prompts/reviewer-style-lane.md` and `work/view/plan.md`'s
register in full before writing a brief, and re-derived every wave-1
row's premise and fix site against the tree rather than against the
row. Four of the fourteen rows moved as a result; `plan.md` §Order's
new subsection records which and why.

**Review tier per unit, with its reason** (protocol v7 item 5, and
`plan.md` §Review posture as amended in this commit). This program runs
no duals, so no unit draws an ordinal and the band 5200–5299 stays
empty:

| unit | tier | why |
|---|---|---|
| `seat-line-spells-the-list-mark-as-a-literal` | style | one constant substituted for its own value; the two halves of each item cannot contain the mark, so there is no wrong answer available |
| `is-instance-collapses-absent-and-wrong-kind` | style | the door's shape is the defect and its one caller early-returns for both states, so nothing observable changes; the sweep is adjudication, not a claim about behaviour |
| `tone-is-a-value-in-frame-and-a-comment-in-two-panes` | style | a rule stated in a comment becomes a value read from one; the rendering is asserted where it matters and a wrong tone reads as a wrong colour, not a wrong fact |
| `a-disabled-control-says-why-in-four-shapes` | **style + correctness arm** | a census whose failure mode is a population that looks complete and is not — a confident wrong answer, which is this program's stated trigger — and its rule decides three other rows |

**Wave 1 is file-disjoint by construction.** `seats.rs` ·
`display.rs` + `pane/properties.rs` · `tree.rs` + `pane/features.rs` ·
no source file at all. **No lane touches `frame.rs`**, which is
deliberate: six of the remaining rows edit it and serializing them is
cheaper than conflicting.

**Filed by this sitting:**

- `work/view/viewer-src-files-no-successor-claims` — eleven
  `crates/viewer/src` files are claimed by none of the four re-scope
  successors, three of them live sites for rows the split handed out.
  Filed on VIEW's slate because the allocation was VIEW's act and
  finishing it is a precondition of its exit walk, the shape
  `the-lane-register-has-no-home-after-views-directory-goes` already
  has.
- `crates/viewer/src/tree.rs` claimed into this program's `paths` in
  this commit, because `tone-…`'s fix lands there and it was in
  nobody's territory. The other ten unclaimed files are left for that
  row to sort rather than swept up here.

**Two rows are not this program's to land, and the order said they
were.** `converged-recourse-has-no-home` needs an `editor-core`
authorisation wider than the `Display` wording Ev granted, so it is
EDIT's or an `[ev]` question; `viewer-preview-names-a-verb-by-its-
variant-identifier` needs `impl Display for Verb` in `crates/profile`,
which is PATHS'. Both stay `open` here rather than `parked`: neither
waits on a trigger the tracker can see fire, and saying so in the plan
is the honest record.

## 2026-09-19 — the disabled-control census, and the rule it lands on

`a-disabled-control-says-why-in-four-shapes` run as a census at merge
base `2654cc111417da806d9786c40136106469096fec`. No file under
`crates/` touched; tier docs. Tiered style review plus a correctness
arm, per the plan's posture — a census's failure mode is a population
that looks complete and is not.

**The rule.** *A control a reader cannot use owes the sentence a click
would have been answered with — when there is such a sentence.* The
test is one question at the control: if the reader got past this gate
and the operation ran, what sentence would come back? One would → the
pre-click sentence and the refusal are one sentence and get one
composition. None would → the control gates a draft, and a literal at
the control is correct. **And a third arm the tree forced**: the
operation would be formed and would SUCCEED, and the chrome declines it
anyway — a chrome-policy gate, which has no refusal to read.
`pane/create.rs`'s bore/radius arm is the one instance, and its own
literal says the door *"would swap the roles rather than refuse"*.

**Attribution.** The rule's FIRST limb is the tree's and is stated nine
times in five files — `session/refuse.rs` (three helpers),
`session/op.rs` (`CancelDoor`), `pane/create.rs` (the parts catalogue),
`parts.rs` (`PartEntry::open_document`) and `pane/properties.rs`
(three sites). **None of them states the second limb**, and the
qualifier is what does the classification work, so it is defended as
this lane's rather than attributed.

**The population is two populations, of different epistemic status,
and they are not added.** P1 is `add_enabled` / `add_enabled_ui` — 30
sites, one grep and one stated exclusion, mechanical. P2 is a branch
drawing a sentence where a control would be — 11 sites, a judgement,
and therefore **enumerated by site in the item** rather than counted.
`on_disabled_hover_text` has 11 call sites, every one attached to a P1
site; that measures `on_disabled_hover_text ⊆ add_enabled`, which is
the narrow half of the proxy table's eighth row and not its wide
disposition claim. The row's filing counts (18 and 10, on 2026-09-11)
are superseded, as are all fourteen of its citations.

**Both of the row's predicted genuine hits are not hits.**
`platform::NO_CHOOSER_BACKEND` sits on buttons that reach no door at
all, so it is a first composition. `pane/create.rs`'s `blocked` gates a
draft in three of its four arms — and the fourth is the chrome-policy
gate above, not a draft gate, which is the census finding a case its
own rule did not cover. The genuine hits are elsewhere: the
New-document Create button (`Refusal::EmptyName`), Undo and Redo
(`Refusal::NothingToDo`, shown as nothing), the slot range button (a
third spelling of the ratified affordance), and two P2 sites that say
in the chrome's words what `Refusal::NoSuchParam` says in the
session's.

**Filed.** `undo-and-redo-are-disabled-in-silence-over-a-refusal-that-
has-words`, `the-range-button-re-mints-the-ratified-affordance` and
`three-spellings-say-a-parameter-is-not-declared` (genuine hits);
`clear-picks-hover-text-is-invisible-while-disabled` (six gated
controls whose only words ride on `on_hover_text`);
`two-pickers-spell-one-not-well-typed-sentence-twice` (found in
passing — `pane/profile.rs` hand-rolls the sentence of
`widgets.rs::offer`, which is module-private and so is not a helper the
site could have called). Across fences:
`work/vdoc/a-disabled-controls-reason-has-one-home` — the README clause
this program's exit shape names, re-written as a request to GENERALISE
two clauses the page already carries rather than to state the rule for
the first time; `work/edit/no-door-refuses-a-blank-parameter-name`;
`work/guard/viewer-disabled-control-population-has-no-gate`;
`work/vseam/op-rs-cites-environmental-facts-at-its-old-path`; and
`work/vdoc/crates-cite-work-view-rows-that-moved-in-the-rescope`, the
class that last one turned out to be — 26 stale `work/view/` citations
under `crates/`, in 13 files across 5 crates.

**Dispositions handed down.**
`the-new-document-button-states-its-refusal-twice` gets answer 1, read
the refusal — its stated cost is the objection `Refusal::exists_wording`
already answers. `environmental-facts-answer-usable-as-a-bool-with-the-
reason-elsewhere` is **not decided** by this rule and is untouched by
it; the two classes are disjoint at `NO_CHOOSER_BACKEND`, and that row
stands on its own argument. Evidence appended to both rows rather than
left in a PR body.

## 2026-09-19 (later) — two rows leave the slate, one of them already dead

Ev, in-chat: `converged-recourse-has-no-home` may move to another
track, **and in general this orchestrator may re-home a unit between
tracks without asking.** Recorded here because it is a standing grant
of authority, not a ruling on one row.

- **`converged-recourse-has-no-home` → `work/edit/`.** `EditError` is
  declared in `crates/editor-core/src/edit.rs` and that file is EDIT's
  territory; EDIT is DOCM's successor for the surface the row needs
  widened, and the row's own closing line names EDIT's slate by
  description. The viewer arm — one forward in `session/refuse.rs` —
  stays this program's and lands with whichever EDIT unit exposes the
  recourse, as an announced crossing.
- **`viewer-preview-names-a-verb-by-its-variant-identifier` → CLOSED,
  and this program never should have carried it.** Checking the tree
  before routing it: `profile::path::Verb` has a `Display`
  (`crates/profile/src/path/program.rs`, in the macro beside
  `Verb::ALL`) and `PreviewError`'s arm in `crates/viewer/src/sketch.rs`
  reads `{verb}`, not `{verb:?}`. `work/fix/verb-and-dimension-render-
  through-debug` (FIX, PR 2347) closed both halves on 2026-09-11 and
  its closing note even records deleting the `prose_census` `UNDECIDED`
  entry for this exact site. The row travelled through VIEW's re-scope
  eight days dead.

**The orchestrator's own miss, recorded because the register says to.**
This morning's §Order subsection re-derived this row's CITATION —
correctly; the `PreviewError` render really had moved out of
`pane/create.rs` — and concluded it needed `impl Display for Verb` in
`crates/profile`. It did not re-derive the row's PREMISE, which had
been false for eight days. That is the register's own rule
(*"a row's premise ages against the tree exactly like a citation
does"*, from #2388's dispatch) missed by the session that quoted it to
four lanes the same hour. The cheap instrument that would have caught
it is the one the lanes were told to run and the orchestrator did not:
a TRACKER pass beside the tree pass — `grep -rl` the defect's subject
across `work/` and read what the hits say, which here was a closed FIX
row naming the site.

**Slate after this sitting: twelve rows**, four of them wave-1 lanes in
flight.

## 2026-09-19 — the census adjudicated, and what it cost to get right

`a-disabled-control-says-why-in-four-shapes` ran as this program's
first unit under the tier its posture names: a **style review and a
correctness arm**, two independent lanes on the frozen head
`0dbbc95f8`, neither with access to the other's report, and a fix pass
against the orchestrator-adjudicated UNION. No dual, no ordinal, no row
in `docs/MODEL-AB-LOG.md`.

**The correctness arm earned its dispatch.** It returned a MAJOR that
the style lane reached from a different direction and neither could have
been talked out of: the population was not re-derivable. It also
verified, rather than assumed, two things the item asserted — that the
`refuse.rs` doc comments say what the item quotes, and that no keyboard
or AccessKit route pushes `Undo`/`Redo`, so the buttons really are the
only hand.

**The two findings worth keeping:**

- **The rule as first written is a dichotomy and the tree has three
  cases.** `pane/create.rs`'s bore-against-radius arm forms a valid
  `SessionOp::AddProfile` — its own literal says *"a larger bore would
  swap the roles rather than refuse"* — so the chrome declines
  something the door would ACCEPT. That is a chrome-policy gate, and it
  owes a true sentence and a disclosure that it is a policy rather than
  a refusal it cannot read. The census found a case its own rule did
  not cover, which is worth more than the count it got wrong.
- **Enumerating the judgement pass produced hits nobody had.** Three
  spellings of *a parameter is not declared*, across two panes and
  `Refusal::NoSuchParam`.

**Two corrections to the orchestrator's own adjudication**, recorded
because this program's register says the orchestrator's misses are the
ones that propagate:

1. I ruled that *"the six classification buckets are all P1 sites"* and
   offered it as the sharpest form of the MAJOR. **It is false.** Three
   buckets already held judgement-pass members. The ruling survived —
   the population was not reconstructible and the buckets did not sum —
   but the diagnosis was wrong, and the evidence was on screen when I
   wrote it: the draft-gate bucket cites `pane/create.rs:52`, which is
   one of them. The fix pass caught it and said so.
2. The class row filed out of S17 said the repair *"touches five
   crates"*; its own table lists four. Corrected here. Re-deriving its
   headline independently: **36 citation sites under `crates/`, 26 of
   them naming a `work/view/` row that the re-scope moved** — that
   figure holds exactly, which is why the crate count mattered enough
   to fix rather than shrug at. A row whose thesis is that citations rot
   cannot carry a count its own receipt contradicts.

**Filed out of this unit: six rows** — three on this slate's
neighbours (`work/edit/`, `work/vseam/`, `work/guard/`), two here, one
on VDOC. The VDOC clause request was rewritten mid-fix from *state this
for the first time* to *generalise what is already stated*, after
`crates/viewer/README.md` turned out to carry the rule for two families
and to name the test that holds it.

**Tier note for the next unit.** The correctness arm cost one extra
lane and returned a MAJOR, two upheld MINORs and a rule change. On this
program's posture that is the trigger working as written: a census's
failure mode is a population that looks complete and is not, which is a
confident wrong answer and not a refusal.

## 2026-09-20 — wave 1 closed: four units, two negative results, 23 rows open

All four wave-1 units are on `main`. The slate went from 14 rows to 23,
which is the wave's largest single output and the thing to read it by.

| unit | outcome | PR |
|---|---|---|
| `a-disabled-control-says-why-in-four-shapes` | the rule, the population, four rows filed | #2908 |
| `seat-line-spells-the-list-mark-as-a-literal` | **negative result** — no code change | #2917 |
| `is-instance-collapses-absent-and-wrong-kind` | `instance_check -> Result<(), AdmissionFault>` | #2916 |
| `tone-is-a-value-in-frame-and-a-comment-in-two-panes` | `RowStatus::tone()`, and `app::toned` | #2915 |

**Both rows closed at this entry were merged and still read `review`.**
A row whose unit is on `main` and whose board entry says otherwise is
the same defect as a stale citation, one level up; the census row closes
too, because its deliverable — the rule and the classified population —
landed, and each control that owes the rule is its own scheduled row.

**Two of four units changed no behaviour a reader can see, and one
changed no code at all.** That is the wave's real finding and it is
about SPEC time, not about the lanes:

- `seat_line` closed as a negative result. Its central argument rested
  on a consumer `LIST_SEPARATOR` had lost the day after the row was
  filed, and its subject fails the same two-part test that removed that
  consumer. The argument is written into the row so the next sweep does
  not re-mint it.
- `is_instance` is a better door with no reader-visible difference, and
  nothing said so until a reviewer read the Charter back at it.

`plan.md` §Dispatch rules now carries what both cost: read a row's
STATUS before its premise; apply the Charter's reader test at spec
time; an orchestrator's fix shape is a claim and gets checked before it
is issued; a `keep_out` carve-out beats a general fence ruling; fixing a
sentence and adding a function are different acts.

**Three lanes falsified something the orchestrator told them**, and in
every case the lane was right: the two sibling rows were closed and not
open; the `pane/create.rs` citation was stale in its number and exact
in its subject, and the third copy was real; the absent-node sentence I
recommended would have minted the node-side twin of a row open on this
slate, and the shape I proposed would have needed an `[ev]` PR because
`Standing`'s second clause is GQ7's ratified constraint.

**The review tiers held.** The one unit given a correctness arm — the
census — returned a MAJOR that a style lane alone would have recorded
as taste. Both style reviews caught defects that would otherwise have
merged: `seat_line`'s whole premise, and `tone`'s silent loss of
compile-time exhaustiveness.

**Operationally**: four concurrent code lanes saturated the box (load
37, 0 GB free of 9), one lane's broad `pkill` damaged another's build,
and GitHub Actions was billing-locked repo-wide for roughly five hours
mid-wave. #2937 withdrew the local workspace-test instruction that
caused most of it. Wave 2 mixes at most two code lanes with reading
work.

**Wave 2's order is `plan.md` §Order group 6**, which the census
produced: the three controls that owe the rule, in the order that
section states. Every one of them changes what a reader is told, which
is the test this wave learned to apply first.

## 2026-09-20 — the order's history moves out of the plan

`work/README.md:24` says `plan.md` is the plan, **"present state
only"**, and `log.md` is the append-only narrative. Two reviewers
independently found the same defect in `plan.md` on 2026-09-20 — §Order
had grown to 382 lines in three chronological strata holding an order,
a superseding order and two retrospectives, so a reader arriving at
opening group 3 had no signal it had been overruled 250 lines below.
The file went 141 → 672 lines in three days under one author with no
individual diff unreasonable, which is the register's own 449-line
header shape.

The repair is the contract rather than taste: **§Order now carries the
live order and a disposition table for the discharged groups, and the
history is here.** What follows is the section that was
`plan.md` §Order's *"What re-deriving the order against the tree
changed (2026-09-19)"*, moved verbatim so nothing is lost:

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

## A note from VIEW (2026-09-21) — one rank row re-homed here

`a-derived-pick-index-failure-outshouts-its-cause` (P1, Ev's report of
2026-09-17) moved by `git mv`, id, body and history unchanged. Ev
approved the move in chat. VIEW is winding down and does not dispatch.

**Why this program and not VSEAM or CHROME.** The row's finding is
that a pick-index failure CAUSED by a failed node is the loud banner
while the node's own Boolean refusal is the quiet line below it — *a
downstream effect of a failure the user already has in front of them
should not outrank that failure*. That is a rank defect, and your
§Charter names `rank-one-discards-the-frames-other-news` as a member
of exactly this class. Nothing it touches survives the frame.

A second half rides it: the two messages name different nodes (root 11
versus the node that actually failed, 13), so a reader cannot tell
they are about one event. The two sites are `viewer`'s `pickindex.rs`
(the *"could not be tessellated or indexed"* arm) and `editor-core`'s
`resolve/hit.rs` (*"no name table to invert"*) — the second is EDIT's
ground and a hand-off rather than a diff from here.

**Live-ground note**: `pickindex.rs` is claimed by VGEOM, VSEAM and
FIT as well, and VGEOM has a live lane on it (`vgeom/pick-distance`,
PR #3007's fix pass). Worth a check before dispatching.

Signed (VIEW orchestrator).

## 2026-09-23 — a CHROME lane touched `pane/properties.rs` under #2961

`chrome/properties-messages` (CHROME's P0,
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`)
edits `crates/viewer/src/pane/properties.rs`, which
`vnews/properties-controls-read-their-refusals` (#2961) edits too.
Functions touched: `properties_ui`, `feature_rows_ui`, `add_param_ui`
(its offer line and its already-declared arm, now the free function
`exists_notice`), `standing_ui`, `entity_standing_ui`, `instance_ui`
(the `free_move_check` fault line only), `slot_value_ui` (its fault
line is removed; it is said under the row now), `slot_notes_ui` (now a
wrapper over the free function `slot_notes`) and `param_bounds_ui`
(now a wrapper over `bounds_notes`). New at the end of the file: the
three free functions and a `#[cfg(test)] mod layout_tests`, beside
#2961's `mod tests`. The diff is per site. Merging it into #2961 is
mechanical except in one place: #2961's new `ui.weak(fault.to_string())`
in `instance_ui`, under the disabled hide toggle, is a sentence under
CHROME's census test and wants `crate::widgets::message_toned(…,
Tone::Advisory)` like its neighbour.

(CHROME implementer lane, chrome/properties-messages)

## CHROME in `pane/create.rs`: #2960's part button moves (2026-09-24)

`chrome/create-messages` (PR 3139) moves `add_part_ui`'s per-entry pick
button out of the window body into a free function,
`crate::pane::create`'s `part_entry`, which still spells it
`ui.add_enabled(…, egui::Button::new(entry.file_name()))` plus
`on_disabled_hover_text(refusal.to_string())`. PR 2960
(`vnews/app-controls-read-their-refusals`) replaces exactly those lines
with `crate::app::refusable_button(ui, entry.file_name(), refusal.as_ref())`.
The two collide in text, and the collision is **semantic**. Whoever
lands second must carry `refusable_button` into `part_entry`. Keeping
`part_entry`'s old spelling loses #2960's change there without any
compile error.

(CHROME implementer lane, chrome/create-messages)

## 2026-09-24 — a VNEWS lane crosses into `pane/profile.rs` and `pane/headless`

`vnews/gated-controls-say-why-while-disabled` closes
`clear-picks-hover-text-is-invisible-while-disabled`. **Announced
crossings:**
- `pane/profile.rs` is claimed by no program. The four step-row glyph
  controls now go through a private `step_control`. Revert moves out
  of `edit_profile_ui` into a free `revert_button`, which the method
  calls.
- In `pane/create.rs`, `blend_commit_row`'s Clear picks moves into a
  free `clear_picks_button`.
- `crate::pane::headless` gains `painted_while_hovering`, and its
  `hit` takes an occurrence index. The three drives now share one
  private `frame`, so the module's *"one drive"* claim is true again.
  It had not been since `painted_after_clicking` inlined its own copy.

Tier: style review, no correctness arm. The failure mode is a wrong
or missing tooltip. That is visible, and every disabled sentence is
asserted by its text.

The fix pass on #3216 (2026-09-25) also crosses into
`crates/viewer/src/forms.rs`, which belongs to author, chrome, forms and
vseam. Only doc text changed there: `SHAPE_LOCKED`'s doc and
`ShapeEdits`'s doc no longer say the notice is drawn "once" above the
list, because the step controls' disabled hovers now read it too.
`pane/profile.rs`'s Revert moved into `apply_and_revert`, and Apply
moved with it.

## 2026-09-25 — the tone unit's two residue rows close; a VNEWS lane crosses into `session/select.rs`

`vnews/salience-read-from-the-value` (#3230) closes
`a-tree-rows-message-line-picks-its-affordance-by-hand` and
`resolution-and-standing-pick-their-tone-by-hand`. Each row states its
decision. **Announced crossings:**
- `crates/viewer/src/session/select.rs`, which CHROME and VSEAM own:
  `Standing` gains `tone()`, and the file gains a test module for it.
- `pane/properties.rs`, shared with AUTHOR, CHROME and VGEOM: every
  standing verdict is drawn by one free `standing_verdict`;
  `entity_standing_ui` becomes the header-only `entity_header_ui`; the
  parameter panel's duplicate `"that parameter is gone"` line is
  deleted, and a deleted node no longer claims to carry no parameters.
- `crates/viewer/src/session/refuse.rs`: `FaceFrameFault` gains
  `tone()`.
- `crates/viewer/src/parts.rs`: `PartChooser` gains `tone()`.
- `pane/create.rs`: the part chooser's body becomes the free
  `part_listing`, reading `PartChooser::tone`, and drops its quiet
  `"no directory"` header; the face-frame fault reads
  `FaceFrameFault::tone`; the add-profile form's held reason is a typed
  `Held`.
- `app.rs`, which CHROME and VSEAM own: `toned`'s doc only.
- `crate::pane::headless` gains `Landed::ink`, `Voices`,
  `landed_voiced` and `find_opening`.

Filed: `a-verdict-drawn-outside-a-tone-has-no-value-to-read`. Moved
from `work/issues/`: `preview-error-picks-its-tone-by-hand-in-a-comment`,
because `pane/profile.rs` is VNEWS-claimed today.
Filed on VDOC: `viewer-readme-counts-one-tone-function-outside-frame`
(the README is VDOC's carve-out).
Final pass: `app.rs` gains a test module, `properties_pane_tests`, which
drives the real app frame headlessly (`ViewerApp::assemble`, eframe's
`Frame::_new_kittest`). Filed: `add-profile-held-reason-is-overwritten-not-first`,
`part-census-dir-iff-refusal-is-held-in-prose`.

## 2026-09-25 — P0 `rank-one-discards-the-frames-other-news`: the first dual on this slate (DR-9)

**Tier: dual**, under `memories/orchestration-model.md`'s tiers, which this program adopted at #3261. Reason: the unit puts a classification (`frame::Retold`) on every status-line `Message`, with no default, so every producer in the crate must answer it — a design decision that is broad and hard to reverse. **Chosen after spec, not at it**: the unit was dispatched expecting a single style review, and that review showed its first rule tested the wrong property (whether the *state* comes back, when the question is whether the *news* does). The fix pass then made the rule broad enough to earn a dual. Recorded as such in the row.

The pair (R1 NOT-MERGEABLE-AS-IS, R2 APPROVE-WITH-FIXES) both found, by independent probes, that a `Strand` on a `Declare` carrier is lost beside a refusal. R1 rated it MAJOR and R2 MINOR, so it is bilateral and not a tally candidate. The fix pass answered every strand `Never` under a new stated burden — *a door answers `Again` only when it can show the retelling from what it holds* — because a carrier poisoned upstream defeats a per-carrier answer. Tally unchanged at 0; fair pairs 6.
