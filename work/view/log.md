# VIEW log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/view/plan.md`. A/B band 1900–1999
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose VIEW section is the
charter this plan restates. Opens after CHROME's slate. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `viewer-session-god-module-split` from `work/issues/`
- `pick-priority-filter-vocabulary` from `work/issues/`
- `camera-fold-clears-status-line` from `work/issues/`
- `focus-marking-is-per-node-not-per-segment` from `work/issues/`
- `pick-index-built-on-ui-thread` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Hand-off from DOCM (2026-09-04)

`layer3-recipenodeid-aliases-across-rewinds` re-homed here by
header-preserving `git mv`: the rule is ratified
(`docs/DOCM-IDENTITY-DESIGN.md` DI1 — a held id is valid on the history
branch that minted it; tools clear on history replacement), the build
is the viewer's. Signed (DOCM orchestrator).

## Opened for work (2026-09-04)

**The opening condition is met.** The plan says this program opens after
CHROME's slate; CHROME's 2026-09-04 entry parks the nine items whose
ground is `session.rs` or `app.rs` on `viewer-session-god-module-split`
and names it their trigger. So CHROME's remainder is no longer
competing for these files — it is queued behind unit 1 — and the
dispatchable half of CHROME's slate names no path this program's
`paths` cover. Nothing is left to wait for.

Orchestrator seat taken this session. The orchestrator work lands from
a session branch rather than `view/orchestrator`; the `view/` prefix
still governs unit branches.

**This program runs no A/B duals (Ev, in-chat, 2026-09-04).** Reviews
here are style reviews against `docs/prompts/reviewer-style-lane.md`,
with a second reviewer on correctness for the units named below. VIEW
therefore draws no ordinal from its band, which stays claimed and
empty; the band table in `docs/MODEL-AB-LOG.md` is amended in this
commit to say so, so an analyst reading 1900–1999 as a gap does not
have to guess why. This is a dispatch-posture instruction, not a
protocol amendment: v6 is untouched everywhere else.

### Review posture, per item, and why

The default is a style review and nothing more. Three items are argued
above that default, and one is argued down.

**`pick-index-built-on-ui-thread` is re-cut into three.** It was the
program's one D→H unit and it stacked three separable things: revising
a ratified seam, moving work across a thread boundary, and inventing a
staleness rule for frame state that now arrives asynchronously. Stacked,
its failure mode is a picture and a pick index that disagree —
intermittent, silent, and invisible to a style lane, which is the
argument for an adversarial review. Unstacked, it is not:

- **6a, the seam ruling** — an `[ev]` PR extending GUI-3 §5's
  frame-state inventory and stating the staleness rule. No code, so
  nothing to review adversarially.
- **6b, the move** — tessellation and `PickIndex::build` onto the
  `EvalService` worker over the shipped `CancelToken` and
  cancel-and-restart, `PickCache`'s retry policy travelling with it.
  Style review plus a second reviewer on correctness.
- **6c, staleness as frame data** — the rule lands in `frame` as values
  with headless rows. `frame` was extracted precisely so these rules
  are values a row can execute rather than app-gated code nobody can
  reach; `camera-fold-clears-status-line` makes that argument in its
  own text. A rule a row executes is testable, so a style review is
  the right instrument.

**If 6a rules the staleness rule is NOT expressible as frame data**,
6b and 6c collapse back into one unit that does want an adversarial
review, and that is a question for Ev before dispatch rather than a
call to make at dispatch time.

**Two units get a second reviewer on correctness**, on the same
argument in both cases — the failure mode is a confident wrong answer
with no refusal anywhere, which is the shape a style lane is not
looking for:

- `layer3-recipenodeid-aliases-across-rewinds`: the filed reproduction
  authors a real `Node::Revolve` of nodes nobody picked, and every kind
  gate passes. Narrow enough (one rule, three holders) that a second
  correctness read is proportionate to it.
- the authored-step to canonical-segment door under
  `focus-marking-is-per-node-not-per-segment`: the item names the
  hazard itself — a wrong map lights a confidently wrong edge, silently,
  with the user believing the picture.

**`pick-priority-filter-vocabulary` is argued DOWN, to parked.** Its own
text sets the trigger at a third asymmetric tool (a vertex pick), and
no such tool exists or is scheduled. `crates/viewer/README.md` GQ7
ratifies the deferral in as many words — the filter vocabulary waits on
sketcher and tree design — so building it now is not executing this
program's plan, it is overturning ratified design for two tools that
are already served. Parked; the blocker is recorded in the item's
header once unit 1 has said where a tool states what it wants, since
`ToolKind::pick_kinds` is one of the hand-maintained lists over the
tool set that unit 1's charter collapses.

## DI1's build is parked on a door DOCM owns, and item 5 was half misfiled (2026-09-04)

Two corrections from reading `docs/DOCM-IDENTITY-DESIGN.md` against the
tree, both landing before any dispatch rather than at one.

**The plan's item 5 named two builds and only one is ours.** It read
"the viewer builds of DOCM's layer-3 identity rule and free-move
answer, when ruled". Both are now ruled, and DI5 assigns the free-move
build to CHROME by name (`no-persistent-setplacement-session-op`),
where it sits parked behind this program's unit 1. Only DI1 is VIEW's.
The plan says so now; nothing was dispatched against the wrong half.

**DI1's mechanism has no layer-3 door.** DI1 defines the hold as an id
plus its minting entry, computed "by walking up until the counter drops
below the id (`History::entry`, `Doc::next_id`)". `History::entry` is
public; `Doc::next_id` is `pub(crate)` with no accessor
(`crates/editor-core/src/doc.rs:315`), so the comparison the walk is
defined by cannot be written from the viewer, and G1 makes reaching
past the public surface a discipline rather than a preference. Filed as
`next-id-has-no-layer3-door` and the DI1 build parked on it. The door
is editor-core's and therefore DOCM's; the shape is theirs to pick
(a `next_id` accessor, or the narrower predicate that answers *could
this document have minted this id* without exposing the counter), and
this program takes no position beyond naming the need. Announce owed to
DOCM on the away channel.

**DI1 also widens the sweep** past the three holders the item's
original text named: it adds the revolve and combining seats,
`BlendTarget::node`, and every held `StableName`, since a name embeds
its minting node. Recorded on the item, because the sweep obligation is
the part of that unit most likely to be read off the stale list.

## The focus map door is two programs' ground, not one (2026-09-04)

The program header's `keep_out` said the authored-step to
canonical-segment map "lives beside the lowering in `crates/profile`
which is S-BOOL's glob — announce". Measured against the territory
globs, that is half the door:

- the authored `step` is `ProfileProgram::step_args`
  (`crates/editor-core/src/program.rs:653`) and `ProfileEdgeRef`'s
  `segment` is `crates/editor-core/src/names/role.rs:140` — both
  **DOCM's** glob, not S-BOOL's;
- only the canonicalization that *decides* the segment chain (the
  reversal, the canonical start, `circle_split`) is `crates/profile`,
  **S-BOOL's**.

So it is two announces, and — more to the point — where the map lives
is not a question this program or either owner can settle alone. The
header and the item now say so. This changes nothing about the unit's
review posture: the failure mode is still a confidently wrong edge lit
silently, so it still gets a second reviewer on correctness. It changes
who has to be in the conversation before it is cut.

**A pattern, second instance today.** `next-id-has-no-layer3-door` and
this are the same shape: a ratified design names a mechanism, and the
mechanism's door is on someone else's ground and not yet open. Both
were found by reading the cited symbols rather than the cited prose,
and neither was visible from the item text. Worth doing for the
remaining items before any of them is cut, which is the order this
program is now working in.

## The pick index's expensive step is not cancelable, and 6a now owes that question too (2026-09-04)

Third instance of today's pattern, and the one that most changes a
unit's shape. `pick-index-built-on-ui-thread` says a δ change "wants
the same cancel-and-restart the evaluation already has". The seam does
have it — `submit`/`poll`/`cancel` over a `Generation`, payloads
already `Send` — but its cancelation "is checked between nodes"
(`evalseam.rs:42`), and the step this unit moves has no nodes.
`mesh::tessellate` takes no `CancelToken` and neither does anything in
`crates/bvh`. So the 6.5 s measured tessellate is uninterruptible, and
6b as originally framed would move an uninterruptible cost onto the
worker rather than a cancelable one.

The three ways out are on the item. Two of them (cancel between roots;
restart-without-cancel) are inside this program's ground and each makes
a *weaker* promise than the evaluation seam does; the third is real
cancel points in `crates/mesh` (MESH's) and `crates/bvh` (CERT's), two
more programs' schedules.

**This goes to 6a rather than to the implementer.** Which promise the
index seam makes is the same question as what its staleness rule is —
a build that cannot be canceled and a build whose result may be stale
on arrival are one design, and §5's inventory should state the
asymmetry with the evaluation seam rather than leave a reader to
discover it. Adding it to 6a costs nothing; discovering it inside 6b
costs the unit.

**And it sharpens the re-cut's condition.** The log's opening entry
said 6b and 6c collapse back into one adversarially-reviewed unit if 6a
rules the staleness rule is not expressible as frame data. Add a second
trigger: if 6a takes option 2 — real cancel points in two other
programs' crates — then 6b is no longer a move within this program's
ground and is not this program's unit to cut alone.

**The pattern itself, now at three instances, is worth stating once.**
`next-id-has-no-layer3-door`, the focus map door, and this all have the
shape: *a ratified or filed design names a mechanism by symbol, the
symbol exists, and the door it needs is closed or on another program's
ground.* All three were invisible from the item text and cost one grep
each to find. The cheap countermeasure is the order this program is
already working in — resolve every cited symbol before cutting the
unit, not at dispatch — so no rule is added; it is recorded here so the
next orchestrator has the reason rather than the habit.

## Unit 1's evidence: both files read end to end (2026-09-04)

A read of `session.rs` (3224) and `app.rs` (5696) end to end, plus
`tools.rs`, dispatched before any design was written. The point of
reading whole files is that nothing else in this project ever does —
specs, diffs and reviews are all per-unit, so accumulation is invisible
by construction — and the read paid for itself: the issue's own
framing is wrong in five places, and four defects turned up that no
unit's diff could have contained.

### Where the issue's framing is now wrong

The issue is from 2026-08-31 and its numbers are the tree as it was.

- **`app.rs` is 5,696 lines, not the 3,474 its comment cites** — 64%
  growth since the sentence naming it "the larger instance of the same
  class" was written.
- **`Tools` holds SEVEN `Option<…Tool>` fields, not six**
  (`tools.rs:113`); `blend` landed after the issue.
- **The per-seat wrong-kind arms the issue predicted were unified** —
  one `WrongNodeKind { node, wanted }`, so that prediction is
  discharged rather than outstanding.
- **The "four hand-lists" over the tool set are down to two** —
  `ToolKind::ALL` (`tools.rs:23`) and the `seated!` invocation
  (`tools.rs:470`), with `commits_a_modal_tool` renamed
  `commits_open_tool` and now delegating to an exhaustive match. Both
  survivors are fixed-length or hand-named, so an eighth tool is
  silently omitted by each; `Tools::open_kind` scans `ALL`, so a
  variant missing from that array makes its tool permanently invisible
  while compiling clean.
- **The "three shapes" of recourse wording are six**, and the const
  the issue records as "removed again in a fix pass" came back in two
  sibling modules (`blend.rs:73`, `frame.rs:343`). The six:
  composing fns on `Refusal` (`session.rs:727/748/757`); a
  wording-bearing struct (`DeleteAffordance::of`, `session.rs:798`);
  19 `Display` arms (`session.rs:638`); a free fn in the app
  (`indeterminate_wording`, `app.rs:146`); a prefixing combinator
  (`ToolKind::says`, `tools.rs:110`, used ~14 times, and `seat_line`,
  `seats.rs:370`); and the two consts. Plus `AtRestBadge::Refused`
  storing a **stringified** refusal (`session.rs:1708`, set at 3199) —
  a typed-values-not-strings exception nothing documents.

None of this changes the unit's direction. It changes what the design
has to answer, and it is why the design was not written first.

### Four defects filed

- `revolve-tool-unreachable-no-axisinplane-form` — **the serious one.**
  `add_revolve` seats a `SketchAxis`, `admits` satisfies that only for
  `Datum::AxisInPlane`, and the datum form offers four kinds with
  `AxisInPlane` not among them. A shipped tool whose seat cannot be
  filled from the running application. Second instance of CHROME's
  `add-profile-mints-no-frame` class, so it is filed with that class
  named and the announce owed.
- `save-is-not-gesture-guarded` — `open` guards, `save` does not,
  thirty-eight lines apart, and the gesture-safe set is not derivable
  from the code (23 guards, no table).
- `two-gestures-can-be-in-flight-together` — `session::Gesture` and the
  free-move gesture share a field name and no guard.
- `opoutcome-superseded-has-no-production-reader` — the GUI reads only
  `.refusal`; a discarded free-move probe reaches the tests and never
  the user.

### Smaller readings, kept here rather than filed

Each has a citation; none asserts a class, so none earns a file.

- `open` (`session.rs:2712`) and `new_document` (`:2777`) are
  near-duplicate reset blocks — **nine landed fields spelled twice**, so
  a tenth must be added in both.
- `standing()` resolves twice per frame for a face or edge selection:
  `app.rs:2913`, then again through `slot_groups` → `slot_rows` →
  `standing` (`session.rs:2002`).
- Three whole-document passes per landing — `product` (`:2064`),
  `at_rest_of` → `assemble` (`:2069`), `run_checks` (`:2078`) — each
  with a comment claiming to be "the one place a result becomes the
  session's", and unbudgeted together. Adjacent to DOCM's
  `check-registry-gathers-product-twice`.
- `ToolKind::ordinal` (`tools.rs:33`) has three call sites, all in
  `crates/viewer/tests/combine_ops.rs`, and none in `src/`.
- One hover sentence spelled twice with different line breaks,
  `app.rs:4779` and `:4816`.
- `DatumKind::ALL` (`app.rs:750`) orders Plane, Frame, Axis, Point; the
  enum declares Plane, Axis, Point, Frame. The order carries UI meaning
  and the divergence is undocumented.
- `PathVerb` (`app.rs:803`, 17 arms, 163 lines) mirrors
  `sketch::PathStep` by hand; `of` is exhaustive but `ALL: [Self; 17]`
  is not.
- `impl ViewerApp` appears twice (`app.rs:1441` and `:1849`) separated
  by nothing but a doc comment — unlike `Refusal`'s split, which at
  least has trait impls between its halves.
- `Refusal::rank`'s comment (`session.rs:617`) orders its two sentences
  against the arms they describe. The code is consistent with the
  intent; the prose is not.
- The README's "Where in the code" row for `src/session.rs` names
  `DocSession`, `SessionOp`, `perform`, `OpOutcome` and is silent about
  `Refusal`, the lowering specs, `DeleteAffordance` and the range
  probe. The split must rewrite that row whatever shape it takes.

### The two facts that decide the split's shape

**The test surface makes a pure module move free.** 459 tests across 44
files, no in-file tests in either `session.rs` or `app.rs`, and
`perform(` appears 500+ times — the suite reaches through
`DocSession::perform`/`SessionOp`, so **not one assertion changes**
under a module move. What a move breaks is import paths: 32 of 44 test
files spell `use viewer::session::{…}` rather than the crate-root
re-exports `lib.rs:139` already provides. Leaving `pub use` shims costs
zero test edits; removing them costs 32 files.

**And `crates/viewer/tests/*` is CHROME's glob, not this program's.**
So the shim removal is not a choice this program can simply make. It is
recorded as its own unit with the announce owed, not as a deferral.

**`app.rs`'s only externally-pinned items are exactly the ones its
header says are not there.** Five test files name six items —
`Pane`, `document_name`, `initial_layout`, `model_stack`,
`indeterminate_wording`, `StartupError`, plus `FieldWriting` and
doc-comment references to `datum_view` and `INITIAL_DELTA` — and every
one is vocabulary or policy, while the entire 2,507-line
`impl ViewerBehavior` (32 `*_ui` fns) has no direct test at all. The
file's header claims "Toolkit adaptation, and nothing else"
(`app.rs:6`), and roughly 900 of its first 1,188 lines contain no egui
call whatever.

## Unit 1's design: one rule, and the split cut in four (2026-09-04)

Written into `crates/viewer/README.md` under **Module boundaries**, to
go to Ev as an `[ev]` PR before anything is cut. `needs_ev` is set on
the item.

**The design is a rule, not a map.** *Every module in this crate is a
vocabulary or a driver, and its `use` block says which* — a vocabulary
names no `DocSession`, no `ViewerApp` and no `egui`; a driver owns
mutable state and dispatches, and there are exactly two. The reason to
prefer a rule is that a map is stale after the next unit and this rule
is mechanically checkable by reading a `use` block. It is also
descriptive rather than invented: it is already true of `camera`,
`frame`, `input` and `display`, and the two files that grew are each
one driver plus vocabulary that never left. That is why the extraction
list falls out of the evidence rather than being argued for module by
module — the inventory found `session::author`'s members reference
`DocSession` **zero** times, and roughly 900 of `app.rs`'s first 1,188
lines contain no `egui` call.

**The unit is cut in four, and the order matters.** Gesture-as-data
comes BEFORE the move: it deletes 23 guards that the move would
otherwise carry into six new modules, so doing it first makes the move
smaller. The move is third and stays purely mechanical. `Option<OpenTool>`
is last.

**Representation changes are deliberately separated from the move.**
The move's entire safety property is that the compiler checks it and
not one of 459 tests changes an assertion. A representation change
folded into an L-size move destroys exactly that property, and the
review has no way to tell which half a failure came from. The plan's
"L-size mechanical refactor" survives as 1c only because 1b and 1d
were lifted out of it.

**Three answers the charter asked for.**

- *`Refusal`'s delegation discipline*: an arm delegates where a module
  below layer 3 owns the failure and its wording, and is flat where
  layer 3 is the only place the fact exists. Applied to the 19 arms it
  classifies 15 and leaves four — `NoSuchSlot`, `NoSuchParam`,
  `ParamExists`, `EmptyName`, all facts about the document, so the
  rule says they should delegate. The README says they stay flat and
  why: moving them changes layer 2's error vocabulary, which is
  `editor-core`'s and therefore DOCM's. Stating the exception is the
  point — a rule that classified all 19 by construction would be a
  rule fitted to the answer.
- *Gesture safety as data*: yes, `SessionOp::gesture_safe`, exhaustive,
  checked once. With two constraints the evidence forced: it changes
  no operation's current answer (`save` included — that stays
  `save-is-not-gesture-guarded`'s question, and a refactor that
  silently fixed it would be a behaviour change smuggled through a
  mechanical move), and it is not one flag for two gestures, since
  `two-gestures-can-be-in-flight-together` shows a predicate reading as
  a guarantee it does not give.
- *The one-of-N tools invariant*: yes, `Option<OpenTool>`. Seven
  fields, not the six the issue predicted, and the argument is stronger
  than the issue's: `Tools::open_kind` scans the fixed-length
  `ToolKind::ALL`, so an eighth tool omitted from that array compiles
  clean and is **permanently unreachable**, which is the same failure
  shape as `revolve-tool-unreachable-no-axisinplane-form` filed today.

**What the design does not settle, said out loud.** The wording family
still has six shapes across five modules and `AtRestBadge` still stores
a stringified refusal. The boundary rule places them; it does not
unify them. Left unnamed, that is the disclosed-blind-spot-read-as-a-
discharge shape `docs/REVIEW-STYLE-DISPATCH.md` §2 names, so the README
says it in its own last paragraph rather than only here.

## The split conversation is with Ev, and three programs are announced (2026-09-04)

**PR #1801**, `[ev]`, from `view/orchestrator` — the branch the #396
convention names, which this session moved to on Ev's word after
opening from a session branch. The orchestrator record before that
point is the same commits, reachable from this branch.

The PR carries `crates/viewer/README.md`'s **Module boundaries**
section and asks three things: whether the vocabulary/driver rule holds
as the boundary, what to do about the four `Refusal` arms the rule says
should delegate to `editor-core` and don't, and whether the four-way
cut is ordered right — particularly gesture-as-data before the move.
It does not ask about the four defects filed today; those are filed and
will be worked in their own units.

**The item is not carrying a `pr:` field.** `work/README.md` is
explicit that an `[ev]` question's PR is not named in the item — "which
PR carries the question is one `git log` away" — and `needs_ev: true`
is the signal `STATUS.md` renders. Practice on the board is mixed
(CHROME's `viewer-first-light-on-real-hardware` carries `pr: 1771`),
and the contract is followed here rather than the precedent. The number
is recorded in this log, which is where a narrative fact belongs.

Woken on comments by a PR subscription on this box, per the same
section's requirement that a question nobody is listening to has not
been asked.

### Announced, on the PR thread

- **DOCM**, the substantive one: the `next_id` door DI1's own walk
  needs and cannot reach, with the two candidate shapes named and the
  choice left theirs; and the correction that BOTH endpoints of the
  focus map are their glob, not S-BOOL's. Plus one adjacency offered
  and not filed — the three whole-document passes per landing, next to
  their `check-registry-gathers-product-twice`.
- **CHROME**: their nine parked rows now have a moving trigger, and
  `session-gesture-guard-spelled-thirteen-times` is confirmed at 23
  guards and likely dissolved rather than relocated by gesture-as-data,
  which is what their own log predicted. The revolve-unreachable defect
  handed over as theirs by charter, with the class named and the
  re-home left to their next move. The test-glob ask for the shim
  removal, with the sweep's blind spots stated.
- **S-BOOL**: no ask and no diff — VIEW will not touch
  `crates/profile`. What they are told is that a change to
  canonicalization changes the map's answers *silently*, since the map
  keeps type-checking and starts lighting the wrong edge; and that if
  they would rather the map lived beside the canonicalization it is
  faithful to than beside the coordinate it translates from, that is
  theirs to say. VIEW holds no position on which.

MESH and S-CERT are **not** announced yet: their cancel points are only
needed if 6a takes that option, and announcing a need that a ruling may
not produce would be asking two programs to hold schedule for a
hypothesis.

## #1801's body rewritten: an `[ev]` PR is a decision document (2026-09-04)

Ev, on the PR as first written: too much unnecessary detail — corrected
line counts and the like — and the things to decide were hard to find.
Correct. The body led with ~150 lines of evidence and put the three
questions last, so the reader had to reach the end to learn what was
being asked and then reread to find which paragraphs bore on it.

Rewritten to three decisions, each carrying only the context needed to
take it and the alternative rejected, with everything else behind one
pointer to this log. What came out: the five ways the issue's framing
was stale, the four filed defects in detail, the three closed doors in
detail, the plan corrections, and the posture restatement — all of it
already recorded here, which is the argument for cutting it there.

The rule is general rather than VIEW's, so it is in
`memories/ev-profile.md` and not only here: every program routes its
questions to Ev through this shape, and the long form is the natural
default when the evidence was expensive to gather. Cost of the finding
is one edit; cost of not having it is Ev's attention on every `[ev]` PR
the board opens.

## Ev's answer, and a claim of mine that did not survive it (2026-09-04)

Ev on #1801: the boundary rule is accepted ("sure"), with the
suggestion to have subagents split the monster files; the sequencing is
mine to choose and the four-way cut makes sense; and on the four
`Refusal` arms, one question — **"what was the reason to move them?"**

**There wasn't one for three of them.** The PR asserted that
`NoSuchSlot`, `NoSuchParam`, `ParamExists` and `EmptyName` "are facts
about the document, so by the rule their owner is `editor-core`". That
was written from the arm names, not from the raising sites, and reading
the sites says otherwise:

- **`EmptyName`** (`session.rs:2783`) validates a document *name*
  string before `Doc::empty_derived`. Not a document fact at all.
- **`ParamExists`** (`:2590`) is a deliberate layer-3 NARROWING —
  `DocEdit::SetDocParam` is create-or-replace and the session's create
  door refuses replacement. Its own doc-comment already says so.
- **`NoSuchSlot`** (`:2299`, `:2454`) means the *properties panel* has
  no row for that slot (`props::slot_rows`), a layer-3 projection, not
  editor-core's slot vocabulary.

**The fourth is real, and it is a defect rather than a boundary
question.** `set_param` (`:2577`) pre-checks that a parameter exists
before committing `DocEdit::SetDocParamValue`, which `apply` already
refuses as `EditError::DocParamNotDeclared`
(`crates/editor-core/src/edit.rs:429`) — and `Refusal::Edit` already
delegates, so deleting the pre-check surfaces the door's own answer.
Filed as `set-param-prechecks-what-the-door-refuses`. The other two
`NoSuchParam` sites are lookups with no edit behind them and are
correctly flat.

**So the README's exception paragraph is gone and the discipline gained
its sharp edge instead**: *a flat arm must not restate a refusal a door
already gives*, with the lookup-versus-pre-check distinction that
separates the real case from the three false ones. The codebase already
stated this rule in `delete_node`'s doc-comment — *"the typed refusal
comes from the door rather than from here"* — one screen from the
violation, which is the ordinary way this project's rules are broken.

The lesson is not "check the code", which is already the rule. It is
that **an exception list is a smell**: four arms that would not fit
should have prompted re-deriving the rule rather than fencing them off,
and the fence is what let an unchecked framing reach a design doc. The
rule that replaced it classifies all nineteen arms and names a defect
the exception list was hiding.

## Unit 1 ratified and merging; 1b dispatched (2026-09-04)

Ev signed off on the boundary rule and the sequencing on #1801, the
`Refusal` question is answered above, CI green on the head carrying the
correction. `needs_ev` cleared; the item goes `dispatched` on 1b's
branch.

**1b is out**, `view/1b-gesture-as-data`, style review to follow per
this program's posture. Its brief carries the three constraints the
evidence forced — change no operation's current answer (including
`Save`'s), do not let one predicate read as a guarantee over two
unrelated gestures, and derive the guarded/unguarded split from the
tree rather than from the dispatch. That last one is deliberate: the
list in the brief is the orchestrator's reading, and
`docs/prompts/reviewer-style-lane.md` is explicit that a dispatch is a
hypothesis. A lane correcting it is the lane working.

**On Ev's "have subagents split up the monster files".** Yes, and the
bound worth being concrete about is that **parallelism here is per
FILE, not per module**: six lanes each extracting one module from
`session.rs` would spend their time resolving merge conflicts on one
file. So 1c is two lanes — one for `session.rs`, one for `app.rs`,
which are independent — each doing its whole file's extraction, after
1b lands and shrinks the session lane's job by 23 guards.

## 1b landed on a branch that had lost it, and a guard reported a pass it never ran (2026-09-04)

The unit itself is good and is PR #1816: 23 guards replaced by one
exhaustive `SessionOp::permitted_during_value_gesture` checked once in
`perform`, 26 refused and 13 permitted, no behaviour change. **The
lane corrected the dispatch twice** — `AddPlacedUnion` was missing from
the guarded list (eleven creation doors, not ten: counting guard LINES
undercounts OPS by two, since `AddPattern`/`AddPlacedUnion` share
`add_pattern` and `AddFillet`/`AddChamfer` share `add_blend`), and
`PreviewGesture`/`CommitGesture`/`CancelGesture` needed rows the brief
never gave them, permitted, because guarding them would leave a drag
with no way to end. Both corrections are the reviewer-brief's rule
working as intended: a dispatch is a hypothesis.

**Two process failures, both this orchestrator's.**

**The state-sync commit was built on a stale tree.** `070be390` — the
one clearing `needs_ev` after Ev's sign-off — has `14f1f9b4` as its
parent, not the branch tip, so its push moved nothing and #1801 merged
without it. Main therefore carried `needs_ev: true` on a question Ev had
already answered, and `STATUS.md` showed VIEW waiting on Ev when it was
not. Corrected: the clearing rides #1816, which is where state-sync
belongs anyway.

**The lane's branch pointer was left behind its own commit.** Subagents
here share this checkout rather than getting per-lane worktrees, so the
lane branched from the shared HEAD (picking up `070be390`), then reset
onto merged main and committed its work as `0ad4274e` — which ended up
on no branch at all, while the remote ref still pointed at the
orchestrator commit. Recovered by merging `0ad4274e` back onto the
branch, never a force-push; nothing was lost, because a commit object
survives being unreferenced.

**The part worth generalising is the second-order failure.** The lane
reported in good faith that `work.py territory` found **0 paths in
another program's territory**. Re-run against the recovered tree it
names two — `crates/viewer/tests/all.rs` and `gesture_table.rs`, both
CHROME's and TCOST's. The tool is fine. It saw a tree without the
lane's work, and a guard run against the wrong tree reports a pass.
That is the same shape as the existing rule about confirming a
`Compiling <crate>` line before trusting a build, one level up, so it
is recorded in `memories/agent-lane-operations.md` beside the
branch-ref hazard it extends rather than filed as a defect.

**The lane got the harder call right**: it could not trigger hosted CI
(`ci.yml` runs on `pull_request`, `push: main` and `workflow_dispatch`;
a branch push starts nothing and `workflow_dispatch` 403s for its
token) and it SAID SO rather than offering its local runs as the gate.
That is the discipline doc's rule followed exactly on the one occasion
it cost the lane something.

CHROME's test glob is touched by this unit (`gesture_table.rs`,
`all.rs`); the announce is owed with #1816 rather than assumed.

## The fix pass hung after editing, and what that cost (2026-09-04)

The style review on #1816 returned no MAJOR and did not block the
merge. Its sharpest findings were about claims rather than code, and
the sharpest of those was mine.

**`crates/viewer/README.md`'s *Gesture safety is data* section — landed
on main by #1801 — described this unit in FUTURE tense, on a premise
sentence 1b had just made false, and named `SessionOp::gesture_safe`,
an identifier that exists nowhere in the tree.** The shipped name is
`permitted_during_value_gesture`. So for the time between the two
merges the project's design doc of record named a symbol that does not
exist, and `scripts/doc-gate.sh` could not have caught it: that gate is
rustdoc-only and never opens a README. The dead name was in four files,
not the one the review found — the plan and two item files carried it
too, and `plan.md` is what lanes 1c and 1d read to learn what 1b did.

Also landed: the sweep of prose citing the old per-site guard was a
half-fix (one site rewritten, three left stating the rule unlinked, one
of them load-bearing); two doc comments claimed the permitted arms
"carry no guard of their own" when four do, which is true only under
the reading this unit exists to prevent; an open question was written
up in code as settled design; and the `Refusal::rank` paragraph's
"exhaustive, so a new arm is compiler-caught" is true over `Refusal`'s
arms and false one level down, where `Display(_)` is a catch-all that
ranks a new `DisplayFault` by default.

**The review earned its keep by mutating rather than reasoning.** It
flipped `Save` in the predicate alone: `the_table_answers_for_every_op`
went red and `every_op_behaves_as_the_table_says` stayed green — which
proves the second copy is genuine AND proves the behavioural row cannot
catch a wrong table entry, since both sides of its assertion read the
same predicate. The PR body and the test header both claimed more than
that. Reversing the whole table showed 20 of the 26 refusals have an
external witness and six do not.

### The lane hung, and the shared checkout made that expensive to see

The fix-pass lane finished its edits by 07:52, wrote its target
directory until 08:00, and then did nothing for over two hours while
still reporting as running. No `cargo`, `rustc`, `rustdoc` or `nextest`
process existed; disk was not exhausted (9.5 G free). Its last words
were that it was about to run a verification mutation and wanted to
commit first so it could revert — so it hung between editing and
verifying.

Because subagents share this checkout, its work sat as six modified
files in the orchestrator's working tree for two hours. **That is the
same shared-checkout hazard recorded earlier today, in its third
form**: first an orchestrator commit landing on a lane's branch, then a
lane's commit orphaned by an orchestrator merge, now a dead lane's
uncommitted work indistinguishable from the orchestrator's own dirty
tree. The rule already in `memories/agent-lane-operations.md` covers
the recovery; what this instance adds is that **a stop-hook or any
"you have uncommitted changes" prompt is not authority to commit**,
because the tree may belong to a live lane.

Recovered by reading all six files, then verifying what the lane never
reached: `cargo fmt --check`, `work.py lint`, the three new rows, the
full 466-row viewer suite, `clippy -p viewer --features app
--all-targets`, and `cargo doc` under `-D rustdoc::broken_intra_doc_links`
— that last one because the fix pass ADDED intra-doc links
(`DisplayState`, `DisplayFault::FreeMoveInFlight`) and an unresolved
link is exactly what the doc gate fails on. All clean. The lane's
target directory was warm, which is how its hang was placed after
compilation rather than during.

**Cost of the hang: nothing but time.** Hosted CI is the verification of
record here, not a lane's local runs, so a lane dying after its edits
loses only the local pre-check.

## 1b merged; an inherited item swept; 1c dispatched into isolated worktrees (2026-09-04)

**1b is on main** (#1816). The mid-gesture policy is one exhaustive
table checked once, the README states it in the present under its real
name, and each row in `gesture_table.rs` says what it is worth rather
than sharing one overstated claim.

**`blamed-mates-lost-its-exhaustive-arm` arrived on this slate from
FILLET-E3** while 1b was in flight. Its code half was closed on
arrival; it left two questions and this program owns one of them.

Swept: **`blamed_mates` is not the only exhaustive match on `MateFault`
outside `editor-core`** — `crates/pncad-py/src/tags.rs:400` is a second,
and it is **LIB's** ground, and it did get its `Unleverable` arm. Both
are correct as of today, and both were repaired by someone who happened
to be looking, which is the part that should not be relied on again.

The finding worth carrying is about the ones that did NOT break:
`pncad-py/src/py/mate.rs` wildcards `MateFault` in eight accessors and
`viewer/src/app.rs:2880` in one. They cannot fail to compile, and that
is not safety — a new fault arm naming a mate returns `None` from every
one of them, silently, which is precisely what `blamed_mates`'s
doc-comment says its exhaustiveness exists to prevent. **The wildcards
are the same defect with the compiler switched off.** Whether
`MateFault` should be `#[non_exhaustive]` is DOCM's
(`crates/editor-core/src/mate.rs`), and the tree already holds both
patterns with no stated rule for choosing — `pncad-py`'s own module doc
names `select_refusal_tag` as a documented `#[non_exhaustive]`
exception. Announced to DOCM and LIB; the CI half (a draw that can hide
a hard compile break on `main` for an unbounded number of merges) is
CIW's and announced there.

### 1c runs in worktrees, because the shared checkout has now failed three ways

Two lanes editing `session.rs` and `app.rs` are independent by file,
which is why 1c is two lanes rather than six. But this session has now
watched the shared checkout fail three times — an orchestrator commit
landing on a lane's branch, a lane's commit orphaned by an orchestrator
merge, and a dead lane's edits sitting in the orchestrator's tree — and
two CONCURRENT lanes in one working tree is not a variant of those
hazards, it is the guaranteed form of them: two agents editing one tree
with one HEAD.

So both 1c lanes get **their own git worktree**. That is a per-lane
checkout, which is what `memories/agent-lane-operations.md`'s branch-ref
rules assume exists and what the remote-session default does not
provide. It is the structural fix rather than a discipline one, which
is the preference this project states everywhere else.

## 1c-session landed; two dispatcher errors and one of mine (2026-09-04)

**`session.rs` is 3,260 → 1,484 lines** across the six vocabularies the
README names, on `view/1c-session-split`. No test file touched, no
assertion changed, 466 + 467 rows green, clippy and the real doc gate
clean, remote ref verified equal to local. The lane audited it
line-by-line rather than trusting the compiler: every non-blank
non-comment line of the old file accounted for in the new set.

One restructuring, disclosed: `probe_bounds` split into a free search
function and a driver method that keeps the guard-then-store order, so
the driven-slot guard still runs before anything and every refusal
still returns before `self.bounds` is written. Eighteen intra-doc links
repointed — as predicted, that was the unit's main breakage source.

### The briefs told both 1c lanes to do the wrong thing

Both briefs said to put `CI-Config: lane=default` on the head commit.
**That instruction was stale by two hours when I wrote it.** `ci.yml`
and `docs/prompts/implementer-discipline.md` changed on main at ~08:22
(`bb17cfbc7`): an un-narrowed run now gates the whole
`{default, interval} × {default, 1e-6, 1e-12}` matrix as twelve test
jobs, and the trailer **narrows** rather than requests. Following the
brief would have bought these units strictly less gate than doing
nothing.

The session lane caught it, declined, and quoted the new text back.
That is the **third** dispatcher error the 1b/1c lanes have corrected —
after the missing `AddPlacedUnion` and the three gesture ops with no
row — which is the reviewer brief's "a dispatch is a hypothesis" rule
earning its place three times in one unit chain. The app lane has been
told directly, mid-flight.

**And the same staleness reached a merged PR body.** #1816's
Verification section claims the run was "drawn, not asked for" and that
interval and the other tolerance rows "were not seen". True of the
07:37 run, false of the 10:07 run it merged on — which gated *more*
than the body claims. Corrected in a comment beneath rather than by
rewriting merged history, because that paragraph exists precisely so a
reader need not assume what the gate saw.

### The disk finding is mine, not a missing rule

The root filesystem hit 100% of 252 G during the session lane's run;
its doc gate aborted with ENOSPC and the harness's own tmpfs went
unwritable. Reclaimed ~10 G by removing the merged VIEW-1b lanes'
target directories.

The lane reported this as a gap — "the missing half is a teardown step
rather than a rule change". **It is not a gap.**
`memories/agent-lane-operations.md` already says to reclaim a lane's
target **"when a review returns, not when a lane runs out of disk — a
review lane's `target/` is pure waste the moment its report is in
hand, and review lanes are the biggest consumers."** `view1b-review-target`
should have gone when the style review came back hours earlier, and
`view1b-target` when #1816 merged. I ran three lanes without doing
either. No memory is added for this, because a second copy of a rule
nobody followed is not the fix.

## 1c is built: both files split, verified together (2026-09-04)

| file | was | now |
|---|---|---|
| `session.rs` | 3,260 | **1,484** |
| `app.rs` | 5,696 | **1,754** |

Thirteen new modules. `view/1c-module-split` merges both lanes' branches
(they merge clean — the session lane declared its submodules inside
`session.rs`, the app lane touched `lib.rs`, no overlap).

**The combined tree was verified here, which neither lane could**: each
verified its own half in isolation and a clean textual merge is not a
compile. 466 rows at default features, 467 with `--features app`,
clippy clean, fmt clean, and the rustdoc gate clean under
`-D rustdoc::broken_intra_doc_links` — 27 intra-doc links were repointed
between the two lanes, which was the predicted breakage source and the
reason that gate is the one that matters here.

Neither lane touched `crates/viewer/tests/`. Both audited their move
line-by-line rather than trusting the compiler: the app lane diffed the
multiset of visibility- and whitespace-normalised non-import lines
across all thirteen resulting files and accounted for every difference.

### Two things the app lane surfaced

**The drag-tick family has one home: `forms`.** It has three consumers
in three new modules, so only `forms` or `app` could serve all three,
and `app` would have reinstated the very header problem 1c exists to
fix. That closes the substance of CHROME's `drag-tick-has-three-homes`
— the RULE now has one home, though the three call-site spellings are
unchanged, which is what that item is actually about.

**And the ratified README was wrong about its own module again.** The
`forms` row said its members are "each a hand-maintained mirror of a
kernel or sketch enum". `FieldWriting` and the four drag speeds are
neither — they mirror nothing and are a product decision on their own.
The lane spotted it, left the ratified text alone, and told me. Fixed
here. **That is the third time in one day that a design doc I wrote
made a claim the tree does not support**, after the dead
`gesture_safe` symbol and the four-`Refusal`-arms framing.

The pattern is now clear enough to name: **I write the design from the
inventory, and the inventory is a snapshot.** Every one of the three
was a sentence that was true when written about a tree that then
moved — and none was caught by a gate, because no gate reads prose for
accuracy. Item #1 of CHROME's `app-rs-doc-comment-merge-scars` is the
same hole seen from the other side: a doc comment that renders literal
`///` and passes every check. The countermeasure that
has actually worked all day is a reader with the tree open — three
lanes and one reviewer caught all four instances between them.

## 1c fix pass: the style review's findings (2026-09-04)

**The review falsified the README's boundary rule on the very module
the split had just added.** `widgets` sat in the table headed *The
app's vocabularies* while naming `egui` (`widgets.rs:13`) and taking a
`&DocSession` (`delete_button`) — the crate's newest module fitting
neither side of a binary rule whose selling point is that it is
mechanically checkable. The rule is right; the classification was
wrong. `widgets` and `pane::*` are the `app` driver **split for
size**, and splitting a driver across modules does not make the pieces
vocabularies. The README says that under its own heading now, and
`widgets.rs`'s header says it of itself.

`session.rs` gained the declaration its other half already had: that
it is a driver, and which six vocabularies sit beside it. Thirteen of
the fourteen modules in this unit opened by declaring their kind;
`session.rs` was the one that did not, and a reader learned of its
vocabularies only from the `pub mod` lines.

Three prose defects fixed: `drag_tick`'s "three constants" against its
four-arm match; the `datum_view` shim in `app.rs`, a dead `pub use`
kept alive by a comment asserting a caller that does not exist (no
`viewer::app::datum_view` anywhere in the workspace — `datum_draw.rs`
imports `viewer::datums`); and `FieldWriting`'s "a third home … filed
rather than fixed here", which stopped being true the moment `forms`
became the one home. Sizes after the pass: `app.rs` 1,752,
`session.rs` 1,500.

**Two of this program's own tracker files were wrong.**
`tip-mark-doc-duplicates-its-own-first-sentence` duplicated item #1 of
CHROME's `app-rs-doc-comment-merge-scars`, filed the same day, parked
on this same split, and explicit that its three scars are one class of
defect with one fix. I filed it without reading the board — the exact
failure `docs/prompts/implementer-discipline.md` §6 names, committed
by the party who is supposed to be able to see the whole board.
Deleted. `session-shims-and-test-imports` claimed its whole list was
already re-exported at the crate root; `AtRestBadge` and `admits` are
not, and `admits` is imported by no test at all, so the scheduled
sweep is less mechanical than it advertised.

Three items filed. `boundary-rule-has-no-mechanical-check`: nothing
reads a `use` block, so the ratified rule's "mechanically checkable"
rests on a mechanism that does not exist.
`stale-file-citations-after-the-split`: 24 open files cite
`app.rs:NNNN` or `session.rs:NNNN` for items this PR moved, green
under the rustdoc gate because every one is an unbracketed code span.
And `loud-skip-marker-says-two-modules-and-there-are-six` — **a fifth
prose claim this crate outran in one day**, found while checking the
fourth: `lib.rs:90`'s loud-skip marker says "the two modules above"
over six `#[cfg(feature = "app")]` modules, having predicted its own
staleness in the next paragraph. Not fixed here because its payload is
a `println!` naming two modules and a test named after them, which is
a decision rather than a typo; this pass was scoped to prose with no
decision in it.

## Unit 1 is done but for 1d, and the review found the rule wrong about its own newest module (2026-09-04)

#1830 merged. `session.rs` 3,260 → **1,500** and `app.rs` 5,696 →
**1,752** (the fix pass's own edits moved both slightly from the
figures the PR table carries). Unit 1's remaining part is **1d**,
`Option<OpenTool>`.

**The style review audited differently from both lanes, and that is
why it found things.** Both lanes had diffed sorted or multiset line
sets; the reviewer extracted all 237 `fn`/`const` items and every type
definition by brace-matching and diffed each body **in order**, which
sees a moved line or a reordered statement that a multiset cannot.
Three independent audits of one mechanical move, each shaped
differently, and only the third could have caught a reordering. Worth
keeping as method the next time a unit's safety rests on "the compiler
checks it".

Claims 1–3 survived. **Claim 4 did not.**

### `widgets` obeyed neither side of the ratified rule

The README filed `widgets` under *The app's vocabularies*; the rule
says a vocabulary names no `DocSession`, no `ViewerApp` and no `egui`.
`widgets.rs` names `egui` at `:13` and `DocSession` at `:20`, used at
`:518` where `delete_button` takes `&DocSession`. It is not a driver
either. So the crate's newest 525-line module fitted **neither side of
a binary rule whose selling point is that reading a `use` block decides
it** — a harder failure than the day's other four, which were stale
sentences rather than a hole in the classification.

The rule survives; the classification was wrong. `app` is a driver, and
a driver too large to read is still a driver: `app.rs`, `pane::*` and
`widgets` are one driver split for size, and splitting a driver does
not make the pieces vocabularies. The README says so under **The app
driver, split for size**, and the check now reports what a module IS
rather than only whether it is a vocabulary.

### Two of my claims, and one item I should not have filed

The PR body said 1c "closes the substance of" CHROME's
`drag-tick-has-three-homes`. The code contradicted me —
`forms.rs` still said the rule had a third home — the item's two
questions are untouched, and 1c made one half **worse**: the hand-picked
constant call sites went from one file to three. 1c gave the RULE one
home, which is not what that item is about. The "two residues" in
`app.rs` also undercounted, and `app.rs` was 1,754 lines and not the
1,746 the PR, this log and my brief all carried.

And `tip-mark-doc-duplicates-its-own-first-sentence`, which I filed
this morning, was **item #1 of CHROME's `app-rs-doc-comment-merge-scars`**
— filed the same day, parked on this very split, explicit that its
three scars are one class with one fix. I wrote that mine was "the one
instance found of a shape nothing in this repo checks"; it had already
been found. `implementer-discipline.md` §6 tells lanes to report rather
than file *because they cannot see the whole board*. The orchestrator
is the party who can, and I did not look. Deleted.

### A fifth stale claim, found while fixing the fourth

The fix pass found `lib.rs:90`'s loud-skip marker saying "The two
modules above" over **six** `#[cfg(feature = "app")]` modules — and
that marker's own next paragraph predicts exactly this: *"a marker that
silently went stale would look exactly like this one."* It declined to
fix it, correctly: the sentence is one word but the payload is a
`println!` naming two modules and a test named after them, so which
modules get named is a decision. Filed as
`loud-skip-marker-says-two-modules-and-there-are-six`.

**That is five prose claims outrunning the tree in one day, four of
them mine.** The two items filed today are the two halves of the
countermeasure: `boundary-rule-has-no-mechanical-check` (the README
calls the rule mechanically checkable and nothing reads a `use` block)
and `stale-file-citations-after-the-split` (24 open files cite moved
lines; the rustdoc gate sees only BRACKETED intra-doc links, which is
precisely why every survivor is an unbracketed code span and the gate
is green). Until one of those lands, the only thing that has caught any
of the five is a reader with the tree open.

### Recovered, again: two log entries that never left this branch

The merge of main after #1830 conflicted because **`1b merged; an
inherited item swept…` and `1c-session landed…` were never on main** —
they were committed here while 1c branched straight from main. Same
shape as the stale-tree state-sync commit earlier today, and the same
lesson: this branch is not a place work becomes durable. Resolved as a
chronological union. **The orchestrator branch needs a PR of its own
before this session ends**, or the day's whole record lives on a branch.

## 1d built; a parking decision of mine was resting on a false premise (2026-09-04)

`Tools`'s seven `Option<…Tool>` fields are one `Option<OpenTool>`
(#1832). The lane answered the gating question by enumeration rather
than assumption — **two tools open was already unreachable**: the
fields were private, no struct literal for `Tools` exists anywhere in
the repo, the only writers were `open` (reset, then one match arm) and
`close`, and `feed`/`reconcile` reach contents through `as_mut()` and
never touch a discriminant. So this is a representation change with no
behaviour change, and `ALL`'s ordering inside `open_kind` was dead code
resting on that same fact.

**`seated!` and its `Seated` trait are gone. `ALL` could not go** — six
uses in `crates/viewer/tests/`, CHROME's glob. What changed is the
sharp edge: `open_kind` no longer routes through the array, so a kind
missing from `ALL` now narrows a sweep instead of making its tool
permanently unreachable. That was the failure shape this unit was for,
and it is the same one as
`revolve-tool-unreachable-no-axisinplane-form` one level up. Half the
stated goal, disclosed as half.

### The correction that matters

`pick-priority-filter-vocabulary` was parked on the split because I
wrote that `ToolKind::pick_kinds` is "one of the hand-maintained lists
over the tool set that the split's `Option<OpenTool>` step collapses".
**It was already an exhaustive match.** There was nothing to collapse,
1d leaves it byte-identical, and the blocker I named never gated the
item. The lane found this and correctly declined to edit the item —
§6's duplicate-filing hazard cuts both ways, and an item carrying an
orchestrator's rationale is the orchestrator's to fix.

That is the **sixth** dispatcher correction in unit 1's chain and the
second against a decision rather than a detail. The first was the
`Refusal` four-arms framing on #1801; both were written from names and
plausibility rather than from the raising sites, which is the same
error twice.

### And the tracker has no word for what this item actually is

Un-parking it is not an improvement — it is the least bad of two wrong
states. The item is not dispatchable: its trigger is a vertex-pick tool
that does not exist and is not scheduled, and README GQ7 ratifies the
deferral. But `parked` requires `blocked_on` to name an item or a PR,
and **nothing on the board names either the absent tool or a ratified
prose deferral**. Between a status that overstates availability and a
`blocked_on` naming something that does not gate it, `open` loses less:
a reader who opens the file meets the truth in its first paragraph,
where a false `blocked_on` would have gone on being believed unread —
as mine was, for a day.

CHROME hit the adjacent shape when it parked nine items and had to
argue in prose why. Two instances is not a rule; a third is worth
putting to Ev.

## Unit 1 is closed (2026-09-04)

Four PRs: #1801 ratified the boundary rule, #1816 made gesture safety
data, #1830 split both files, #1832 made the one-of-seven tool
invariant unrepresentable. **`session.rs` 3,260 → 1,500 and `app.rs`
5,696 → 1,752**, thirteen new modules, and across the whole chain **no
test file was touched and no assertion changed**. CHROME's nine parked
items are unblocked and told.

### 1d's review, and the finding that should not be lost

All four claims survived. Two things came out of it worth more than the
diff:

**The reviewer verified the no-behaviour-change argument leg by leg and
then added a leg nobody had considered.** `Tools` derives `Debug`, and
derived `Debug` output IS observable behaviour that the change alters.
It holds only because nothing formats a `Tools`. So the claim survives
**by luck rather than by the enumeration**, which covered constructors
and writers and not observers. *Who can construct this* and *who can
observe this* are different sweeps, and the second is the one that gets
skipped.

**`ToolKind::ALL` had exactly one production reader — `open_kind`'s
scan — and 1d removed it, leaving zero.** `ordinal` had zero already.
So they are a `pub` pair existing solely for a test to sweep a list only
that test reads, guarded by a second test. Same class as
`opoutcome-superseded-has-no-production-reader`, filed by this program
this morning. And the reviewer's Q6 point landed exactly: **1c filed an
item for this shape of CHROME-glob residue and 1d filed none.** Now
filed as `tool-kind-all-and-ordinal-have-no-production-reader`.

### The seventh correction, and it is the one I was warned about

The fix-pass lane checked a claim I had passed through and found it
false: `forms::BOOLEAN_OPS` and `MATE_PRIMITIVES` are **not** that
class — both are `pub(crate)` and both have production readers
(`pane/create.rs:129`, `:144`, `:876`). The reviewer had named them as
unswept siblings; I relayed that into a dispatch brief as an
instruction without checking it.

`docs/REVIEW-STYLE-DISPATCH.md` §3 names this exactly: *"A lane's
unverified observation, repeated back to it as an instruction, arrives
carrying the dispatcher's authority and is one commit from a ratified
doc. Check a lane's claim before you build a brief on it."* It says the
rule "binds the dispatcher hardest", and it was right. The lane caught
it because the brief also told it the brief was a hypothesis — the two
halves of that posture are what saved it, not either alone.

**Seven dispatcher corrections across unit 1's chain**, two against
decisions rather than details. Every one improved the unit. The posture
that produced them is cheap: state the dispatch's claims as claims, and
say so.

### Where the program stands

Unit 1 closed. **Thirteen open items, one parked**, nothing dispatched,
no lane running. The three items that gate other work are all waiting on
other programs: `next-id-has-no-layer3-door` (DOCM), the focus map door
(DOCM and S-BOOL), and the pick-index seam ruling (an `[ev]` PR nobody
has opened). The plan's items 3–6 are all still ahead.

## Orchestrator handover; three lanes out; two false tracker rows (2026-09-04)

The previous orchestrator exited with unit 1 closed, thirteen open
items, one parked and no lane running. Picked the program up cold from
`work/view/plan.md` and this log's tail. Nothing was lost: the working
tree was clean, the branch was exactly `origin/main`, all four of unit
1's PRs were merged, and there were no open `view/` PRs. `lint` was
green — which is part of the finding below.

### Ev's ruling on review posture, taken in chat

**No A/B duals and no row in `docs/MODEL-AB-LOG.md`, whatever review a
unit gets.** Style review is the default; a second correctness reviewer
where the failure mode is a confident wrong answer rather than a
refusal, dispatcher's judgement, argued in the dispatch. The band
1900–1999 stays claimed and empty. Recorded in `plan.md` so a
successor does not re-derive it.

### Two rows the board was carrying falsely

**`session-shims-and-test-imports` was parked behind an item that
closed.** `viewer-session-god-module-split` closed 2026-09-04; this row
went on reading `parked` for a day. `lint` does not object — a closed
`blocked_on` resolves fine — so a trigger that fires makes nothing go
red. **CHROME has nine rows in the same state**, all parked behind the
same closed item, and this log has been claiming since unit 1 closed
that they were "unblocked and told". The board says otherwise; being
told is not a status.

Un-parked rather than re-parked, for the reason the previous
orchestrator argued on `pick-priority-filter-vocabulary`: between a
status that overstates availability and a `blocked_on` naming
something that does not gate it, `open` loses less, because a false
`blocked_on` goes on being believed unread. That is now the second
VIEW row and the ninth CHROME row whose real state this vocabulary
cannot spell — the third instance the previous orchestrator said was
worth putting to Ev. Filed as
`tracker-has-no-status-for-an-unscheduled-trigger`, `needs_ev`, with
four candidate shapes and one half that needs no ruling at all: **lint
could refuse a `parked` row whose `blocked_on` names a CLOSED item**,
today, with no vocabulary change, and three of CHROME's nine would
have gone red on the commit that closed the split.

**`blamed-mates-lost-its-exhaustive-arm` was open over a fix that had
landed twice.** The arm is at `crates/viewer/src/tree.rs:325` and has
been for a day. What kept the row open was its two residues, which
were "announced" *in prose inside the item and nowhere else* — the
exact shape `work/README.md` names as invisible to the re-homing
sweep, so both would have died with this directory at close. Given
files, in `work/issues/` because neither owner's slate is VIEW's to
write on:
`ci-draw-can-hide-a-compile-break-on-main` (**CIW's** — the `filter`
draw can hide a hard compile break on `main` for an unbounded number
of merges, and the 2026-09-04 twelve-job widening may close it for PR
runs but not obviously for `main` push runs) and
`mate-fault-accessors-wildcard-into-silence` (**LIB's ground, DOCM's
ratification** — ten `_ => None` accessors over `MateFault`, which is
the same defect as the missing arm with the compiler switched off).
Item closed.

### The citation sweep, and the one file it was wrong about

Paid VIEW's half of `stale-file-citations-after-the-split`: five files
in `work/view/` re-pointed against `d799235e`, each carrying a note so
a reader can tell a re-point from a claim change.

**One was not a re-point, and it is this program's hazard again.**
`save-is-not-gesture-guarded` reasons from 23 `if self.gesture.is_some()`
guards at 23 call sites, and from `open` carrying one where `save`
does not. **VIEW-1b deleted that mechanism** — the rule is one
exhaustive table at `session/op.rs:586`, two `is_some()` reads survive
and neither is a dispatch guard. Correcting the two line numbers would
have produced a file whose citations resolve and whose sentences are
false, which is *more* dangerous than broken numbers, because a
resolving citation reads as checked. Recorded against the general case:
**a citation gate that resolves numbers would have passed the one file
whose claim had gone stale.** That is a real limit on the guard
`stale-file-citations-after-the-split` proposes, found by paying the
cheap half.

That makes seven prose claims outrunning this tree in two days.

### Three lanes out

Style review each unless the meta-review says otherwise; no A/B rows.

- **`view/status-lifetimes`** — plan item 3. The design call, made here
  and stated in the brief AS a call rather than a fact: *the status
  line carries per-frame NEWS and `frame::frame_status` owns its
  ranking; a fact that stays true after the frame ends is not news.*
  The item filed the choice as open between three shapes; the reason
  for calling it is that shape (3) already exists in the tree —
  `frame::frame_status` (`frame.rs:103`) ranks refusal > every notice
  joined > the batch verdict, and argues it at length — and `land`
  (now `pane/viewport.rs:26`) predates the rule and bypasses it. What
  is left is a lifetime split, and the item's own prose reaches it
  twice. **Disclosed as a call over an item that says "none obviously
  right"**; the lane is told to report if it thinks the split is
  wrong. Fenced hard: the ~15 further direct writers of the line
  (`pane/create.rs` x10, `pane/view.rs`, `pane/viewport.rs` x3,
  `app.rs` x3) are censused and FILED, not refactored — CHROME has
  nine newly-unblocked items in those same files. The item's
  four-writer framing is an undercount and the brief says so as a
  claim to check.
- **`view/set-param-precheck`** — `set_param` pre-checks what
  `DocEdit::SetDocParamValue` refuses typed, plus the sweep for the
  class (42 `OpOutcome::refused` sites in `session.rs`). Style review:
  the failure mode is a refusal's text changing, which CI reports
  loudly. The brief names the trap explicitly — a test asserting
  `NoSuchParam`'s wording must not be silently re-baselined.
- **`view/module-kind-gate`** — `boundary-rule-has-no-mechanical-check`
  plus `loud-skip-marker-says-two-modules-and-there-are-six`. The
  countermeasure the hazard above wants. **Corrected my own first
  reading before dispatching**: I assumed a new gate in
  `scripts/gates/` would be picked up automatically, and
  `gate-roster.sh` proves the opposite — it requires `ci.yml` to name
  every gate by a `--selftest` call and a real call, so the unit
  reaches two lines into **CIW's** territory. Told to make the reach
  minimal, argue it under a `## Territory` heading, and escalate
  rather than silently take the item's fallback (deleting the word
  "mechanically" from the README), which is the orchestrator's call.
  The brief also names the tension it must answer: part A builds a
  machine that reads a hand-kept declaration, part B deletes a
  hand-kept enumeration for going stale.

### What is still not moving, and why

Items 4, 5 and 6a are all waiting on other programs and **nothing in
this session changes that**: the focus map door straddles DOCM and
S-BOOL, `next-id-has-no-layer3-door` is DOCM's, and 6a is an `[ev]` PR
that gates 6b and 6c. 6a is the one this program can act on alone and
it is the next orchestrator build, not a lane's.

## Third orchestrator; six merges nobody logged; the glob widened (2026-09-04, evening)

Picked the program up from `work/view/plan.md` and this log's tail, and
**the tail was false**. It described three lanes as running. All three
had merged, along with three more:

| PR | what | logged? |
|---|---|---|
| #1846 | `set-param-prechecks-what-the-door-refuses` | no |
| #1848 | the module-kind gate + the loud-skip marker | no |
| #1849 | `camera-fold-clears-status-line` | no |
| #1857 | Ev's `deferred` ruling + the fired-trigger lint check | no |
| #1872 | `opoutcome-superseded-has-no-production-reader` | no |
| #1873 | `two-gestures-can-be-in-flight-together` | no |

Every item file was closed correctly and `lint` was green, so the board
was true and only the narrative was not. **That is the eighth instance
of this program's standing hazard and the first where the stale prose
is the log itself** — the file whose whole job is to be the thing a
successor reads. The item files saved it: reconstructing what happened
took one `git log` over `work/view/` and five minutes, because the
per-item record is where `work/README.md` puts the state and the log is
only the story. Worth stating as the reason that split exists.

No countermeasure filed. A gate cannot tell a session that merged six
PRs to write about them, and the honest instrument is a successor who
reads `git log` before believing the tail — which is now written into
`plan.md` where the next one will meet it.

### CHROME is dormant, and the wait clause is spent

`work/view/program.md` has said since 2026-09-03 that this program
waits on CHROME's slate. Measured rather than assumed:

- CHROME's last self-authored commit is `e59f43dc`, **06:53**. The two
  later touches of `work/chrome/` are DOCM's fix pass and VIEW's own
  #1857.
- Its one open PR, **#1813** (tracker-only, 26 lines), has
  `updated_at == created_at` at 07:09 — untouched for sixteen hours
  while VIEW, DOCM, CIW, CURVED and CERT all committed.
- Its own closing log entry says the slate is complete: *"All nine
  units are answered. CHROME does NOT close with them"* — what holds
  the directory open is residue parked on **this** program's split,
  which is the dependency pointing the other way.

Put to Ev with that evidence; **the glob is widened**
(`crates/viewer/tests/*` into `paths`) and three CHROME rows are
claimed by `git mv`, which is what `work/README.md` requires of a
claim. Not all of them — the ones claimed are the ones where VIEW holds
the ground or the dependency:

- `session-gesture-guard-spelled-thirteen-times`, **claimed and closed
  as dissolved**. Its own text asked for exactly this re-home, and
  VIEW-1b answered both questions it said a fix had to answer: the
  table exists (`session/op.rs:650`), it is exhaustive, and it is
  checked once (`session.rs:675`). Two `gesture.is_some()` reads
  survive in the crate and neither is a guard.
- `viewer-const-all-tables-have-no-exhaustiveness-guard` — three of
  its five citations were pre-split and are corrected in the claim
  note. Held, to be taken with
  `tool-kind-all-and-ordinal-have-no-production-reader`: "delete two of
  them" and "guard five of them" are one question.
- `no-persistent-setplacement-session-op` — DI5's build, which
  `two-hand-written-copies-of-the-g1-gesture-machine` waits on. Both
  halves now sit on one slate.

Left with CHROME: the forms rows, the GPU and pixel rows, the mate and
badge attribution rows, and the coverage rows. Those are that
program's word, not this one's, and a dormant orchestrator is not a
departed one.

### Ev's four rulings, taken in chat

1. **Wave shape** — the small units *and* 6b together, 6b being the
   long pole worth starting early.
2. **Test glob** — take `crates/viewer/tests/*`, and re-home the CHROME
   rows worth re-homing, "which may not be all of them".
3. **The editor-core halves** — reach in narrowly rather than
   announcing and waiting. `EditError`'s user-facing `Display` wording
   only: the `edit: ` prefix and the `{:?}`-quoted payloads. The
   precedent is CHROME's `mate.rs` amendment (#1748). Recorded in
   `program.md`'s `keep_out` so it is a fence with an exception rather
   than a fence that was ignored.
4. **The three design forks** — one `[ev]` PR carrying all three,
   opened while the build lanes run so the answers arrive without
   blocking anything.

### Out on the wire (2026-09-04, evening)

Three implementer lanes, each in **its own git worktree** under
`/home/user/lanes/` with its own `CARGO_TARGET_DIR` and its own scratch
directory. The shared-checkout hazard is why: this program watched it
fail three ways during unit 1, and two concurrent lanes in one working
tree is not a variant of those failures but the guaranteed form of
them.

- **`view/prune-report`** — the two `prune` discards as one unit, since
  they are one change to one signature and would be one merge conflict
  taken twice. Style review. The brief hands over three claims to
  falsify, of which the weakest is the item's "nine assertion sites
  across seven test files" — asserted against an older tree and not
  re-counted by me.
- **`view/clearing-walk`** — the four hand-maintained copies of one
  reset. Style review. Told explicitly that `bounds` and `gesture`
  already sit outside the walk and must be answered rather than
  absorbed, and that my reading of the shape (*the value `land` writes
  is the value the constructor clears*, not one `reset()` called four
  times) is a reading and may be wrong.
- **`view/pick-index-offthread`** — 6b. **Style AND correctness**, the
  only unit in this program's history to carry a second reviewer. The
  posture's test is met exactly: the failure mode is a pick answered
  against an index built for another generation, which is a confident
  wrong answer, not a refusal. The brief carries all three parts of
  Ev's #1843 ruling, both withdrawn positions marked as withdrawn, and
  the instruction that the GQ6 paragraph rides THIS PR because there is
  no off-thread index to describe until the lane lands one.

### The three forks are on #1883

`[ev]` PR from `view/ev-three-forks`, subscribed for wake-on-comment
per `work/README.md` — a question nobody is listening to has not been
asked. Written as a decision document: three questions, each with the
options, my reading, and what I would do with no answer. The one fact
worth carrying out of writing it is that **all three are inputs to the
same nineteen-site sweep**, which is why they are one PR and not three:
`status-line-writers-bypass-the-ranking` sorts nineteen writers into
news and standing facts, and cannot be dispatched until it knows what
each of those is.

`pick-and-parts-name-the-session-driver` is the one that is on Ev's
desk for a reason other than difficulty: the rule it falsifies is text
Ev ratified at #1801 one day earlier, and a gate this program built at
#1848 is what proved it false.

## `view/prune-report` built and green; one dispatcher correction; the class's third member filed (2026-09-05)

**#1886, CI green** on `eaa41580` — run 33931590377, 37 jobs, 0
failures, **12 `test (…)` and 5 `k-lint (gate, …)` jobs**, which is
what a full code-tier run must show since the 2026-09-04 widening. No
`CI-Config:` trailer, nothing narrowed. Not merged: `view/clearing-walk`
(#1885) is an open sibling touching `session.rs`, and the lane
correctly left the sequencing to the orchestrator rather than taking it.

### The design call the lane made, and why I am letting it stand

The item asked, without answering, whether re-showing a **fused**
instance is a supersession at all. The lane answered **no**, and the
argument is better than the one I would have given: a supersession is
a **substitution** — the mate answers the placement question better
than the hand placement did — whereas a dropped hide is superseded by
nothing; the user's question stopped being *askable* rather than being
answered differently. It then found the thing that decides the
wording, which neither the item nor my brief had: the two arms of
`display_check` are not symmetric to a person. On a **fuse** the part
is drawn AGAIN — material the user removed is back on screen — and on
a **delete** nothing reappears. So one sentence cannot carry both, and
the shipped preamble says only what is true of either.

It is disclosed, argued in the PR body and in `PruneReport`'s docs,
and pinned by a row. Sent to the style review as the first claim to
falsify rather than accepted here.

### The dispatcher correction

My brief carried the item's "**nine** assertion sites across **seven**
test files, **all** spelled `vec![bench.post_b]`" — flagged in the
brief as inherited and unverified, which is why the lane checked it.
It is **11 sites across 8 files in six spellings**, and two of them
(`superseded.is_empty()`) are type-agnostic and never moved. The
count came from the closed `opoutcome-superseded-has-no-production-
reader`'s own correction against a **pre-#1872** tree — so this is the
citation-staleness class again, one layer up: not a `file:line` that
stopped resolving but a **census** that stopped being true, carried
forward by an item that had no reason to re-run it. That is the eighth
prose-outran-the-tree instance and the second whose subject is a
count rather than a location.

### The class's third member, filed rather than disclosed

`prune` reconciles three pieces of display state. This unit made two of
them report. **The third — `gesture_dies` — still spells
`free_move_check(...).is_err()` and throws the fault away, nine lines
below the discard this unit was dispatched to fix.** Verified at
#1886's head.

It was defensible before this unit (a bare `Vec<RecipeNodeId>` had
nowhere to put a killed gesture) and is less so after it, because
`PruneReport` now has a field per kind of withdrawal and the third
clause declines to use it. Filed as
`prune-kills-a-gesture-and-reports-nothing`, because a residue
disclosed in a merged PR body warns nobody once this directory is
deleted — which is the rule `work/README.md` states and the shape this
program has now caught four times in two days.

## `view/clearing-walk` built and green; the walk was half-defensive (2026-09-05)

**#1885, CI green** on `00275985` — run 33932029198, 37 jobs, 31
success / 6 skipped, twelve `test (…)` and all five
`k-lint (gate, …)` rows. First run was red on `rustfmt + rustdoc
(gate) + wasm32` (a renamed accessor needed reflowing) and the lane
fixed it rather than reporting green over it. Not merged; sequencing
is mine.

The shape is the one my brief predicted and said might be wrong:
`land` writes the same value the constructor clears, rather than one
`reset()` called four times. Two values — `Derived` (selection, hover,
scratch, landed, bounds) and `LandedRun` (the six `landed_*` fields as
one, so `landed_pair` can no longer hand out half of it). Under style
review.

### Three findings from the lane worth carrying whatever the review says

**`display` cannot join a reset-by-construction value, and the reason
is a counter.** `DisplayState::clear` deliberately preserves and
**bumps** its `revision` — the chrome's "does the drawn scene need
rebuilding" key — while `DisplayState::new()` starts at 0. Rebuilding
the field would send that counter **backwards**, and a scene built
under the old count would then read as current. So the walk is one
assignment plus one `clear()`, with the reason written where the walk
is. That is a real constraint on the *idea* of reset-by-construction
and not an exception to it: a field added inside `DisplayState` is
still cleared by that type's own `clear`.

**The old walk was HALF-defensive, which the item did not know.**
`scratch` is `Some` only while `gesture` is `Some` — set only in
`preview_gesture` under a live gesture, taken at both gesture ends — so
`self.scratch = None` in the two doors was as unreachable as a
`gesture = None` would have been. The doors were clearing the preview
while leaving the drag that owns it. The item filed `bounds` and
`gesture` as "two fields already sit outside the walk"; the truer
statement is that a third field was inside it for no reason.

**The one behavioural addition, disclosed rather than smuggled.**
`clear_for_new_document` asserts `gesture.is_none() && scratch.is_none()`,
over state the policy table makes unreachable, so that relaxing
`permitted_during_value_gesture` reds a test instead of silently
leaving a drag pointed at a document that is gone. Disclosed and
argued, which is the procedure `docs/prompts/implementer-discipline.md`
§3 asks for. **Sent to the review as the first claim to falsify** —
whether "unreachable" holds on every path, and whether an `assert!`
(a panic, in a GUI, compiled differently in release) is the right
instrument or whether the honest one is a refusal. I have not
adjudicated it here.

### Reclaimed

`review-prune-target` (1.7 GB) deleted the moment its report was in
hand, per `memories/agent-lane-operations.md`. The box was at 12 GB
free with the 6b lane's target at 9.4 GB and still growing, which is
the one lane that must not meet a full disk.

## 6b is built and green; the dispatcher was wrong about the worker (2026-09-05)

**#1888, CI green** on `d1fca1a8` — run 33932680543, 37 jobs, all
twelve `test (…)` points, all five `k-lint (gate, …)` unifications,
both render lanes. Four commits. Under **correctness review**, which is
the review this unit was singled out for at dispatch; the style review
follows it rather than running beside it, to keep this session's
concurrency down after the rate limit below.

### The dispatcher correction, and it is the design

My brief claimed the `EvalService` seam "can carry a second payload
kind without being redesigned". **Half wrong, and the wrong half is the
worker.** The submit-cancels-and-restarts half holds. But an index
build is **uninterruptible and takes seconds**, so a shared queue would
put it in front of the next evaluation — and an edit made during an
index build would then wait for that build to finish. That silently
weakens the cancel-and-restart promise GUI-3 ratified *above* it:
**a seam cannot keep a promise from behind a queue it does not
control.** So the unit reuses the vocabulary and duplicates the
worker, and says so in the module docs rather than only in the PR.

That is the ninth dispatcher correction this program has taken and the
third against a decision rather than a detail. It is also the argument
for the posture that produces them: the brief said the claim was a
claim.

### The three shape decisions

- **No `cancel` on the index trait at all** — not a cancel that
  quietly does nothing, but no door. Ev's Q3 answer made structural, so
  a later lane cannot wire a token through without meeting the
  argument first.
- **The key is the pair `(generation, δ)`, carried on the ANSWER.** The
  refusal arm has no index to read a generation off, and δ is an input
  to the tessellation but not to the evaluation — so an index for the
  document on screen at a δ nobody asked for any more is the same class
  of wrong as one for the wrong document, and only the pair separates
  them.
- **The cache drops its index at the SUBMIT, not when the replacement
  lands**, which is what makes Ev's Q1 answer true rather than
  aspirational: current or absent, never behind. That is the
  correctness review's first claim to falsify.

### What the window looks like, which is the ruling in the tree

The viewport keeps drawing the previous document's mesh (an older
picture, which the ruling permits), a **click** is refused typed and
visible as `pick::NotIndexed`, the toolbar says `indexing…` with the
refusal's own sentence as hover text and **no Cancel button** — the
weaker promise showing through the chrome. The fourth chrome condition
went into the existing block as `frame::progress(busy, running,
indexing)`, a total function of three booleans, so two spinners cannot
be lit for one wait.

**A hover is deliberately left unrefused**, because it is pushed on
every frame the pointer is in the pane and refusing it would rewrite
the status line sixty times a second. The lane flagged that as its own
judgement for a reviewer to check, which is the right thing to have
done with it; it is named in the correctness brief as the likeliest
place a confident wrong answer hides.

### The claim I doubted, and was wrong to

I told the lane that `Send` on the index and its BVH was "the likeliest
place my brief is wrong". It was not — they are `Send`, and the lane
did not leave it to a grep: a `const _: fn()` in `evalseam` asserts it
for both payload types on **every** target, including wasm, where the
threaded implementation that would otherwise force it is compiled out.
A doubt that turns out to be misplaced and gets closed by a
compile-time assertion instead of a sentence is the cheapest possible
outcome.

### A filing collision, resolved in the lane's favour

The lane filed `work/view/ui-thread-work-after-the-index-seam.md` on
its own branch (commit `2622d14f`) before my message telling it not to
arrived — reading `docs/prompts/implementer-discipline.md` §6's *inside
your own program's fence a disclosed residue owes a file in the same
PR*, which is correct and is the rule. **Its file stands and I write
none.** The collision is my doing: I told it to report rather than file
in order to keep `work/view/` clear for my orchestrator branch, which
is a convenience of mine set against a rule of the project's. The rule
wins; the lesson is that the orchestrator's own branch is not a reason
to suspend §6, and future briefs should say "file it, I will merge
around you" instead.

That file covers three further UI-thread costs the sweep found and
**does not measure**: `scene::fit_delta`'s probe tessellation (~1/8 of
a full one, once per document that arrives), `scene_focused`'s walk
over every drawn triangle including on hide/focus changes, and
`DocSession::land`'s gather plus check registry plus A5 certification.
It overlaps `scene-gathers-the-landed-product-twice-more` on the second
and asks a different question of it — where the cost runs, not that the
product is gathered twice.

### Territory, clean

Nothing from `crates/mesh/` or `crates/bvh/` was needed or nearly
needed — Q3's answer is precisely what removed the need and the
no-cancel-door shape is what keeps it removed. **No `session.rs` line
was touched**, so both live sibling lanes are untouched; the request is
built from `landed_pair()` and `evaluation_arc()`, which already
existed. The one cost is cloning the `Doc` into each request rather
than sharing the session's `Arc` — once per attempt, and it needs no
new door on the driver.

## The session hit a rate limit and killed three agents at once (2026-09-05)

At 00:38 UTC all three running agents died mid-flight on a session
limit (reset 02:00). **Nothing was lost, and the recovery is worth
recording because the tracker's own rules are what made it cheap:**

- the 6b lane had already pushed four commits and had a green run, so
  its state was entirely on the remote and only its *report* was
  missing;
- the clearing-walk review had done nothing yet and restarted clean;
- **the prune-report fix pass had an uncommitted working tree**, which
  is the only real exposure. Committed by the orchestrator as
  `0350f832` with a message naming exactly what it is and what it is
  not, and pushed, then the lane resumed and told to read that diff
  rather than trust its memory of where it got to.

The rule that made the first two free is `implementers commit AND push
after every coherent unit` (`memories/agent-lane-operations.md`). The
third is the case it exists for. Five concurrent agents is what
exhausted the budget; the correctness review runs alone rather than
beside a style review as a result.

## #1885's style review: the ratified prose asserts a policy the walk violates (2026-09-05)

The strongest review this program has taken. It verified the lane's
claims 2 and 3 independently — `DisplayState::clear` really does bump a
counter `new()` would reset, and there really is no path to
`scratch.is_some() && gesture.is_none()` — and then declined claim 1
for a reason neither the lane nor I had.

### The assert is not a precondition

`open` writes `resolver`, `history` and `path` **before** calling
`clear_for_new_document`. So if the assert ever fires it fires with the
session **already half-replaced** — which is precisely the half-acted
state the same function's doc-comment says refusal exists to prevent.
The precondition belongs in `perform`, where a `Refusal` is free and
one is already returned two lines away.

That is the sharpest form of the dispatcher's own exposure: I sent the
assert to review as "the first claim to falsify", and the reviewer
falsified something better than the question I asked. I asked whether
*unreachable* held; the answer is that it does, and that the
instrument is in the wrong place regardless.

**And the "reds a test" half is half true.** Flipping the table row
alone reds `gesture_table.rs`'s hand-restated `expected()`, not the
assert. Flip both and `NewDocument` panics — but **`Open` does not**:
every mid-gesture `Open` sample in the whole suite uses a nonexistent
path and dies in `docio::open` before the assert is reached. The
guarantee rests on one fixture for one door and on nothing for the
other.

### The README ratified four sentences that are not true

This is the eighth-and-ninth instance of this program's standing
hazard, and the first where the prose was **written and ratified in
the same PR that made it false**:

- *"`gesture` is cleared by nothing and must not be"* — one line above
  `display.clear()`, which sets `free_move = None`. That is the
  **other** drag, documented as independently open, and **nothing
  refuses `Open` or `NewDocument` while a free-move is in flight** —
  `permitted_during_value_gesture` governs value gestures only. So the
  ratified sentence states a policy the walk applies to one gesture
  kind and silently violates for the other, and the lane's own argument
  for the assert indicts the line beneath it.
- *"a value drag is refused while either door is asked for"* —
  backwards. `perform` refuses **the door**; the drag is untouched.
  Written twice, once in ratified prose.
- *"cleared by being declared"* — overstates. The struct literal
  refuses to compile until an author writes the cleared value **at one
  site, by hand**. That is one site instead of three, which is the
  win; it is not automatic.
- *"`landed_pair` cannot hand out half of it"* — it returns two of six
  fields, so it does hand out part. The true claim is that the two can
  no longer come from **different runs**.

Plus three different counts of one thing across three artifacts: the
README says three call sites, the item's title says four, its body says
twelve statements.

### A dispatcher correction, of me and not the lane

`land` never wrote `selection`, `hover`, `scratch` or `bounds` — only
the six `landed_*`. It is **three** sites for `Derived`'s walk and one
for `LandedRun`'s. The item's *filename* ("three times") was right and
its *title* was not, and my brief carried the title forward. Tenth
correction.

### Four residues, and the lane files them itself

Reversing my earlier instruction, which was wrong: `implementer-
discipline.md` §6 puts a residue inside a program's own fence in the
PR that discloses it, and my reason for overriding that was to keep
`work/view/` clear for this branch — a convenience against a rule.

1. the free-move drag dissolved silently by `Open`/`NewDocument`;
2. **a fourth hand-maintained walk survives one screen below the fix** —
   `Debug for DocSession` lists fields by hand and is
   `finish_non_exhaustive()`, so a field added to `Derived` is silently
   absent from it. The class the unit exists to close, reproducing
   itself in the same file;
3. `app.rs`'s `matches!(op, SessionOp::Open(_))` re-frame, which
   `NewDocument` never gets though it replaces the document too — the
   lane's **own declared blind spot**, found in a second module;
4. `DisplayState::clear` dropping free-move placements silently while
   `prune` reports them — and #1886 is at this moment making `prune`
   report *more*, which widens the gap rather than closing it.

## #1886 MERGED; 6b's correctness review earned its own existence (2026-09-05)

### #1886 is on main

Merged at `85742e08` after the fix pass came back green on `a87d9984`
— run 33939685666, 37 jobs, twelve `test (…)` and five
`k-lint (gate, …)`, `mergeable_state: clean`, nothing in flight, and
the six skips confirmed habitual against the branch's earlier run
(TIER=closure scoping; none in `viewer`'s dependent closure). Both
prune items closed on main.

The fix pass took every finding on the take-list and **fixed S7 as a
class rather than one member**: the instance/node vocabulary is now a
rule stated on the `DisplayFault` enum — an arm whose subject *is* a
part instance says "instance N", an arm whose whole content is that the
id does *not* denote one says "node N", three arms name no id — so the
pre-existing `NotAnInstance` tension is covered by the same rule
instead of surviving as a second case. It also went past a rename on
S11: rather than renaming a misleading row, it made the row drive
**both** arms, so the case the stale comment claimed exists is covered
rather than papered over.

One correction of mine to record: I told the lane its WIP commit had
landed roughly half the take-list. It had landed eight of eleven. My
read of a diff I had committed on its behalf undercounted it, which is
the same class of error as the census claims this program keeps
finding — a count asserted from a quick read and not re-run.

### The correctness reviewer found a MAJOR, and it is exactly the shape the posture predicts

**#1888 is HELD.** This program's review posture adds a second reviewer
only where the failure mode is *a confident wrong answer rather than a
refusal*. The finding is that failure mode, reproduced by an executed
test:

> `PickCache::sync`'s **`Nothing`** arm returns before touching
> `attempted`. So an index build in flight across an `Open` or
> `NewDocument` still matches `attempted` when it lands, installs into
> the cache, and — because `sync_scene` returns on `Nothing` before the
> scene rebuild — leaves `scene` holding the mesh of one document and
> `index` holding the index of another, with `indexing()` false, no
> status line, and every pick path taking the `Some(index)` arm.

It cannot self-heal: `sync` returns `Nothing` every frame until the
newly-opened document lands, which is seconds precisely because it is
the big document the user chose to open.

**Three things make this the review paying for itself.**

1. It is a **regression 6b creates**. On main the build is synchronous,
   so index and scene install in one `sync_scene` call and there is no
   in-flight build to survive an `Open`.
2. **The whole 483-row viewer suite is green with the hole present**,
   and the reviewer's candidate fix (clear the four fields in that arm)
   leaves 482/482 passing — so no lane test encoded the bug and the fix
   costs nothing. A green suite was evidence about the suite.
3. **The lane's own reasoning was right everywhere it looked.** δ
   changing mid-build, δ going A→B→A, two generations during one build,
   the refusal arm, `Held`, a landing after its request is gone — the
   reviewer attacked all of them and broke none. The hole is the one
   arm that returns *early*, which is exactly what a reasoning-from-the-
   happy-path sweep does not visit.

**And the lane's own flagged judgement was vindicated.** It asked a
reviewer to check whether leaving `hover` unrefused was safe. It is,
and for four independent reasons the reviewer traced: no `Hover` op is
queued without an index, nothing is drawn from a stale hover,
`Leave` is re-synthesised every frame so a clear cannot be lost, and
`IdQueryLog::step` keys on the generation so a landing forces a re-ask
under a motionless cursor. A lane naming its own uncertainty and being
told it was right is the posture working in the cheap direction.

Two MINORs go with the fix: `ThreadIndexer::poll` discards an answer
the cache is *currently waiting for* when the key round-trips
(δ A→B→A costs a gratuitous second full build, up to 13.4 s), and
`(busy, running, indexing) = (true, false, true)` is reachable via
Cancel-during-index and makes the toolbar and the status line describe
one moment two ways.

### What the review could not reach, recorded so it is not read as covered

No `wasm32-unknown-unknown` build was run — the `Send` assertion's
evidence is structural plus a native compile failure at the assertion
line, which is strong but is not the wasm target. And the 2.3 s / 13.4 s
timings were **not re-measured**; they remain the item's numbers.

## #1885 MERGED; the assert was deleted, not relocated (2026-09-05)

Merged at `a7799628`, green on `51f9b2d6` after merging main (which
carried #1886) — 37 jobs, twelve `test (…)`, five `k-lint (gate, …)`,
no conflict markers anywhere in the tree, and CI confirmed to have
fired on the merged head rather than left in a CONFLICTING no-run
state.

### The lane's call on the assert is better than my instruction

I told it to move the check into `perform`, where a `Refusal` is free.
It **deleted the check instead**, on the ground that `perform` already
holds the precondition: it refuses `Open`/`NewDocument` mid-value-
gesture before any door writes anything, so the invariant is enforced,
correctly placed and typed, and a restatement in the arm would have
**broken a stated invariant in order to enforce one** — `perform`'s own
docs say no arm carries a guard against the value gesture, "the table
and only the table".

Accepted, and its offer of a gesture-table row declined for the reason
the style review already supplied: M2 established that flipping the
table row reds `gesture_table.rs`'s hand-restated `expected()`, so a
relaxation **is** machine-caught, one layer up, where the rule lives. A
second check in the walk would be a copy of a rule, which is the class
this program exists to close. Nothing further owed.

It also declined the one thing I offered as an alternative and gave the
right reason: a mid-gesture `Open` row over a real document would
assert the *refusal*, not the reset, because the value gesture makes
`Open` refuse before the door — so the "fails loudly" claim was
**withdrawn** in both `session.rs` and the README rather than
propped up by a row that tests something else. Withdrawing a claim you
cannot support is the outcome this program wants and rarely gets.

Four residues filed on its own branch under the corrected §6 rule, one
of them rewritten against the merged tree rather than its branch point:
`DisplayState::clear` still returns `()` while `prune` now returns a
report with **two** kinds of withdrawal, so #1886 widened that gap
rather than narrowing it, and the file says so.

One correction of the reviewer, from the lane, worth keeping:
`Debug for DocSession` is the **only** hand-written `Debug` in the
crate, so the class to sweep is *hand-listed field census*, not the
trait. A sweep aimed at `impl Debug` would have found one instance and
called it done.

### Next out

`scene-gathers-the-landed-product-twice-more`, dispatched to the same
lane on `view/scene-gathers`. It was sequenced behind the clearing walk
on purpose and the reason is now sharper than when the plan said it:
storing the scene's two derived facts "beside `landed_checks`" means
**joining `LandedRun`**, the value that lane just built — so this unit
is the first real test of the property it shipped, that a new
derived-from-the-landing field joins by being declared. If that fights,
it is a finding about `LandedRun` and the brief says to report it
rather than work around it.

Told, as a claim to check hardest, that `scene_of_evaluation`'s
"no production caller" rests on a grep that cannot see a caller reached
through a re-export or a trait method; and that DOCM-5's 248 ms/8 ms is
**inherited, not this unit's measurement**, and must be cited as such.

## 6b round 2: the fix removed the shape rather than patching the instance (2026-09-05)

**#1888 head `e00f3775`, green** — run 33940663246, 37 jobs, twelve
`test (…)`, five `k-lint (gate, …)`, both render lanes. `origin/main`
merged in (carrying #1886 and #1885) as a merge commit; one conflict,
in `frame_policy.rs`'s import block only, resolved by keeping both
sides.

### The lane improved on the reviewer's fix, and said why

The reviewer's candidate was to clear four fields at the offending
`return`. The lane instead **collapsed the two nothing-landed arms into
one** destructuring of the three landed reads — `landed_generation`,
`landed_pair`, `evaluation_arc`, which `land` sets together — behind a
new `PickCache::forget`.

Its argument: **taking those three reads apart in two places was the
latent half of the defect**, so patching one arm would have left the
shape that produced it. That is the difference between a fix and a
patch, and it is the second time in this wave a lane has answered a
finding at the class rather than the instance (#1886's vocabulary rule
was the first).

It also named four consequences the reviewer had not, which is what
"own the reasoning rather than paste it" was asking for: clearing
`attempted` is the load-bearing part because it is what turns the late
answer into `Stale`; the collapsed arm cannot reintroduce a retry stall
because it submits nothing and `forget` is idempotent; the refusal it
drops is about a document that no longer exists; and picks in that
window become `Absent` rather than `Building`, which is only *true*
because of the two-arm refusal it added in round 1.

**Sent back for a delta round**, not merged. A fix that departs from the
one the reviewer verified is not covered by that verification, and the
questions are specific: does `forget` cover exactly the four fields,
does folding the already-correct arm into the fixed one change it, and
would the new row go red under a *different* wrong fix rather than only
under the one its author tried.

### MINOR 2's answer is a rule, not a variant

`Progress::Canceled` grew an `indexing: bool` rather than a fourth
variant, on the rule **the spinner follows the work, never the name**:
the cancel's label and its Re-evaluate button stay put — the recourse
must not vanish for the seconds a build runs — and a spinner plus a
weak `indexing…` appear beside them, so the toolbar and the status
line's `Building` refusal describe one moment one way. A payload
precisely because the recourse is unchanged. The row now covers all
eight combinations and **asserts-and-labels the two unreachable ones
rather than omitting them**, which is the right treatment of a case a
reader would otherwise wonder about.

### MINOR 1's row, and the difficulty it names

Two builds of one key are indistinguishable by result, so a naive row
for "the answer was kept, not rebuilt" passes either way. The lane made
the *waiting* request carry a **broken** document under the key the
worker is already building the good one for, turning kept-vs-rebuilt
into `Ok` vs `Err`. Recorded because the technique generalises: where
two paths agree on the observable, make the discarded one carry
something the kept one cannot.

### The style review 6b still owed

Dispatched now, deliberately after the correctness lane rather than
beside it. Its brief points at the thing the correctness lane is blind
to by construction: **this unit built a second seam modelled on an
existing one, in the same file** — `IndexService`/`InlineIndexer`/
`ThreadIndexer` beside `EvalService`/`InlineEvaluator`/`ThreadEvaluator`
— which is a near-duplicate by design and exactly the shape that
drifts. `evalseam.rs` roughly doubled, and the open
`frame-module-has-eight-concerns` item is the warning about what
happens next.

### The filing collision did not happen

The lane asked whether my orchestrator branch had written the same
residue file. **It had not** — I recorded at round 1 that its file
stands and I write none, and I kept to that. The duplicate never
existed.

## 6b's delta round: the MAJOR stays fixed, and the reviewer corrected itself (2026-09-05)

**Merge recommended.** The reviewer verified the collapse rather than
accepting it, and its verification is stronger than the lane's own
argument: `landed_generation`, `landed` and `landed_doc` have exactly
three writers, all three move them together with no early return
between, so the destructuring cannot pick up a generation without its
pair — **the old second arm was unreachable rather than merely
redundant.** The retry receipt holds by construction too: `open` and
`new_document` both call `request_eval`, which mints a new generation,
so the same key cannot return after a `forget`. MINOR 1 and MINOR 2
both sound, including that the two progress rows labelled unreachable
genuinely are, since `running()` implies `busy()`.

### The row constrained half of the fix, and the missing half is the ordinary case

The question I sent — *would the new row go red under a different wrong
fix, or only under the one its author tried* — paid for itself. The row
reds under the no-op stub, under `forget` omitting `attempted`, and
under `forget` omitting `outstanding`. It does **not** red under
`forget` omitting `self.index = None`: that mutation leaves the whole
481-row suite green.

The reason is structural. On the path the row drives, `index` and
`error` are **already** `None` when `forget` runs, because the
preceding `sync` submitted and cleared them — so the row only ever
exercises the two fields that are non-`None` there. **The arm it never
reaches is the ordinary one**: a *current* index at the moment the
document is replaced, i.e. an `Open` with no build in flight.

This is the shape `docs/prompts/reviewer-style-lane.md` Q3 exists for —
a row that passes and cannot fail in the direction that matters — found
by asking a reviewer to mutate against a fix rather than to read it.

### The live harm channel, which is neither the lane's nor the reviewer's first story

`frame::disagreement` (`pane/viewport.rs:366`) reads `self.index` with
**no `session.evaluation()` co-guard**, unlike the pick path at `:161`.
The GPU id pass renders the previous document's mesh and the id is
resolved through the replaced document's id map, so a mismatch writes
*"the two picking paths disagree"* — which issue #1097 §4 instructs an
operator to read as an `R32Uint` clear fault. **A false sentence
pointing at the wrong subsystem is worse than silence.**
`blend::mark_segments` (`:215`) is ungated the same way. Filed as a
class by the lane, with the sweep as the fix's obligation; pre-existing,
but 6b is what makes the window it needs common.

### The reviewer corrected its own severity argument

Its first report said the defect produced a wrong **pick answer**. It
did not: `viewport.rs:161` gates the pick path on `(Some(index),
Some(eval))`, and after an `Open` the evaluation is `None`, so clicks
were refused rather than misanswered — and no frame has both `Some`,
because `sync_scene` runs at the top of `ui` and clears the stale index
before the viewport draws.

**The defect and the fix are both real; the mechanism the severity
rested on was already blocked by a second guard.** A reviewer applying
"the dispatch is a hypothesis" to its own previous report, unprompted,
is the discipline reaching the place it is hardest to apply. Recorded
so the wrong framing does not survive into the PR body — the lane is
told to write the invariant breach and the `disagreement` channel,
not the wrong-answer story.

That makes eleven corrections in this program's history, and the first
a reviewer made against itself.

## `view/scene-gathers` green; the lane re-took the measurement (2026-09-05)

**#1908, CI green** on `6b317c72` — 37 jobs, twelve `test (…)`, five
`k-lint (gate, …)`. Under style review. Not merged: it overlaps #1888
on `app.rs` (eight lines inside `sync_scene`'s fit block) and
`scene.rs`, and sequencing beats resolving twice.

### It removed the door rather than making it cheaper

The finding was that `scene_of_evaluation` gathers a product it is
handed and **has no production caller** — a test-only door that would
have paid per frame if anything wired it. The obvious fixes are to
delete it or to make it cheaper. The lane did neither: `scene_of` now
**composes** `scene_of_body`, so the shared core gained a production
caller (startup, `app.rs:501`) and the redundant door is gone.

Worth carrying, because this board holds two open items of the same
shape — `tool-kind-all-and-ordinal-have-no-production-reader` and its
`Seat::ALL` sibling — and both are probably better closed this way than
by deletion.

### It re-took the measurement instead of repeating the one it was handed

The brief said DOCM-5's 248 ms / 8 ms was **inherited, not this unit's**,
and must be cited as such. The lane cited it that way and then measured
its own path: **87 ms gather against 2.4 ms body-clone at 165 roots /
990 faces**, and 27.2 against 0.69 ms at 40 / 240 — 36–39× at both
scales, so nothing in the PR rests on DOCM's number.

That is rare and it is the behaviour the measure-first rule wants. Most
lanes repeat the figure they are given; `memories/refusal-text-is-not-
cause.md` exists because of the ones that do.

### The hole it found, measured before choosing

An assembly whose **A5 gate refuses** keeps no body: `assemble_gathered`
takes `Product<T>` **by value**, and that is an editor-core door this
unit may not change. Three ways out, costed rather than argued:

| | cost |
|---|---|
| clone before every gate | 2.4 ms **per landing** of every assembly |
| skip the fit when no body | regression — a gate-refused assembly opens unfitted |
| gather once there, memoize | 87 ms **per landing that asks**, refused gate only |

Cloning is 2.7% of a gather but is paid per *landing* while the gather
it saves is paid per *opened document*, so it loses after a few edits.
`landed_body` therefore takes **`&mut`**, gathers in that one case and
memoizes. Filed as `refused-a5-gate-eats-the-body-the-fit-then-
regathers`, with the real fix named as DOCM's door: a refusal that
carries its product back.

**The `&mut` accessor is the decision I am least confident in and I
have said so to both the lane and the reviewer.** The arithmetic is
sound; what is untested is the *shape* — whether a getter that takes
`&mut` and may cost 87 ms is a good way to say "reading this can cost
you", or an invariant living in a signature nobody reads. If the review
says take the clone, I want the lane's view before deciding.

### `LandedRun`'s property held

This unit was dispatched partly as the first real test of the value
`view/clearing-walk` shipped — that a new derived-from-the-landing
field joins by being declared. It did: the field was declared, filled
in the arm that already had the value, `Derived::none()` refused to
compile until the cleared value was written, and **no walk was
edited**. The only friction was editor-core's by-value door, which is
not `LandedRun`'s.

## #1908's style review: an evil merge, a false disclosure, and the `&mut` overruled (2026-09-05)

The sharpest review of this session. Two of its three dispatcher
corrections are of the orchestrator and one is of a lane claim made to
the orchestrator and repeated by it.

### An evil merge, verified

`18a5368da` is titled `Merge remote-tracking branch 'origin/main'` and
its conflict note names one tracker file — and it contains **18 lines
present in neither parent**: new doc prose at `scene.rs:914-916` and
`session.rs:350-362`. Confirmed here with `git show --cc` before
passing it on.

**Authored content folded into a merge is content nobody reading the
unit's commits will ever see.** `git log -p` on the branch does not
show it without `--cc`, and this repo is merge-only precisely so that a
merge is a *resolution* and not an authorship channel. One of the
review's own findings (S7, an over-claim about what the rows guard)
lives inside those lines — so a defect entered the tree through a
channel with no review at all, which is the whole hazard in one
instance.

Nothing in the tree forbids this and nothing detects it. Filed as a
class by the lane.

### A disclosure that was disclosed to nobody

The lane reported the steady-state memory point — the session now
retains the aggregate body for the life of a landing — as *"in the PR
body rather than hidden"*. **It is not in the PR body.** Not in the
log, not in either item file. It was disclosed in a report to the
orchestrator, which is precisely the channel `work/README.md` says is
not a record: *"a report that exists only in a session's context is one
outage from never having happened."*

**And this orchestrator repeated the shape of it without checking.**
That is the eleventh prose-outran-the-tree instance and the second
whose author is this seat. The lane is told to put it where a reader
meets it; the lesson for the seat is that "disclosed in the PR body" is
a claim about a file, and files can be read.

### The `&mut` overruled, on an argument neither of us had

I flagged `landed_body`'s `&mut self` as the decision I was least sure
of and sent it to review as such. The review killed it twice over:

- **It forecloses the move a sibling has already filed.** #1888 builds
  its worker requests from `&DocSession` reads and states "no
  `session.rs` edit at all"; its own residue file names
  `scene::fit_delta`'s probe as the **next** thing to move off the UI
  thread. A `&mut self` accessor cannot be called from a worker or
  from an `&DocSession`-shaped request builder. So this unit narrows
  the exact move its sibling filed one round ago, **and nothing in
  either PR would catch that** — the premise lives in the other tree.
- **The memo has no fixture.** Delete the memo write and all three new
  rows stay green: they cover the paths where `land` already owns the
  body. The refused-**gate** path, the sole reason the door is not a
  `&self` getter, is untested. The `&mut` is carried for a path no row
  reaches and costs a capability a sibling needs.

**A fourth shape nobody costed, offered to the lane to check rather
than to take:** do the fallback gather **eagerly in `land`**, in the
refused-gate arm only. The fit is the consumer and always asks, so the
memo pays that same 87 ms on exactly the same landings — same cost,
and `landed_body` becomes an ordinary `&self` getter like every
neighbour. No memo, no `&mut`, no untested path, and #1888's move stays
open. If the fit does not always ask there, the fallback is the clone.

The lane measured three options carefully and picked correctly among
them. The review's contribution was that the option set was wrong —
which is the argument for reviewing a decision and not only its
arithmetic.

### The rest

Three stale counts, one of them in the ratified README (*"the **six**
things a landing produces"*, now seven); `landed_body`'s doc naming two
`None` causes where the code has three; "exactly once" false on the
path that swallows its own error; the 2.4 ms figure misattributed at
one of its four claim sites; and **an existing scheduled register that
already re-takes half the measurement** — `m4_pr8_latency`'s
`gather_ms`, run and committed by `nightly.yml` — which the "no guard"
paragraph does not mention, so the unguardable claim is true only of
the denominator.

## #1908 MERGED, and the lane corrected my premise (2026-09-05)

Merged at `b20e13da`, green on `a8d09399`. **The wave's fourth and
last unit.**

### I was wrong about "the fit always asks", and the lane checked it

I overruled the `&mut` accessor — correctly, on the review's S18 and
S8 — and then proposed a shape of my own: gather **eagerly in `land`**
on the refused-gate arm, arguing that "the fit is the consumer and it
always asks, so the memo pays that same 87 ms on exactly the same
landings".

**That premise is false and the lane checked it rather than taking
it.** `fit_delta_on_scene` is latched at construction (`app.rs:593`)
and on `opened` (`app.rs:861`) and nowhere else — **once per opened
document, never per landing**. So eager would have paid 87 ms on
*every* landing of a refused-gate assembly (an ordinary authoring state
while a mate does not certify) to save nothing on the landings nobody
asks about. That is the same per-landing-against-per-open trade that
rejects the clone, an order of magnitude worse: 87 ms against 2.4 ms.

Verified here against the tree before accepting: two writers, both
latches, neither per-landing.

**What it built instead gets all three properties.** `landed_body` is
`&self -> Option<&Body<f64>>`, pure, never gathers; the fallback moves
to the **consumer**, spelled out at the fit's own call site, so the one
path that costs a gather is the one path that names one. No memo, no
`&mut`, no untested path, #1888's move stays open, and the cost on that
path is what it was before this unit.

That is the twelfth correction this program has taken and the second
against a *premise* of the orchestrator's rather than a detail — and
this one I asserted in the same breath as telling the lane to **check
it rather than take it**. The instruction is what saved it. Worth
keeping as the argument for writing dispatches that way: a claim
labelled as a claim gets checked, and this one was wrong.

### The evil merge, fixed and filed

The 18 lines are re-landed as ordinary content in `4e9f7dcf` — and
**rewritten**, because one of them made a false claim about what
`landing_gathers.rs` guards. That is the cost of prose reaching no
reviewer, demonstrated inside the instance that named it. Filed as a
class: `authored-content-folded-into-a-merge-commit`, with the shape of
a check (`git show --cc` scoped to the conflicted paths) and the one
decision it needs first.

### One thing the sweep for stale prose found and one it did not

The lane swept DOCM-5's "three consumers" phrasing: one hit inside its
own fence, and **none in `editor-core`** — neither `checks.rs` nor
`product.rs` carries it, so nothing is owed to DOCM. A sweep that
returns nothing to route is still a result.

Routed by me, since one-file-one-item forbids the lane doing it:
`ui-thread-work-after-the-index-seam`'s hit (2) cited *"`fit_delta`'s
probe tessellation **and gather**"* and is now half-false — the gather
is gone, the probe tessellation remains. Amended in place, with the
note that #1908 leaves a gather of its own on the refused-gate path at
the fit's call site, which belongs on that list rather than only in its
own item.

## The wave is closed; one lane out on the edit-door wording (2026-09-05)

**All four units merged**: #1886, #1885, #1888, #1908, plus #1912 for
the orchestrator's own state-sync. Board: 37 open, 16 closed, one
dispatched, one parked, one deferred.

**The open count rose by fourteen while five items closed**, and that
is the wave's real output rather than an embarrassment: **eighteen new
items, every one a file rather than a sentence in a merged PR body.**
That is the rule `work/README.md` states, the rule this program was
failing when this session picked it up (a residue disclosed in prose
dies with the directory), and the rule two lanes now apply without
being told. The board is longer because it is finally honest.

### Housekeeping, and one cost of mine

Reclaimed every finished lane's `target/` (the 6b lane's was 11 GB
alone) and swept seven merged worktrees; 24 GB free, two worktrees
live. One cost to record: I deleted `clearing-walk-target` while that
lane was mid-resume, so it paid a cold rebuild it did not owe.
`memories/agent-lane-operations.md` says to reclaim *when a review
returns*; I reclaimed when a **unit** returned, and the lane was not
done with it.

### Out now

`view/edit-door-wording` — `refusal-edit-arm-doubles-a-prefix-and-
splits-one-mistake` carrying `self-boolean-precheck-duplicates-the-
doors-duplicate-input` and `save-permitted-row-argues-only-half-of-
save`. It is the unit **Ev's `crates/editor-core` amendment was
authorised for**, and the brief states the fence as the authorisation:
`EditError`'s user-facing `Display` wording only — the `edit: ` literal
and the nine `{:?}`-quoted payloads — no variant, no semantics, no
other file.

The amendment is what unblocks the self-boolean deletion. That item's
own argument was a **sequencing** claim, not a carve-out: deleting the
layer-3 arm today would hand the user `DuplicateInput`'s worse
sentence, so the door's wording had to be fixed first, and VIEW could
not fix it from its own side. With the amendment in hand the sequence
can run in one unit.

The claim I told the lane to check hardest: that the prefix and the
quoting are **presentation only**, with nothing in the suite, the
serialisers or `pncad-py` matching on `EditError`'s rendered text. 54
arms is a wide blast radius for a wording change, and that is where it
stops being safe.

### Still waiting on Ev

[#1883](https://github.com/evgunter/cad/pull/1883), green since 23:50,
no answer. It carries the three design forks — the news vocabulary's
expiry, the boundary rule #1848's gate proved false, and the badge
family — and they gate `status-line-writers-bypass-the-ranking`, the
nineteen-site sweep that is the largest single item left on this board.
Nothing else is blocked on it.

## Ev ruled all three forks; #1883 merged; the sweep is unblocked (2026-09-05)

> 1. b sounds good
> 2. i think a sounds good, since it's easy to switch to b later and hard to do the reverse
> 3. sure

Merged at `ecd5d237` after resolving one conflict in
`four-badges-five-spellings.md` — my `## Put to Ev` section on the
branch against the `## A sixth member` evidence I added on the
orchestrator branch. **Union, both kept, chronological order**;
neither was a competing claim about the same thing.

### The rulings

**1 — the news vocabulary: a message carries its SUBJECT** (candidate
2). A later message about the same subject supersedes it: a camera
verdict expires on the next camera event, a projection refusal on the
next camera move, a disagreement on the next cursor move, a
supersession on the next document transition. So the sweep is a
**vocabulary change, not a routing change** — `Show(String)` grows a
subject and all nineteen writers answer it, which is exactly why the
sweep waited.

It also settles a sentence already in the tree: VIEW-6's
`supersession_notice` says the supersession is "true of nothing" after
its frame and nothing implemented that lifetime. Under this ruling the
written lifetime becomes the implemented one — the sentence stops being
aspirational instead of being deleted.

**2 — the boundary rule: HOIST the read, do not widen the rule.** The
session hands out a value; `pick.rs` and `parts.rs` take that. *No
vocabulary may name a driver* stays unqualified, and
`viewer-module-kinds.sh`'s two site-granular `VOCAB_EXCEPTIONS` retire
with the sites.

**Ev's reason is worth more than the answer and this program should
carry it as a rule**: *"easy to switch to b later and hard to do the
reverse."* The two branches are not symmetric in reversibility.
Hoisting keeps widening available; widening does not keep hoisting
available, because the clause gets relied on and by the time anyone
wants it back there is a set of sites written against it. That is the
general test for a fork between a strict rule and a rule with a clause,
and it answers the item's own worry — a clause **is** "exactly the kind
a later unit widens again", and the asymmetry is why the strict branch
is the safe one rather than merely the tidier one.

Recorded on the item, with the note that the evidence this exemption
offered to `work/code-quality/D103.md` **stands and is not withdrawn**:
its retirement is evidence about the per-seam allowlist shape, not a
reason to stop offering it.

**3 — the badge vocabulary: yes**, and with the news vocabulary as one
unit, which was the recommendation it was put with.

### Dispatched

`view/news-and-badges` — `the-news-vocabulary-has-no-expiry` carrying
`four-badges-five-spellings`. **Fenced hard against the sweep itself**:
build the two vocabularies and convert what already goes through
`frame`; do **not** touch the nineteen writers in `pane/*` and `app.rs`.
The brief says that if the vocabularies cannot express one of those
nineteen cases, **that finding is worth more than the sweep** — it
means the shape is wrong before nineteen sites are written against it.

The claim I told it to check hardest is my own weakest: that a
supersession's subject is *the document transition*. A supersession is
about an instance, and the thing that makes it stale may be the
instance's next event rather than the document's. If those differ, the
lane is to say which is right.

Still to dispatch from this ruling: `pick-and-parts-name-the-session-
driver` (the hoist) and then `status-line-writers-bypass-the-ranking`
(the nineteen), which waits on the vocabularies this lane is building.

## The box nearly ran out of disk, and the cause is a lane-operations gap (2026-09-05)

Caught at a scheduled check-in, not by a monitor: **2.5 GB free, 94%
used, with two lanes building.** One more link step and both would have
met ENOSPC.

**The cause is `debug/incremental`, and it is not the lanes' fault.**
`clearing-walk-target` was 19 GB, of which **12 GB was incremental
state alone** — that target has been reused across four units
(clearing-walk → scene-gathers → edit-door-wording), which is the
correct thing to do for build speed, and the incremental cache
accumulated across all of them because nothing ever prunes it.

Reclaimed the 12 GB and nothing else; `deps/` and `examples/` untouched.
14 GB free, and the lane's `cargo test --workspace` survived it.

**Checked before acting, because deleting under a live build is the
kind of thing that produces a failure nobody can explain later:** the
running process was the edit-door lane (`/proc/<pid>/cwd` and its
`CARGO_TARGET_DIR`), it was in a test-running phase, and `incremental/`
had not been written for 28 minutes. Incremental state is regenerable
by construction and never load-bearing for correctness. The lane was
told plainly that I touched its build directory and to tell me rather
than debug anything odd — a build directory altered under a lane, and
not disclosed, is exactly the shape that turns into an unexplainable
result three hours later.

### The standing fix, and why it is not just tidiness

Both lanes are told to export **`CARGO_INCREMENTAL=0`** beside their
`CARGO_TARGET_DIR`. A lane builds a handful of times and then hands off
to hosted CI, so the cache buys very little here and costs gigabytes
per unit.

What it prevents is the expensive failure rather than the annoying one.
`memories/agent-lane-operations.md` records that a disk-full crash
leaves torn binaries behind and makes **every result taken in the
pressure window suspect** — so an ENOSPC does not cost a rebuild, it
costs the trust in whatever was measured near it. Cheap insurance.

### What this says about the operations memory

`memories/agent-lane-operations.md` is detailed about reclaiming a
lane's `target/` **when a lane or review finishes**, and says nothing
about a target that grows without bound *while the lane is alive and
correct to keep*. This session hit both: earlier I reclaimed a live
lane's target too early (costing it a cold rebuild it did not owe), and
now the opposite failure. The rule the memory is missing is about the
**incremental cache specifically** — that it is the only part of a
target that grows monotonically across units, and the only part that is
free to delete.

That is a `memories/` amendment, which is Ev's call and not a lane's or
mine (CLAUDE.md: memory text is read at the start of every session, so
what goes in it waits for sign-off). Recorded here rather than filed as
an item, because it is an amendment to a memory rather than work in
this program's territory — and named in this log so a successor
orchestrator meets it.

## `view/news-and-badges` green; my weakest claim was right to doubt (2026-09-05)

**#1933, green** on `ce935cf5` — 37 jobs, twelve `test (…)`, five
`k-lint (gate, …)`, nothing narrowed. Under style review. Not merged,
and the lane's reason for not merging is the right one: the two shapes
a reviewer might want different are design calls it made **inside** the
ruling, and the brief said a shape problem found before nineteen sites
are written against it is worth more than the sweep.

### The shapes

**News**: `Show(String)` → `Show(Message { subject, text })`, plus a
fourth arm `Expire(Subject)` — an event about one subject retires the
line's message iff it is about that subject. `frame::fold_status`'s
clean arm becomes `Expire(Subject::Camera)`, which **keeps its ratified
argument exactly** (it decides no sentence it did not write) while
retiring the refusal it did write. `Subject`'s five variants are each
named for **the event stream that retires them**, not for the site that
produces them, which is the right axis.

**Badges**: `frame::Badge { label, Tone, detail, Affordance }`, four
constructors, one draw. Both ratified constraints survive as values
rather than as prose: `Tone::{Advisory, Actionable}` states the
weak/unresolved rule, `Affordance::Opens` keeps the checks badge a
button with the reason on the variant.

And the second shape the #1886 review found is gone:
`supersession_notice` + `dropped_hide_notice` are one typed value with
`Display` (`frame::Withdrawal`), matching `ToolNotice`/`prefs::Notice`.
Zero `fn … -> Option<String>` left in `frame`.

### My weakest claim, and the lane was right to check it

I flagged as least-sure that **a supersession's subject is the document
transition**. The answer is *"true as stated, and empty"*: a document
transition is an accepted op, and such a frame already answers `Clear`,
which sweeps the whole line. So `Subject::Document` has **no `Expire`
issuer that `Clear` does not subsume**, and a supersession's behaviour
is unchanged by this PR.

The interesting fork — subject = the *instance* rather than the
document — needs a payload on `Subject` and is not ruled. Filed as
`a-supersession-outlives-its-own-frame` with three forks, and
`Withdrawal`'s doc now states the lifetime it **has** beside the one
its argument wants, so nothing in the tree claims the unimplemented
one.

### Two things this unit surfaced that outrank it

**A limitation the sweep will hit.** The vocabulary cannot express a
frame whose rank-2 notices are about **different** subjects:
`joined_subject` falls back to `Document`, so a `Cursor` notice joined
with a `Document` notice loses its cursor expiry. Unreachable today —
**reachable the moment the sweep routes the picking disagreement into
`notices` beside a tool notice**, which is one of the nineteen. The
lane's read is that the fix is per-subject line state, which changes
rank 1's ratified *"a refusal wins alone"* and is **not ruled**. If
that holds, the sweep needs another ruling before it can finish, and
finding that now rather than at site fourteen is exactly what the fence
was for.

**A conflict between two texts, resolved by the lane picking one.**
`status-line-writers-bypass-the-ranking` classifies `viewport.rs`'s
`projection: {error}` as a **standing fact**; #1883's ruling text names
a projection refusal among the **news** instances. The lane took the
ruling and made it `Subject::Camera`. **That may be my error rather
than the item's** — I wrote the ruling's examples in the PR body, and I
may have used "projection refusal" loosely for a fact whose lifetime
happens to be "until the camera moves". Put to the style review for an
independent read; it decides one of the nineteen either way.

### Inherited red, diagnosed correctly

The first run was red on `geom-core`'s `bounds_census`, and the lane
**reproduced it at the merge base with none of its own changes** before
concluding anything — inherited, fix already on main, cleared by
merging main. That is the procedure `memories/agent-lane-operations.md`
asks for and it is the first time this session a lane has met a red it
did not cause.

## `view/edit-door-wording` green; a lane met a fence correctly (2026-09-05)

**#1932, green** on `1c2da907` — 37 jobs, twelve `test (…)`, five
`k-lint (gate, …)`. Under style review.

### The fence, and the right way to meet one

The amendment Ev authorised says **"no other file in that crate"**.
`crates/editor-core/tests/lib_doors_node_result.rs:160` asserted the
exact string `"edit: node 7 is not live"` — **the pin for the very
sentence the amendment authorises changing**. Three options: edit it,
stall, or ship red.

The lane **edited it, disclosed it at the top of its report, and asked
for ratification rather than assuming it.** That is the behaviour a
fence is for: it did not quietly widen the scope, and it did not stop
work over a line that a wording change necessarily entails.

**Ratified here**, with the reasoning short enough for DOCM to check:
a wording change that leaves its own pin red is not a wording change,
it is a broken build — so the edit is *entailed by* the authorisation
rather than an extension of it. The blast radius was **measured, not
assumed**: `cargo test --workspace` finds exactly one such prose
assertion, and `crates/pncad-py/tests/` asserts only on `.variant`
tags. Announced on the PR with an explicit offer to hand it back.

### Both of my claims to falsify were false

1. *"Nothing in the suite matches on `EditError`'s rendered text."*
   **False** — the one assertion above. This is why claim 1 was the one
   the brief said to check hardest, and it was still wrong.
2. *"Deleting `Refusal::SelfBoolean` leaves no reader."* **False** —
   four: `rank()`'s or-pattern and three test sites. All now assert the
   door's refusal instead, and `story_authoring`'s asserts the rendered
   sentence names the double-picked node, which is what that story was
   about.

Thirteenth and fourteenth corrections.

### The item was wrong about the door's sentence

`self-boolean-precheck-duplicates-the-doors-duplicate-input` argued
the deletion was **blocked** on `DuplicateInput`'s wording because it
"names no recourse". The lane found that reading was **of the
doubled-prefix rendering, not of the sentence**: the forwarded
`InputFault::Duplicate` clause already states the rule. So the fix
needed less than the item claimed, and the sequencing argument that
made this unit wait was resting on an artifact of the defect it was
waiting to have fixed.

Worth keeping as a shape: **an item that reasons from a rendered
string can be wrong about the value behind it**, and the tell is that
the reading and the defect share a cause.

### The convergence decision

One mistyped parameter refused two ways by route. The lane **converged
on the recourse, not the sentence**: `NoSuchParam` now names the same
remedy the door names over the same fact, while the frames stay apart —
the door's sentence is about an edit that was refused, and a drag has
no edit behind it, so a gesture borrowing that frame would report a
refusal of something nobody attempted. Converging further would mean
restoring the pre-check #1846 deleted. Pinned by a row that asserts
both routes say it.

### The row that proved the point about rows

`panel_edits.rs`'s *"every refusal renders through `Display`, not a
debug dump"* asserted `!contains('"')` and **stayed green while
`Refusal::Edit` dumped a quoted name** — because it walked `Io` alone.
Its title claimed the universal and its evidence was one point. It now
walks five arms through five real ops. The item flagged that row as
CHROME's glob; it is VIEW's since Ev widened `paths`, so the lane took
it.

Residue filed: `refusal-has-no-all-to-walk` — every property over the
`Refusal` vocabulary is a hand-maintained list, to be decided with
`viewer-const-all-tables-have-no-exhaustiveness-guard`, which this
program claimed from CHROME for exactly that reason.

### Reported to DOCM, not taken

`edit.rs` renders a slot id through `Debug` in **six** more places
(`{slot:?}` → `PointX`) where `SlotId::label()` gives `point x` and
`refuse.rs` already uses it. Same class as the nine names, **not in the
amendment's enumeration**, so the lane left them and I announced them.

## Correction: I was wrong about the self-boolean item, and a reviewer caught it (2026-09-05)

**The entry above says** the item's *"`DuplicateInput` names no recourse"*
was "a reading of the doubled-prefix rendering, not of the sentence",
and drew a lesson from it about items that reason from rendered
strings. **That is wrong.**

#1932's style review checked it: the reading was **of the sentence**,
and both halves still stand. `a node's inputs are pairwise distinct`
is a **rule, not a recourse**, and nothing in the rendered sentence
tells the user to pick a different second body.

The accurate account is that the item was **partly right and the fix
was smaller than it asked for** — not that it misread the tree. The
lesson I drew was therefore about a shape that was not present, which
is worse than no lesson: a plausible generalisation from a false
instance is the thing this program's review posture exists to catch,
and I produced one while writing up someone else's.

The lane is told to correct its closure section, which inherited my
framing.

## #1932's style review: the sentence got worse (2026-09-05)

Fourteen findings. It **agreed with the fence ratification** and swept
independently to confirm `lib_doors_node_result.rs:160` is the only
exact-equality prose pin in the workspace.

### The central finding, and it is a regression

A boolean mispick now renders **a phantom node id**:

> the edit was refused: node **N** would be left invalid: node 7 is
> taken as an input twice — a node's inputs are pairwise distinct

`N` is the id `InsertNode` **would have minted** — a node the user has
never seen and that will never exist. The sentence it replaced was *"a
boolean needs two different bodies; node 7 is in both operand seats"*.
The row asserts two substrings, so the phantom id, the cadence and the
missing recourse are all **unpinned**.

That is the unit's central user-visible outcome, and it is what the
item was actually worried about — which is the other half of the
correction above. Sent back to be fixed and pinned; a refusal naming
an id that does not exist is worse than the doubled prefix the unit set
out to remove.

### Two more of my premises wrong

- *"`pncad-py` asserts only on `.variant` tags."* **False** — 19
  `assertIn` message assertions across the Python suite plus an exact
  `assert_eq!` on a rendered message in `src/tests.rs`. None break
  here, every one a surviving substring — but **the sweep's stated
  shape could not have told us that**, and the same sweep against a
  future wording change would report clean for the wrong reason.
- *"The deletion left no hole."* **False** — `SelfBoolean`'s
  doc-comment was grafted onto `Refusal::Edit`, so a public API type
  now carries a rustdoc paragraph about booleans with two sentences
  fused by the deletion. A hole in exactly the arm that replaced it.

### The row that was rewritten is still sampling

`panel_edits.rs`'s five-arm walk is **5 of `Refusal`'s 18 arms**, and
one of its two asserted properties is **false over the vocabulary its
title claims**: `!contains('"')` does not hold for `Edit(MetaNotSet)`
and three siblings whose `key: String` renders `{key:?}`, one of which
embeds a literal `\"v\"`. The `Edit` arm has ~50 sub-variants and the
row samples one.

`crates/pncad-py/src/prose_census.rs:16-33` names this failure mode in
as many words — *"a roster that picks its own samples excludes the
failing mode by construction… what decides the rendering is the
variant of the PAYLOAD, one level down."* The unit widened a row from
one sample to five and met the same wall one order up.

### Ratified: the fence is "wording", not an enumeration

The em-dash→colon change is a **third** wording edit outside the two
the amendment lists. Ratified: the fence's intent is *`EditError`'s
user-facing `Display` wording*, and the two items were a description of
what needed changing rather than an exhaustive licence. Restated that
way on the announce so DOCM reads it rather than infers it.

### A cross-crate coupling nobody would have looked for

`prose_census.rs:1626` pins `crates/viewer/src/frame.rs`/`Disagreement`
with an **exact count**, in a gating roster that reds in both
directions. So a lane adding *or removing* a `{:?}`-in-prose site in
`frame.rs` reds a row in **LIB's** crate, which it is not touching and
has no reason to read. Warned `view/news-and-badges` directly, since
its fix pass makes `Badge`'s and `Message`'s fields private — a
refactor that can move an interpolation without anyone thinking of it
as a wording change.

### One item is pointing at nothing

`work/fix/verb-and-dimension-render-through-debug.md:48` cites
`crates/viewer/src/session.rs:750` for a `{dimension:?}` site the 1c
split moved; it is now `session/refuse.rs:337`. **FIX's item, VIEW's
split that broke it** — announced, not edited. This is
`stale-file-citations-after-the-split`'s general case, still open,
producing its next instance on schedule.

## #1933 MERGED; the sweep's last blocker is on Ev's desk (2026-09-05)

Merged at `dd8e91da`, green on `495af188`.

### The fix pass did three things better than asked

**It made the subjects type-pinned rather than tested.** I asked for
rows pinning the subject at each writer. It built **ten doors in
`frame`**, each taking one *typed* refusal and answering the subject
from it, so nine of eleven **cannot be got wrong at all** — calling the
wrong door does not compile. `Message::new` no longer appears outside
`frame`. A row that can go red is the standard; a shape that cannot go
wrong is better, and it is the same move VIEW-1b made with the gesture
table.

The two it could not type-pin are **stated per site in their own
docs**, not left silent: `startup_notices` takes notices already
rendered from three sources with three types, and `tool_news` comes
through `ToolKind::says` as text — filed as `document-news-has-no-home`
with the real blocker named (`says` returning a bare `String`).

**It swept the class, not the instance.** I named `draw_badge`
swallowing `chrome`'s doc; it found `Disagreement::notice` had
swallowed `disagreement`'s too, and checked the other two insertions
clean.

**It gave a negative result a mechanism.** On the `prose_census.rs`
coupling I warned it about, it did not report "clean" — it showed why:
the surviving positional site is `Disagreement`'s own `Display`,
untouched, and the `{typed:?}` this pass moved *into* `frame.rs` is
inlined in a plain function, so it is neither positional nor a raise
site and lands in no roster. A negative result without a mechanism is
worth nothing.

### Why I merged over the lane's reservation

The lane held it back twice, correctly, because
`news-and-standing-facts-are-orthogonal-axes` is a question about the
classification this diff commits to. What changed is that the question
is now **filed, argued and on Ev's desk**. Holding a green, ruled,
twice-reviewed unit for a fork its own item schedules would hold it
indefinitely; the vocabularies are what was ruled and built, and the
classification bites at the **sweep**, which is a different unit and is
where twenty sites get written. Merging puts a recorded inconsistency
on main — holding does not fix it, because the fix is a rule nobody has
ratified, and inventing one here would be exactly the self-certification
this posture exists to prevent.

Told the lane the argument rather than the outcome, and that it is
reversible in the direction that matters since the sweep has not
started.

### [#1945](https://github.com/evgunter/cad/pull/1945) — the last blocker

`[ev]`, subscribed for wake-on-comment. **News vs standing fact** asks
whether a fact outlives its frame; **`Subject`** asks which event
retires it. A fact whose lifetime is *"until the camera moves"* answers
yes to the first and `Camera` to the second — they were designed as one
axis and are two.

The cost is already visible inside one diff: `scene_refusal` and
`index_refusal` took `Subject::Display`, `projection_refusal` took
`Camera`, all three are the same class of fact, the sweep item calls
all three standing facts, and #1883's ruling names the projection one
as news. Four defensible answers, no rule.

The PR offers Ev an escape hatch that costs him less than the general
rule: answering only *is `projection: {error}` a badge or a line
message* unblocks the sweep, and this program takes the general rule as
whatever that implies.

## #1933 MERGED; the sweep's last blocker is on Ev's desk (2026-09-05)

Merged at `dd8e91da`, green on `495af188`. Two conflicts on the
orchestrator branch, both the same shape as before — `status:
dispatched` here against `status: closed` written by the lane in its
own PR. Main's side taken both times; the lane's closure is the true
state.

### The fix pass did three things better than asked

**Type-pinned rather than tested.** I asked for rows pinning the
subject at each writer. It built **ten doors in `frame`**, each taking
one *typed* refusal and answering the subject from it, so nine of
eleven **cannot be got wrong at all** — calling the wrong door does not
compile, and `Message::new` no longer appears outside `frame`. A row
that can go red is the standard; a shape that cannot go wrong is
better. Same move VIEW-1b made with the gesture table.

The two it could not type-pin are **stated per site**, not left
silent: `startup_notices` takes notices already rendered from three
sources with three types, and `tool_news` arrives as text through
`ToolKind::says` — filed as `document-news-has-no-home` with the real
blocker named (`says` returning a bare `String`).

**Swept the class, not the instance.** I named `draw_badge` swallowing
`chrome`'s doc; it found `Disagreement::notice` had swallowed
`disagreement`'s too, and checked the other two insertions clean.

**Gave a negative result a mechanism.** On the `prose_census.rs`
coupling I warned it about, it did not report "clean" — it showed why:
the surviving positional site is `Disagreement`'s own `Display`,
untouched, and the `{typed:?}` this pass moved into `frame.rs` is
inlined in a plain function, so it is neither positional nor a raise
site. A negative result without a mechanism is worth nothing.

### Why I merged over the lane's reservation, and what it answered back

The lane held the branch twice, correctly, because
`news-and-standing-facts-are-orthogonal-axes` questions the
classification this diff commits to. I merged because the question is
now filed, argued and on Ev's desk: holding a green, ruled,
twice-reviewed unit for a fork its own item schedules holds it
indefinitely, and the classification bites at the **sweep**, a
different unit, which is where twenty sites get written.

**The lane agreed and then improved the argument**, which is the part
worth keeping. It observed that **the recorded inconsistency on main is
not behaviourally symmetric**: `Subject::Camera` has an `Expire` issuer
(`fold_status`, every clean fold) and `Subject::Display` has none. So
of the three facts of one class, `projection_refusal` is **now retired
on the next camera move — exactly what #1883 ruled for it** — and the
two `Display` ones are **swept only by `Clear`, unchanged from before
the unit**.

One writer improved in the ruled direction, two left exactly as they
were, nothing regressed. The inconsistency is a difference in *how far
along* three writers are, not two rival behaviours fighting — so the
fork can be answered on its merits rather than under time pressure, and
the answer either way is three call sites and one variant's issuer
rather than an unwind. Added to #1945, because it changes what the
decision costs.

### [#1945](https://github.com/evgunter/cad/pull/1945)

`[ev]`, subscribed. **News vs standing fact** asks whether a fact
outlives its frame; **`Subject`** asks which event retires it. A fact
whose lifetime is "until the camera moves" answers yes to the first and
`Camera` to the second — designed as one axis, and they are two.

The PR offers an escape hatch cheaper than the general rule: answering
only *is `projection: {error}` a badge or a line message* unblocks the
sweep, and this program takes the general rule as whatever that
implies.

## #1932 MERGED; the phantom id had a root cause worth the name (2026-09-05)

Merged at `22d72a06`, green on `b566f7e5`. Three item-file conflicts on
the orchestrator branch, all `dispatched` here against `closed` from
the lane's own PR; main's side taken each time. **That is the fourth
wave in a row with the same conflict shape**, and it is structural
rather than careless: a lane closes its items in the PR that carries
the work, and the orchestrator branch that dispatched them still says
so. Cheap to resolve, and the alternative — the orchestrator not
recording dispatch — is worse.

### The regression had a root cause, not just a symptom

I sent this back saying the boolean sentence named a phantom node id
and should not. The lane found **why**, which is better: `check_node_inputs`
is reached from `InsertNode` with `RecipeNodeId(new.next_id)` — an id
that does not exist and never will if the edit is refused — **and**
from `SetMembers` with a live one. The `Display` cannot tell which, so
**no framing that names that id is honest**, not merely the one we had.

What it names instead is `input`, which the forwarded fault carries and
which is live on both callers, and the action became a **second
sentence** rather than a third em-dash clause:

> the edit was refused: the node this edit writes would be invalid:
> node 7 is taken as an input twice — a node's inputs are pairwise
> distinct. Replace one of the two with a different node.

The forwarding rule survives — `InputFault` still owns the fault's
words, the door adds the frame and the action — and `TooFewMembers`
gets the same frame with **no** action, because its own fault names the
count required. The asymmetry is stated rather than accidental.

**And it is pinned whole.** `combine_ops.rs` now asserts the entire
rendered sentence with `assert_eq!`, with the reason in a comment:
substring sampling is exactly what let the phantom id and the missing
action through in the first place.

### The lane declined to widen, and said why

`EditError`'s `MetaNotSet`/`MetaNonFinite`/`MetaUnversioned`/
`RebindMetadataCollision` render a `key: String` through `Debug`, one
of them embedding a literal `\"v\"` — the same defect this unit removed
from the nine name-bearing arms, in arms the amendment did not
enumerate. The lane left them, on the ground that *"widening the unit
on my own reading of a ratified fence is the thing I should not do
twice in one PR"*. Correct, and it is the disclosure from the first
fence crossing paying for itself: a lane that got one ratification does
not treat that as a licence for the next.

**I restated the fence on the announce rather than extending it**: the
authorisation is `EditError`'s user-facing `Display` **wording**, and
the two items Ev named were a description of what needed fixing rather
than an exhaustive licence. Under that reading these four arms and the
six `{slot:?}` sites are in scope for the same kind of change — but
they are DOCM's to sequence, and VIEW is not going to keep reaching for
them one review at a time. Offered either way.

### Two amendments the lane made to its own items

`refusal-has-no-all-to-walk` **no longer proposes a third home**: it
records that `prose_census.rs` already answers the `{`/variant-name
half by source census over the whole tree including `crates/viewer/`,
that `display_contract.rs`'s `assert_f6` is a third spelling, and that
the uncovered half is the **quoting** question. The ask became "extend
the census", not "add a roster" — which is the right correction to a
finding I passed on as "file it".

And `self-boolean-…` now says, in the review's words, that **the item
was right and the fix was smaller than it asked for**. My framing that
it had misread the tree is removed from the item, the log and the PR.

### Routed, not taken

To FIX: `verb-and-dimension-render-through-debug.md:48` cites
`session.rs:750` for a site the 1c split moved to
`session/refuse.rs:337`. **VIEW's split broke the citation**, so the
correction is owed; the item is FIX's to edit. Another instance of
`stale-file-citations-after-the-split`'s general case, which stays
open.

## Ev ruled #1945: state both axes; the sweep is unblocked (2026-09-05)

> "1945's proposal sounds good"

Merged at `cfce962e`. **A badge is a read of held state a reader
consults; a line message is the outcome of something that just
happened; either can carry a subject, because a subject only says what
retires it.** Two questions with independent answers, where the crate
had one.

**#1883 is not overturned by this and the item says so.** Its
subject-carrying mechanism stands exactly as ruled and as built; what
#1945 adds is that carrying a subject was never what decided *which
channel* a fact goes to. The projection refusal is a badge that has a
subject — an answer that was **unavailable when #1883 was written**,
because `Badge` had no subject then. That is the useful shape here: the
first ruling was not wrong, it was answered from a vocabulary that did
not yet contain the right option, and building it is what produced the
option.

### Dispatched, two lanes

**`view/axes-and-badges`** — the rule, plus the three call sites it
moves (`scene_refusal`, `index_refusal`/`unindexed_refusal`,
`projection_refusal`, all reads of seam state and therefore badges),
plus `Badge` gaining a subject and the rule written into `frame.rs`'s
header and the README.

Fenced against the sweep exactly as #1933 was, and for a reason that
has now paid once: **if the rule cannot classify one of the sweep's
twenty, that finding outranks the diff.** The last unit fenced this way
found the mixed-subject limitation, which is why this unit exists.

The claim I told it to check hardest is my own weakest: that all three
sites are reads of held state. **`unindexed_refusal` is doubtful** — a
pick that was refused *is* an outcome of something the user just did,
even though what it reports is seam state. And a prior question the
brief makes it answer before writing the field: **what does a subject
mean on a badge?** A badge is a read of held state, so it stops being
shown when that state changes — if a subject means nothing there, that
is a finding worth more than the field.

**`view/hoist-the-session-read`** — Ev's #1883 answer (a) built: the
session hands out values, `pick.rs` and `parts.rs` take those, the rule
stays unqualified, and `viewer-module-kinds.sh`'s two site-granular
exemptions retire with the sites. Told to keep the D103 evidence alive
at the deletion site rather than letting it vanish with the entries.

### Next after these

`status-line-writers-bypass-the-ranking` — twenty writers, the largest
item on this board, blocked since #1849 filed it and now blocked on
nothing but the lane above landing.

## #1957: the lane built a sharper rule than the ruling, and found the ruling's example refutes itself (2026-09-05)

**Green** on `4f1a08a4` — full code-tier run, twelve `test (…)`, five
`k-lint (gate, …)`, all render lanes. Under style review.

### What a subject means on a badge — the prior question, answered

The brief made the lane answer this **before** writing the field, and
told it that "nothing" was an acceptable answer worth more than the
field. It answered: **the same thing as on a message — the event whose
next occurrence makes this the wrong answer — reached by a different
road, and the difference is the ENFORCEMENT.** A message is stored, so
retiring it is bookkeeping (`apply` matches the subject against an
`Expire`). **A badge is stored nowhere and nothing retires it**: it is
recomputed from the state it reads, so its subject names the event that
changes that state, and the badge goes because the read does. The field
is consulted by no retiring machinery.

It then earned the field two ways rather than asserting it: it is the
half a `String` could not carry on the channel that had no way to say
it, and it made *"one seam must not speak with two voices"* mechanical
— `frame::SeamSubject`, a private trait with one const implemented at
each seam's refusal type, so two doors reading one seam **cannot
disagree by construction**.

### It wrote a sharper test than Ev's words, which is the useful part

Ev's rule is *"a read of held state a reader consults"*. The header
makes it mechanical: **a door whose input includes the frame's own
EVENTS is reporting an outcome and belongs on the line.** That is a
discriminator a twenty-site sweep can actually apply, where the
original words need a judgement per site.

And it states the lifetime as a **consequence, not the test**: a badge
outlives its frame and the line carries one frame's news, but that is
"what the split reads like from outside, and it cannot sort a fact that
is both true after its frame and provoked by one" — which is precisely
the confusion that produced the original conflict.

### My weakest claim was right, and the ruling's own example is the casualty

I flagged `unindexed_refusal` as the doubtful one of the three. It is
an **outcome**: `pick::unindexed` answers `Some` for a `Select` and
`None` for a `Hover`, so **half its input is this frame's pick stream**
— the sentence exists because the user clicked. Its own doc already
said so.

Three checkable consequences, all found by the lane: as a badge it
would be lit whether or not anyone clicked; the seam state it reports
is **already read by two other channels** (`index_badge` and
`Progress::Indexing`, whose hover text is literally this same
sentence); and `frame::Progress`'s header **forbids** the result in as
many words — two indicators lit for one wait with no rule saying which
to believe.

**So #1945's rule and #1945's worked example disagree, and the lane
built the rule.** Filed as `unindexed-refusal-is-an-outcome-not-a-read`
for Ev to confirm. What is at stake is not the door — it is a two-line
change either way — but **which test the sweep sorts twenty writers
on**: "reports seam state" (the example) or "reads held state" (the
rule).

### The finding that outranks the diff, which the fence exists to produce

**The mechanical test is necessary and not sufficient.** `Disagreement`
is mechanically a read of held state — the outstanding id answer, the
index, the cursor, all held, recomputed every frame the cursor holds —
so the held-state half alone would badge it. What sorts it onto the
line is the rule's **third clause**, *a reader **consults*** a badge: a
claim about where the pointer is this instant is something a reader is
**told**, not something they consult later.

That clause was in Ev's ruling and reads as decorative until a case
needs it. The sweep needs it stated as load-bearing before it writes
twenty sites — sent to the style review to confirm before I decide
whether it is a fix-pass line or its own ruling.

### A fourth site, and the sweep item is one short

`app.rs:665`'s `startup_notices` — the struct-literal writer the
sweep's own grep could not match — is a **badge** under the rule, but
moving it means *holding* the notices rather than rendering them once,
so it is the sweep's work. And the sweep item's hit list **never names
`unindexed_refusal`**, so its census was one short at its own merge
base. Noted on that item by the lane.

## #1953: the hoist landed, and it found the gate could not pass a clean tree (2026-09-05)

**Green** on `4dfad829` — 37 jobs, twelve `test (…)`, five
`k-lint (gate, …)`. Under style review.

Two values: **`pick::IndexInputs<'a>`** (generation, doc, evaluation,
tol — exactly what `sync` used to destructure by hand off a
`&DocSession`) and **`parts::PartCensus`** (dir, offered). The property
the call site's destructuring spelled out — *the four are read together
because they are set together* — is now the value's own. Both
`VOCAB_EXCEPTIONS` entries retired; the gate reports **0 recorded
exceptions, 31 vocabularies naming none of 2 driver types**.

**No site was awful to hoist** — every read was four fields or fewer,
all already behind accessors. The lane says if one had been, Ev's
asymmetry is what it would have weighed. Nothing came close, so the
ruling cost nothing it did not have to.

### The find: a gate that could not pass a clean tree

**Emptying the exception list turned the gate red with exit 1 and no
diagnosis.** With zero hits the union pipeline's `grep -v` matched
nothing, exited 1, and `set -euo pipefail` killed the assignment.

The failure mode is the worst available: **on CI it is
indistinguishable from a real finding**. And it survived because the
clean fixture *plants the exempted files*, so no run in the gate's life
had ever seen a zero-hit tree — the gate was only ever exercised in the
state its own exemptions created.

That is the same shape this program keeps meeting one level up: a
guard whose passing case was never run. VIEW built this gate two days
ago at #1848 and it took the first unit to actually empty the list to
find it.

### And the self-test stops testing when the list empties

Filed by the lane: four of the five exception arms aim at
`VOCAB_EXCEPTIONS[0]`, a **live** entry, so they go dormant the moment
the list is empty. Guarded off with the gap stated in the self-test's
own printed summary rather than silently skipped, which is the right
interim — and every driver-name arm is now *stronger*, since it asserts
a vocabulary naming a driver **with no exemption in force**. The fix is
a fixture the self-test plants and exempts itself; filed, not taken, in
a PR whose subject it was not.

### A claim of mine, false again

*"Nothing outside `crates/viewer/src` depends on those signatures."*
**False** — `crates/viewer/tests/` depends on them at **27 sites** (22
in `frame_policy.rs`, 5 in `instance_authoring.rs`). Inside VIEW's
territory and mechanical, but the inventory was not empty, and I
asserted it was. Fifteenth correction.

### D103, restated rather than withdrawn

The entries retired **in the same PR as the seam they described**,
mechanically, because a site fixed without lowering the count reds.
That is the property a file-granular entry does not have — so the
retirement is **more** evidence for the open ruling, not the end of it.
Written at the deletion site beside `interval-square-allowlist.sh`'s
equivalent argument, which is where a reader of the deletion will meet
it.

## #1953's style review: the gate fix re-minted the defect its own library forbids (2026-09-05)

The review **reproduced the gate story** rather than accepting it (pre-fix script with an empty list exits 1, zero output), confirmed all 27 test rewrites faithful, and confirmed the fifth self-test arm independent. Then it found the fix.

### The finding

`scripts/gates/lib.sh:113-141` exists because of exactly this: *"the
trailing `|| true` that an exclusion filter legitimately needs for exit
1 swallowed the rest along with it… A scanning pipeline writes
`gate_grep` everywhere it wrote `grep`."*

The fix wrote `|| true` on a plain `grep -v`, **on the whole pipeline**
— so `awk`, `sort` and `grep` exiting **2** all become "no hits" and
the gate greens with no marker for `gate_ok` to refuse over.

**A gate that could not pass a clean tree was fixed by giving it a way
to pass a broken one.** In the directory that owns the contract against
that, two days after this program built the gate.

**And it is a class in the same function** — three more sites, of which
`:458` is the worst: its pattern is *interpolated from an exception
entry*, so a malformed needle empties `hits` and greens the gate rather
than reddening it.

The fix also has **no negative control**: it is covered only because an
empty list makes the clean fixture plant nothing, which is verbatim the
mechanism the lane's own comment says let the original bug survive.

### A class this session has now seen three times

**Inserting an item above an existing one silently steals its doc
comment.** `PartCensus` was inserted inside `PartChooser`'s doc and
inherited its first paragraph, so rustdoc would publish `PartCensus`'s
summary as text describing `PartChooser`, and `PartChooser` opens on an
orphan sentence.

Third instance: `draw_badge`/`chrome` and
`Disagreement::notice`/`disagreement` were the first two, both in the
sibling lane, both found by review rather than by any gate. Told to
file it as a class. It is also, precisely, what check 2 of this
program's own module-kinds gate catches one level up — a header whose
claim is false while rustdoc publishes it.

### The same invariant-not-held finding, twice in two units

Both new types have **all-`pub` fields and no constructor**, so the
invariant each doc claims — *"a caller cannot pick up a generation
without the pair it describes"* — is not held by the type. `LandedRun`
one file away does it correctly. The sibling lane took the identical
finding on `Badge` and `Message` last round.

Two lanes independently wrote a doc-comment invariant where a private
field would hold it, in one session. That is worth watching as a
pattern rather than treating as two nits.

### Prose

*"The **four** move together because they are SET together"* enumerates
**three** and asserts it of four: `tol` is a construction-time field
`land` never writes. Written at three sites. `PickCache`'s docs still
describe the shape the unit removed. The README names
`PickIndex::sync`, which does not exist, one line from where the same
file spells `PickCache::sync` correctly. And the D103 restatement in
the README reads as if **D103 has been decided** — it is unruled, track
K, as this program's own gate header says.

The reversibility argument is written in full at **three code sites
plus the item**, with #1883 narrated at nine sites across five files —
a justification longer than the code it defends, and the shape that
goes stale in five places at once. Told to keep one home.

## #1957's style review: the rule over-claims, and "held" is author-chosen (2026-09-05)

Twenty-one findings, the most severe review of this session. It
reproduced both mutations, confirmed the `prose_census` roster
untouched and correct, and confirmed the `unindexed_refusal`
escalation is **done right** — accurate citation, real schedule.

**Corrected me twice.** My brief said this unit appended to
`frame-module-has-eight-concerns-and-no-holds-row`; it did not — that
was #1933, and I carried the claim forward without checking. Sixteenth
correction, and the second running where I repeated a lane's report as
fact.

### Three findings that compound into one problem

`frame.rs:29` says of the mechanical restatement **"That is the whole
test."** It is not:

- **The headline example is false about the signature it cites.** The
  header and README say `unindexed_refusal` "takes this frame's pick
  stream". **It takes `&NotIndexed`** — the pick stream is
  `pick::unindexed`'s. So a test stated *at the door* sorts the unit's
  own boundary case wrongly when applied literally at the door; it
  works only by tracing transitively to the caller, which is the move
  that would also badge `Disagreement`.
- **`Disagreement` confirms held-state is necessary and not
  sufficient** — every input held, none an event stream, recomputed
  every frame the cursor holds still. What sorts it is the words the
  restatement **drops**: *a reader consults*.
- **And "held" is a property the author chooses, so the mechanical test
  is circular.** `scene_fault` and `projection_fault` **did not exist
  before this unit**; they were added so the fact would be held state.
  Any outcome can be badged by storing it — and the sweep item says so
  openly for the prefs notices: *"which means the notices have to be
  HELD rather than rendered once."*

**That last one is the deepest thing found this session.** Ev's rule is
sound as a rule; what this unit discovered by building it is that its
mechanical restatement cannot decide alone, because one side of the
test is under the author's control. The fix is not a different rule —
it is prose that says so, with `Disagreement` as the worked case and
"consults" marked load-bearing. **The next unit sorts twenty writers on
that paragraph**, so a paragraph claiming completeness it lacks is
worse for the sweep than an honest one.

### Claims that were false

`SeamSubject`'s *"the two answers are the same answer by
construction"* — the pick-index seam's two channels read **two
different consts on two different types**, each a `Subject::Display`
literal, kept equal by a test rather than by construction. The trait's
headline property is not delivered for the seam that motivated it.

*"Nothing retires a badge, because nothing stores one"* — contradicted
by two of the three badges the unit adds: `scene_fault = None` and
`projection_fault = None` are hand-written retirements, the same
bookkeeping spelled as an assignment.

**And the defect the unit fixed is real while the reason given for it
is wrong, in four places.** Both production `land()` calls sit inside
`viewport_ui` before `view_projection` with no return between, so the
`Expire(Camera)` story does not reproduce. The real mechanism is that
`perform_batch` is the last statement of `update()` and the line is
drawn in the toolbar **before** the panes — `Clear` erased the sentence
and it was never drawn on an acting frame. A true fix with a false
causal story enshrined at four sites is precisely what
`memories/review-and-dependency-policy.md` warns the dispatcher about,
arriving from the other direction.

### A behaviour regression to verify

`projection_fault` may go **permanently** stale: `viewport_ui` returns
early when `aspect()` is `None`, before either writer, and nothing else
writes the field. A pane dragged to zero extent or tabbed away leaves
the last value standing forever. **Under the old code the same
condition left a stale sentence that the next `Clear` swept — a badge
has no sweeper.** The unit's own doc argues the one-frame lag is benign
and does not consider the no-frame case. Told to verify and file it
whether or not it is fixed here.

### The Holds row, three units deferred

No `Holds` row was taken, the README's app-vocabularies table still has
no `frame` entry, and this unit **grew the prose section beneath that
table from 29 to ~46 lines** — the exact shape the item names as the
problem. `frame.rs` 2036 → 2229. **Third unit running to skip the cheap
half while making the file bigger.** Told to take it in this PR and to
append what deferring it three times cost.

## Ev ruled the rule over its own example (2026-09-06)

> "your recommendation seems fine here"

**The rule governs; `unindexed_refusal` stays on the line.** The lane
built the rule against the ruling's worked example, escalated the
contradiction rather than resolving it quietly, and was right.

### The reasoning, recorded because it is the sweep's sorting rule

The two candidate tests differ on **whose event decides**:

- the **rule** asks *what caused this sentence to exist* → an act;
- the **example** asks *what this sentence is about* → a seam.

Every refusal is about something and caused by something, so both are
coherent axes. **Provenance wins because it is the only one a user can
see.** `pick::unindexed` gates on `any(|a| matches!(a, Select))` and
`.then_some(...)`, so the sentence exists only if the frame carried a
click; as a badge it would be lit for the whole 2–13 second window
regardless. That is the observable difference and there is no other.

Three consequences, all verified against the tree before the answer
went out:

1. **It would put one sentence in two places.** `app.rs:1225` and
   `:1237` already hang `NotIndexed::Building.to_string()` as the
   spinner's hover text, and `frame::Progress`'s header
   (`frame.rs:1051-1057`) forbids two indicators lit for one wait "with
   no rule anywhere saying which the reader should believe".
2. **It would undo half of #1843**, whose deliverable was the indicator
   *and* the pick path distinguishing "not indexed yet" from "nothing
   under the cursor" — the indicator held state, the refusal the answer
   to a click. Merging them returns to a spinner over inert picks,
   which that ruling refused as fail-quiet.
3. **The example's test costs frame state and the rule's does not.**
   Since "held" is author-chosen — this unit *created* `scene_fault`
   and `projection_fault` so those facts would qualify — badging seam
   refusals means minting a held field per refusal, each a new entry in
   `ViewerApp`'s frame-state inventory, which is what GQ6's toolkit
   decision rests on and what #1843 was careful not to grow.

### What it settles beyond the door

`pane/viewport.rs:172` ("a cursor action the pick index refused")
**stays news** for the same reason. Under the example it would have
moved, and with it the whole class *"a refusal about a seam"* — the
largest coherent group on the sweep's news list after the tool
refusals. So the answer sorts a class, not a door, which is why it was
worth asking rather than deciding.

The sweep item gets a sentence naming the settled axis; it does **not**
get re-sorted here, that being the sweep's own unit.

**Third ruling this program has taken where building the thing
produced the option the ruling needed.** #1883 answered from a
vocabulary that had no "badge with a subject"; #1945 supplied it; and
#1945's own example then turned out to be refuted by the code it named.
Each time the ruling was sound and the example was the casualty —
which is an argument for worked examples being checked against the
tree at the moment a ruling is written, not a reason to stop giving
them.

## #1953's fix pass: the `|| true` class was three sites older than its fix (2026-09-06)

Green on the merged head after **386 commits of `origin/main`** — gate
and `--selftest` re-run against the merged tree rather than the branch
point, and the roster absorbed a day of other programs' work (31
vocabularies, 10 drivers, 41 modules).

### Corrected me twice, both about state rather than judgement

The whole fix pass had **already landed** before the rate limit; I
resumed it believing it had stopped early. And its earlier head had
**already gone green** — the lane was mid-poll when it was cut off, so
the result existed and neither of us had seen it. Exactly one item was
genuinely missing.

**The cause of that one is worth recording**: a python heredoc edit
block raised on an earlier substitution, so the write that followed it
never ran — **a silent partial apply with no error surfaced**. The lane
re-verified every owed item by grep rather than by its own transcript,
which is the right response and the one that found it.

### The class was older and wider than I framed it

I sent one instance and named three more as a sweep. **All four were
live, three of them predating this unit's fix**, and the worst — the
exception filter, whose pattern is interpolated from an entry — could
have greened the gate over a malformed needle indefinitely. All four
now use `gate_grep`. The lane swept the whole `scripts/gates/`
directory rather than the file I named: **no other live instance**.

### It caught itself making the opposite mistake

Folding `grep -q` into `gate_grep` where **exit 1 *is* the answer**
would have turned a predicate always-true. It found that because a new
check then **passed on a fixture that lacked its subject** — not "did I
do what the review said" but "did doing it break something the review
did not mention". `lib.sh` carves that case out by name, which is the
confirmation.

That is the second time in two units a lane has been caught out by a
fix that reproduced the defect it closed, and the first time a lane
caught itself.

### Two negative controls, neither an accident

The zero-hit control forces the list empty so the gate must still print
OK — it stays exercised the day an entry returns. And the four
exception arms **stopped borrowing a live defect**: they used to aim at
whatever `VOCAB_EXCEPTIONS[0]` happened to be, so retiring the last
entry retired their coverage. They now plant their own, via an override
honoured **only when `GATE_ROOT` is set** — under `--root`, which only
the self-test passes, so no environment can exempt the repo.

**That closed the residue the lane had filed, and its file was deleted
rather than left scheduled.** Right: a residue closed in the PR that
filed it should not survive as an open row. Asked for the PR body to
say so, since a deletion is otherwise indistinguishable from an
oversight.

### One file owed before merge

The lane answered the `select_pick.rs` question well — **`build` must
not take `IndexInputs`**, because `build` is the pure function the
worker runs off a request that *owns* its copies, and a session-borrowed
value on the far side of that seam contradicts a contract saying the
worker owns everything it reads. Its reading is that **`IndexRequest`
is already the concept** and `IndexInputs` is its borrowed counterpart
at the one door that mints it.

That is a residue inside VIEW's own fence and it is in a PR body rather
than a file. Asked for the file before merge — merging without it
repeats exactly the failure this program has spent the session fixing.

## #1957's fix pass: the rule reads honestly, and a regression is confirmed (2026-09-06)

Green on the merged head after 386 commits of `origin/main`. Ev's
ruling recorded and the item closed.

### The rule now says what it can and cannot do

The header used to claim of its mechanical restatement *"that is the
whole test."* All three reasons it is not are now written where the
sweep will meet them:

- **held-state is a property of the FACT, not of a signature** —
  `unindexed_refusal` takes a `&NotIndexed`; what makes it an outcome
  is that `pick::unindexed` raises it for a `Select` and nothing else;
- **tracing to the raiser does not settle it either** — `Disagreement`
  reads only held state and is recomputed every frame the cursor holds
  still, so the mechanical form alone badges it; what sorts it is *a
  reader consults*, which is load-bearing;
- **whether a fact is held is a choice the author makes** — with this
  unit's own two new fields as the worked case.

That last sentence is uncomfortable to write about one's own diff and
is the most useful one in the header. A rule that states its own limit
is worth more to a twenty-site sweep than a crisper rule that does not.

### S18 confirmed: a regression this unit introduces

`projection_fault` is written in exactly one place, and `viewport_ui`
returns before both writers when either extent is zero — and is not
called at all when the pane is not drawn. **The old code had a sweeper
and the new code does not**: the same condition used to leave a stale
sentence that the next `Clear` took, and no `StatusUpdate` reaches a
badge.

Filed with both arms analysed. **The cheap arm — clearing at the
zero-aspect return — is asked for before merge**, because it costs one
line, adds no state, and closes the arm a user reaches by dragging a
splitter. The tabbed-away arm needs a "the viewport did not draw this
frame" latch, which is a **third** piece of app-gated state and so
exactly what `frame.rs`'s header condemns; that is the sweep's or its
own unit's.

**Why the half rather than filing both and merging**:
`docs/REVIEW-STYLE-DISPATCH.md` §4 says disclosure must not function as
immunity, and this unit *introduces* the regression. A filed item is
not a licence to ship the cheap arm of one. The lane is invited to
push back if the one line is wrong.

### The Holds row, taken — and the honest number

`frame` has a row in the README's app-vocabularies table at last, after
three units deferred it. The row argues, as `forms`' does, and says
plainly that the charter justifies taking each concern **out of `app`**
and not their being one module — pointing at the split item.

And the lane reported that **the file still grew: 2,037 → 2,298.** The
prose beneath the table shrank where the row took the load; the header
grew more, because making the rule true cost more lines than the false
version did. A lane that takes a deferred obligation and then reports
its own headline metric getting worse is what makes the metric worth
having.

## #1953: the owed file landed (2026-09-06)

`IndexRequest`/`IndexInputs` filed as VIEW's residue rather than left
in a merged PR body. CI running on the new head. That is the last thing
either of these two units owed.

## #1953 MERGED, and two working-style findings worth keeping (2026-09-06)

Merged at `44192896`. Ev's #1883 hoist is on main: the boundary rule
stays **unqualified**, `pick.rs` and `parts.rs` take values, and the
gate's two site-granular exemptions retired with the sites they
described. Board: 40 open, 22 closed.

The owed file landed with it —
`index-request-and-index-inputs-are-one-concept-twice` — carrying the
part that is hard to rediscover: **`build` must not take
`IndexInputs`** because `PickIndex::build` is the pure function the
worker runs off an `IndexRequest` that **owns its copies**, and that
ownership is the seam's contract — it is what lets the session go on
being edited while a build is out. A session-borrowed value on the far
side either fails at the lifetime or forces the seam to hold a borrow
across the thread boundary it exists to avoid.

The file's job, in the lane's own words, is to make the next reader
**meet the seam before the duplication**: anyone seeing `IndexInputs::of`
beside `IndexRequest`'s five fields sees two names for one shape, and
the collapse that looks like a simplification is the one thing that
must not happen. Its recommendation — two types with the relationship
written down, rather than one parameterised over ownership — is right
for two: a relationship is worth stating at two, and a mechanism does
not pay for itself until more.

### Two findings about how the work is done, not what it produced

**The `grep -q` catch, and why its framing matters.** The lane did the
mechanical thing the review asked — *write `gate_grep` everywhere you
wrote `grep`* — and then asked **what that broke**, not what it fixed.
`gate_grep` folds exit 1 to 0, which is right for a filter and
**inverts a predicate**; the tell was a *new* check passing on a
fixture that lacked its subject, a green that should have been
impossible. `lib.sh` had already carved the case out by name, so the
rule was there to be read; what was missing was reading it before
applying its sibling.

That is a better account than "made a mistake and caught it", and it
generalises: **after applying a review's rule mechanically, ask what
the application broke.** A review says what is wrong; it does not say
what obeying it costs.

**The silent partial apply.** A multi-edit python heredoc that raises
mid-script leaves every write after the raise undone **with nothing on
screen saying so** — which is how the one genuinely missed item in that
unit was missed. The cheap countermeasure, which found it: **re-verify
every owed item by `grep` against the tree, never against your own
transcript.**

That is this program's own prose discipline pointed at one's own work,
and it is the same shape as the citation-staleness class: a record of
what you did is not evidence of what is there. Recorded here rather
than filed, since it is about how a lane works rather than about this
codebase — a `memories/` amendment if it proves to recur, and that is
Ev's call.

### Standing down the lane

Told not to start new work. The sweep wants #1957 on main first: it
holds `frame.rs`, `app.rs` and `pane/*`, which the sweep rewrites, and
merging a twenty-site sweep against a moving version of the rule it
sorts on is the one avoidable mess left.

## The sweep's brief, drafted from a lane's own two conditions (2026-09-06)

The hoist lane stood down clean and left two conditions for the
twenty-writer sweep's dispatch. Both are right and both go into that
brief rather than being rediscovered:

**Merge `origin/main` before starting, not only before pushing.** Its
last two units each found that a day-old branch point turns a
mechanical pass into a conflict resolution, and the merge is cheaper
before the edits than after. The sweep lands in files #1957 rewrites,
so this is not hypothetical.

**Derive the inventory BY SHAPE, not by the count the brief gives.**
This program has now been wrong about that count three times — the item
said nineteen, the README and `frame.rs` said eighteen, the true figure
is twenty, and the extra one (`viewport.rs:205`, the `unindexed`
refusal) was absent from the item's own hit list at its own merge base.
A sweep that trusts a handed-down census inherits its errors; one that
re-derives by pattern and **reports what the pattern could not match**
does not. The lane's claim 3 in the hoist was false as worded and only
turned up because the sweep was run rather than trusted.

Recording them here so the brief carries them whoever writes it.

## #1957 took the cheap arm (2026-09-06)

`d6e53420` — *"a pane with no extent holds no projection refusal"*. The
title is the argument: a pane that is not projecting holds no
projection refusal, so clearing at the zero-aspect return is the honest
read rather than the convenient one. The tabbed-away arm stays filed;
it needs the "did not draw this frame" latch, which is app-gated state
`frame.rs`'s own header condemns.

49 commits behind main after #1953 landed, so another merge is owed
before it can go.

## #1957 MERGED; the sweep is out, and nothing is blocked (2026-09-06)

Merged at `8fec47fb`. **Nine units on main this session** plus three
tracker/ruling PRs. Board: 40 open, 23 closed, one dispatched.

The cheap arm of the regression landed with a better argument than the
one I sent. I gave three reasons; the lane found the one that decides
it: **with no extent the pane never reaches `view_projection` at
all**, so a held fault is not merely stale on that path — it is a claim
about a computation that was not attempted. Clearing is the only honest
read, not a cheaper approximation of one.

And it re-derived what the field *means* as a consequence: **"what the
viewport said the last time it could project"**, which is a statement
the field can keep, where "the last time it drew" was one it could not.
It also checked the risk I flagged rather than accepting my framing:
one reader, `frame::projection_badge`, so the whole observable
consequence is the badge going dark while the pane has no extent and
returning a frame after the pane does.

**It declined the other arm for the right reason** — taking it would
have added the third piece of app-gated state that this unit's own
header argues against, in the same diff that argues it.

### The item closed on what the unit added to the ruling

`news-and-standing-facts-are-orthogonal-axes` is closed, and what it
records is that **the rule as built states its own limit**, which the
ruling alone could not give the sweep: held-state is a property of the
fact and not a signature; tracing to the raiser does not settle it
either; and whether a fact is held is **a choice the author makes**,
with the unit's own two new fields as the worked case. Ev's ruling then
names the axis all three are shadows of — **provenance**.

That is the third time this program has taken a ruling, built it, and
had the building produce something the ruling needed and did not
contain. Worth stating as the pattern it is: **a ruling is a
hypothesis about a vocabulary that does not exist yet, and the unit
that builds it is the first thing able to test it.**

### The sweep is dispatched

`view/status-line-sweep`, twenty writers, the largest item on this
board and blocked since #1849 filed it. Both conditions the lane set
when it stood down are in the brief: **merge main before starting, not
only before pushing**, and **derive the census by shape rather than
from the count I hand over** — with the count explicitly marked as the
least trustworthy claim in the brief, because this program has been
wrong about it three times.

The brief also carries the rule's three limits as things to apply
rather than skip, and the standing instruction that **a site the rule
cannot sort outranks the sweep** — the same fence that has now produced
a finding on each of the two units it was applied to.

## The sweep: the census was wrong four times because the TEST was wrong (2026-09-06)

**#2026 green** on `a0e71d3b0`. Under style review. The twenty-writer
sweep is built — and the headline is that there are **eighteen**, and
the lane found why the number kept moving.

### The membership test, not the counting

Every previous count used *"reaches the field outside the ranking"*.
The right test is **"can put a SENTENCE on the line that the ranking
never saw"** — because **applying a verdict outside the ranking is not
writing one, and a retirement must not be ranked.**

That is why the number was wrong four times (nineteen in the item,
eighteen in the README and `frame.rs`, twenty from #1933's recount, and
now eighteen again on a different test): each recount counted more
carefully against a test that was itself wrong.

Two sites come off, and both are instructive:

- **`frame::cursor_status` returns only `Keep`/`Expire`.** It can never
  put a sentence on the line, so it was never one of these writers —
  **it is the well-behaved shape the item asks every writer to
  become**, and it had been counted as one of the offenders.
- **`frame::dialog_status`'s `Show` arm is unreachable at both call
  sites.** Open… and Save As… are `add_enabled(chooser.usable(), …)`
  and the arm needs `usable: false`, so a click implies usable.
  Latent, not live — and it goes through the new door anyway, so if
  that guard is ever removed the sentence lands in the notices.

**The README and the header now state the distinction, not the
number**, so the next reader inherits the test. That is the correct
repair for a figure that has been restated wrongly in three documents.

### `notices` is an addition, not a move — and the reason is the finding

I briefed this as *"`ViewerBehavior` carries `status` and not
`notices`, so that field moves with the sweep"*. Wrong: **`status`
stays.** A notice cannot express a retirement, so `cursor_status` and a
clean fold still need the field.

Hence `frame::deliver`, a door for a policy that may or may not have
something to say: `Show` → notices, `Keep`/`Expire`/`Clear` → field.
`apply_status` stays for the **ranked** verdict. Whether two doors over
one vocabulary survive contact is the first thing the style review is
asked.

### An honest miss, reported rather than found

Two rows **failed in CI and not locally**, and the lane says why: the
`app` feature gates whole test modules, and it ran `-p viewer` and
`clippy --all-features` but not `cargo test -p viewer --features app`.
It reported the gap and added the command to what it runs.

That is worth carrying past this unit: **`--all-features` on clippy
does not imply the test binaries under that feature were run.** It is
the same shape as `memories/agent-lane-operations.md`'s "a step can be
green having EXECUTED nothing", one level down and inside a lane rather
than in CI.

### Filed rather than built, as the brief asked

`startup-notices-need-holding-to-badge` — the startup preferences
notices badge under the rule but **cannot badge without being held**:
they are consumed at construction, and what would retire a held one is
a design question, since nothing watches the file. Three shapes costed.
That is the one site the brief named as report-not-build, and it came
back reported.

## #2026's style review: the sweep's own doc-comments described the world it had just removed (2026-09-06)

`status-line-writers-bypass-the-ranking` landed on `view/status-line-sweep`:
seventeen of eighteen writers routed onto the frame's `notices`, and
`frame::deliver` added as the door for a policy that may or may not
have news. The review returned two dispatch-level corrections and nine
findings; this is the fix pass.

### What the review caught, and the shape of it

**Every finding but two was a doc comment describing the tree as it
stood BEFORE the diff in the same file the diff was in.** That is worth
naming as a class, because it is not carelessness about prose — it is
what a sweep does. A sweep touches every writer and no reader, and a
doc comment eighty lines from the nearest changed line is a reader.

- `frame.rs`'s module header said *"It does not yet reach the line
  through the ranking for every writer"* and named `cursor_status` as
  one of two that bypass it. Every clause false. The lane's only edit
  to that header was `twenty` → `eighteen`, seventy lines earlier, so
  one doc comment said the sweep was done in one paragraph and open in
  another.
- `joined_subject`'s doc said *"Every notice a frame produces today
  agrees, and agrees on `Document` ... the disagreeing case is
  reachable only by a writer that does not exist yet."* The diff put
  four more subjects on that list.
- `ViewerBehavior::notices`' doc enumerated its contents as *"what
  THIS frame has to say that is NOT a refusal"*. The diff filled it
  with refusals.
- `apply_status`'s doc named the dialog policy as one of its two
  callers; the diff moved the dialog policy to `deliver_status`.

### The correction that is not a doc fix

**"Every sentence the line can hold now comes through the ranking" was
false, in `crates/viewer/README.md` and in the item's own `## Closed`.**
`app.rs`'s `status: frame::startup_notices(&notices)` still builds the
field directly, and it is one of the eighteen the next sentence
enumerates. Seventeen-of-eighteen is the honest claim and is now what
both carry, with `startup-notices-need-holding-to-badge` named at the
README and not only in the item. The census has now been wrong five
times and the reason has been the same every time: a count is a claim
about a membership test, and the test — **"can this writer put a
SENTENCE on the line the ranking never saw"** — is what the header now
states instead.

### Taken

`deliver`'s catch-all arm (`retirement => apply(...)`) was the one
finding that was a defect rather than a description: it routed any
future `Show`-shaped variant to the field by default, which is exactly
the door's reason for existing, unguarded by the compiler. All four
arms are written out now, which also disposed of the binding named
`retirement` that was catching `Clear`.

The `deliver` row gained a `Clear` block and a pre-seeded notice, so it
asserts APPEND rather than arrival — `frame_status`'s rank 2 joins
*"in the order they happened"*, which a `Vec` seeded empty cannot see.
`pane/viewport.rs`'s moved row stopped hand-simulating the ranking with
`status = notices.pop()` and now runs `frame_status` + `apply`, which
is `perform_batch`'s own pair; the row's name claims the camera refusal
was one `land` landed, and now it is.

### The S7 census, since it was asked for

Every `frame.rs` row composing a `*_status` producer with `frame::apply`:
**five rows, six compositions, two of them dead** — both
`apply(status, fold_status(refused))`, a shape no caller performs since
a refused fold goes through `deliver`. One is the row the review named
(renamed to say what it pins). The other,
`a_clean_fold_retires_the_camera_refusal_it_did_write`, uses it only to
build a fixture and its name overclaims nothing, so it is filed rather
than fixed: `a-fold-row-composes-a-producer-with-a-dead-door`.

### Filed

- `ranked-and-unranked-verdicts-are-one-type` — the reviewer's, and the
  sharpest thing in the report. `frame::apply` and `frame::deliver` take
  the same type, and `pane/viewport.rs` calls both, ten lines apart, in
  one file. The rule for choosing is *"can this policy ever answer
  `Show`?"*, a fact about the callee invisible at the call site. Give
  `cursor_status` a `Show` arm and `:178` reintroduces the swept defect
  with **no diff at which it looks wrong**. The sweep's own fix left one
  instance of the class the sweep was about, one level up. A ranked
  verdict should be a different TYPE from a policy's verdict.
- `a-fold-row-composes-a-producer-with-a-dead-door` — above.
- `loud-skip-row-did-not-stop-a-lane-verifying-the-wrong-build` — the
  lane verified a diff that is `app`-gated in every file but one with a
  default-feature `cargo test -p viewer`, which compiles none of them.
  `lib.rs`'s loud-skip row exists to make that absence visible and it
  printed into a green run nobody read. The closed
  `loud-skip-marker-says-two-modules-and-there-are-six` is about the
  marker going stale; this is the marker being accurate and read past,
  which its own fix does not touch.

### Appended

`one-line-one-subject-loses-a-mixed-frames-expiry` — the item predicted
this unit would make its fallback reachable and named the Cursor arm.
**The Camera arm is new and worse**: a frame carrying a camera refusal
and a tool notice joins to `Subject::Document`, and `fold_status`'s next
`Expire(Camera)` then retires nothing — `camera-fold-clears-status-line`'s
defect reappearing through the join instead of through `land`. Its
*"Why it is not reachable yet"* heading was falsified by the diff and is
corrected; `SUBJECTS_WITH_AN_EXPIRY_ISSUER` is the assertion that says
why it matters.

`joined-notices-nest-their-own-separator` — notice producers went from
four to eighteen, so multi-notice frames went from unusual to routine
(a create-pane refusal plus a camera fold refusal is one drag).

`frame-module-has-eight-concerns-and-no-holds-row` — the accumulation
ledger stopped being written two units ago: 984 → 1,131 (#1886) → 2,037
(#1933) → 2,298 (#1957) → **2,475** here, a second door and a 45-line
row into the concern that was already the largest. Recorded on it as
evidence and not as work: `chooser_backend`/`zenity_on_path`/
`session_bus_hinted`/`prefs_path`/`prefs_path_in`/`running_under_wsl`
is ~165 lines of startup environment probing in a module whose first
line is *"The per-frame policies the viewport runs — as values, so they
are replayable."* Neither per-frame nor replayable — and the
`no-ambient-env` ruling that put them here settles WHERE the ambient
door is, not whether the charter sentence covers it.

`stale-file-citations-after-the-split` — **seven sibling files cited the
now-closed sweep as live**, and not one of them by a `file:line`. The
item's thesis arriving from the other side: `work.py lint` resolved the
id perfectly and the prose around it had gone false. Live rows
corrected (`plan.md`, `news-and-standing-facts-are-orthogonal-axes`,
`one-line-one-subject-...`); closed rows left as written, because
editing a closed item to agree with a later tree destroys the only
thing it is for; this entry is the log's correction, since the log is
not rewritten.

### The count that is worth carrying forward

The nineteen/twenty/eighteen churn was never an arithmetic problem. It
was five successive attempts to count a set nobody had defined, and it
stopped when the test was written down instead of the number. The same
lesson is what the review's dispatch-level findings are: a doc comment
that states a COUNT goes stale silently; one that states a TEST goes
stale loudly, because the next reader can apply it.


## #2026 MERGED, and the orchestrator branch had gone 32 commits unmerged again (2026-09-06)

**#2026 merged at `c2d7e78ce`.** Ten units on main this session. CI: 37
jobs, twelve `test (…)`, five `k-lint (gate, …)`, `gate ok` success,
zero non-green — verified against the job list rather than a summary,
because the shape is the check and a narrowed run would still say
"green".

### The slow job, and why it was not treated as a flake

`test (interval, eps = 1e-12, 1/2)` ran for ~35 minutes against **528s
on this same branch's previous green run**, while its sibling shard
finished in 77s. The shard imbalance is the tier's normal shape; the
3.5× was not. It was left to finish rather than re-run, on the
reasoning that a job progressing through its steps is not a flake and
"flake" is not a root cause — `docs/prompts/implementer-discipline.md`
and this session's own CI rules both say a re-run is earned by evidence
that a failure is not this PR's, not by impatience. It finished green.
Nothing in the diff (doc comments, one `Clear` test block, one viewport
row that now calls `frame_status`) could have added twenty minutes to
an interval-backend run, so the read is a degraded runner — stated as a
read, not a finding.

### What the fix pass took, and the one thing it did not

Both dispatch corrections landed. **D1** rewrote `frame.rs`'s header
around the MEMBERSHIP TEST rather than a count — *"a writer is one of
these if it can put a SENTENCE on the line that the ranking never
saw"*, and a retirement says nothing, so ranking one is a category
error rather than a stricter discipline. That makes [`apply`] a
legitimate door by argument instead of by exception, which is the part
the old header could not say. **D2** replaced *"every sentence"* with
**seventeen of eighteen** in the README, the item's `## Closed` and the
PR title, and made the README name the filed followup itself rather
than leaving it discoverable only from the item.

**S5 was the only real defect in a fourteen-finding report**, and it
was in the door the unit had just built: `deliver` matched `Show`
explicitly and bound the other three arms by wildcard, so a future
`Show`-shaped variant would have been routed silently to the field —
the exact defect the door exists to stop, unguarded by the compiler in
the one place that could not afford it. Four explicit arms now.

Everything else was prose describing the tree as it stood before the
diff in the same file as the diff. That is the ninth time in three days
this program's prose has outrun its tree, and the first time the
mechanism was named rather than just counted: a doc comment stating a
COUNT goes stale silently, one stating a TEST goes stale loudly,
because the next reader can apply it.

### Filed rather than fixed, deliberately

`ranked-and-unranked-verdicts-are-one-type` (S10) is the reviewer's
proposal that a ranked verdict should arrive as a different TYPE from a
policy's verdict, so the compiler picks the door. It is right, and it
is a design change on `frame.rs`, so it is a file and not a fix pass.
The sweep's own defect class survives one level up: `viewport.rs:46`
and `:178` are ten lines apart, both take a `StatusUpdate`, and which
door is correct is a fact about the CALLEE's arms that no type carries.

`a-fold-row-composes-a-producer-with-a-dead-door` carries the full S7
census (five rows, six compositions, two dead, both
`apply(status, fold_status(refused))`). The second instance was filed
rather than renamed because its name does not overclaim — it uses the
dead composition only as fixture setup — and renaming a row that
asserts something true is churn.

`loud-skip-row-did-not-stop-a-lane-verifying-the-wrong-build` is the
one the lane filed against itself, and its argument is what makes it
not a duplicate: the closed `loud-skip-marker-says-two-modules-and-
there-are-six` was about the marker going STALE, and this is the marker
being **accurate and read past anyway**. Making a sentence true is
orthogonal to whether the sentence does any work.

### The standing hazard, reproduced by its own orchestrator, again

This branch was **32 commits and ~1,550 log lines ahead of main** when
#2026 merged — a whole session of adjudication that existed only here,
while main's `plan.md` and `log.md` described a state two waves old.
That is the same failure #1912 was opened to repair, in the same
program, by the same role, four days later. The countermeasure is not
another item: it is that the state-sync is part of merging a unit, not
a thing done afterwards when there is a gap. Recorded here because the
plan's "standing hazard" section says the only instrument is a
successor reading `git log` before believing the tail, and a successor
reading this one should know the instrument failed for the person who
wrote it down.

## `view/const-all` dispatched and reported; its review is running (2026-09-06)

The two `const ALL` items were dispatched together —
`viewer-const-all-tables-have-no-exhaustiveness-guard` and
`tool-kind-all-and-ordinal-have-no-production-reader` — because both
items say answering them apart answers one question twice: whether a
table should exist at all, and what forces the ones that remain.

**The brief stated the PROPERTY and refused to prescribe the
mechanism**, on the grounds that the item's own proposal (`fn all()`
from a `match`) satisfies *"a new variant fails to compile"* and fails
*"membership is written once"* — it has two lists. That was the one
judgement the dispatcher could get wrong cheaply, so it was handed over
as an open question with the trap named.

**The census was stale in this program's usual direction**: the item
said five tables, the tree has ten `const ALL` names. The lane reports
nine class members with `Theme::ALL` excluded (a registry of struct
constants, not an enum's variants). That is the second census this
program has got wrong because the MEMBERSHIP TEST was wrong rather than
the counting — the same finding #2026 made, in a different item, four
hours apart.

The lane's answer is a `vocabulary!` macro in a new
`crates/viewer/src/vocab.rs`: one variant list expands into the enum
AND its `ALL`, so the two are the same tokens rather than two lists a
check has to reconcile. Stronger than what was asked for. Three of its
claims were checked by the orchestrator before the review was
dispatched, because each could have been a confident wrong answer
rather than a refusal:

- **The `DatumKind` reorder is inert**, and better than inert. The enum
  derives only `Debug, Clone, Copy, PartialEq, Eq` — no `Ord`, no
  serde, no integer cast — and its only readers are two exhaustive
  matches in `pane/create.rs`. The old DECLARATION order was
  Plane/Axis/Point/Frame while the old `ALL` DREW Plane/Frame/Axis/
  Point, so the reorder collapses a discrepancy rather than creating
  one.
- **The precedent claim is real.** `crates/profile/src/path/program.rs`
  `:175` and `:361` are `macro_rules! arc_modes` and
  `macro_rules! transition_table`.
- **The deleted test was vacuous once `ordinal` went.**
  `every_tool_kind_is_listed_in_all` asserted only that `ordinal` and
  `ALL` agreed; with `ordinal` deleted and `ALL` projected from the
  enum's own tokens there is nothing left to compare.

Those three went into the review brief AS the dispatcher's claims, with
a request to refute rather than repeat them. This program's dispatcher
corrections have outnumbered its implementer defects, and unit 1's
chain alone produced seven.

Style review only, no correctness lane — but this is the closest a unit
has come to the line Ev drew, and the brief says where: a macro that
DEFINES enums plus a variant reorder is a shape whose failure mode is a
confident wrong answer, so the three questions above were settled by
the orchestrator first rather than left to the style lane to notice.
## The `const ALL` class: ten tables, nine members, and a census wrong for the same reason again (2026-09-06)

`view/const-all` took
`viewer-const-all-tables-have-no-exhaustiveness-guard` and
`tool-kind-all-and-ordinal-have-no-production-reader` together, because
both said they had to be: one asks whether these tables should exist
and the other how to force them, and answering them apart answers one
question twice.

**The count in the item was five. The tree had ten, and the class is
nine.** That is the second census in this program to be wrong because
the MEMBERSHIP TEST was wrong rather than the counting. `grep "const ALL"` is a name test; the class is a shape. The
corrected census was derived in two passes — the name grep, then a
structural scan for array literals holding two or more
`Type::Variant` entries anywhere under `crates/viewer/src` — and the
second pass is what turned up the members that are not called `ALL`
(`forms::BOOLEAN_OPS`, `forms::MATE_PRIMITIVES`) and the near-members
that are (`Theme::ALL`). Both passes and the membership test are
written into the closed item so the next census does not have to
re-derive them.

`Theme::ALL` is confirmed out: `Theme` is a struct and `ALL` is a
registry of three struct constants, so there are no variants and no
exhaustiveness for a match to borrow. The dispatcher's belief was
right.

**The mechanism is the repo's, not a new one.**
`crates/profile/src/path/program.rs`'s `arc_modes!` and
`transition_table!` already declare an enum and its `ALL` from one list
("ONE declaration, THREE projections"); `crates/viewer/src/vocab.rs` is
the viewer's two-projection case. It gives more than the property the
dispatch asked for: adding a variant without extending the list is not
a compile error but unwriteable, because they are the same tokens. A
derive crate was rejected (a dependency in a crate whose default-feature
graph is deliberately the kernel's, to save fifty lines), a successor
walk was rejected (satisfies both properties and spreads a
seventeen-verb order across seventeen arms), and `fn all()` from a
match was rejected as the item's own proposal that fails its own second
property.

**No gate, and the reasoning is the interesting part.** The claiming
note named `scripts/gates/viewer-module-kinds.sh` as the machinery a
fix would use, and it would have worked. But once the compiler owns the
property, a gate can only catch a NEW hand-written list — and the three
kinds of list that legitimately stay hand-written (a struct registry, a
deliberately partial list, a mirror of another crate's enum) are told
apart by judgement, not by a scan. A gate over them would be a checker
of judgement. The answer is a sentence, and the sentence is
`crates/viewer/README.md`'s new **Closed vocabularies are declared
once**, which is where a new hand-written list meets its three
neighbours and has to say which it is.

**The reader-count sweep the second item asked for was run and its
pattern is too coarse to be the class.** "`pub` items whose grep hits
outside `src/` are all under `tests/`" returns twenty names on
`167dc4f84` and finds no new instance: eighteen are read-back doors
whose docs describe what they answer, which is the crate's stated
headless posture rather than a defect. The discriminator that makes an
instance is the second half of the item's own title — the doc naming a
production consumer that does not exist — and the reader count alone
over-collects by roughly ten to one. It also under-collects: `ALL`
never appears in the sweep's output, because the sweep keys on a bare
name and `Theme::ALL` shares it and has production readers. **The
item's own class was invisible to the item's own sweep**, which is the
named blind spot (a name grep does not resolve a name to a definition)
biting at home rather than at the re-export case the item predicted.

`ToolKind::ordinal` and `Seat::ordinal` are deleted: each existed to be
the compiler-forced half of a hand-written list's completeness, and
there is no hand-written list left for them to be read against.
`ToolKind::ALL` and `Seat::ALL` stay `pub` — the suites that read them
are integration tests and see only the public surface, so every
"move it behind the suite" answer puts a hand-written list in a test
file with nothing forcing it.

The mirror residue (`BOOLEAN_OPS`, `MATE_PRIMITIVES` — tables whose
enum is declared in another crate, so nothing here can project them)
was filed as its own item at the moment it was disclosed, per
`work/README.md`: a residue named only in a PR body dies with the
directory.

## #2046's style review: the mechanism held, and the costs it hid were the findings (2026-09-06)

The `const ALL` unit's review returned eighteen findings and four
filed items, with a verdict on the substance that is worth recording
before the fixes: **no behaviour change anywhere, no lost `ALL` order,
no visibility change**, all nine converted tables byte-order-identical
to the arrays they replaced, and each of the three claims the
dispatcher handed over as its own confirmed — one upgraded from
`likely` to `sure`. Nothing in this pass changes what the unit did.

What the pass is about is a pattern the program should expect from a
mechanism change: **the defects were not in the mechanism, they were
the mechanism's costs going unmentioned, and the prose that outran the
tree.**

### rustfmt stops at the invocation, and the idea for fixing it is refuted

The largest of them. `rustfmt` does not reach inside a `macro_rules!`
invocation in item position, so it now formats none of the nine
converted enums — every variant and every variant doc of `PathVerb`
(17), `Seat` (9), `ToolKind` (7), `ArcMode` (6) and five more. The
reviewer demonstrated it rather than asserting it: a variant
re-indented to column 21 inside `blend.rs`'s block leaves
`cargo fmt --check` at exit 0.

The dispatch's idea was that rustfmt bails because `pub const ALL;`
does not parse as a Rust item, and that moving the `ALL` declaration
onto the enum as an attribute would leave the body one well-formed
item. **Tested and refuted**: with the body rewritten to exactly that
shape, the mis-indent still passes; delimiting the invocation with
`()` instead of `{}` does not reach it either. It is the invocation
rustfmt declines, not the body. The cost is real, unavoidable inside
this construction, and is now stated in `vocab.rs`, in the README
section, and in an item of its own — and the kernel side has been
paying it for `arc_modes!` and `transition_table!` for longer, which
is where the next lane should look first.

### A doc link that only a reader of the rendered page could see

Five new rustdoc warnings, and the one that matters is on a `pub`
item: `ToolKind::ALL`'s page rendered *"Projected from this enum's
declaration by [crate::vocab::vocabulary]"* with literal brackets,
because `mod vocab` is private. The reviewer read the generated HTML.
`cargo doc -p viewer --features app --no-deps` went 39 → 44 and
`doc-gate.sh` passes over that number, so nothing in CI would have
said. Fixed with plain code spans rather than by making the module
public: the macro is an internal construction and a reader of a
vocabulary's page does not need a link into it. Back to 39.

### The counts, again

`vocab.rs` and the README both opened with "a dozen enums", six lines
above "ten such tables existed; nine were of this kind". In a unit
whose subject is that this program's counts keep going wrong, the
headline sentence overstated by a third. Both now say nine, and the
README says why both figures appear.

`forms.rs` also asserted "the three `DatumSpec` arms" for a four-variant
enum mirroring a five-arm one — pre-existing prose that this unit moved
and rewrote around without reading. Corrected to four-of-five, with the
arm it does not offer named and the reason.

### The census's scan was narrower than its own stated test

The sharpest finding. The unit wrote down its membership test — the
thing that had been missing every previous time — and then described
its scan as "an array literal holding two or more `Type::Variant`
entries, anywhere in `crates/viewer/src`", which is not what it ran:
the regex required the `[` to follow `=`, `[`, `(` or `,`, so an array
introduced by a keyword was never a hit. `for (dimension, label) in
[ … ]` at `pane/properties.rs:156` — a complete inline mirror of
`editor-core`'s `Dimension`, in production, driving what a user can
pick — was invisible to it. The reviewer's identically-*described*
scan returns it.

**Writing down the test is not enough; the scan that applies it is a
second thing and can be narrower.** Re-run without the anchor, the pass
returns five more hits; all five are dispositioned in the closed item
so the next census inherits the work rather than the number.

### An argument falsified by a scope the unit did not disclose

The PR rejected a suite-local list for `ToolKind::ALL` because it
"would be hand-written, unforced and invisible to the compiler — the
same defect one directory over". There are four such lists in
`crates/viewer/tests/` already. The conclusion survives and is sharper
for it (moving `ALL` there would make a fifth), but the argument as
written treated as hypothetical a thing that was actual, and it did so
because every sweep in the unit read `src/` only and never said so.
Both the scope and the corrected argument are now in the item.

### Two judgement calls, decided

**The "no gate" argument was a non-sequitur and is withdrawn.** It said
a gate would be "redundant for every converted one — the compiler owns
those"; a converted vocabulary has no array literal left, so it is not
a hit and redundancy was never the objection. The cheap gate is real
and would work. It is filed rather than written, and the reason is
siting rather than size: a gate must fire on its own inputs, and this
one's allowlist lives in a README, so it needs a `ci.yml` step, a
roster registration and a tier decision — none of them questions about
`const ALL`. §Q6 says a disclosed non-take owes a named schedule, and
an item is one where a paragraph is not.

**The rule that decides a vocabulary's arm is now written down.** The
mechanism was introduced to remove author judgement about membership
and left author judgement about shape unstated. The rule was there to
be read off the tree: a word goes in the TABLE when the row that
iterates the table is its only reader, and in a METHOD when anything
asks a single value for its word — a method can be called on one value
and a table can only be iterated. That is exactly why `PathVerb`,
`ArcMode`, `ToolKind` and `Seat` are bare. The second ordered copy it
leaves in those four is filed; it is not the old defect, because a
match cannot silently miss a variant.

Six items now ride out of this unit: the mirror question and its three
siblings the reviewer filed, plus the rustfmt cost, the gate and the
second-copy question. That is a lot for one style unit, and it is the
right shape: a mechanism that changes nine types at once should leave
its costs on the board rather than in a PR body.

## #2046 MERGED; the census was wrong a THIRD time, and the anchor was the bug (2026-09-06)

**#2046 merged at `7b63c345`.** Twelve units on main this session. CI:
37 jobs, twelve `test (…)`, five `k-lint (gate, …)`, `gate ok` success,
zero non-green.

### The dispatcher's hypothesis was tested and refuted, which is why it was stated as one

The fix brief handed the lane an idea for the rustfmt loss — that
rustfmt bails because `pub const ALL;` does not parse as an item, so
moving the `ALL` declaration onto the enum as an attribute would leave
the body one well-formed item — **explicitly as an idea to test rather
than a shape to build**, after this session's earlier episode where a
dispatcher's "eager gather in `land`" was premised on a fit that never
asks.

The lane ran three experiments and refuted it: the attribute form, the
body rewritten as a single well-formed item, and the invocation
delimited `()` instead of `{}`. All three still let a variant
re-indented to column 21 pass `cargo fmt --check`, while the identical
mis-indent on an ordinary enum ten lines below is caught. **It is the
invocation rustfmt declines, not the body**, so no rearrangement inside
the delimiters helps.

That is the right outcome from a stated hypothesis, and it is worth
naming as a practice: a dispatcher who states a shape gets it built; a
dispatcher who states a hypothesis gets it tested. The cost is now
disclosed in `vocab.rs`'s module doc WITH the demonstration, in the
README, and filed as `vocabulary-macro-bodies-are-outside-rustfmt` —
which also records that `arc_modes!` and `transition_table!` have been
paying the same price longer, so this is a repo-wide fact rather than a
debt this unit invented.

### The third census failure, and the first where the TEST was right

`viewer-const-all-tables-have-no-exhaustiveness-guard` said five;
the tree had ten; the class is nine. That was the second wrong count in
a day, and the log recorded the lesson as *the membership test was
wrong, not the counting*.

**This time the test was right and the SCAN did not implement it.** The
review found a live instance the census missed
(`pane/properties.rs:156`, a complete inline mirror of `editor-core`'s
four-variant `Dimension`, driving what a user can pick) and inferred
that `pane/` had not been scanned. It had. The regex required the
opening `[` to follow `=`, `[`, `(` or `,`, so an array literal
introduced by a KEYWORD — `for (dimension, label) in [ … ]` — could
never be a hit. Re-run unanchored it returns five more, two of them
members.

So the class of census failure has three members now and they are three
different mistakes: a stale count inherited from another program, a
membership test that admitted the wrong things, and a scan narrower
than the test it claimed to implement. **A census owes both halves in
writing** — the test AND the method — and the closed item now says so.
That is the sentence a fourth failure would have to get past.

### What the review confirmed, which was most of it

No behaviour change anywhere; all nine converted tables byte-order-
identical to the arrays they replaced; no lost order, no visibility
change. The three claims the orchestrator checked before dispatching
the review — the `DatumKind` reorder being inert, the `arc_modes!`
precedent being real, the deleted test being vacuous — were each
confirmed against the tree, one upgraded from `likely` to `sure`. The
brief asked the reviewer to refute rather than repeat them and it did
the work either way, which is what that instruction is for.

### The two judgement calls came back with better arguments than the dispatch had

**The gate (F11).** The orchestrator leaned "file it, the gate is small
enough for a later lane" and said the call was the lane's provided it
came with an argument. The lane filed it and gave a different reason,
which is the right one: the cost is not the grep, it is that the
allowlist would live in a README section, so the gate needs a `ci.yml`
step, a `gate-roster.sh` registration, a `ci-local.sh` line and a
`check-ci-mirror-parity.py` tier decision — none of which are questions
about `const ALL`. `a-new-hand-written-all-table-meets-no-gate` carries
that, plus `lib.sh:113-141`'s `|| true` hazard and the
read-the-allowlist-don't-restate-it contract. The README no longer
offers prose as the answer; it says a gate is owed.

**The arm rule (F12).** Asked to write down what decides which
`vocabulary!` arm a type takes, the lane read the rule off the tree
rather than inventing one: *a word goes in the table when the row that
iterates it is its only reader, and in a method when anything asks a
single value for its word* — because a method can be called on one
value and a table can only be iterated. Verified at all four `label`/
`name` sites and against the five labelled tables, whose words appear
nowhere but their radio rows. The second-copy question is filed with
the trap named: a labelled vocabulary can still carry `label()`, so if
that ever holds, the two-shape rule collapses and should be DELETED
rather than amended.

### The slow interval shard, resolved

`test (interval, eps = 1e-12, 1/2)` ran ~9 minutes here against ~530s
baseline and ~35 minutes on #2026. Same job, same branch family, three
runs. That retroactively confirms #2026's reading — a degraded runner
on one occasion, not a systematic change the diff caused — and it
confirms the decision not to re-run it. A job stepping through its
phases is not a flake, and this program now has the measurement to say
so next time rather than the argument.

### The wave's output beyond its diff

**Nine new items**, every one a file. `hand-maintained-mirrors-of-a-
kernel-enum-are-unforced`, `vocabulary-macro-bodies-are-outside-
rustfmt`, `a-new-hand-written-all-table-meets-no-gate`, `bare-
vocabularies-declare-their-words-a-second-time`, and the reviewer's
four (`dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum`,
`viewer-suites-hold-hand-written-complete-variant-lists`, `two-
datumkind-enums-name-the-same-four-datum-kinds`, and `work/issues/
boolean-op-has-a-third-hand-written-complete-list.md`, which is in
`issues/` because its site is `editor-core` and no program obviously
owns it).

The board is now **48 open, 27 closed, nothing dispatched, nothing on
Ev**, and no large item left: small units, two waiting on DOCM's
`next_id` door, and the focus-map siting question no single program can
answer.

## `Refusal` has no `ALL`, and does not need one — 2026-09-06

`refusal-has-no-all-to-walk` closed answered-and-fixed. The item asked
for an instrument that cannot exist in the shape it named, and the
question underneath it turned out to have a live defect in it.

**No `ALL` can exist, and none is needed.** A `Refusal` value needs a
payload and a payload needs a document, so an `ALL` would mint fixtures
rather than state a fact about the type; `vocabulary!` (#2046) projects
one for fieldless variants only, and this vocabulary is nearly all
payload arms. What an `ALL` was wanted for — *a new arm must answer for
itself* — the type already has: `Display for Refusal` and
`Refusal::rank` are exhaustive matches with no wildcard, so nothing can
join this vocabulary and reach a user unrendered. That reasoning is in
the item's `## Closed`, because the next reader's first question is why
there is no `Refusal::ALL`.

**The property left over was prose, and one arm was failing it.**
`Refusal::exists_wording` rendered `({dimension:?})`, so the status
line and the add-parameter form's pre-click notice both said
"parameter width already exists (**Length**)" — the variant identifier,
against `Dimension`'s `Display` in editor-core, which declares itself
the one home of the dimension-in-prose rule and says it holds
*wherever a dimension reaches a user*. Three siblings stood in the
properties panel. All four fixed; the refusal one is pinned by
`refusals_render_as_sentences`, which now walks `ParamExists` through
`CreateParam`.

**It got past both instruments for two independent reasons**, and this
is the part worth keeping. `prose_census` scans `impl Display` bodies
and nothing else — but this vocabulary composes three sentences in an
inherent `impl` on purpose, so a pre-click surface and the status line
cannot drift, and `Display` delegates to them through a bare `{}`. The
census cannot see a delegated wording. And had it seen this one, the
verdict would still have been `Prose`: a fieldless enum carries no
`" { "`, so a census that asks about BRACES never asks whether a
`Debug` here spells an identifier a person then reads. Both gaps are
LIB's ground (`crates/pncad-py/*`) and are handed over in the PR rather
than filed onto another program's slate.

**The item's own ask is answered no.** It wanted the census extended so
a `{binding:?}` over a `String` reds, because the row asserted
`!contains('"')`. That would enforce an unratified rule against
deliberate prose — F6 (`display_contract.rs`) lists the `Debug`
fingerprints and a quotation mark is not among them; `EditError` quotes
a user's key on purpose and `MetaUnversioned` names the D7 `"v"` field
by writing it. The clause was a tripwire on correct prose, and the row
now asserts F6's shape instead, with a doc comment naming which half is
the compiler's and which is the census's so it stops reading as a claim
over the vocabulary.

One near-duplicate avoided: the add-parameter form's hand-written
`Dimension` table was filed hours earlier by #2046's style review as
`dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum`, whose
last section reads its labels against the same clause. It stays open
and is not what this unit touched.

## PR 2053's style review, and what a sweep that reads only the tree misses — 2026-09-06

The substance of `refusal-has-no-all-to-walk` held under review: both
matches name all eighteen `Refusal` arms with no bare `_ =>`, every
number in the PR body reproduced, and the closure stands. Six findings
came back. Three are fixed here, two the reviewer filed and left, one
is a coordination miss worth the program's attention.

**The miss is the entry that matters.**
`work/fix/verb-and-dimension-render-through-debug` — open, on FIX's
slate since 2026-09-04 — already enumerated all four `Dimension` sites
this unit "found", at their pre-split paths, and holds a fifth of the
same class the sweep did not reach: `profile::path::Verb` rendered
`{verb:?}` at `crates/viewer/src/sketch.rs:663`, on screen through
`pane/create.rs`. So a unit half-completed another program's open item
without knowing, and the reason is stateable: **the sweep grepped the
tree and did not read the board.** Every sweep this program has run has
scoped itself by shape; none has asked whether the shape was already
filed. The `Verb` half stays untouched — its fix is `impl Display for
Verb` in `crates/profile`, not ours, and forwarding it from the viewer
would mint a fourth spelling of the word list — and
`viewer-preview-names-a-verb-by-its-variant-identifier` now records
where both halves of the FIX item stand.

**Two rows had been left weaker than they looked.** The refusal row
traded `!contains('"')` for F6's `node:`/`name:` clauses, and the
dropped clause is the only one that catches its founding case: a `{:?}`
over a `String` or a `ParamName` renders `"width"`, which carries no
brace, no field punctuation, and leaks the PAYLOAD's identifier rather
than the arm's. The PR's argument for dropping it was about
`EditError`'s metadata arms, which that row does not walk. Restored.
The distinction it turns on is worth keeping: **a tripwire over named
samples may be stricter than the ratified contract; a claim over a
vocabulary may not.** Separately the new `ParamExists` row asked for
the dimension it had declared, so it could not tell the arm reporting
what already stands there from the arm forwarding the request — the
roster-excludes-its-own-failing-mode shape, inside the row pinning this
unit's fix. It now asks for an angle over a length.

**And the closure overstated its reach by one level.** `Refusal::rank`
carried `Self::Display(_) => 1` beside two `DisplayFault` arms
hand-listed at rank 2, so an eighth display fault took a rank nobody
chose. Fixed rather than caveated — that arm matches its payload
exhaustively now — because a closure that has to be qualified is worse
than one made true. `Edit` and `SlotUnit` keep one rank per vocabulary
and the code now says why that is a default and not an oversight.

**One correction to the record.** The PR claimed `prose_census` did not
run hosted because the `python suite` job was skipped. It did:
`prose_census` is a Rust `#[cfg(test)]` module in
`crates/pncad-py/src/`, `scripts/ci-filter.py` puts `pncad-py` in
`PKGS` via the read reach (verified on this diff: `CARGO_SCOPE` carries
`-p pncad-py`, `RUN_PNCAD_PY=false`), and the skipped job is the
maturin suite over `crates/pncad-py/tests`. A false gap recorded in a
merged PR body is the same defect as false prose in a doc comment.

The two census findings are now
`work/issues/prose-census-cannot-see-a-bypassed-prose-renderer` — one
row, not two, because neither half alone catches the defect that
motivated them.

## #2053 MERGED; the sweep read the tree and not the board (2026-09-06)

**#2053 merged at `736846beb`.** Fourteen units on main this session.
CI: 37 jobs, twelve `test (…)`, five `k-lint (gate, …)`, `gate ok`
success, zero non-green.

### The unit found a live user-visible defect while answering a filed hazard

`refusal-has-no-all-to-walk` asked for an instrument that cannot exist
in the shape it named — `vocabulary!` is fieldless-only and `Refusal`
is nearly all payload arms, so there is no `ALL` of `Refusal` VALUES to
project, because a value needs a payload and a payload needs a
document. The dispatch said so and said closing it as ANSWERED was a
legitimate outcome.

It closed answered-AND-fixed, because underneath the question was a
sentence a person reads: `Refusal::exists_wording` rendered
`({dimension:?})`, so the status line and the add-parameter form's
pre-click notice both said *"parameter width already exists
(**Length**)"* — a Rust variant identifier where a person reads prose,
against a rule whose home is `Dimension`'s own `Display`. Four sites.

**Why both instruments were green over it, and this is the part worth
keeping.** Two independent failures. `prose_census` scans
`impl Display` bodies, and these three sentences are composed in an
INHERENT impl **on purpose** — each is "its one home" so the pre-click
notice cannot drift from the status line — with `Display` delegating
through a bare `{}`. A delegated wording is invisible to it. And had it
seen the site the verdict would still have been `Prose`: the census
asks about BRACES, never about whether a `Debug` spells an identifier a
person then reads.

The item's own ask was answered **no**: extending the census to the
quoting question would enforce an unratified rule against deliberate
prose, since `EditError` quotes a user's key on purpose.

### The finding that is the orchestrator's as much as the lane's

`work/fix/verb-and-dimension-render-through-debug.md` — **open, on
FIX's slate since 2026-09-04** — already enumerated, at pre-split
paths, the exact four `Dimension` sites this unit fixed, PLUS a fifth
it did not: `profile::path::Verb` rendered `{verb:?}` at
`crates/viewer/src/sketch.rs:663`, which `pane/create.rs:581-585` puts
on screen.

**The sweep read the tree and not the board.** The dispatch that
briefed it did not say to check the tracker either, so this is the
dispatcher's miss as much as the lane's, and it is recorded that way.
Every sweep this program runs from here owes a tracker pass: an
instance already filed on another slate is one a tree-grep cannot tell
you about, and half-completing another program's item without saying so
is how two programs come to disagree about what is done.

Handled by REPORTING, not by acting: the `Verb` fix is
`impl Display for Verb` in `crates/profile`, which is not VIEW's
territory, and rendering the verb in prose from the viewer would mint a
fourth spelling — the defect one of the review's own findings is about.
The PR body names FIX's item, says its `Dimension` half is complete and
by which PR, says its `Verb` half is untouched, and says it should not
be closed on the `Dimension` half alone.

### What the review caught in the unit's own instruments

**The test lost the clause that caught its own defect.**
`panel_edits.rs` traded `!contains('"')` for two prefix checks, and the
dropped clause is the only one that catches the row's founding case: a
`{:?}` over a `ParamName` renders `"width"` — no brace, no field
punctuation — and the identifier it leaks is the PAYLOAD's, not the
arm's, so `contains(arm)` misses it and `assert_ne!` against the whole
dump cannot see a `Debug` fragment inside prose. The lane restored it
and wrote down the distinction it had got wrong: **a tripwire over
named samples may be stricter than the ratified contract; a claim over
a vocabulary may not.**

**`Refusal::rank` wildcarded its payload** — `Self::Display(_) => 1`
beside two `DisplayFault` arms hand-listed at rank 2 — which was the
one place the closure's "no wildcard" argument did not reach. The lane
fixed it rather than qualifying the closure, on the argument that a
closure needing a caveat is worse than one made true, and labelled the
two arms that legitimately default (`Edit`, `SlotUnit`) with the reason
they may.

**The PR's own CI worry was false.** It said `prose_census` did not run
hosted because the `python suite` job was skipped. `prose_census` is a
Rust `#[cfg(test)]` module in `crates/pncad-py/src/`; `ci-filter.py`
puts `pncad-py` in `CARGO_SCOPE` via the read reach, so it ran in all
twelve `test (…)` jobs, and the skipped job is the maturin suite over a
different directory. Removed rather than restated — a false gap in a
merged PR body is the same defect as false prose in a doc comment.

### One judgement taken against the dispatch, and accepted

The dispatch said to file the two `prose_census` findings as two items
in `work/issues/`. The lane filed **one**, arguing that neither half
alone catches the founding defect — widening the scan set leaves
`Dimension` a `Prose` verdict, and adding a bypass verdict leaves
`exists_wording` unread — so a lane taking one would ship a guard that
still passes over the case and reasonably believe the class closed. It
stated that as a decision and invited a split. **Accepted**: the
argument is right, and it is the same "instance versus shape"
distinction this program applies to code.

## `frame::progress`'s two swappable bools became one session value (2026-09-06)

`progress(busy, running, indexing)` is closed by
`session::Outstanding` — `Current | Evaluating | Canceled`, minted by
`DocSession::outstanding()` — with the chrome door now
`progress(Outstanding, bool)`.

The interesting part was not the type, it was **where the pair stops
existing**. A struct with `busy` and `running` fields types the
signature and moves the swap into the constructor; three named types
type the arguments and leave the caller assembling them in order. The
enum is the only shape where the two reads are never a pair a caller
holds: `outstanding()` consults each by name in an if/else chain, and
the eighth combination the old function was total over
(`!busy && running`, unreachable through a session) stops being
expressible rather than staying documented.

**The test was half the defect and got the other half of the fix.**
The `frame_policy` row repeated the same positional convention as
`app.rs`, so a swapped call site and a swapped test agreed; naming
states instead of positions leaves nothing to mirror, and the swap is
now a type error. But that row still says nothing about which SESSION
state produces which chrome state, and `app.rs` is `app`-gated and
untested, so the fold got coverage where it can execute without the
feature: `tests/eval_seam.rs`'s cancel row now walks a real
`DocSession` through `Evaluating` → `Canceled` → `Evaluating` →
`Current`, asserting `outstanding()` at each. That mapping had no
assertion anywhere before.

`busy()` and `running()` both stay: nineteen and five readers use them
singly, mostly as wait predicates. What is gone is any signature that
takes both.

The rule is in `crates/viewer/README.md`'s session paragraph, beside
`Landing` and `AtRestBadge` — the other values the session mints for a
vocabulary to consume, which is what settled where `Outstanding` lives
rather than in `frame` (a per-frame policy module already carrying
eight concerns).

## #2055's style review: the receipt was the finding, and it was wrong about its own file (2026-09-06)

The unit's thesis held and its best claim survived checking — the fold
is behaviour-preserving over all eight combinations, and `eval_seam.rs`
really does pin a mapping nothing asserted before. Two things did not,
and both are about **what the PR said**, not what the code does.

### A sweep receipt that a reader would have trusted

`frame::chooser_backend_of(zenity_on_path: bool, session_bus: bool)`
sits a hundred lines below `frame::progress` and is the same defect in
every part: adjacent differently-defined bools, a `match (a, b)` body,
a positional call site, a test row repeating the convention. The
receipt said the crate held no second instance. The `rg` behind it
**cannot match a multi-line signature**, which is the whole lesson: a
grep over a signature is a grep over one line of it.

The re-run parses every `fn` header's parameter list and reports each
adjacent identically-typed pair. Two `bool` hits in `crates/viewer/src`,
both now fixed — the second by naming the two readings (`Zenity`,
`SessionBus`) rather than folding them, because there is no third party
minting them; `ChooserBackend` is already the value that ranks them.
Taking it here rather than filing it is what makes the unit remove the
SHAPE from the crate instead of one instance of it.

**A wrong negative result is worse than none**, because it is the one
form of evidence that stops the next reader looking. That is the
sentence worth keeping from this round.

### A coverage regression disclosed as its opposite

The PR said the unreachable eighth combination "stops being expressible
rather than staying documented". It stops being expressible at
`progress`'s signature and moves into `outstanding()`'s first arm — and
the diff deleted the tree's only executable statement about it. Net:
documented and asserted → documented only, written up as the reverse.

Restored one level down, where it is a stronger row than the one lost:
a `NeverIdle` seam reports work while the picture is current, and the
session answers `Current`. The reason the state is unreachable is now
written as its two mechanisms — `request_eval` bumping the generation
on every submit, and both shipped seams handing a result up only with
nothing queued — instead of the restatement of its own conclusion that
stood there. That second mechanism is a property of two
implementations and not of `EvalService`, and `HeldEvaluator` already
departs from it, so it left with its own file.

### Fifty lines of prose out

One argument written five times, two near-verbatim, one of them
defending against a shape the tree no longer contains. The README is
now declared the home in its own text and the four other sites keep
invariant-plus-pointer; `frame::progress`'s new paragraph, which
described the signature printed beneath it, is deleted outright.

### The residue this time

Six items came back from the review. Four closed here; two stay open by
the brief's instruction. Two more were split out at close rather than
left in closing prose — the unenforced `EvalService` coalescing rule,
and the same README/type-doc double statement for `LandedRun`. The
first pass's "no residue" over three disclosed blind spots is what
produced most of that list.

## #2055 MERGED; a receipt is a claim, and this one was wrong about its own file (2026-09-06)

**#2055 merged at `471786d5c`.** Fifteen units on main this session.
CI on the conflict-resolved head `9c50a7b1a`: 37 jobs, twelve
`test (…)`, five `k-lint (gate, …)`, `gate ok` success, zero
non-green.

### The finding was the receipt, not the code

The unit's own thesis held under review and its best part was real: the
fold is behaviour-preserving across all eight combinations (checked
independently by the orchestrator and again by the reviewer), the 19/5
read counts are right, no reader changed, and `tests/eval_seam.rs`
genuinely drives a real `DocSession` through `Evaluating` → `Canceled`
→ `Evaluating` → `Current`, a session-to-chrome mapping asserted
nowhere in the tree before.

What was wrong was the **sweep receipt**. It reported one hit for the
shape; there were two, and the second —
`frame::chooser_backend_of(zenity_on_path: bool, session_bus: bool)` —
is a hundred lines below the function that was fixed, in the same file,
with the same `match (a, b)` body, the same positional call site, and
the same test row repeating the convention.

The cause is worth writing down in the form the lane found for it: **a
grep over a signature is a grep over one line of it.** `rg` cannot see
a multi-line `fn` header. The corrected method parses every `fn`
header under `crates/viewer/src` and splits its parameter list at
top-level commas, and it returns: two adjacent `bool` pairs, one
`bool` pair separated by an argument (judged out of scope, and
recorded), zero tuple-struct adjacencies, and forty-three same-typed
adjacencies of any type.

A receipt offered as evidence and wrong about its own file is worse
than no receipt, because a reader stops looking. That is the sentence
this program should keep.

### Both open calls were taken the hard way, and argued

**The second instance was fixed, not filed.** `Zenity`
(`OnPath | NotOnPath`) and `SessionBus`
(`Advertised | NotAdvertised`) are named types now, returned by the two
probe functions and taken by `chooser_backend_of`, so the bools are
gone from the whole chain. Two named types rather than the fold
`Outstanding` got, because no third party mints these — they are
independent environment probes and `ChooserBackend` already ranks them.
The argument for taking it rather than scheduling it: the unit then
removes the SHAPE from the crate instead of one instance, and a later
lane would have re-derived the same argument for a fifteen-line change
in a file this PR already edits.

**The false claim was struck and the assertion restored one level
down, stronger than the one lost.** The first pass said the unreachable
eighth combination *"stops being expressible rather than staying
documented"*; it stopped being expressible at `progress`'s signature
and moved into `outstanding()`'s first arm, while the diff deleted the
tree's only executable statement about it. Documented-and-asserted
became documented-only, written up as the reverse.
`a_current_picture_reads_current_even_when_the_seam_claims_work` now
hands `DocSession::new` a `NeverIdle` seam — an `InlineEvaluator` whose
`busy()` is always true — lands the first result, and asserts both that
`!busy() && running()` really holds and that `outstanding()` answers
`Current`. The old row asserted at `frame::progress`, which no longer
takes the pair.

The REASON was rewritten too: what stood there was a restatement of its
own conclusion, and the actual mechanisms are that `request_eval` bumps
the generation on every submit and that both shipped seams hand a
result up only with nothing queued. The doc now says plainly that the
second is a property of **the two implementations and not of
`EvalService`**, and that `HeldEvaluator` already departs from it —
which got its own file rather than a sentence.

### Two operational corrections

**The slow interval shard is not a fixed shard.** The measurement this
log recorded named `test (interval, eps = 1e-12, 1/2)`; on this run the
slow one was `2/2` (~9 min) while `1/2` finished in 75s. nextest's
partitioning moves the heavy tests between runs, so the durable fact is
that ONE interval shard carries most of the tier's work, not that a
particular one does. Anything reading a specific shard name as a
baseline is reading it wrong.

**A merge conflict is work now, not a bounce.** #2055 came back
mergeable-false because #2053 had landed under it and both had appended
to `work/view/log.md`. Resolved in the lane's worktree by keeping both
entries in chronological order rather than sending it back — the lane
had reported and the conflict was in a file whose merges are
append-and-order, not a semantic one.

### Residue

Four reviewer items closed here; two left open by instruction (the
class one type away, and the two three-state enums a hop apart, with
two of the class item's own blind spots now checked and recorded on
it); **two split out at close rather than left in closing prose** —
`evalservice-coalescing-rule-is-prose-no-implementor-is-held-to` and
`readme-and-type-docs-restate-one-argument-for-landing-and-landedrun`,
the second checked and found true of `LandedRun` and NOT of
`AtRestBadge`, which is the kind of qualification a claim like that
usually loses.

Five adjacent-`bool` signatures outside this program's fence were
reported and not filed (implementer-discipline §6): two in `sweep`, two
in `topo`, one in `step-export`. That last is
`composed_direction(bound_orientation, oriented_edge_flag)` in an
**oracle** — where a transposition that agrees with the code under test
is the worst case there is, because the thing meant to confirm the
implementation independently would confirm the same mistake.

The first pass's "no residue at all" over three disclosed blind spots
is what produced most of that list. `work/README.md` is explicit that
disclosing a residue is not scheduling it, and this is the clearest
instance of that rule paying for itself.

## The `evalseam`/`pick` cycle, broken by two moves (2026-09-06)

Ev ruled (d) on #2076 and the lane landed it: `Generation` to a leaf,
`pick.rs` split at its layer boundary. Both, because neither breaks the
cycle alone — `Generation` alone leaves `PickIndex` beside `PickCache`,
the split alone leaves `pickindex` reaching into `evalseam` for the
counter. The chain is `generation ← pickindex ← evalseam ← pick`, and
`evalseam` keeps both seams and therefore both sets of threads, which
is the property that made this shape beat a third seam module.

### The siting call, and why `scene` lost it

The ruling offered `Generation` its own module or a seat beside
`DisplayTolerance` in `scene`. The lane took its own module. The
argument that decided it is that the two candidates pass DIFFERENT
tests: `scene` imports nothing from this crate, which is what the
analysis measured, but `generation.rs` imports nothing at all — its
`use crate::` count is zero and its kernel count is zero — and that is
the property a leaf is supposed to have. `scene`'s written charter
covers what the viewport draws at a δ, so a request counter would have
been a second concept in a module this program is already trying to
keep to one.

### The boundary was verified, not trusted

The dispatch said to check the split the analysis proposed rather than
take it. It held exactly: over the policy half the only names reaching
into the index half are `PickIndex` and `PickIndexError`, and over the
index half the only name reaching forward is a doc mention of
`IndexInputs` in the header sentence that belongs to the policy. The
in-file `mod tests` belongs to `PartWindows`/`IdMap` and travelled with
them, unedited.

### The move discipline held, and the count says so

501 passed / 1 ignored on `--test all` and 24 passed / 1 failed on
`--lib` (the Vulkan-less `gpu::every_pass_builds_on_a_real_device`,
expected off hardware), before and after, identical. No assertion was
touched; every test-side change is an import or a path. No `pub use`
shim was left, so `session-shims-and-test-imports` is no larger.

### The tracker pass was the expensive half of the sweep

A tree-grep re-points source; it does not see the twelve tracker rows
citing `crates/viewer/src/pick.rs:NNNN`. Those went onto
`stale-file-citations-after-the-split`, which already owns the class
from the 1c split — VIEW's five live rows corrected in the same PR, six
rows under `work/chrome/` and `work/code-quality/` announced rather
than edited, and the closed VIEW rows left as written. **The new
member's contribution to that item is a negative result worth having**:
the 1c entry is remembered for the row where the claim, not the number,
went stale, and this split produced none of those — a move that changes
no behaviour cannot falsify a sentence about behaviour. So the item's
two halves, "resolve the number" and "check the claim", come apart
cleanly here, and a `<file>.rs:<line>` gate would have caught all of
this member and none of the previous one.

## #2079's style review: the move survived, the prose did not (2026-09-06)

The reviewer could not break the split and said so with a measurement
worth keeping: a whitespace-sensitive sorted-line diff of the merge
base's `pick.rs` against `pick.rs + pickindex.rs` is **one line removed
and 43 added**, all doc-header and import lines; no visibility,
signature, `derive`, field or `impl` changed; `#[test]` 531 both sides;
`Generation` byte-identical. That is the strongest form the "it is a
move" claim has had in this program, and it is stronger than the claim
the lane itself made — the boundary has ZERO back-references, not the
one the PR conceded.

Nine items came back. Six are closed by the fix pass, three stay open
(the second `pickindex` split, `Generation::get`'s missing reader, and
the naming question below). What follows is the two that generalise.

### A pure move cannot falsify a sentence about behaviour, and reliably falsifies one about structure

The lane's negative result was that this split produced no stale
CLAIMS, only stale numbers, *"because a move that changes no behaviour
cannot falsify a sentence about behaviour"*. True premise, false
conclusion, and the reviewer produced three counterexamples — including
`crates/viewer/README.md:323`, whose `session::op` row listed `pick` as
a `SessionOp` reader when `pick.rs` has zero hits and `pickindex.rs`
seven, **in a file the lane re-read and corrected two other rows of**.

A tracker is not mostly sentences about behaviour. It is mostly
sentences about where things are and what names what, which is exactly
what a move falsifies. The corrected statement is on
`stale-file-citations-after-the-split`, and it moves that item's two
halves onto a better axis: not *numbers versus claims* but **what a
machine can reach**. A `<file>.rs:<line>` gate resolves every numeric
citation in this member and catches none of the three, the README row
least of all — it names a module in prose and cites nothing.

**The lesson for this program's move-shaped units** is that the
discipline aimed at behaviour is aimed away from the failure a move
actually causes.

### A grep that cannot run reads exactly like a grep that found nothing

The sweep missed two live VIEW rows that its own disclosed pattern
matched. The PR blamed a pattern gap; the real cause was worse and has
two halves: the first pass's `grep` over `work/view/*.md` was
**malformed** — an unescaped `|` printed `command not found` above
output the lane read as a result — and the disposition step then acted
only on the `pick.rs:NNNN` hits, leaving bare paths and
`module::symbol` citations matched but unhandled.

So the honest statement is not "the pattern was too narrow". It is
**the pattern was right and the reading of it was not**, which is the
harder failure to catch because the output looks like a clean negative.
The re-sweep ran three greps and checked each hit's status before
deciding; it found a third out-of-fence row
(`work/fix/error-types-with-no-display-class.md`) the first pass had
missed.

### The naming question, declined with a reason

`pick.rs` claimed to be *"the policy half of picking"* and is not — the
policy (`op_for`, `hovered_for`, the priority and miss rules) is in
`pickindex`; `pick.rs` is a cache. The false sentence is fixed and both
headers now say the real boundary, which is **stateful against pure**
and is a better boundary than the one the first draft claimed.

The rename is declined and the row parked behind the second split. A
rename is not a move: it would rewrite call sites across twenty-nine
files and destroy the sorted-line diff that made this unit certifiable,
to reach names that are not knowable until
`pickindex-holds-the-frames-marks-as-well-as-the-index` is decided —
and `pick` → `pickcache` would strand `NotIndexed`/`unindexed` in a
module named for a cache anyway.

## #2079 MERGED; Ev's (d) landed, and my merge criterion was the wrong shape (2026-09-06)

**#2079 merged at `116757bd3`.** Sixteen units on main this session.
CI: 37 jobs, twelve `test (…)`, five `k-lint (gate, …)`, `gate ok`
success, zero non-green.

The cycle Ev ruled on is gone, verified from the imports rather than
the receipt:

    generation.rs   → (nothing at all)
    pickindex.rs    → camera, display, generation, input, scene, session
    evalseam.rs     → generation, pickindex, scene
    pick.rs         → evalseam, generation, input, pickindex, scene

`evalseam` keeps BOTH seams and both sets of threads, which is the
property that made (d) win over a third seam module.

### The lane went against the dispatch twice, and was right both times

**On `Generation`'s siting.** The brief offered its own module or
beside `DisplayTolerance` in `scene`. The lane took its own module on
the observation that the two candidates pass DIFFERENT tests: `scene`
imports nothing *from this crate* — which is the property the
orchestrator's costing measured — but `generation.rs` imports nothing
*at all*, and `scene` names `bvh`, `pncad::mesh` and ~20 kernel types.
Add that none of `Generation`'s readers is `scene`, so siting it there
makes each of them import a display module to name a counter. **The
costing had measured the wrong property**, and the lane found the right
one.

**On the rename (F6).** The style review established that `pick.rs`
claimed to be "the policy half of picking" and is not — the policy is
in `pickindex`, `pick.rs` is a cache, and the real boundary landed is
stateful-against-pure, which is *better* than the one claimed. The
false sentence was owed and is fixed. The rename was declined, and the
first of its three arguments is the one to keep: **a rename is not a
move, and this PR's entire warrant is that it is one** — it would
rewrite call sites across twenty-nine files and destroy the
sorted-line diff that let the reviewer certify it, replacing a diff a
reviewer can check mechanically with one they cannot. Also: the right
names are not knowable until
`pickindex-holds-the-frames-marks-as-well-as-the-index`'s ~405 lines
come off, and even the easy half is unclean (`pick` → `pickcache`
strands `NotIndexed`/`unindexed`, a refusal vocabulary, in a module
named for a cache). The item stays open as the schedule, parked behind
the second split.

### My merge criterion was an absolute where the invariant is relative

The fix brief said the test counts "must still be 24/1 and 501/0/1".
They came back **24/1 and 503/0/1**, and the +2 was not the lane's:
merging `origin/main` brought two new rows into
`crates/viewer/tests/mate_tool_flow.rs`. **The invariant that matters
is against the branch's own base**, and `#[test]` over
`crates/viewer/{src,tests}` is 533 on `origin/main` and 533 on the
branch — checked here, not taken on report.

Pinning an absolute count in a brief is a defect of the same family
this program has been fixing all day: a number that goes stale the
moment anything else lands, where a stated TEST would not. A brief that
says "the count must not move against your own merge base" survives a
concurrent merge; one that names 501 does not.

### The class bit inside the fix pass that was correcting it

Rewriting the two module headers moved every import below them, so two
of the three line numbers the reviewer cited for the surviving ring
went stale **in the same commit that acted on them** — and had already
been published in the PR body. The lane caught it by re-running the
receipt instead of trusting it. Fourth unchecked citation this program
has paid for today, and the first caught by its own author before a
reader found it.

### The review's own best finding

The lane's negative result — that a move producing no behaviour change
produces no stale claims — had a true premise and a false conclusion.
The corrected statement is worth more than the original and is now the
parent item's: **a behaviour-preserving move cannot falsify a sentence
about behaviour and reliably falsifies one about STRUCTURE — the half
such a unit is least primed to look for, because its whole discipline
is aimed at the other one.** Three sentences had gone stale that way,
including `crates/viewer/README.md`'s `session::op` row still naming
`pick` as a `SessionOp` reader with zero hits there and seven in
`pickindex`.

That also re-cut `stale-file-citations-after-the-split`'s two halves
onto **what a machine can reach** rather than numbers-versus-claims: a
`file:line` gate resolves every numeric citation in this member and
catches none of the three.

### Residue

Nine items filed by the review, six closed in the fix pass, three left
open: `seam-split-leaves-a-cycle-through-the-session` (the ring through
the driver's vocabulary, pre-existing and deliberately held open by the
`IndexInputs` hoist), `pickindex-holds-the-frames-marks-as-well-as-the-
index` (the named ~405-line second boundary), and
`generation-get-has-no-reader` — the last untouched on purpose, because
deleting a public item inside a move is exactly what
`implementer-discipline` §3 forbids.

The tracker re-sweep turned up a third out-of-fence row the first pass
missed (`work/fix/error-types-with-no-display-class.md:73,87`), and the
honest blind-spot statement is worse than "a pattern gap": the first
pass's grep over `work/view/*.md` was **malformed** — an unescaped `|`
printing `command not found` above output that was read as a result.
The pattern was right and the reading of it was not.

## The marks come off `pickindex`, and the naming question answers itself (2026-09-06)

Two rows, taken together because the naming one was parked behind the
split: `pickindex-holds-the-frames-marks-as-well-as-the-index` and
`pick-and-pickindex-are-named-against-their-contents`. Both closed. Two
commits, deliberately separate — a rename is not a move, which is
argument 1 of #2079's decline and the one that does not expire.

**Commit 1, the move.** All eleven named members —  `Highlight`,
`highlight`, `EdgeOverlay`, `edge_overlay`, `edge_segments`,
`edge_id_segments`, `segments_of`, `focus`, `marked_for`, `drives`,
`cursor_projection` — into `crates/viewer/src/marks.rs`. Nothing failed
the three properties and nothing outside the range passed them; the one
judgement call was `cursor_projection`, which takes no index at all and
went because its consumers are the marks' and its subject is the id
pass. Certified the way #2079 was: merge base `pickindex.rs` against
head `pickindex.rs + marks.rs`, sorted and whitespace-sensitive, is **34
removed and 75 added with no code line among them** — every removed line
is a header line, a doc line the `Camera::project` link-fix reflowed, or
a `use` whose name list shrank — and the 144 declarations are identical.
`#[test]` 533 → 533 against this lane's own merge base; `--test all` 503
passed / 1 ignored on both sides, `--lib` 24 / 1 on both.

**Commit 2, the naming, and the answer is not the one the parking
predicted.** #2079 parked the rename on the split with the expectation
that *"what is left is genuinely the index and the cursor questions over
it, at which point `pickindex` may be the right name after all"*. Half
right. The marks coming off did NOT narrow `pickindex` to the index —
`op_for`, `op_under`, the miss rule and the priority rule are untouched
by the move, so the original finding survives word for word. What the
split makes visible is that **the complaint was a complaint about a type
having methods**: after it, every item left is `PickIndex`, one of its
keys, its construction machinery, its errors, its answers, the two
constants its queries take, or a private helper for one of its methods,
and the policy is inherent `impl` on the type the module is named for.
Before the move six items were none of those. So `pickindex` is right,
and this is the first point at which that could be said on evidence.

`pick` is not, and nothing above changes that half. It is a squatter on
a name whose subject belongs to a module that already has a better one,
so the answer is a straight rename with no successor rather than the
swap the contents superficially argue for — a swap would put
`PickIndex::scene_focused`, the drawable-scene builder, in a module
called `pick`, which is the same false sentence pointed the other way.
`pick.rs` → `pickcache.rs`.

**The `NotIndexed` objection dissolves on the contents**, which is why
the brief was right to hold it as a real constraint and why it does not
bind. Argument 3 read `refusal` and `cache` as different subjects.
`NotIndexed::Building` is defined as `PickCache::indexing` and
`::Absent` as *"the last attempt refused (its reason is
`PickCache::error`)"*; `unindexed`'s doc names `PickCache::indexing` as
*"the one value that knows"*. The refusal exists because the cache can
be empty and is computed from nothing but the cache's state.

The rule the three now follow, stated once: each module is named for the
type it is built around (`pickindex`/`PickIndex`,
`pickcache`/`PickCache`), and `marks` — which has no spine type — for
what it produces.

### The sweep, and what it could not match

Three patterns over `work/**/*.md`, `docs/` and the tree: the bare paths
`pick.rs`/`pickindex.rs`, the module spellings `pick::`/`pickindex::`
(as a Rust path segment, so `pickindex`, `pick_face` and `PickCache` do
not false-positive), and each moved symbol name. **The greps' exit
status was checked**, which is the #2079 lesson. What they cannot match:
a multi-line `fn` header, since `rg` reads one line; and a citation that
names a subject without naming the file, which no pattern reaches.

The move's own header rewrite shifted every line in `pickindex.rs` by
−18 and the rename's by −2 in `pickcache.rs`, so four open VIEW rows
whose numbers moved were refreshed (`adjacent-same-typed-arguments`,
`ui-thread-work-after-the-index-seam`, `the-picture-key-never-became-a-
type`, `focus-marking-is-per-node-not-per-segment`, whose whole subject
moved to `marks::focus`) and six more re-pointed for the rename. **Three
of those numbers were already wrong at this lane's merge base** —
`:405`/`:442`, `:756` and `:928` against real values of `:440`/`:477`,
`:791` and `:912` — invalidated by #2079's own fix-pass header rewrites
in the same commit that published them, which is the failure that pass
recorded and then committed. Every number in this member was re-read
after the last edit.

### Residue

One item filed, outside VIEW's fence and therefore disclosed rather than
fixed: `renamed-module-leaves-citations-in-three-other-programs` — four
open rows on CHROME's and code-quality's slates cite
`crates/viewer/src/pick.rs`, and three of the four were already citing
the wrong file before this rename. `generation-get-has-no-reader` stays
untouched, for the same reason as last time.

## The fix pass for #2083, which committed the class it caught (2026-09-06)

**Say it first, because it is the entry a successor needs: this unit
named #2079 for shifting a line number instead of re-deriving it, and
then did exactly that, in the same section that names it.**
`ui-thread-work-after-the-index-seam:31` was written `pickindex.rs:910`
— which is `:928 − 18`, the number the PR's own prose had just declared
wrong, moved by a delta computed correctly somewhere else. The PR also
claimed *"the numbers here were all re-read after the last edit"*; a
re-read of `:910` returns `mesh: part.mesh(),`. The claim was false and
the check it described was not performed.

The correct citation is two citations, and the single parenthetical was
hiding that: `PickIndex::scene_focused` is `pickindex.rs:894` and
`SceneMesh::build_parts_focused` is `scene.rs:410` — a different file,
which it has been all along.

**Why the class survives being named.** Naming it produces vigilance
about OTHER people's numbers and none about one's own, because a delta
felt like a derivation. It is not: a delta is only as good as the
origin it is applied to, and the origin here was a number the same
paragraph called wrong. The instrument that works is not a grep and
never was — it is to enumerate every `file:line` in every row the
branch touches, `sed -n Np` each, and read whether the subject is
there. Twenty-odd citations, a few minutes, and it is the only thing
that catches this.

Doing that turned up a sixth instance nobody had named: **the header
rewrites in this very fix pass moved five of the review's own fresh
citations** — in `cursor-projection-landed-in-marks-for-want-of-a-home`,
`highlight-and-edge-overlay-disagree-on-hover-equals-selected`,
`the-point3-to-gpu-corner-cast-is-at-three-sites` and
`a-module-named-for-its-spine-type-is-unfalsifiable`. Re-derived by
subject and verified one at a time. That is the third split in a row to
feed `stale-file-citations-after-the-split`, and the row now says so.

### The other counts, all four wrong the same way

Four rows were merge-base-wrong, not three (`outstanding-and-progress:49`
was fixed and not counted). Four of the four out-of-fence rows were
already wrong, not three — the item said so and the PR body contradicted
its own item. The receipt sentence said *"merge base against head"* when
34/75 is true of the move commit `6702d15` alone; at head it is 35/76
after the rename and 35/90 after this pass, because prose in those
files keeps moving. Both head figures were MEASURED — a draft of the
closure wrote one of them by estimate ("38/89") one paragraph after
confessing the estimating habit, and it was caught by re-measuring
before the commit rather than by noticing.
The number was always right about the commit whose warrant it is, and
naming the tree instead is the same error as the line numbers: a true
measurement, quoted against the wrong subject. And the residue row's id
said three programs where there are two — ids are stable for life, so
it is renamed now rather than never:
`renamed-module-leaves-citations-in-three-other-programs` →
`renamed-module-leaves-citations-in-two-other-programs`. Every citation
of the old id is re-pointed except the previous log entry, which is
append-only.

### The header this unit wrote from scratch

#2079's lesson was that a split goes wrong in its header, and this unit
wrote `marks.rs`'s from scratch and put three universals in it that the
module falsifies: *"every door here takes a `PickIndex`"* (false for
`cursor_projection`, which the PR's own member table records as taking
none), a section titled *"The four marks"* counting a projection matrix
as a mark, and *"`Theme::marks` names the same four"* when the two lists
share one name. Rewritten: three marks in the opening section, scoped
to those three; `cursor_projection` given its own section saying
plainly that it is not a mark and is here for TESTABILITY rather than
subject, pointing at the item that argues `camera` is its home; and the
theme passage now says the two enumerate different lists that cannot be
lined up, and why. A per-door tally in the first draft of that fix was
itself removed — it was an inference over `gpu.rs`'s uniform block
rather than something either module states, and this was not the item to
over-claim on.

**`cursor_projection` is not moved.** The reviewer's argument that
`camera` is its home is good, and moving it would be a third move in a
PR whose warrant is two clean ones. Filed, and the header now stops
calling it a mark, which is what makes the item honest rather than a nag.

### The naming rule is withdrawn as a rule

*"A module named for the type whose inherent `impl` is its spine is
named correctly"* cannot fail for any module built on one big type and
would have certified `session.rs` before the 1c split. The rename is
still right; the argument for it is not that rule but **the six items
that left** — `Highlight`, `EdgeOverlay`, `highlight`, `edge_overlay`,
`focus`, `cursor_projection`, none of which was `PickIndex` or about
it. The closure and `pickcache.rs`'s header both now say the shared
property is a description of two modules rather than a rule that
decided a name, and point at
`a-module-named-for-its-spine-type-is-unfalsifiable`, which also
carries the structural symptom the closure does not resolve:
`op_for`/`op_under` return `SessionOp`, so `pickindex` imports
`crate::session` and that import is a leg of a live ring.

### The sweep that was described but not run

The PR disclosed its first pattern as *"the bare paths
`pick.rs`/`pickindex.rs`"*. It was run over `work/` and `docs/` and not
over `crates/`, and the body did not say so — which is how
`tests/pick_windows.rs:297` survived inside the pattern's own reach,
and `README.md:672` survived four lines below two sites the PR lists as
fixed. Both fixed. The bare-word instrument was then run properly and
found two more: `session.rs:1104` and `generation.rs:10`. One hit was
triaged as not the module (`input.rs:338` names `InputMap::pick`,
fourteen lines below it) and two as history. The hit list and its
disposition are in the PR body, per `implementer-discipline` §5, which
the first pass owed and did not pay.

Also fixed: the closure enumerated `PickKinds` as one of *"the two
constants"* — it is a `pub enum`, and the file holds three constants,
one public.

### Residue

Four review items stay open and are not this pass's:
`cursor-projection-landed-in-marks-for-want-of-a-home`,
`a-module-named-for-its-spine-type-is-unfalsifiable`,
`the-point3-to-gpu-corner-cast-is-at-three-sites` and
`highlight-and-edge-overlay-disagree-on-hover-equals-selected` — the
last two pre-existing. `generation-get-has-no-reader` still untouched.

## #2083 MERGED; the citation class caught by a method, not a grep (2026-09-06)

**#2083 merged at `62a51835a`.** Seventeen units on main this session.
CI: 37 jobs, twelve `test (…)`, five `k-lint (gate, …)`, `gate ok`
success, zero non-green. `#[test]` 533 at merge base `2beeb0b` and 533
at head — checked here, against the branch's own base.

Two commits, deliberately separate: the eleven marks lifted into
`marks.rs`, then `pick.rs` renamed to `pickcache.rs`. The move's receipt
(34 removed / 75 added at `6702d15`, empty set after filtering doc
comments, `use` and blanks) was re-derived by the reviewer, and the
reviewer added a check the dispatch had not asked for: `pickindex`'s
`mod tests` references none of the moved members, so the split orphaned
no coverage.

### The naming answer overturned what the previous unit predicted

#2079's lane parked the rename on the theory that the marks coming off
would narrow `pickindex` to "the index". **It did not** — `op_for`,
`op_under`, the miss rule and the priority rule are all still there.
What the split exposed is that the complaint was about **a type having
methods**: everything left is `PickIndex`, its keys, machinery, errors,
answers or a private helper, and the "policy" is inherent `impl` on the
type the module is named for. So `pickindex` was right and `pick` was
not, and the answer is a straight rename rather than the swap the
earlier framing implied — *a swap would put `PickIndex::scene_focused`
in a module called `pick`*.

The `NotIndexed`/`unindexed` objection, which #2079's lane had named as
the reason even the easy half was unclean, **dissolves**: `Building`
IS `PickCache::indexing`, `Absent` is defined by `PickCache::error`,
and `unindexed`'s doc names `PickCache::indexing` as "the one value
that knows". The refusal exists because the cache can be empty.

**The rule the closure first rested on was withdrawn**, correctly: *"a
module named for the type whose inherent `impl` is its spine is named
correctly"* cannot fail for any module built on one big type — it would
have certified `session.rs` before the 1c split. The rename stands on
the six items that left, not on a rule that cannot fail.
`a-module-named-for-its-spine-type-is-unfalsifiable` carries it.

### The citation class, and the method that finally caught it

**#2083 caught #2079 for shifting a line number instead of re-deriving
it, and then did the same thing.** It wrote `:910` where its own prose
had just called `:928` wrong — `:928 − 18`, the wrong number moved by a
delta computed correctly somewhere else. Its claim that "the numbers
here were all re-read" was false; a re-read of `:910` returns
`mesh: part.mesh(),`.

Re-deriving it **split the citation in two**, because the single
parenthetical had been hiding two subjects in two files since before
the branch: `PickIndex::scene_focused` at `pickindex.rs:894` and
`SceneMesh::build_parts_focused` at `scene.rs:410`. Both verified here
by reading the lines.

**The method, and it should be program practice**: enumerate every
`file:line` in every row the branch touches, `sed -n Np` each one, and
read whether the subject is there. **39 citations, 39 verified.** No
grep finds this class — a citation pointing at the wrong line still
parses — which is why five instances got through today before anyone
looked properly.

It then caught two more nobody had named:

- **its own header rewrites, in this fix pass, invalidated five of the
  reviewer's fresh citations** — the same failure #2079's fix pass
  committed, caught this time before publication;
- it had written the post-fix receipt as "38/89" **by projection**;
  measured, it is 35/90.

Four counts were wrong and are corrected: four merge-base-wrong rows
(not three), four of four out-of-fence rows already wrong (not three),
the receipt now names the commit `6702d15` rather than the tree, and
the filed item is `renamed-module-leaves-citations-in-TWO-other-
programs` (chrome ×3, code-quality ×1), renamed from "three". **The
orchestrator propagated the original three-and-three into a check-in
before the fix pass corrected them** — repeating a lane's arithmetic
without checking it, which is the same defect one level up.

One disclosure moved to where it belongs: `new-document-owes-the-
reframe-open-gets:54` carries a range the lane could not re-derive, and
now says "range unverified" **in the item** rather than only in a PR
body, which `work/README.md` says is not a slate.

### The header this unit wrote from scratch

`marks.rs`'s first draft opened with three universals its own module
falsified: *"Every door here takes a `PickIndex` as an ARGUMENT"*
(false for `cursor_projection`, which the PR's own table records as
taking none), a section titled *"The four marks"* counting that matrix
as a mark, and a claim that `Theme::marks` names "the same four" when
it names a different four sharing only `focus`. All three gone;
`cursor_projection` now has its own section saying it *is not a mark,
and is here for want of a home*.

The lane also cut a per-door tally from its own fix draft on the
grounds that it was an inference over `gpu.rs`'s uniform block and
"this was not the item to over-claim on" — which is the right instinct
in a fix pass about over-claiming.

### The rename sweep had been described but not run over `crates/`

Confirmed: `work/` and `docs/` only. Two live sites named by the
reviewer plus **two more** the bare-word instrument found
(`session.rs:1104`, `generation.rs:10`); three triaged and left with
reasons. The full tree hit list is in the PR body, which
`reviewer-style-lane` §5 asks for and the first pass had not given.

### Left open

`cursor-projection-landed-in-marks-for-want-of-a-home` — the reviewer
argues `camera` is its home (no index, no selection, no document, one
production consumer, and its stated reason for living in `marks` is
testability rather than subject; its doc composes with
`Camera::project`, the very link the move had to widen to a full path).
**I agree**, and it stays filed: moving it would be a third move in a
PR whose warrant is two clean ones, and it needs its own commit and its
own re-verification. The header rewrite above is what keeps the item
honest rather than a nag.

Also open: `a-module-named-for-its-spine-type-is-unfalsifiable`, and
the two pre-existing prose findings the review filed (three
`Point3<f64>` cast sites against a doc sentence naming two;
`highlight`/`edge_overlay` called "twins" while disagreeing on whether
a hover that IS the selection is reported as hovered).

## Two things put where they belong: `cursor_projection` home, `Generation::get` gone (2026-09-06)

The two "this is not where it belongs" rows from #2079's and #2083's
reviews, in two commits because they are two different kinds of change
and only one of them is certifiable by a sorted-line diff.

**`cursor_projection` → `camera`, and the home was verified before the
move rather than after.** The item PROPOSED `camera`; four checks made
it a finding rather than a guess. `camera` already builds the matrix
the function transforms (`view_projection`) and already answers in the
frame its `cursor_ndc` is in (`project`, `ray_through`), so the subject
is the module's. The signature is three arrays of `f32` and nothing
else, so `camera` gains no import and its module kind is unchanged.
`camera` already holds free `pub fn`s (`apply`, `fold`,
`fold_recorded`), so a function outside `impl Camera` is the file's
existing shape. And the doc link NARROWED —
`[\`crate::camera::Camera::project\`]` back to `[\`Camera::project\`]`
— which is #2083's tell running in reverse.

**No shim, and no test-side re-pointing needed.** Every suite already
spelled it `viewer::cursor_projection`; that crate-root re-export moved
from the `pub use marks::{…}` list to `pub use camera::{…}`, so there
stays exactly ONE path to the function. `session-shims-and-test-imports`
is open because unit 1c left two spellings of every moved path; this
left one. `gpu.rs` is the only production consumer and imports
`crate::camera::cursor_projection`.

**`Generation::get` deleted, and the precedent it looked like does not
reach it.** `tool-kind-all-and-ordinal-have-no-production-reader` kept
a `pub` item and documented the suites as its only readers — the item
was preserved BECAUSE it had a reader. `get` has none anywhere: not
`src`, not `tests`, not `examples`. What survives the deletion is the
need its doc named: `Generation` derives `Debug`, so a log line and a
debugger still show the counter, and the accessor was a second door
onto what `Debug` already opens. `next`'s twelve-line ceiling paragraph
lost the part describing a state nothing can construct and kept the
part that is the reason for `saturating_add` over `+`.

**The tracker pass found eighteen stale citations, and eleven of them
predate this branch.** Re-derived, never shifted, every one verified by
reading the line it names. **Seven this branch broke**: the six
`marks.rs` citations in three open rows (`:132-135`, `:286`, `:200`,
`:224-227`, `:118-121`, `:261`) all move by −13, the header section
this commit deleted, and `camera.rs:908` moves to `:954` under the
insert. **Eleven that were already wrong on `origin/main`** and would
have gone on being wrong: `session-shims`'s `lib.rs:147` (`:163` on
base, `:160` on head, so this branch moved a line that was already
mis-cited), `two-datumkind`'s `lib.rs:124` (`:128`) and `forms.rs:55`
(`:93`), `session-shims`'s `combine_ops.rs:1327` (`:1340`), and seven
in `adjacent-same-typed-arguments-are-the-same-swap` — five
`session.rs` citations all off by exactly four, `session/refuse.rs:308`
(`:323`) and `frame.rs:1658` (`:1689`). Six of those seven are the
brief's own warning working: they cite MULTI-LINE `fn` headers, which
is exactly what a grep over a signature cannot re-find.

That is the lesson worth keeping. The instrument was sold as a check on
what your own diff shifted; run over every row a branch touches it
finds more drift than the branch caused, because a row that has sat
open through a few refactors accumulates it silently and nothing else
reads those numbers. Eleven of eighteen here.

One more, `work/tcost/loud-skip-marker-is-a-hand-kept-idiom`'s
`lib.rs:103` and `lib.rs:92-100`, is outside this program's fence —
the symbol it names, `app_lane_skipped_no_chrome_or_gpu_coverage_here`,
is not in `crates/viewer/src/lib.rs` under any line number today — so
it is in the PR body and this report rather than edited here.

## The fix pass for #2089: the receipt was wrong about itself, twice in a row (2026-09-06)

The entry above is left as written — it is the record of what this lane
believed when it wrote it — and this one corrects it. Nothing in the
code was wrong; every claim corrected here is prose or arithmetic in
the durable record, which is the part a later reader trusts.

### "There stays exactly ONE path to the function" is false

`crates/viewer/src/lib.rs:55` is `pub mod camera;`, so
`viewer::camera::cursor_projection` resolves beside
`viewer::cursor_projection` — and both spellings existed through
`marks` before the move. **What the move preserved is the COUNT of
public paths, not a uniqueness that never held.** The contrast the
entry above drew against `session-shims-and-test-imports` therefore
does not land as written: that row's hazard is a `pub use` shim INSIDE
a module, which is a lie about where an item lives, not a crate-root
re-export, which is a convenience. Every item in `lib.rs`'s `pub use`
blocks has two public paths for the same reason, which is the class the
reviewer filed as
`every-crate-root-reexport-is-a-second-path-not-the-only-one`. The
sentence is corrected in the PR body and in the item's `## Closed`.

### Every summary number in the citation receipt was wrong

This is the second consecutive PR of this program's for which that is
true — `citation-repoint-shifted-a-number-the-lane-knew-was-wrong`
(#2083, closed) is the class row, and
`the-citation-receipts-summary-numbers-are-not-re-derivable` is the new
one. **Saying it plainly: the per-citation work was right both times
and the summaries wrapped around it were wrong both times**, which is
the more dangerous half, because a summary is what a reader quotes.

- **"60 citations checked" had no enumeration rule behind it.** The
  rule, stated so a second reader can re-derive it: a `path.ext:N`
  regex, one hit per match, a `:a-b` span counting once and a `:a,:b`
  comma-list twice, over the eight VIEW item files this branch's own
  commits edit. At `abf518285` that gives **51** — the reviewer's
  number, reproduced here exactly. At this fix pass's tip it gives
  **57** over those eight, **64** counting the `work/issues/` row filed
  below. A count without its rule is not a receipt.
- **"The six `marks.rs` citations all move by −13" — five do.** The
  sixth, `the-point3-to-gpu-corner-cast-is-at-three-sites`'s
  `:132-135`, became `:119-121`: a three-line span, not a shifted
  four-line one, because `marks.rs:135` on `bc44531e1` was
  `#[derive(…)]` and was never part of the quoted sentence. So that
  citation was **already** wrong on `main` and was filed in the wrong
  bucket. **The split is 7 + 12, not 7 + 11.**
- **"Twelve lines argued the ceiling … six instead of ten."** Nothing
  in either tree is twelve, and the comparison was a paragraph against
  a block. Re-derived: `next`'s rustdoc block is 10 lines on
  `bc44531e1` (`generation.rs:33-42`) and 8 on head (`:36-43`); the
  ceiling paragraph inside it is 8 (`:35-42`) and 6 (`:38-43`).
- **"Two public items" — three**, all with readers: `Generation`,
  `Generation::FIRST` (25 occurrences of that spelling tree-wide) and
  `Generation::next` (`session.rs:1814`, `review_gui2_r2.rs:1563`).

### The 51/51 was the joke, and the paragraph is now two lines

`generation.rs` was 51 lines on base and 51 on head: the close deleted
four lines of code and added a six-line paragraph about their absence,
in a unit whose finding was that a reader-less `pub` item is a third of
a 51-line leaf module's API — and then quoted 51 as an outcome. The
paragraph does earn a place, because the next reader reaching for the
counter needs to know `Debug` is the door and would otherwise re-add
the accessor; it earns **two lines** (`generation.rs:27-28`), not six.
The module is now **47**.

### The subject check in the `camera` argument did not hold as stated

Three of the four checks were exact. The fourth said `camera` "already
answers in the frame its `cursor_ndc` is in", and it does not: the
doors do not meet at the type — `f64` matrices and pixels with `+y`
down against an `f32` NDC function — and the conversion happens two
modules away in a driver. **The move is still right**; the other three
checks carry it, and the module header no longer claims otherwise.
`cursor-projection-is-f32-in-a-module-whose-matrices-are-f64` holds the
type question. The placement prose also did not shrink in the first
pass (19 lines before, 19 after); it is 14 now.

### §6 has now produced nothing twice on the same row, so it is a file

The stale TCOST loud-skip citations were reported out-of-fence in the
PR body — and #1848's close, on this program's own slate, already
reported the SAME row the same way on 2026-09-04, with the same
nothing resulting. A merged PR body is not a slate, and a disclosure
inside a closed item's prose is invisible to the re-homing sweep by
`work/README.md`'s own account, so the second lane could not see the
first. Filed as
`work/issues/loud-skip-marker-row-cites-a-lib-paragraph-that-was-reversed`,
which also records that one of the row's citations is not stale but
REVERSED: VIEW fixed that marker at #1848, so `lib.rs` now says the
opposite of what the row quotes it as saying.

## #2089 MERGED; a unit that closed net-zero, and a §6 report that failed twice (2026-09-06)

**#2089 merged at `aa4b64a17`.** Eighteen units on main this session.
CI: 37 jobs, twelve `test (…)`, five `k-lint (gate, …)`, `gate ok`
success, zero non-green. `#[test]` 534 at merge base `4887d865c` and
534 at head — checked here, against the branch's own base, which had
moved twice under the lane.

Two commits: `cursor_projection` from `marks` to `camera`, and
`Generation::get` deleted.

### The unit's own arithmetic was the review's best finding

The first pass deleted four lines of code and added a **six-line
paragraph about their absence** — in a unit whose opening finding was
that *a reader-less `pub` item is a third of a 51-line leaf module's
API*. `generation.rs` was **51 lines on base and 51 on head**, and the
PR quoted 51 as an outcome.

The lane's answer is the right one: the paragraph earns its place at
**two lines**, because a future reader reaching for the counter needs
exactly one fact — `Debug` is the door — and everything else in the
six-line version was argument for the deletion, which belongs in the
item's `## Closed`. 51 → 47.

### Every summary number was wrong while every checked fact was right

The reviewer read all 51 resolvable citations and confirmed the eleven
pre-existing stale ones individually. The summaries around them did not
survive: *"60 citations"* had no enumeration rule behind it; *"the six
`marks.rs` citations all move by −13"* was five, and the sixth had also
**narrowed** — base `marks.rs:135` was a `#[derive(…)]` never part of
the quoted sentence, so that one was already wrong on `main`, making
the tally **7 + 12**, not 7 + 11; *"twelve lines … six instead of
ten"* compared a paragraph to a block when nothing was twelve; *"two
public items"* was three.

**Second consecutive VIEW PR with this defect** —
`citation-repoint-shifted-a-number-the-lane-knew-was-wrong` (#2083) is
the class row, and the corrected receipt now states the rule that
produces its number so a second reader can re-derive it.

**And re-running the receipt after the last edit paid again**: the
`camera` prose trim moved `clamp_distance` from `:954` to `:948`,
invalidating a citation the same lane had corrected two hours earlier
in the same PR. Caught, re-derived. That is the third time today the
rule has caught a lane invalidating its own freshly-published
coordinates.

### The claim that was false in three places

*"Exactly one path to the function"* — `lib.rs:55` is
`pub mod camera;`, so `viewer::camera::cursor_projection` and
`viewer::cursor_projection` both resolve, and both existed through
`marks` before. The move preserved the COUNT, not uniqueness, and the
contrast drawn with `session-shims-and-test-imports` (a shim *inside a
module*) was the wrong one. Corrected in the PR body and the item;
**appended** as a correction to this log rather than rewritten, which
is the convention this program has held all day.

The class is the reviewer's:
`every-crate-root-reexport-is-a-second-path-not-the-only-one` — every
item in `lib.rs`'s `pub use` blocks has two public paths.

### The `camera` argument was three-quarters true

Three of the lane's four pre-move checks held exactly. The **subject**
check did not: `Camera::view_projection` returns `[[f64;4];4]` against
an `f32` argument, `Camera::project` answers `[f64;3]` against an
`[f32;2]` NDC input, and `ray_through` takes pixels with `+y` down. The
doors do not meet at the type; the conversion happens two modules away
in a driver. **The move is still right** — the other three checks carry
it — and the sentence claiming otherwise is rewritten to what is true,
with the f32/f64 question left to
`cursor-projection-is-f32-in-a-module-whose-matrices-are-f64` (which
also names four spellings of the f64→f32 matrix cast with no home).

### §6 failed twice on the same row, and now has a durable artifact

The out-of-fence TCOST citations this unit reported had **already been
reported once** — `loud-skip-marker-says-two-modules-and-there-are-six`
(VIEW's own slate, closed 2026-09-04 at #1848) ends with the same
report, *"reported rather than edited, since the issue is homed outside
this program's fence"*. Same defect, same channel, second traverse, no
durable artifact either time, and the answer was one grep away inside
this program's own directory.

Filed as
`work/issues/loud-skip-marker-row-cites-a-lib-paragraph-that-was-reversed`,
which is the cross-program home this repo already uses. It carries all
four non-resolving citations (including the reviewer's third, which the
first pass missed by reading two of the table's entries rather than the
table), the two that do resolve, the #1848 pointer, and the finding:
**§6's "report, don't file" produces nothing a later reader can find** —
exactly the case §6 warns about when it says you cannot tell whether
the item already exists.

One of those citations turned out not to be stale but **reversed**:
VIEW fixed that marker at #1848, so `lib.rs` now says the opposite of
what TCOST's row quotes. A better finding than "stale", and one only a
read-the-line pass produces.

### Disclosed by the lane, worth keeping

Mid-pass it ran `git checkout HEAD -- crates/viewer` to take a base
measurement and clobbered its own uncommitted edits with it. It caught
this on the next grep, re-applied, and **verified every coordinate came
back identical** before committing, re-taking the reported measurements
after the re-apply. Disclosed unprompted. The lesson for a successor:
take base measurements in a separate worktree, never by checking out
over live edits.

## The debug walk stops being hand-maintained, and takes a sibling with it (2026-09-06)

`debug-for-docsession-is-a-fourth-hand-maintained-walk` closed on
`view/debug-walk`. Four `Debug` impls now open with an exhaustive
`let Self { … } = self;`: `DocSession`, `Derived`, `LandedRun` and —
the sibling — `PickCache`. A field added to any of the four is E0027 in
its own walk, verified by adding a witness field to each and reading
the compiler, four for four, then reverting. `DocSession` renders
`Derived` as one field, so `Derived`'s members travel with their
declaration rather than being listed twice.

**`finish_non_exhaustive` now means exactly the `_` arms above it.**
That is the answer to the question the item left open. `Derived` has
none — all five fields render — so it `finish`es and the marker is
gone. `LandedRun` keeps it for the evaluation and the document it
answers; `DocSession` keeps it for five (`eval`, `requested_doc`,
`tol`, `display`, `resolver`); `PickCache` for its `IndexService`. A
field the walk will not carry is bound to `_` rather than left out of
the pattern, which is what makes the marker's list one the compiler
holds complete instead of a shrug.

**The derive was available and was rejected.** `Doc`, `Evaluation`,
`ProductError`, `ChecksReport` and `AtRestBadge` all derive `Debug`
today, so the "blockers" the item named are not blockers — and that is
the argument against it, not for it: a derive would print the whole
recipe DAG and the whole result DAG at every `{:?}` on a session, and
it cannot summarise, which is the one thing the existing walk's taste
does (`states`, `gesture`).

**The item's own sweep claim had gone stale in a day.** It recorded
`impl.*Debug for` and `finish_non_exhaustive` at one hit each under
`crates/viewer/src/`; both give two now, because `PickCache`'s impl
landed at `83fcb9540` the day after the file was opened. The entry at
line 1833 of this log carries the same claim, from the lane that made
it, and it is overtaken rather than wrong: the class to sweep is still
*hand-listed field census*, and the trait-shaped grep now finds two of
them instead of one. A sweep is accurate as of its merge base, and this
one was not re-run until the fix.

**Seven out-of-fence instances, reported not filed.** `Span`,
`SplineCoeffs`, `RationalCoeffs`, `CoeffWindow`, `RationalWindow`,
the macro-generated NURBS curve windows, `SurfaceWindow` and
`ParamSource` all hand-list their fields and end in `finish()` — the
same silent absence, with the stronger claim of completeness on top.
They are geom-core, geom and topo ground; §6 puts them in the report
and the PR body.

**Behaviour moved and says so.** `selection` and `hover` are now inside
a `derived` block, `landed_generation` became `derived.landed` with its
four verdicts beside it, and `derived.scratch` and `derived.bounds`
render for the first time — those two are the fields this defect had
already eaten. Nothing in the tree renders a `DocSession` or a
`PickCache`, so no assertion moved; the rendering is unobserved and the
PR says so rather than leaving a reader to find that out.

## The fix pass for #2093: a receipt that carried a false negative, and the class sitting eleven lines away (2026-09-06)

Seven items from the style review, five closed on the same branch, two
annotated and left open. What the review proved and the lane had not:
deleting both `session.rs` walks and running
`cargo check --workspace --all-targets` gives zero errors, so the
rendering change is unobserved by anything — mechanical where the lane
offered a grep; and replacing both walks with `#[derive(Debug)]`
compiles, so "the item was wrong about its blockers" is a fact and not
a reading.

**The receipt carried a false negative result.** It said *"every
`impl Display` in `crates/viewer/src/` (20 of them) is a `match` over
an enum's variants"*. The grep gives **36**, over 19 files, and **five**
are over structs reading fields by hand — the three the review found
plus `prefs.rs:304` and `blend.rs:128`. §5 makes the blind-spot
sentence part of the receipt, so this was an unverified negative in the
one place a reader trusts as verified: it told a reader that `Display`
under `crates/viewer/src/` had been looked at and was clean. It had not
been looked at. The re-derivation is under a stated rule with each
subject resolved against its declaration, and the five go on
`field-censuses-inside-view-survived-the-debug-sweep`.

**Three miscounts.** "Seven outside it" over eight rows and nine
concrete impls; `crates/topo/src/param_source.rs:84` carried under a
sentence claiming every one ends in `finish()` when it is a `write!`
over a one-field newtype; and
"16; 15 resolve", which was neither reading of the lane's own rule
(16 occurrences carry 14 resolutions, because the failing coordinate
appears twice — the second time in the sentence saying it fails). The
lane wrote a rule to avoid exactly this and then did not run it.

**An instance of the item's own class sat eleven lines from the fix.**
The item defines the class as *a hand-listed field census of any kind,
not the `Debug` trait* — and the sweep grepped the trait.
`PickCache::forget` cleared four of five fields by hand, eleven lines
below the walk that was being fixed, in the file the sweep was reading,
with a doc arguing that a missed field there installs an index of a
document nobody is looking at. Taken. Two further in-fence instances
(`impl PartialEq for Camera`, `DisplayState::clear`) stay filed; the
lesson is that a grep over a trait is the wrong instrument for a class
defined over a shape, and the item said so in its own text.

**The lane's own argument cut against the walk it shipped.** It
rejected the derive because *"a derive cannot summarise"* — and the
walk inlined a whole unbounded `ChecksReport` into a dump that had
carried `landed_generation` and nothing else, while printing
`scratch: false` in place of a document. Both were fixed rather than
either kept: `checks` is now its two counts, `resolver` is rendered as
its presence like the three fields beside it, and the stated reason to
reject the derive is DAG size, which is the reason that actually bites.
The rule is now written down because it was being applied unevenly —
**a `Vec`, a document or a DAG is summarised; a single value is
printed.**

**Prose that named an error and a door that do not exist.**
`Derived::none` is a struct literal, so its error is E0063, not the
E0027 the walks claimed by analogy; and there is no accessor for
`resolver`, so the justification for omitting it covered two of its
three subjects. The analogy is kept and made true; the `_`-arm
justification is per field; `resolver` left the omitted list entirely.

**One home plus pointers**, and one spelling. The argument lives in
`crates/viewer/README.md` and the four doc comments state only their
own `_` arms; all four walks are `core::fmt` now, including
`DocSession`'s pre-existing `std::fmt` that the two new ones had
matched.

**Filed:** `work/issues/hand-listed-debug-censuses-in-geom-core-geom-and-topo.md`
for the out-of-fence set, on the orchestrator's direction — nine impls
across three crates, ending in `finish()`, which is a completeness
claim over a hand-listed census and strictly worse than what #2093
fixed. §6 makes the report the filing act outside the fence; this
program established today that the report alone is not a durable
artifact, and the orchestrator wrote the file rather than leaving the
finding in a PR body. **Residue filed:**
`finish-marker-cannot-say-summarised`, for `std`'s two-valued marker
being asked to carry a three-way split.

**Declined:** moving `DocSession`'s walk to its declaration, 1,779
lines up. A pure move inside a mechanism PR is the mixing this program
spent today learning not to do;
`four-debug-walks-are-spelled-and-placed-two-ways` is retitled to that
move and stays open for a pass allowed to make it.

## #2093 MERGED; the class reached eleven lines and the sweep did not (2026-09-06)

**#2093 merged at `55ef18a26`.** Nineteen units on main this session.
CI: 37 jobs, twelve `test (…)`, five `k-lint (gate, …)`, `gate ok`
success, zero non-green. `#[test]` 535 at merge base `9d411c942` and
535 at head, checked here against the branch's own base.

Five `Debug`/census walks now open with an exhaustive
`let Self { … } = self;` — `Derived`, `LandedRun`, `DocSession`,
`PickCache`'s walk, and `PickCache::forget`.

### The review verified two claims by COMPILING, not by grepping

Asked whether any test observed the dump, the reviewer did not grep for
`{:?}`. It **deleted both `Debug` impls and compiled
`cargo check --workspace --all-targets`** — zero errors, so nothing in
the workspace requires either impl and the behaviour change is
genuinely unobserved. Asked whether the rejected derive would even
compile, it **replaced both walks with `#[derive(Debug)]`** and built
it.

That is a better instrument than the one the dispatch asked for, and it
generalises: **prove a claim by compiling when the compiler can answer
it.** A grep answers to the limit of a pattern; a build answers
completely. It is the counterpart to this morning's *a grep over a
signature is a grep over one line of it*.

### The class reached eleven lines and the sweep did not

The item defines its class as *a hand-listed field census of any kind,
not the `Debug` trait*. The sweep grepped the trait. **`PickCache::forget`
clears four of five fields by hand ELEVEN LINES below the walk the same
PR fixed**, in the same file the sweep was reading, and its own doc
argues that a missed `attempted` is what lets a late build install "an
index of a document nobody is looking at". The lane took it and said
so in those terms: *the class's own definition reaches eleven lines and
I did not.*

The other two survivors stay filed with a real distinction:
`DisplayState::clear`'s deliberate `revision` exception means its `_`
arm carries an argument that wants writing, not a mechanical
destructure.

### The unit's own argument cut against the walk it shipped

It rejected the derive on *"a derive cannot summarise"* and then
inlined an unbounded `ChecksReport` and a `Refused { message: String }`
into a dump that had carried only `landed_generation`, while printing
`scratch: false` in place of a document.

The fix did not keep the argument. `checks` became its two counts —
and the lane checked `ChecksReport`'s `Display` rather than deferring
to it, finding it **is not a summary**: only its first line is, then
`render_list` prints every finding. The rule the walks now state is
the one they were applying unevenly: **a `Vec`, a document or a DAG is
summarised; a single value is printed.** The stated reason to reject
the derive is now DAG size, which is the true one.

`Gesture` was named rather than fixed, with its own sentence corrected:
its derive had been read as the answer where the unit's own argument
reads it as the problem (`base: Doc`).

### Numbers, again, and the rules that catch them

Every corrected figure came with the rule that produces it:

- the Display blind spot was *"20, all enum matches"* — an unverified
  negative in the one place §5 makes a receipt. It is **36 across 19
  files, 31 enums and 5 structs**, and the lane found two the reviewer
  had not (`prefs.rs`'s `StoreError`, `blend.rs`'s `BlendTarget`);
- the out-of-fence set is **eight rows, nine impls**, and one of the
  eight is not the claimed shape;
- the citation receipt is **46 occurrences, 25 distinct, 24 resolve** —
  the earlier "16; 15" was neither reading of the lane's own rule;
- `Derived::none` is a struct literal, so its error is **E0063**. The
  re-run witness experiment produced E0027 at five sites and E0063 at
  that one, and the analogy is now true in both walks and the README.

Both receipt blind spots are stated, including the one that bites here:
the rule cannot see a line number written as prose, and `log.md:1833`
is the coordinate the sibling-check paragraph turns on.

**The orchestrator's own miscount**: an earlier check-in attributed
`83fcb9540` to #2083; it landed via **#2079**. Corrected in the
check-in with an instruction not to propagate the old number. Five
counts have come back wrong today, four from lanes and one from here.

### The out-of-fence file, and the sharper hazard inside it

Filed as directed, per §6's own text that the ORCHESTRATOR writes the
`work/issues/` file:
`work/issues/hand-listed-debug-censuses-in-geom-core-geom-and-topo.md`,
with the corrected count and shape claim.

The lane added something better than what it was sent for: **six
hand-written `PartialEq` impls sit beside those `Debug`s**, and an `Eq`
that misses a field answers *wrong* rather than merely printing less.
That is the sharper half of the same class, and it would have been
invisible to a sweep keyed on `Debug`.

### Declined

Moving `DocSession`'s walk 1,779 lines up to its declaration. It is a
move inside a PR whose warrant is a mechanism change, and this program
spent today learning not to mix those.
`four-debug-walks-are-spelled-and-placed-two-ways` is retitled to that
move and stays open. All four walks are `core::fmt` now, including the
pre-existing `std::fmt` the new ones had copied.

## The remaining seven field censuses get their tie, and the compiler names who was watching (2026-09-07)

`field-censuses-inside-view-survived-the-debug-sweep` closes. All seven
instances across the four hats convert to exhaustive destructuring:
`impl PartialEq for Camera`, `DisplayState::clear`, and the five
`impl Display` over structs (`StoreError`, `Message`, `Withdrawal`,
`Disagreement`, `BlendTarget`).

### The four hats needed four arguments, not one

The `PartialEq` is the sharp one and it moved behind a private
`Camera::coordinates`, so the census is stated ONCE and both sides of
`eq` go through it — a census written twice is a census that can
disagree with itself. That function carries a second pattern over
`Point3`'s `x`, `y`, `z`, because reading `target.x` by hand was where
the census stopped at the crate boundary and it did not have to.

`DisplayState::clear` needed no `_` arm: it USES `revision`, bumping it
when the reset was visible. The pattern's value there is that the
exception is named at the site the exception lives at.

The five `Display`s all convert, and the argument is deliberately not
"we did it to the others". The cost is one line, not five, and two of
the five come out shorter than they went in. What earns it is that four
of these five renderings are meant to be a COMPLETE account of their
value — `Disagreement`'s own doc argues exactly that, and the pattern
is what holds that paragraph to the value rather than leaving it
asserted. `Message` is the fifth and the opposite: its account is
deliberately partial, and `subject: _` is now where that decision
lives.

### The compiler answered both behaviour questions

Deleting `impl PartialEq for Camera` and driving to a fixpoint names
every consumer of camera equality — seven sites, one of them
`Folded`'s DERIVED `PartialEq`, which is transitive and invisible to
any grep for `==`. Two of the others compare whole cameras to check
that `camera::fold` agrees with sequential `apply` and that
`map_stream`'s camera agrees with folding its own ops, so a coordinate
outside `eq` is a coordinate those properties silently do not check.
That is the concrete cost the item claimed abstractly.

Perturbing all five renderings and running both suites in both feature
configurations names every assertion on them: four lib tests on
`Withdrawal`, one integration test on `Disagreement`, and nothing at
all on `Message`, `StoreError` or `BlendTarget` — though deleting those
three impls proves all three are rendered, at twelve sites. Rendered
and unasserted is a different answer from unrendered, and only the
compiler distinguishes them.

### Witnesses

Seven witness fields in one build: E0027 at all seven converted
patterns, E0063 at ten struct literals. The `Point3` pattern was
witnessed separately by dropping `z` from it (E0027, `camera.rs:128`).

### Receipt

Enumeration rule for the `Display` hat: every `impl … Display for T`
under `crates/viewer/src`, `T`'s declaration looked up and classified
struct or enum. 36 impls, 5 structs, 31 enums. `PartialEq` hat: 1.
The rule cannot see a macro-generated or proc-macro-derived impl; both
are closed by inspection instead — `vocab.rs` is the crate's only
`macro_rules!` and generates neither trait, and the manifest depends on
no derive-Display crate.

## The census row's fix pass: an eighth census, a false universal, and 21 citations (2026-09-07)

The style review of #2103 found real defects in the row above. Taken:

**An eighth census, in a file the PR had already edited.**
`BlendTool::clear` (`blend.rs:497-501`) set two fields by hand under a
doc that says *"Drop every pick"* — a census claim — 355 lines below
the `BlendTarget` census the same PR converted. Converted. Its
existence is what falsified the README's new universal, and the
universal is now BACKED rather than merely repaired: the sweep rule is
written down (every `fn` under `src/` naming two or more distinct
`self.<field>` writes, read against its declaration), it produces
**23 hits at head and none is a census**, and a converted census does
not match the rule at all — which is why a clean sweep is the receipt.
`DocSession::clear_for_new_document` is the case the rule matches and
the design answers, in its own doc.

**A `match` is exhaustive over VARIANTS, not over a variant's FIELDS.**
The closing sentence conflated the two, and two of the 31 enum
`Display`s do drop a field. Swept with `{ .. }` / `, ..}`, catch-all
and bare-binding arms, and tuple patterns below arity: zero catch-alls
over a subject enum, zero tuple drops, exactly two `..`. Neither is
converted — rendering either would move a rendering, and this PR's
whole claim is that none did. `MateToolEvent::PickLost` already carried
its argument; `CameraOp::Frame` carried none, and one is written at the
arm now.

**`Camera::coordinates` left its second `impl Camera` block.** It is a
`fn` nested in `eq` — its only caller — so `impl Camera` is one block
again and a reader sees the type's inherent surface in one place. Why
it reads fields and not the six public accessors (an accessor call is a
field read, so the E0027 tie would be gone) is now written at the
helper, since without it the helper reads as an unexplained second
census.

**The receipts were witness-tree numbers.** The witness experiment was
re-run on head and every E0027 line read back after the witness fields
came out. The E0063 count was **ten** and is **11**, with the
enumeration that produces it; the run is 18 errors, 7 x E0027 +
11 x E0063.

**Citations, as a class.** A behaviour-preserving move falsifies
sentences about STRUCTURE, and this one moved five files plus the
README. Every non-closed item under `work/view/` was swept for
citations into them and each was re-derived by re-finding its SUBJECT
at head, never by shifting its offset — the defect
`citation-repoint-shifted-a-number-the-lane-knew-was-wrong` closed on.
**25 citations corrected across 12 items**, and **20 of the 25 were
already wrong at this PR's merge base** — which is what re-deriving
finds and shifting hides. Five were true at the merge base and this PR
falsified them: `frame.rs:1689`, `camera.rs:948`, `display.rs:845` and
`display.rs:839-846` (whose quoted spellings changed too), and
`frame.rs:1122`. Three are left alone and reported instead: a quoted
`camera.rs` passage that exists nowhere in the tree at base or head, a
`session.rs` citation this PR did not touch, and a pasted `cargo fmt`
transcript, which is a record of a run rather than a citation.

**The lead the row closed over now has a file.** `## Where else to
look` named `ViewerApp`'s two hand-written blocks and the `## Closed`
section did not mention them. Both were read: neither is a census —
`ViewerApp` has 32 fields and neither block's population comes from the
declaration — but `perform_batch`'s reset-on-open arm draws from *the
fields derived from the outgoing document* with nothing at the
declaration marking that set, which is the `Derived` question one layer
up rather than the destructuring one.
`viewerapp-document-derived-state-has-no-boundary` carries it.

## 2026-09-07, orchestrator: #2103 merged, and what its two rounds cost

**#2103 is on main** (`574223e5d`), twentieth unit. Full code tier
green before the merge: 37 jobs, `gate ok` success, 12 `test (…)`,
5 `k-lint (gate, …)`. `#[test]` in `crates/viewer` 535 at the merge
base and 535 at head.

The unit went out as a style review because the class was settled by
#2093 and the conversions looked mechanical. That was right about the
CODE and wrong about what a review of it would find. **The review found
no broken code and four broken sentences**, and both rounds since have
been about prose:

- every compiler claim survived independent re-derivation — seven
  conversions byte-identical, `[f64; 8]` equality run against NaN,
  `±0.0` and `±INFINITY`, the `Folded` observer set reproduced at
  exactly seven by driving the compiler to a fixpoint, twelve rendering
  sites by delete-and-compile;
- two UNIVERSALS in `crates/viewer/README.md` were false. *"Every place
  in this crate that lists a value's fields by hand destructures the
  value instead"* missed `BlendTool::clear` — two fields named by hand,
  in a file the PR edited, 355 lines below a census it had already
  converted. *"Every other one is over an enum and is exhaustive by its
  `match` already"* conflated two axes: a `match` is exhaustive over
  VARIANTS, and `Display for CameraOp` drops `bounds` behind a `..`
  that carried no argument at all.

**The eighth census and the two `..` arms are the substance of the
second round.** `BlendTool::clear` converts; `camera.rs:357` and
`matetool.rs:353` do NOT — rendering either would change what the
chrome says, and the PR's whole claim is that no rendering moved, so
what they get is the argument for the drop rather than the field. That
is the orchestrator's call and it is the §3 line: a behaviour change
smuggled through a mechanical one is still a behaviour change, even
when the mechanical one is right.

**The citation sweep is the number worth carrying.** Re-deriving every
`<file>.rs:<line>` in `work/view/`'s open items into the five files
this PR changed corrected **25 citations across 12 items — and 20 of
the 25 were already wrong at this PR's own merge base**, two of them by
~80 lines. So `stale-file-citations-after-the-split` is not a residue
of one split; it is the tracker's steady state between sweeps, and a
sweep that only runs when a PR moves code will always find more than
that PR moved. Three could not be repaired and are now three named
classes on that row: a citation whose SUBJECT is gone rather than moved
(repointing it fabricates), one into a file outside the sweeping PR's
diff (so a diff-scoped sweep cannot converge), and a pasted TOOL
TRANSCRIPT that is not a citation at all — which constrains any
mechanical repointer, since one built for this row will match
transcripts and one that edits them is worse than none.

**The retitle was re-minted by the commit that made it.** The fix pass
retitled the row from "three" to "seven", converted an eighth census,
wrote the eighth row into the README table and the eighth section into
the body — and left the title at seven. Corrected to eight before the
merge. The rule it argues for is the one already on the plan: a title
is read against its own BODY, not against the brief that cited it.

**Residue, all filed rather than disclosed**:
`viewerapp-document-derived-state-has-no-boundary` (new — the reset arm
draws from the fields derived from the outgoing document and nothing at
the declaration marks that set, which destructuring cannot reach), and
the three citation classes above.

## 2026-09-07 — the const-ALL rule stops being prose

`a-new-hand-written-all-table-meets-no-gate` closed on `view/all-gate`.
PR 2046 converted nine vocabularies and left the tenth author
unguarded; `scripts/gates/viewer-vocab-declared-once.sh` is the guard.
It hits on a `const ALL` and on the un-named shape — any `const` array
literal of two or more `Type::Variant` entries — under
`crates/viewer/src`, and allowlists what `crates/viewer/README.md`'s
new `#### The lists that stay hand-written` table carries.

**The work was the siting, as the item said.** A ci.yml step in the
`mirror` job, a line in `ci-local.sh`'s `tier_blind_rows`, and a
`TIER_BLIND` entry; `gate-roster.sh` derives its roster from the
directory and needed no edit, and now counts 21 gates. The
tier-blindness argument is stronger here than for
`viewer-module-kinds.sh`: that gate reads the README for two of its
checks, this one reads it for its whole allowlist, so a docs-only PR
is precisely the change class it exists to judge.

**Two things this lane learned about the tooling.** `gate_rust_code`'s
statement view cuts records at `;` and a Rust array TYPE carries one —
`[(BooleanOp, &str); 3]` — so a `const` table splits in half and its
initialiser lands in a record with no `const` in it. The gate
reassembles items by bracket depth instead. And the `|| true` hazard
this program re-minted at #1953 has an `awk` twin: a reader that dies
inside a process substitution folds to "the document is empty", which
here would have printed *"delete this row, its list is gone"* about
four rows whose lists are fine. The readers write `lib.sh`'s marker and
the gate reads it where it resumes, not only at `gate_ok`.

**Left open on purpose.**
`viewer-suites-hold-hand-written-complete-variant-lists` — the suites'
four lists are inline arrays in a row, not `const` tables, so this scan
would not see one of them if it were pointed at `tests/`. Scoping the
gate to `src/` is the honest claim; the suites need their own decision
per enum.

**Two fences and a borrowed reader**, recorded because the next VIEW
gate meets both. The branch touches `scripts/gates/*` (GATES') and
three CIW wiring surfaces; ruled in — VIEW already authors
`viewer-module-kinds.sh` in that directory, GATES' `keep_out` admits a
new gate's wiring row as one announced line, and the `TIER_BLIND` entry
cannot be split off because a TIER-blind gate without one reds parity
itself. `work/issues/gate-wiring-fence-is-undrawn-for-the-parity-entry`
carries the announcement and the proposal to write the clause down.
Separately, `work/issues/gate-rust-reader-splits-an-array-type-at-its-semicolon`
is the out-of-fence half of this unit: `lib.sh`'s statement view is
what forced the gate's own item reader, and that reader should go when
the shared one is fixed.

### The correctness review of that gate, and what it cost (2026-09-07)

A review of #2106 before merge found three defects that a green run
could not have shown, and each is the same lesson at a different depth.

**A guard is per STAGE, not per pipeline.** The fix that bought the
resume-time marker read guarded `const_items`; the `awk` that decides
what a HIT is runs after it, in the same process substitution, and had
no guard at all. It survived an immediate death only by SIGPIPE
upstream — which reds naming the wrong reader — and a classifier that
consumes its input and THEN fails produced `OK` and exit 0 over two
planted breaches whenever the roster held no data rows, which is a
docs-tier edit away. The gate now states the RULE that produces its
reader population (a command that reads the subject and whose status
the shell discards, one entry per pipeline STAGE) and enumerates the
six it yields, rather than asserting a universal over a list nobody
re-derived.

**`const` opens a generic parameter.** `fn stack<const N: usize>` and
`struct Stack<const N: usize>` match an unanchored `const NAME:`
opening, and the depth counter counts `()[]{}` and never `<>` — so
accumulation began mid-signature and the item did not close until the
next depth-zero `;`, which inside an `impl` is never. Both directions
were live on a copy of the real tree: a hand-written `ALL` under a
const-generic `struct` went green, and a const-generic `fn` above
`BOOLEAN_OPS` produced two reds naming the wrong repair. The opening is
now anchored to a declaration at the start of a line, one spelling
shared by the test and the name extraction; the anchored population is
the same 104 lines the unanchored one matched, under `gawk` and `mawk`
alike.

**A key that drops the type ratifies a name, not a list.** A roster row
`Theme::ALL` keys on `theme.rs` and `ALL`, so a second
`impl Badge { pub const ALL: … }` appended to that module was ratified
by the row written for `Theme` — the item's own worked example, evaded
by choosing its home. Using the type was considered and rejected: an
associated constant's declaration says `[Self; 3]`, so its type is the
enclosing `impl` header, which the lexed view does not delimit and only
a parse would find, and three of the four rostered lists are free
`const`s with no type to use. What holds instead is a COUNT — one row
ratifies exactly one list, none is the retiring direction and more than
one is a red — which is loud in the direction that matters and costs no
parse. Its price is stated where it is paid: two same-named lists in
one module cannot both be rostered.

**And a self-test that cannot see the name a gate prints.** All twelve
cases matched a name-independent fragment, so a broken name extraction
shipped green over three garbage diagnoses. Four cases now match a
fragment containing the subject, and
`work/issues/gate-selftest-cannot-observe-the-identity-a-gate-names` is
the durable half — a harness affordance, not this gate's to build.

## 2026-09-07, orchestrator: #2106 merged, and what a correctness lane bought

**#2106 is on main** (`24be3075d`), twenty-first unit, 37 jobs — 34
success, 3 correctly skipped, zero failures. The unit closes
`a-new-hand-written-all-table-meets-no-gate`, the schedule #2046 owed
under §Q6.

**This is the wave's one unit that was NOT style-only, and the posture
was right.** The trigger for escalating it was not size or risk in the
usual sense: it was that **a gate which silently never fires is
indistinguishable from a gate that passes**, so the failure mode is a
confident wrong answer by construction. The correctness lane found
three MAJORs. One of them made the gate print `OK` and exit **0** over
two planted breaches whenever the roster table was empty — a docs-tier
edit away, and the #1953 class exactly, one program-generation after
this program last re-minted it.

**The fix pass then improved on its own review**, which is the part
worth keeping:

- it derived a **reader-population rule** — *a reader is any command
  reading the gate's subject whose exit status the shell DISCARDS, i.e.
  every STAGE of every pipeline inside a process substitution*. Stages
  rather than pipelines is precisely what the old universal got wrong,
  and the rule found a **second** unguarded reader the review had
  missed;
- it got the guard SHAPE right for a reason neither the review nor this
  orchestrator had: a brace group, not `|| reader_failed` on the
  pipeline, because `pipefail` reports the rightmost non-zero stage —
  so guarding the pipeline would diagnose an upstream death as the
  classifier and re-mint the bug inside its own fix;
- it ran **negative controls**: the self-test must FAIL when the gate
  is broken, verified three ways. That answers the review's MINOR-8,
  which was itself found by accident when a broken name extraction left
  a twelve-case self-test green over three garbage diagnoses.

**Verified here rather than taken on report**: the killer case rebuilt
by hand — roster emptied, breach planted — gives **exit 1 with five
errors**; the gate green on the real tree; 21 gates and every self-test
`FAIL=0` on the MERGED tree; `gate-roster` 22 registered;
`check-ci-mirror-parity OK`.

**Two rulings this unit needed.** The territory fork went four paths
wide and is settled in
`work/issues/gate-wiring-fence-is-undrawn-for-the-parity-entry.md`: the
`TIER_BLIND` entry in `scripts/check-ci-mirror-parity.py` is ruled the
same "one announced line" class as CIW's workflow files, because it is
FORCED — a TIER-blind gate in the `mirror` job with no such row reds
parity itself, so the entry cannot be split into a follow-up by its
owning program. And the lane's §6 report, which it had recorded as
"None": `gate_rust_code --statements` splits a Rust array type at the
`;` inside it, filed as
`gate-rust-reader-splits-an-array-type-at-its-semicolon` with the
property its eventual fix must inherit — the shared reader does not
track `<>` either.

**The merge itself is a rule.** `gate ok` was green at `77505a340` and
the merge was still refused: `main` had moved and #2103 had rewritten a
neighbouring section of `crates/viewer/README.md`, the file this gate
parses its allowlist from. Resolved by merging `main` IN — never
rebasing, never force-pushing — with `work/view/log.md`'s two halves
both kept in merge order, which is what an append-only file needs when
two units land on one day. Green CI on an old head is not a merge
criterion; it is a criterion about the head it ran on.

## The summarised field says so itself, and the marker is left alone (2026-09-08)

`finish-marker-cannot-say-summarised` closed, on none of the three
candidates it listed. Ev's ruling: the tree already held the right
mechanism in one of the four walks. `LandedRun` carried `checks`
through a `format_args!` that renders as something obviously a summary
(`0 finding(s), 0 skipped`), while the other summarised fields
rendered as `is_some()`, a `len()` or a mapped generation — and
`scratch: false` is a `bool` a reader who knows `std` and not the
README takes for the whole of a `Doc`. Every summarised presence now
renders as an elision naming what it stands for — `Some(<Doc>)`,
`Some(<Body>)`, `Some(<Gesture>)`, `Some(<DirResolver>)`,
`Some(<PickIndex for Generation(1)>)` — and `states` and `checks` are
untouched, because a count already reads as a count. `Derived` still
`finish`es; the other three still `finish_non_exhaustive`; no key was
renamed.

**The distinction belongs at the FIELD, not at the marker.** The
marker answers *are all fields shown?*, which is the only question a
two-valued flag over a field SET can answer; the question a reader has
at a summarised field is *is this value the whole field?*, and the
value is where that gets answered. The three-way split was never the
marker's to carry.

**Nothing rendered these dumps, and the compiler said so, not a grep.**
Deleting all four impls leaves the workspace building `--all-targets`
under both of `viewer`'s feature configurations, with its one doctest
unaffected: no `{:?}`, no `#[derive(Debug)]` over these types, no
`T: Debug` bound reached them, so no test could have been asserting on
one either. `crates/viewer/tests/debug_dumps.rs` is now the only
reader and holds the seven summarised fields to their spellings; it is
also the reason the README's new universal is not a claim over an
unread population.

**The sweep behind a universal is stated at the sentence.** The README
paragraph now gives the rule that produces its list — read every
`.field(…)` call in the four walks, 22 of them, and take the nine
whose value argument is not the destructured binding — and, because
nothing can hold the NEXT summarised field to the rule, the reason it
has no guard is written where the claim is: the destructuring makes
the compiler send a field's author to the walk, the rule is stated
there for them to read, and nothing in the tree computes on a dump.

**The citation pass found 26 of 29 already wrong at its merge base.**
Three of the re-pointed citations were falsified by this change; every
other one had been wrong at `92b2c303d`, some by hundreds of lines.
Four rows were left alone deliberately — a receipt dated to a SHA is a
record, and re-pointing it falsifies the record it is. The full table,
its enumeration rule and its blind spot are in the closed item.

## Corrections to the entry above, before it merged (2026-09-08)

Appended rather than edited, because this file is the narrative record
and not a slate: what the entry said is what it said.

**"Nothing in the tree computes on a dump" denied its own premise two
paragraphs earlier**, which named `tests/debug_dumps.rs` as the only
reader of these dumps — a reader that computes on one is exactly what
that suite is. The claim carrying the no-mechanical-guard argument is
about PRODUCTION code: no shipped path reads a dump, so a lapse in a
spelling costs a reader a misreading and can never cost an answer. The
test suite is the one reader, and what it computes on a dump is the
spellings themselves. `crates/viewer/README.md` now says it that way.

**The spelling claim was true of six fields and not of the seventh.**
`resolver: Some(<DirResolver>)` was asserted nowhere: the suite read
`resolver: None` and refused `resolver: false`, which catches a
regression to `is_some()` but not a rewrite of the elision's text —
mutating it to `<Resolver>` left the suite green. `resolver` is written
only by `Open` and `Save`, so reaching its present arm needs a file:
the suite now saves into a tempdir the way `tests/doc_io.rs` does and
reads both arms. The same mutation now fails a named assertion, and
"holds the seven to their spellings" is true at the granularity of
spelling, not only of field.

**Four citations were falsified by this change, not three.** The three
counted were tokens; the fourth is
`viewerapp-document-derived-state-has-no-boundary`'s `(:1555-1560)`,
the doc paragraph quoted in the same sentence as the `:1583-1586` that
WAS re-pointed — a continuation left behind by the re-point it hangs
off, which is worse than a visibly stale row because the sentence reads
as freshly verified. It is `:1563-1568` at head. Two more citations the
pass had passed or shifted were re-derived by subject in the same fix:
`a-module-…`'s README range (a delta of +17 applied to a hunk that
added 22 lines — the exact defect
`citation-repoint-shifted-a-number-the-lane-knew-was-wrong` closed at
#2083, re-minted), and `outstanding-and-progress-…`'s
`README.md:345-347`, passed as still true when its quoted phrase is at
`:349-350`. The receipt's split is therefore **30 re-pointed / 6 still
true**, of which 27 were already wrong at the merge base; and the split
counts CITATIONS — by leading number the same 36 split 28/8, because
two ranges had their ends re-derived while their anchors did not move.

**A count fixed in one place contradicts itself.**
`four-debug-walks-are-spelled-and-placed-two-ways` had `1953 − 173 =
1780` re-derived in its body while its own title and closing paragraph
still said 1,779 — #2103's defect re-minted. Both are 1,780 now; the
1,779s in this file and in `debug-for-docsession-…` are records of what
was said then and stay.

## 2026-09-08 — the two-shape rule stops counting readers

`bare-vocabularies-declare-their-words-a-second-time` closed on **Ev's
ruling, which is neither answer the item framed**: two of the four bare
vocabularies convert, and the README's rule is CORRECTED rather than
deleted.

**What was wrong with the rule, not with the population.** It asked
"is there a single-value reader?" and answered "method". That sends
`PathVerb` and `ArcMode` to the bare arm although a production loop
walks their table and wants a word per entry — the labelled arm's whole
purpose — because each *also* names one value's word on a combo's
closed face. The corrected test is **does anything walk the table for
its WORDS?**: a question about whether the words are table data, which
a sweep can answer, where a count of readers cannot.

**The sweep, stated at the sentence it produces.** Every loop over a
vocabulary's `ALL` under `crates/viewer/src`, read for what it asks
each entry for. Two exist and both ask for the word —
`pane/create.rs:727` (`:738`, `:748`) and `widgets.rs:300` (`:301`) —
so `PathVerb` (17) and `ArcMode` (6) are labelled and their loops now
read `(option, label)` pairs. `ToolKind` and `Seat` have no `src/`
reader of `ALL` at all and stay bare; the suites walk both lists for
the VALUES (`tests/combine_ops.rs:1290`, `:1422`), and a seat's word
reaches only an assertion message about the one seat that failed
(`:1471`, `:1478`) — a walk that would still do its job if the words
did not exist is not a reader of them.

**The accessor is opt-in, which is what made this a unit.** Giving the
labelled arm a `label()` unconditionally would hand one to the five
labelled vocabularies that never ask for a single value's word: dead
code under `-D warnings`, and an `#[allow(dead_code)]` over the arm
would silence the report that an accessor has lost its last reader. So
a vocabulary DECLARES the projection it wants — `pub(crate) fn label;`
under its `ALL` — and the macro emits a `const fn` MATCH over the same
tokens the array is built from. Not a scan of `ALL`: exhaustive by
construction, no fallback arm to write, const-evaluable (which
declaring it `const fn` is what checks), and the same codegen the
hand-written match had.

**The words are unchanged, proved by running.** A throwaway unit test
printed `index, variant, word` for all 23 entries at the merge base and
again after the conversion; the two outputs are byte-identical, same 23
rows and same md5. That is the receipt #2103's lesson asks for — these
words are on the user's screen, and a literal moving from a match arm
into a declaration is exactly where a rendering goes quietly wrong.

**Citations re-derived by subject, in the open rows that cite the files
this branch touched.** Four were wrong at the merge base and none of
the four was shifted by this branch:
`hand-maintained-mirrors-of-a-kernel-enum-are-unforced` cited
`forms.rs:44` for `BOOLEAN_OPS` (`:67`), `forms.rs:471` for
`MATE_PRIMITIVES` (`:511` at the merge base, `:487` here) and
`create.rs:887` for its production reader (`:892`);
`revolve-tool-unreachable-no-axisinplane-form` cited `forms.rs:52` for
`DatumKind` (`:93`) and `create.rs:354-363` for the four `DatumSpec`
arms (`:358-371`); `tone-is-a-value-in-frame-and-a-comment-in-two-panes`
cited `create.rs:592-594` for "a third copy" of the weak/coloured rule,
which is the "Add profile" button — the copy is at `:581-585`. The
`Was`/`Now` table in `stale-file-citations-after-the-split` is NOT
re-pointed: it is dated to `d799235e`, where `forms.rs:52` and
`create.rs:354-363` are exactly what it says they are, and rewriting a
dated record would make it false about the tree it names.

## 2026-09-08 — #2143's fix pass: the sweep rule did not produce its own population

The style review returned **merge after named fixes** on #2143, and the
first of them is this program's own rule broken by the PR that states
it: *a universal in prose owes the sweep rule that produces its
population, written at the sentence.* The entry above states the sweep
as "every loop over a vocabulary's `ALL` under `crates/viewer/src`" and
then says **two** exist. **Seven exist, and all seven ask for the
word** — `pane/create.rs:309` (datum row), `:409` (profile row),
`:727` (path verb), `:989` (pattern rule), `:995` (pattern output),
`:1093` (blend kind) and `widgets.rs:300` (arc mode), each binding
`(value, label)` and putting that label on the control it draws. "Two"
was the count of vocabularies this unit CONVERTS, which is a fact about
the diff and not about the population; and at head it is no longer even
expressible as a restriction, because `PathVerb` and `ArcMode` are
labelled now and sit among the seven indistinguishably. It was a
REGRESSION as well as an error: the paragraph the rewrite deleted
(`crates/viewer/README.md:940-941` at `92b2c303d` — *"the five labelled
ones are labelled because their word appears nowhere but the radio row
that draws them"*) was the only text accounting for the other five, and
nothing replaced it.

**The repair states the population, not a scoped sweep.** The other
open shape was to scope the sweep to "a vocabulary whose shape is in
question", and it is the wrong one here. The section's whole subject is
which shape each of the NINE has, so a sweep that answers for a subset
does not produce that population; and "in question" is a prior
judgement, not a mechanical filter — the same defect as the "count the
readers" rule this unit replaced, one level up. Stated as all seven,
the sentence is re-runnable by a reader: grep the loops, get seven,
check each binds a label, and the two that never appear are the two
bare ones.

**The sweep's scope now says `src/` AND `tests/`, because its reasoning
already did.** The entry above declares the sweep `src/`-only and then
rules `ToolKind` and `Seat` bare on evidence from
`crates/viewer/tests/combine_ops.rs` — which is where the only
word-touching walk of a bare vocabulary would live. A `src/`-only sweep
has nothing to discriminate on exactly the two rows it is deciding, and
a future tests-only word-walk goes unseen. The README carries the
corrected scope; the suites' two word-reading walks
(`tests/combine_ops.rs:896`, `:880`; `tests/blend_authoring.rs:831`)
are over already-labelled vocabularies and confirm their shape rather
than deciding it.

**The `for … in <V>::ALL` shape is the blind spot, and it is wider than
the entry above admitted.** Four walks reach a vocabulary's `ALL`
without that spelling: `tests/combine_ops.rs:880`
(`PatternOutputChoice::ALL.map`) and `tests/blend_authoring.rs:831`
(`BlendKindChoice::ALL.map`) READ THE WORD; `tests/combine_ops.rs:1290`
and `:1321` (`ToolKind::ALL.map`) and `tests/blend_authoring.rs:786`
(`ToolKind::ALL.into_iter().filter(…).all(…)`) read values only. None
is under `src/` and none reaches a BARE vocabulary for its word, so the
ruling is unchanged — but that is the sentence the receipt owed, not
"neither exists today".

**`vocabulary!`'s accessor name is no longer a free parameter.** The
matcher spelled it `$fvis:vis fn $word:ident;`, so the macro would
project whatever identifier a site handed it — and the crate already
spells this concept two ways (`ToolKind::label`, `Seat::name`). That is
the family's own argument turned on itself: `vocabulary!` exists
because a second hand-written copy of a list drifts, and a free
parameter reintroduces the drift in what the readers have to call. The
matcher is now a literal `fn label;` (`crates/viewer/src/vocab.rs:178`)
with the visibility left free, because visibility is a fact about who
may read the word rather than a second spelling of anything. It costs
nothing today: `PathVerb` and `ArcMode` are the only declarers and both
said `label` already, and `Seat::name` (`seats.rs:153`) and
`ToolKind::label` (`tools.rs:85`) are hand-written methods in their own
`impl` blocks that the bare arm never projects.

**`PathVerb::label`'s doc is trimmed to its own subject.** Eleven lines
for a one-line declaration, half of them about `ALL`'s completeness and
about issue #1385's coverage gap — migrated from the deleted `impl`
block rather than written for the new site. What is true of `label`
stays; the `ALL` half moves onto `ALL`'s own doc, where its subject is.

**The arithmetic.** `forms.rs` is 515 lines at `92b2c303d` and **486**
here, **−29**; the entry above says −24 (and #2143's body said −23),
both stated before this fix pass. No line at or above `:182` moves.
`MATE_PRIMITIVES` is therefore at `forms.rs:482`, not `:487` as that
entry and `hand-maintained-mirrors-of-a-kernel-enum-are-unforced` say;
the item is re-pointed, and this line corrects the record here.

## 2026-09-08, orchestrator: #2148 and #2143 merged, and the shape of the day

**Both on main** — `f982823b0` (#2148, the summarised-field rendering)
and `65929a454` (#2143, the two vocabularies and the corrected rule).
Twenty-three units. Both green on the full code tier, 37 jobs each,
verified from the job list rather than a summary. Both were **Ev's
rulings, taken in chat**, and in both cases the ruling was **not** one
of the options the item itself framed — which is the finding worth
carrying out of the day.

**Re-deriving a fork against the tree moved the question, twice.**
`finish-marker-cannot-say-summarised` offered three candidate spellings
and called none obviously right; the tree held a fourth, because
`LandedRun` already summarised through `&format_args!(…)` while every
other summarised field used `is_some()` and rendered `false`. The
answer was to follow the precedent already in the file.
`bare-vocabularies-declare-their-words-a-second-time` framed a
dichotomy — the labelled arm absorbs all four bare vocabularies, or
none — and tracing every reader gave **two**, with the rule CORRECTED
rather than deleted. **An item's own menu of options is a claim like
any other, and re-deriving it against the tree is what a fork costs.**

**Two closures under a week old were re-minted inside one PR**, both
found by a reviewer re-deriving citations by hand — never by a gate,
never by a lane's receipt. A count was re-derived at one site and left
in its file's `title:` and body (#2103's defect), and a citation was
re-pointed by applying a DELTA rather than re-finding the subject —
`787+17` where the hunk was `+22`, so the words the row quotes ended up
outside the range it named (`citation-repoint-shifted-a-number-the-
lane-knew-was-wrong`, closed at #2083 two days earlier). **A closed
item is a record, not a guard.** The orchestrator did it too, passing
`README:936-937` forward from a review report when the line was
`:940-941`; #2143's lane caught that and said so.

**Mutation testing earned its place.** #2148's reviewer did not read
the new test file and conclude it held the seven renderings — it
changed `resolver`'s elision to `<Resolver>` and ran the suite, which
stayed **green**. Six of seven were held; the seventh was only claimed.
The fix pass then reached that arm through a `Save` into a tempdir, and
the same mutation now fails by name. *Asserted-somewhere is not
asserted-here, and only a mutation tells them apart* — the counterpart
to #2103's *rendered-and-unasserted is not unrendered*.

**Residue**: `vocab-gate-counts-bullets-across-a-whole-prose-section`
(the #2106 gate reads every `- **` between its heading and the next
heading of any level — 131 lines of prose — so the real constraint is
"no bulleted list in this section" and the error misdiagnoses it), and
`work/issues/code-quality-item-quotes-a-viewer-doc-string-that-was-
rewritten` (§6, quotation rot on another program's slate). Both filed
by the orchestrator, neither taken here.

## 2026-09-08 — the vocab gate's kind scan is anchored to a paragraph, and the item's own numbers were wrong (#2172)

**The region, re-derived.** `readme_kinds` stopped at the next heading
of ANY level, and `#### The lists that stay hand-written` is one, so the
scan region was never the `###` section. Running the reader's own awk
over `crates/viewer/README.md` at `d02bb0e6b` gives `:931-1079` — **149
lines** — with the three bullets at `:1023`, `:1026`, `:1031`, all
inside the enumeration paragraph `:1020-1033`. The residue entry above
says "131 lines of prose" and the item said `:907-1037`; at the tree the
item was written against the region was `:908-1021`, 114 lines, and
`:1037` is inside the table's trailing prose — neither the end of the
scan region nor the end of the section (`:1044`). Both endpoints and the
count were wrong, and the item's `title:` carried the figure. **An
orchestrator's filed number is a claim like any other**, and the lane
that takes the item is where it gets checked; this one was told to check
it and it did not survive.

**The fix is an anchor on the announcing sentence**, which is what the
item proposed. Two things it did not: the anchor matches a PREFIX of the
line rather than the whole line, because a paragraph's wrap point is an
artifact of the fill column while the four constants this gate already
pins are headings and a table header, whole lines by construction; and
the anchor carries the count word, so the README's prose number and
`KIND_COUNT` now hold each other. Before this the section could say
"Four kinds of list stay hand-written" over three bullets and nothing
read the sentence at all.

**A pass case is the receipt, and it has to fail unfixed.** The case
this owed — a bolded bullet in the section's PROSE, expecting GREEN —
was run against the whole-section scan restored into the file, and
failed with the misdiagnosis the item described: *"ratifies 3 kinds …
and this pass read 4: "A bolded bullet""*. Two more cases and three
negative controls turn the suite red on demand. **A self-test case that
passes before and after proves nothing**, and the way to know which one
you wrote is to put the old code back and run it.

**Two citations into this gate were stale before the branch touched
it.** `work/issues/gate-selftest-cannot-observe-the-identity-a-gate-
names.md:25` names `:611-625` and "twelve `gate_selftest_case` rows";
at `d02bb0e6b` those rows were `:930-973` and there were twenty. `work/
issues/gate-rust-reader-splits-an-array-type-at-its-semicolon.md:65`
names `:139-153` for a sentence that lives in the item reader's header,
`:212-218`. Both were reported in the PR body rather than repointed from
a unit branch, per §6. **A file that grows fast falsifies its own
citations quietly**: neither was wrong when written, and nothing reads a
line range to check it.

## 2026-09-08 — a correction: the count hold was reachable around, and the kind reader read column 0 only (#2172, fix pass)

**The entry above is falsified in one sentence and it is corrected
here, not rewritten.** It says *"the anchor carries the count word, so
the README's prose number and `KIND_COUNT` now hold each other."* They
did not. The anchor holds the README against the GATE — amend the
section to four kinds and no line starts "Three kinds of list stay
hand-written", so the anchor reds. It did not hold the gate against
ITSELF, and the red named the way around as a co-equal repair: *"restore
the sentence, or change `KIND_ANCHOR` in $0 in the same diff"*. Take
that second repair alone — `KIND_ANCHOR='Four kinds of list stay
hand-written'`, `KIND_COUNT` left at 3 — and a section announcing Four
across three bullets was GREEN, with `3 kinds read from "Four kinds of
list stay hand-written"` printed on the OK line, a self-contradiction
nothing reads. **A hold whose own diagnosis names the edit that
defeats it is not a hold**, and the cost was one extra edit.

**Both copies are kept; the third edge is what was missing.**
`KIND_COUNT` is what makes an amendment cost an edit to the gate, so
deriving the number from the anchor and deleting the constant would
close the divergence by giving up the reason it exists.
`anchor_states_count` checks the anchor's first word against
`KIND_COUNT` before the README is opened, and the two anchor
diagnoses now say the number word moves with `KIND_COUNT`. The
count-mismatch red now names WHICH SIDE each number came from — it
asserted *"$README's section ratifies $KIND_COUNT kinds"* using the
GATE's number, telling an author the README ratifies four when the
README ratified three, whose plausible wrong repair is to add a fourth
bullet. The self-contradicting OK line is unreachable rather than
diagnosed: a green line nobody reads is not a guard.

**The kind reader read a bullet at column 0 only, and CommonMark does
not.** A list marker sits at up to three spaces of indent and may
interrupt a paragraph, so `  - **A fourth kind** …` on the line after
the announcing sentence renders to every human reader as the first item
of the announced list — and state 1 swallowed it as the sentence's own
wrap, state 2 as a continuation. Three kinds read, `OK`, exit 0: **the
wrong-and-quiet shape this gate exists to prevent, in the gate.** Fixed
as a class at all three sites that assumed column 0 (state 1's escape,
state 2's bullet, the `sed` that extracts the name), with the other
side of the boundary encoded exactly rather than approximated: at four
spaces the marker is a lazy continuation of the paragraph, a nested
item of the bullet above, or an indented code block, and a leading tab
advances to column four. Every rendering was checked against
`markdown-it-py` in CommonMark mode.

**Five fixtures and four direct rows, each run against the shape it
covers.** A case that passes before and after proves nothing, so each
was run against the unfixed spelling: reverting state 1's escape, state
2's bullet or the `sed` each makes a planted indented kind pass;
dropping the "the anchor must OPEN a paragraph" rule makes a QUOTATION
of the sentence red as a second announcement; widening the boundary to
`[[:space:]]*` reds on all three four-space near misses. The constant
pair is not expressible as a fixture — a fixture plants a tree and the
pair lives in the script, and a test hook that let one vary it would be
a way to set the count from outside — so it is four direct calls on the
predicate the guard is one `case` over.

**The README's universal owed its exception.** *"The prose in this
section may carry bulleted lists like any other prose"* is false for
one position: a list separated from the ratified list by nothing but
blank lines is ONE loose list in CommonMark, so its items are read as
ratified kinds and red. The sentence now carries the exception and the
rule that produces it. **A universal in prose owes the sweep rule that
produces its population** — the same obligation §5 puts on a scope
sentence in a PR.

## 2026-09-09 — the two wasm dead items are `cfg`, and the refusal that IS unsaid is somewhere else

`viewer-items-unreferenced-at-wasm32` closed. CIW's PR 2263 added the
first CI row compiling this crate for `wasm32-unknown-unknown` and
found `WINDOW_TITLE` and `ViewerApp::deliver_status` unreferenced
there; the row is `cargo check` and not `-D warnings` because of them,
which made it the one viewer row in the workflow that cannot fail on a
warning.

**Both took shape (1) — `#[cfg(not(target_family = "wasm"))]` on the
item — but only after shape (2) was run down.** The item declined to
choose, and the reason to choose carefully is that shape (2) is a live
defect and shape (1) buries it: if the browser build ought to be
delivering status for a refusal it swallows, the `cfg` makes the
silence permanent. Three findings settle it for `deliver_status`. The
dialog verdict it carries cannot occur on wasm, because the browser
links no dialog to return from. The refusal wasm *does* raise on those
controls is raised earlier and already said, through
`.on_disabled_hover_text(frame::NO_CHOOSER_BACKEND)` — the same const
string `dialog_status`'s `Show` arm would put on the line, so #1125's
posture is met on the hover route, not the status route, and the same
is true on a desktop with no zenity and no portal. And the status-line
sentence is unreachable on every target anyway: one `self.chooser` copy
both gates the button and feeds `dialog_status`, so every reachable
verdict at both sites is `Keep`.

**Shape (2) is real in this crate, and the sweep found it one control
over.** The sweep rule was every `cfg(…target_family = "wasm"…)` site
under `crates/viewer/src` — 35 by that spelling — asked whether the
browser takes a different arm, whether the difference is a refusal, and
whether anything says so. The preferences store is one: on wasm
`Store` is `prefs::Absent`, `remember_theme` returns early on
`!store.usable()`, and the palette picker is an ordinary enabled
`ComboBox`. So a browser user picks a theme, it applies, the tab
reloads, and the default is back with nothing having said why — against
`prefs.rs:318-321`'s *"disabled with a reason, never offered and then
silently ineffective"* and `app.rs:88-91`'s *"the Save control disables
itself"*, which names a control that does not exist. Filed as
`wasm-theme-choice-is-offered-and-silently-not-kept`. It gives
`deliver_status` no wasm caller — `remember_theme` reports through
`notices.push`, and the repair is chrome — which is why the two
questions came apart.

**The flip is proved and not made.** `.github/workflows/ci.yml` is
CIW's and is untouched; **no YAML was written here** — what went to the
orchestrator to route is the evidence that the flip would pass, and the
two candidate commands it would have to hold, not a diff. Both spellings run clean at the wasm target on
the closing tree, and the negative control is what makes that evidence:
the same clippy command on the unfixed tree exits 101 with those two
warnings as errors and nothing else, which is also the enumeration rule
behind *exactly two* — the compiler's reachability verdict over the
compiled configuration, not a grep. `--all-targets` is not available at
that target and never was: `crates/viewer/tests/` reaches
`ThreadEvaluator`, which is host-only.

**A second residue, disclosed with its number.** A rustdoc pass at the
browser target was already red at `1d29a8eeb` with nine unresolved
intra-doc links — doc comments compiled at both targets linking
host-only items, including `run_web`'s own doc linking `run`, which is
unresolvable in the only configuration that compiles the item it
documents. Nothing runs that pass, so nothing held the number. This PR
makes it ten, deliberately: `apply_status`'s doc links
`ViewerApp::deliver_status`, and de-linking a working host link to hold
a count nothing reads would make the host docs worse for no reader.
Filed as `viewer-docs-do-not-build-at-wasm32` with both shapes.

**Citation sweep.** Every `app.rs:NNN` in `work/view/` was enumerated
and each read at its base line; a pure 21-line insertion shifts every
citation at or after 114. Most were already stale — the split's residue,
which `stale-file-citations-after-the-split` holds — and a citation
already wrong is not one this change falsified. Re-derived by finding
the subject by name on the closing tree, in the three OPEN rows where
the base line really was the subject: `to_f32` (`cursor-projection-is-
f32-…`), `enum Pane` (`viewer-suites-hold-hand-written-complete-
variant-lists`), and `ViewerApp`'s declaration, `sync_scene`'s six
writes, the `None if opened` arm and its door comment
(`viewerapp-document-derived-state-has-no-boundary`). Closed rows and
this log's own past entries were left alone: both are records of what
was true when written, not guards.

## 2026-09-09 — #2272's fix pass: the wasm framing was half a class, and one dead symbol in `src/` was the cause of a tracker row

The style review returned mergeable with no MAJOR. Four fixes, and
three of them are the same mistake seen from three distances.

**A target is not a class.** `wasm-theme-choice-is-offered-and-silently-
not-kept` was filed as a browser defect — *"the one target where the
store is known unusable"*. It is not: `remember_theme`'s guard reads
`store.usable()`, which is a property of the store's STATE, and the
native `FileStore` answers `false` too whenever `frame::prefs_path()`
returns `None`, which it does when neither `XDG_CONFIG_HOME` nor `HOME`
is set — a rule `frame.rs:1703-1706` states in the imperative and this
path breaks. The item is re-framed around `usable()`; the reason it
matters is that a fix keyed on `target_family` would have repaired the
browser and shipped the desktop instance untouched.

**And one level down, the refusal written for exactly that environment
is dead.** `FileStore::save`'s pathless arm (`prefs.rs:407-413`) says
*"no config directory in this environment"*; `store.save` has one call
site in `src/` (`app.rs:1022`) and the `usable()` guard returns before
it under precisely the condition that arm fires. No test reaches it
either — the suite only builds `FileStore` through `at`. So the crate
holds a typed sentence for a case it answers by returning quietly, and
the guard and the refusal are one decision, not two.

**A doc comment in `src/` was the source of a wrong tracker citation,
and the sweep found the symptom.** `session/op.rs:742` said the chrome
renders supersessions through `frame::supersession_notice`. **That
symbol has never existed.** The real path is
`frame::Withdrawal::superseded` through `Withdrawal::notice`, reached
at `app.rs:932-935`. Fifteen occurrences of the dead name are in
`work/view/*.md`, four in still-open items — including the
`free-move-drag-dissolved-by-open.md:53-55` citation that
`stale-file-citations-after-the-split` had already, correctly, decided
not to repoint. The tracker learned the name from the code.

The rule that makes it invisible: **`scripts/doc-gate.sh` fails only on
BRACKETED intra-doc links**, so a bare `` `frame::foo` `` code span in a
doc comment is prose to rustdoc and to the gate. Swept
`crates/viewer/src` for the shape — 19 spans over 12 names for
`frame::`/`session::`, 132 over 97 admitting every module prefix —
restricting the definition check to this crate's own modules, which is
what makes "not defined here" mean "does not exist". **Exactly two dead
names, both `frame::`**: the one fixed, and `frame::dropped_hide_notice`
thirteen lines below it. Filed as
`doc-comments-name-symbols-that-do-not-exist`; the fixed one is now
bracketed, so rustdoc holds it. The blind spot that bites is plain `//`
comments — `app.rs:939` carries the same dead name and rustdoc can
never reach it however it is written.

**§Q6 on an unguarded reachability claim: write the reason, at the
claim.** `deliver_status`'s doc says no `Show` reaches it at either call
site. Nothing holds that: the one row over the arm exercises
`dialog_status` as a pure function and stays green through any chrome
change, and the door exists precisely because the `add_enabled` gate
might be loosened. The paragraph now says so and names what a reader
who loosens the gate must re-check. **No guard was invented for it** —
Q6's third option is a written reason, and the honest answer here is
that a guard would cost more than the arm.

**What went to Ev rather than being answered.** Whether the status
route was SUPPOSED to fire for an absent chooser, or whether
`add_enabled` quietly took its job, is not decidable from the tree:
every line is consistent with both readings and the difference is what
#1125 intended. Filed as a `ruling`,
`was-the-status-route-supposed-to-fire-for-an-absent-chooser`, with
`needs_ev: true` and no answer in it.

**Two citations found wrong, left alone, and now written down.** The
PR's sweep found `free-move-drag-dissolved-by-open.md:53-55` (a symbol
that does not exist) and `new-document-owes-the-reframe-open-gets.md:18,
20` (right subjects, wrong lines, at base and at head), judged both
correctly, and recorded neither anywhere a later reader could find. A
PR body is not a slate (§6). Both are rows in
`stale-file-citations-after-the-split` now, with the base and head
locations derived, and the first of them is a FOURTH class that item's
line-number sweep cannot fix: a citation whose file and line are
repairable but whose named symbol never existed. Class 1 is a subject
that is gone; this is a subject that was never there, and repairing the
number would leave a false sentence pointing somewhere real.

**Operational.** The §Q6 paragraph is a ten-line insert at `app.rs:1065`,
so every `app.rs` citation at or after it moved by +10 — including four
this branch had already re-derived once. Re-derived again after the
last edit, by finding each subject by name. That is the second time in
one PR that the LAST edit invalidated an earlier sweep; the instrument
is fine, the discipline is to run it last.

## 2026-09-10 — the absent chooser was in the wrong CHANNEL, and `Keep` had to be proved a no-op first (#2278)

Ev ruled **(c)** on #2275 — *"(c) is right!"* — so `frame::dialog_status`
is deleted whole, `ViewerApp::deliver_status` with it (no callers left),
and #2272's `#[cfg(not(target_family = "wasm"))]` and the fifty-line
paragraph defending the arm's latency go with them. `app.rs` 2,022 →
1,961. The surface stays the hover text: Ev ratified (c) without asking
for a badge, and (c)'s own argument is that a disabled control with its
reason on hover already IS a read of held state.

**The deletion rested on a claim nobody had checked, and checking it is
the whole unit.** Every reachable verdict at both call sites was `Keep`
— that is what made the arm unreachable — so removing the calls is
behaviour-preserving *if and only if* `frame::deliver(…, Keep)` does
nothing. Not "obviously": if `Keep` touched `status`, or reset or
preserved anything a later frame reads, this would be a rewrite wearing
a deletion's clothes. Settled three ways rather than assumed. The
compiler argument is airtight — `deliver`'s `Keep` arm is
`apply(status, Keep)` and does not name `notices` at all, and `apply`'s
`Keep` arm is `{}` — but a compiler argument is not a demonstration, so
it was demonstrated too.

**And the demonstration found that the existing row could not have
caught the interesting failure.**
`deliver_sends_news_to_the_notices_and_retirements_to_the_field`'s
`Keep` block started from `notices = Vec::new()` and asserted
`notices.is_empty()`. That is green for a `Keep` that does nothing AND
green for a `Keep` that SWEEPS the frame's news — an empty vector cannot
tell "did not push" from "cleared what was there" — and a sweep is
exactly the behaviour whose absence the deletion depends on. The block
now starts non-empty and asserts against a snapshot. Two perturbations
prove the new assertion falsifiable in both directions: `Keep` made
`notices.clear()` reds `frame.rs:2170` where the old assertion stayed
green, and `apply`'s `Keep` made `*status = None` reds `:2171`. This is
#2148's rule again — **asserted-somewhere is not asserted-here, and only
a MUTATION tells them apart** — met before the deletion rather than
after it.

**A deletion falsifies sentences about structure, and two of the four it
falsified were not on the dispatch's list.** The sweep rule was an
unfiltered grep for the three deleted names plus a prose pass for the
phrasings that describe the route without naming it (*belt to*,
*braces*, *loud arm*, and `dialog` inside the viewer README and
`frame.rs`); its blind spot is a description sharing no token with any
of those. Beyond the six sites the dispatch named it found
`frame.rs:102`, where the module header's examples of what goes on the
line included *"a dialog that could not open"* — a sentence nothing in
the crate can now produce — and, in the tracker,
`viewer-suites-hold-hand-written-complete-variant-lists`, whose FOURTH
instance was the deleted test's three-`ChooserBackend` loop. That row
now says three and says why: **the member was removed, not repaired.**
`ChooserBackend` still has three variants and still has no `ALL`.

**The `frame.rs` growth ledger falls for the first time, and it falls by
one line.** 2,538 → 2,537. Written into
`frame-module-has-eight-concerns-and-no-holds-row` with the honest
caveat rather than as a win: 2,475 → 2,538 is unrecorded because the
units between #2026 and here did not write the row, so the fall is
noise inside a gap nobody measured. That file also asserted *"is 984
lines"* in the present tense at its own head, contradicted by the
ledger four paragraphs below it; re-derived by `wc -l`, and it was the
file's only present-tense copy.

**The row that predicted the delete miscited its own span.** The ruling
names the defending paragraph as `app.rs:1056-1073` in two places; the
deleted item and its doc ran `:1046-1095`, so the cited range was the
doc's middle rather than the item. Nothing turned on it, and it is
recorded in the row because **re-derive means find the SUBJECT** applies
to a prediction as much as to a citation — and this is the first time it
has caught the predicting row instead of the lane.

**§6, across the fence and staying there.** `.github/workflows/ci.yml`'s
wasm row says in the present tense that the crate carries two dead-code
warnings, `WINDOW_TITLE` and `ViewerApp::deliver_status`, and that
`-D warnings` would red on them. #2272 `cfg`-ed both and proved the flip
clean, so the comment named a resolved state before this branch existed;
now one of its two subjects does not exist at all. The row is CIW's, the
closed item says CIW will take the edit on request, and the orchestrator
is filing it on their slate. Reported, not edited.
