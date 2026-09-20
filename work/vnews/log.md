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
the control is correct. The rule is not minted here: `refuse.rs`
states it three times, at `Refusal::self_instance`,
`Refusal::affordance` and `Refusal::exists_wording`, as the reason each
helper exists.

**The population.** 35 members — 30 `add_enabled` / `add_enabled_ui`
call sites plus 5 branches that draw a sentence where a control would
be. `on_disabled_hover_text` has 11 call sites and **every one attaches
to a member the first pass already had**: the proxy table's eighth row,
measured. The row's filing counts (18 and 10, on 2026-09-11) are
superseded, as are all fourteen of its citations — `pane/profile.rs`
and `widgets.rs` did not carry members when it was written and carry
sixteen between them now.

**Both of the row's predicted genuine hits are not hits.**
`pane/create.rs`'s `blocked: Option<&'static str>` gates a draft in all
four arms — no `SessionOp` can be formed without the plane or the
loops — and `platform::NO_CHOOSER_BACKEND` sits on buttons that reach
no door at all, so it is a first composition, not a second. The
genuine hits are elsewhere: the New-document Create button
(`Refusal::EmptyName`), Undo and Redo (`Refusal::NothingToDo`, shown as
nothing), and the slot range button (a third spelling of the ratified
affordance).

**Filed.** `undo-and-redo-are-disabled-in-silence-over-a-refusal-that-
has-words` and `the-range-button-re-mints-the-ratified-affordance`
(genuine hits); `two-pickers-spell-one-not-well-typed-sentence-twice`
(found in passing — `pane/profile.rs` hand-rolls `widgets.rs::offer`'s
sentence); and on VDOC's slate,
`a-disabled-controls-reason-has-one-home`, the README clause this
program's exit shape names.

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
