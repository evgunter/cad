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
(`crates/editor-core/IDENTITY.md` DI1 — a held id is valid on the history
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

Two corrections from reading `crates/editor-core/IDENTITY.md` against the
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

## 2026-09-08, orchestrator: #2172 merged, and a gate that failed its own thesis

**#2172 is on main** (`2679a9451`), twenty-fourth unit, 37 jobs green.
It closes `vocab-gate-counts-bullets-across-a-whole-prose-section`:
the #2106 gate read every `- **` between its section heading and the
next heading of any level — 149 lines of prose — so the constraint it
actually imposed was *never write a bulleted list in this section*, and
its error told the author they had added a **ratified kind** they had
not. #2143's author had already paid for it, writing a whole README
rewrite with no bulleted list to keep the gate green.

**The review found the gate failing its own thesis.** A bullet indented
two or three spaces renders — to CommonMark and therefore to every
reader of the page — as an item of the announced list. The new reader
escaped its paragraph state on `/^- /`, column zero only, and swallowed
anything indented as a continuation. So a fourth ratified kind, visible
to every human, read as three and printed `OK`, exit 0: **the exact
defect this gate exists to prevent, inside the gate written to prevent
it.** A `^`-anchored pattern over markdown is a claim about column zero
that markdown does not make.

**And the hold the change was argued on was not a hold.** The unit put
the count word inside `KIND_ANCHOR` on the ground that the section
could otherwise say "Four kinds…" over three bullets unread. The
reviewer did not argue with that; it took the resulting red's OWN
second suggested repair — change `KIND_ANCHOR` to match — and got
**green, exit 0**, with the `OK` line printing `3 kinds read from
"Four kinds of list stay hand-written"`. The count word bought one
extra edit, and the error message named that edit. **A guard whose
diagnosis offers a way around it is a speed bump, and the only way to
learn that is to follow the repair the tool prints.**

Both are closed, and both were verified here rather than taken on
report: the indented bullet now exits 1 listing `"A fourth kind"` among
the bullets read, and the `Four kinds…` state is now unreachable rather
than merely diagnosed — a green line that contradicts itself is read by
nobody, so it gets a guard, not a message.

**The fix pass went past its brief three ways.** It settled the indent
boundary with a CommonMark parser per position rather than reasoning
about it (two and three spaces are items; four after the anchor line is
a lazy continuation; four after a blank is an indented code block) and
checked the regex under both gawk and mawk. It found a paired case
nobody had named — the anchor now matches only where it OPENS a
paragraph, which is the renderer's own lazy-continuation rule. And it
caught a defect of its own: the new guard was first a bare call under
`set -e` and killed the gate before its own `gate_error` could print.
**A guard that dies before its own diagnosis is worse than no guard.**

**The orchestrator's filed figures were wrong, and the diagnosis is
better than "wrong".** The item said `crates/viewer/README.md:907-1037`,
131 lines; the region was 114 lines at the tree it was written against
and 149 at the branch's base. The reviewer found where 131 came from:
`447124324`, one commit earlier, where `907-1037` inclusive *is* 131.
**The figures were a blend of two trees.** The orchestrator's numbers
get re-derived like anyone's.

**Residue, filed rather than absorbed**:
`gate-section-scans-end-on-any-column-zero-hash` (both README scans end
at any column-0 `#`, so a Rust attribute inside a fence truncates the
section and the anchor then reports a sentence that is visibly present
as gone) and
`gate-reader-guards-count-six-where-the-stated-rule-yields-nine` (the
header's number counts guards where its own rule counts stages, leaving
three diagnoses able to name the wrong reader — the
misdiagnosis-by-`pipefail` that same file argues against elsewhere).

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

## 2026-09-09, orchestrator: #2272 merged, and the question it left standing

**#2272 is on main** (`47bfaedae`), twenty-fifth unit, 37 jobs green.
It closes `viewer-items-unreferenced-at-wasm32`, CIW's §6 report from
PR 2263: two items in `app.rs` were unreferenced at
`wasm32-unknown-unknown --features app`, and were the only reason that
CI row cannot deny warnings.

**The unit was the choice, not the edit.** The item offered two shapes
and declined between them — `#[cfg]` the items, matching every user, or
find the wasm caller that was lost, in which case the warning is the
symptom of a live defect. **Shape (1) for both**, and the review agreed,
but narrowed the ground it rests on: of the three the PR argued, only
one carries it. A single `self.chooser` copy both gates the button
(`add_enabled(chooser.usable(), …)`) and feeds the verdict
(`frame::dialog_status`), so `Show` requires a click on a widget built
`enabled = false`. The reviewer closed that by reading egui 0.36.1
rather than assuming it: pointer, keyboard and AccessKit click routes
are each `if enabled &&`-gated, so the button cannot report a click.

**Asking what would have shown the OTHER shape is what found the real
defect**, one control over. The store's `usable()` guard makes
`remember_theme` return early, so the palette picker is offered,
applies, and its persistence is silently ineffective — against two
prose claims that say the opposite, one naming a "Save control" that
does not exist anywhere in the chrome. The fix pass then re-framed that
finding off the target and onto the guard: `FileStore::usable` is
`path.is_some()` and `frame::prefs_path()` returns `None` with neither
`XDG_CONFIG_HOME` nor `HOME`, so **the native build takes the identical
silent return**. A `target_family` fix would have repaired the browser
and shipped the other half untouched. Filed as
`wasm-theme-choice-is-offered-and-silently-not-kept`, now framed on the
guard.

**A dead symbol in `src/` was the cause of a wrong citation in
`work/`.** `session/op.rs` named `frame::supersession_notice`, which
has zero definitions; the real renderer is `frame::Withdrawal::superseded`.
The tracker sweep had found the symptom and not the cause, and
`doc-gate.sh` could not see it because the span was unbracketed. Fixed
bracketed, so the name is held rather than merely correct today, and
the class filed as `doc-comments-name-symbols-that-do-not-exist` —
19 spans over 12 names for `frame`/`session`, exactly two dead, the
second thirteen lines below the one repaired.

**A fourth class for `stale-file-citations-after-the-split`**: class 1
is a citation whose subject is GONE; this is one whose subject was
NEVER THERE. Repairing the number leaves a false sentence pointing at
something real, which is worse than a visibly broken one.

**What stays open, and it is Ev's**:
`was-the-status-route-supposed-to-fire-for-an-absent-chooser`. Ground 3
proves `dialog_status`'s `Show` arm has **zero production reachability
on any target** — its only reader is a unit test on the pure function.
That sentence was written for #1097, a WSL box where Open silently did
nothing. Everyone in this chain read "it can never fire" as
reassurance; it may instead be the finding, with `add_enabled` having
quietly taken the status route's job. Pressing the PR's second ground
points the same way: a disabled button's hover text is neither of the
two channels `crates/viewer/README.md`'s provenance rule enumerates,
and by that rule's own test it is badge-shaped — so "the same const
string" was never the substitute it reads as. Harmless only because the
route is empty everywhere.

**Corrections to the orchestrator, both from lanes**: `superseded_text`
is a `#[cfg(test)]` helper, not the production renderer, and the
contract clause in `stale-file-citations-after-the-split` is at `:40-41`,
not `:47-49`. Both were mine, both stated in briefs, both caught.

## 2026-09-10 — the absent chooser was in the wrong CHANNEL, and `Keep` had to be proved a no-op first (#2278)

Ev ruled **(c)** on #2275 — *"(c) is right!"* — so `frame::dialog_status`
is deleted whole, `ViewerApp::deliver_status` with it (no callers left),
and #2272's `#[cfg(not(target_family = "wasm"))]` and the fifty-line
paragraph defending the arm's latency go with them. `app.rs` 2,022 →
1,956. The surface stays the hover text: Ev ratified (c) without asking
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
`notices.clear()` reds `frame.rs:2166` where the old assertion stayed
green, and `apply`'s `Keep` made `*status = None` reds `:2170`. This is
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

**The `frame.rs` growth ledger falls for the first time, by one line**;
the entry and its caveat are in
`frame-module-has-eight-concerns-and-no-holds-row` and are not repeated
here. What is only here: that file also asserted *"is 984 lines"* in the
present tense at its own head, contradicted by its own ledger four
paragraphs below — and it was the file's ONLY present-tense copy, which
is what made re-deriving it a whole-file fix rather than #2148's
half-fix.

**A correction this lane wrote and then had to withdraw.** The first
draft of the record above accused the ruling row of misciting its own
defending paragraph, comparing its `app.rs:1056-1073` against the
deleted item's `:1046-1099`. Those are two spans of two subjects: read
on `origin/main`, `:1056-1073` is exactly the two paragraphs the row
names. **A re-derivation that compares two different subjects invents a
defect**, which is the same failure as shifting a number by a delta and
is easier to commit under a rule that says to look for one.

**A diff that shifts a file is the diff that broke the citations in
it, and this one shifted two.** `frame.rs` moves +1 below ~line 382 and
−12 below ~1767; `app.rs` moves −61 below 1046. The census: **101
citations into `app.rs`, `frame.rs` or `frame_policy.rs` across every
OPEN row in `work/*/`** — 50 unmoved, 30 moved, 21 pointing at a line
that is gone or past the file's end. Method: build an exact old→new
line map per file with `difflib.SequenceMatcher` over the `origin/main`
and head versions, then map every `file:line` a regex finds in an open
row. **Eleven were mine to fix and are fixed** — the five the review
named plus `frame-module`'s concern span, `wasm-theme`'s four, and
`stale-file-citations`' two live derivations. What the method cannot
see: a citation written as prose (*"the arm at the top of `deliver`"*),
a range whose END line is stale while its start is not, a citation into
a file this diff did not touch, and — the one that produced a false
positive on every row this branch had already re-derived — it cannot
tell a number written against `origin/main` from one written against
this head, so nine hits had to be read back by hand before being
dismissed.

**And the census had to be run TWICE, which is this program's own rule
arriving on schedule.** The first pass fixed eleven citations; the fix
pass then shortened three doc comments and lengthened one, moving
`frame.rs` by +2 below line ~382 and `app.rs` by −5 below 1172 — so
every number the first pass had just derived was wrong again. *The last
edit invalidates the earlier sweep* is written in `plan.md` twice, and
it still cost a second full derivation here. The instrument that closed
it: find each subject BY REGEX ON ITS OWN TEXT at head, print the line,
and read all 31 back — never map a delta, and never derive before the
prose is final. Final: `frame.rs` 2,538 → **2,536**, `app.rs`
2,022 → **1,956**, `#[test]` 539 → **538**.

**Two of the thirty are not arithmetic and are not this diff's.**
`chrome/drag-tick-has-three-homes.md:19-22` says the drag tick is
answered in three places in `crates/viewer/src/app.rs` and cites
`app.rs:1093-1099` and four constants at `:1056`-`:1079`; `drag_tick`
and all four constants are in `crates/viewer/src/forms.rs` (`:391`,
`:354`), so `app.rs` holds none of them and the row was pointing at the
wrong FILE before this branch existed. `prune-report-rows-are-nine-
copies-of-one-assertion.md:21` cites `frame_policy.rs:963` for a
15-line `outcome.superseded` block; on `origin/main` that line is `);`
inside a hover-midpoint closure and the block is near `:1893`. Both
left as written — the first is CHROME's, and the second is one entry of
a nine-site census whose other eight are unverified, so fixing it alone
is the half-fix this program has a rule against.

**Nothing asserts `NO_CHOOSER_BACKEND`'s wording now, and the answer is
a row rather than a test.** The deleted suite carried
`contains("zenity" / "xdg-desktop-portal" / "command line")`. Restoring
them would read as coverage for `README.md:35-46`'s promise — that a
person sees three remedies in a tooltip — while covering none of it: a
`const &str` is fixed at compile time, and the untested step is the
const REACHING a tooltip, which `chrome_labels.rs`'s own header says
this crate cannot test (*"not testable without a window"*). A test that
looks like it holds a claim it does not hold is worse than a stated
gap. Filed as `hover-route-for-an-absent-chooser-has-no-test`, with two
candidate shapes and neither costed.

**§6, across the fence and staying there.** `.github/workflows/ci.yml`'s
wasm row says in the present tense that the crate carries two dead-code
warnings, `WINDOW_TITLE` and `ViewerApp::deliver_status`, and that
`-D warnings` would red on them. #2272 `cfg`-ed both and proved the flip
clean, so the comment named a resolved state before this branch existed;
now one of its two subjects does not exist at all. The row is CIW's, the
closed item says CIW will take the edit on request, and it is filed as
`work/ciw/wasm-row-warning-debt-comment-names-a-closed-item-and-a-deleted-symbol.md`,
which names this deletion at `:48-51`. Reported, not edited.

## 2026-09-10 — `view/gate-readers`: the vocab gate's two reader defects (#2282)

Two filed items, both pre-existing, both found by the style review of
#2172, both against `scripts/gates/viewer-vocab-declared-once.sh`:
`gate-section-scans-end-on-any-column-zero-hash` and
`gate-reader-guards-count-six-where-the-stated-rule-yields-nine`. One
pass over one file, not one defect.

**The population count re-derived: nine, and the same nine.** The
item's enumeration was a reviewer's and unchecked. Deriving it from the
stated rule against the file at `104f1445b` — every stage of every
pipeline inside a process substitution — gives five substitutions and
nine stages: `find`+`sort`; kinds `awk`+`sed`; table `awk`; rows `sed`;
`gate_rust_code`+`ITEM_AWK`+`HIT_AWK`. The item's exclusion of `printf`
holds, and now has a second argument at the site: it is a bash BUILTIN,
so nothing on PATH can shadow it away.

**Where the item was wrong is its CONSEQUENCE half, in both
directions**, and this is the half worth keeping. It names three
misdiagnoses. Two are real, one is not, and one it does not name is:

- a dead `sort` reported as *the source enumerator* — real, and a
  genuine wrong name;
- a dead kinds `sed` reported as *the kinds reader* — real, but not a
  WRONG name: one name covering two stages, so a CI log could not say
  which died;
- a dead rows `sed` reported as *the row reader* — **not a defect**.
  `table_rows` is `printf | sed`, `printf` is not a reader by the
  gate's own rule, so that guard already covered exactly one reader
  stage and *the row reader* IS that `sed`;
- **the one it missed**, and it is the same shape as its `sort` case: a
  dead `gate_rust_code` drew TWO diagnoses — `lib.sh`'s own correct
  *the shared Rust reader* AND this file's *the const-item reader*, for
  a stage this file does not own.

An item's enumeration is a claim like its options are (#2172's lesson,
one column over): the arithmetic was right and three of the four
consequences it asserts came out differently when reproduced. All four
were reproduced by hand before any edit.

**The repair, not the disclosure**, at every site: nine stages, eight
guards in this file plus `lib.sh`'s own, each brace-grouped so the
status it reads is that stage's own. That is the shape `const_hits`
argued for alone and the rest of the file has now been brought to.

**Item 1 fixed wider than it asked for, deliberately.** One
`FENCE_AWK`/`md_fenced` helper prepended to both README readers the way
`gate_record_awk` prepends `gate_record_split`, and both readers ask it
of EVERY rule they have rather than only of `^#` — because a `|` line
inside a fence was being read as a roster row, which is the same defect
with the other sentinel. The rule is CommonMark's and not a toggle: a
bare toggle lets a ``` line close a `~~~` block and hands the rest back
to the heading rule, which is the repaired defect re-minted by the
cheaper spelling of the repair, so that case is planted.

**The defect was LATENT on the real tree, not firing**, and the item's
*"live rather than theoretical"* heading does not say so. All fourteen
fence lines in `crates/viewer/README.md` are at `:3-262` and the section
opens at `:930`, so every one of them is above the scanned region — the
defect was one fenced example inside the section away. The tracker runs
over all fourteen every pass regardless, and they balance; an unclosed
one would leave the section heading itself fenced and red the gate,
which is why the closed-fence case is planted rather than argued.

**The negative-control table.** Control = the file at `104f1445b` with
the new planters spliced in verbatim and a one-case dispatcher in place
of `gate_selftest`, so the only difference between columns is the
reader.

| case | before | after |
|---|---|---|
| fenced `#[derive(Debug)]` above the anchor | RED — *"no line … begins "Three kinds of list stay hand-written""* | GREEN |
| fenced `#!/bin/sh` + `# a comment` below the list | RED — *"carries no "#### The lists that stay hand-written" heading"* | GREEN |
| backtick line inside a tilde fence | RED — same missing-anchor red | GREEN |
| worked table row written inside a fence | RED — *"row `GHOSTS` says `ghosts` declares"* | GREEN |
| closed fence, decoy roster outside the section | RED — same missing-anchor red | GREEN |
| `sort` dead | RED — got *the source enumerator over* | GREEN |
| `awk` dead | RED — got *the kinds reader over* | GREEN |
| kinds `sed` dead (consumes, then exits) | RED — got *the kinds reader over* | GREEN |
| `gate_rust_code`'s `awk` dead | GREEN | GREEN |
| `ITEM_AWK` dead (consumes, then exits) | GREEN | GREEN |

**Eight of ten are controls; the last two are declared coverage and not
evidence**, in the file as well as in the PR. The `gate_rust_code`
split REMOVED a wrong second name, and `gate_selftest_case` and its
broken-tool twin match a substring with no way to assert a string is
ABSENT — so a removal cannot be observed. That is the AFFORDANCE half
of `work/issues/gate-selftest-cannot-observe-the-identity-a-gate-names`,
restated at this gate rather than filed a second time; `lib.sh` was
read, called and not edited. Worth recording what the harness CAN do,
since the item is read as saying otherwise: it matches a substring of
the whole output and `--also` requires several, which is enough to
assert a reader's name and is what three of the controls above use. The
gap is absence, not identity.

**The `ITEM_AWK` case is the sharpest single finding.** Before this PR
the only case asserting *the const-item reader* actually killed
`gate_rust_code` — a different stage. The defect the item describes was
sitting inside the self-test written to hold it, with a `want` string
that was itself an instance of it.

**Operational, and out of fence: `mawk` 1.3.4 aborts its regex compiler
on an interval followed DIRECTLY by `(`.**
`REcompile() - panic: values still on machine stack`. The boundary was
derived, not guessed: `/^ {0,3}(a)/` and `/^a{2}(b)/` panic;
`/^ {0,3}-(a)/` and `/^(a){2}/` compile. `gawk` 5.2.1 compiles all
four. The natural spelling of *"three or more backticks or tildes"* is
exactly that shape, so the first draft took the gate down under mawk
with `exit 100` while staying green under gawk — and since the box's
`awk` is gawk now and the hosted runner's is too, nothing on either
lane would have shown it. This is a sibling of the directory's
*no backslash, use `[(]`* rule
(`loop-boundary-discards.sh:222-234`, `lib.sh:230-235`) and belongs
beside it; reported in #2282 rather than filed, because whether
`lib.sh`'s conventions block carries it is GATES' call. Two incidentals
worth having: the failure surfaced as *"the kinds scanner over
crates/viewer/README.md exited 100"* — this unit's own repair naming
the right stage on its first real use — and the plan's note that the
box's `awk` moved to gawk is exactly why the mawk run had to be done by
hand rather than assumed.

**`crates/viewer/README.md` was not touched.** The gate passes the real
tree unchanged, same `4 ratified … 3 kinds` line as before, under both
awks.

**The §5 sweep, and it found one sibling.** Both defects are classes,
so the shape was swept rather than the symbol: every gate that scans a
markdown section, and every `|| status=$?` / `|| reader_failed` sitting
on a multi-stage pipeline. **One hit outside the unit's file**, and it
is VIEW's own: `scripts/gates/viewer-module-kinds.sh:220-227`'s
`readme_table_modules` has BOTH — `:223`'s bare `^#` region end and
`:224`'s `inside && /^\|/`, plus two unguarded reader stages inside two
process substitutions (`:270`, `:277`) in a gate that has **no reader
guard apparatus at all**. Filed as
`module-kinds-table-scan-repeats-both-vocab-gate-reader-defects`, a
file rather than a sentence in a merged PR body.

**Two things the sweep learned that reading the vocab gate would not
have taught.** First, the callers there red on an EMPTY roster
(`:271-274`, `:278-282`) but never on a SHORT one, so defect 1 has a
worse direction at that gate than at this one: a fence part-way down a
table truncates the region, the rows below vanish, and the gate goes on
printing OK while enforcing its rule over fewer modules than the README
lists. Second, that same empty check means defect 2 there is a
MISDIAGNOSIS rather than a false green — a dead reader reds with *"the
heading was renamed or the table was reshaped"* about a README that is
fine. Both stated in the item, because *"the same defect next door"* is
the claim a reader would otherwise assume and it is not true in either
direction.

**What the sweep could not match**, stated because a blind spot left
unstated is not a negative result: it is a grep over one line at a
time, so a section scan whose `^#` rule is spelled across two awk rules,
or built by string concatenation before `awk` sees it, would not appear;
and it covers `scripts/gates/*.sh` only, so a markdown section scan
anywhere else under `scripts/` is outside it. The three `lib.sh` hits it
did return are single-stage pipelines and not this class — checked by
reading each, not by filtering on the path.

## 2026-09-10 — `view/gate-readers` fix pass: the fence repair had left a FALSE GREEN open

The review of #2282 found, and the orchestrator reproduced, a **false
green over an unratified fourth kind**. Reproduced here again before
anything was edited, both directions, plus the control that separates
them.

**A boolean fence answer is the wrong answer for one predicate, and
"ask it of every rule" is what made that look closed.** `readme_kinds`'s
`opens` does not ask *is this line markdown structure*; it asks *did the
previous line END a block*. An OPENING delimiter starts one, so the line
under it is content and the boolean is right. A CLOSING delimiter ENDS
one, so the line under it BEGINS a paragraph — and the boolean has that
backwards. `markdown-it-py` 4.2.0 in CommonMark mode emits `fence` then
`paragraph_open` for both plants, so markdown draws them as paragraphs
and the gate did not.

**Both directions were live on that one answer, one blank line apart.**
A second announcement DIRECTLY under a closing fence was not counted as
an announcement, so the gate found one anchor, read three kinds under it
and printed `OK`, exit 0, over a duplicate announcement AND an
unratified fourth kind bulleted beneath it. The same plant with a single
blank line between fence and announcement reds correctly. The other
direction is the anchor itself under a closing fence, which reds with
*"the paragraph … is gone"* — the exact misdiagnosis the fence work was
filed to remove.

**The repair:** `md_fenced` became `md_fence` and returns `open`,
`inside`, `close` or `""`. Callers wanting *is this markdown structure*
test `!= ""`; `opens` additionally counts `close` as ending a block. The
RENAME is the point — a contract change that a caller can miss is a
contract change that will be missed.

**Neither direction was a regression**, and the controls say so by
running against three trees rather than two:

| case | base `104f1445b` | first repair `7e70be4d3` | now |
|---|---|---|---|
| second announcement under a closing fence | RED — *"PASSED on a planted violation"* | RED — same | GREEN |
| the anchor directly under a closing fence | RED — *"the paragraph … is gone"* | RED — same | GREEN |
| the other ten rows | unchanged | unchanged | unchanged |

**The lesson, and it is not "add a case".** The header claimed both
readers *"ask this question of every rule they have"*, and that sentence
was TRUE and still insufficient — every rule got the answer, and one
rule needed a different question. **A helper that answers one question
well invites callers to assume it answers theirs**, so the thing to
check is not whether every caller consults it but whether any caller's
question is a different one. The three-answer return makes that
structural: `close` cannot be spelled as `!fenced` by accident.

**Three corrections to the last entry's own claims**, all from the
review and all confirmed here:

- *"their guards were already on their own stage"* was **false for stage
  6**: base `table_rows` was `printf | sed || status=$?`, a guard on a
  two-stage pipeline, which this unit moved into a brace group. The
  conclusion survives and the reason does not — the true reason no case
  is owed is that neither guard's NAME changed, and a case can only
  assert a name is PRESENT.
- The mawk paragraph said *"the other interval in this file"*. The
  sweep rule `grep -nE '[{][0-9]+,[0-9]*[}]' $0` returns **five lines
  carrying six intervals**; the paragraph now states the rule and both
  counts, which differ because two share a line. A universal without its
  sweep rule, in the paragraph whose whole job is to let a successor
  re-derive the hazard.
- A citation named the helper's argument as `:541-614`, which is the
  comment prose plus the assignment line; the argument is the function.

**The sweep's arm 2 was shaped like the symptom, and that is what let a
live false green sit unopened elsewhere.** `|| status=$?` /
`|| reader_failed` can only match a pipeline that **already has a
guard**, so it structurally cannot see one with none — which is exactly
the population with the worse direction. The right arm is the rule the
gate itself states: *every stage inside a process substitution whose
exit status the shell discards*, i.e. `grep -n '< <('` and read each.
Arm 1 was genuinely class-shaped and the reviewer's wider re-run (all of
`scripts/` and `local-scripts/`, `.sh` and `.py`) returned the same
single hit, which discharges the blind spot the last entry named.
Re-running arm 2 on the right rule is the orchestrator's, already done
and filed on **code-quality's** slate as
`gate-roster-and-probe-census-have-no-reader-guards` —
`scripts/gates/*` returned to code-quality when the `gates` program
closed. Named in prose and not in `refs:`, because it is not on `main`
yet and the reference would not resolve.

**The sibling row split in two**, on the test *can half of it be
closed?* — `module-kinds-table-scan-ends-at-any-column-zero-hash` is
`md_fence` plus a length check; `module-kinds-gate-has-no-reader-guards`
is apparatus that gate has never had. Different repairs, different
controls, and a row that can only be half-closed is what one-file-one-item
protects against.

**Trims, and what was kept.** The pipefail-blames-the-wrong-reader
argument had grown to three wordings; it now has one home in the header
and `viewer_sources` keeps only its local fact (the ORDER is a read, so
`sort` earns a guard). The reproduction narrative in `THE ANCHOR MADE IT
WORSE` and the stage-7 narrative both became invariants — comments state
the invariant, not the history, and the history is in this log. The
self-test's re-enumeration of the nine now points at the one home and
keeps only the technique a reader cannot derive: killing a right-hand
stage needs a shim that CONSUMES and then exits, or SIGPIPE names the
wrong reader. **Kept in full deliberately**: the mawk paragraph, whose
four probe regexes are the only record in the tree of why a spelling is
forbidden.

**Left alone, with the reason:** `md_fence` mutates `FENCE_CHAR` and
must be called once per line, held by convention. Caching on `NR` would
enforce it but adds state across four return paths in the file's most
delicate function; judged not free, and the single call site is the
first rule of each program where it is visible.

**Re-derived after merging `origin/main` at `d268d319b`, because #2278
moved this unit's SUBJECT.** That PR rewrote the absent-chooser
paragraph inside `### Closed vocabularies are declared once` — the
region this gate's readers scan — so the green above was taken on a
tree without it and the entry's own numbers were measurements of a
tree that had moved. Re-measured on the merged tree: **fourteen fence
lines, still `:3-262`, still seven balanced pairs, still none inside
any scanned region**, and the gate's verdict byte-identical. What DID
move is the section, `:930` → **`:958`**, carrying its anchor to
**`:1048`**; `gate-section-scans-…` is corrected, and this note is the
append-only half so the two cannot be read against each other. The
`### The drivers` region `module-kinds-table-scan-…` measures is
unchanged at `:290` — #2278's edit sits below it — which is why one
row moved and the other did not. **The rule this pays for is
`plan.md`'s**: green CI on an old head is not a merge criterion when
the diff's SUBJECT moved under it, and "the inputs are byte-identical"
was a true argument that stopped being true.

## 2026-09-10 — `view/module-kinds`: the sibling gate's two defects, and the one that was genuinely silent

Both rows split off #2282's §5 sweep, closed separately in one PR:
`module-kinds-table-scan-ends-at-any-column-zero-hash` (`md_fence` plus
a length check) and `module-kinds-gate-has-no-reader-guards` (apparatus
`scripts/gates/viewer-module-kinds.sh` never had). **Every claim below
was reproduced on a copy of the real tree before anything was edited**,
which is how three of them came out different from the rows.

**The fence question went to a SIDECAR, and the deciding fact is
`gate-roster.sh`.** The row says reuse `md_fence` rather than re-derive
it, and the honest options were `lib.sh` (out of fence), a shared
helper both viewer gates source, or a second copy. The middle one is
blocked in the spelling everyone would reach for:
`gate-roster.sh:139` derives the gate roster from `scripts/gates/*.sh`
and excludes exactly ONE member by name (`NOT_A_GATE=lib.sh`), so a
second sourced `.sh` fragment there reads as a gate that runs nowhere
and must be wired into both CI halves — and teaching that roster about
it means editing code-quality's file. `scripts/gates/viewer-readme-fence.awk`
is invisible to that glob, to `local-scripts/ci-local.sh:380`'s loop
and to `check-ci-mirror-parity.py`'s `SCRIPT_RE` (which matches `.sh`
and `.py` only), while `scripts/**` still widens the CI tier the same
way. So: **one copy of the tracker, no duplication to disclose**, and
`viewer-vocab-declared-once.sh` now loads it too. Both gates' loaders
are guarded and both self-tests call the load DIRECTLY, because no
`--root` tree can express a tracker that lives outside every fixture.

**A prepended comment is part of every program it is prepended to.**
The sidecar's header first said the tracker is loaded *"the way
`lib.sh` prepends `gate_record_split`"*, and the vocab gate's
const-item shim keys on exactly that string — so the kinds scanner died
instead and the case failed naming the wrong stage. Loud, not silent,
and the sidecar now says so at the site. A shared awk file is shared
TEXT, not just shared code.

**Three things the rows got wrong, all three read off the tree.**

- **The driver table's truncation is not silent.** Check 3 holds that
  roster against the tree in BOTH directions, so a fence four rows into
  `### The drivers` reds — as a misdiagnosis naming the seven modules
  the table lists BELOW the fence. Only the vocabulary tables have
  the quiet direction, because check 4 is one-directional: the same
  fence in `### The session's vocabularies` left the cross-check
  covering **2 of 6** and printed OK with a **byte-identical** line.
- **The unguarded population is twelve stages, not two.** The item's
  arm could only see `readme_table_modules`. The gate also has
  `find | sed | sort`, `awk | sed | tr | sort`, the kind extractor's
  `sed`, and the hit union's deduplicator and `sort`. A dead `sed`
  reported *"no modules under crates/viewer/src … besides lib.rs and
  bin/"* about a tree holding forty-five — a second live misdiagnosis
  the item does not name.
- **A status the shell KEEPS still buys no diagnosis.** Three stages
  sit in `$(…)`, so errexit ends the gate — with no gate name, no
  `::error::` framing and nothing said about what was undecided. Their
  base-side cases fail as *"exited non-zero WITHOUT a gate_error
  diagnosis"*, which is `lib.sh`'s S157 second half, and it is why they
  are guarded rather than left to `pipefail`.

**What the length check can honestly check is argued at the reader**
(`readme_table_block`'s header and `read_table_roster`'s two blocks),
and the short of it is that nothing in the gate knows how long the table
is SUPPOSED to be. `!stray` and not the fence tracker is what closes the
silent half.

**The control table.** Control = the gate at `09b0ef5a8` with the new
planters and a one-case dispatcher spliced in verbatim, in a tree
holding only `lib.sh`, the sidecar and the real manifest, so the only
difference between the columns is the reader. **All 27 fail on base**
and pass here — twenty-six because the base reader answers wrongly,
and one (the table row reader) because its shim keys on a program only
the new reader has, so on base nothing dies and the gate passes.

| case | base | head |
|---|---|---|
| a fence opening inside the table body | GREEN over an OVER-inclusive roster — the base reader has no fence rule, so it reads all six rows where `markdown_it` draws one | RED — *"is INTERRUPTED by a fenced code block"* |
| a row below a blank line | RED, wrong message | RED — *"table line(s) BELOW the table"* |
| a second table in the section | RED, wrong message | RED — same |
| the `\|---\|---\|` separator gone | GREEN | RED |
| the header columns reordered | GREEN | RED |
| a row whose module cell is not backticked | GREEN | RED |
| the table gone, the heading kept | RED, wrong message | RED — *"no table follows it"* |
| a header and a separator and no rows | RED, wrong message | RED — *"no rows under them"* |
| the heading twice | GREEN | RED |
| the driver heading renamed | RED, wrong message | RED — *"carries no … heading"* |
| a vocabulary heading renamed | RED, wrong message | RED — same |
| a fenced `#[derive]` above the table | RED (false) | GREEN |
| a worked table row inside a fence | RED (false) | GREEN |
| a backtick line inside a tilde fence | RED (false) | GREEN |
| a closed fence, section still ends | RED (false) | GREEN |
| `find` (module enumerator) | RED, wrong message | RED — names the stage |
| `sed` (module path trimmer) | RED, wrong message | RED — names the stage |
| `sort` (module sorter) | RED, wrong message | RED — names the stage |
| `awk` (table reader) | RED, wrong message | RED — names the stage |
| `sed` (table row reader) | GREEN — no stage on base carries that program | RED — names the stage |
| `sed` (kind extractor) | RED, NO diagnosis | RED — names the stage |
| `awk` (manifest feature reader) | RED, wrong message | RED — names the stage |
| `sed` (dep extractor) | RED, wrong message | RED — names the stage |
| `tr` (dep speller) | RED, wrong message | RED — names the stage |
| `sort` (dep sorter) | RED, wrong message | RED — names the stage |
| `awk` (hit deduplicator) | RED, NO diagnosis | RED — names the stage |
| `sort` (hit sorter) | RED, NO diagnosis | RED — names the stage |

**`crates/viewer/README.md` is untouched**, and it was checked rather
than assumed: each of the three scanned sections holds exactly ONE
contiguous run of `|` lines and no stray table line, so the contiguity
rule and the `!stray` answer are both green over the page as written.

**The census of the shifted bands.** No open row cites into
`viewer-module-kinds.sh`; the three that do
(`a-new-hand-written-all-table-meets-no-gate:33` → `:10-18`,
`pick-rename-left-two-live-sites…:95` → `:74`,
`pick-and-parts…:103` → `:156-159`) are all CLOSED, and the first two
are above the diff and unmoved anyway. In
`viewer-vocab-declared-once.sh` two open rows cite in:
`gate-rust-reader-splits-an-array-type-at-its-semicolon:65,88` →
`:139-153`, unmoved and re-read; and
`gate-selftest-cannot-observe-the-identity-a-gate-names`, whose
correction table already said `:930-973` — stale AGAIN since #2282, and
now `:1528-1597` over 27 rows. Corrected there with the reason, which
is that **a citation into a self-test's case list moves whenever anyone
adds a case**: it is the wrong half of a gate to cite by line, and this
is the second correction that row has needed in two days.

**Fix pass, same day: a hand-written test for a diagnosis matched ONE
of `gate_error`'s two spellings.** The first CI run red at
*"SELFTEST FAILED: load_fence_awk … failed without a gate_error
diagnosis"* — with the diagnosis printed two lines above it, in the
`::error::` form. `lib.sh:77-83` writes `ERROR: ` locally and
`::error::` under Actions, so `selftest_fence_load`'s own `case`
matched on a developer's box and could not match on the runner. Both
gates now call `gate_selftest_assert_diagnosed`, which is the one place
that knows both spellings and is what every other case in this
directory already goes through. **The class**: a self-test that
re-implements a check `lib.sh` already owns is a check that agrees with
it only by accident, and the environment where it disagrees is the one
nobody runs by hand. Re-verified under `GITHUB_ACTIONS=true` as well as
without it, and that is now the way to run a gate self-test before
pushing one.

## 2026-09-10 — `view/module-kinds` fix pass: the reader re-minted the defect it was written to remove

**The lead finding is the row's own headline, inside the unit that
closes it.** `readme_table_block` anchored every rule at `^`, so a table
row indented one to three spaces — a row every renderer draws — was seen
by no rule. Indenting `crates/viewer/README.md:326`'s `session::probe`
row by two spaces dropped that module out of the vocabulary roster with
the gate at **exit 0 and its whole output `cmp`-identical** to the clean
run. That is the silent short roster
`module-kinds-table-scan-ends-at-any-column-zero-hash` was filed for.

**The rule was already written down and this program earned it.**
`plan.md`: *a `^`-anchored pattern over markdown is a claim about column
zero that markdown does not make* — #2172, whose new reader escaped its
paragraph on `/^- /` and read three kinds where a two-space-indented
bullet renders as a fourth. Same reader family, same boundary, one
sentinel over: `^|` for `^- `. **A rule this program has paid for is not
a rule this program applies**, and the gap is that the rule was learned
about BULLETS and filed as being about bullets; nothing in it said
*every anchored pattern in every markdown reader here*.

**The repair is one strip, not a widened pattern.** `sub(/^ {0,3}/, "",
line)` once per line, before any rule, and every rule reads the stripped
line — heading match, `^#` region end and every `^|` test at once. Four
spaces keeps one space and matches nothing; a tab keeps its tab. Settled
with `markdown-it-py` 4.2.0 in CommonMark mode with tables on, over all
six positions rather than the one that broke: **the header, the delimiter
row and a body row each accept one to three spaces**, four spaces on the
header is a code block, and four on the DELIMITER leaves no table at all.

**Being right about four spaces is not being loud about it**, which is
the residue the strip alone leaves and `!indent` closes. A row the author
indented four spaces is correctly not read — and if it is the table's
last row, nothing follows it to become a `!stray`, so the roster is one
shorter and every other figure is unchanged. The reader now reports the
line and still does not read it as a row. Sweep:
`grep -nE '^[[:space:]]+[|]' crates/viewer/README.md` returns nothing
over the whole page.

**The OK line carries two README-derived counts now.** Every other
figure on it is tree- or manifest-derived, which is exactly *why* a
narrowed roster came back byte-identical; `${#driver_rows[@]}` and
`${#vocab_rows[@]}` cost nothing and make a narrowing visible in a log
even where it does not gate. It is not the fix and does not pretend to
be — it is what makes the next one of these findable.

| case | at `b37a67fb2` | here |
|---|---|---|
| a ghost row indented two spaces | **GREEN** — the row is invisible, the roster is one short | RED — *"is not a module in the tree"* |
| a real row indented four spaces | **GREEN** — one shorter, silently | RED — *"indented FOUR or more spaces"* |
| a real row indented with a TAB | **GREEN** — same | RED — same |
| a section heading indented three spaces | RED (false) — *"carries no heading"* | GREEN |

The ghost is what makes the first case observable at all: with the row
read the gate reds naming a module the tree does not hold, and with it
unread the gate is green — and no figure the gate prints could tell the
two apart before this pass. The other three are the upper side of the
boundary and are what keeps it from being widened to `[[:space:]]*`.
The original 27 rows were re-run against this head unchanged.

**Three claims of mine that were false, and one was false in three
places.** *"It needs the tracker's THIRD answer"* is not true at this
gate: `close` and `inside` are unreachable where a table body ends, so
`fence != ""` decides identically — mutated and confirmed green on the
fixtures and on the real tree, while returning `open` where `md_fence`
returns `close` reds the VOCAB gate's self-test and not this one. The
third answer is load-bearing for `opens` and for nothing else, and the
sidecar now says which caller holds it. The receipt *"red against four
modules"* over a 3-of-10 roster was arithmetically impossible: re-run,
it is **seven** — 10 − 3 — and a receipt is a citation. And the control
table's first row gave the opposite reason: the base reader has no fence
rule at all, so its error there is over-INCLUSION, six rows where
`markdown_it` draws one.

**The duplication I told the orchestrator did not exist.** True of the
tracker, false of what loads it: `load_fence_awk`, `VIEWER_FENCE_AWK`
and `selftest_fence_load` are spelled twice, the last byte-identically.
It is irreducible in fence — the tracker has one home because it is
AWK, while a loader is SHELL, and the only shared shell homes are
`lib.sh` (code-quality's) and a second `.sh` here, which `gate-roster.sh`
reads as a gate that runs nowhere. So it is **disclosed at both sites**
with that reason and with what bounds it: a path, an existence test and
a message. **"One copy of the tracker" was a true sentence doing the
work of a false one** — the claim a reader takes from it is that nothing
about the tracker is duplicated.

**Two prose claims corrected while they were being read.** The
*"only two things in this file are hand-kept"* sentence was already off
by one before this unit and this unit added two more; it is now four,
with the rule that produces them (`grep -nE '^[A-Z_]+=' $0`) and with
`VIEWER_FENCE_AWK` marked as the one held by nothing but its own
existence test — a path cannot rot silently, which is what makes it not
check 6b's class. And the FIFTEEN STAGES census certified a population
it did not produce: at least nine more `gate_grep`/`gate_rust_code`
stages exist, and it sited `gate_rust_code` under the wrong assignment.
It is now TWELVE with its rule stated — *every stage on PATH whose
status this file must read for itself* — and the self-diagnosing ones
named so their absence does not read as an oversight.

**Also unstated and now listed**: setext headings are a fourth
structure the reader does not model (`grep -nE '^(=+|-+)$'` over the
README: nothing), alongside HTML blocks and block quotes.

**The `unsure` the reviewer left dissolved rather than being fixed.**
`!row` is asked before `!stray`, so indenting a table's FIRST data row
used to red about a table having no rows when it had two. With the
strip, a one-to-three-space row IS a row and the case cannot arise; a
four-space one is reported by `!indent`, which is asked before both.

## 2026-09-10 — `view/wasm-docs`: the repo had already answered this one axis over

`viewer-docs-do-not-build-at-wasm32` offered two shapes over nine broken
intra-doc links at `wasm32-unknown-unknown`. The split is seven-a-cost,
two-dead-links, but the unit's substance is that **neither the row nor
this lane's first close cited the precedent**:
`scripts/doc-gate.sh:243-262` already rules the isomorphic problem on the
FEATURE axis — with F off every link into F-gated code is unresolvable BY
CONSTRUCTION, answered by allowing `rustdoc::broken_intra_doc_links` for
that pass only (`RUSTDOC_LINTS_INERT`, `:559`), population enumerated
complete, line numbers deliberately not carried.
`crates/viewer/README.md`'s **Rustdoc posture** is now that shape with
`target_family` for `feature`. The first close reached the same verdict
on the seven by a worse argument — *the browser docs have no reader* —
where the reason is that the lint cannot tell *this link is broken* from
*this link's target is in the other half*.

**Re-derived first.** The row's command at `ac4a69dd5` printed nine
errors whose `-->` lines matched the filed table cell for cell, every
third-column citation and `doc-gate.sh:555`/`:638` resolving.

**The classifier the first close shipped was a proxy, and a reviewer
caught it.** It read the `cfg` on the item a doc comment is attached to.
`WebStartupError` carries that `cfg`; its five variant doc comments carry
none, so the test called them unconditional and permitted exactly the
brackets it meant to forbid — #2278's shape again, a rule over an
attribute where the claim is over existence at a target. The test is now
page existence in rustdoc's own output, which also dissolves the case the
dichotomy missed (`app.rs:41` sits on `pub mod app`, `cfg(feature =
"app")`, which the host pass documents).

**The two `run` links were measured.** Host renders `app/fn.run.html` and
no `app/fn.run_web.html`; wasm the reverse — the link resolved on no page
any pass renders. Repaired within the line, so `app.rs` is 1,956 lines
before and after. `indexer`'s `# Errors` was repaired in four lines for
the same reason: it does NOT inherit `evaluator`'s infallibility sentence
(separate rendered section), and a +1 shift there would have broken
CHROME's and CIW's `app.rs` citations, which §6 forbids fixing from here.

**Two further corrections.** `--bins --examples` at wasm32 does not
merely add no site, it does not compile (`E0432`,
`examples/r1_e2e.rs:19:37`); "cannot" was too strong. And the new row's
class is links, not doc comments — `bin/viewer.rs` has five such doc
comments and no link.

**The count moved out of this entry and out of a title.** The first close
claimed it lived only in the closed row; false, since that row's TITLE
carried the number `work/STATUS.md` renders and the new row carried live
figures. The README owns the population now, dated, by identifier, no
line numbers; the row's table is kept as the superseded reading.

**The intermediate was priced and wins.** A browser pass with that one
lint allowed costs nothing per site, buys what pass 3 buys, and is clean
today at lib scope; its precondition is the `E0432`. `cfg_attr` is
refused on precedent, not price: it closes a blind spot the feature axis
accepts permanently (#1317), and doc-gate's rejected *per-root deny list*
is the same argument one level coarser.

**Census and verification.** README grew at `:1428` and the highest
README line cited anywhere is `:1079`, so nothing moved; `app.rs`
unchanged. Seven errors after, quoted in the PR; `doc-gate.sh
--selftest` and `--pr --scope '-p viewer'` green, fmt clean, both
README-parsing gates green, clippy green at host and wasm32 at
`-D warnings`, `tests/all.rs` 508/0. The one local red is the WGPU
adapter row, green hosted.

## 2026-09-10 — `view/theme-store`: the theme picker says what it keeps

`wasm-theme-choice-is-offered-and-silently-not-kept` closed. A user
picked a theme, it applied, the session ended and the default came
back; `ViewerApp::remember_theme` returned early on
`!self.store.usable()` and said nothing, and the reporting arm below
that guard was reachable only from the `save` the guard had skipped.

**The channel was decided by the ratified rule and not by taste.**
`store.usable()` is settled when the store is built — either arm of the
`cfg` can answer false — and answers the same on every frame after, so
the sentence exists on a frame where nobody acted. That is provenance's
own visible test, and Ev's ruling on
`was-the-status-route-supposed-to-fire-for-an-absent-chooser` supplies
the negative half: a whole-run environmental fact has no correct
sentence on a line carrying one frame's news. So `frame::prefs_badge`,
`Subject::Preferences`, `Tone::Advisory`, `Affordance::Read`, drawn
beside the picker rather than in the badge run above it, because it is
the only badge about a CONTROL. It is drawn from the first frame, which
is a better reading of *once and not on every switch* than the item's
own: a reader learns it before spending a choice, not after.

**Annotated, not disabled**, which is where this parts company with the
chooser it was filed against: a dialog with no backend can do nothing;
the picker applies the theme and loses only the memory.

**The fork on the two dead refusals went the second way, and what went
was the refusals as independent prose.** Neither `save` could drop its
refusal — a store that keeps nothing has no honest `Ok(())` — so the
condition is stated by the party that knows it. `usable() -> bool`
became `unusable() -> Option<Unusable>`; the chrome renders those words
and `Unusable::refusal` renders them for a caller that saves without
asking. Two hand-written refusals for an unspoken condition became zero
and one value a reader sees. **What that buys is that the two
renderings cannot DIVERGE, not that the condition has one spelling** —
the suite asserts them equal, so a second spelling reds when it says
something different and is green while it says the same thing, which
was measured rather than assumed. The guard stays and is now
load-bearing rather than silent — it keeps a whole-run fact off the
outcome channel — and it says so where it stands.

**A bool could not have carried this.** With `usable() -> bool` the only
party that knew WHY was the store and the only party with a person to
tell was the caller, so any sentence a reader saw had to be composed
where the reason was not. That is the same shape as the census failure
this program keeps meeting from the other end.

**Mutation, not reading.** Four perturbations of the doors, each red:
`FileStore` claiming it is usable (2 rows), `Absent::save` re-wording
its refusal (2 rows), the badge's subject and tone (2 rows), and the
badge speaking for a store that is fine (1 row). What is NOT held is
the wiring —
`ViewerApp` cannot be built in the suite, so the guard and the draw are
read by eye. Said plainly rather than implied, and filed as
`the-guard-that-decides-whether-a-preference-is-kept-has-no-test`,
which is `hover-route-for-an-absent-chooser-has-no-test`'s boundary
with a second instance that differs in kind: a control-flow decision, not
a tooltip.

**Both universals carry their sweep rule.** The badge family's
population is every `frame` fn returning `Option<Badge>` — eight —
which ranges over the property rather than the `_badge` naming
convention it agrees with today, and is complete because `Badge`'s
fields and its three constructors are private to `frame`. The store's
is every read of `app::ViewerApp::store`, a private field — **three**,
all in `app.rs` — complete because `prefs_store()` has one caller.
That said four until a reviewer ran it: `store.load()` in the
constructor reads the LOCAL binding, before the struct literal that
makes the field exist, so a rule ranging over the field had a count
ranging over the name. Both numbers are now held by a `#[test]` that
re-derives them from the sources and reads the README's word, rather
than by the sentences that state them. Neither rule ranges over
`target_family`, which is the half-fix the item predicted.

**Census of the shifted bands — the first one was wrong, and wrong in
the way the rule exists to catch.** The entry that stood here claimed
*"every `file:line` in `work/` landing at or past those lines was
enumerated"*. The pattern behind it required a full `crates/viewer/...`
path, so it could not see a bare `frame.rs:1747-1749` or a backtick
continuation `` `:1194` `` — the two spellings this tracker actually
uses most. It found 19 citations; the population is **122**. A sentence
that certifies a population it does not produce tells the next reader
to stop looking, which is the whole of why the rule is written.

The corrected rule: a file token naming any of the six changed files
(bare or fully qualified), plus every backtick-quoted `:NNN`
continuation attributed to the last file token on the line, over every
live row as it stood at the merge base. **Its own blind spot, stated:**
a bare `frame.rs` or `prefs.rs` is ambiguous across crates and only the
sentence disambiguates it — eight hits in two rows are `geom-core`'s
`linalg/frame.rs` and `src/camera.rs`, caught by reading and not by the
pattern.

**122 citations, 25 rows, disposed as:**

| n | rows | disposition |
|---|---|---|
| 33 | 13 | **repointed** — the subject was at the base line, so my diff moved it |
| 34 | 11 | left: **already wrong at the base line**, and repointing manufactures a fresher wrong number |
| 26 | 1 | this unit's own closed row, left as written and naming its SHA |
| 18 | 1 | `stale-file-citations-after-the-split`, left whole |
| 8 | 2 | another crate's file — the pattern's blind spot |
| 3 | 3 | unmoved (the band's edge) |

**Text identity and subject are two different questions and only one
instrument answers each.** The first entry offered *"checked by TEXT
IDENTITY at the new line rather than by trusting a delta"* as the
stronger check. It is not stronger, it is orthogonal: identity at the
new line proves the MAPPING is right, and says nothing about whether
the citation was ever right. Five of the first pass's nineteen passed
identity at both endpoints and landed on text that is not the row's
subject, because they were broken before this branch existed. The
instrument that catches those is #2083's — read the line and ask
whether the subject is there — and it has to be run at the BASE line,
which is where the row was written. Both are run now, in that order,
and the five are reverted to the numbers they had.

**`stale-file-citations-after-the-split` is left whole, deliberately.**
Every number in it is either a stale citation quoted as evidence
(*"what it cites"*) or a correction qualified by a named SHA (*"is
`:925` at #2272's head"*). Repointing the first kind destroys the
evidence; repointing the second makes the record false. Half-fixing a
file whose numbers are half evidence and half record is the
contradicts-itself defect one level up, so its re-derivation is that
row's own work.

**Past EOF, and not this diff's.** 31 citations in 11 live rows name
`app.rs` beyond its 1,985 lines, spanning `work/chrome/`, `work/ciw/`,
`work/code-quality/`, `work/fix/`, `work/issues/` and `work/view/`.
They were already past EOF at the merge base — that file has been 1,985
lines since the split — and they are that row's population. The first
entry said *"seven, in five rows across `work/chrome/` and
`work/fix/`"*, which was the same pattern's blind spot again.

**Verification.** fmt clean; clippy `-p viewer --features app
--all-targets` green; wasm32 clippy green at `-D warnings` (the hosted
row still runs as `cargo check`); `doc-gate.sh` OK; both
README-parsing gates green — module kinds 10/10 and 9 tabulated
vocabularies, vocab declared-once 4 ratified; `no-ambient-env` OK;
`work.py lint` clean. **`tests/all.rs` 511/0/1 against 508 at the
merge base** — three new rows, two on the stores and one holding the
README's two counts — and the count is checked against this branch's
own base rather than against a number in a brief. The one local red is
`gpu::…every_pass_builds_on_a_real_device`, which wants a WGPU adapter
this box has none of and is green hosted.

**Ten mutations, all red.** Four on the doors (a store claiming it is
usable; `Absent::save` re-wording its refusal; the badge's subject and
tone; the badge speaking for a healthy store), four on the new count
guard (the README's word wrong in either population, a ninth badge
door, a fourth field read), and — the one that came back GREEN and is
recorded as a limit rather than a proof — an identical literal written
back at `Absent::save`, which the equality assertion cannot see. Two
deletions were also run and stayed green at 511: the guard line and
the badge draw, which is the disclosure that has its own file.

**The guard that holds a count is itself a source reader, and it
arrived without registering.** `gate ok` went red on three jobs — the
`1/2` shard at all three eps values, so deterministic rather than
eps-dependent — on one row:
`test-utils::reader_census::every_site_that_reads_rust_source_is_in_the_ledger`.
`the_readme_counts_its_two_populations_correctly` reads `src/frame.rs`
and `src/app.rs` through `test_utils::source::code_only`, and that file
keeps one line per site that does. Registered as
`crates/viewer/tests/frame_policy.rs`, `Shared`, *"the README's
badge-door and store-read counts, code view"*, in the ledger's sort.
The census's header names three honest dispositions and this is the
first of them; the third — a new hand-rolled reader — is the one it
refuses, and the shared lexer was used from the start.

**Announced, not silent.** `crates/test-utils/*` is S-TCOST's and
`crates/viewer/tests/*` is S-TCOST's and Track W's by declaration. The
act is sanctioned rather than a crossing on two clauses read here
rather than inherited: CIW's `keep_out` scopes S-TCOST's claim there to
*"the Shared ledger row's AUDIT"*, and TOPO's says D261 converts *"its
own census entries"* — so a program registering its own arriving reader
is the mechanism working, and the audit of that row stays S-TCOST's.
The line is one `Shared` entry and touches nothing else in the file.

**Running the crate's own suite is not running the suite**, and this is
the rule the wave earned. `cargo test -p viewer --features app --test
all` came back 511/0/1 and was never going to see this: the row that
fired lives in another crate's test binary. **A guard that reaches
outside its crate is held by a row outside it too** — and the shape
generalises past readers, because this same unit registered a new badge
in a README count and a new store read in another. The receipt to run
is the workspace suite as CI runs it, and a report names the COMMAND as
well as the number.

## `view/dead-symbols` — the doc-comment dead-name class, closed (2026-09-10)

`doc-comments-name-symbols-that-do-not-exist`, closed in place. The
item's two figures both reproduce exactly at the SHA it states them at
— 19 spans / 12 names / 2 undefined with `<mod>` restricted to
`frame`/`session`, and 132 / 97 unrestricted — and the unrestricted
one needed its wording pinned before it would: *"every module prefix
is admitted"* means every **module-shaped** (lowercase) prefix. Admit
type-qualified prefixes as well and it is 284/197; restrict to this
crate's own modules and it is 66/49. Only the middle reading gives
132/97, so that is what the sentence means.

**The whole decidable set was taken, not the two names.** 64 spans
over 47 names at the merge base — every unbracketed `<own-mod>::<path>`
span in a doc comment under `crates/viewer/src`. 44 are now
`[`crate::…`]` links; 20 are named rather than linked, and the
undefined count is **0**. The item scoped its own decidability
argument to 12 names; it applies to all 49, and blind spot 5's
*"85 of the 97"* should read 48.

**The ruling's test came back wrong at 20 sites, and the gate is what
said so.** `crates/viewer/README.md`'s *Rustdoc posture: the host pass
is the gate* asks whether the HOST pass renders a page for the item the
doc comment sits on. Asked that way all 64 answer *it does*, and the
`--all-features` host pass agreed at zero errors — then
`scripts/doc-gate.sh` reded on **13**, because it documents `viewer` a
second time at DEFAULT features under `--skip-viewer-toolkit`, also at
`-D warnings`, where `app`/`forms`/`pane` do not exist. Chasing that
also turned up **7 spans inside `#[cfg(test)]` modules**, which no pass
renders, so bracketing them is inert. **That one is this lane's own
mis-reading and not a gap**: `cargo doc` does not set `cfg(test)`, so
the page is absent and the ruling's literal answer is *it does not* —
the remedy taken. The reason all 64 first answered *it does* is that
the question was asked of the MODULE rather than of the ITEM the doc
comment sits on, which is not what the ruling says. **One** gap
survives, the feature axis, filed as
`rustdoc-posture-test-names-one-axis-of-three` rather than edited into
ratified text that merged this morning. A row is better for being one
finding than three.

**The trap never fired, for reasons of population**, and the receipt
for the whole disposition is the browser pass: **7 unresolved links
over 4 identifiers** after the diff — the README's dated population,
unchanged. 44 new links, zero new browser-pass errors.

**A third dead name, and the bracketing found it.** `frame.rs:216`
named `pane::viewport::viewport_ui`; bracketed, the host pass reded.
`viewport_ui` is an inherent method on `ViewerBehavior`, an `app`
item merely written in `pane/viewport.rs:58`, so the module path names
nothing. The item's own rule could never have found it — the leaf IS
declared under `crates/viewer/src`, so a declaration regex resolves it
and it reads as live. Only a resolver that checks the PATH sees it,
and bracketing borrows rustdoc's. That is candidate 1's argument
demonstrated rather than asserted.

**The item's central claim is wrong, and correcting it strengthens
the case.** The title said the two names *"have never existed"*, and
`stale-file-citations-after-the-split` built a claimed **fourth**
citation class on it — *a subject that was never there*, distinct from
class 1, *a subject that is gone*. `git log -S` over `crates/` refutes
it: `supersession_notice` and `dropped_hide_notice` were both `pub
fn`s in `frame.rs` from `6877a40ff` until `4db112ada` — **#1957**,
which replaced them with the `Withdrawal` vocabulary, rewrote the call
sites and left every prose mention behind. The fourth class does not
exist. `four-badges-five-spellings.md:107` turns out to be the proof
rather than the oddity: it cites `frame.rs:232`, and `6877a40ff` puts
`supersession_notice` at exactly `frame.rs:232`. An invented name is
one author's slip; a deleted one is a rename that outran its prose,
and a bracketed link would have reded #1957 on its own branch.

**A dated sentence is not a stale citation.** The tracker count also
reproduces exactly — 15 occurrences across 8 items, 4 in open rows —
and three of the four were repaired. The fourth,
`frame-module-…-no-holds-row.md:76`, is dated *"(#1886, 2026-09-05)"*
and all three names it uses were real on that date, so it is left as
written because it is TRUE. Believing the *"never existed"* claim, the
honest move would have been to correct it, and correcting it would
have falsified a true record. The class is *present-tense claim*, not
*dead name*.

**No line shifts, and the receipt is `origin/main...HEAD`: 11 files,
+40/-40.** Every edit within-line, every file's line count identical to
the merge base. The figure to quote is that one — an earlier draft said
58/58 across 16 files, which is the first commit alone and was partly
reverted by the second; a receipt is a citation and gets no exemption.
The unconditional argument is better than either: **every changed line
under `crates/viewer/src` is a comment line**, zero non-comment lines,
so the diff cannot move a compile result at any target or feature set.

**The disposition is durable in a tracker file and nowhere else, and
that is a defect this lane created.** Eleven of the thirteen
feature-axis sites now carry a bare span with nothing saying it is
deliberate — `frame.rs:6,85,216,385,554,1766,1802`, `pickindex.rs:12,13`,
`props.rs:40`, `tree.rs:278` — in a crate whose `theme.rs:9-12`,
`vocab.rs:51-52` and `forms.rs:18-20` all explain exactly this choice
in prose. The sharpest is `pickindex.rs:12-13`, where a bracketed
`[`crate::marks`]` and a bare `pane::viewport` sit in one sentence and
a reader repairing the "inconsistency" reds the gate. Not fixed here:
the note adds lines, and `frame.rs`/`pickindex.rs`/`props.rs`/`tree.rs`
carry **155** `file:line` citations between them, so it costs a census
and is its own unit —
`named-not-linked-is-a-silent-disposition-at-eleven-of-thirteen-sites`.

**The revision the ruling wants is EXISTENTIAL, and getting that
quantifier wrong is how this lane nearly shipped a rule against its own
diff.** The first draft proposed *"every rustdoc pass that runs at
`-D warnings` renders a page … and can resolve the target"*, one line
after claiming it changed no disposition. It changes 25 of them: only
**19** of the 44 links are in modules the default-features pass renders
at all, and the other 25 sit in `app`, `forms`, `pane` and `widgets`,
which it never renders. A link checked by ONE pass is a checked claim;
demanding every pass check it forbids linking anything feature-gated.
The row now proposes *some* pass, and says why.

**Three residues filed rather than disclosed**, the two above and
`comment-symbol-names-outside-rustdocs-reach-have-no-gate`. Rustdoc
reads `///` and `//!` and nothing else, so the same dead name at
`app.rs:950` — a plain `//` comment — could only be corrected by hand
and is held by nothing afterwards. 26 plain-`//` own-module names under
`crates/viewer/src`, every one live today, and zero split-span ones: a
clean population with no gate holding it clean.

## 2026-09-11 — `view/cancel-doors`: both gesture cancels get a door, and the stranding is traced

`gesture-drags-have-no-cancel-door` closed. `SessionOp::CancelGesture`
and `SessionOp::CancelFreeMove` had an arm in `perform`, coverage in six
suites and **no emitter in the crate**; they have one each now, composed
as `DocSession::cancel_doors` and drawn in the toolbar beside Undo and
Redo.

**The item left reachability open and named three candidates; it is the
first of them, and it needs nothing a second pointer or a relayout would
have to supply. The drag's own preview strands it.** `slot_rows`
answers nothing when `standing().live()` is false, and a face whose name
did not resolve is not live; every frame a drag moves submits its scratch
document, so a preview taking an extrude's distance to zero lands an
evaluation the picked face does not survive, the panel is handed no
rows, and the field whose `drag_stopped()` is the drag's only exit is
not drawn on the release frame. The item could not find this because it
went looking for an OPERATION that changes what the panel draws and
correctly found all of them click-driven. The drag is not another
operation. Held end to end at the session layer; the last link — a group
absent from the list is not drawn, `properties_ui`'s
`for group in &groups` — is read and not run, because the crate has no
headless egui harness, and the row says so in as many words.

**The item's sharpest claim names the wrong sentence, and the shape it
describes is real.** `DisplayFault::FreeMoveInFlight` (*"finish the
free-move first"*) cannot be shown to a user at all: reaching it needs a
free-move already in flight, which needs a free-move strand nothing has
traced — the probe's field is drawn off the shown document, and a
document change under an in-flight probe is pruned rather than
stranded. The reachable inverted refusal is `Refusal::GestureInFlight`,
*"finish the drag first"*, which the trace above reaches with no pointer
behind the drag. Filed as
`free-move-in-flight-refusal-has-no-reachable-producer`; the free-move
door is owed either way, on the emitter count alone.

**Where the door went, and why not a key.** The toolbar, because the
defect is that the chrome owning the gesture can stop being drawn — a
cancel sited beside the field would vanish with the exit it replaces.
`input.rs` could not hold a key for it whatever we decided: it maps what
the pointer did inside the VIEWPORT, and `input::PRESETS` records that
this crate binds no key to any operation anywhere and what a keyboard
vocabulary would have to settle first. So the item's "no Escape binding
in `input.rs`" points at a module with no room for one, and the key is a
decision rather than a row.

**Both doors are one composition**, `CancelDoor::of(label, op,
in_flight, refused)`, and out of flight each carries the `Refusal` its
own operation answers with rather than a sentence written beside the
button — the fix `environmental-facts-answer-usable-as-a-bool-with-the-
reason-elsewhere` is open about one facility over, applied prospectively
here. Drawn in every state, disabled with that reason when there is
nothing to cancel, which is the posture the two file-dialog controls
take. `DisplayState::probing` gains its first production reader.

**The census is a match over `SessionOp`, not a search for `Cancel`.**
`CancelEvaluation` is spelled `Cancel` and cancels a run, so a
name-shaped rule would hand it a gesture door and keep agreeing with
itself. `every_gesture_cancel_has_a_chrome_door` runs the exhaustive
predicate against the door list both ways, so a third gesture cannot
join the enum with no door and a door cannot exist with no operation
behind it.

**Six mutations, each reverted**: the drag's door removed (census +
strand rows red), `perform`'s cancel arm no longer taking the gesture
(strand), `blocked` inverted (three rows), the toolbar loop deleted
(`the_cancel_doors_have_a_reader_in_the_chrome`), `CancelFreeMove`
marked not-a-gesture-cancel (census), and `slot_rows`' dead-standing
guard removed — the last so that the reachability half is not green over
its own absence.

**A new source reader, registered.** The chrome half of the claim cannot
be executed, so the emitter count is held as text: one read of
`cancel_doors` under `crates/viewer/src`. That made `gesture_table.rs` a
source reader and `reader_census.rs` red until its ledger line was
written — the mechanism working, announced rather than landed quietly,
and the row's own three dispositions say who writes the line.

**The citation census of the bands this diff shifted, and a refinement
the rule needs.** `session.rs` +39 from `:595`, `app.rs` **+28** from
`:1241` (the hunk is `@@ -1238,6 +1238,34 @@`, 28 added and 0 removed —
+26 was the pre-`cargo fmt` figure and is the number this entry first
carried, fixed here because the log is the artifact that survives),
`README.md` **+58** from `:977` (+42 before the fix pass grew the
clause; both figures are of the same one hunk at `:977`, and the only
in-band `README.md` citations are in a CLOSED row and in this log, so
neither number reaches a repoint), `reader_census.rs` +4 from `:306`;
`op.rs` and `gesture_table.rs` grew at EOF and shifted nothing.
**Re-swept at the merge, and one band moved**: `origin/main` (`9893bdcdb`)
added a ledger entry of its own, so `reader_census.rs` is **+8** from
`:306` against `8cf86ec32` — four lines this branch's and four not. The
out-of-fence rows citing into it are owed the +8, not the +4 this entry
first carried, which is the *a sweep is accurate as of your merge base,
not your merge* rule collecting on a lane that ran for one day. The
instrument was per-citation text identity — `base[i]` against
`head[i+shift]`, machine-checked — which **certifies the mapping and
says nothing about the subject**, and that is stated rather than dressed
up as a re-derivation. **Eleven open rows repointed.** The enumeration
rule is *files under `work/view/` this branch MODIFIES, less this log
and less the row being closed* — every one of those eleven was modified
because a citation in it moved, and nothing else was. Sixteen was the
POPULATION rather than the count of repoints (the eleven, plus the four
declared exclusions below, plus the closed row), and stood one line
above a paragraph saying four were deliberately left — a number
contradicting its own next sentence. The eleven carry their bare
`:NNN` continuations and, for
`four-debug-walks-are-spelled-and-placed-two-ways`, the two prose counts
its own stated rule derives (`1,780` → `1,819`, `2,000-line` →
`2,039-line`, title included — a count fixed in one place contradicts
itself).

**Four rows in the population were deliberately NOT repointed, and this
is the refinement**: a row whose SUBJECT is citations must be left
alone. `stale-file-citations-after-the-split` holds a `Was | Now` table
of numbers that were wrong; `viewer-preview-names-a-verb-by-its-variant-
identifier` quotes another program's citations and says "at their
pre-split paths"; `sweep-blind-spots-the-precheck-sweep-could-not-see`
is a receipt pinned to `8604dfb3`; `the-citation-receipts-summary-
numbers-are-not-re-derivable` audits a receipt's numbers. Shifting any
of them re-mints `citation-repoint-shifted-a-number-the-lane-knew-was-
wrong` — and the first draft of this lane's census did shift all four
before the diff was read. Closed rows were excluded for the same reason:
a closed row is a record.

**Out of fence, reported and not filed** (implementer-discipline §6):
the same shift is owed by in-band citations in `work/chrome/`
(`app-rs-doc-comment-merge-scars`, `placed-union-has-no-session-op`),
`work/ciw/` (`gui-wasm-build-is-not-gated-at-all`,
`tree-wide-guards-outside-the-change-closure`), `work/docm/`
(`check-registry-gathers-product-twice`,
`docm1-face-frame-owes-a-reader-census-ledger-line`), `work/fix/`
(`boolean-error-has-no-fieldless-kind`,
`verb-and-dimension-render-through-debug`), `work/instr/`
(`baseline-census-partition-assert-cannot-fail`), `work/tcost/`
(`source-lacks-an-item-body-carve-and-shared-means-any-mention`) and
`work/issues/tracker-file-line-citations-measured`. Each is in the PR
body with its old and new number.

**Residue, filed**: `preview-and-commit-carry-no-gesture-identity` — in
the stranded state a drag on any OTHER field is refused its begin and
then previews and commits into the stranded slot, observed with the
extrude's `Distance` taking a datum origin's number. The door is a way
out and does not repair that. Plus the `FreeMoveInFlight` row above.

**An operational near-miss worth the line**: `git checkout -- <file>`
to revert a mutation discarded the lane's own uncommitted work, because
nothing was staged. Commit before mutating; the mutation evidence here
was taken against a committed tree.

### Fix pass on #2320 — what the review found, and what it cost

Mergeable on the verdict; five record defects and three reports. The
review reproduced all six mutations and added two of its own on the
agreement row (mutating `perform`'s refusal, then the door's), ran the
citation population sweep independently and got exactly this lane's four
declared exclusions, and put an instrument on the `input::PRESETS`
universal rather than reading the prose.

**Four of the five were in the RECORD, not the code, and that is the
lesson.** A repoint that should have been a revert
(`four-debug-walks…:33` — `session.rs:1991` is a blank line, the
`Debug for DocSession` subject is at `:2010`, and `origin/main:1952` was
blank too, so it was wrong at the merge base and this lane's own stated
rule says revert; that file's `:46-50` also declares its `std::fmt`
citations left as written, and this was one of them). The log's own
`+26` where the PR body carried the corrected `+28`. "Sixteen open rows
repointed" where eleven were, one line above the paragraph naming the
other four. And two citations in the closed row left at `properties.rs`
`:100`/`:557` when the `CommitGesture` pushes are at `:103`/`:560` —
the half-fixed-file shape, in a row whose other citations this lane DID
re-derive. **A lane that repoints thirty citations correctly and leaves
four wrong has produced a file a reader cannot trust**, which is the
cost the class has always had; the instrument that caught all four was a
reviewer re-deriving by hand, again.

**The fifth was a false precedent in ratified text.** The README clause
and `CancelDoor`'s docs both cited `frame::ChooserBackend`'s two dialog
controls for the whole posture. They are the precedent for *drawn in
every state* and the **counter-example** for *typed*: they hand
`frame::NO_CHOOSER_BACKEND`, a `&'static str` composed at each button
(`app.rs:1198`, `:1217`), to `on_disabled_hover_text`. The precedent for
the typed half was in this crate and uncited —
`pane/create.rs:248-259`, *"carrying the op's own refusal — read off
the entry, not minted here."* Both texts now cite one for each half, and the
clause says which of its universals is held by a TEST
(`a_closed_door_says_what_its_own_operation_refuses`) rather than by the
type, because nothing structural stops a future door composing its own
sentence.

**The clause's "drawn in every state" is now scoped to the states it was
checked against** — the selection, the standing and the evaluation. The
toolbar is one non-wrapping `ui.horizontal` (`app.rs:1148`) and the row
now holds twelve controls, two of them this unit's, so a narrow window
can push them out of reach. Nobody measured it and nobody can here;
`the-toolbar-row-does-not-wrap` holds the question and says in its own
`## What is NOT established` that the clipping is egui's documented
rule, not an observation of this toolbar.

**Two reports taken as files rather than as sentences.**
`a-disabled-control-says-why-in-four-shapes` — the review found the
cancel door is the fourth spelling of *a control a reader cannot use
that says why*, and the sweep rule it owes is over that DISPOSITION and
not over `on_disabled_hover_text`, because two members (a
`blocked: Option<&'static str>` in `pane/create.rs:445` sharing this
unit's field name with the opposite typing, and `properties.rs:347-352`)
do not call it at all. And
`the-new-document-button-states-its-refusal-twice`, eighteen lines above
these doors: a comment claiming `Refusal::EmptyName` backs a disabled
button whose tooltip is a literal saying something else.

**A correction to the dispatch that every later lane needs: the expected
WGPU red is in the `--lib` target, not `--test all`.**
`cargo test -p viewer --features app --test all` is **517 passed / 0
failed / 1 ignored** and carries no `gpu` row at all; the adapter row is
`cargo test -p viewer --features app --lib`. A lane told to expect one
red and running only `--test all` gets a clean number that means
something else entirely — *running the crate's own suite is not running
the suite*, one target deeper. `cargo nextest run -p viewer --features
app` covers both (542 = 517 + the lib rows) and is the command to quote.
Second correction: `clippy --all-targets --target wasm32-unknown-unknown`
does not compile at all (`ThreadEvaluator` is
`cfg(not(target_family = "wasm"))`, `lib.rs:139-140`); CI's row is
`cargo check -p viewer --features app --target wasm32-unknown-unknown`
under the `getrandom_backend` flag, and that is what this lane ran.

**Out of fence, reported**: `crates/test-utils/tests/reader_census.rs:82`
says the ledger is "Sorted by path" and nothing enforces it — `found.sort()`
at `:592` sorts the tree walk, not the ledger.

## 2026-09-11 — #2320 merged; the proxy class is written down

**`view/cancel-doors` is on main** (#2320, merge `4f621cf31`), green on
the full code tier: 39 jobs, twelve `test (…)`, five
`k-lint (gate, …)`, `gate ok` success, one `neutral` on
`render drift (gui)` and five skips that belong to closures this diff
does not open. Read from the job list rather than a summary. Its two
lane worktrees and their private target dirs are reclaimed.

**The eighth instance of the proxy class was mine, and the class is now
a rule in `plan.md` rather than eight scattered post-mortems.** A sweep
rule fails when its classifier is a PROXY for the property the claim is
about — and a proxy agrees with itself over the population it can see,
so it reads complete from the inside every time. The eight are
tabulated at the sweep-rule paragraph: a constant standing in for a
fact (#2278), a boolean for a three-way question (#2282), `^`-anchors
for markdown's 1–3 spaces of indent (#2172, re-minted #2287), an
attribute's presence for existence at the target (#2288), full paths
for the bare filenames the tracker writes and the name `store` for the
field (#2293 twice), "the host pass" for a gate that runs two (#2304),
and `add_enabled` + `on_disabled_hover_text` for a DISPOSITION (#2320 —
the dispatch was mine, the correction the lane's). The check that
catches all eight is the same one: name the property first and the
pattern second, then ask what a member could look like that the pattern
cannot match.

**Both of #2320's command corrections are in `plan.md`** — the viewer
suite has two targets and the WGPU adapter red lives in `--lib`, and at
wasm32 the CI row is `cargo check` because `clippy --all-targets` does
not compile there at all.

**Still running**: `view/link-thirteen`, building Ev's rustdoc ruling
(link the thirteen bare spans with the house `[`crate::X`]` spelling,
make the lint inert on the skip-mode viewer pass, retire the three
module notes, amend the README's Rustdoc posture clause). It is the only
open lane.

## 2026-09-11 — `view/link-thirteen`: the thirteen are links, and the pass that forbade them stopped judging links

**Ev ruled, in chat on 2026-09-11: link them, and make the lint inert
on that pass.** `rustdoc-posture-test-names-one-axis-of-three` had put
both options up with their costs; the ruling took the tool change and
the links together. Both halves are here, plus the residue each one
left.

**The thirteen, re-derived at `6891829ee` rather than trusted.** The
list had not moved: `frame.rs:6`, `:85`, `:216`, `:385`, `:554`,
`:1766`, `:1802`; `pickindex.rs:12`, `:13`; `props.rs:40`;
`tree.rs:278`; `vocab.rs:50` ×2. All thirteen are `` [`crate::X`] ``
now. The three inside `#[cfg(test)]` were left, `frame.rs:2001`
included — it sits *above* the `#[cfg(test)]` at `:2003` because it is
the doc comment ON the test module, so it reads as production by line
and is rendered by nothing by item.

**Red before, green after, and the commands, because this PR's own CI
cannot run the pass it changes.** `scripts/ci-filter.py --files` over a
diff touching `crates/viewer` sets `RUN_VIEWER_TOOLKIT=true`, so
`ci.yml:1834` takes the non-skip path and the skip-mode viewer pass
never executes on this branch — which is exactly why these thirteen
reds never appeared on #2304. So the evidence is local and named:
`scripts/doc-gate.sh --pr --scope '-p viewer' --skip-viewer-toolkit`
exits **1** with the links in place and the gate unchanged, naming
**15** distinct sites (the thirteen, plus `theme.rs:7` and `:8`, linked
when that file's note retired), and exits **0** after. The non-skip path
CI does take, `--pr --scope '-p viewer'`, exits 0 both ways: all
thirteen resolve at `--all-features`.

**The fence: `scripts/doc-gate.sh` is CIW's** (`work/ciw/program.md:11`).
Ev ruled the change directly, so it is authorised rather than a lane's
decision, and the obligation that came with it was to announce it —
cited at the site, written in the PR, and filed on CIW's slate as
`view-made-the-skip-mode-viewer-doc-pass-lint-inert`, a notice row that
asks CIW for nothing but a read and a close.

**The pass is NOT dominated, and the item's own plan said it was.**
*"Delete that pass as dominated"* rested on the default-features pass
rendering a strict subset of `--all-features`. The two viewer passes are
the `if` and the `else` of one branch (`doc-gate.sh:890-907` at the
base) and **never run on the same invocation**: under
`--skip-viewer-toolkit` the all-features invocation does not name
`viewer` at all, so on a skip-mode run the default-features pass is the
only rustdoc that reads this crate, carrying every lint that is not
about a link target. Deleting it would have removed the crate's doc gate
from precisely the runs it was built for. This is the same shape the
plan warns about under *a universal without its sweep rule* — the
universal here was *"the pass is redundant"*, and the thing it ranged
over was lint coverage rather than pass scheduling.

**The spelling stayed the house one.** The item's table recommended
``[`X`](crate::X)`` so default-features prose would read as it does
today. Ev on the residue: *"totally fine for `cfg(not(feature))` stuff
to work badly — we already assume that several places."* So the leaked
`[crate::app::…]` brackets are accepted, the 1022-site house spelling
holds, and the crate does not grow a second link form.

**Two module notes retired, not three, and the item was wrong about the
third.** `theme.rs:9-12` and `vocab.rs:51-52` each said an intra-doc
link into the gated half breaks the headless pass; both are gone, and
`theme.rs`'s two bare module spans (`app`, `gpu`) became links in the
same edit rather than being left silent behind a deleted explanation.
**`forms.rs:18-20` stays**: its reason is `pub(crate)` items on a
public module page, not the headless pass — `named-not-linked-…` read
that correctly and `rustdoc-posture-…` did not, and the ruling does not
touch it. **Where #1330's reason now lives**: in
`crates/viewer/README.md`'s posture section, in the DEFAULT-features
bullet of the new *what is checked where* list, which says the same
thing in the present tense — the lint is off on that pass, so the link
no longer breaks it.

**The README ruling now names two host passes.** The heading was
*"Rustdoc posture: the host pass is the gate"*, and *"the host pass"*
was the exact word this row was filed about; it is *"the host
all-features pass is the link gate"* now, with a new paragraph saying
there are two, which one judges links, and an exhaustive three-bullet
*checked where* — everything at `--all-features`; everything except
`broken_intra_doc_links` at default features; and **nowhere** for a
broken renderer-free link written on a branch that never takes the
all-features pass, which is empty because writing one means diffing
`crates/viewer` and that diff seeds the toolkit. The closed row
`doc-comments-name-symbols-that-do-not-exist:216` still quotes the old
heading and is left as written: it records the heading it applied, on
the date it applied it.

**The selftest moved with the gate, and the arm that matters is the one
added.** Two arms inverted — a planted broken link in the fixture's
`viewer` member no longer fires under `--skip-viewer-toolkit`, by the
ruling. Left there, that pass would have had **no firing arm at all**
and could have been deleted with every case still green, which is
#2106's shape in the file whose subject is that shape. So
`plant_bare_url_in_viewer_member` was added as the positive control on
the same pass (a rustdoc lint that is not about a link target), and the
same planted link under `--pr --scope "-p clean -p viewer"` as the
control in the other direction. `--selftest` exits 0.

**No line shifts where they would have cost.** The thirteen edits are
in-place on their own lines, so `frame.rs`, `pickindex.rs`, `props.rs`
and `tree.rs` — **155** `file:line` citations between them — are
exactly the length they were, and lines up to 88 characters are within
this crate's practice: at the head tree **80** doc lines under
`crates/viewer/src` exceed 75 characters, to a maximum of **114** at
`drafts.rs:261`. (An earlier draft of this paragraph said *"116 at
`datums.rs:273`"*, read off the first ten rows of an unsorted list —
the plan's *distrust the first number out* in miniature, and the real
maximum was a line this diff has since repaired.) `theme.rs` loses 3 lines at
`:10-12`, so its `:13` and below move by -3; `vocab.rs` loses 2 at
`:50-55`, so its `:56` and below move by -2;
`crates/viewer/README.md` grows 32 below `:1493` and
`scripts/doc-gate.sh` grows 54. **The census of those bands**, by
finding the subject rather than shifting a number: one OPEN row is
affected and it is not ours —
`work/chrome/chrome-weight-is-outside-the-palette.md:22` cites
`theme.rs:425-428`, whose subject now sits at `422-425`. Reported to
the orchestrator rather than edited, per the implementer discipline's
§6; CHROME's slate is CHROME's. Everything else in those bands is on a
closed row, which is a record of its own tree:
`marks-header-…:34,46,87`, `viewer-const-all-…:105`,
`doc-comments-…:387`, and two of CIW's, both named in the notice row.

**The wider sweep is what earned the residue, and it found a dead
symbol.** The ruling's population is path-shaped —
`` `<app-gated mod>::<path>` `` — and running only that rule would have
been a sweep over the pattern that surfaced the defect rather than over
the property. Re-derived from the property (*a doc comment in the
renderer-free half naming an item behind the `app` feature*), there are
**five more**, spelled as a possessive across two spans:
`blend.rs:169`, `pickcache.rs:256`, `prefs.rs:387-388`, `scene.rs:887`,
`pickindex.rs:1582`. No sweep this crate has run can see that spelling
— it is not among the six enumerated blind spots of
`comment-symbol-names-outside-rustdocs-reach-have-no-gate`, whose blind
spot 3 is a span split across two LINES and measured at zero. And one
of the five is live: **`blend.rs:169` names `app`'s `unit_picker`, and
there is no `unit_picker` anywhere in this workspace** — the class
`doc-comments-name-symbols-that-do-not-exist` closed over 64 spans on
2026-09-10, alive in the same crate because that sweep's rule could not
match the spelling. Filed as
`possessive-code-spans-are-invisible-to-the-path-shaped-sweep-rule`.

**A second class fell out of checking that number, and it is three
lines long.** Re-deriving the longest doc line turned up
`sketch.rs:1015` at 125 characters, and the reason it was long is that
it says its own first sentence twice with a `///` wedged between the
copies: *"`/// **How big the tip marks in a profile preview are**/// **How
big the tip marks…**, in sketch-plane metres`"*. Rustdoc renders that
literally, on a page in the renderer-free half that BOTH host passes
build. Swept with *a doc line carrying a second `///` or `//!` after
column zero, outside backticks and outside an indented doc code block*
— over every tracked `.rs` in the repository, not just this crate —
and it is exactly **three**, all here: `datums.rs:201`, `:273` and
`sketch.rs:1015`. All three repaired in place, no line shift. **No gate
holds this**: it is not a broken link, so `doc-gate.sh` is green over
it at every pass, and the sweep's blind spot is the honest one — a
duplicated sentence that did NOT keep its `///` is invisible to this
rule and to every other.

**Two rows closed and two opened.**
`rustdoc-posture-test-names-one-axis-of-three` is closed with the
ruling and both corrections to its own plan recorded, `needs_ev`
cleared. `named-not-linked-is-a-silent-disposition-at-eleven-of-
thirteen-sites` is closed as **dissolved** — its Category A is links
now, so there is no silent disposition left to annotate and the
per-file note it costed is not owed. Its Category B did not dissolve
and does not die in a Closed section: seven `#[cfg(test)]` spans, which
the ruling cannot reach because nothing renders them, sitting beside ten
bracketed links in the same two files with no rule saying which is
right. Re-filed as `cfg-test-bare-spans-have-no-stated-disposition`.

### The review pass on #2332, and both fixes were the same mistake

**A dangling row id inside a CLOSED row.** The dissolved row's prose
said Category B was *"re-filed as
`bare-spans-outside-the-path-rule-have-no-stated-disposition`, together
with a second population"*. Neither half was true of the tree: the row
is `cfg-test-bare-spans-have-no-stated-disposition`, no row of that
first name exists, and the possessive population is a **separate** row,
`possessive-code-spans-are-invisible-to-the-path-shaped-sweep-rule`.
The dangling name was this lane's own working title, kept in the prose
after the row was split in two. `work.py lint` passes over it because
it resolves `refs:` frontmatter and not prose — so *a receipt is a
citation and gets no exemption* applies to a row id in a sentence, in a
file that outlives this program's directory. Both rows are named now,
and the row says they are two and why the rules differ.

**A universal whose reason did not produce its population, in the
section that ratifies the rule against exactly that.** The README's new
*Nowhere* bullet argued no branch can break a renderer-free link
without taking the all-features pass, because *writing* a link means
diffing `crates/viewer`. The property is a link being BROKEN, and a
link breaks when its TARGET moves — on a branch that never touches this
crate. `cargo_scope` is the dependent closure while
`run_viewer_toolkit` is seed-keyed (`ci.yml:1833-1836`), so such a
branch takes skip mode with `viewer` in scope and the now-inert pass is
the only rustdoc reading the crate. The position the bullet called
impossible is reachable.

**It is still true, and the reason is a contingency worth writing
down.** Swept by *every intra-doc link in the renderer-free half whose
first path segment is an external crate, against
`VIEWER_TOOLKIT_SEEDS`*: **twelve sites, all into `pncad`**, and
`pncad` is itself a seed (`ci-filter.py:1428`). So every branch that
can move one of these targets buys the all-features pass. A first link
into any crate outside that set opens the hole.
`renderer-free-cross-crate-links-are-ungated-off-the-seed-set` owns it.

**The count came in at twice the estimate, and the twelfth is the one
that matters.** The review offered six sites; the derivation gives
twelve — `blend.rs:425`, `display.rs:262`, `docio.rs:85`,
`marks.rs:297`, `matetool.rs:33`, `:54`, `:153`, `:220`, `parts.rs:11`,
`props.rs:652`, `sketch.rs:939`, `tree.rs:143`. The conclusion is
unchanged and the sweep rule is what moved the number. **`sketch.rs:939`
is the reason the rule says "intra-doc link" rather than "bracket"**: it
is the reference form, `` [`ProfileVertex`](pncad::profile::ProfileVertex) ``,
the only one in the crate and invisible to a bracket-shaped grep — the
same spelling this unit declined to adopt, reaching up to hide from the
sweep that would have policed it.

**The backstop the ruling names cannot red, and that is measured.**
`ci.yml:1821-1825` says the skip's lost coverage is re-taken by
`nightly.yml`'s `rustdoc (viewer, all features)`. That row is
`cargo doc -p viewer --all-features --no-deps` (`nightly.yml:291-293`)
and **`nightly.yml` sets no `RUSTDOCFLAGS` at all**. Planting
`` [`pncad::document::NoSuchItemAnywhere`] `` in `tree.rs` and running
that exact command gives **one `warning: unresolved link` and exit 0**.
It re-takes the RENDER and cannot re-take the LINT — enough for the
feature-axis coverage the skip gives up, since a page that fails to
build fails the command, and not enough for this class. So the bullet
names it for what it is instead of citing it as cover, which is what
the review asked for and the opposite of what the citation would have
said. The repairs that would make it a real backstop live in
`nightly.yml` and `scripts/ci-filter.py` — CIW's and S-TCOST's — and
are named in the row rather than taken here.

**The re-sweep before landing found a fourteenth site, and `main` red.**
*A sweep is accurate as of your merge base, not your merge.* Re-run
after merging `origin/main` (#2320, `view/cancel-doors`), the
path-shaped population is **fourteen**: `session/op.rs:797` names
`` `pane::create` `` in a production `///` comment, in a file that
arrived with the merge. Linked with the other thirteen; the red-before
on the merged tree is **17** distinct sites, not 15.

**Sixteen of those seventeen are this branch's. The seventeenth is
`main`'s, and it is this row's thesis firing in the wild.**
`session/op.rs:773` carries `` [`crate::widgets::drag_gesture_ops`] ``
— renderer-free module, `app`-gated target, spelled as a LINK — so
`origin/main` is **red on the skip-mode viewer doc pass right now**.
Measured at `origin/main` in a throwaway worktree with its own target
dir: `error: unresolved link to `crate::widgets::drag_gesture_ops``,
exit 1. It never showed on #2320's CI because that diff touched
`crates/viewer`, so `RUN_VIEWER_TOOLKIT=true` and `ci.yml:1834` took the
non-skip path — *"the defect fires on someone else's branch, not on the
branch that writes it"*, which was an argument when the row was filed
and is now a property of `main`. The next branch to reach `viewer`
through the closure without seeding the toolkit would have worn it.
**This PR clears it as a side effect of the ruling**, which is worth
saying plainly: the merge is not only a docs improvement, it takes a
standing red off `main`.

## 2026-09-11 — #2332 merged; it cleared a red #2320 put on main

**Ev's rustdoc ruling is landed** (#2332, merge `9664acdfa`): the
fourteen `app`-gated spans in the renderer-free half are links in the
house ``[`crate::X`]`` spelling, and `scripts/doc-gate.sh`'s skip-mode
viewer pass runs `RUSTDOC_LINTS_INERT` — CIW's file, on Ev's direct
authorisation, announced at the site, in the PR body and as
`work/ciw/view-made-the-skip-mode-viewer-doc-pass-lint-inert`. Two
module notes retired, not three. Full code tier, 38 jobs, twelve
`test (…)`, five `k-lint (gate, …)`, no unsubstituted placeholders,
`gate ok` success.

**The lane corrected the dispatch six times and every correction
stood.** `forms.rs:18-20`'s note is about `pub(crate)` items on a
public module page, not the headless pass, so it stays. There is no
wasm32 *clippy* row at `-D warnings` — CI's row is `cargo check` and
`ci.yml:2241-2250` says in terms that it is the one viewer row that
cannot fail on a warning. Retiring `theme.rs`'s note required linking
its two spans or the silent disposition returns. The selftest could not
simply lose two arms without leaving that pass unfirable — the #2106
shape, in the file whose subject is that shape. **The nightly re-take I
told it to cite as the backstop cannot red**, measured. And the six
cross-crate sites I handed it are twelve.

**#2320 landed a red on `main` and its own CI could not show it.**
`session/op.rs:773` links ``[`crate::widgets::drag_gesture_ops`]`` from
an ungated module into an `app`-gated one. A `crates/viewer` diff seeds
the toolkit, so that branch takes the non-skip path and documents the
crate at `--all-features`, where the link resolves. The skip-mode pass
— the only one that renders the renderer-free half alone — runs only on
branches that reach `viewer` through the closure without seeding it.
FIX's orchestrator hit it from `crates/quantity` (#2335) and filed it
on our slate (#2340) rather than absorbing it. I verified the clear on
merged `main` rather than inferring it from the merge:
`scripts/doc-gate.sh --pr --scope '-p viewer' --skip-viewer-toolkit`
exits 0 at `9664acdfa`.

**Answered on #2340** with three corrections: the repair the item
proposes is now contradicted by Ev's ruling (the link is correct as
written and the gate is where the fix belongs); "every code-tier PR"
generalises one step past the wiring, since a PR seeding `viewer`,
`pncad` or `bvh` takes the non-skip path and is green; and the window
was hours, not a week. Its silent-coverage paragraph is a real addition
and is taken. The sweep it asks for — an ungated module's doc linking a
gated one, which nothing mechanical reads — is genuinely open and VIEW
will home it.

**The proxy class is at nine and the ninth is mine, from inside the
review of the eighth.** I swept for cross-crate links with a
bracket-backtick pattern and got six; there are twelve, because
`sketch.rs:939` is the reference form ``[`X`](path)``. The conclusion
held — all twelve target `pncad`, a toolkit seed — but the population
was the argument. Writing the class down does not exempt the next
sweep from it.

**Three rows opened**: `cfg-test-bare-spans-have-no-stated-disposition`,
`possessive-code-spans-are-invisible-to-the-path-shaped-sweep-rule`
(whose `blend.rs:169` names `unit_picker`, a symbol that exists nowhere
in the workspace — alive because the 64-span sweep's rule was
path-shaped), and
`renderer-free-cross-crate-links-are-ungated-off-the-seed-set`. Three
doc comments that said their first sentence twice were repaired in
place (`datums.rs:201`, `:273`, `sketch.rs:1015`).

**VIEW stands at 73 open / 73 closed, with nothing waiting on Ev.**

## 2026-09-11 — #2343 merged; the out-of-fence table is not a repoint

**#2343 on main** (merge `a2447044`): the skip-mode CI hole, FIX's third
face of the silent-coverage class, and the ninth proxy instance. Docs
tier, 21 jobs, `docs-only ok` and `gate ok` both success.

**A convention defect found while reviewing #2348, and it is the
orchestrator's rather than a lane's.** Implementer-discipline §6 has
lanes REPORT another program's shifted citations and the orchestrator
place them. Two lanes have now reported the same CHROME row with
different answers — #2320 said `app.rs:1722-1723` → `1750-1751`, #2348
said `1722` → `1732` — each correct about its own diff and neither
correct once both land. An out-of-fence table is a statement about one
diff against one base and expires as soon as another diff touches the
file. **The table's value is the population it identifies, not the
numbers beside it**; placing it means re-deriving by subject at placing
time. Written into `plan.md`.

**With a second half that is sharper**: the table asserts *should read
X* for citations it never checked against their subjects. CHROME's
`app-rs-doc-comment-merge-scars.md:24` cites `app.rs:1722-1723` for a
sentence about `perform_batch`, which is at `app.rs:918` at that
branch's base and at its head. Shifting that number makes the row worse
while looking like a correction — the exact thing
`citation-repoint-shifted-a-number-the-lane-knew-was-wrong` names, which
this program already refuses in fence and had no rule for out of it.

**The proxy class reached ten and eleven in one census, both in #2348's
lane, and it self-reported all of them.** Its citation pattern required
a repo-rooted path where the tracker also writes bare filenames and
line-only continuations; it then applied the correction as a SECOND
pass over rows the first had already moved, double-shifting fifteen —
and the content check meant to catch that compares the old file's line
against the new file's line, so it verifies the shift MAP rather than
the starting point and returned true on every wrong answer. **That is
the best find of the session**: a check that cannot fail for the reason
it was written.

An eleventh, which reached a published PR comment: the resolver paired
a line number from one citation with a filename token from elsewhere in
the same row, reporting `.github/workflows/ci.yml:4030` as
`README.md:4030` — and the correcting comment then named a third wrong
file. Sent back. The verdict (strike those rows) was right both times;
the stated reason was false both times, and it was published as a
verification claim.

## 2026-09-11 — #2340 resolved without a row landing here

FIX reworked #2340 after VIEW's answer and **dropped the row it had
filed on our slate** — `git diff --name-only 8522bce76^1 8522bce76`
returns twenty-two files and not one under `work/view/`. That is the
right outcome: the red was cleared by #2332 before their PR merged, and
the repair their row proposed (respell the link) is contradicted by
Ev's ruling that the renderer-free half MAY link into the `app`-gated
half. **Nothing is owed from VIEW on it.**

The one part of their filing that was not answered by the ruling — the
sweep, *an ungated module's doc linking a gated one is a class and
nothing mechanical reads feature gates against intra-doc links* — is
already homed, and was before they asked: within the crate the ruling
makes it permitted rather than a defect, and the case that remains
ungated is the cross-crate one, which is
`renderer-free-cross-crate-links-are-ungated-off-the-seed-set` (filed
by #2332's lane, with the measured fact that the nightly re-take cannot
red). No new row; I said VIEW would home it and VIEW already had.

**Fence record repaired on our own side.** S-TCOST split on 2026-09-11
(`d6a9b948a2`, *"open S-TINT and move the non-cost half of S-TCOST's
board to it"*) and both programs now declare `crates/*/tests/*` and
`crates/test-utils/*`. VIEW's `keep_out` named only S-TCOST, so
`work.py lint` reported the S-TINT overlap as *"neither `keep_out`
names the other"* — invisible from both sides, on 54 paths this program
writes to constantly. It now names S-TINT as well, and says which half
a VIEW lane actually touches: the integrity half, not the cost half, is
what a lane adding assertions to `crates/viewer/tests/*` is in. That
moves the warning to one-sided, which is as far as VIEW can take it
alone; S-TINT's own `keep_out` is theirs.

## 2026-09-11 — `view/silent-withdrawals`: the two halves of one asymmetry

Two rows, both in `crates/viewer/src/display.rs`, dispatched together
because they are the two doors that withdraw display state and report
differently.

**`prune-kills-a-gesture-and-reports-nothing` — CLOSED by fixing.**
`PruneReport` grew its third field and `prune` stopped throwing the
fault away at the instant it had it. The field is
`killed_gesture: Option<Withdrawn>`, an `Option` and not a `Vec`
because a `DisplayState` holds ONE free-move gesture — which is also
why the new sentence is the only kind with no plural, and why
`Display for Withdrawal` now matches an `Option` for its count rather
than carrying a `many` string its own constructor cannot reach. The
wording followed #1886 twice over: a killed gesture is not a
supersession (nothing substituted for it), so it is a third sentence —
*"free move: the drag in flight was ended — <fault>"*.

**`display-clear-drops-free-move-placements-silently-while-prune-reports-them`
— CLOSED by ANSWERING.** The item offered a fork and its own
counter-argument won, but not for the reason the item gave. Costing it
against the tree rather than against the file moved it: implementing
the proposal (`clear` reporting through `prune`'s channel) and MEASURING
it, a reopen of the same file produces an EMPTY report while all three
kinds of state are taken — ids are minted per `Doc`, so
`free_move_check` answers `Ok(())` about the incoming document's nodes
— and `NewDocument` produces `NoSuchNode { node: 0 }`, a true sentence
about a document the user has never held state on. So the report cannot
distinguish "nothing went" from "everything went". The silence is kept
and is now written at `clear`, with a row that reds if it is widened
back into an oversight. The IN-FLIGHT drag at that same door is left to
`free-move-drag-dissolved-by-open`, whose fork is a refusal and not a
report.

**The census bit that cost the most.** The citation sweep's first
pattern required a repo-rooted path and missed two other spellings the
tracker actually uses — a BARE filename (`display.rs:861`) and a
line-only continuation (`` `:1632` ``) — which is this program's proxy
class again, the tenth instance. Worse, the correction was applied as a
SECOND pass over rows the first had already moved, double-shifting
fifteen of them; the content check that was supposed to catch it
verifies the shift MAP rather than the starting point, so it passed on
every wrong answer. Redone as one pass from the pre-repoint state. And
`plan.md`'s two hits are QUOTATIONS of past citations, not pointers at
the tree: repointing them corrupts the record, so `plan.md` is left
alone. A shift-repoint cannot tell a pointer from a quotation, which is
the same class one level up.

Residue filed rather than left in prose:
`a-fourth-withdrawal-kind-is-forced-at-one-of-its-three-sites` — the
three kinds are declared on `PruneReport`, re-declared on `OpOutcome`
and hand-fanned into `app.rs`'s notices, and only the copy between the
first two is exhaustive.

### The out-of-fence table was wrong, and the corrections were wrong too

Adjudication caught two defects in the table this unit published for
other programs' rows, and running them down found three more proxy
failures in the same resolver. Recording all five, because the census
rule this program keeps re-learning is that **a shift map is not a
citation check**.

1. **The file token and the line number came from different
   citations.** The resolver bound a bare `:NNNN` to the last filename
   it had seen, and its extension list omitted `.yml` — so
   `.github/workflows/ci.yml:4030` was read as a line number belonging
   to a `README.md` mentioned elsewhere in the row. Two rows entered
   the table that hold no `crates/viewer` citation at all.
2. **The correction to that was wrong in the same way.** Told the rows
   were misattributed, the lane re-resolved them by the nearest ROOTED
   path in the row — `tools/README.md`, `demos/README.md` — and
   published that as *"confirmed by the rooted paths in those rows' own
   text"*. Same defect, one file further along, and this time inside a
   verification claim on a PR. The strike verdict was right and the
   stated reason was false twice over.
3. **The shift map never asked whether the cited line exists.**
   `crates/viewer/src/app.rs` is **2019** lines at the merge base and
   `session.rs` is **2039**; fifteen of the table's entries cite lines
   above those — `app.rs:5014`, `:4626`, `:3075` — relics of the
   pre-split file (`app.rs` was 5,696 lines before #1830). Pure
   arithmetic on a line number produced a confident "should read 5024"
   for a line that does not exist.
4. **No entry was checked against its SUBJECT.** Seven more are in
   range and name something that is somewhere else:
   `app-rs-doc-comment-merge-scars` cites `perform_batch` at
   `app.rs:1722-1723` where it sits at `:918` — and that row states
   outright *"Line numbers are as of PR 1776's head; the function names
   are the durable anchors"*, so its numbers were never tracking the
   tree.
5. **Eight entries are quotations, not pointers.**
   `drag-tick-row-cites-app-rs-for-a-finding-that-lives-in-forms-rs` is
   a table OF the wrong `app.rs` numbers beside the right `forms.rs`
   ones — repointing it edits the evidence — and
   `wasm-row-warning-debt-…` quotes compiler diagnostics captured at a
   named SHA.

**Of 31 out-of-fence citations, exactly ONE is a true shift**, and it
is the row someone had already re-derived by hand.

The standing rule this yields, and it is the orchestrator's to put in
`plan.md`: an out-of-fence table is a statement about ONE diff against
ONE base and expires the moment another diff touches the same file —
#2320 reported the same CHROME row as `1722-1723` → `1750-1751` where
this unit reported `1722` → `1732`, both right about their own diff and
neither right once both land. So a repoint is re-derived at PLACING
time, by subject, and a citation already wrong at the base is disclosed
rather than moved to a new wrong number.

## 2026-09-11 — #2348 merged; one of thirty-one

**`view/silent-withdrawals` is on main** (#2348, merge `88bacdd2fe`),
green on the full code tier at `606555b9c8`: 38 check runs, twelve
`test (…)`, five `k-lint (gate, …)`, `rustfmt + rustdoc (gate) +
wasm32` and `gate ok` all success, six change-filter skips, no
placeholders — read from the job list. `prune` reports the killed
free-move gesture with its cause instead of throwing it away at the
instant it has it, and `clear`'s silence is now a decision written at
`clear` rather than an oversight.

**The `clear` fork was settled on a typing fact, not the wording
argument the item offered, and the lane was right to move it.** A
`Withdrawn` carries a `DisplayFault` about a document, and the only
document left to ask at `clear` is the replacement — where the ids mean
other nodes, because `next_id` is a counter the `Doc` owns
(`crates/editor-core/src/doc.rs:315`, `:380`). The lane implemented the
item's own proposal and measured it: reopening the same file reports
EMPTY while a hide, a placement and a drag in flight are all taken, and
`NewDocument` reports `NoSuchNode { node: 0 }`. A report that cannot
tell *nothing went* from *everything went* is worse than the silence.

**The out-of-fence census came back one placeable of thirty-one**, and
that is the result rather than an embarrassment: fifteen past EOF,
seven subject-elsewhere, eight quotations. Written into `plan.md` with
the reason the instrument could not see it — a shift map is arithmetic
on an integer and cannot be wrong in its own terms.

**Three corrections to my own messages, all the lane's and all
standing.** `check-ci-mirror-parity.py` was never in its validation
table, so there was no old result to carry and I said there was. The
workspace count did NOT move as I predicted — `crates/mesh/src/
nurbs_cert_fuzz.rs` changed by 71/13 lines and still declares three
`#[test]`s, so 6848 held, and the lane checked rather than reporting
the same number twice and hoping. And earlier: its "CI is blocked on
your credential" was wrong, it was blocked on having nothing real to
push — which it named as its own worst error of the lane, because it
had offered a force-push to route around a diagnosis it had not
checked. It was offered and refused; the merge that was owed on the
merits was the answer, and it is what fired Actions.

## 2026-09-11 — `view/gesture-doors`: the two gesture doors, and one fan-out

Two items, both adjacent to what #2348 landed.

**`free-move-drag-dissolved-by-open` — the FREE-MOVE side was wrong.**
The item asked which of the two drags gets the wrong treatment and
#2348's `DisplayState::clear` clause had already removed one of the two
possible answers: a per-instance report cannot be built truthfully at a
replacement, because a `Withdrawn` carries a `DisplayFault` about a
document and the only document left to ask is the incoming one. So the
report half of the item's menu was never available, and the refusal is
the answer. `SessionOp::permitted_during_free_move` is a SECOND
exhaustive table rather than a widened first one: the two drags refuse
different sets, and one predicate could serve both only by refusing the
union — a commit landing under a probe is pruned and reported, which is
a better answer than a refusal. The two tables agree on exactly the two
doors that REPLACE the document.

**`a-fourth-withdrawal-kind-is-forced-…` — both candidates, because
neither alone is the fix.** `OpOutcome` holds the `PruneReport` (the
re-declaration and the copy are gone, and with them `from_prune`'s
destructure — the one site that held and the wrong one), and
`frame::Withdrawal::all` destructures the report at the RENDERING call,
with the three constructors made private so nothing outside `frame` can
fan out by hand. `app.rs`'s three `extend`s are one.

**The defect had a FOURTH instance and it was in a test.**
`frame_policy.rs`'s hand-written mirror of the `app`-gated loop — the
one whose own comment warns that a half-mirror passes while the real
loop drops a kind — listed two producers after #2348 added the third
beside it. Nobody found it by reading; it fell out of collapsing the
fan-out and finding a caller that could not be collapsed. A comment
saying *"this must model every producer"* is not a hold.

**Two operational notes, both costs paid here.**

1. `git checkout -- <file>` on an UNCOMMITTED tree is `git checkout
   HEAD -- <file>`. A mutation harness that reverted three source files
   that way discarded every uncommitted edit in them — #2089's clobber
   in a new shape, and the fix is the same: commit first, or mutate in
   a throwaway worktree with its own target dir. Nothing was lost
   because the edits were scripted and re-runnable; that was luck about
   the method, not a property of the harness.
2. Adding a tenth `vocabulary!` moves SIX live counts in
   `crates/viewer/README.md` and three in `src/vocab.rs`, and the
   README states one HISTORICAL census beside a live one ("Ten `const
   ALL` tables existed…; nine were of this kind") which must not be
   bumped with the others. A count that is a claim about the tree at a
   named moment is not the same count as a claim about the tree.

Residue filed:
`census-table-in-the-viewer-readme-is-not-its-own-population` — the
README's non-dump census table says nine and `PruneReport::is_empty`
(`display.rs:592-599`) is a tenth it does not carry, added by #2348
after the table was built.

## 2026-09-11 — #2358 merged; a comment that warned about half-mirrors was one

**`view/gesture-doors` is on main** (#2358, merge `6aee1efe96`), green
on the full code tier at `2b1e69a0`: 38 check runs, twelve `test (…)`,
five `k-lint (gate, …)`, `rustfmt + rustdoc (gate) + wasm32` and
`gate ok` all success, six change-filter skips, no placeholders — read
from the job list. `Open` and `NewDocument` refuse under an in-flight
free move, and a fourth kind of withdrawal is now E0027 at the call
that words it.

**The find of this unit is a test that was the thing its own comment
warned against.** `crates/viewer/tests/frame_policy.rs` carried a
hand-written mirror of the `app`-gated notice loop whose comment read
*"it has to model every producer that feeds the notices there — both
withdrawal channels… A half-mirror would pass while the real loop
dropped the other."* After #2348 added the third producer beside it, the
mirror listed two. **The comment named the failure mode exactly and did
not prevent it**, because a comment is not a hold. Nobody found it by
reading; it fell out of collapsing the fan-out and meeting a caller that
could not be collapsed. And #2348 is a PR I reviewed and merged.

**The lane departed from its item on one point and was right.** The item
said the mid-gesture table's subject widens past the value gesture and
its name should widen with it. One predicate could serve both drags only
by refusing the UNION, and the union is wrong in both directions — a
commit landing under a probe is pruned and REPORTED (`killed_gesture`,
#2348), which is a better answer than a refusal. So
`permitted_during_free_move` is a SECOND exhaustive table refusing two
rows. The README section widened; the name did not.

**Three things the lane asked me to check, and my answers.** The
`vocabulary!` conversion of `WithdrawalKind` cost six live README counts
and three in `vocab.rs`: worth it, because it is the house pattern
rather than a special case — the crate's tenth — and without it the
reverse direction (a kind with no producer behind it) is held by
nothing. The re-baselined `a_document_replacement_takes_all_display_
state_and_reports_none_of_it` is exactly what `CLAUDE.md` asks for: a
stored bit that changes is not a cost to weigh against making the code
right; re-baseline and say what moved, which its doc now does. And the
`git checkout --` clobber is a rule rather than a blocker — see
`plan.md`, where it now sits, because this is the SECOND lane in a day
to do it.

**The out-of-fence census came back 0 placeable of 16**, on a
deliberately different diff from #2348's 1-of-31. Two independent
measurements, same verdict: those rows are damaged and were damaged
before either branch existed.

**And #2332's documented cost was paid immediately**, in the direction
nobody was watching: the skip-mode pass exited 0 over three intra-doc
links to items this diff had just deleted, while the full workspace pass
exited 1. Both passes are owed by a viewer lane. In `plan.md`.

## 2026-09-11 — `view/gesture-identity`: the driving operations name their gesture

`preview-and-commit-carry-no-gesture-identity` closed. Shape 1 of the
item's three, spelled as the tree spells the two begins rather than as
the item wrote it, and the free-move quartet taken with it.

**Costing the fork against the tree moved two of the three.** The
item's shape 1 — `PreviewGesture { node, slot, value }` — has nothing to
say for a document-parameter drag, so it is either a union payload (a
second spelling of a target vocabulary `BeginGesture` and
`BeginParamGesture` already have between them) or one preview and one
commit per door. The second is what `BeginParamGesture`'s own docs
argue for, costs no new public type and no translation at any call
site, and is what landed: 39 operations became 41.

Shape 3 cannot be built where the item puts it. The chrome queues ops
and `ViewerApp::perform_batch` performs them after the layout walk, and
a begin and its first preview reach the same batch — `Refusal::rank`'s
own worked example says so. Nothing in `widgets::drag_gesture_ops` can
know a refusal that has not happened yet, and the cheapest form that
works asks the session which gesture is open, which needs the identity
public anyway and then sites the decision where no other driver of
`SessionOp` can reach it.

Shape 2 refuses the one recovery the chrome has. A stranded drag's own
field, dragged again, names the same slot: its begin is refused and its
preview and release land the number the user dragged it to, against the
same base document because nothing that moves the document is permitted
mid-drag. A per-begin token would refuse that and strand the reader
twice. `the_open_drags_own_field_dragged_again_lands_its_number` holds
it, and mutation 7 below is what reds it.

**The refusals are spelled apart from the in-flight ones**, and that is
the tree's answer rather than taste: `permitted_during_value_gesture`
is a function of the operation alone and cannot answer a question about
a payload, so folding the mismatch into `GestureInFlight` would make
`every_op_behaves_as_the_table_says` unable to tell a table answer from
a payload answer. `Refusal::WrongGesture` and
`DisplayFault::WrongFreeMove` rank with the bookkeeping refusals: they
arrive in a batch behind the `GestureInFlight` that refused the drag's
begin, and that is the sentence with the remedy in it.

**#2358's precedent followed rather than departed from**: no new
predicate and no third table. Each check sits in the door of the state
it is about — `DocSession::preview_gesture`/`commit_gesture` and
`DisplayState::preview_free_move`/`commit_free_move` — which is the
same argument that gave the two drags two tables.

**The reachability route holds on today's tree**, re-checked rather
than assumed: `a_drags_own_preview_can_strand_it_and_the_door_closes_it`
passes at `dba1afd053`, and its strand half is now a helper two rows
share — the second continues into the drag a reader makes when the
panel comes back.

**The free-move half is API-reachable only.** No route to a second
probe under an open one has been traced; that is
`free-move-in-flight-refusal-has-no-reachable-producer`'s question and
is not answered here. What changed for that row is its population:
`DisplayFault` now has two arms with the same standing, not one.

Residue: `the-two-drags-name-their-gestures-in-two-shapes` — the
identity is a `(node, slot)`, a name, or an instance, and `drag_ops` is
generic over the difference with nothing holding the three to each
other.

## 2026-09-11 — #2361 merged; the sharpest defect on the slate is closed

**`view/gesture-identity` is on main** (#2361, merge `a88eedd036`),
green at `a578aecb`: 38 jobs, twelve `test (…)`, five
`k-lint (gate, …)`, `gate ok` success, six change-filter skips, no
placeholders — read from the job list after waiting for `gate ok` to
post, which it had not when the lane reported. Six gesture-driving
operations now name the gesture they drive and are refused on a
mismatch, so a drag on one field can no longer steer — or commit into —
another.

**Two of the item's three candidate shapes moved once re-derived
against the tree, and that is the result.** Shape 1 as written cannot
be spelled: `PreviewGesture { node, slot, value }` has nothing to say
for a document-parameter drag, so it is either a union payload or one
preview and one commit per door — the lane took the second, which is
`BeginParamGesture`'s own stated argument applied to the two operations
that lacked it. Shape 3 cannot be built where the item puts it: a begin
and its first preview reach the SAME batch, so at push time there is no
refusal to react to. Shape 2 refuses the one recovery the chrome has —
the stranded drag's own field, dragged again, names the same slot and
lands the number the user dragged it to; a per-begin token strands the
reader twice.

**A correction to that reasoning's receipt, which the PR body got
wrong.** The worked example it leans on for rejecting shape 3 —
*"`BeginGesture` refuses with the affordance and the same frame's
`PreviewGesture` refuses `NoGesture` on top of it"* — is real and says
exactly what the argument needs, but it lives on
`ViewerApp::perform_batch` (`crates/viewer/src/app.rs:913-914`), not on
`Refusal::rank` as the body claims. A receipt is a citation and gets no
exemption. The ratified text is clean: the misattribution never left
the PR body, so this note is the correction rather than a diff.

**A loop closed on the orchestrator's own earlier finding.** I told
#2348's lane that `work/chrome/app-rs-doc-comment-merge-scars.md:24`'s
`app.rs:1722-1723` was *"never about its subject"* because
`perform_batch` is at `app.rs:918`. The staleness verdict was right and
the located subject was not: that row's subject is the DOC COMMENT on
`perform_batch`, the two stacked summaries — *"Perform one operation and
record what it refused."* immediately above *"Perform one frame's whole
batch of operations"* — and it sits at **`app.rs:906-907`**, pre-existing
and untouched by any VIEW diff. So the row is live, its citation is
stale, and the re-derived location is recorded here for whoever places
it. That is the `plan.md` rule — *an out-of-fence table is a population,
and placing it means re-deriving by subject at placing time* — executed
rather than restated.

**The lane reported before CI finished**, offering an all-local
evidence table with the PR marked open. The evidence was sound and the
run went green, but a report that says *here is the PR* without a
`gate ok` is a report about a tree and not about a merge. Dispatches
say to wait for it.

**VIEW stands at 71 open / 78 closed, nothing waiting on Ev.**

## 2026-09-11 — #2360 merged; the tracker was re-cut under us, and one lane dispatched

**#2360 merged** (`bf6bdca140`), docs-only tier verified from the job
list: **21 jobs, `docs-only ok` success, `gate ok` success**, the other
17 skipped by the change filter. Its body was widened to cover #2361
before the merge, so the three units it ratifies and the two
corrections it records are all in the description rather than in a
commit message.

**`main` was re-cut while VIEW was mid-wave.** #2370 and #2371 split
`work/issues/` and `work/code-quality/` into eleven programs; VIEW's own
directory came through unchanged, but the slate now has neighbours it
did not have this morning (`work/door/`, `work/wire/`, `work/suite/`
among them). The merge into the orchestrator branch was clean. Worth
recording because a dispatch written against the old layout would cite
paths that moved — the standing rule about re-deriving item files from
`main` before dispatching now also means re-deriving which *program*
owns them.

**A VIEW row arrived from outside.** The FIX orchestrator filed
`seeded-draft-is-the-commit-path-and-does-not-round-trip` onto this
slate (found by PR #2366's lane sweeping out of fence for
fixed-precision renderers, verified and re-framed by FIX before
filing). It is the sharpest row on the slate: the δ field's seeded
draft **is** its commit path, so focusing the field and clicking away
commits a value nobody typed — silently quantising δ to the nearest
micrometre, or, below 500 nm, producing a refusal about a number the UI
itself put in the box.

Dispatched as `view/delta-round-trip` with two findings the item does
not have, both recorded as rules in `plan.md`: that the item's
seed-an-exact-spelling shape is arithmetically unachievable because the
lossy step is the unit conversion (~14% of sampled δ fail to
round-trip even at the shortest round-trip spelling), and that
`drafts.rs`'s own doc already says `Some` means *as typed* while
`get_or_insert_with` makes it mean *has focus* — which turns the item's
stated *preference* into a written contract the code breaks.

**Ev is handing off two rows personally** —
`work/door/boolean-op-has-a-third-hand-written-complete-list` and
`work/door/viewer-pathverb-all-hand-written-seventeen`. Both sit in
DOOR rather than VIEW; neither is to be dispatched from here. The
dispatch says so explicitly.

**VIEW stands at 72 open / 78 closed, nothing waiting on Ev.**

## 2026-09-11 — `view/delta-round-trip`: a draft is text a user typed

`seeded-draft-is-the-commit-path-and-does-not-round-trip` is **closed**.
The View pane's δ field seeded `drafts.delta_mm` with a `{:.3}` render
of the δ in force, and that same field is what `lost_focus` parses and
commits — so focusing the field and leaving it committed a number
nobody typed: `0.0` below 500 nm (refused by
`DisplayTolerance::new`), and a quantisation to the nearest micrometre
everywhere else, silently.

**The shape taken was the item's third, not its first.** A keystroke is
now the only thing that makes the draft `Some`; an untouched field has
nothing to commit. That is not merely the preferred shape — it is the
one `crates/viewer/src/drafts.rs:33-39` **already documents**, *"the
View pane's δ field … in millimetres AS TYPED"*, and which the seeding
made false. The field's whole body moved out of `ViewerBehavior` into a
free `delta_field` taking the four things it actually reads, so its
focus lifecycle is testable without building a thirty-field behaviour;
`pane/viewport.rs`'s `land` is the precedent.

**The item's first shape — seed the exact spelling — was measured
before it was dropped, and the measurement corrects the dispatch.**
Seeding the shortest round-tripping spelling of `δ · 1e3` and parsing
it back through `· 1e-3` returns a different `f64` for 4,155 of 28,600
sampled δ in [1e-12, 1e-1] m — 14.5%, every one by exactly 1 ULP,
because the lossy step is the unit conversion and not the format. The
dispatch said no spelling can close it; that is true of 669 of those
28,600 (2.3%), which have no `f64` millimetre preimage at all, and for
the other 97.7% a preimage does exist within 1 ULP of the naive
product. It is unreachable in practice rather than in arithmetic: a
budget-chosen δ is `constant / TRIANGLE_BUDGET`
(`crates/viewer/src/scene.rs:972`), whose exact spelling is seventeen
significant figures in a 56-point field. The conclusion stands and the
reason for it is narrower than stated.

**The three siblings were checked, not taken.** `Bounds::wording`
reaches `pane/properties.rs:702` and `:756` through `ui.weak`,
`frame::delta_badge` builds a `Badge`, `FittedDelta::wording` is that
badge's detail; nothing parses any of them back. Render-only, as the
item says. What the fix leaves behind in the δ field itself — the
render still reads `0.000` for a sub-micrometre δ, and an
edit-then-undo commits that reading — is the fourth member of that
family and has its own file,
`delta-field-renders-a-sub-micrometre-delta-as-zero`, filed in the same
PR that discloses it.

**Six rows, driven through a headless `egui::Context`.** Nothing short
of egui's own focus lifecycle tells a typed field from a visited one,
so the rows run real `RawInput` against `Context::run_ui` rather than a
stub for `changed`/`lost_focus`. Re-seeding the draft unconditionally
makes exactly the two failure rows fail, with `Some(0.0)` and
`Some(2e-6)` — the item's own two numbers. `egui::Response::changed()`
was checked rather than assumed: `TextEdit` marks it from
`text_changed` alone, set only inside the `has_focus` event pass
(egui 0.36.1, `src/widgets/text_edit/builder.rs:551-589,810-812`), so
it cannot fire on focus gain or on hover.

## 2026-09-11 — #2382 merged, and the lane corrected a rule I had already written down

**#2382 merged** (`f66047dfbf`), full code tier verified from the job
list: **38 jobs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, no unsubstituted placeholders; the six skips are the two
cache primes, the two interval-backend rows, `step import (freecad)`
and `python suite` — the last correct by construction, since `viewer`
sits above the wheel.

The lane took the shape that makes an untouched field
unrepresentable, and led its argument with the written contract rather
than with taste: `drafts.rs` documents `delta_mm` as the field *"in
millimetres as typed"*, so `Some` was already documented to mean
*typed* while `get_or_insert_with` made it mean *has focus*. It also
sharpened that doc to say when `Some` **begins**, not only when it
ends — the half that was missing is the half the defect lived in.

**The correction, and it lands on me.** I dispatched with a finding
stating that seeding an exact spelling is *"arithmetically
unachievable by any spelling"*, and wrote that into `plan.md` before
the lane reported. It is too strong, and the lane said so: the seed
need not be a spelling of the **product** `δ·1e3` — it can be a
spelling of a **preimage**, some `m` with `m * 1.0e-3 == δ` exactly.
The lane measured a preimage existing for 97.66% of sampled δ; I
re-measured on my own grid and got 97.73%, with 2.27% having no `f64`
preimage at all over ±64 ULP. So the shape is dead outright only for
the ~2.3%, and merely expensive — a ULP-neighbourhood search per
render — for the rest.

My conclusion survived; my reason did not, and the difference matters
because a rule stated too strongly is a rule that will be believed
past the point where it is true. `plan.md` now carries the corrected
statement together with what actually kills the shape, which is the
lane's own find rather than mine: a budget δ is
`constant / TRIANGLE_BUDGET`, seventeen significant figures in a
56-point field. **This is the fourth lane correction this week and the
fourth that was right.** The general form is now written beside it:
*"no spelling of X works"* is not *"no seed works"*, and the gap
between them is where a correction lives.

**Three pieces of residue, all handled the way the rules ask.** The
`{:.3}` render survives as a render and still reads `0.000` for a
sub-micrometre δ, with one path that still commits that reading
(type a character, delete it, leave — the render has become the user's
own draft); filed as
`delta-field-renders-a-sub-micrometre-delta-as-zero` with the
arithmetic carried into the file so nobody re-derives it. The lane's
`drafts.rs` edit shifted a line
`four-debug-walks-are-spelled-and-placed-two-ways:26` cites, re-derived
by subject to `:393`. And while placing that shift the lane found the
row's framing census stale in ways its own diff did not cause — four
`Display` impls named where there are five, and *1,819 lines* now
1,896 — disclosed on the row rather than half-repaired, per this
program's own rule about a count fixed in one place.

The three render-only siblings were checked rather than taken on the
citation, and the item's separation held: nothing parses any of them
back.

**VIEW stands at 72 open / 79 closed, nothing waiting on Ev.**

## 2026-09-11 — `view/free-move-reachability`: the refusal is reachable, and the keyboard is the second hand

`free-move-in-flight-refusal-has-no-reachable-producer` asked whether
`DisplayFault::FreeMoveInFlight` can be shown to anybody. It can, and
the answer is a row rather than an argument:
`crates/viewer/src/widgets.rs`'s
`a_keyboard_bump_begins_a_second_probe_under_a_held_drag` drives the
probe field's three `DragValue`s through the real `drag_ops` against a
headless `egui::Context` and reads the ops back — a pointer press and
move give `["begin", "preview"]`, and a Tab/ArrowUp pair on a component
the pointer is not holding gives `["begin", "preview", "commit"]` with
no commit and no cancel between it and the first begin. The mutation
the row's own doc comment names as its repair — a typed arm guarded on
the drag state — turns it red.

**The item's two untraced candidates were the wrong two, and one of
them is dead structurally.** egui carries `dragged`, `drag_started` and
`drag_stopped` as a single `Option<Id>` each
(`egui-0.36.1/src/interaction.rs:24-40`), so no second pointer and no
touch opens a second drag; multi-touch feeds `MultiTouchInfo`, a
zoom/rotate aggregate. The hand the search missed is not a pointer at
all: a `DragValue` enters keyboard-edit mode the frame it takes focus,
deliberately, for screen readers (`drag_value.rs:462-466`), and egui's
focus and key handling never consult the pointer. The same blindness
covers buttons — `Response::clicked` is true from keyboard focus plus
Space/Enter, or from an AccessKit `Action::Click`, with no pointer
(`context.rs:1464-1478`, `response.rs:183-184`). **A reachability
question asked over pointer states is a proxy for one about input**, and
this program's table gains a twelfth row for it.

**#2358 had already moved the answer and the item predates it.**
`session.rs:1089-1090` raises the same `DisplayFault::FreeMoveInFlight`
for every operation `permitted_during_free_move` refuses — `Open` and
`NewDocument` — so a second `BeginFreeMove` was never the only route,
and the item's *"every route needs the free-move strand"* was false
when it was written. #2348's `killed_gesture` cuts the other way and
closes the strand the item was hunting: `prune` runs on every document
transition (`session.rs:1611`, `:1994`) with the same predicate that
takes the field away.

**The honesty inversion does not land on this arm.** Every route above
has the pointer still holding the drag, so *"finish the free-move
first"* is followable; and `cancel_doors` draws *"Cancel free-move"*
enabled exactly while `probing()` is `Some` (`session.rs:660`) anyway.
What the search did NOT rule out is the selection: `instance_ui` draws
only for `selection().node()` and no prune covers that, so a `Select`
under an open probe would strand it. Every `Select` producer in
`crates/viewer/src/` today is a pointer click and cannot land under the
same pointer's drag — but the keyboard reaches those controls too. The
row closed without it, and it is written down rather than left in a
head.

Two residues, each its own file in the same PR:
`escape-commits-a-free-move-instead-of-abandoning-it` (egui aborts a
drag on Escape by clearing `dragged`, so `drag_stopped` fires and the
chrome commits the probe the user asked to abandon — measured
`["commit"]`), and
`a-keyboard-bump-lands-and-closes-the-pointers-own-probe` (all three
components name one instance, so the typed arm's preview overwrites and
its commit lands and closes the pointer's own gesture — the user is
shown a refusal naming a state the same batch destroyed).

## 2026-09-11 — #2388 merged; the twelfth proxy, and the plan's own count was one of them

**#2388 merged** (`dcab0bf2ee`), full code tier verified from the job
list: **38 jobs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, six skips (two cache primes, the two interval-backend rows,
`step import (freecad)`, `python suite`), no unsubstituted
placeholders. The lane settled its `RUN_VIEWER_TOOLKIT` question by
**running** `scripts/ci-filter.py` rather than reading `ci.yml`, which
is the rule as written.

**`DisplayFault::FreeMoveInFlight` is reachable, and the answer is a
test rather than an argument.** `crates/viewer/src/widgets.rs` now
carries a row that reproduces the probe field's exact shape — three
`DragValue`s over one instance, each through the real `drag_ops` with
the free-move triple — and drives it against a headless
`egui::Context`: pointer press and move give `["begin", "preview"]`,
keyboard-only frames answer nothing, and the step the focus reaches
another component gives `["begin", "preview", "commit"]`. A second
`BeginFreeMove` under an open one, from the chrome.

**The twelfth proxy instance, and it is the sharpest.** The natural
sweep for a reachability question is over *pointer* states — can one
pointer hold two drags, can a click land under a held drag — and that
sweep is closed, self-consistent, and answers **no**. The property is
not pointer states; it is **input**. A `DragValue` enters keyboard-edit
mode the frame it takes focus, deliberately, for screen readers, so the
keyboard reaches a second component while the pointer still holds the
first; and `Response::clicked()` is true for a keyboard Space/Enter and
for an AccessKit `Action::Click` with no pointer anywhere. egui's API is
*built* to make the three indistinguishable at the widget, which is
exactly why the pointer-shaped sweep cannot see the other two. The
dispatch warned against concluding unreachable from a failed search and
named multi-touch and wasm relayout as the untraced candidates; **both
of those were wrong** — multi-touch is dead structurally (egui carries
one `Option<Id>` each for `dragged`/`drag_started`/`drag_stopped`) —
and the real hand was one neither the item nor I had thought of.

**The item's premise was already false when it was dispatched, and I
did not catch it.** It said *every* route to the fault needs the
free-move strand. The fault has a **second producer**: `session.rs:1090`
raises it for every op `permitted_during_free_move` refuses, which is
`Open` and `NewDocument` (`op.rs:854`) — added by #2358 four hours
earlier. I verified the item's three `file:line` citations against main
before dispatching and they all landed; a premise is not a citation and
my check did not cover it. `plan.md` now says the dispatch owes a
re-derivation of the **premise**, not only of the citations.

**And the count in the proxy section was itself a member of the class
it documents.** The lead-in read *"Eight instances"* over a table of
nine rows, and then over ten. The table is now declared the population
of record with the number struck from the prose — this program's
count-fixed-in-one-place rule applied to the section that tabulates it.

**Two residues filed rather than fixed, both verified here before the
merge.** `escape-commits-a-free-move-instead-of-abandoning-it`: egui
aborts a drag on Escape by clearing `dragged`, so `drag_stopped` fires
and `drag_gesture_ops` (`widgets.rs:95-98`) emits **`CommitFreeMove`** —
the key every other control spells *abandon* lands the probe. And
`a-keyboard-bump-lands-and-closes-the-pointers-own-probe`: all three
components name one instance, so after the second begin is refused the
same batch's `preview_free_move` passes its instance check
(`display.rs:778-779`) and `commit_free_move` (`display.rs:806`) lands
it and closes the gesture — the user is shown a refusal naming a state
the same batch destroyed. I read both call sites; both hold.

**One hole the lane disclosed rather than let pass.** Its new row's doc
comment carries two intra-doc links, and rustdoc builds under `cfg(doc)`
not `cfg(test)`, so **neither doc pass judged them**; it checked both
targets by hand. That is ground `cfg-test-bare-spans-have-no-stated-
disposition` and `comment-symbol-names-outside-rustdocs-reach-have-no-
gate` already own.

**VIEW stands at 71 open / 80 closed, nothing waiting on Ev.**

**A count correction inside the entry about count corrections.** I
first wrote *73 open* here from arithmetic in my head — previous total,
minus the row closed, plus the two residues filed. `work.py status`
said 71. I then miscounted the files by hand and got 72, decided the
tool and its own item table disagreed, and started reading `work.py`
for the bug. **There was no bug.** My shell loop globbed
`work/view/*.md` and `program.md` carries a `status: open` of its own —
the program's status, not an item's. The tool was right at every step
and both of my counts were wrong, in two different ways, in the space
of five minutes. `work.py status` is the count of record; a number
reached any other way is a guess wearing a number's clothes.

## 2026-09-11 — `view/escape-abandons`: Escape ends a drag as a cancel

**Shape 1, and the argument that settles it is not the one the item
gave.** The item's fork was *read Escape where the triple is mapped* or
*ratify that a probe lands whatever it previewed*. Both readings rest on
whether the chrome has another way to abandon, and it does not: the
cancel doors are TOOLBAR controls, so pressing one costs the pointer
release that lands the value. `cancel_doors` is enabled during a live
pointer drag and unreachable during one. Until this change a held drag
had no abandon at all, and the one input that can end it other than a
release — `egui`'s Escape abort — was translated into the commit.

**The gesture triple became a `GestureVocabulary`** with a fourth
operation (`crates/viewer/src/widgets.rs:38-49`), and
`drag_gesture_ops` emits that one instead of the commit on a
`drag_stopped` frame carrying an Escape press (`:125-151`). A struct
rather than a fourth positional parameter, and not only because clippy
counts to seven: `commit` and `cancel` are both bare `SessionOp` and
mean opposite things, so positionally they sit one transposition away
from a chrome that lands what the user abandoned.

**Three corrections to the item, all from re-deriving its premise
against the tree rather than reading it.**

- **No `Option`.** The item said the fix *"needs a fourth operation
  parameter and an `Option` for the vocabularies that have no cancel"*.
  There are none: all three vocabularies the panel maps carry a cancel,
  and `gesture_table.rs`'s `every_gesture_cancel_has_a_chrome_door`
  matches exhaustively over `SessionOp`, so a gesture that joined the
  enum without one would red there first. The parameter is a
  `SessionOp`.
- **The value gesture has the same defect, with the LARGER stake.** Same
  function, same release arm. A free-move commit lands a display frame
  no history holds; a value-gesture commit reaches the document and
  costs an undo step. Measured rather than read: with the Escape branch
  removed, both new rows report `["commit"]` where `["cancel"]` belongs.
- **The premise the orchestrator asked me to check held.**
  `PreviewFreeMove`'s *"the identity is one node rather than a target"*
  was written BY #2361, in the same diff that gave the value drags their
  names, and is still true of the tree (`op.rs:309-323` against
  `:188-206`). It is also not what the fork turns on: the cancel side is
  where the two gestures agree, because both cancels name nothing.

**Read the key, do not infer it.** `drag_stopped` with no pointer
release would have been a proxy — a long touch ends a drag with no
release too (`egui-0.36.1/src/interaction.rs:143-155`) and means a
context menu, not an abandon. The property is *the user pressed
Escape*, so the branch asks that.

**Two prose claims the change made false, both amended in the same
diff.** `crates/viewer/README.md`'s *"There is no key for it… an Escape
binding is that decision and not a row to add"*, and `input::PRESETS`'
*"no key denotes an operation anywhere"*. The honest amendment is not
that the claims survive: the crate now reads exactly one key. What
survives is the narrower fact — `egui` ends a drag on Escape whatever
this crate does, so the branch decides which of two things the toolkit
already did is reported, not which operation a key denotes, and a
keyboard vocabulary still needs every decision `PRESETS` names.

**The citation census over the bands this diff moved.** Five files
shifted. Open rows citing into them, by subject rather than by delta:
two repointed (`the-two-drags-name-their-gestures-in-two-shapes` —
`op.rs:301-315`→`309-323`, `widgets.rs:57-100`→`78-151`,
`properties.rs:557-574`→`563-583`, and its *"three ops"* is now a
four-field value;
`a-disabled-control-says-why-in-four-shapes` — `properties.rs:208`→`211`
and `:727`→`736`). **Three were already wrong at the merge base and are
left alone rather than shifted onto something else**:
`comment-symbol-names-outside-rustdocs-reach-have-no-gate`'s
`session/op.rs:828` (the site is deleted, which that row's own note
already says), `is-instance-collapses-absent-and-wrong-kind`'s
`properties.rs:336` (`instance_ui` is at `:338` at the base), and
`a-disabled-control-says-why-in-four-shapes`' `properties.rs:350-355`
(the `ui.weak(fault.to_string())` is at `:356`; `plan.md` cites a third
band, `:348-353`, for the same subject). `stale-file-citations-after-
the-split`'s `op.rs` numbers are QUOTATIONS — a table of what a past
repoint said — and are not repointed for the same reason.

**Out of fence, reported not edited** (implementer-discipline §6):
`work/door/dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum`
(`:14`) and `work/door/plan.md` (`:66`) cite
`crates/viewer/src/pane/properties.rs:159-163`, and
`work/census/the-prose-word-for-a-kind-has-four-spellings-and-only-
display-is-censused` (`:36`) cites `:158-161`. **Neither is a true
shift, and the subject check is what says so**: the inline
`(Dimension::Length, "Length")` array both rows are about is at
`:162-167` at my merge base and `:165-170` at my head, so DOOR's band
names the name field and catches only the array's first element, and
CENSUS's names the name field alone. Both were wrong before this diff;
the +1 is real but repointing either band would move a number that was
never about its subject.

## 2026-09-11 — #2390 merged; Escape abandons, and the lane beat my argument for it

**#2390 merged** (`6c758dab86`), full code tier verified from the job
list: **38 jobs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, six skips (two cache primes, the two interval-backend rows,
`step import (freecad)`, `python suite`), no unsubstituted
placeholders. The lane settled the CI-scope question by **running**
`scripts/ci-filter.py`, which returned `SEEDS=viewer` →
`RUN_VIEWER_TOOLKIT=true` → the non-skip path, confirming from the tool
that this PR's own CI could not reach the skip-mode doc pass it ran by
hand.

**Escape now abandons a drag instead of landing it**, at the slot
field, the parameter field and the free-move probe alike.

**My dispatch argued the fork the weak way and the lane said so.** I
wrote that the chrome already draws a *"Cancel free-move"* door, so
Escape should cancel. That is beatable — a door existing does not
oblige a second route to it. The argument that actually settles it:
**`cancel_doors` cannot be reached during a live pointer drag at all.**
They are toolbar controls; pressing one costs the pointer release, and
the release ends the drag through the *commit* arm. So for the whole
life of a held drag the doors are enabled and unoperable — they are the
STRANDED drag's exit, which is what `crates/viewer/README.md` says they
were built for. Until this change **a drag under the pointer had no
abandon at all**, and the only non-release input that could end it was
being reported as a release. Shape 2 would have had to argue that a
user holding the button has no business abandoning.

**The value gesture had the same defect and the larger stake**, and the
lane established it by mutation rather than by reading: with the Escape
branch replaced by an unconditional commit, both new rows go red
reporting `["commit"]` where `["cancel"]` belongs. The asymmetry runs
opposite to the guess — a free-move commit lands a display frame no
history holds, while a **value-gesture commit reaches the document and
costs an undo step**.

**Three corrections to the item and to me.** (1) The item said the fix
needs an `Option` for vocabularies with no cancel; there are none, and
that is structural — `gesture_table.rs`'s
`every_gesture_cancel_has_a_chrome_door` matches exhaustively over
`SessionOp`, so a gesture joining the enum without a cancel reds there
first. The parameter is a plain `SessionOp`. (2) I flagged
`PreviewFreeMove`'s *"identity is one node rather than a target"* as
possibly stale after #2361; it is not — **#2361 wrote that sentence**,
in the same diff that named the value drags, and it is not what the
fork turns on, because the cancel side is where the two gestures
AGREE: both cancels name nothing. (3) The reliability worry was the
wrong way round. Escape is readable; what is unreliable is the proxy
the lane nearly used — *"`drag_stopped` with no pointer release"* also
matches a long-touch context menu and means something else entirely.
**Named the property, then picked the pattern** — the twelve-row table
applied before the fact rather than after it.

**Ratified crate prose was amended rather than weaselled, and that is
worth flagging.** `crates/viewer/README.md` said *"There is no key for
it: this crate binds no key to any operation at all"*, and
`input::PRESETS` said *"no key denotes an operation anywhere"*. Shape 1
makes both false, so the lane rewrote them. What survives is narrower
and is the honest version: `egui` ends the drag on Escape whatever this
crate does, so the branch decides **which of two things the toolkit
already did is reported**, not which operation a key denotes —
*reading is not binding*. I checked `docs/DESIGN.md` before merging:
it carries no keyboard-binding claim (its "key" hits are map keys and
escape hatches), so this is VIEW's own text and not an Ev gate.

**The commit/cancel transposition is now unrepresentable.** The triple
became a named `GestureVocabulary` struct — not for clippy's
`too_many_arguments`, which does fire at eight, but because `commit`
and `cancel` are both bare `SessionOp` and mean opposite things: sitting
positionally adjacent they are one transposition away from a chrome
that lands what the user abandoned.

**The sibling residue is unaffected**, checked rather than assumed: the
second probe in `a-keyboard-bump-lands-and-closes-the-pointers-own-probe`
is opened by the *typed* arm, which Escape does not reach, because on an
Escape frame `changed()` is false — egui declines to apply the typed
edit. Its test row now shares a five-frame preamble helper instead of
carrying its own copy; what it asserts is unchanged and it still passes.

**Out of fence, reported and not edited** — and the lane's reading of
them is the sharper half. `work/door/dimension-radio-row-…:14`,
`work/door/plan.md:66` and `work/census/the-prose-word-…:36` all cite
bands near a `(Dimension::Length, "Length")` array that this diff moved
by three lines. **Neither is a true shift**: both were pointing at the
name field rather than at the array, so they were wrong before this
diff, and repointing them would be moving a number that was never about
its subject. Three VIEW rows in the same state were likewise left alone
rather than shifted (`comment-symbol-names-…`'s `op.rs:828`, whose site
is deleted; `is-instance-collapses-…`'s `properties.rs:336`;
`a-disabled-control-…`'s `properties.rs:350-355`) — and `plan.md` cites
a *third* band for that last subject, which is the census-table rule
earning itself again.

**VIEW stands at 70 open / 81 closed, nothing waiting on Ev.**

## 2026-09-11 — `view/keyboard-bump`: the probe's three boxes become one gesture

`a-keyboard-bump-lands-and-closes-the-pointers-own-probe` **closed,
fixed at the CHROME.** The fork the item left open was *name the
component, or the gesture, at the operation* against *one gesture
mapping for the three components at the chrome*. The chrome side wins,
and the argument that settles it is not taste:

**The operation side cannot supply an identity the chrome does not
already hold.** `widgets::drag_ops` is handed the whole vocabulary as
VALUES before any of it is performed, and the typed arm literally
builds `vec![Begin, Preview, Commit]` — so no payload in that batch can
carry a token the begin returned. A client-minted id would work, and
then the chrome is the thing deciding which drivers are one gesture,
which is the chrome fix with an extra field on four operations. The
component is dead outright: the op takes any rigid `Frame`
(`crates/viewer/src/session/op.rs:321-335`) and three translation boxes
are one chrome's decomposition of it.

So `widgets::vec3_row_ops` (`crates/viewer/src/widgets.rs:168-211`)
draws the row, unions the three responses with `egui::Response`'s `|`
— egui's own documented summary of a row — and calls `drag_ops` once.
Under the union `dragged()` means *the pointer is holding this gesture*
rather than *this box*, which is the question the typed arm
(`changed() && !dragged()`) was already asking and getting a per-box
answer to. `drag_ops` and `drag_gesture_ops` became generic over the
gesture's value type for it (`:93-166`); that is forced, not
decorative, because once the row is one gesture there is no single
box's number to pass.

**The outcome is better than a refusal, which is worth saying.** The
keyboard bump on a sibling box is not wrong and does not need
refusing: the instance has one probe and all three components drive
it, so the keystroke is another hand on the open gesture. It emits one
`PreviewFreeMove` and the release still lands everything. The
per-box mapping turned a legitimate input into a begin the door
refused and a commit that closed the drag under the pointer.

**#2390's two claims about this item, both re-derived rather than
taken.** (a) *Escape does not reach the typed arm* — holds, and by
construction rather than by observation: `DragValue` marks itself
changed only on `get(..) != old_value`
(`egui-0.36.1/src/widgets/drag_value.rs:671-673`) and both of its
write paths are guarded on `!key_pressed(Escape)` (`:540`, `:582`), so
an Escape frame cannot make `changed()` true and `changed() &&
!dragged()` cannot fire. (b) *the test row moved to a shared
five-frame preamble with its assertions unchanged* — holds for the
row's own two assertions. One word moved inside the preamble that was
extracted: the opening assertion's message read *"the pointer drag
opens a probe and holds it open"* and reads *"opens a gesture"* now,
and a comment about egui needing pointer motion before it calls a drag
a drag was dropped in the move. Neither changes a predicate.

**`the-two-drags-name-their-gestures-in-two-shapes` is neither closed
nor mooted nor conflicted.** It asks where the gesture-identity CONCEPT
lives across six operations in three spellings; this fix adds no
spelling and removes none. It is strengthened if anything: the probe's
identity is now *the instance, because the chrome gives it one driver*,
which is a fact about a convention rather than a type — exactly that
row's complaint.

**Residue, filed rather than disclosed**:
`probe-identity-stops-at-the-instance`. The door still cannot refuse a
second DRIVER on one instance; today's chrome has one, and
*unreachable from today's chrome* is precisely the claim
`free-move-in-flight-refusal-has-no-reachable-producer` was filed on
and that was false twice over. `gesture_table.rs`'s
`a_drag_on_another_field_cannot_steer_the_open_one` is the value drag's
row for this property; the probe has no counterpart because there is
nothing for one to assert.

**`DisplayFault::FreeMoveInFlight` keeps a producer** — `Open` and
`NewDocument`, the two `false` rows of `permitted_during_free_move`
(`crates/viewer/src/session/op.rs:875`), raised at
`crates/viewer/src/session.rs:1089-1091`. The fix removes the keyboard
route to it and not the fault.

**Sweep**: every chrome site that emits a gesture triple, found by
grepping `SessionOp::(Begin|Preview|Commit|Cancel)` under
`crates/viewer/src/` — three, all in `pane/properties.rs`. The
parameter drag (`:98`) is one widget and one gesture. The probe
(`:388`) was three widgets and one gesture: fixed. The slot row
(`:563`) is three widgets and three gestures and is not an instance, on
two independent grounds — each component is its own `SlotId` so the
identity is per-field, and it calls `drag_gesture_ops` with no typed
arm at all, its typed path being `SetSlot`. **Blind spot**: the pattern
is a literal `SessionOp::` constructor in `src/`, so it cannot see a
site that builds a gesture op through a helper or a variable, nor one
outside `crates/viewer/src/`.

**One prose universal corrected.** `crates/viewer/README.md`'s *"A
driving operation names its own gesture"* section ended *"The subject
of a driving operation is the field the user has hold of, and naming it
is what makes the mismatch refusable"* — stated over all six driving
ops, and false of the probe, whose three boxes were three fields over
one named subject. The section now carries the probe's asymmetry, what
makes it enough, and the row that owns what it does not close.
`SessionOp::PreviewFreeMove`'s *"the identity is one node rather than a
target"* is KEPT and argued rather than corrected: #2361 wrote it
deliberately and it is right — one node is the whole identity because
an instance has one probe.

**Citation census over the bands this diff moved** (`widgets.rs`,
`pane/properties.rs`, `session/op.rs`, `crates/viewer/README.md`):
47 rows carry a citation into those four files; mapping every one
through the diff gives 45 citations that actually move, across 24 rows
— 46 counting the one written bare, below, which a filename-anchored
scan cannot see. Six of those 24 rows are open, this item among them.

**Repointed**, subject checked at the base and at the head:
`a-disabled-control-says-why-in-four-shapes`'
`pane/properties.rs:736` → `:734` (the `on_disabled_hover_text`
literal), and `the-two-drags-name-their-gestures-in-two-shapes`' four —
`op.rs:309-323` → `:321-335`, `widgets.rs:78-151` → `:93-166`,
`widgets.rs:38-49` → `:46-57` (written bare, which a filename-anchored
scan does not see), `pane/properties.rs:563-583` → `:561-581`.

**Left alone and disclosed, because the number was never about its
subject**: `two-hand-written-copies-of-the-g1-gesture-machine`'s
`widgets.rs:30-52` names `crate::widgets::drag_ops` and points at
`GestureVocabulary`'s doc and struct — #2390 inserted that struct above
`drag_ops` and the row has been stale since. `stale-file-citations-
after-the-split`'s `op.rs:586` calls itself the exhaustive table and
lands inside `SessionOp::AddChamfer`'s doc comment; it and `:633` are
QUOTATIONS in a table recording what a past repoint said, which is a
record rather than a pointer. `comment-symbol-names-outside-rustdocs-
reach-have-no-gate`'s two `session/op.rs:828` are worked-example sites
the row's own Note already records as deleted. The eighteen closed rows
in the population are records, not guards, and are untouched.

**Nothing out of fence.** Every citation this diff moved is in
`work/view/`.

## 2026-09-11 — #2392 merged; the fix that closes a route and says so

**#2392 merged** (`2040450f88`), full code tier verified from the job
list: **38 jobs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, six skips (two cache primes, the two interval-backend rows,
`step import (freecad)`, `python suite`), no unsubstituted
placeholders.

**The lane took the chrome side of the fork on a structural argument,
not a preference.** The operation side *cannot* supply an identity the
chrome does not already hold: `widgets::drag_ops` is handed the whole
vocabulary as VALUES before any of it is performed, and the typed arm
literally builds `vec![Begin, Preview, Commit]` — so no payload in that
batch can carry a token the begin returned. A client-minted id would
work, but then the chrome is the thing deciding which drivers are one
gesture, which is the chrome fix with an extra field on four
operations. The component flavour is dead outright, because the op
takes any rigid `Frame` and three translation boxes are one chrome's
decomposition of it.

So `vec3_row_ops` draws the row, unions the three responses with
`egui::Response`'s `|` — egui's own summary of a row — and calls
`drag_ops` **once**. Under the union `dragged()` means *the pointer is
holding this gesture* rather than *this box*, which is the question the
typed arm was already asking and getting a per-box answer to. The
previewed value is composed after all three are drawn, so a component
changed this frame is in the value this frame previews.

**And the keyboard bump turns out not to be wrong.** The instance has
one probe and all three components drive it, so the keystroke is
another hand on the open gesture: one `PreviewFreeMove`, release still
lands everything. The per-box mapping was turning a legitimate input
into a refused begin and a commit that closed the drag under the
pointer. `PreviewFreeMove`'s *"the identity is one node rather than a
target"* is kept and argued rather than overturned — one node is the
whole identity **because an instance has one probe**, and what makes
that enough is that one chrome gesture drives it.

**#2390's two claims about this item both held, one of them more
strongly than reported.** Escape does not reach the typed arm *by
construction*: `DragValue` marks itself changed only on
`get(..) != old_value` and both write paths are guarded on
`!key_pressed(Escape)`. The shared preamble moved one word and dropped
one comment; no predicate changed.

**The half the fix does NOT close is filed, not disclosed in prose.**
`probe-identity-stops-at-the-instance`: closing the reachable route
leaves `preview_free_move`/`commit_free_move` still unable to refuse a
second DRIVER on one instance — today's chrome merely has none. That is
precisely the shape `free-move-in-flight-refusal-has-no-reachable-
producer` was filed on and wrong about twice, so **"no chrome can reach
it" is recorded as a property of the chrome, not of the door.** The
value drag has `gesture_table.rs`'s
`a_drag_on_another_field_cannot_steer_the_open_one` for this property;
the probe has no counterpart because there is nothing for one to
assert.

**A rule for the next dispatch that touches this module**, which the
lane is right about and I had not seen: the test module **hand-copies**
the production wiring, so a fix sited in `pane/properties.rs` would
leave the row asserting a wiring it wrote for itself — green whatever
`instance_ui` did. That forced the fix into `widgets.rs`, and it is the
better home anyway. **A row in `widgets.rs`'s test module can only hold
what `widgets.rs` owns.**

**Mutation receipt**: with `vec3_row_ops` handing `drag_ops` the box
that *changed* instead of the row, the new row reports
`["preview", "begin", "preview", "commit"]` — the item's defect exactly
— and reds; the other four rows stay green under it.

**The lane's own cost, self-reported:** it pushed a one-paragraph
rewrap of the log after CI had started, which cancelled a 37-job run
and cost a full re-run. **Batch the cosmetic pass before the first
push** — the first push is what starts the clock.

**VIEW stands at 70 open / 82 closed, nothing waiting on Ev.**

## 2026-09-12 — `view/possessive-spans`: the picker exists and it is `length_picker`; five possessives become links

`possessive-code-spans-are-invisible-to-the-path-shaped-sweep-rule`
closed. Both halves answered in one PR, which is what the row asked for.

**Half 1, `blend.rs`.** `unit_picker` has a referent and it is
`crate::widgets::length_picker`, *"The creation forms' written-unit
picker"*. `pane/create.rs` draws it in the same `ui.horizontal` as the
field `BlendKindChoice::size_label` labels, immediately after
`unit_field` — so the affordance the sentence asserts exists and this was
a bad NAME, not a false claim about the chrome. Both halves of the name
were wrong: module and item. `length_picker`'s own doc carries the same
clause nearly verbatim (*"free to say metres beside a field written in
millimetres"*), so the sentence was copied from it and mis-attributed.
The alternative outcome the brief asked me to watch for — that the chrome
has no such control and the sentence is asserting an affordance that does
not exist — did not obtain.

**Half 2, the fork: LINKED, all four.** Ev's 2026-09-11 ruling is what
made a bare span optional rather than forced, and it settles the spelling
too: at `origin/main` this crate already links into the `app`-gated half
nineteen times from the renderer-free half (thirty-three spans crate-wide),
so *"a possessive span is prose"* would have been a second answer to a
question already ruled, and the row's own complaint is that one
relationship is spelled two ways. `ViewerApp::fit_delta_on_scene` is a
private FIELD and `ViewerApp::remember_theme` a private method; both
resolve because both host doc passes run `--document-private-items` with
`rustdoc::private_intra_doc_links` allowed, and `doc-gate.sh`'s selftest
already pins *a public link to a private sibling*. **The disposition is
stated once**, in `crates/viewer/README.md`'s *Rustdoc posture* section,
beside the ruling it follows from and carrying the sweep rule that
produces its population.

**The wider sweep found a second `unit_picker`.** Dropping the
module-name requirement — any `` `X` ``'s `` `Y` `` pair on one `///` or
`//!` line — gives fifteen sites at base where the row's rule saw four
(`prefs.rs`'s pair is split across two lines, so the two rules together
see sixteen). Resolving every second span turned up
`widgets.rs`'s test-module `crate::pane::properties`'s `slot_row_ui`,
which exists nowhere; it is `slot_value_ui`, the one `properties.rs`
function that calls `drag_gesture_ops` directly. Also moved: `gpu.rs`'s
`` `crate::pickindex`'s `OCCLUSION_SLACK_REL` ``, the other end of
`pickindex.rs`'s deliberate pointer pair — leaving one end a link and the
other a possessive would re-mint the defect inside the pair.

**Two notes onto `comment-symbol-names-outside-rustdocs-reach-have-no-gate`,
which are notes and not a diff to the gate.** A gate built to that row's
spec (`<own-mod>::<ident>` resolves) would still have missed
`unit_picker`, because `unit_picker` carries no module qualifier at all —
the qualifier was in a separate span, and was the wrong module. And the
`slot_row_ui` site is that row's own axis in a shape it does not list: a
`///` comment inside a `#[cfg(test)] mod` is read by no rustdoc pass this
repo runs, so a bracket there would be punctuation for a reason
unrelated to `//` versus `///`.

**One citation in the closed row is left alone and disclosed.** Its
blind-spot list cites `theme.rs:69-70` and quotes *"`app` maps a
[`Theme`] onto the chrome"*. `69-70` is a genuine member of the class
(*"`app` maps this onto the toolkit's own light and dark `Visuals`"*),
but the quoted words are at `theme.rs:7-8`, where `app` is already
written `` [`crate::app`] `` and is therefore not an example of a bare
span. The number names its subject; the quotation is what is wrong, so
nothing is repointed.

**What the widened rule still cannot see**, stated because the row was
held to this standard and so is its closure: a module or item named in
prose with no backtick span at all — `sketch.rs`'s *"(`app`'s drafts say
so)"* names the `drafts` module in bare words, and the possessive
pattern matches the `app` half while the thing it is about is unspanned;
and any name in a `//` comment, which is the open row's subject.

## 2026-09-12 — #2400 merged; the thirteenth proxy is mine and I had been repeating it all session

**#2400 merged** (`4c4a784c20`), full code tier verified from the job
list: **38 jobs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, six skips, no `render drift (…)` row posted at all.

**`unit_picker` had a real referent and the sentence was true.**
`crate::widgets::length_picker` (`crates/viewer/src/widgets.rs:578`),
*"The creation forms' written-unit picker"* — and
`pane/create.rs:1099-1108` draws `ui.label(size_label())`, then
`unit_field`, then `length_picker` in one `ui.horizontal`: the picker
literally beside the field. So the citation was wrong in **both** halves,
module and item, and the affordance exists. The giveaway is that
`length_picker`'s own doc carries the same clause nearly verbatim
(*"free to say metres beside a field written in millimetres"*), so the
sentence was copied from it and mis-attributed. The bigger finding I
told the lane to watch for — a sentence asserting an affordance the
chrome does not have — did not obtain.

**My `forms.rs` hint was wrong about the file.** That crate's "picker"
prose is `PathVerb`/`ArcMode` combo pickers, a different control. The
method I gave (follow the word) was right and the destination was not;
I had flagged it as a weak hint, so no cost.

**The widened sweep found a SECOND dead name.** `widgets.rs:678` said
`crate::pane::properties`'s `slot_row_ui`, which exists nowhere; it is
`slot_value_ui` (`pane/properties.rs:519`), the one function there
calling `drag_gesture_ops` directly. Both dead names are now fixed and
`grep -rn slot_row_ui crates/` is empty. The widenings: dropping *"a
second span must follow"* gives 8 sites; dropping the module-name
requirement too gives 15; together 16, where the item's rule saw **5**.
Stated blind spot of the wider rule: a name in prose with **no backtick
span at all**, and any name in a `//` comment or inside a
`#[cfg(test)] mod`.

**"Either link them or leave them" was not available for all four.**
`pickindex.rs:1582` points at `EDGE_CLIP_Z_SHRINK`, a module-scoped
private `const`. Linking it made `scripts/doc-gate.sh --pr` **exit 1**:
*no item named EDGE_CLIP_Z_SHRINK in module gpu*. A private **field**
and a private **method** DO resolve — rustdoc reaches an associated item
through its type — so the item's stated worry about "a spelling for a
private field" was a non-issue, and `ViewerApp::fit_delta_on_scene` and
`::remember_theme` linked fine. A module-scoped private `const` or `fn`
is a different test: `--document-private-items` decides what rustdoc
RENDERS, while a path is resolved by ordinary **visibility**, and
`crate::gpu::EDGE_CLIP_Z_SHRINK` is not a path anyone outside `gpu` may
write. The lane reproduced it on a three-file scratch crate, so it is a
language fact and not a tree fact. That pair stays named at both ends,
normalised to one spelling, with the exception and its measured error
text in the README clause. The single statement of the disposition lives
in `crates/viewer/README.md`'s *Rustdoc posture* section beside Ev's
ruling, with the sweep rule that produces its population.

### The thirteenth proxy instance is mine, and it is the one I was using to verify every merge

**I read `docs-only ok` success as the docs-tier marker.** I wrote, this
session, *"21 jobs, `docs-only ok` success — the docs-only tier
exactly"*. That job concludes **success on the full code tier too**:
#2390, #2392 and #2400 each carry a green `docs-only ok` inside a
38-job closure-tier run, beside `gate ok`. So its presence is no
evidence of a tier at all, and the 21-count was carrying the whole
argument by itself every time. **The reading was never falsified
because on a docs-only run both facts are true at once** — the proxy
agreed with itself on every observation it could make, which is the
class exactly. What makes this one worth the row is that it was the
instrument, not the object: I was using it to certify the merges in
which I was also tabulating the class.

Recorded in the table and in the prose beside it, with the verification
restated: a tier is **21 jobs**, or **38-39 with 12 `test (…)` and 5
`k-lint (gate, …)`**, plus `gate ok` — never which summarising job
reports green.

### And a validation command that silently did not validate

`cargo test -p viewer --features app`, which this register and several
of my dispatches specified, **aborts at the `--lib` adapter red** (no
Vulkan on the lane boxes), so the **524-row `--test all` suite never
builds**. A lane following the brief literally reports a green-looking
`37/1` and never runs the suite its own diff is about. It needs
`--no-fail-fast`; #2400's lane ran it that way and then told me the
brief was wrong. Both the command and its expected shape are now in
`plan.md`.

**Citation left alone and disclosed**, the house rule working: the
closed row cites `theme.rs:69-70` and quotes *"`app` maps a [`Theme`]
onto the chrome"*. `69-70` **is** a genuine class member; the quoted
words are at `theme.rs:7-8`, where `app` is already `` [`crate::app`] ``
and so is not an example of a bare span. The number names its subject —
the **quotation** is what is wrong, so nothing was repointed.

**VIEW stands at 69 open / 83 closed, nothing waiting on Ev.**

## 2026-09-12 — the δ render, and the second path that committed it

`delta-field-renders-a-sub-micrometre-delta-as-zero` is **closed**, both
halves, by two changes with nothing between them.

**The render is chosen by the property, not by a precision.**
`DisplayTolerance::render_mm` returns the shortest decimal spelling of δ
in millimetres that fits ten characters and reads back — through the
`mm * 1.0e-3` the field's own commit path uses — as a δ
`DisplayTolerance::new` accepts, within four significant figures of this
one; `{:.3e}` carries the δ no decimal spelling can. So the fix has no
threshold to go stale against a format string: the candidate is read
back through the door that refuses zero, and the door is the only judge.
0.4 µm reads `0.0004`, 1 pm reads `1.000e-9`, and a swept grid from
`f64`'s smallest subnormal to a kilometre has no render the door would
refuse.

**The relative tolerance is derived, not chosen.** `{:.3e}` carries four
significant figures, so it can misread the δ it renders by 5·10⁻⁴ of it.
That is the number a decimal spelling is held to — a decimal form is
preferred exactly while it is no less truthful than the form that would
replace it — so the constant is the fallback's accuracy rather than a
taste about how a field looks.

**The field is 88 points, and the test says why.** A ten-character
render needs 73.3 points of text area; 56 points offered 48, so anything
past six characters was clipped, and a clipped render reads as a
different δ — the defect the render's own bound exists to prevent.
`the_field_shows_the_longest_render` measures both numbers through
egui's font metrics at the widest character a render can use, so the
width is pinned to the render's bound rather than to a sentence.

**Half B needed no precision at all.** A draft that reads as the render
commits nothing: the render is the text the box already held, so a field
typed back to it carries no number the render does not, and since the
render is a rounding of δ, committing it could only move δ to a coarser
spelling of itself. The escape hatch is any other spelling of the same
number, and a row holds it open.

**Filed, not swept:**
`fixed-precision-length-renders-can-read-as-a-value-they-cannot-be`.
The badge and the budget sentence hold a `DisplayTolerance` and could
call the new render today; what stops that being this unit is that both
render δ inside a SENTENCE, so the change is a wording decision about a
badge and a status line rather than a control's arithmetic, and
`display_budget.rs`'s needles would move with it. `Bounds::wording`
renders a probed length in the user's own unit and cannot use a δ method
at all — it needs the same rule at its own precision. **And the sweep
that wrote that row found a member the parent had not named**: the
camera readout at `pane/view.rs:43`, `distance {:.1} mm (band …)`, where
`min_distance` is `scene_radius * 0.05` so every part under about a
millimetre reads `band 0.0–…`. It is in the same function as the δ
field, and what keeps it out of this fix is the line above it — an
ANGLE at the same precision, where `0.0°` is a yaw a camera really has.
The pattern was a precision spec in a format string; what it cannot
match is a render with no precision spec, a precision passed as a
variable and a rounding done by hand, all three checked in the item.

**VIEW stands at 69 open / 84 closed, nothing waiting on Ev.**

## 2026-09-12 — #2425 merged; a render that cannot lie, and four corrections to the brief

**#2425 merged** (`99550244d6`), full code tier verified from the job
list: **38 jobs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, six skips, nothing failed.

**Half A took none of the three shapes the dispatch named.** The render
is now `DisplayTolerance::render_mm` — the shortest decimal spelling
that fits, scientific when none does — with the arm chosen by **reading
the candidate back through the door that refuses zero**, not by a
magnitude threshold:

    (0..=RENDER_MM_MAX_CHARS).map(|d| format!("{mm:.d$}"))
        .find(|s| reads_back_as_this_delta(s, mm))
        .unwrap_or_else(|| format!("{mm:.3e}"))

0.4 µm → `0.0004`, 1.6 µm → `0.0016`, 1 pm → `1.000e-9`. A threshold
would be a **proxy** for *"the fixed form lies here"* that can drift from
the format string; the read-back **is** the property, so `0.000` is
structurally unreachable rather than merely unlikely. The lane also
measured and rejected *exact-while-it-fits*: that arm depends on the bit
pattern, so δ = 0.1+0.2 mm renders `3.000e-1` and the field flips
spelling for ordinary values. Its relative-closeness constant is derived
rather than chosen — `{:.3e}` carries four significant figures, so a
decimal form is preferred exactly while it is no less truthful than the
form that would replace it. **A budget δ's seventeen figures render as
four** (`0.0003746`), which is why the text stays a render and never a
commit path.

**Half B took the option I named, on a better argument than mine.** I
offered "a draft character-identical to the render commits nothing" as
the *untouched commits nothing* principle extended. The lane's argument
is stronger: since the render is a **rounding** of δ, committing it can
only move δ to a coarser spelling of itself, so **there is no δ for
which committing its own render is the user's intent** — which means the
cost I flagged (a deliberate re-assert gets silence) is not a cost at
all, and the escape hatch is any *other* spelling of the same number,
which still commits. It also declined the broader *"any draft that
parses to the δ in force is a no-op"*: that is idempotence of the
request and belongs to `delta_request`'s consumer, not to a field whose
job is to tell a render from a draft.

### Four corrections, all to the brief rather than to the work

1. **The sibling population was three because I repeated it instead of
   re-deriving it.** There are **four**, and the fourth is in the file
   the lane was editing: `pane/view.rs:43` renders `distance {:.1} mm
   (band {:.1}–{:.1})`, and `Camera::min_distance` is
   `scene_radius * 0.05`, so any scene under about a millimetre of
   radius reads `band 0.0–…` — a distance the camera refuses. **I
   verified this before merging.** This program's own census rule says a
   table is a population and placing it means re-deriving by subject; a
   population quoted from the item into the dispatch is that defect one
   step earlier.
2. **`plan.md` was carrying a moving number as a fixed expectation.**
   The `--test all` baseline was written as **524** and `main` is at
   **527** four merges later, so a lane checking against it would read a
   six-row gain it had not made. The register now asserts the SHAPE —
   every `--test all` row passing, `--lib` one row red, that row and no
   other — and says explicitly not to put the count there. The
   `--no-fail-fast` half of that rule is right and the lane confirmed it
   would have missed the suite without it.
3. **Naming the unit-switching render as a fork option was a trap, not a
   neutral offer.** The box is labelled `mm display δ` and parses
   millimetres, and the same brief's Half B says the render is the text
   an edit starts from — so a µm render is a **1000× wrong commit one
   keystroke away**. It is not merely less legible than the
   alternatives; it is the one shape that makes the defect worse. A fork
   option named in a dispatch inherits the rest of the dispatch, and I
   had not checked it against the other half.
4. **"A 56-point field" understated the ceiling by more than it
   sounds.** `desired_width` is not a character budget: `TextEdit`'s
   `Margin::symmetric(4, 2)` leaves **48 points of text** at ~7.33 per
   digit — about six and a half characters — so the old field could not
   display `0.001667` even before any render change. A fix to the render
   that left the width alone would have been delivered **clipped**, and a
   clipped render reads as a different δ. The field is now 88 points,
   with a row measuring both numbers through egui's own font metrics and
   going red at 56. **That width change is a scope call the lane flagged
   for me rather than slipping in**, and it is the right one: Half A
   undelivered is Half A unfixed.

All four are in `plan.md`.

**The sibling class is filed on VIEW's own slate** as
`fixed-precision-length-renders-can-read-as-a-value-they-cannot-be`,
carrying all four members. Filing there is in-fence; the lane reported
rather than wrote while `work/door/lane-cross-program-filing-two-binding-
docs-conflict` is open for Ev, which is the conservative side of that
unresolved ruling.

**VIEW stands at 69 open / 84 closed, nothing waiting on Ev.**
### 2026-09-12 — the four fixed-precision length renders, through one door

`fixed-precision-length-renders-can-read-as-a-value-they-cannot-be`
**closed**. The shape chosen was ONE door, not four decisions and not a
generalisation of `render_mm` off `DisplayTolerance`: two of the four
sites hold no such value, so a rule living on that type would have been
spelled twice — once as the method and once by hand at the sites
without it, which is this program's N-spellings-no-home shape arriving
while a different defect was being fixed.
`crates/viewer/src/readout.rs` is the home; `render_mm` is the δ-facing
door onto it and keeps every caller and every existing row.

The camera readout took the decision the row asked for rather than a
patch per number: its two ANGLES keep `{:.1}°` and its three DISTANCES
do not, because `0.0` is a yaw a camera has and is not a distance a
camera can be at.

**`display_budget.rs` turned out to assert less than the row feared.**
`a_coarsened_picture_says_so_in_both_numbers` built its needles with
`format!("{:.3}", …)`, so it held the sentence to a FORMAT rather than
to the δ — a needle that would have kept passing by accident on one of
the two numbers (the budget δ's render is a superstring of its own
`{:.3}` rounding) and failed on the other. The needles are the renders
now. Nothing anywhere asserted the badge's label text or the camera
readout's, and `valid_range.rs` asserted only phrases and the unit
symbol, never a digit.

**What the re-derivation moved: nothing in the population, one thing
outside it.** The four members held at their cited lines against the
merge base, each read rather than re-grepped. But the row's own sweep
pattern — a precision spec in a format string — has a fifth blind spot
its three stated ones do not cover: a widget that picks the precision
itself. `egui::DragValue` derives max decimals from its DRAG SPEED, and
a length field at `forms::FIELD_DRAG_SPEED` shown in millimetres falls
back to `{:.3}`, the exact spec the δ field was filed against. Filed as
`a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets`, with
the eleven-hit census and each non-member's own reason — one of which is
a length field at speed `0.5` that is excluded only because it lives in
a `#[cfg(test)]` harness.

Second residue, found by the unit's own new test rather than by reading:
`the-scientific-arm-rounds-out-of-the-type`. `{:.3e}` rounds, and within
half a unit in the fourth figure of `f64::MAX` it rounds out of the
type, so `number(f64::MAX)` reads back as infinity. Pre-existing (worse
in `render_mm`, where the millimetre multiply overflowed first), kept as
a stated exception rather than fixed, because the truthful spelling is
twenty-two characters and `readout::MAX_CHARS` is what `FIELD_WIDTH` is
measured against. A row pins the exception so it is met rather than
rediscovered.

**VIEW stands at 70 open / 85 closed, nothing waiting on Ev.**

## 2026-09-12 — #2444 merged; and the doc-gate commands in this register never existed

**#2444 merged** (`d8988ee461`). Job shape from the list: **42 jobs, 12
`test (…)`, 5 `k-lint (gate, …)`, `gate ok` success**, three
`render drift (…)` neutral (passing), `python suite` **green** where a
local `ci-filter.py` run said `RUN_PNCAD_PY=false` — the hosted filter
saw a different base. The total has moved 38 → 42 in a day, which is the
register's own point about carrying a number: **the tier evidence is the
12 and the 5 and `gate ok`, not the total.**

**One door, not four patches.** A new `crates/viewer/src/readout.rs`
with `number(value)` — the shortest decimal spelling that reads back as
the value within the render's own accuracy, scientific otherwise.
`DisplayTolerance::render_mm` survives as the δ-facing door onto it,
carrying only the millimetre conversion, so no existing caller moved.
The lane's three arguments: two of the four sites hold no
`DisplayTolerance`, so a rule on that type gets written twice — the
N-spellings-no-home shape arriving while a different defect is fixed;
it is **not** `render_mm` with δ removed, because δ is strictly positive
and a probed bound may be zero or negative, so the general rule is *a
text reads back as the value* and zero renders `0`; and `scene.rs` was
the cheap home and the wrong one, since `bounds` and `pane::view`
depending on the scene vocabulary for a text rule is a false edge.

`bounds.rs` overclaimed at **both** ends of one sentence —
`valid from 0.0000 mm` for a floor found above zero, and `1024.0000`
for a reach a doubling probe established to one figure. The camera
readout took the one decision the item asked for: three distances
through the door, two angles keep `{:.1}°`.

**`display_budget.rs` was holding a sentence to a FORMAT, not to a δ.**
`a_coarsened_picture_says_so_in_both_numbers` built its needles with
`format!("{:.3}", …)`, so it would have kept passing by accident on one
number (the budget δ's render is a superstring of its `{:.3}` rounding)
and failed on the other (`0.010` is not a substring of `0.01`). Needles
are the renders now. Nothing anywhere asserted the badge label or the
camera readout.

**A fifth blind spot the item's three did not cover**, found by the
lane and filed rather than swept: an `egui::DragValue` derives max
decimals from its **drag speed** and falls back to the range maximum
when nothing reads back, so a length field at `FIELD_DRAG_SPEED` in
millimetres lands on `{:.3}` — the exact spec the δ field was filed
against. Five production sites. Filed as
`a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets`,
with `the-scientific-arm-rounds-out-of-the-type` (the lane's own new
test found `{:.3e}` rounds past `f64::MAX`; pre-existing, kept as a
pinned exception with the width-vs-truth trade written down).

### The correction that matters: the doc-gate commands never existed

**This register and every dispatch built on it specified
`scripts/doc-gate.sh --pr` and
`scripts/doc-gate.sh --pr --scope '-p viewer' --skip-viewer-toolkit`.
Neither flag exists.** `doc-gate.sh` parses `--print-roots` and
`--skip-viewer-toolkit` itself and hands the rest to `gate_parse_args`
(`scripts/gates/lib.sh:55-64`), whose `*)` arm prints a usage line and
**exits 2**. I ran it: exit 2. The real commands are CI's
(`ci.yml:1804-1808`) — `--selftest`, then bare, then
`--skip-viewer-toolkit`.

**Several lanes reported exit 0 for that invocation.** A command that
cannot run cannot return 0, so those receipts were not measurements.
The rule's SUBSTANCE is untouched — both passes are owed, the skip pass
is the only rustdoc on a skip-mode run and the full pass the only one
that judges links, and that rests on `RUSTDOC_LINTS_INERT` in the
source rather than on any receipt. **What did not survive is the
verification chain**, and it was the chain this program built two rules
and a merged README section on.

So I re-established the fact instead of trusting it: on `main` at
`d8988ee461`, `scripts/doc-gate.sh` exits **0** and
`scripts/doc-gate.sh --skip-viewer-toolkit` exits **0**, both run here.
No bad doc state reached `main` — known now because it was measured.

The general rule is in `plan.md`: **before putting a command in a
dispatch, run it once.** A flag that does not exist fails loudly and
instantly; a flag that is never tested is believed for a week, and a
dispatch is where a wrong command propagates fastest because it is
copied verbatim into every lane.

**Two smaller corrections, both accepted.** `gate ok` is not a check run
until the end — it appeared as the 42nd row, after the last
`test (interval, …)` finished, so a poll that treats its absence as a
verdict reads a tier off an incomplete list. And this register's stated
test shape (`--test all` 524/0/1, `--lib` 37/1) was stale again at
533/0/1 and 41 lib rows; the SHAPE assertion that replaced the count
yesterday is what held.

**Three VIEW citations were already wrong about their subject** at the
merge base and were repointed by finding the subject, not by shifting:
`scene.rs:492` was `.iter()`, `scene.rs:917-919` was a `FittedDelta`
field rather than `fit_delta`, `scene.rs:410` was a comment rather than
`build_parts_focused`. One of them, in
`ui-thread-work-after-the-index-seam`, is a **split-span citation**
(`crates/viewer/src/\n   scene.rs:917-919`) caught only because the
census also matched the bare basename — the same split-span blind spot
I walked into myself today grepping `implementer-discipline.md` §6.

**Four out-of-fence citation shifts are recorded here and NOT applied**,
which is a deliberate call rather than an omission: `work/chrome/
mispaired-ids-exempts-the-empty-window.md:12` (`scene.rs:413`→`374`),
`work/chrome/probe-rows-assert-in-one-direction-only.md` lines 17 and 28
(`valid_range.rs:405`→`441`), plus `valid_range.rs:342`→`378` and
`bounds.rs:380`→`399` on that row, and `work/tint/
loud-skip-marker-is-a-hand-kept-idiom.md:39` (`lib.rs:92-100`→`93-101`).
The lane verified all of them as true shifts. I checked that the new
lines exist and stopped there: **repointing another program's row needs
that row's CLAIM checked against the new line, not merely that a line is
there** — this program's own rule is that a number can be wrong about
its subject rather than merely shifted, and discharging that standard
means reading three other programs' items in context. Getting it wrong
would plant the exact defect this program keeps tabulating into two
other slates. Handing them over is the honest move; silently shifting
them would not be.

**VIEW stands at 70 open / 85 closed, nothing waiting on Ev.**

## 2026-09-12 — §6's rewrite landed, and placing the handed-over citations found the class a third time

**#2443 merged** (`5eec65ef5e`): **22 jobs**, the whole code matrix
skipped, `gate ok` success — the docs tier by its shape, since the docs
total moved 21 → 22 the same day the code total moved 38 → 42.

**`implementer-discipline.md` §6's rewrite is now on `main`**, checked
with a multiline-safe read rather than a grep. A lane now files
out-of-fence findings directly on the owning program's slate, finding
the owner with `work.py territory --files -` and grepping that
directory for a duplicate first. Dispatches say so from here.

### Placing the four handed-over shifts, and why holding them was right

#2444's lane handed over four citation shifts in CHROME's and TINT's
rows, reporting that it had verified **all** of them as true shifts —
*"all three name their subjects at the old numbers"*. I had declined to
apply them unchecked, on the rule that repointing another program's row
needs that row's CLAIM checked against the new line. Now that §6
permits placing them, I checked each. **Two of four were wrong, and the
TINT one was not a shift at all.**

| row | proposed | verified | verdict |
|---|---|---|---|
| `chrome/probe-rows…` `valid_range.rs:405` | `:441` | `:441` | correct |
| `chrome/probe-rows…` `valid_range.rs:342` | `:378` | `:378` | correct |
| `chrome/probe-rows…` `bounds.rs:380` | `:399` | **`:402`** | wrong subject |
| `chrome/mispaired-ids…` `scene.rs:413` | `:374` | **`:445`** | wrong subject |
| `tint/loud-skip…` `lib.rs:92-100` | `:93-101` | — | **not a shift** |

`bounds.rs:399` is a doc-comment line; `MAX_REACHES` is at `:402`.
`scene.rs:374` is `Self::empty(nowhere, delta)`; the `MispairedIds`
guard the row quotes verbatim is at `:445`. **This is the out-of-fence
citation class measured a third time** — after 1-placeable-of-31 and
0-of-16 — and by the same mechanism each time: a shift map is
arithmetic on an integer and cannot be wrong in its own terms, so it
returns a number for every input and nothing in it asks whether that
number names the subject.

**The TINT row is the worse shape and the more useful find.** Its
citation does not need moving; the sentence it QUOTES no longer exists.
`grep -n "Nothing here goes red"` over `crates/viewer/src/lib.rs`
returns nothing, and the loud-skip paragraph now at `:96` says the
opposite of the quotation — *"the roster is the
`#[cfg(feature = "app")]` block above, which the compiler keeps, so
there is no hand-kept…"*. The hand-kept enumeration that row is about
was replaced by a compiler-kept one at this site. Shifting the numbers
would have preserved a quotation the source no longer contains: **a
citation that still resolves and now misdescribes**, which is worse
than one that dangles, because nothing about it looks wrong.

Applied: the two correct shifts plus the two corrected ones, and one
the lane did not list at all (`valid_range.rs:418` → `:455`, the
`1.0e-4` threshold in the same row). TINT got a written note rather
than an edit to its numbers, saying what VIEW checked and what it did
not — `crates/viewer` is the only site on VIEW's ground, so whether the
row closes depends on sites VIEW cannot speak for.

**The rule this earns, and it is about receipts rather than citations:**
a lane reporting *"I verified these as true shifts"* is reporting a
CONCLUSION, and the conclusion is produced by the same arithmetic that
cannot see its own failure mode. Handing four over with that sentence
attached is not evidence about four; it is one claim about a method.
**Check the subject at the new line, every time, no matter who did the
arithmetic** — including when the arithmetic was mine.

**VIEW stands at 70 open / 85 closed, nothing waiting on Ev.**

## 2026-09-12, later — `view/drag-field-precision`: every numeric field gets a text that reads back

`a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets` is
**closed**, and closed wider than it was filed. The item held itself
back on three reasons; the third is the interesting one and it turned
out to be an argument for the wide shape rather than against it.

**The fork the dispatch could not answer was answerable from egui's
source, and the answer is that it is not a fork.** *"What does a drag
mean when the text stops matching the tick?"* — it never does. A drag
commits `emath::round_to_decimals(value, auto_decimals)`
(`egui-0.36.1/src/widgets/drag_value.rs:654-659`), and `auto_decimals`
is the BOTTOM of the very range the formatter is handed, so every value
a drag can produce is spelled exactly by the range's shortest member and
both rules return it. `a_field_shows_what_the_widget_shows_wherever_that_reads_back`
asserts that as sameness over the drag's own landing set. This is the
register's *prove a claim by COMPILING, not by grepping* one step over:
the gesture question was a reachability question, and the widget's own
arithmetic answered it where weighing could not.

**The item understated the defect, and the understatement was its
reason 2.** It said a field reading `0.00` does not COMMIT `0.00`,
citing the `change != 0.0` guard. That guard covers the drag and the
arrow keys and nothing else: a `DragValue` seeds its keyboard edit with
the text it last showed and parses that text back on losing focus
(`drag_value.rs:540-552`, `:554-557`, `:577-591`), unconditionally on
whether the text changed. Driven headlessly through the real widget, a
field holding 4·10⁻⁵ mm came back holding **0.0**. The render is the
commit. `crate::pane::properties`'s `slot_value_ui` already knew the
path was there — its *"Text that says what the slot already says is not
an edit"* guard exists for exactly that click, and assumed the render
round-trips.

**So `crate::readout::number` is the rule and is NOT the whole answer
here**, which is what the dispatch asked. Its ten-character bound and
its relative tolerance are a READOUT's, and a field is auto-sizing and
is a commit path. `crate::widgets::number_text` keeps egui's own
spelling wherever `crate::readout::reads_back` accepts it and hands only
the rest to `number` — which is why the bound and the scientific arm are
never reached for a large value (they are reachable only where egui's
spelling already misreads, a band bounded above by one display unit;
`nothing_at_or_above_one_display_unit_renders_differently` pins it).
Applying `number` outright would have turned `12345678901.0` into
`1.235e10` and committed the difference.

**The five members were the wrong population and the door made the
classification unnecessary.** `crate::widgets::number_field` is one
constructor and all eleven `DragValue::new` sites go through it. The
count field is the one real non-member and is now provably so rather
than by judgement: `DragValue::new` gives an integral value
`max_decimals(0)` (`drag_value.rs:61-65`), so its range is `0..=0`. The
item's angle exclusion imported the READOUT class's rule (*is zero a
value this can have*) into a class about whether the text names the
number, and does not hold here.

**Two residues, both files**:
`nothing-holds-a-new-numeric-field-to-the-fields-door` (the twelfth site
is unguarded; three candidate guards costed, none free) and
`a-fields-text-commits-within-the-renders-own-tolerance` (the accepted
band is 5·10⁻⁴ relative, and committing a render inside it moves the
value by that much).

**One out-of-fence row filed, on CHROME's slate.**
`work/chrome/parameter-row-field-cites-a-pre-split-app-rs`:
`parameter-row-field-has-no-text-door` cites four `app.rs` bands
(`:2927-2975`, `:4549-4626`, `:4583-4596`, `:4611-4626`) against a
2,018-line file — past-end-of-file, the class the register measured at
fifteen of thirty-one. Its two other citations were read and are
correct, and are recorded as correct. The new homes are named by
SUBJECT, because the lane was editing `pane/properties.rs` in the same
PR and any number it wrote would have been wrong before the file
reached `main`.

**The #2278 obligation, discharged mechanically.** This diff moves
`widgets.rs` by +71/+67 and `pane/properties.rs`/`pane/create.rs` by
+2/+1, so it owed the census of every open row citing into those bands.
Built by mapping base→head with a line-level diff and then **checking
that the text at the new number is byte-identical to the text at the
old** — the subject check, done by the instrument rather than by
arithmetic. **22 numbers across 11 in-fence rows** repointed, zero
`TEXTDIFF`, `plan.md` included (its `widgets.rs:300` → `:367` is the
`ArcMode` loop the bare-vocabularies ruling rests on). One open row
outside the fence shifts — `work/census/the-prose-word-for-a-kind-has-four-spellings-...`
at `properties.rs:158-161` → `160-163` — and is **not** filed: §7 says a
number may ride along beside a name and is allowed to go stale, and a
two-line shift whose file and subject are unchanged is that, where
CHROME's four past-end-of-file citations are a defect. The distinction
is the one this register already draws between a citation wrong about
its SUBJECT and one merely shifted. Two closed `work/door/` rows also
shift and are left: a closed row is a record.

**Receipts.** `doc-gate.sh --selftest` 0, `doc-gate.sh` 0,
`doc-gate.sh --skip-viewer-toolkit` 0 — all three run locally, and the
third is the one this PR's own CI structurally cannot reach.
`cargo test -p viewer --features app --no-fail-fast`: every `--test all`
row passing, `--lib` one row red (`gpu::tests::every_pass_builds_on_a_real_device`,
no Vulkan here) and no other. `cargo test -p test-utils` green, clippy 0,
`cargo fmt --check` 0, `cargo check -p viewer --target wasm32-unknown-unknown` 0.
The commit row was mutation-checked: deleting `.custom_formatter` from
the door reds `clicking_into_a_field_and_away_again_leaves_the_value_alone`
with *"clicking into a field holding 0.00004 and away again committed 0"*.

**VIEW stands at 71 open / 86 closed, nothing waiting on Ev.**

## 2026-09-12 — #2453 merged; its lane was lost to a container restart, and the PR body survived it

**#2453 merged** (`8d653a7ce2`). The lane died in a container restart
**after** pushing three commits, opening the PR and writing a full body,
but before reporting. The orchestrator did the report's job from the
diff and the body; the work needed nothing re-run. **Worth recording as
a harness fact: a lane's PR body is durable and its chat report is not,
so a lane that writes its argument into the body loses nothing to a
restart.** Dispatches should keep saying the logical documentation lives
in the PR description.

Job shape verified from the list at the second look: **39 jobs, 12
`test (…)`, 5 `k-lint (gate, …)`, `gate ok` success**, nothing
non-(success/skipped). At the FIRST look it was **38 with four jobs
still running and no `gate ok` row at all** — the rule about `gate ok`
posting last earning itself within a day of being written. The total
also moved 38 → 39 purely because `gate ok` posted, which is the same
rule from the other side.

**The merge was refused once with `405 Base branch was modified`** and
succeeded on a retry with no change to the head. A 405 there is a race
with `main` moving, not a mergeability verdict; `mergeable_state` read
`unknown` at the same moment because GitHub had not recomputed it.

### What the unit landed

`widgets::number_text` — **a field's text reads back as the value the
field holds** — keeping egui's own spelling wherever
`readout::reads_back` accepts it and handing the rest to
`readout::number`. `widgets::number_field` is the constructor that
attaches it, and **all eleven `DragValue::new` sites in the crate go
through it**.

**The item understated its own defect and the lane proved the stronger
form.** The item held back on the ground that *"a field reading `0.00`
does not commit `0.00`"*, citing the `change != 0.0` guard. That guard
covers the drag and the arrow keys only: a `DragValue` **seeds its
keyboard edit with the text it last showed and parses that text back on
focus loss, unconditionally**. So clicking into a field and away again
commits what the field said — measured through the real widget, a field
holding 4·10⁻⁵ mm shows `0.000` and comes back holding **0.0**. The
render is a commit path, which is the δ field's defect on every numeric
field in the chrome.

**The dispatch's open question was answered from egui's source rather
than decided.** I asked what a drag means when the text stops matching
the tick. It never does: a drag commits
`round_to_decimals(value, auto_decimals)` and `auto_decimals` is the
bottom of the very range the formatter is handed, so every value a drag
can produce is spelled exactly by the range's shortest member. The row
asserts that as sameness over the drag's own landing set rather than
stating it.

**`readout::number` is the rule and deliberately not the whole answer
here** — its `MAX_CHARS` is sized against `pane::view`'s fixed field
width while a `DragValue` auto-sizes, so applying it outright would
render `12345678901.0` as `1.235e10` and then commit the difference.
Deferring to egui in the accepting band keeps `number` reached only
where egui's spelling already misreads the value.

**The five-member population was the wrong one**, and the door made the
classification unnecessary: the property *a field whose text is not the
value it holds* has no dimension in it, so the item's angle exclusion
had imported the READOUT class's rule (*is zero a value this can
have*) into a class about whether the text names the number. The count
field is the one real non-member and is now provably so — an integral
`DragValue` gets `max_decimals(0)`, so its range is `0..=0` and a whole
number reads back as itself.

**Two residues filed on VIEW's slate** (`nothing-holds-a-new-numeric-
field-to-the-fields-door`, with three candidate guards costed; and
`a-fields-text-commits-within-the-renders-own-tolerance`), and one
out-of-fence row filed on CHROME's under the rewritten §6:
`parameter-row-field-cites-a-pre-split-app-rs` — four `app.rs` bands
cited against a 2,018-line file, the past-end-of-file class `wc -l`
catches. Its two correct citations were read and **recorded as
correct**, which is the half these censuses usually omit.

**The census discipline was done by instrument rather than by
arithmetic**: base→head mapped with a line-level diff and then the text
at the new number checked byte-identical to the text at the old — 22
numbers across 11 rows repointed, zero mismatches. One out-of-fence
shift was deliberately **not** filed, on §7's rule that a number riding
beside a name may go stale, and two closed `work/door/` rows left alone
because a closed row is a record.

**VIEW stands at 71 open / 86 closed, nothing waiting on Ev.**

## 2026-09-12 — `probe-identity-stops-at-the-instance`: answered

`view/probe-identity`. The fork was *should the door refuse a second
driver on one instance, or is "one probe per instance, driven by
whoever names it" the rule?* It is the rule, and it was already the
value drag's rule — stated for one half of the pair and not the other.

**The item compared a second DRIVER against a second SUBJECT.**
`a_drag_on_another_field_cannot_steer_the_open_one` is the value drag's
row for another subject, and the probe's counterpart for THAT exists
(`a_probe_on_another_instance_cannot_steer_the_open_one`, #2361). The
like-for-like row is
`the_open_drags_own_field_dragged_again_lands_its_number`, and it
asserts, for the value drag, the exact three behaviours the item calls
a hole: the second batch's begin refused `GestureInFlight`, its preview
steering the open gesture, its commit landing it and ending the drag
the first was holding. The README argues that as the whole difference
between a target and a token. So the item's *"There is no free-move
counterpart, and the reason is that there is nothing for one to
assert"* is false at both readings, and what was actually missing was a
test row, not a door.

**The second driver is a stranded probe, and the strand was already
traced and left.** `free-move-in-flight-refusal-has-no-reachable-
producer` (closed) wrote: *"The hole left is the SELECTION, which no
prune covers: `instance_ui` is drawn only for `selection().node()`, so
a `Select` performed under an open probe would take the field away with
the drag still live."* `Select` is permitted by
`permitted_during_free_move`, and the hand that reaches it under a held
drag is `a-keyboard-bump-…`'s: the feature tree's row is a
`selectable_label(…).clicked()` (`pane/features.rs:59-60`) and egui
answers `clicked()` for Space/Enter on a focused widget and for an
AccessKit `Action::Click`. Re-select and drag: begin refused, preview
and commit land. A per-begin token would refuse the recovery and strand
the reader twice. **So the guard the item asked for would fire on the
one route that exists, and refuse it wrongly** — which answers the
*dead-code-or-keeping-a-later-caller-honest* question in the third
direction neither arm named.

`gizmo` occurs in this tree exactly once, inside the item. No second
panel and no scripted chrome is on any roadmap. The hypothetical driver
was hypothetical; the real one is one reader recovering one drag.

**Provenance, per `CLAUDE.md`'s check-that-Ev-ever-agreed rule.** The
prose changed is `crates/viewer/README.md`'s *A driving operation names
its own gesture*, which the #2462 split left in the README's RECORD
half — `GUI-DESIGN.md` holds `G1`–`G5`/`GQ1`–`GQ7` and says
nothing about driving-op identity. `git log -S'A driving operation
names its own gesture' -- crates/viewer/README.md` → `26c62d3a0e`
(#2361, `view/gesture-identity`); `git log -S"The probe's target is
coarser than a field"` → `c69eafc4d0` (#2392, `view/keyboard-bump`).
Both are
VIEW lanes, neither PR is `[ev]`. No ratification exists, so this
lands with the change rather than waiting.

**What landed.** The rule stated in the README and on
`SessionOp::PreviewFreeMove`, and
`the_open_probes_own_instance_driven_again_lands_its_frame` holding it
— the probe's counterpart to the value drag's row, carrying the strand
and the recovery. Three mutations red it, each reverted on a committed
tree: permitting a second `begin_free_move` (3 rows red), refusing
`Select` mid-probe (3 rows, and this row is the only one about the
recovery), inverting `commit_free_move`'s name check (13 rows).

**Filed outside the fence** (§6): `work/door/gq5-recap-citation-
points-at-the-readme-the-split-emptied`. DOOR's
`dimension-all-has-readers-outside-the-viewer` cites
`crates/viewer/README.md:1500` for the GQ5 recap; the README is 1444
lines, the citation was correct at `625722e79e` against an 1849-line
file, and **#2462 moved the subject to another file** —
`crates/viewer/GUI-DESIGN.md:155`, located by its own words. Not a
line shift, so no repoint inside the README could find it. The row
says what it does not claim: the rest of #2462's casualty population,
which a `wc -l` filter cannot see.

**VIEW stands at 72 open / 87 closed** — re-derived from
`work.py status --program view` on the MERGED tree, not carried
forward. It moved twice while this lane ran: the previous entry's
*71 open / 86 closed* was true of its own merge base, and main gained
two more open rows (`two-partial-mirrors-in-the-viewer-have-no-growth-
alarm` among them) between this branch's base and its merge-forward. A
count taken before the merge would have been stale on landing.

## 2026-09-13 — #2479 merged; the row closes as RATIFIED, and the guard the item asked for would have been wrong

**#2479 merged** (`14d084e10b`), verified from the job list: **39 jobs,
12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok` success**, 33 success /
6 skipped, nothing non-passing.

**`probe-identity-stops-at-the-instance` closes as RATIFIED, not
fixed.** The item's premise was an asymmetry with the value drag, and
the asymmetry is not there:
`the_open_drags_own_field_dragged_again_lands_its_number` already
asserts, **for the value drag**, the exact three behaviours the item
calls a hole — second begin refused `GestureInFlight`, its preview
steers the open gesture, its commit lands it and closes the drag the
first was holding. So *one gesture per subject, driven by whoever names
it* was the rule already; the probe obeys it at its own subject. What
was missing was a **row**, not a door: the second-DRIVER row now exists
as `the_open_probes_own_instance_driven_again_lands_its_frame`.

**The item compared the wrong pair, and so did my dispatch.** I wrote
that *"the probe has no counterpart"* to
`a_drag_on_another_field_cannot_steer_the_open_one`. It does —
`a_probe_on_another_instance_cannot_steer_the_open_one`
(`crates/viewer/tests/gesture_table.rs:1496`), added by #2361. **I
verified it is on `main` before merging**, so the correction is to the
item's sentence *"There is no free-move counterpart, and the reason is
that there is nothing for one to assert"* and to my repetition of it.
That row is the second-**subject** row; the property at issue is the
second-**driver**. **A proxy standing in for the property, in the
sentence that made the row look like a door defect** — the class this
register tabulates, arriving inside an item about a different thing.

**And the guard the item asked for would have fired on the one route
that exists, and refused it wrongly.** A second driver is not a gizmo —
`gizmo` occurs in this tree exactly once, inside the item, and nothing
is roadmapped. It is a reader recovering a **stranded probe**, and
`free-move-in-flight-refusal-has-no-reachable-producer` — this row's
own sibling, closed — had already traced it: *"`instance_ui` is drawn
only for `selection().node()`, so a `Select` performed under an open
probe would take the field away with the drag still live."* `Select` is
permitted mid-probe and the feature-tree row is
`selectable_label(…).clicked()` (`crates/viewer/src/pane/features.rs:59-60`),
which egui answers for keyboard Space/Enter and AccessKit with no
pointer — the same hand #2392 was about.

**The correction that stings: the evidence was one closed row away.** I
flagged *unreachable from today's chrome* as this program's
most-repeated defect and was right to, but the row that decides the
fork is named in the item's own `refs` and neither the item nor my
dispatch read it. **A reachability fork owes a read of the closed rows
the item references**, not only of the tree. That is now the rule.

**Provenance, run rather than assumed.** `git log -S` on both prose
sites: *"A driving operation names its own gesture"* → `26c62d3a0e`,
carried by **#2361**, not `[ev]`; *"The probe's target is coarser than a
field"* → `c69eafc4d0`, **#2392**, not `[ev]`. `GUI-DESIGN.md` carries
no driving-op-identity clause. Record text VIEW wrote itself, changed
with the change and said so in the body — `CLAUDE.md`'s new rule used
on its first live fork rather than quoted.

### #2462's casualty population, censused — it is one row

The lane filed `work/door/gq5-recap-citation-points-at-the-readme-the-
split-emptied` and said the rest of the population *"nobody has
censused"*. Censused here, since #2462 was mine: every citation of the
form `viewer/README.md:<line>` at or past the cut (old `:1401`) across
`*.md`, `*.rs`, `*.py`, `*.yml`. **Four hits, one live row.**

- `work/door/dimension-all-has-readers-outside-the-viewer.md:60` —
  `:1500`, the GQ5 recap, correct before #2462 and now in
  `crates/viewer/GUI-DESIGN.md`. The lane's filed row covers it.
- Two hits are that filed row itself, and one is `work/view/log.md` —
  append-only history, which records what was true when written and is
  not repointed.

So the split cost exactly **one** live citation, already filed on the
owner's slate. Recording the number because *"nobody has censused it"*
is the state a population stays in until someone does, and the
instrument is a one-line grep with a line-number threshold.

**VIEW stands at 74 open / 87 closed, nothing waiting on Ev.** (The lane reported 72/87 after re-deriving at its own merge-forward; `work.py status` here reads 74 — two more open rows arrived from other programs in between. The tool at merge time is the count, not the tool at any earlier point.)

## 2026-09-13 — `view/numeric-field-door`: the twelfth field is held by the context

`nothing-holds-a-new-numeric-field-to-the-fields-door` — **closed.**
The row costed three shapes and asked for a choice; the choice is the
third (`egui::Style::number_formatter`), and two of the row's own costs
did not survive reading the tree.

`crate::widgets::install_number_formatter` writes `number_text` onto
both of the context's styles through `all_styles_mut`;
`ViewerApp::new` calls it beside `apply_polarity`. `number_field` is
untouched — it keeps `.custom_formatter(number_text)` and every test
over it — because the two are not alternatives. The door is the visible
statement at the call site; the context default is the floor under a
site that misses it.

**What settled it was the helper case**, which the row raised and left
open. The other two shapes detect a token, and the token is
`DragValue::new`: they catch a new helper only because the helper also
writes that token one level down, and they catch nothing that reaches a
numeric field another way — an aliased import (`use egui::DragValue as
DV`), an `egui::Slider`, a wrapper that overrides the formatter.
Setting the default is not a detection at all; a site that does not
deliberately spell its own `custom_formatter` is already right.
`crates/viewer/src/pane/properties.rs:568` is the live instance of a
site that DOES spell one — a fixed expression-sourced field showing its
source — and it is exactly the case that should stay writable, which is
also the argument against banning the constructor.

**The row's test-invisibility cost is a siting cost, not the shape's.**
It follows only from installing the formatter inside app startup. A
`pub(crate)` function is callable from a bare `egui::Context`, so
`widgets::field_tests` now drives an `egui::DragValue::new` that has
never seen the door — a wider test than any the crate had, since every
existing row goes through `number_field`. Three rows: the round trip on
a bare field with the rule installed, the control on a bare field
without it (which still commits 40 nm as zero), and a theme switch in
both directions. Perturbation receipt for the third: replacing
`all_styles_mut` with `style_mut_of(Theme::Dark, …)` reds
`a_bare_field_survives_a_theme_switch` at `widgets.rs:1427` and nothing
else (8 passed, 1 failed).

**Two corrections to the row, both checked rather than inherited.**
Its second bullet says a source-text guard *"owes a line in
`crates/test-utils/tests/reader_census.rs`"*. That is true of a Rust
one only: the census's own header cedes `scripts/`, whose gates read
Rust through `scripts/gates/lib.sh`'s `gate_rust_code` — *"a second
home, in a second language, which this row cannot see and does not
claim to."* VIEW already owns two members of that family
(`viewer-module-kinds.sh`, `viewer-vocab-declared-once.sh`), so the
shape had a cheaper siting than the one it was costed at. Its third
bullet says the formatter is the only shape that also catches an
`egui::Slider`; true in mechanism (`slider.rs:925` builds a
`DragValue`, which falls through to `drag_value.rs:534`), but **there
is no `Slider` anywhere in this repository**, so that is a claim about
the next one rather than about a site the other shapes miss today.

**What is not held, said rather than papered over**: one line in
`ViewerApp::new`. It takes an `eframe::CreationContext` and no test in
this crate builds one, which is why `apply_polarity` — two lines above,
same exposure — has no test either. The rule is covered; the wiring is
not, and a `scripts/gates/` member with its `ci.yml` pair and planted
fixture is not proportionate to one call line.

**Sweep and its blind spots.** `grep -rn 'DragValue' crates/ demos/
tools/ benches/ --include=*.rs` — one production construction
(`widgets.rs:96`, the door), one in the new test harness (deliberate,
`widgets.rs:1308`), the rest prose. `grep -rn 'Slider'` over the same
trees — zero. `grep -rn 'custom_formatter\|custom_parser'` — the door,
`properties.rs:554`'s parser and `:568`'s formatter override, the rest
prose. `grep -rn 'set_global_style\|set_style_of\|style_mut_of\|
set_visuals\|set_style'` — nothing but `apply_polarity`'s own doc
comment, so nothing in the crate clobbers a style after install. What
none of these can match: a numeric field reached through a toolkit type
this crate does not name today. That is the residue the formatter shape
answers and a grep cannot.

**Tracker pass.** `work/chrome/parameter-row-field-has-no-text-door`
is the nearest neighbour and is NOT a duplicate: its subject is the
parameter row's missing `custom_parser` and no-op guard, it is parked
on a DOCM door, and `props`' module docs already cite it accurately.
Its body's `app.rs:4549-4626` citations are pre-split and stale
(`app.rs` is 2,028 lines), which is `stale-file-citations-after-the-
split`'s general case, already open on this slate — no second row
filed.

**VIEW stands at 73 open / 88 closed, nothing waiting on Ev**
(`python3 scripts/work.py status`, re-run at merge time).

**Filed out of fence, from this lane's own CI run.** PR 2519's run
(34777661121) is a closure seeded only in `viewer` —
`SEEDS=viewer`, `RUN_PNCAD_PY=false` in the `change filter` log — and
`python suite (wheel + guide + north-star)` **ran and passed anyway**,
which `docs/prompts/implementer-discipline.md` §2 says in bold it
should not. `ci.yml:397-401` deleted that gate on 2026-09-12
(`b6cc8d4d2e`) and says so at the site; the doc paragraph dates from
2026-09-06 (`370a7dfdb9`), when the axis really did gate the job.
Filed as `work/meta/implementer-discipline-python-suite-paragraph-
describes-a-deleted-gate` — META's, because `docs/prompts/` is META's
by `work.py territory` and the text binds every lane by path. Not a
duplicate of `tcost/run-pncad-py-is-computed-and-gates-nothing`,
`ciw/python-suite-axis-skips-only-two-members` or
`ciw/ciw-rows-and-ci-local-prose-rotted-by-the-c1-c3-restore`; the row
says why against each.

**Job shape on that run**, checked rather than counted: twelve
`test (…)` rows green (both lanes x three eps x two shards), five
`k-lint (gate, …)` rows green, `gate ok` green, run conclusion
`success`. 39 jobs total, which is not the instrument.

## 2026-09-13 — #2519 merged; the item's fork was false, and a rate limit cost a day

**#2519 merged** (`722d39fd32`), verified from the job list: **39 jobs,
12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok` success**.

**An 18-hour outage first.** The lane dispatched against this row died
to a **weekly rate limit** while still reading `plan.md`, having done no
work — its branch never left `main`, verified rather than assumed. Four
check-ins queued into a session that could not act. The re-dispatch
reused the clean worktree, re-synced it (`main` had moved 7,717 commits,
almost all of it one long-lived branch's August history landing late),
and told the lane to **skim the register for the rules that bear on its
unit rather than chunk all ~1000 lines** — reading the register to death
before touching the tree is what spent the budget.

**The shape taken is the third: `egui::Style::number_formatter`**,
installed on both styles through `Context::all_styles_mut` beside
`apply_polarity`. `number_field` is untouched.

**The item's fork was false, and that is the finding.** It framed the
choice as *visibility OR guarantee*. Taking the formatter costs the door
nothing — the door keeps `.custom_formatter(number_text)`, its doc and
every row over it. The context default is a **floor under** a site that
misses the door, not a replacement for it.

**The helper case decided it.** Shapes 1 and 2 detect a TOKEN, and the
token is `DragValue::new`; they catch a new helper only because the
helper writes that token one level down, and catch nothing that reaches
a numeric field another way. Setting the default is not a detection at
all: a site that does not deliberately spell its own formatter is
already right, so there is no arrival to notice. The escape table in the
PR body has `use egui::DragValue as DV` defeating the text guard and an
`egui::Slider` needing a second pattern in both detector shapes.

**The test-invisibility cost was a SITING cost, not the shape's** — it
follows only from installing inside app startup. A `pub(crate)` fn is
callable from a bare `egui::Context`, so the new rows drive a
`DragValue::new` that has never seen the door, which is wider than any
existing row. Perturbation receipt: `all_styles_mut` →
`style_mut_of(Theme::Dark, …)` reds `a_bare_field_survives_a_theme_switch`
and nothing else. One residual stated rather than papered: the line in
`ViewerApp::new` is untested, because nothing constructs an
`eframe::CreationContext` — which is why `apply_polarity` two lines
above is untested too.

### Three corrections, two of which I had repeated

1. **The item's second cost is overstated and I passed it on.**
   `reader_census.rs`'s own header cedes `scripts/`: its gates read Rust
   through `scripts/gates/lib.sh`'s `gate_rust_code`, *"a second home,
   in a second language, which this row cannot see and does not claim
   to."* So a `scripts/gates/*.sh` guard — the family VIEW already owns
   two of — owes **no** line in test-utils. A cheaper shape than either
   the item or I costed; still not taken, on the escape table.
2. **There is no `egui::Slider` anywhere in this repo.** Verified here:
   zero hits across `crates/`, `demos/`, `tools/`. The item's *"the only
   shape that also catches an `egui::Slider`"* is true in mechanism and
   catches nothing today — a claim about the next one, not this one.
3. **`docs/prompts/implementer-discipline.md` §2 is false about the
   python suite, and this PR's own run is the counterexample.** The doc
   says *"A closure seeded only in one of those two skips it"* —
   `viewer` and `test-utils`. This PR is seeded on `viewer` alone and
   `python suite (wheel + guide + north-star)` **ran and passed**,
   visible in the job list. The gate was deleted on 2026-09-12
   (`b6cc8d4d2e`); the paragraph dates from 2026-09-06, when it was
   true. Filed on META's slate (`docs/prompts/` by `work.py territory`)
   as `implementer-discipline-python-suite-paragraph-describes-a-
   deleted-gate`, with a why-not-a-duplicate against three neighbours.
   **It bears on every dispatch this program writes**, since that
   paragraph is what tells a lane what a green run covers.

**And the split-span trap caught me a third time in one day.** Checking
correction 3, `grep -n "closure seeded only in one of those two"`
returned nothing and I nearly recorded the lane's quote as
unverifiable — the sentence spans a newline (*"seeded only in\none of
those two"*). A multiline-safe read found it immediately. That is the
same blind spot as `implementer-discipline.md` §6 yesterday and the
TINT citation the day before. **A line-based grep over prose is a proxy
for the prose**; when one returns nothing, re-run it joined before
believing the absence.

**VIEW stands at 73 open / 88 closed, nothing waiting on Ev.**

## 2026-09-14 — `view/toolbar-wrap`: the row was measured, and it misses by 580 points

`the-toolbar-row-does-not-wrap` — **closed.** The row's own framing
was that a repair chosen before the measurement is a guess, and its
stated
blocker ("this crate has no headless egui harness") had already stopped
being true when it was written — `widgets.rs`, `pane/view.rs` and
`pane/viewport.rs` each drive a headless `egui::Context` with
`RawInput`. So the measurement came first.

**What it took to measure the REAL toolbar rather than a replica of
it.** Two extractions, neither of which changes what any frame draws.
`ViewerApp::new` is split into `assemble` — everything startup does
that needs no graphics device: document, evaluation, tessellation,
camera, preferences and the two context-wide styles — and the device
half that installs the viewport pipeline, which is the only part a
headless context cannot run (`StartupError::NoWgpuRenderState` was the
whole blocker). The toolbar's 280 inline lines come out of
`ViewerApp::ui` as `ViewerApp::toolbar_ui`. A measurement of a
hand-built row with the same twelve labels would have been evidence
about the replica.

**The numbers.** The row's natural width is **964 points** at the
default style, on the startup document, with no gesture in flight and
no status line — every one of those a lower bound. A 400-point window
(an upright phone browser, which `run_web` ships this same toolbar
into) offers the panel 384: **580 points, 60% of the row, laid out past
the right edge.** Not a phone-only case either — 964 does not fit a
desktop window tiled to half of a 1920-point screen (960).

**The item's control list was short**, which makes the doors worse off
than it says. Beyond the twelve it names the row also holds the theme
`ComboBox` (`viewer_theme`, landed `cf2164600f` on 2026-09-03, before
the item was filed), up to three badges and the status label — all of
them to the RIGHT of the two cancel doors. The doors are not at the
end of the row; they are near the middle of it, and still off-screen.

**`ui.horizontal_wrapped`, and the cost the item feared is not real.**
egui's wrapped horizontal layout wraps only when the content does not
fit, so at every width where the old row fitted the new one is
identical — "it changes the toolbar's look at every width" is not what
the layout does. `ScrollArea::horizontal` was refused on the doors'
own siting argument: a scrolled-off control is still not visible, and a
cancel door reachable only after a user notices a scrollbar is the same
defect with an extra step.

**A row, not prose, and no pixel of the toolbar is pinned.**
`the_toolbar_asks_for_more_width_than_a_narrow_window_gives` holds that
the wrapping is answering something; `the_toolbar_wraps_rather_than_
running_past_a_narrow_window` holds that the row stays inside the
window it is given. The only number either fixes is the WINDOW's (400
points, stated and argued at `NARROW`) — the row's own width is read,
never asserted, so a relabelled control re-baselines nothing. If the
toolbar ever shrinks enough to fit 400, the first row reads red and
says in its message that both should be retired.

`crates/viewer/README.md`'s cancel-door siting paragraph carried the
parenthetical *"Drawn, not reachable at every window width"* and cited
this item; it now claims both and names the two rows. Record half,
VIEW's own, and a re-wording forced by the code the clause describes.

**Filed, in fence.** `nothing-holds-startups-two-context-wide-style-
installs` — the numeric-field-door unit disclosed on 2026-09-13 that
nothing holds `apply_polarity` or `install_number_formatter` being
CALLED at startup, and argued the gap from a blocker: `ViewerApp::new`
takes an `eframe::CreationContext` no test can build, so a guard meant
a `scripts/gates/` member. That row closed without giving the residue a
file. The split above removes the blocker — `assemble` takes an
`&egui::Context` and `app.rs`'s test module builds one — so the row
exists now and says what is still open about it (what the assertion
should read). VIEW's own ground; not a duplicate of the closed
`nothing-holds-a-new-numeric-field-to-the-fields-door`, whose subject
is the rule rather than the call.

**Sweep.** `grep -rn 'ui\.horizontal(' crates/viewer/src/` — 48 hits,
47 of them rows inside a pane or a form. `grep -rn 'Panel::top|
Panel::bottom|Panel::left|Panel::right|TopBottomPanel|SidePanel'` over
the same tree returns the toolbar and nothing else, so the toolbar is
the crate's ONLY panel: every other non-wrapping row lives in a tile a
user can resize or re-split, and none of them is anybody's only exit
from a modal state. Not swept, and not this unit: whether a pane's own
rows clip at a narrow tile. What the greps cannot match: a row laid
out through `Layout::left_to_right` or `ui.columns` directly —
`grep -rn 'left_to_right|ui.columns('` returns nothing in the crate
today.

## 2026-09-14 — #2541 merged; the lane built the instrument instead of guessing

**#2541 merged** (`c759a216fa`), verified from the job list: **40 jobs,
12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok` success**; six skipped;
`render drift (gui)` **neutral**, which passes.

**The item said "measure first" and named the blocker; the lane removed
the blocker.** `ViewerApp::new` needed a wgpu render state, which is why
nothing could build this app in a test. It split that into
`ViewerApp::assemble` (everything startup does that needs no device)
plus the device half, pulled the toolbar's 280 inline lines out of
`ViewerApp::ui` as `ViewerApp::toolbar_ui`, and laid **the real
toolbar** out in a headless `egui::Context` — a hand-built row with the
same labels would have been evidence about the replica, not the row.

| | |
|---|---|
| the row's natural width | **964.09 points** |
| a 400-point window gives the panel | 384 |
| laid out past the right edge | **580.09 — 60% of the row** |

All lower bounds: default style, startup document, no gesture in
flight, no status line. **Not only a phone** — 964 does not fit a
desktop window tiled to half of a 1920-point screen.

**And the item's stated cost for the repair is not real, measured the
same way.** `ui.horizontal_wrapped` was held back by *"it changes the
toolbar's look at every width"*; at 1280 the two layouts produce the
**identical rect** `[[8.0 2.0] - [972.1 20.0]]`, height 18. So the
objection that kept the one-word fix off the table for three days
evaporated the moment anyone could measure it. `ScrollArea::horizontal`
was refused on the doors' own siting argument: a scrolled-off door is
still not visible.

**A row, not prose, and the open question answered.** I had asked
whether the measurement should become a hold pinned to a pixel width,
with its own maintenance cost. The lane's answer is that the two rows
pin the **window** (400, argued at `NARROW`) and never the row:
`..._asks_for_more_width_than_a_narrow_window_gives` keeps the second
from being a tautology, `..._wraps_rather_than_running_past_a_narrow_window`
holds the property, and a relabelled control re-baselines nothing. Both
verified failing with `ui.horizontal` restored.

**Where my dispatch was incomplete.** I repeated the item's list of
twelve controls. Beyond those the row holds the theme `ComboBox`
(landed 2026-09-03, **before the item was filed**), up to three badges
and the status label — **all to the right of the two cancel doors**. So
the doors are not at the end of the row; they are near the middle of a
964-point one, and the item understated its own case.

**A harness fact worth keeping**: driving the app headlessly requires
`output.textures_delta.clear()` or epaint panics in `Drop`.
`widgets.rs:1311` already does it; the lane's first run died exactly
there. Anyone driving a `Context` in this crate needs that line.

**Filed**: `nothing-holds-startups-two-context-wide-style-installs`
(VIEW's own). #2519 disclosed that nothing holds `apply_polarity` and
`install_number_formatter` being *called*, argued it from the
`CreationContext` blocker, and closed without giving the residue a
file. **This split removes that blocker**, so the row now exists and
says what is still open.

**VIEW stands at 73 open / 89 closed, nothing waiting on Ev.**

## `two-datumkind-enums-name-the-same-four-datum-kinds` — closed, two types kept (`view/two-datumkinds`)

Settled **two types, not one**, and removed the name collision that was
the whole of the defect. `forms::DatumKind` is now
`forms::DatumKindChoice` (`crates/viewer/src/forms.rs:124`),
`pub(crate)` behind `app` as before, so no public surface moved;
`viewer::DatumKind` (`crates/viewer/src/datums.rs:344`, re-exported at
`lib.rs:129`) is untouched.

**The argument.** The two are different functions of the same domain,
not two spellings of one concept. The draw tag partitions
`DatumValue`'s five arms onto four DRAWINGS — `AxisInPlane` is a line
in space, drawn as the axis it is — while the form choice selects four
of `DatumSpec`'s five arms for what a plain-numbers form can AUTHOR,
`AxisInPlane` being the one that needs a frame pick first. The same arm
is both the value that collapses and the spec that is not offered, for
unrelated reasons, and that coincidence is the entire reason the
memberships matched. Merging would make `ALL` — which the radio row
walks — claim that everything drawable is offered by the add-datum
form, and `revolve-tool-unreachable-no-axisinplane-form` is already on
this board asking for the counterexample: its fix grows the form choice
to five while the draw tag stays at four.

`Choice` is not a coinage. It is what this crate already spells a form
choice with where there is a thing chosen among — `PatternKindChoice`
beside the kernel's `PatternKind`, `blend::BlendKindChoice` — and the
add-datum form was the one that took the bare name.

**No mechanical hold, deliberately.** There is no invariant between the
two to assert; a test pinning them identical would hold the coincidence
and would have to be deleted the day the revolve row lands. What holds
them honest is the name plus a stated relationship at both
declarations.

String tags and `ALL` did not move: the words stay table data on the
form side (`add_datum_ui` walks `DatumKindChoice::ALL` for them), and
the draw tag keeps its hand-written `label` match and gains no `ALL`,
since nothing iterates it for words and it is not a vocabulary the
chrome offers. It therefore does not become a `vocabulary!`
declaration; the item's note that "the survivor should be declared
through `vocab.rs`" applied to the merge outcome, which was not taken.

**Neighbour rows.** `two-partial-mirrors-in-the-viewer-have-no-growth-alarm`
is **not closed and not subsumed** — the spec-to-kind growth alarm it
asks for is still unwritten. Its title and citations are respelled for
the rename, with a note that the public draw tag is not a third site
for that instrument (no `ALL`, not a partial mirror, growth already
forced by `draw_one`'s exhaustive match).
`revolve-tool-unreachable-no-axisinplane-form` had two stale claims
refreshed: the type name, and the member order it quoted as
`Plane, Axis, Point, Frame`, which PR 2046 reordered to form order.

## 2026-09-14 — #2561 merged; two types stay two, and a justification I had mis-stated

**#2561 merged** (`c77690782a`), verified from the job list: **39 jobs,
12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok` success**, four
render-lane rows green.

**The two `DatumKind`s stay two types, and the name collision goes.**
The lane's argument is the one the item did not make: they are two
different functions of the same domain.
`datums::DatumKind` **partitions the datum VALUES by how they are
drawn** — four members over `DatumValue`'s five arms because
`draw_one`'s `AxisInPlane` arm hands it `Axis`. `forms::DatumKind`
**selects what a plain-numbers form can author** — four of
`DatumSpec`'s five arms, `AxisInPlane` absent because it needs a frame
pick. **The memberships match because the same arm is both the value
that collapses and the spec that is not offered, for two unrelated
reasons.** That is the whole of the identity.

**Merging would assert something false, and the board already schedules
the counterexample.** A merged `ALL` is what the radio row walks, so
*drawable* would imply *offered by the add-datum form* — and
`work/view/revolve-tool-unreachable-no-axisinplane-form` is **open** and
asks for `AxisInPlane` to become authorable, which grows the form enum
to five while the draw tag stays at four. So the hold is the NAME:
`forms::DatumKind` → `forms::DatumKindChoice`, which is this crate's own
spelling for a form choice as distinct from the thing chosen among
(`PatternKindChoice`, `blend::BlendKindChoice`). **No mechanical hold,
deliberately** — there is no invariant to hold, and a test pinning the
two identical would have to die the day the revolve row lands.

The string tags stay on the form side (table data, walked by
`add_datum_ui`); `datums::DatumKind` gains no `ALL` and does **not**
become a `vocabulary!` declaration, because nothing iterates it for
words and it classifies drawings rather than offering a vocabulary. The
item's *"the survivor should be declared through `vocab.rs`"* applied to
the merge outcome only, and the closed item now says so.

**The neighbour row survives intact**:
`two-partial-mirrors-in-the-viewer-have-no-growth-alarm` is not closed,
not subsumed and not made wrong — its spec-to-kind alarm is still
unwritten, and the rename helps it, because its roster lives in
`forms.rs` and would otherwise have named a bare word that also resolves
to a public type.

### The correction that lands on this register

**I wrote "the real commands are the ones CI runs: 1, 2, 3". CI never
runs all three.** `ci.yml:1802-1809` runs `--selftest` and then an
**if/else** — bare when `run_viewer_toolkit` is true, else
`--skip-viewer-toolkit`. Alternatives on every run, never both.
Verified here against the workflow after the lane said so.

**And the wording undercut the very rule it introduced.** The reason a
viewer lane owes BOTH passes locally is that CI runs exactly ONE, so the
other is covered by nothing anywhere. A brief that says CI runs both
hands the lane a reason to skip the local run. **A mis-stated
justification for a correct rule is worse than none: it survives review
because the rule it guards is right.** `plan.md` now carries the
if/else and that lesson.

**A neighbour I failed to name.** The dispatch pointed at
`two-partial-mirrors-…` and missed
`revolve-tool-unreachable-no-axisinplane-form`, which carries the
**decisive** evidence — it turns the merge-is-false argument from a
thought experiment into a scheduled item. It also held two stale claims
(the type name, and the member order quoted as `Plane, Axis, Point,
Frame` before #2046 reordered it); both refreshed and both disclosed
rather than silently fixed.

**An observation passed on, not claimed**: `python suite` posted
`success` on a run where `ci-filter` reports `RUN_PNCAD_PY=false` and
`SEEDS=viewer` — either the local filter and CI's disagree, or the job
is green over skipped steps, which is the *green over a skipped row*
shape the discipline warns about. Related to the already-filed META row
about §2's deleted gate; not this lane's, and not resolved here.

**The sweep**: every `enum`/`struct`/`trait`/`type` declaration under
`crates/viewer/src`, deduped by bare name, with `vocabulary!` bodies
covered. Six repeats, all false positives or test-local. One real
cross-crate hit — `profile::ArcMode` vs `forms::ArcMode` — dispositioned
NOT this unit, because the viewer's is `pub(crate)` so the two are never
both reachable by the bare word from one scope, and the deliberateness
is already recorded on a DOOR row.

**VIEW stands at 72 open / 90 closed, nothing waiting on Ev.**
## `two-partial-mirrors-in-the-viewer-have-no-growth-alarm` — closed, one macro in two shapes (`view/partial-mirror-alarms`)

**The row asked whether one instrument covers both sites. It does, and
they take different shapes of it, and the shape split is not
cosmetic.** `partial_mirror!` moved out of `forms` to `vocab.rs` beside
`vocabulary!` — the move its own doc named for the moment a second
caller arrived — and grew from one arm to three, sharing the exhaustive
half through an internal `@exhaustive` rule.

- **`labelled <list>`** (`MATE_PRIMITIVES`) and **`bare <list>`**
  (`SUBJECTS_WITH_AN_EXPIRY_ISSUER`) are the two shapes `vocabulary!`
  already draws and carry its words. They differ only in whether a seat
  reads `list[n].0` or `list[n]`, and both get the seat half: one
  `assert!` per offered entry, and the count check.
- **`onto <Choice>`** (`DatumKindChoice` over `DatumSpec`) gets the
  exhaustive half ALONE, and the reason is structural rather than a
  saving. Its offering is an enum whose `ALL` is projected from the
  declaration, so there is no second copy of the membership for a seat
  assertion to hold — naming `DatumKindChoice::X` as a counterpart
  already says the radio row draws it. The roster is the spec-to-kind
  mapping instead, which is the direction that was held by nothing.

**The alarms are falsified, not assumed.** A sixth `Subject` reds
`E0004` at the roster's match; classifying it offered without growing
the list reds `E0080`, index out of bounds, at the seat assertion; a
sixth `DatumSpec` arm reds `E0004` at the `onto` roster — and at
`datum_node`'s lowering match, which is the point: that match asks what
a spec lowers to and says nothing about whether the FORM offers it.

**It survives the revolve row, demonstrated.** With
`revolve-tool-unreachable-no-axisinplane-form`'s fix simulated —
`DatumKindChoice` grown to five, `AxisInPlane` moved from the roster's
absent section to its offered section, `DatumSpec` and
`datums::DatumKind` untouched — the only reds are `pane::create`'s
three matches over `DatumKindChoice`, which that row has to write
anyway. #2561's ruling is intact: nothing here holds the form enum
against the draw tag, because what the roster mirrors is `DatumSpec`.

**The row's own citation named a symbol that does not exist.** It cited
`subject_of`'s match at `frame.rs:572`; there is no `subject_of` in
that file (the only one in the tree is a test helper in
`crates/geom-core/tests/bounds_census.rs`). The function is
`joined_subject` and the `_ => Subject::Document` arm is at `:574`.
That arm was the EVIDENCE for the first bullet's claim, so the wrong
name cost the argument its check. Corrected in place with a note; the
claim was true.

**Swept, and a third site filed.** The shape grep — a deliberately
partial mirror of an enum, in `crates/viewer/src/` — turns up
`session::author::PatternRuleSpec` over `pncad::document::PatternKind`
(two of three, `Explicit` ruled out by the plan). It is the one of the
three whose mirrored enum lives in another crate, so it is the
strongest case of the three and the only one where growth reds nothing
in the viewer at all; and `onto` does not serve it, because that arm
names its counterpart as a VALUE and both `PatternRuleSpec` arms carry
`Expr`s. Filed as
`patternrulespec-is-a-partial-mirror-with-no-growth-alarm` rather than
fixed, with the two answers stated. What the grep could not match:
mirrors that are not `const` items or enum declarations (an inline list
in a test row), mirrors spelled across two crates with no shared token,
and a list whose partiality is stated nowhere.

## 2026-09-14 — #2585 merged; and the item's own design would have written an assertion that cannot fail

**#2585 merged** (`f8cea967a6`), verified from the job list: **40 jobs,
12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok` success**, `render drift
(kernel)` neutral, no non-green row.

**One macro, three arms, and the split is structural.** `partial_mirror!`
moved from `forms.rs` to `crates/viewer/src/vocab.rs`, sharing the
exhaustive half through an internal `@exhaustive` rule. `labelled` and
`bare` are `vocabulary!`'s own two list shapes and keep the seat half;
`onto <Choice>` is the exhaustive half **alone**.

**The `onto` arm is where the item's own proposal was wrong, and the
reason is this register's favourite shape.** The item asked for a seat
half asserting against `DatumKindChoice::ALL`. But `ALL` is *projected*
by `vocabulary!`, so there is no second copy of the membership for a
seat assertion to hold — naming `DatumKindChoice::X` as a counterpart
**already** says the radio row draws it, because `add_datum_ui` walks
`ALL`. **The proposed assertion could not fail.** A seat roster there
would also have been the third hand-written enumeration the item itself
warned against. Each offered entry reads
`DatumSpec::Plane { .. } => Plane`, so the roster **is** the
spec-to-kind mapping.

**My own framing was off in both directions**, and the lane's is
better. I said `partial_mirror!` fits the second site *"only if you
squint"*, and the item said *the same skeleton, not the same arm* —
both locating the difference in *list versus enum*. The real difference
is that **one offering can drift from its roster and the other cannot**,
because `vocabulary!` already removed the second copy. That is what
decides the arm, and it is the fact that shows the seat half is inert
there rather than merely awkward.

**The macro had to MOVE, and that was not polish.** I passed on the
item's *"transfers with one more macro arm"*. `frame` is ungated
(`lib.rs:61`) and `forms` is `#[cfg(feature = "app")]` (`:87-88`) —
**verified here** — so taking the arm without the move would have left
an ungated module importing a macro from an app-gated one, which does
not compile at wasm32 or at default features. The macro's own doc had
named the move as the precondition. A cost stated as *one more arm* was
a cost of *one arm plus a move the compiler requires*.

**Survival across the scheduled row, demonstrated rather than argued.**
What the `onto` roster mirrors is **`DatumSpec`**, not
`datums::DatumKind`, so #2561's ruling is untouched. The lane simulated
the revolve fix — `DatumKindChoice` grown to five, `AxisInPlane` moved
absent→offered, `DatumSpec` and the draw tag untouched — and the only
reds were `pane::create`'s three matches, which that row must edit
anyway. `absent []` parses; the instrument stays silent.

**Falsification, each on a committed tree and reverted**: a sixth
`Subject` → `E0004` at the roster; that variant offered without growing
the list → `E0080` index-out-of-bounds at the seat assertion; a sixth
`DatumSpec` arm → `E0004` at the `onto` roster **and** at `datum_node`.
The lane is right that the second red is not redundancy: `datum_node`
asks what a spec lowers *to*, never whether the form offers it — which
is exactly how the revolve tool shipped with an unfillable seat.

**`subject_of` confirmed dead.** No such name in
`crates/viewer/src/frame.rs`; the only one in the tree is an unrelated
test helper in `geom-core`. The function is `joined_subject`
(`frame.rs:570`), wildcard arm at `:574`. Name fixed on the item with
the correction distinguished from a claim change.

**A fourth site, filed not fixed**:
`session::author::PatternRuleSpec` mirrors two of
`pncad::document::PatternKind`'s three arms with no alarm. It is the
**strongest** of the four — the mirrored enum is in another crate, so a
kernel lane adding an arm reds nothing in the viewer — and it does not
fit `onto`, whose counterpart is a *value*, because both
`PatternRuleSpec` arms carry `Expr`s. Filed as
`patternrulespec-is-a-partial-mirror-with-no-growth-alarm`, with the
macro doc stating the restriction and naming the site.

**VIEW stands at 72 open / 91 closed, nothing waiting on Ev.**

## 2026-09-14 — `ui-thread-work-after-the-index-seam`, hit (1) taken

**One of three, and the other two re-stated from a stopwatch rather
than from a shape.** The item said each hit was its own decision; the
lane took (1), the display budget's probe tessellation, and measured
all three before choosing.

**(1) moved onto a third worker.** `evalseam` gains `FitService`,
`FitRequest`/`FitSubject`/`FitDone`, `InlineFitter` and `ThreadFitter`
beside the two seams it had — the index seam's shape exactly, keyed by
generation alone because the δ is the request's ANSWER. The ordering
the item asked about turned out to be **load-bearing**: the index is
built at the δ the fit chooses, so it has to wait for it.
`PickCache::sync`'s δ became an `Option`, and an unsettled one takes
the same nothing-to-index way out the nothing-landed arm takes —
forgetting the held index, so *current or absent, never behind* is
unchanged in the new window. Making it an `Option` rather than an `if`
at the call site is the point: the un-budgeted build cannot be
submitted by forgetting to write the guard.

**Measured, release, over `viewer`'s own corpus.** The ladder costs
**0.10–0.13 of a full tessellation** on every document dense enough to
matter — this file's "about an eighth" confirmed with an instrument —
which is 118 ms (`loft_prism`), 116 ms (`tube_ring`), 64 ms
(`hollow_tube_ring`) of frozen window at 1e-5, and is what makes 6b's
6.5 s `hollowring` row the ~0.8 s the item recorded.

**The ranking the item guessed is inverted.** It said "(2) and (3) in
particular could be milliseconds". (3) is: `DocSession::land` is under
6 ms on 27 of 28 corpus documents, one outlier at 197 ms. **(2) is the
biggest of the three by an order of magnitude** — `scene_focused` over
an already-built index reads **5 123 ms** on `hollow_tube_ring` at
1e-5, 2 322 ms on `tube_ring`, 1 682 ms on `loft_prism` — and it runs
per HIDE and per FOCUS change rather than once per document. It is
also **ten times a full tessellation of the same body**, which is the
part a taker should chase before assuming the answer is a seam: at
~5 µs per triangle, a corner-copy-and-normal walk is not doing what it
looks like it is doing.

**Two citations repointed as the orchestrator said** — `fit_delta` at
`scene.rs:1094` (was `:994-1000`), `scene_focused` at
`pickindex.rs:941` (was `:894`); subjects present at both new numbers.
`build_parts_focused` at `scene.rs:439` and `DocSession::land` at
`session.rs:966` were exact.

**Filed out of the way:** `crates/viewer/README.md`'s "23 hits" sweep
count reads 24 under a mechanical re-take of its own words, on
`origin/main` and before this change —
`viewer-readme-multi-field-write-sweep-count-does-not-reproduce`.

**VIEW stands at 73 open / 91 closed, nothing waiting on Ev** — 72
before this lane, plus the sweep-count row above. The item itself stays
OPEN: one hit of three is done.

## 2026-09-14 — #2606 merged; the fit gets a third worker, and the item's cost ranking was inverted

**#2606 merged** (`3d12be4739`), verified from the job list: **39 check
runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok` success**, all four
render-lane rows success, six skipped, nothing failed or neutral.

**One site of three, deliberately**, and the item stays **open** with
the other two re-stated from a stopwatch. The dispatch said a wide
shallow pass across three seams would be worse than one argued change,
and this is what that looks like: `scene::fit_delta` now runs on a third
worker (`FitService`, `FitRequest`/`FitSubject`/`FitDone`,
`InlineFitter`, `ThreadFitter`), with `DocSession::fit_request` minting
the request the way `index_inputs` mints the index's.

**The ordering I asked about is load-bearing, and the answer shaped the
design.** The fit's answer IS the δ the index is built at, and
`pickcache`'s doc ratifies *"δ is built at, verbatim"* — so the index
must wait, or a fit that merely moved to a worker would let the index
submit at the δ in force and pay the un-budgeted build the budget exists
to avoid. `PickCache::sync`'s δ became `Option<DisplayTolerance>`, where
`None` is *landed, no δ settled yet* and takes the nothing-to-index way
out. **An `Option` rather than a caller-side `if` is the point**: the
un-budgeted submit cannot happen by someone forgetting a guard.

**`evalseam` had no reusable harness to offer** — two parallel seam
traits with their own `Inline`/`Thread` pairs, so a third seam is ~200
lines mirroring the second. The lane took it anyway and gave the reason:
the real constraint was on the OTHER side of the seam (pickcache's
verbatim-δ rule), not inside it. Folding the fit into `IndexRequest` was
considered and rejected — it collides with that rule and with the
`(generation, δ)` cache key.

### Measured, and the item's ranking is upside down

Release, the viewer's own corpus, scratch harness.

- **Site 1** (taken): 118 ms `loft_prism`, 116 ms `tube_ring`, 64 ms
  `hollow_tube_ring` at 1e-5 — ratio **0.10–0.13** of a full
  tessellation on every dense document, which confirms the item's
  *"about an eighth"* as a measurement and makes 6b's 6.5 s row ≈0.8 s.
  After: an `Arc` clone and a channel send.
- **Site 3** is the smallest: `land` under 6 ms on 27 of 28 gathering
  documents, one outlier at 197 ms.
- **Site 2 is the biggest by an order of magnitude**, and the item
  guessed it *"could be milliseconds"*. `scene_focused` over an
  ALREADY-BUILT index at 1e-5: `hollow_tube_ring` **5,123 ms**,
  `tube_ring` 2,322 ms, `loft_prism` 1,682 ms. It runs per hide and per
  focus change, not once per document, and on `hollow_tube_ring` it is
  **ten times a full tessellation of the same body** (511 ms). At
  ~5 µs/triangle a corner-copy-and-normal walk is not doing what it
  looks like, so the item now says **find out where the time goes before
  choosing a seam** — the measurement changed the next unit's shape.

### Where my dispatch was wrong

**"Three files drive a headless `egui::Context`" — four do, and the
distinction that matters is different.** Only `app.rs` drives a
`ViewerApp`, and `ViewerApp::assemble` is **private**, so that harness
lives in `app.rs`'s own `mod tests` — the `--lib` suite — not in
`--test all`. An integration-test receipt about frame work cannot use it
as it stands. I had offered that instrument as available for exactly
that purpose.

**And the lane hit the grep trap I had warned it about, caught itself,
and said so**: `grep -rln "assemble\|toolbar_ui" crates/viewer/tests/`
returned three files and looked like confirmation — it had matched
`pncad::…::assemble`, the kernel function. The warning worked as a
warning rather than as a prohibition, which is the useful outcome.

### The ratified-page question, ruled

`crates/viewer/GUI-DESIGN.md` took three edits and the lane flagged the
third as possibly-binding rather than assuming. **Ruled: it lands.**
Every paragraph describes what this change did — a third seam stated in
the terms the existing two are stated in — and the one generalisation
(*why the two seams are two workers* → *why each seam is its own
worker*) is the existing clause's own argument covering three cases
instead of two. Nothing retires a clause or changes what one decides,
which is `CLAUDE.md`'s test.

The lane also ran the provenance check before flagging and found the
surrounding sentences trace to `e9824abf3b` — the move that created the
file — **not an Ev ratification**. And its `-S` search for
*"It runs on its own worker now"* returned nothing until joined across
the line break: the prose-grep trap, live, for the fourth time this
week.

**Filed out of fence**, and carefully:
`viewer-readme-multi-field-write-sweep-count-does-not-reproduce` —
`README.md`'s *"23 hits, and none is a census"* reads **24** under a
mechanical re-take of its own words, on `main` before anything of this
lane's. One hit of difference means the instrument is approximate, so
the row **asks for a re-take with a stated instrument rather than
asserting 24**. The claim above the number survives either way. The
phrase in that paragraph the change genuinely falsified — *the two
`Drop`s in `evalseam`* — was corrected.

**VIEW stands at 73 open / 91 closed, nothing waiting on Ev.**

## 2026-09-15 — `ui-thread-work-after-the-index-seam` hit (2): DIAGNOSED, not fixed

Took the measurement unit the item's (2) section asked for and stopped
where the brief said stopping was an outcome. **Neither candidate the
item named is what the time is.**

- **The walk is linear and it runs at copy-loop speed**: 53–67
  ns/triangle across `hollow_tube_ring`, `tube_ring`, `loft_prism`,
  `hollow_tube_elbow` and `die_composed_tour`, spanning 6.4× in
  triangle count and both δ rows. `PickIndex::parts()` is ONE part on
  all five, and `SceneMesh::stats().triangles` equals the
  tessellation's own count exactly — so the walk is over the product's
  triangles once, not several times.
- **The item's "~5 µs per triangle" and its "ten times a full
  tessellation" are both the same arithmetic error**: the 5 123 ms is
  δ=1e-5 and the 511 ms tessellation it is divided by is δ=1e-4 (I
  measure 518 ms there). At equal δ `hollow_tube_ring` tessellates in
  **7 349 ms** and `scene_focused` is **0.61× cold, 0.09× steady**.
- **Where the time goes is first-touch on ~1.25 GB of vertex buffers.**
  Phase timing inside `build_parts_focused`: every phase that allocates
  is 6–9× slower on the first build at a given size and flat after;
  `Aabb::from_points`, which allocates nothing, does not move at all.
  So a hide or focus change costs **~0.7 s** on the worst corpus
  document at 1e-5, not 5 s — and the 4–5 s first build is paid by
  construction, because `ViewerApp::sync_scene` holds the previous
  `Arc<SceneMesh>` alive across the new build so a refusal leaves the
  stale picture up.

**Changed nothing, deliberately.** The seam question is now a 0.7 s
question and the lever is rebuild SCOPE and buffer SIZE rather than
where the walk runs: a focus change alters 4 of the 108 bytes a
triangle emits and still rebuilds all of them. That is a change to what
`SceneMesh` is, and it wants a ruling before a diff. **The item stays
OPEN**, with the numbers, the phase split and that argument recorded on
it; (3) also remains untouched.

**Filed:** `scene-mesh-carries-an-identity-index-buffer` — the scene's
index buffer is `(0..n).collect()`, 139 MB and 8–11 % of the step on
`hollow_tube_ring` at 1e-5, with one production reader that only wants
its length. Left for its own unit because it touches the draw call and
no headless row here has a device.

## 2026-09-15 — #2613 merged; hit (2) is diagnosed and the item's headline number was an artifact

**#2613 merged** (`d5acf3db90`), verified from the job list. Diff is
`work/`-only, so this is the **docs tier** and the shape that certifies
it is not the code tier's: **22 check runs, every code row `skipped`**,
with `gate ok`, `change filter`, `CI half parity + gate wiring (every
tier)` and `docs-only ok` success and nothing failed or neutral. My
dispatch template's validation item 8 asks for `test (…)` and `k-lint
(gate, …)` counts and forbids citing `docs-only ok`; on this tier both
counts are zero and the forbidden row is the only green one, so the
lane was left with no receipt it was allowed to give. **The lane said
so, and it is right** — recorded in `plan.md`, template fixed.

**Neither candidate the item named is where the time goes.** I checked
both load-bearing claims rather than the summary:

- The walk in `SceneMesh::build_parts_focused` sums triangles over
  `parts × patches × triangles` and emits from the same iteration
  (`crates/viewer/src/scene.rs:452-466`), so
  `stats().triangles == 11 605 976` on both sides is a stronger check
  than the part count itself — a per-`(node, body)` multiplication
  could not produce equality.
- **The δ mismatch is corroborated by the item's own text.** The record
  divided a δ=1e-5 time (5 123 ms) by a δ=1e-4 tessellation (511 ms)
  to get *"ten times a full tessellation"*. The item's hit (1) section
  already cites 6b's **6.5 s** fine-δ tessellation of that same body —
  511 ms cannot be the fine-δ number, and the lane measures 518 ms at
  1e-4 and 7 349 ms at 1e-5. The contradiction was inside the file.

The lane's sweep reproduces exactly: `grep -rn "\.indices()"` over
`crates/viewer/src`, `crates/viewer/tests`, `demos/` returns the three
hits it dispositioned and no others, and `gpu.rs:1172` does carry the
non-indexed `draw` the filed row proposes to reuse. `app.rs:904-913`
confirms *by construction*: `scene_focused` builds the whole new mesh
while `self.scene` still holds the old one, and the assignment sits in
the `Ok` arm with a comment giving the reason — a refused build must
leave the stale picture up. Peak of two pictures' buffers is a
contract, not an oversight.

**So the seam question shrank from 5 s to ~0.7 s**, and the item stays
OPEN with the real lever named (rebuild scope and buffer size, not
where the walk runs). Site (3) untouched. Filed:
`scene-mesh-carries-an-identity-index-buffer`.

**VIEW stands at 74 open / 91 closed, nothing waiting on Ev.**

## 2026-09-15 — `index-reads-without-the-evaluation-co-guard`

Branch `view/index-co-guard`. **The item's framing moved twice under the
sweep**, and both moves are recorded on the item.

The population is **eight** uses of a pane's `&PickIndex` at four
bindings, not the item's six at one file: `pane::create`'s all-edges
button takes one too (already with the evaluation), and the two the item
does not enumerate — `marks::highlight` and `marks::edge_overlay` — were
both ungated and both picture-side.

And the harm site does not want the evaluation. An id is a word of the
alphabet of the index that minted the drawn corners, so
`frame::disagreement` wants the scene and the index to be ONE BUILD.
`ViewerApp::scene_generation` — written twice, read nowhere — became
`scene_key: Option<(Generation, DisplayTolerance)>`, taken from the
index rather than re-derived from the session, and
`pane::viewport::drawn_index` asks `PickIndex::current_for` with it.
Both halves of the key, because a δ typed while the document stands
rebuilds the index at the same generation over a different tessellation;
a generation-only guard would have read as co-identity while checking
something else. Five picture-side uses route through it, the two
document-side ones keep the index+evaluation pair, and the identity read
(`PickIndex::generation` as a cache key) keeps neither.

Two rows filed rather than absorbed:
`id-query-is-keyed-on-the-generation-not-on-the-picture` — a SECOND
producer of *the two picking paths disagree*, which no co-guard closes,
because there the index is the drawn one and the GPU ANSWER is stale —
and `a-pick-over-a-stale-picture-answers-about-a-picture-nobody-can-see`,
the product question the rule's document-side half leaves open.

## 2026-09-15 — #2615 merged; a picture-side index read is guarded on the picture's key, and my proposed guard was short a term

**#2615 merged** (`26f77ea66d`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, six skipped, nothing
failed or neutral.

**I dispatched a wrong guard and said it was a claim; the lane found
the missing term.** My brief proposed `scene_generation ==
index.generation()` as the honest check at `frame::disagreement`. It is
right in substance and **insufficient**: `PickIndex::current_for`
(`pickindex.rs:820-822`) compares `(generation, δ)`, because a δ typed
while the document stands rebuilds the index at the **same generation**
over a different tessellation — a different id alphabet under an
identical generation. A generation-only check would have read as
co-identity while checking something else, **which is the exact defect
the unit was sent to remove**. Recorded as such: proposing the fix
shape is where a dispatcher's exposure is highest, and stating it as a
claim is what let the lane overturn it.

**`scene_generation` had no reader at all.** `git grep` on `main`:
declaration, initializer, one write, zero reads — and its doc said
*"when it disagrees with the session's landed generation, the picture
is out of date"*, a sentence nothing checked. It became
`scene_key: Option<(Generation, DisplayTolerance)>`, taken from the
index rather than re-derived from the session, and got the reader that
makes its doc true.

**The item's population was wrong twice** and the lane's census stands:
eight uses, not six, across two files. `marks::highlight` and
`marks::edge_overlay` are picture-side and the item omits both, so a
fix guided by its list would have repaired two of five. `create.rs:1133`
is the document-side read the item never mentioned, already correct
(`target.zip(self.session.evaluation()).zip(self.index)`).

**My shift map: four rows right, one gloss wrong.** I wrote that `:302`
*"reads `self.indexing`, not `self.index`"* — true of the line and
wrong about the site, which is the `else` arm of the `:284` guard and
so is exactly the item's read-about-the-index's-ABSENCE. A lane
trusting the gloss would have concluded the item's citation was bogus.
Four of five subjects right is better than the 2-of-4 that made me
warn about shift maps in the first place, and the failure mode moved
from *wrong line* to *right line, wrong description* — which a lane
catches only by opening the file, exactly as this one did.

**One correction I made after merging, not on the lane branch.** The
README's sweep paragraph named the guard's binding `drawn`; the lane's
final commit renamed it `on_screen` precisely because two unrelated
inner bindings at `viewport.rs:424` and `:432` are called `drawn`, so
the prose sent a reader to the wrong symbols. The branch was verified
green at `469eb52b32` and re-pushing for one word would have cost a
full code-tier re-run and a second 39-row verification; the fix rides
here instead. Same end state on `main`, stated in both places.

Filed by the lane, both on VIEW's slate:
`id-query-is-keyed-on-the-generation-not-on-the-picture` — a **second
producer of the same false sentence that no co-guard closes**, since
`IdQueryLog::step` keys on `(cursor, generation)` while the scene also
rebuilds on display-revision and focus changes that hold the generation
still — and
`a-pick-over-a-stale-picture-answers-about-a-picture-nobody-can-see`,
the product question the rule's document-side half deliberately leaves
open.

Item **closed**. **VIEW stands at 75 open / 92 closed, nothing waiting
on Ev.**

## 2026-09-15 — #2622 merged; the id query's key is the pair, because the item's own check found the counterexample

**#2622 merged** (`3c79322358`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, six skipped, nothing
failed or neutral.

**The item proposed `revision`; the answer is `(revision, generation)`,
and the item is what found that out.** Its own *check before taking it*
asked whether a generation change exists with no scene rebuild behind
it. One does: `ViewerApp::revision` has exactly two writes (`app.rs:738`
init, `:929` bump) and **the bump is inside `sync_scene`'s `Ok` arm**,
beside `scene_key`, `scene_display`, `scene_focus` and `scene`. A
refused `scene_focused` writes `scene_fault` and nothing else, on
purpose, so the pair is retried rather than marked current — which
leaves **a new generation beside the picture already on screen with the
revision unmoved**. Keyed on the revision alone the query would Hold,
and the hover it skips on a `Hold` is a question about the DOCUMENT,
which has moved. I verified both writes myself.

`frame::IdSubject { revision, generation }` now carries the argument for
asking both, with each direction's reachability written at the type. A
paragraph in a doc is where this program's invariants keep dying; a
named type is where this one now lives.

**The δ omission is argued, not overlooked** — and that is the half
worth checking, because an omission is where this program's defects
live. `PickCache::sync` nulls the held index at every submit
(`pickcache.rs:324`, *"dropped before the answer, not after it"*) and
`land` is the only installer (`:417`), so an index cannot change δ
without the key seeing `None` in between. `Option<Generation>` carries
that `None`. Checked.

**#2615 did not close this**, and the lane checked rather than reasoning
from the PR: on a hide, `scene_key` is written from
`(index.generation(), index.delta())` and a hide moves neither, so
`drawn_index` answers true and hands the index straight back.

**The mutation receipt found the silence the item predicted.** Planted
generation-only: the new row fails at *"a new picture at one generation
re-asks"*. Planted revision-only: it fails at *"a new index at one
picture re-asks"*. **The pre-existing row
`the_id_query_is_asked_once_per_cursor_and_re_asked_when_the_picture_moves`
stayed green under BOTH** — it moves cursor, generation and revision
together and cannot tell the three keys apart. That is exactly the
silence the item warned of, demonstrated rather than asserted.

**The sweep's blind spot was closed by the compiler, not by a pattern.**
Five patterns, none of which can match a call through a binding under
another name — the class that cost the co-guard unit five of eight
index uses. What closes it here is that `step`'s second parameter
changed TYPE, so every caller anywhere must be edited or the build
fails. Worth recording as a rule: **when a sweep's blind spot is
"a use under another name", changing a type is a census a grep cannot
be.**

Item **closed**. **VIEW stands at 74 open / 93 closed, nothing waiting
on Ev.**

## 2026-09-15 — #2625 merged; a justification that borrowed another path's mechanism was hiding which guard was load-bearing

**#2625 merged** (`9c379b9fa6`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, six skipped, nothing
failed or neutral.

**The mechanism check is the whole unit, and it inverted the fix.** My
brief said candidate 1 — make `highlight` dedupe like `edge_overlay`
does — *"needs the shader's precedence checked, not assumed"*. Checking
it found something better than a caveat: **the edge path has no
downstream precedence at all**, so `edge_overlay`'s filter is not an
echo of the face path's ruling. It is the edge path's ONLY statement of
one. I verified every link:

- an edge vertex carries exactly one mark word — `EDGE_MARK_SELECTED`
  is `0` and is the ABSENCE of `EDGE_MARK_HOVERED`, with the doc at
  `gpu.rs:265` saying why (*"two bits would admit a state meaning
  both"*);
- `ensure_geometry` writes the selected lane (`:925`) then the hovered
  lane (`:929`) into one buffer for one draw;
- the edge pipeline is `blend: None`, `depth_write_enabled: Some(false)`,
  `depth_compare: LessEqual` (`:863-886`), so identical geometry drawn
  second **overwrites**;
- the face shader's `Uniforms::highlight` is `[selected, hovered, 0, 0]`
  (`:1087`) and hovered is reached by an `else if` (`:1335`), so the
  face path gives **selection** precedence.

**Without the filter the two halves would disagree in OPPOSITE
directions** — edges resolving hover over selection, faces the reverse.
So they obey one rule, each applying it at the last place that can see
both answers: the fragment for faces, Rust for edges.

**And candidate 1 would have been a silent guard removal.** It changes
no pixels, because `fs_main`'s `else if` already gives selection
precedence — so it would have left the shader's stated arbitration with
no reachable input while every row stayed green. `Highlight`'s fields
are `pub` and the type is re-exported from `lib.rs`, so `crate::marks`
is not its only possible producer and the shader arbitrates for all of
them. That is this program's own silently-never-fires shape, and my
brief offered it first.

**The rule this earns** (recorded in `plan.md`): **a justification that
cites another path's mechanism is not yet a justification.**
`edge_overlay`'s doc said its filter was *"the precedence the shader's
face path already states"*. The borrowed wording is what made an
essential guard read as a redundant one — and it would have survived
review, because the sentence it borrows is true about the path it
borrows from.

What was actually wrong was the record: `EdgeOverlay::hovered`'s own
field doc said *"the hovered edge's segments"*, which is the convention
the function does not implement. Both types now state their own
convention where a reader meets them.

**No README amendment, argued rather than skipped.** The README has no
mark-precedence clause, and putting the invariant there would re-create
this item's defect one level up — the record living somewhere other
than where the type is read. The lane said so instead of adding prose
by reflex.

**The sweep found no third member**, and both of the item's named
candidates are non-members with reasons: `theme`'s `selected`/`hovered`
are palette entries, always both present, never an answer about one
gesture; `Mark::over` composites one mark rather than ordering a pair,
and the ordering the item attributes to it is in the shader.
`Selection` against `DocSession::hover` is the SOURCE of the pair,
deliberately un-narrowed. Stated blind spot: **the pattern is the
word** — it cannot see a `committed`/`transient` or
`primary`/`secondary` pair, or one split across two types.

Item **closed**. **VIEW stands at 73 open / 94 closed, nothing waiting
on Ev.**

## 2026-09-15 — #2638 merged; my ctrl+scroll claim was false, and the toolkit's own source says so

**#2638 merged** (`1024e37ca6`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, six skipped, nothing
failed or neutral.

**I passed on the item's false claim and added my own weight to it.**
The row said an unread `ctrl` means *"a ctrl+scroll and a plain scroll
are the same event"*, and my dispatch called it *"the one place where
the dropped input has a conventional meaning the viewer already
implements by accident"*. Both false. I checked egui 0.36.1 myself
rather than take the correction on report:

- `input_state/mod.rs:455` starts `smooth_scroll_delta` at `Vec2::ZERO`;
- `:461` `is_zoom = wheel.modifiers.matches_any(options.zoom_modifier)`,
  and `zoom_modifier` is `Modifiers::COMMAND` (`:118`);
- `:463-468` is an if/**else**: on zoom the delta goes to
  `zoom_factor_delta` and `smooth_scroll_delta` **stays zero**.

So a **ctrl+wheel produces no `ViewportEvent` at all** — it does not
zoom, and it is not the same event as a plain scroll. Reading `ctrl` in
the adapter would have recovered nothing. It is a live product gap in
the gesture every browser and every mainstream CAD package zooms with.
Filed as `ctrl-wheel-reaches-no-zoom`, not disclosed in prose.

**The same mechanism answers two gestures the item never reached**, and
I verified these too (`input_state/wheel_state.rs:120-133`):
`horizontal_scroll_modifier` is SHIFT, so a **shift+wheel is folded onto
`x`** and dropped — `x` is not merely a trackpad swipe, it is where
shift+wheel lands on any mouse; `vertical_scroll_modifier` is ALT, so an
**alt+wheel folds onto `y`** and zooms like a plain wheel. That makes
the `x` decision specific rather than plausible: zoom is a scroll's only
binding, so a passed-on `x` would have to zoom, and a sideways swipe
that zooms is worse than one that does nothing.

**Generalises, and it is the borrowed-mechanism rule one step out.**
The item reasoned about `egui::Modifiers` as a bag of bools the adapter
reads, and asked which the viewer binds. The toolkit had already
consumed three of them upstream of the field the adapter reads, so the
question *"which modifiers do we bind"* was the wrong question — the
answer lived in `begin_pass`, not at our call site. **When a row says a
field is unread, check what the producer does with it before deciding
the fix is to read it.**

**The citations were stale by arithmetic and right about their
subject** — #2450 inserted above them, so the reads are at `:247` and
`:261`, not `:195`/`:209`. The lane read the lines rather than shifting
the numbers, which is the method that keeps working.

**The compiler lever was available and taken, with its limit stated.**
`egui::Modifiers` and `Vec2` are plain structs, not `#[non_exhaustive]`,
so a destructuring pattern naming every field is the struct form of
#2450's exhaustive match — demonstrated with a sixth field:
`error[E0027]`. The lane then followed the compiler's own printed
repairs and found the third suggestion is `..`, so the doc says **stop,
not wall**, instead of claiming the compiler forces a decision. The
PAIRING is still unheld — `(shift, alt)` swaps type-check — so a row
holds it over six modifier combinations.

Item **closed**. **VIEW stands at 73 open / 95 closed, nothing waiting
on Ev.**

## 2026-09-15 — #2637 merged; the item's eval half was false when it was written, and the fit seam shows why

**#2637 merged** (`30a6b9e5c0`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, six skipped, nothing
failed or neutral.

**Census: three seams, ONE defect.** My dispatch expected a third
instance on the fit seam. Half right — `ThreadFitter` has the identical
`Disconnected` handling, but **its consumer has no gap**, and the reason
is the finding: `ViewerApp` holds the `Box<dyn FitService>` directly and
keeps **no mirror of it**. `app.rs`'s own comment at the `settled` read
argues that very rule — a δ typed while a fit is outstanding is
*"accepted rather than tracked: the alternative is a second record of
what the seam already holds"*. The index seam kept exactly that second
record. So the defect is not "consumers forget to ask the seam"; it is
**keeping a duplicate of the seam's state at all**, and two of three
consumers already avoid it.

**The item's eval half was wrong when written, not stale — I checked
the history rather than taking it on report.** At `bf4e3d16b5`,
`frame::progress(busy, running, indexing)` reads
`(true, false, indexing) => Some(Progress::Canceled { indexing })`, so a
dead evaluator (`busy` true, `running` false) has **always** reached the
toolbar as *canceled — showing an older result*, never as a permanent
`evaluating…`. And `DocSession::running()` is literally
`self.eval.busy()` (`session.rs:860-862`) — it consults the seam.
`DocSession::busy()` is `landed_generation() != generation`
(`:846-848`), which answers *am I showing the current document* and is
**true** of a dead evaluator: the right answer to a different question.
**The item conflated the two.** Its stated reason was respected, not
overridden.

**The fix is both halves**: `outstanding.is_some() && seam.busy()`, each
with a row that reds without it — the record alone promises an answer
nobody will send, and the seam alone lights the indicator for a build
`forget` has already orphaned.

**The test panics a real thread**, behind the same two channel ends a
shipped handle keeps, with the impossibility stated honestly: the
shipped handles own their worker's entry point (`index_work`/`eval_work`
private, no injection door), so a shipped worker can only die by a panic
inside a build — which a test would have to manufacture as a kernel bug
and which would go green for the wrong reason the day the kernel
hardened. The panic is genuine; only its site is the test's.

**The census rested on ownership, and the lane said so rather than
claiming the compiler.** `indexing()`'s signature does not move, only
its body, so **the compiler is NOT the census here** — that lever
(`plan.md`'s type-as-census rule) was unavailable, and the lane named
its absence instead of borrowing the rule. What carries the census is
that a seam handle is a `Box<dyn …Service>` field, so grepping the
handle type finds every owner: exactly three. Its own blind spot is
stated too — `ViewerBehavior::indexing` is a bare `bool` and matched
none of the name patterns.

**The item's README citation was imprecise and the lane corrected it
instead of inheriting it**: there is no one-progress-state rule in
`crates/viewer/README.md`; that rule is `frame::Progress`'s own doc
header. Checked with a newline-collapsed grep, for the split-span case.

**Residue for Ev**: `a-dead-seam-worker-reads-as-an-ordinary-idle-state`
— what the chrome should say INSTEAD of the withdrawn promise, on all
three seams. It records two things this fix leaves standing: a dead
evaluator's **`Re-evaluate` button submits into a `Sender` whose
receiver died and changes nothing**, and a dead fitter silently makes
every index build the un-budgeted one.

Item **closed**. **VIEW stands at 73 open / 96 closed.**

### Two harness facts from this lane, one of them a rule breach

**The first CI run red on infrastructure and ran no test at all**:
`actions/download-artifact` failed after five retries fetching
`nextest-interval` from blob storage, and all three legs reported *no
output captured*. Unreachable by the diff.

**This account gets 403 on `rerun-failed-jobs`**, via both the API and
the MCP tool — so the lane could not re-run the job and **pushed an
empty commit to re-take the run**, saying so in the message. That is
against the standing rule *never push an empty commit to kick CI*. The
judgement underneath was right — a genuine infra failure with no test
executed — but the mechanism was the forbidden one, and the reason it
was reached for is the 403. Raised with Ev: **no lane can re-run a
failed job here**, so the only paths left are an empty commit or a
force-push, and both are barred. That needs an answer before the next
infra flake, not after.

## 2026-09-15 — #2661 merged; the identity index buffer goes, and the receipt was a CI job read by its STEPS

**#2661 merged** (`3c43ef8ebe`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, six skipped, nothing
failed or neutral. `gate ok` posted at 14:03, about 40 minutes after the
run began — `k-lint (gate, release-default)` runs the demos-tour suite
and two sweeps before its three gates, so a long tail there is
structural, not a stall.

**The unit's whole difficulty was that nothing local could test it.**
There is no GPU adapter on this machine, so the `--lib` suite cannot
exercise the draw call the change rewrites, and I said so in the
dispatch rather than letting a lane discover it. The lane's answer is
the right one and worth keeping as the pattern: **it read the
`render lanes / viewer gui montage` job by its STEPS, not by its name**
— release build with `--features app`, the software-Vulkan headless
stack, `demos/render-gui.sh` for 28 s, artifact upload, then the pixel
drift and re-baseline steps, each success. A green job over a skipped
step is a failure shape this repo has been bitten by before, and a job
name does not distinguish them.

**Two corrections to my brief, both of which I verified in the code:**

1. **I said `gpu.rs` swaps `draw_indexed` for `draw` — singular. There
   are TWO scene passes**, the shaded pass in `paint` and the id pass in
   `read_id_at`, each with its own `set_index_buffer` + `draw_indexed`.
   The item reads singular too. Both changed.
2. **I told the lane — and told CHROME — that this resolves
   `gpu-index-counts-substitute-u32-max`. It does not.** That row names
   two sites, and this change RELOCATES the first rather than resolving
   it: `corner_count` (`gpu.rs:414`) still spells
   `u32::try_from(…).unwrap_or(u32::MAX)`, now over
   `scene.positions().len()`, and the edge overlay's `vertices`
   (`:965`) was always the second. The typed refusal that row wants is
   unwritten at both. Correction relayed to CHROME.

**The replacement assertion is the point of the unit.** The old row
asserted `indices().len() == positions().len()` — a check no bug can
break. The new pair asserts the draw RANGE is
`stats().triangles * 3` and equals the length of every per-corner table
the passes bind (**`flags` was asserted nowhere in the suite before**),
and that both passes draw that count with no index buffer. Both
mutation-proved. Their limit is stated rather than glossed: one pins a
value, the other pins the TEXT of two call sites, and **neither reaches
a device** — the draw call is covered by the montage and by nothing
local.

**The lane declined to re-measure and was right to.** It cited the
139 MB / 8–11 % figures as the diagnosing lane's, **at δ=1e-5**, rather
than re-taking them against a release build and a corpus harness that
was never in the tree with ~7 GB free. It did check the one part that is
arithmetic rather than measurement: 34 817 928 corners × 4 B =
139 271 712 B, so 139 MB is exact decimal. That is the
number-with-its-setting rule applied correctly, including to a number it
inherited.

**It also disclosed four citations that were ALREADY STALE at its merge
base** rather than repointing them — which is exactly the trap CHROME's
repoint unit fell into, minting fresh wrong claims while fixing a
uniform shift. Two in-fence shifts it re-derived by subject at both
ends; one true out-of-fence shift and the four pre-existing ones it
reported without editing.

**And it stood down cleanly when asked.** After five addenda reporting
identical CI state it had re-armed a repeating monitor; told to cancel,
it did, and **declined to push a one-line correction to its own PR body
because the push would have re-triggered CI on the head I was watching**.
That is the right trade and the right instinct about what a push costs.

Item **closed**. **VIEW stands at 74 open / 97 closed.**

## 2026-09-15 — `view/stale-pick`: a click over a stale picture refuses

Ev ruled the product question on 2026-09-15: **refuse**. The pick path
now asks `drawn_index` — the same picture-side predicate, keyed on
`(generation, δ)` through `PickIndex::current_for` — and differs from
the picture-side reads only in what it does on `None`: they skip
silently, it refuses typed. The claim that the pick might want a
different predicate did not survive reading it; an id and a pick resolve
against one alphabet.

The refusal is a **third arm of `pickcache::NotIndexed`**,
`AnotherPicture`, rather than a vocabulary of its own: that type's
stated subject is *no index describes the picture on screen*, and an
index for a picture nobody has seen is the third way the sentence is
true. `unindexed` takes the index in hand as a parameter; it and
`indexing` cannot both be set, because `PickCache::sync` drops the held
index in the step that marks a build outstanding, so they are not a pair
of flags a caller could swap. The arm is retired by a SCENE rebuild
where the other two wait on an index build — both seams sit under
`Subject::Display`, so the subject read off the type is right for all
three, and the arm's doc names its own event for a later split.

**Reachability: no end-to-end row is arrangeable.**
`ViewerBehavior::viewport_ui` is a private method over an `egui::Ui`
painting through a wgpu callback; nothing headless drives it, which is
the wall `crates/viewer/tests/panel_display.rs` already records for the
parameter field's widget and the reason the viewport's own probe senses
only the event translation. Stated rather than approximated with an
adjacent row. What landed instead: the door's arm in `frame_policy.rs`,
and the PAIR of predicates the pick path composes — over a real
session — in `pane/viewport.rs`'s own tests.

`crates/viewer/README.md`'s *A pick id is one index's word* no longer
says the pick path is deliberately ungated; the re-statement is record
of what the code does and lands with the change.

## 2026-09-15 — #2662 merged; Ev's ruling landed as a third arm, and the wall was stated rather than papered over

**#2662 merged** (`31e1bcadd9`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, six skipped, nothing
failed or neutral. The lane's own log entry is above; this records the
verification.

**Ev's ruling (refuse) is implemented, and the vocabulary question was
decided by the vocabulary's own stated subject.** The refusal is a
THIRD ARM, `NotIndexed::AnotherPicture`, not a new type — because
`pickcache`'s module doc already says `NotIndexed` is *"the typed
refusal a pick stream earns while no index describes the picture on
screen"*. I checked that quote and it is verbatim at
`pickcache.rs:21-22`. The subject was always the PICTURE; the two
existing arms are the two ways that holds when no index exists, and an
index in hand for a picture nobody has seen is the third.

**My proposed predicate held**, and the lane looked for a reason it
should not before accepting it: the id path and the ray path resolve
against the same index, `current_for` is the one door that answers
whether that index is the picture's, and **the only difference is what
`None` means** — picture-side reads skip silently, the pick path
refuses typed. `on_screen` is hoisted so there is still exactly one
currency read per frame.

**The reachability wall is STATED, with both arms named.** No
end-to-end row is arrangeable, for two independent reasons: the pane's
`viewport_ui` is a private method over an `egui::Ui` painting through a
wgpu callback (a wall `tests/panel_display.rs` already records), and a
refused `scene_focused` over a landed index needs `MispairedIds` or
zero visible triangles out of an index `PickIndex::build` already
tessellated. The two rows that did land pin the door's third arm and
the pair of predicates the pick path composes — and **their own test
docs say the wiring is held by its call site being one line, not by a
row**. That is the honest shape: a stated impossibility with its
location, not an assertion about something adjacent.

**`held: Option<&PickIndex>` is not #2055's swappable bool pair**, and
the lane argued why rather than asserting it: `held` and `indexing`
cannot both be set, because `PickCache::sync` drops the held index in
the same step that marks a build outstanding (`pickcache.rs:324`,
*"dropped before the answer, not after it"* — which I had already
verified on a previous unit). Its cost is disclosed: the door cannot
itself check that precondition, because the scene's `(generation, δ)`
is the pane's, not `pickcache`'s.

**The create pane's all-edges button is explicitly NOT covered, said
out loud** in both the README and the PR: it is a button in a panel,
not a cursor over the picture, so the ruling's premise is not made
there. Declining to widen, in writing, beats widening quietly.

Item **closed**. **VIEW stands at 73 open / 98 closed, nothing waiting
on Ev.**

### A refinement of the split-span trap, from my own verification

Collapsing newlines is **not enough** when the wrapped continuation
carries a comment marker. Verifying the module-doc quote above,
`tr '\n' ' '` still returned nothing, because the joined text reads
`typed refusal a pick //! stream earns…` — the `//!` prefix lands in
the middle of the sentence. What worked was collapsing newlines AND
squeezing spaces, then grepping a short fragment rather than the whole
quoted phrase. Recorded in `plan.md`: **a rustdoc or module-doc
sentence that wraps is broken by its own comment markers, not only by
the newline**, so the standard newline-collapse still produces a false
negative.

## 2026-09-15 — #2665 merged; the boundary became a mark no notice can contain, and the type holds it

**#2665 merged** (`bf79ece0ea`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, six skipped, nothing
failed or neutral.

**`NOTICE_SEPARATOR` is now `" • "`** between notices, with the old
`"; "` surviving as `LIST_SEPARATOR` for the level in (a `Withdrawal`'s
causes, `startup_notices`). No inner rendering changed and no notice's
words changed. **This is a visible chrome change** made without an Ev
ruling, on the provenance rule: the lane ran
`git log -S'joined into rank'` on the README sentence it edited and
found one commit, `d01b009171`, a VIEW unit's own maintenance — not a
ratification — and `GUI-DESIGN.md` mentions neither *status* nor
*notice* at all. Flagged to Ev as visible rather than buried.

**The type holds it, in two halves, and I checked both.** `Message`'s
fields are private (`frame.rs:344-347`), so `Message::new` — the only
public door — is on every notice text and rewrites any boundary mark to
the within-a-notice one; and `Message::joined` (`:384`) is
module-private and takes **`&[Message]`, not strings**, so the only way
to a boundary mark is to have had two notices. `line.split(
NOTICE_SEPARATOR)` is therefore exactly the notices that went in. That
is the shape `plan.md` prefers over an assertion, and it is why the
notice-producer sweep needs no blind-spot caveat: private fields make
the type the census.

**The second half was found by the tests, not by the lane** — its first
shape had `frame_status` build the line through `Message::new`, so the
door stripped the separators it had just inserted, and three rows went
red including the suite's existing two-notice row. Reported against
itself, which is the behaviour this program wants.

**Three corrections to the item, one of them a shape I have not seen
before:**

1. **`render_causes` does not exist — and it was RIGHT when the item was
   written.** I checked: `git log -S'render_causes'` returns exactly two
   commits, `6877a40ff1` creating it (2026-09-04) and `4db112ada0`
   deleting it (**2026-09-05, the day the item was opened**). So this is
   not the usual stale-by-a-merge; the citation was true and died within
   hours. `stale-file-citations-after-the-split` had left it *as
   written* because a successor "is a guess" — it is nameable now, and
   the lane reported that rather than editing another row.
2. **The em-dash is not the same shape as the separator**, and the item
   reads as if it were. No join anywhere writes one; it is in-sentence
   punctuation and was never a boundary a reader could mistake for the
   outer one.
3. **The hazard was never confined to `NonRigidFrame`.**
   `BlendEvent`, `SeatEvent` and `MateToolEvent` all carry a `"; "` in
   ordinary prose and all reach the line through `ToolKind::says` →
   `tool_news`.

**The sweep's decisive pattern was not the obvious one.** `\.join\(`
missed the site that mattered most — `Display for Withdrawal` joins with
a `for` loop and `f.write_str` — and it was found only through the
constant. The literal-`"; "` pass is what then found the real width. A
verb is no more a pattern than a name is.

**Not a refusing door, and the reason is reachability:** a door that
refused a notice containing the mark is reachable from the keyboard,
because `delta_not_a_number` echoes the δ field, so a pasted bullet
would crash the app. Pinned as a row.

**Filed**: `withdrawal-causes-join-on-a-mark-a-fault-may-contain` — the
inner level has no hold. Not wrong today, and the lane verified WHY
rather than trusting the existing comment: `prune` fills every
`Withdrawn.cause` from `free_move_check`/`display_check`, whose
`# Errors` name four faults and not `NonRigidFrame`. But `Withdrawn.
cause` is the whole `DisplayFault`, so a sixth variant re-opens it with
nothing going red.

Item **closed**. **VIEW stands at 73 open / 99 closed.**

## 2026-09-15 — #2666 merged; the coalescing machine is written once, and a dated design census survived a 705-line rewrite

**#2666 merged** (`707388d942`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, six skipped, nothing
failed or neutral.

**The item was stale in its central number and said so itself** — it
ended *"catching it one unit earlier is the only reason to file it now
rather than after the third seam"*, and the third seam landed at #2606.
Three traits, three `Inline*`, three `Thread*`, three `dispatch`
bodies, six `busy` bodies: the machine was written **six** times, not
four, and the header's *"Both implementations do this, by the same
mechanism"* was wrong about the number AND about there being one
mechanism. The test half was stale the same way and worse than my
dispatch said: `tests/eval_seam.rs` carries **six** copies of the
10 000 × 1 ms harness in three shapes, not two, with four more in three
other files.

**The fork was decided, and the argument for NOT splitting is the
better half.** Taken: a generic `Coalescing<J>` plus a private `Job`
trait carrying the one rule that differs, `supersedes`. Not taken: the
split — because it removes no copies (the item's own finding), the
header's disjointness is a *symptom* of three restatements of one
shape, and `README.md`'s *The seam modules are a chain* makes
one-file-owns-every-thread the property that won the current module
shape, which makes a split a design change rather than a tidy-up.

**`supersedes` is where a seam's notion of "the same picture" now
lives, once**, and each of the three answers carries its reason at the
impl. I read them. `EvalJob` is **always** true and the argument is
grounded in the type — `EvalJob` carries a `CancelToken`, so a job only
waits because a submit put it there, and that same submit canceled the
run the answer in hand describes. The other two compare keys,
`(generation, δ)` and `(generation, requested)`.

**#2637's caution was checked and held**: no second record of seam
state. `app.rs`, `session.rs` and `pickcache.rs` are **not in the
diff** — I confirmed from the diff stat, not the report — so every
consumer still asks the seam through `busy()`/`poll()`.

**The dated census in `GUI-DESIGN.md` survived, deliberately.** That
page carries a COMPLETE wasm-doc-link enumeration read 2026-09-14 —
`evalseam.rs`: `ThreadEvaluator` ×2, `ThreadIndexer` ×1 — and the lane
wrote the header rewrite to keep it exactly true. **My first check of
it was a proxy and nearly produced a false alarm**: a raw grep for
`` [`ThreadEvaluator`] `` over the file returns 4, not 2. The census
counts only what the WASM rustdoc pass sees, i.e. links from OUTSIDE
the `cfg(not(target_family = "wasm"))` module — the page says so in as
many words, *"its own links sit inside the `cfg(not(wasm))` module,
which the browser pass does not render at all"*. Counting the real
population (everything above the `mod threaded` boundary at `:673`)
gives `ThreadEvaluator` ×2 at lines 19 and 53, `ThreadIndexer` ×1 at
line 82, `ThreadFitter` ×0 — the census verbatim. **Reading what a
census is a census OF, before counting, is what stopped me reporting a
discrepancy that was my instrument's.**

**Three stale counts fixed as record**, all pre-existing: `Worker`'s
doc said *"this module's two workers"* over a three-variant enum;
`SpawnError`'s said a reader would be sent to *"the wrong half"* of a
three-way module; and the header claimed *"Every test in this crate
drives the inline one"* when six rows in `eval_seam.rs` alone drive the
threaded one.

**One ordering difference disclosed rather than buried**:
`ThreadEvaluator::drop` now drops its waiting job before the join
rather than after, because `close()` is shared.

**Evidence added to an open row rather than a number changed.** The
multi-field-write sweep row's 23 was left alone — that row owns it —
and a third reading was added with its instrument stated: **28 on
`origin/main`, 21 on this head**, all of the −7 in `evalseam`. Three
instruments disagreeing by up to five is itself the answer to that
row's open question: the stated rule does not determine a number.

**Two citations re-pointed by subject**, both broken by this diff:
`the-picture-key-never-became-a-type`'s fifth site →
`<IndexRequest as Job>::supersedes`, with a note that
`<FitRequest as Job>::supersedes` now sits one impl away spelled
identically while being a different key; and
`a-dead-seam-worker-reads-as-an-ordinary-idle-state`'s
`ThreadEvaluator::dispatch` → `Coalescing::dispatch`.

**Filed**: `threaded-seam-wait-loops-are-hand-copied-across-four-test-files`.

Item **closed**. **VIEW stands at 73 open / 100 closed.**

## 2026-09-15 — `view/picture-key`: the key becomes a type

`(Generation, DisplayTolerance)` is now `pickindex::PictureKey`: private
fields, `PictureKey::of` the only door, `PartialEq` over the pair. The
five spellings the item named were five; the compiler found **fifteen
files and 54 type errors**, because `ViewerApp::scene_key` and
`pane::viewport::drawn_index` carry the same key and the item did not
name them, and because ten test files build indexes through
`PickIndex::build`. A grep over the tuple type found five sites and a
grep over the positional pair found three; neither can see the other's,
which is the argument for the type rather than a sweep.

The cache's two fields are one. `PickCache::outstanding` was always
`None` or exactly `attempted` — four writes in the file, read one by
one — so the pair is `Attempt::Asked(key)` / `Attempt::Answered(key)`,
one value with a state. A cache waiting on a picture other than the one
attempted is unrepresentable rather than merely absent, and `land` moves
a state where it used to clear a second field.

Both landmines held. `<FitRequest as Job>::supersedes` still compares
`(generation, requested)`, now with the argument at the impl for why it
is not this key; `frame.rs` was not opened. `crates/viewer/README.md`'s
*A pick id is one index's word* re-states by symbol name;
`GUI-DESIGN.md` is untouched, its `(generation, δ)` sentence still true
and its wasm doc-link census re-taken above the module boundary at
2 / 1 / 3, unchanged.

## Announced seam from WIRE (2026-09-15)

WIRE's `nobodyroots-classification-has-two-homes` gave the
empty-document reading of a gather refusal ONE home:
`ProductErrorKind::means_no_body` in
`crates/editor-core/src/product.rs` (WIRE's), with the argument moved
onto it. A predicate with no caller would be the very defect this
program has an open row for
(`work/wire/frame-linear-generic-door-has-no-consumers.md`), so the
consumers that re-derived the partition now cite it. Four did; the three sites below are yours.

**`crates/viewer/src/frame.rs` and `crates/viewer/src/session.rs`
(CHROME's and VIEW's), two files, three edits.**

- `frame::product_badge`'s filter: the `ProductError::NoBodyRoots`
  alternative leaves the `matches!` and becomes
  `fault.kind().means_no_body() || matches!(…)` over the other
  three. The four declined arms are the same four.
- `product_badge`'s doc, the first "arms that stay silent" paragraph:
  the *"EMPTY, not malformed / a fresh document is in that state / one
  whose last feature was just deleted"* argument becomes a citation of
  `pncad::document::ProductErrorKind::means_no_body`, **worked examples
  included** — the paragraph now says only what is the chrome's: the
  blank viewport is already the picture of this state, so a badge here
  would make an ordinary state look like a failure. The examples are
  MOVED, not copied; leaving them on both sides is the defect this unit
  closes, one size smaller. **The second paragraph is untouched**: the
  three per-node arms are declined because the Features pane already
  badges them with a typed cause, which is not the same reason and is
  not WIRE's to move.
- `DocSession`'s landing: `matches!(fault, ProductError::NoBodyRoots)`
  becomes `fault.kind().means_no_body()`, and the comment above it
  cites the predicate instead of restating *"has no product and no
  failure either"*.

**No signature moved.** `product_badge`, `run_checks`,
`DocSession`'s landing and `checks_report` keep their signatures,
their arms and their behaviour — `means_no_body` is true of exactly
`NoBodyRoots` and of nothing else, which this lane pins as a census
test over `product::tests::every_arm`. The doc/comment edits replace a
re-argument of the shared classification with a citation of it and
leave every site-specific sentence standing (the viewer's three
per-node arms stay the viewer's chrome policy, argued where they are).

Filed while sweeping, on FIX's slate:
`work/fix/subject-refused-accepts-the-one-refusal-that-must-not-go-through-it.md`
— `Subject::refused` is public and takes the one arm that must not
reach `Subject::Unavailable`.

Signed (WIRE implementer lane `wire-n1`, PR #2629).

## 2026-09-15 — #2670 and #2672 merged; a key became a type and G1's three rules are held once

**#2670 merged** (`8088012a11`) and **#2672 merged** (`7ea4ae319c`'s
parent), each verified from the job list on the head that actually
landed: code tier, **39 check runs, 12 `test (…)`, 5
`k-lint (gate, …)`, `gate ok` success**, six skipped, nothing failed.
Both needed main merged in first; #2670's conflict was `log.md` only
(another program had appended an announced-seam entry) and #2672's
merged clean, including the `README.md` collision I had expected.

### #2670 — the picture key

**The item's population was five sites; there are six.** The sixth is
`ViewerApp::scene_key` (`app.rs:317`), written from
`(index.generation(), index.delta())` — the same key, in `app.rs` and
`pane/viewport.rs`, which the item does not list. I could confirm this
one independently, having read that field myself when verifying #2615.

**The two-field invariant is TRUE**, verified write by write rather than
taken on report: `outstanding` is only ever set beside `attempted`
(`sync`), cleared beside it (`forget`), or cleared alone (`land`). It
really was a boolean wearing the key's clothes. It is now
`attempt: Option<Attempt>` with `Asked`/`Answered`, so the diverged
state is **unrepresentable rather than merely absent**.

**The compiler census is the receipt**: changing the type gave 54 errors
across 15 files, where the tuple-type grep finds 5 and the positional
grep finds 3 and neither sees the other's. And the lane stated what
remains writable — `a.key().generation() == b.key().generation()`, via
the two accessors the build and the id query genuinely need — instead of
claiming an impossibility it had not achieved.

**Both landmines avoided**: `FitRequest::supersedes` still compares its
own pair, `frame.rs` is not in the diff.

**The three-form doc-gate earned itself again**: the BARE pass caught a
real red first — two `[PictureKey]` links inside `mod threaded` were
unresolved for want of an import there, and **clippy is blind to that
lint**.

### #2672 — G1's three rules

**The sequencing caveat was answered, not inherited.** The item said
sequencing after `no-persistent-setplacement-session-op` was *"probably
right"* because DI5 moves the free-move commit onto the document. The
lane checked both documents and the tree: DI5 changes what the commit
LANDS, and none of the three rules is stated in terms of the landing.
What DI5 brings closer is the two machines' value kinds and side
effects — **the sharing the item itself rules out**. So the caveat lands
on the shape already refused, not on this one. Holding the rules first
is also the cheaper order: `g1::Slot::commit` hands the caller back the
value it took, which is the shape DI5's session-side edit needs.

**The item's live evidence was historical, and the lane said so.**
*"`CancelGesture` and `CancelFreeMove` both have zero emitters"* was
true when written; `gesture-drags-have-no-cancel-door` closed
2026-09-11 (branch `view/cancel-doors`) and I confirmed both have doors.
Re-derived: the two copies **agree on all three rules today**. So this
closes a hazard, not a present divergence — which is why **no test can
red on the base tree**, stated plainly rather than worked around.

**The receipts measure the property instead**, which is the right move
when the defect is structural: M1 and M2 red BOTH integration rows, and
**M3 is the measured LIMIT** — a caller-side `names` closure mutated to
`|_| true` reds the value drag and leaves the probe green, because the
closures stay the caller's. The lane wrote that limit into the module
and the README rather than letting the claim read wider than it is.

**Both reconciling prose sentences were checked against their own path
before rewriting**, and they differed: `display.rs`'s was an
attribution true of its own path; `session.rs`'s `CancelGesture` comment
was the borrowed-mechanism shape — it analogised a *commit* rule to
justify a *re-submit* decision — and now reads the answer off
`g1::Slot::cancel` with a `debug_assert_eq!` against the scratch.

**One asymmetry filed rather than absorbed**:
`the-value-drags-in-flight-refusal-has-two-spellings` — rule 1 is now
answered for the value drag by both `perform`'s table and the slot, and
moving it down touches the mid-gesture policy, its hand-written
`expected` table, and an ordering consequence against `guard_driven`.
Both answers written out.

### A harness fact

The `merge_pull_request` call for #2672 returned a **Cloudflare 502 on
the response**. The merge had gone through — the PR reads `merged: true`
at 18:38:12. **Read the PR before retrying a failed merge call**: a 502
is a statement about the response, not about the action.

**VIEW stands at 75 open / 102 closed.**

## 2026-09-15, `view/withdrawal-causes` — the INNER join gets a type

Closes `withdrawal-causes-join-on-a-mark-a-fault-may-contain`, filed
by #2665 when it fixed the outer level. `Display for Withdrawal` joins
a withdrawal's causes with `LIST_SEPARATOR`, flat, and
`DisplayFault::NonRigidFrame` writes one inside a single sentence. It
was unambiguous only because neither admission test happens to raise
that arm — a property of two functions' error sets, with no type
carrying it.

**The fix is the item's own first option: `Withdrawn.cause` narrows.**
`display::AdmissionFault` holds the four faults the admission tests
answer, MOVED out of `DisplayFault` rather than copied, and
`DisplayFault` gains `Admission(AdmissionFault)`. The three check
functions answer the narrow type; every door still answers
`DisplayFault` through `From` at the `?`; ~30 call sites moved under
the compiler.

**Why not the other two.** The mark treatment (`Message::new` /
`Message::joined` one level in) needs a mark that is never legitimate
in-band, and one level in from the bullet there is none left — every
candidate is punctuation a sentence is entitled to, and a rewriting
door would show a reader words its author did not write, which is the
objection `frame_status`'s own doc already makes to escaping at a
join. Re-wording `NonRigidFrame` is a claim about one arm, and it has
a cost nobody had priced: #2665's
`a_joined_line_splits_back_into_the_notices_it_was_made_from` is built
on that sentence as the one REAL fault text carrying the mark, and
guards itself with an assert that reds if it stops carrying one. That
row would have had to fall back to prose written for the row, which is
what its doc says it refuses to do.

**Receipts.** Reverting the field to `DisplayFault` and letting the
census range over the type it then has: the split returns **nine
pieces for eight causes**, `NonRigidFrame`'s single sentence cut in
two. Giving `MateConstrained` a semicolon on the fixed tree: both new
rows red. Widening `free_move_check` to answer `NonRigidFrame`: E0308.

**Two findings filed from the sweep**, both on this slate.
`startup-notices-join-on-a-mark-a-prefs-notice-contains` is the same
class at `LIST_SEPARATOR`'s other consumer and is **live, not latent**
— three `prefs::Notice` arms carry a `"; "` and two startup notices
need only a prefs file naming an unknown theme and an unknown preset.
`seat-line-spells-the-list-mark-as-a-literal` is the mark's second
spelling.

**Also corrected, both stale before this branch:** `display.rs` and
`frame.rs` each said *"the remaining three"* `DisplayFault` arms name
no id, written 2026-09-05 when there were seven arms and falsified on
2026-09-11 by `WrongFreeMove`; the split states it structurally
instead. `crates/viewer/README.md` said `rank`'s `Display(_)` is a
catch-all, which #2053's fix pass made false on 2026-09-06 —
`git log -S` finds an agent commit and no ratification.

Signed (VIEW implementer lane `withdrawal-causes`, PR #2693).

## 2026-09-15 — `view/style-installs`: startup's two context-wide installs are held, and the premise was measured before it was built on

`work/view/nothing-holds-startups-two-context-wide-style-installs.md`.
The premise was checked the only way it can be — on a committed tree,
each call line deleted in turn, the whole viewer suite run. Both times
the suite was **unchanged** (`--lib` 88 passed / 1 failed, the standing
`gpu::tests::every_pass_builds_on_a_real_device` adapter red;
`--test all` 562 passed / 0 failed / 1 ignored), so no existing row
covered either install. **Both installs are on the `assemble` side**,
so both rows are ordinary `--lib` tests and no gate was needed:
`egui_ctx` occurs four times in `app.rs` — `new`'s call, `assemble`'s
parameter, and the two installs — which is also why the population is
two rather than a claim that it is.

One qualification the item does not make: deleting
`install_number_formatter`'s call leaves the function with no
non-test caller, so the lib build warns `never used`. That is not a
row and says nothing about which context the install reaches or when,
but a `-D warnings` build would notice the deletion. Deleting
`apply_polarity`'s call is silent — it has a second caller in the
palette picker.

Two rows rather than one, because a single row cannot say which
install went. Both read behaviourally: a `NumberFormatter`'s
`PartialEq` is `Arc::ptr_eq`, so the formatter is read by spelling
40 nm through the context's own styles and comparing the text with
`widgets::number_text`'s, with a third assertion holding the witness
apart from the toolkit's default so the row reports when it stops
being able to see the install. The polarity row reads BOTH the
preference the context states and the `dark_mode` a first frame would
paint, because `egui`'s `fallback_theme` is `Theme::Dark` and
`Theme::DEFAULT` is a dark palette: the `dark_mode` read alone is
green over a context nobody touched, which is the flattering reading
this item warns about, one level in.

Mutation receipts, on the committed tree with the rows in it: deleting
`apply_polarity(egui_ctx, theme.polarity)` reds
`startup_states_the_resolved_polarity_on_the_context` and nothing else
(`System` where `Dark` was wanted); deleting
`crate::widgets::install_number_formatter(egui_ctx)` reds
`startup_installs_the_number_rule_onto_both_of_the_contexts_styles`
and nothing else (`"0.000"` where `"0.00004"` was wanted). `--test
all` stayed 562/0/1 under both.

The sweep and its blind spot are in the PR body. Nothing in
`GUI-DESIGN.md` moved: G5 decides what a theme IS and says nothing
about when it reaches the chrome, so this unit touches no clause it
decides.

Signed (VIEW implementer lane `view/style-installs`).

## 2026-09-15 — #2692 merged; the item's own suggested assertion would have been a false green

**#2692 merged** (`af6d7c76a2`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, **six** skipped, nothing
failed. (The lane's report says seven skipped; the job list says six —
`step import (freecad)`, `corrupt input (release profile)`, the two
interval rows and the two cache primers. 33 + 6 = 39. Immaterial to the
verdict, recorded because a count in a report is a claim like any
other.)

**The premise was measured, not assumed.** Each install's call line
deleted in turn on a committed tree left the whole viewer suite
identical — `--lib` 88/1, `--test all` 562/0/1 in all three states. So
no existing row covered either install, exactly as the item said.

**One qualification the item does not make**, and the lane volunteered
it: deleting the formatter's call leaves `install_number_formatter`
with no non-test caller, so the lib build warns *never used* — silent
about which context or when, but a `-D warnings` build would notice
that one mutation. Deleting `apply_polarity`'s call is completely
silent, because the palette picker is a second caller. The two gaps
were not equally invisible.

**The best finding is that the item's own suggested read would have
passed under mutation.** The item proposes *"read the polarity back off
the applied visuals"*. I checked egui 0.36.1 myself: `fallback_theme`
is `Theme::Dark` (`memory/mod.rs:331`), `ThemePreference::System`
resolves to `system_theme.unwrap_or(fallback_theme)` (`:367`), and
`System` is the default (`:205`). So an untouched context already reads
dark, and a naive visuals row would have been **green with the install
deleted** — a row that looks like it covers the install while covering
nothing. The landed row reads the **preference** the context states as
well as the visuals, and it is the preference read that fails when
nobody applies anything.

That is this item's own finding — *a row covers the RULE an install
carries, not that the install happened* — turned on the row the unit
was writing. Worth saying plainly: the brief warned about it in the
abstract, and the lane found the concrete instance in the item's own
suggestion.

**The number row is behavioural because it has to be**: `NumberFormatter`'s
`PartialEq` is `Arc::ptr_eq`, so comparing function values is not the
reading anyone wants. It spells 40 nm in mm through **both** of the
context's styles — not the active one — so it reads nothing the
polarity install decides, and a third assertion holds the witness apart
from `emath::format_with_decimals_in_range` so the row reports when it
stops being able to see the install.

**Both installs are on the `assemble` side** and I confirmed it:
`egui_ctx` occurs exactly four times in `app.rs` — the call at `:657`,
the parameter at `:685`, and the two installs at `:723` and `:729`.
Neither is in the device half, so no gate was needed and the old
blocker really is gone.

**The sweep's argument is compiler-grade rather than a grep**: the
context reaches startup as one parameter and every use of it is one of
the two calls. Blind spots stated: effects inside callees that do not
receive the context are named as calls rather than as effects (partly
closed by grepping four spellings), and anything `eframe` does to the
context before `new` is outside the crate.

**G5 reported, not edited** — the lane ran `git log -S` on its sentence
and found `e9824abf3b`, so it is genuinely ratified; it decides what a
theme IS and says nothing about when it reaches the chrome.

Item **closed**. **VIEW stands at 77 open / 103 closed.**

## 2026-09-15 — #2693 merged; the narrowing closed the population, and found a LIVE instance of the same class next door

**#2693 merged** (`c659225121`), verified from the job list on the
merged head `22a3b92fcd`: code tier, **39 check runs, 12 `test (…)`,
5 `k-lint (gate, …)`, `gate ok` success**, six skipped, nothing failed.
It conflicted on `log.md` against the style-installs entry;
`crates/viewer/README.md` **auto-merged** despite both lanes editing it.

**The item's latency claim is true, and it was checked by reading
BODIES rather than `# Errors` prose** — which is the trap I named in the
dispatch, because a doc section is prose about a function and not a
census of it. `prune` is the only producer of a `Withdrawn` in `src/`;
`free_move_check` is `display_check?` plus `MateConstrained`;
`drawn_targets` calls only infallible things; and `NonRigidFrame` has
**exactly one construction site in the workspace**, returned to its
caller. Latent, not live.

**Fix: option 1, the narrowing** — `display::AdmissionFault` carries the
four admission faults, MOVED rather than copied so no sentence exists
twice, with `DisplayFault::Admission(_)` beside its own four. The three
checks answer the narrow type; every door still answers `DisplayFault`
through `From` at the `?`, so refusal payloads are unchanged.

**The lane said plainly what the fix does NOT buy**, and that is the
part worth keeping: the type carries the **population**, not the claim
about strings. The old population was decided in two other functions
and could change unnoticed; a fifth arm now cannot join without going
through a census.

**`refusal-rank-wildcards-the-display-fault-payload` is CLOSED** (#2053,
2026-09-06) — the item calls it "nearby" and does not say so; I
confirmed the frontmatter. The useful half is not the correction: the
narrowing **did** collide with it, because a naive `Admission(_)` arm in
`Refusal::rank` would have re-created that item's own defect one level
down. `rank` now walks the admission family arm by arm, so a fifth
admission fault reds there too.

**A LIVE instance of the same class, found on the way out**:
`startup-notices-join-on-a-mark-a-prefs-notice-contains` — `frame::
startup_notices` joins with `LIST_SEPARATOR` and three of four
`prefs::Notice` arms write a `"; "` mid-sentence; a prefs file naming
an unknown theme AND an unknown preset produces two such notices with
no error path. **Not latent.** Narrowing cannot reach it — that join
takes `&[String]` from three types by choice — so the fix lands in
`prefs.rs`/`app.rs`. Also filed:
`seat-line-spells-the-list-mark-as-a-literal`.

**Two stale counts corrected as record**, both pre-existing and both
found rather than inherited: `display.rs` and `frame.rs` each said *"the
remaining/other **three**"* `DisplayFault` arms name no id — written
2026-09-05 at seven arms and falsified by `WrongFreeMove` on
2026-09-11. There are four, and neither sentence carries a number now.
And the README's claim that `rank`'s `Display(_)` is a catch-all was
dated by `git log -S` to an agent commit with **no ratification** and
falsified by #2053 — corrected rather than left for Ev.

**The M1 receipt is the nicest of the three**: reverting the field to
`DisplayFault` and letting the census range over the type it then has
made the split return **nine pieces for eight causes** —
`NonRigidFrame`'s sentence cut in two, which is the defect itself
rendered as a number.

Item **closed**. **VIEW stands at 78 open / 104 closed.**

## 2026-09-16 — `view/scientific-arm`: the render's last resort is truth

`readout::number`'s scientific fallback was the one arm not held to the
module's own rule, and it rounds. It now falls through to the exact
spelling exactly where the four-figure one does not read back — which
measurement says is only the band `[1.7975000000000001e308, f64::MAX]`,
about 9.7·10¹¹ `f64` values each sign. Over `1.0e-320`..`1.0e300`
stepped by 1.05, **no render changes**.

`the-scientific-arm-rounds-out-of-the-type` **closed**. Its three
producer claims were measured and all three fail as bounds: every
producer's guard is `is_finite()` and every one then multiplies toward
the top of the type (δ by `1.0e3`, a camera distance by `1.0e5`, a
probed bound by up to `1.0e3`), so the headroom is three to five decades
rather than the three hundred the carve-out claimed. A δ in
`[1.7975e305, 1.7976931348623156e305]` is accepted by
`DisplayTolerance::new` and lands *inside* the band.

Not an Ev question: `git log -S` on every sentence of the carve-out and
of `MAX_CHARS`'s bound returns four agent commits of 2026-09-12, and
`docs/DESIGN.md`, `crates/viewer/GUI-DESIGN.md` and
`crates/viewer/README.md` name `readout` nowhere at all.

Two residues filed, both on this slate:
`render-mm-overflows-to-inf-for-a-delta-the-door-accepts` (the δ door's
millimetre product overflows for a δ the door accepts, and
`no_delta_renders_as_a_number_a_delta_cannot_be`'s sweep stops at a
kilometre so it cannot see it) and
`the-fields-door-has-no-width-bound-at-all` (`number_text` returns the
widget's own spelling at any width — 311 characters at the top of the
type, and a twelve-character text that an existing row already asserts).

`the_field_shows_the_longest_render` renamed to
`the_field_shows_every_render_the_bound_covers`: it measures the bound,
not the render's worst case, and those stopped being the same thing.
Its two citations outside `log.md` were repointed.

Signed (VIEW implementer lane `view/scientific-arm`).

## 2026-09-16 — `view/startup-notices`: the third consumer was the second level misread

`startup-notices-join-on-a-mark-a-prefs-notice-contains` closed (#2710). Unlike
its two siblings the defect was live, so the red is on the base tree
rather than a post-hoc mutation: a preferences file naming a theme and
an input preset the registries do not hold renders
``preferences: no theme called `aurora`; using `dark-neutral`;
preferences: no input preset called `modal`; using the default`` — four
`"; "`-delimited pieces for two notices, no error path involved.

None of the item's three candidate fixes was taken and the reason is
one fact: two of `prefs::Notice`'s four arms echo a TOML key out of the
user's own file, so there is no mark out of band here and no type that
bounds the sentences. `withdrawal-causes-…` rejected its own option 2
because one level in there is no mark that is never legitimate in band;
the objection is stronger at this level, not weaker.

What was wrong was the classification. A `Withdrawal`'s causes are the
items a counted preamble introduces; the startup notices have no
preamble and nothing counts them, so they are several notices and take
the boundary mark. `startup_notices` now builds one `Message` per
element and joins with `Message::joined`, which puts the startup line
under the hold `frame_status`'s line already has.

The door's own doc claimed three types and named the wrong three:
`resolve_theme` and `resolve_keys` return `Option<Notice>`, the same
type `from_toml` yields; the unnamed third is `prefs::StoreError`.
Corrected at the door and in the README, whose *"NOT held this way"*
paragraph this change falsified and replaces.

Two findings filed outside the fence, both the same class:
`work/exch/step-import-joins-rendered-refusals-on-a-mark-they-may-contain`
and
`work/props/stackup-report-joins-rendered-blockers-on-a-mark-they-may-contain`.
`seat-line-spells-the-list-mark-as-a-literal` is NOT subsumed and stays
open: its argument is about a constant spelled twice, not about a mark
an element may carry.

Signed (VIEW implementer lane `view/startup-notices`).

## 2026-09-16 — #2710 merged; the fix was none of the three, because the item's CLASSIFICATION was the error

**#2710 merged** (`d90436b6ee`), verified from the job list: code tier,
**39 check runs, 12 `test (…)`, 5 `k-lint (gate, …)`, `gate ok`
success**, all four render-lane rows success, six skipped, nothing
failed.

**The first of this separator family that is LIVE, and it was
reproduced**: a prefs file naming an unknown theme and an unknown preset
renders ``preferences: no theme called `aurora`; using `dark-neutral`;
preferences: no input preset called `modal`; using the default`` — four
`"; "`-delimited pieces for two notices, no error path involved. **Red
taken on the BASE tree** at `4d3ff671c0`, which none of the five units
before it could do.

**The fix is none of the three the item named, and the reason is that
the item mis-CLASSIFIED the level.** A `Withdrawal`'s causes are the
items a counted preamble introduces; the startup notices have no
preamble and nothing counts them. They are **several notices**, so they
take the boundary mark and inherit the hold the outer level already
has — `startup_notices` now builds one `Message` per element and joins
with `Message::joined`. No new mechanism, no wording changed.

**One fact kills all three candidates, and I verified it**: two of
`prefs::Notice`'s four arms echo a TOML key straight out of the user's
file — `UnknownKey(String)`, *"Carries the dotted path"*, and
`WrongType { key: String }`, *"The dotted path of the offending key"* —
and a quoted TOML key may hold any character. **No type bounds that
payload**, so the type pin cannot make the claim, and the two-half
treatment has no mark that is never legitimate in band.

**#2693's in-band objection applies here and is STRONGER, not weaker**,
which is the observation worth keeping: there the in-band marks were
punctuation four *authored* sentences were entitled to; here the payload
is arbitrary user text, so every mark is in band. And a rewriting door
has nowhere to demote to — `Message::new` can rewrite a bullet to a
semicolon because there is a level below; at the bottom there is not.

**A NEW failure mode of the suggested-assertion rule.** Yesterday's rule
says an item's suggested read fails most cheaply by being GREEN on the
broken tree. This one fails the other way: counting `LIST_SEPARATOR`
pieces gives **4 on base and 3 on the fixed tree** for two notices — red
on BOTH, because the fix moves the boundary rather than removing the
semicolons. It would have sent a lane after the wrong fix. Measured,
not reasoned. `plan.md` extended.

**Two corrections to the item, one of them to a door's own doc**: the
door says the notices come *"from three sources with three types
(`prefs::Notice`, `prefs::PrefsError`, and the theme and preset
resolutions)"* — the count is right, the **membership is wrong**, since
`resolve_theme`/`resolve_keys` return `Option<Notice>`, the same type.
The unnamed third is `prefs::StoreError`. And a measured property the
fix deliberately does **not** lean on: a list of more than one element
is always all-`Notice` today, because both `Err` arms produce a
singleton — a property of two arms that nothing carries, which is
exactly what #2693 refused to rest on.

**The sweep widened its own scope rather than inheriting one.** #2693
swept `crates/viewer/src`; this one swept the literal across **every
cargo root**, and filed two out-of-fence rows —
`work/exch/step-import-joins-rendered-refusals-on-a-mark-they-may-contain`
(one of whose sites is the `for`-loop + `write!(f, "; ")` shape a
`.join(` sweep cannot see) and
`work/props/stackup-report-joins-rendered-blockers-on-a-mark-they-may-contain`.
New stated blind spot worth keeping: **`Debug` on a collection joins on
`", "` and is invisible to every pattern** — and it is live in one of
the filed rows.

**`seat-line-…` is NOT subsumed**, said explicitly rather than quietly
folded: its argument is a constant spelled twice, not a mark an element
may carry. This change shrinks the drift population from three sites to
two and the finding stands.

**Neither README clause it falsified was ratified** — `git log -S` on
both returns #2693's and #2665's own lanes, no commit from Ev — so it
proceeded and said where it looked, which is `CLAUDE.md`'s rule working
as intended for the third time this week.

Item **closed**. **VIEW stands at 77 open / 105 closed.**

## 2026-09-16 — #2713 merged; the carve-out was not safe, and all three of its supporting claims fail

**#2713 merged** (`136a658bff`), verified from the job list on the
merged head `cf72021cb0`: code tier, **39 check runs, 12 `test (…)`,
5 `k-lint (gate, …)`, `gate ok` success**, all four render-lane rows
success, six skipped, nothing failed. Conflicted on `log.md` against the
startup-notices entry; `plan.md` auto-merged.

**The unit was framed as a design question and it was not one.** The
item described the rounding fallback as a deliberate, argued carve-out
with a test pinning it, and asked whether a render owes a width bound
at all. Both of the reasons to leave it alone turned out to be false.

**All three producer claims fail, measured.** The carve-out rested on
*"no length this chrome shows is within three hundred decades of it"*.
Every producer guards with `is_finite()` and then multiplies **up**;
real headroom is three to five decades.

- **δ is false outright, and I confirmed the code myself**:
  `DisplayTolerance::new` is `delta.is_finite() && delta > 0.0` with
  **no upper bound** (`scene.rs:76`). Every δ in
  `[1.7975e305, 1.7976931348623156e305]` survives `* 1.0e3` and lands
  inside the failing band; above it the product is `inf` and
  `render_mm` returns `"inf"` **for a finite δ** — the same rule failing
  earlier and harder. Filed as its own row.
- **Camera**: `Camera::new` checks finite and `>= f64::MIN_POSITIVE`,
  no upper bound, and `mm(max_distance)` is `scene_radius * 1.0e5`.
- **Probe**: the three `f64` `number_field` sites carry **no
  `.range()`** — only the integer pattern count does — so the origin is
  whatever a user typed.

**And the row pinning the exception had a false NAME.** Bisecting on
*does `{:.3e}` read back* gives the band
`[1.7975000000000001e308, f64::MAX]` — **9.68·10¹¹ `f64` values per
sign**, not one. The row's comment *"an exception rather than a
region"* was false of the tree when it was written. A test can pin the
wrong shape of a defect and still pass.

**The Ev question resolved by the rule rather than by deference.**
`git log -S` over five sentences returns `readout.rs`'s **entire
history: four commits, all Claude, all 2026-09-12** — I confirmed the
log — and `docs/DESIGN.md`, `GUI-DESIGN.md` and `README.md` name
`readout`, `MAX_CHARS` and the read-back rule **nowhere at all**. So
nothing was ratified and nothing waited. Third time this week that
`CLAUDE.md`'s *check that Ev ever agreed before you wait for Ev* turned
a would-be block into work.

**What it did NOT overturn is as careful as what it did.** The
caller-named-width rejection still holds untouched — no caller names a
width — and the lane left that paragraph alone. What the third arm
costs is `MAX_CHARS`'s *consequence* sentence, which was **already
false** of the crate's other two number renders: `props::render_number`
spells `f64::MAX` in 22 characters in a field today. And the decisive
line was already in the module: `reads_back`'s doc says *"a wide text
that names this value is not improved by a narrow one that does not"* —
which is exactly what the old fallback did.

**Surgical, and held**: over `1.0e-320`..`1.0e300` by 1.05, ~65 000
renders, **not one text changes**, with
`the_four_figure_arm_still_carries_everything_below_the_band` as the
row. Two mutation receipts, both reverted.

**The sweep's stated blind spot is the useful one**: the population is
RENDERS, not the values reaching them — so the reachability half was
done by reading each producer's validation, *"which is why the δ window
turned up and no grep would have found it."* That is the
check-the-producer rule from #2638 applied without being told.

Item **closed**, two rows filed:
`render-mm-overflows-to-inf-for-a-delta-the-door-accepts` (whose
existing row *"asserts exactly the broken property and is green because
its sweep ends at a kilometre"*) and
`the-fields-door-has-no-width-bound-at-all`.

**VIEW stands at 78 open / 106 closed.**
## 2026-09-16 — the δ door gets an upper bound (`view/render-mm-inf`)

`render-mm-overflows-to-inf-for-a-delta-the-door-accepts` closed.
`DisplayTolerance::new` accepted any finite δ > 0 and `render_mm` is
`readout::number(δ * 1.0e3)`, so a δ the door accepted rendered as
`"inf"`. The item's table was re-measured rather than believed and
holds in every row; the bisection adds the edge it did not name, which
is `f64::MAX * 1.0e-3` — the coarsest δ whose millimetre product is
finite, and also, exactly, the coarsest δ the field's `mm * 1.0e-3`
commit path can name.

Answered at the door rather than in the render, because past that edge
the millimetre value is not an `f64` and no text of it reads back as
one: answering in `render_mm` means a second carve-out at the top of
the type, which is what the previous unit had just removed a level
down. `MM_PER_METRE` is now the one factor the door and the render
both read, and
`SceneError::DisplayToleranceOverflowsMillimetres` is the typed
refusal — its own arm because the δ it names IS finite and strictly
positive and `mesh::tessellate` would take it.

The row over it owed an end regardless and has one: the sweep reaches
the top of the type, the character bound moved out of the property (the
band above `1.7975e305` is spelled exactly, at twenty-two characters),
and past the bound the row asks the door's own answer, so it reds on a
tree whose door has none. **Base-tree red on both new rows at
`bfc577bbdd`**, green with the fix — stronger than a planted mutation
and taken because the defect was live.

Sweep residue filed:
`renders-that-multiply-a-finite-guarded-length-spell-the-product-inf`
— the camera readout, `Bounds::wording` and `props::field_text`, all
three the same *finite guard, multiplication up, render of the
product* shape and none of them with a door of its own to narrow.

`readout` and `DisplayTolerance` are named in no README roster table
and in neither design doc; checked rather than assumed, so nothing was
owed there.

Signed (VIEW implementer lane `view/render-mm-inf`).

## 2026-09-16 — `view/two-spellings`: rule 1 moves down for the value drag

`the-value-drags-in-flight-refusal-has-two-spellings` closed with a
change rather than a written reason. `SessionOp::BeginGesture` and
`BeginParamGesture` are `true` in `permitted_during_value_gesture`, and
the second-begin refusal a user meets is `g1::Slot::begin`'s — the same
door the probe's begin goes through, with the value drag's words.

The site a user reached before was the table's: `perform` consults it
before dispatch, so the slot's arm was unreachable through the only
door that calls it. The alternative — keep the floor, write down which
row is policy and which is safety — was refused because the two answers
have no input on which they differ: same state, same `GestureInFlight`.
The table's own certifying sentence did not cover the rows either
(*"everything else moves the document, the history or the file"*, false
of both begins, and of `ProbeBounds`, which is now named as the one
refusal that reads rather than moves).

Both doors' target checks moved inside `DocSession::start`'s closure, so
rule 1 still answers before them and no user-visible refusal changed —
`driver_of` and `guard_driven` are free functions over the committed
document to make that borrow work. Nothing could red on the base tree
as a result, so the receipt is the other direction: deleting
`g1::Slot::begin`'s in-flight arm reds 5 tests on `origin/main` (all
probe rows) and 12 on this branch, the seven new ones all the value
drag's.

The hand-written `expected` table keeps its census — the free-move
table's property shape is not available here, because `ProbeBounds` is
a counterexample to the nearest short description of the 24 refusals.
Its calibration paragraph was re-measured on this tree rather than
carried forward.

Signed (VIEW implementer lane `view/two-spellings`).

## 2026-09-16 — #2739 merged; the slot's arm was a floor nobody stood on, and the measurement proves it

**#2739 merged** (`bb1ea62fda`), verified from the job list on the
merged head `15e316fd42`: code tier, **39 check runs, 12 `test (…)`,
5 `k-lint (gate, …)`, `gate ok` success**, six skipped, nothing failed.

**The item's control-flow claim is true and I checked it rather than
took it**: `begin_gesture` is private (`session.rs:1548`) with
`perform`'s match arm its only caller, and `perform`'s preamble at
`:1106` is `if self.gesture.held().is_some() && !op.permitted_during_value_gesture()`
— consulted before dispatch. So under an open value drag the table
refused and `g1::Slot::begin`'s arm was **unreachable through every
door a user has**.

**The decision went to the code, on three grounds, and the third is the
one that generalises**: the two answers are extensionally identical
(same state, same `Refusal::GestureInFlight`, no input distinguishes
them); the table's own certifying sentence — *"everything else moves
the document, the history or the file the drag is previewing against"*
— is **false of both begin rows**; and `session/op.rs` already argues
exactly this for `BeginFreeMove`, an argument that **survives** the
change and becomes one rule about rule 1 rather than one table's
exception.

**The receipt is a measurement, not a red, and it is the best shape
this program has produced for a structural defect.** Nothing can red on
the base tree — the spellings agree on every input and ordering is
preserved deliberately — so the lane deleted `g1::Slot::begin`'s
in-flight arm on both trees instead:

- on `origin/main`: **5 red**, all probe rows;
- on the branch: **12 red** — the same five plus seven value-drag rows.

**Breaking rule 1 in the module that holds it once left every
value-drag row green on `main`.** That is the arm being a floor nobody
stands on, rendered as a number.

**The ordering consequence was moved, not paid.** Removing the rows
naively would have made a begin on an expression-driven slot refuse
`DrivenByExpression` and on an undeclared param `NoSuchParam` — both
wrong, because rule 1 validates the target only once the slot is free.
Both target checks moved **inside** `DocSession::start`'s closure, with
`driver_of`/`guard_driven` becoming free functions over the committed
document (disjoint field borrows, no clone, no `RefCell`). **No
user-visible refusal changed anywhere**, and a new row pins both
directions.

**The `expected` table stayed a census, with the reason** — and the
reason is a proxy defect caught before it was minted. The free-move
half can be property-checked because its refusals have a name; the
value table's remaining 24 have no short description except the
table's own sentence, and **`ProbeBounds` falsifies it** (it reads the
shown document and commits nothing). A property test on that sentence
would have been a proxy with a known false member. The sentence in
`session/op.rs` was corrected instead — 23 of 24, with `ProbeBounds`
named as the exception.

**And the table's calibration paragraph was RE-MEASURED rather than
carried forward**: whole-table reversal reds **8 tests, 5 outside the
file** (the paragraph said five); the per-op witness census is **19 of
24** with its enumeration rule written beside it, the five unwitnessed
named, and an explicit statement that 19 is a floor. That is the
number-with-its-instrument rule applied to a number the lane inherited.

**The sweep's second pattern is the interesting one**: a refusal raised
through `g1::Slot` **never names its variant at the raising site** —
the word travels as a struct field — so grepping the variant cannot see
either door, *"which is how this instance stayed invisible"*. The class
is empty after this change: the only other `false` rows reach no `Slot`
door.

**All three sentences it changed were checked for ratification** and
all three trace to agent lanes (`0b8baf2b46`, `5932ca7e53`,
`d6461d9ec0`), so nothing waited. G1 untouched — it ratifies the
three-layer split and preview-vs-commit, not which layer raises a
second begin's refusal.

Item **closed**. **VIEW stands at 77 open / 107 closed.**
## 2026-09-16 — `frame.rs` split in three (lane `view/frame-split`)

`frame-module-has-eight-concerns-and-no-holds-row` is closed. The
orchestrator's rule was the module's own first sentence — *"the
per-frame policies the viewport runs, as values, so they are
replayable"* — and it removes two things: the environment probes, which
take the machine as their argument and cannot be replayed from any
value a test builds (`platform.rs`), and the id pass's query
bookkeeping, which is state carried ACROSS frames because a query and
its answer sit on different ones (`idpass.rs`). 2,996 lines became
2,583 + 257 + 254. A move: no test assertion changed, and the only test
edits are import paths.

**The membership was checked against the ruling rather than taken from
it, and five things moved that neither list names** — `Zenity`,
`SessionBus`, `PREFS_DIR`, `PREFS_FILE` and `NO_CHOOSER_BACKEND`, the
last of which the item files under concern 4 and the ruling omits. The
item's own span, `frame.rs:1671-1878`, was wrong at the merge base and
not merely stale: the probes are at `1856-2076`, so the range's start
was two hundred lines short and its end landed inside
`ChooserBackend`'s variants. Recorded in the item's closing section
with the rest.

`scripts/gates/no-ambient-env.sh`'s allowlist entry moved with the
code. `git log -S` on *"ONE file on purpose: every ambient read the
viewer performs"* over that path returns exactly one commit,
`cf2164600f`, the merge of **#1717** from `m10/m10-7-spec` — an agent
program branch, and every commit in this repo carries `evgunter`'s
signature, so authorship is not evidence either way. No ratification by
Ev turns up, and `work/README.md` says Ev does not edit files. The
entry moved and the sentence was re-worded to name `platform.rs`; what
the gate DECIDES is untouched. Territory names `scripts/gates/*` as
GUARD's, the same warning VIEW already carries for
`viewer-vocab-declared-once.sh`.

**One roster elsewhere had to register the move**, and the local viewer
suite was structurally incapable of seeing it — #2293's shape again.
`crates/pncad-py/src/prose_census.rs`'s `UNDECIDED` table names
`Disagreement`'s positional `{:?}` site by PATH, so the row said
`crates/viewer/src/frame.rs` and the file is now `idpass.rs`. Caught by
hosted CI: six `test (…, 1/2)` shards red on
`prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason`,
one test, deterministic across all six lane/eps points. The path was
corrected in place — a census that exists so a site registers itself,
which is the registration half rather than the audit half, announced
here because `prose_census.rs` is LIB's ground by territory. Receipt on
the fixed tree: `cargo nextest run --workspace --no-fail-fast`, 7,570
run, 7,570 passed, 38 skipped.

Signed (VIEW implementer lane `view/frame-split`).

## 2026-09-16 — the supersession lifetime is stated, not confessed (lane `view/supersession-lifetime`)

`a-supersession-outlives-its-own-frame` is closed on Ev's fork-1
ruling. `frame::Withdrawal`'s *Why the line and not a badge* section
said *"That is a weaker lifetime than the argument above wants"* and
pointed at this item as residue. It now states the lifetime the code
has, in three legs with the row that holds each, and says why that
lifetime is the right one — including the part the old sentence made
underivable, that `Clear` is **tighter** than a per-instance
retirement rather than looser.

**Re-derived, not inherited.** `frame::acts` is
`!matches!(op, SessionOp::Hover(_))` at `frame.rs:561-563`, not the
`537-539` the item cites: `frame.rs` was split this morning and every
number in the row predates it. `batch_status` is at `571-582` and
reads as the ruling describes.

**The "survives navigation" clause is true, and for two independent
reasons rather than the one the item gives.** No `SessionOp` variant is
a camera move — `camera::CameraOp` is its own vocabulary — so a fold
never reaches `batch_status` at all; and
the retirement a clean fold does issue is `Expire(Subject::Camera)`,
which `apply` drops only against a message whose subject matches.
Either alone is enough.

**The ruling's one wrong fact.** It says `Subject::Document` *"has no
typed `StatusUpdate::Expire` issuer where three other subjects do"*.
Two do. `SUBJECTS_WITH_AN_EXPIRY_ISSUER` is `[Subject; 2]` —
`Camera` and `Cursor` — and `Document` shares the absence with
`Display` and `Preferences`, which the roster's own paragraph states
in as many words. So `Document` is not the odd one out; it is one of
three, and the asymmetry the ruling narrows the item's framing to does
not exist in the direction it names. The conclusion is untouched: the
arm's doc is accurate and was not this unit's to change.

**No new assertion, and the argument is a mutation table rather than a
preference.** Each leg of the sentence was falsified in the source and
the suite run:

| mutation | rows that red |
|---|---|
| `apply`'s `Keep` arm clears | `a_cursor_that_has_not_moved_retires_nothing`, `deliver_sends_news_to_the_notices_and_retirements_to_the_field`, `keep_clear_and_show_are_four_different_sentences` |
| `acts` returns `true` (a hover acts) | `a_hover_only_batch_leaves_the_status_line_alone` |
| `apply`'s `Expire` is subject-blind | `a_clean_fold_keeps_a_message_it_did_not_write`, `expiry_reaches_one_subject_and_no_other`, `landing_a_clean_fold_does_not_clear_a_message_it_did_not_write`, `a_gather_fault_the_tree_cannot_badge_outlives_the_open_that_raised_it`, `a_joined_line_keeps_a_shared_subject_and_falls_back_when_they_differ` |
| `batch_status`'s acting arm keeps instead of clearing | `a_supersession_survives_the_accepted_edit_that_caused_it`, `a_superseded_free_move_is_news_the_ranking_shows`, `a_clean_action_clears_and_a_refusal_shows_even_from_a_hover_batch`, `a_tool_notice_survives_the_batch_that_carried_its_own_pick`, `an_acting_frame_sweeps_the_line_a_seam_refusal_would_have_been_on` |

So a row composing the three legs over a real `Withdrawal` could not
be red on any tree where it would be the row that caught the break —
four sibling rows red first. That is the register's *an item's
suggested assertion is a claim whose cheapest failure is being GREEN
on the broken tree*, answered by measurement: this one would be green
on the broken tree because the tree cannot be broken past it. The
deliverable instead is that the doc NAMES the five rows, which is what
the item was actually missing — a stated lifetime nobody could trace.

The one leg with no mutation is `frame_status` delegating to
`batch_status` on a hover-only batch, and it needs none:
`a_tool_notice_survives_the_batch_that_carried_its_own_pick` asserts
the two are equal verdict for verdict over `[Hover(None)]`, so a
`frame_status` that stopped delegating reds there by construction.

**Filed:** `a-doc-comment-names-a-test-row-and-nothing-checks-it-exists`
— naming rows in a doc comment is the crate's convention
(`blend.rs:347`, `readout.rs:121`) and is checked by nothing, because
bracketing does not help: a `#[cfg(test)]` row and a row under
`crates/viewer/tests/` are invisible to every rustdoc pass this repo
runs, so a link would be broken rather than checked. 35 spans over 29
names, all this crate's own resolving; five of them arrived with this
unit.

Signed (VIEW implementer lane `view/supersession-lifetime`).

## 2026-09-16 — `view/datums-basis`: the datum basis takes the kernel's door

Two rows, one unit:
`datums-basis-hand-rolls-the-least-aligned-axis-basis` and
`datums-unit-helper-normalizes-with-a-silent-x-fallback`.
`crates/viewer/src/datums.rs`'s `basis` is now
`UnitVec3::orthonormal_basis`, and the local `dot`, `cross` and `unit`
helpers are deleted with the seed rule that needed them. The sweep's
one other in-fence hit, `camera::up` spelling a cross by hand beside
`Vec3::cross`, is bit-identical and taken here too.

**The ruling was to adopt the door, and the measurement it turned on
was whether the drawn axis moves. It does.** Over the world axes, the
equator from both sides of the signed zero, the `<=` ties the local
rule breaks, and 20,000 random unit normals, the pair turns about the
normal for every normal sampled — none agreed to within 1e-9°. What a
reader sees is less: a square grid is symmetric under a quarter turn
and a tick under a half, so the six world axes (which move by exact
multiples of 90°) leave the three default PLANES drawing the same
picture — **while `MAX_GRID_LINES` does not bind; the review of #2783
measured it binding at 2560 px and the row carries the correction** —
while an axis datum along ±y or ±z has its end ticks turned 90° about
itself. Every other normal's grid turns visibly, up to 45°.

Not a cost to weigh: the pair is a display convention with no document
meaning — the module says so itself, and `a_frames_grid_follows_its_
own_axes` is the row that exists because a frame must NOT be drawn
through it. What the trade buys is measured too: the replaced rule is
discontinuous at each of its three magnitude ties and the kernel's
door at one seam, the equator, which it states. Three conditions for
one circle. **The per-crossing figures this entry first carried were
seam samples read as seam values and are corrected on the row**: all
three local ties share one profile (grid 0°→30°), and the door's
equator costs up to 45° of grid, which is worse per crossing, not
better.

**The dispatch's premise was half wrong and the item's was right.**
The brief called the overflowed-norm-to-paint-path a live defect that
nothing catches. Transcribed and executed, `unit` really does return
`[0,0,0]` for `[1e200; 3]` and `[1,0,0]` for `[1e-200; 3]` — but its
only two call sites were in `basis`, whose only parameter is a
`UnitVec3<f64>`, so neither input could arrive. The item's own body
said so; the dispatch strengthened it. The ruling survives on the
other two legs (a prose-unreachable fallback is the shape the charter
rejects, and the viewer was maintaining a conditioning argument for a
policy it did not decide) — and the correction changes the repair:
what an unreachable defect justifies is deleting the code that needs
the argument, not adding a finiteness question to it.

**Base-tree red**, taken from a committed tree at `27d0571b09`, not by
checkout gymnastics: `a_planes_ruling_runs_along_the_kernels_
orthonormal_basis` and `an_axis_datums_ticks_run_along_the_kernels_
first_basis_axis` fail on `origin/main` with the seed rule's directions
printed; the other two rows are green on both and are labelled as
standing guards rather than receipts. Four mutations on the fixed tree
red each row by name — swapping the pair reds the axis row ALONE
(a grid cannot tell `b1` from `b2`, and the plane row correctly does
not claim it can), a 30° turn reds the three direction rows, and a
planted zero axis reds `no_normal_makes_a_datum_draw_something_that_is
_not_a_drawing` through the axis tick only, because `rule_patch`'s
`span > 0.0` arm already refuses a collapsed ruling.

**Filed:** `sketch-headings-guard-zero-length-but-not-an-infinite-one`
(VIEW's own — two `hypot` sites the `sqrt()` pattern could not see,
both guarding `> 0.0` and both handing back a zero vector for an
infinite length) and, on MESH's slate,
`degenerate-normal-rows-model-resolution-cites-a-deleted-helper` — its
worked resolution cites `datums::unit`'s fallback as the model, and
the subject is deleted rather than moved, so the sentence cannot be
repointed.

Signed (VIEW implementer lane `view/datums-basis`).

## 2026-09-16 — `view/datum-refusals-named`: two refusals that were right and had no name

Closed `datum-view-propagates-rather-than-refusing-by-name` and
`a-datum-the-view-cannot-scale-vanishes-without-a-word` as one unit —
the signature change is the second row's prerequisite.

**`datum_view` answers `Result<View, CameraError>`**, refusing a
non-finite width or height by name with the value in the message and a
zero-area viewport with `UnusableBounds`. The `Result` over the
`Option` the row also offered was the dispatch's ruling and it paid:
`datum_view_refuses_a_window_the_way_the_cameras_own_door_does`
compares this door's reply against `Camera::ray_through`'s on the same
`ViewportSize`, variant for variant, which is an assertion an `Option`
cannot carry. Compared through `Debug` rather than `==`, because half
the inputs are `NaN` and a `CameraError` holding one is not equal to
itself.

**The class instance the first row recorded against itself is
retired.** Both sides refused means `viewport_px` is `width.max(height)`
over two finite positive numbers, so the `NaN` arm is deleted and no
value the function did not compute leaves the door. `View`'s two field
docs stop citing `datum_view` as the producer of one and say instead
what is still true: the fields are the caller's, and the doors below
owe their own check whatever is written there.

**Both rows' reachability premise is right about its conclusion and
wrong about its reason, and the gap is live.** `ViewportSize::aspect`
asks whether both sides are above zero — so it refuses zero and
refuses `NaN`, and **admits `inf`**. A pane of infinite extent has an
aspect, passes `viewport_ui`'s early return and reaches `datum_view`,
where every mark then refuses in silence. So the door's refusal is not
a subset of the pane's guard, which is the whole reason the rows gave
for calling this a hardening.
`the_panes_aspect_guard_admits_an_extent_this_door_refuses` pins both
halves. No production route was found that hands egui an infinite pane
extent; what is established is that the guard does not exclude one —
*a guard that admits everything positive is not a bound*, the register's
own rule with the sign flipped.

**So: the second row is the LIVE one and the first is hardening**, and
the first is hardening for a narrower reason than it claims. Row A's
two reachable causes are both real — the eye exactly on a datum, and a
ruling whose extent is lost to the datum's own magnitude — and neither
needs a pathological window.

**`draws` answers `DatumDraws`**, the wireframes plus
`vanished()`: how many came out with nothing drawn. A type change, so
the compiler is the sweep over callers, and a method rather than a
field, so the count cannot disagree with the thing it counts. The
property is **"drew nothing"**, not "has no scale" — the second is a
proxy that would miss the lost-extent case, which is one of the two
live ones.

**The shape pinned is the DIFFERENCE**, because "the segment list is
empty" is equally true of a document with no datums.
`how_many_datums_this_view_drew_nothing_of_is_a_fact_the_caller_is_handed`
measures four cases under one view: four datums at `f64::MAX` (four
vanished), the same four with the eye on them (four), the same four
from an ordinary place (**none** — without which the first two pass
for a module that never draws), and a document with no datums (none,
and an empty list). `a_datum_that_drew_some_of_itself_has_not_vanished`
holds the far boundary.

**No second latch was minted.** `frame::datums_badge` reads a count,
`Subject::Camera`, `Tone::Actionable`, silent at zero — and
the frame entry point (`<ViewerApp as eframe::App>::ui`, which is what
the crate's prose elsewhere miscalls `update`) zeroes the local the
panes write **before** they
draw and assigns it back **unconditionally** after, whether or not the
viewport was one of them. That is `profile_form_drawn`'s discipline,
which `projection-fault-has-no-sweeper.md` names as the pattern the
fault still lacks, so the count cannot outlive the view it describes
and there is no sweeper to be missing. At `datum_view`'s refusal the
pane writes the EXISTING `projection_fault` and returns; that is not a
new latch and not a new condition either — every input this door
refuses makes `view_projection` refuse a hundred lines down (an
infinite height gives aspect `0.0`, an infinite width aspect `inf`,
both aspect `NaN`), so the badge was going to be written on that frame
and what changed is that it now names which side was not pixels.

**Verification.** Both changes are signature changes, so a base-tree
red can only be a compile failure and is not offered as one. Six
mutations on the fixed tree instead, each redding a named row: deleting
both refusals reds the parity row AND the aspect-gap row; deleting only
the zero-area arm reds the parity row alone (the gap row is about `inf`
and correctly does not claim the other); `vanished()` returning `0` and
`vanished()` counting every drawing each red the distinguishability row
(the second also reds the boundary row); a badge that never fires and a
noun that never agrees each red the badge row.

**The badge family's own header was wrong before this touched it.**
`frame.rs`'s module doc enumerated the members as seven where the
population is eight — `prefs_badge` was missing, and the README's list
of the same family had it. Corrected to nine with the sentence now
pointing at the counted population rather than restating it.

**Sweep**, for the class *a door that hands back a value it did not
compute, in a field shaped like one it did*, over `crates/viewer/src`:
five patterns, ten hits, two filed —
`id-readback-failure-reads-as-nothing-under-the-cursor` (a failed GPU
readback becomes `IdMap::NOTHING`, and `idpass::disagreement` then
reports it to the reader as the two picking paths disagreeing about the
picture) and
`corner-count-substitutes-u32-max-for-a-length-it-could-not-cast`
(unreachable below 51.5 GB of position data, and still the class). The
hit list and the patterns' blind spots are in the PR body.

Signed (VIEW implementer lane `view/datum-refusals-named`).

**Addendum, same lane.** The first push's `rustdoc (gate)` went red on
three intra-doc links this unit wrote to `ViewerApp::update`. There is
no such function and there never has been — the frame entry point is
`<ViewerApp as eframe::App>::ui` — and the name was taken from prose
already in the tree (`frame.rs`'s `projection_badge` doc, and one row
in `work/view/`), which says `update` in a code span where nothing
checks it. Links were corrected; the two pre-existing spans are left as
written and filed as
`viewer-prose-calls-the-frame-entry-point-update`, per the in-fence
rule about a citation that was wrong at the merge base.

**And the receipt that missed it was mine.** `scripts/doc-gate.sh 2>&1
| tail -20; echo "BARE=$?"` reports **`tail`'s** exit code, not the
gate's. The bare pass had in fact failed locally before the push and
the `0` was a measurement of the pipeline's last stage. Redirect to a
file and read `$?` off the command itself; a pipeline's `$?` is never
the receipt you think you are taking. All three forms re-run this way:
`--selftest` 0, bare 0, `--skip-viewer-toolkit` 0.

**Second addendum: the review's central correction was itself wrong,
and finding that out corrected me too.** The review reported that at
`f64::MAX` every mark refuses for want of a scale and `rule_patch`'s
lost-extent arm is never reached, so the unit's justification for
counting "drew nothing" rather than "has no scale" rested on an
unreachable case. Instrumenting `grid` and `rule_patch` and running the
fixture says otherwise: **the patch centre has a scale at every
magnitude** — `Some(1.87e-4)` — because the centre is the point the
camera is aimed at and the eye is a decimetre from it, so `rule_patch`
is entered and the lost-extent arm fires. The review's probe read the
scale at the datum's ORIGIN, which is where the normal tick is scaled;
the module's own doc says the two marks are scaled at two different
points, and that is the distinction the probe collapsed. Its ratio
argument (`half / cv ≈ 1.46`, magnitude-independent) assumes the eye is
about as far from the centre as the datum's origin is; measured, `half`
is `0.26 m` against a `cv` running to `1e308`.

**But the complaint under the wrong evidence was right.** The prose
named `f64::MAX` as THE lost-extent case when it is three refusals at
once — the tick's depth overflowing to `inf`, one ruled direction's
`cv / pitch` overflowing past the finiteness guard, and the other
losing its extent — and no row split them. The measured band is in the
closed row's resolution as a table, and
`a_plane_can_lose_its_extent_while_every_point_of_it_still_has_a_scale`
is the row that splits them: at `1e100` the origin still scales, the
tick draws, one direction still loses its extent, and the plane has NOT
vanished. It reds under a `rule_patch` that emits its collapse.

**Generalises, and it is the register's proxy rule with the reviewer
holding the proxy.** *Scale* is not one quantity in this module — it is
one per mark, at one point per mark, which the module says in as many
words. A probe that samples it at a single point and reports
`has_scale` has already chosen an answer. The check is the one this
register states: name the property first and the instrument second,
then ask what a member could look like that the instrument cannot see.

**`git checkout <file>` took an uncommitted doc rewrite, again.**
Reverting the mutation that certified the new row also discarded the
`vanished()` doc edit made an hour earlier in the same file, and only
`git status` showing `datums.rs` unmodified caught it. Third instance
this program has recorded; the rule is already written (*commit before
you mutate*) and I did not follow it for an edit made after the commit.

Signed (VIEW implementer lane `view/datum-refusals-named`).

## 2026-09-17 — `view/clamp-nan`: a clamp is not a bound against NaN

Three doors that handed back a value the arithmetic did not compute,
in a field shaped like one it did. All three of the row's claims
re-executed and held; all three got a different repair, because the
question a door can answer is a property of the door.

`theme::channel_to_srgb8` refuses with `is_nan` and not `is_finite`,
which is the whole argument at that site: **an infinity is ordered and
a `NaN` is not**, so the clamp genuinely bounds one and cannot see the
other. Refusing an infinity there would have been a second, unargued
decision wearing the first one's clothes.

`sketch::arc_points` refuses with `is_finite`, which is the OPPOSITE
call for the opposite reason: an infinite radius emits `NaN` points
just as surely as a `NaN` radius does. Two sites in one diff, two
different finiteness tests, and a lane that reached for one rule for
both would have been wrong at one of them.

**The row's own framing was wrong about site 1, and the correction
made the site stronger.** It called `channel_to_srgb8` a paint path
that cannot refuse mid-frame. `Mark::over` and `from_linear` have one
caller between them — `crates/viewer/tests/theme.rs`, the colourblind
check — because the shader mixes in WGSL. So the consequence is not a
black pixel; it is that the SAFETY MEASUREMENT would have taken pure
black, the far end of every distance it computes, as the composited
colour and certified the palette on it. A door that can refuse was
argued as one that cannot, and the argument came from the item.

**Site 2's live producer is not a `NaN` at all.** A bulge of `1e-320`
on a horizontal chord — a finite literal `Expr::literal` accepts, typed
through an ordinary `DragValue` — gives an infinite apothem, and the
chord's left normal is exactly zero there, so `0 * inf` puts a `NaN` in
the centre. 256 points at `[NaN, NaN]`, reached through the CAP rather
than the floor, from an authored path. Hunting only the `NaN` input
would have closed the arm nobody can reach and left the one anybody
can.

**A distinguishability assertion over an `f32` needs two rows, not
one.** The share row was first written as *the poisoned answer differs
from every legitimate answer*, which is the right shape for the `u8`
and `usize` sites and worthless for this one: a `NaN` differs from
everything including itself, so `Some(NaN)` — the broken door's own
output — passed. The mutation found it, not the reading. The landed
pair is *the poisoned inputs answer `None`* plus *every legitimate
input answers a share*; neither row says anything alone. Generalises to
the register's rule about a suggested assertion's two cheapest
failures: for a float-valued door there is a third, which is an
assertion the defect satisfies by being a defect.

**Site 3's red could only be a compile failure**, because the function
it holds did not exist on the base tree. Certified instead by three
mutations, each redding the named row: the base tree's guard verbatim,
the stack-half alone, and the `wanted`-half alone. Sites 1 and 2 took
ordinary base-tree reds from a committed tree.

Signed (VIEW implementer lane `view/clamp-nan`).

### The review, and the three guards that were held by nothing

**A `git log -S` that finds nothing in a SHALLOW clone is not evidence
of anything, and I reported a provenance receipt I had not taken.**
The PR body named `07b41f6bf6` as the commit that wrote `theme.rs`'s
clamp sentence. That SHA does not resolve in this repository. The real
commit is `df8cc27873` (2026-08-30). Two things are worth keeping:
this checkout is shallow — `.git/shallow` exists and history bottoms
out at a grafted boundary — so a pickaxe over it reports an absence it
has no standing to report; and the sentence spans a `///`
continuation, which is the register's split-span trap and would have
defeated the search even in a full clone. **The conclusion survived
anyway** — `theme.rs` is in no companion table, so nothing was waiting
for Ev — which is exactly what makes the class expensive: a receipt
that is wrong about its own evidence and right about its answer passes
every reading that stops at the answer.

**Three of the guards I landed were held by nothing, and mutation is
the only thing that said so.** The reviewer ran three and all three
were green: undoing the theme refusal into the cap
(`unwrap_or(255)`), deleting `flatten`'s `centre`/`start` filter, and
replacing its `return Err` with `continue`. I reproduced all three
before repairing any. The general shape is this program's own — *a
typed refusal whose production is asserted nowhere* — and what it
cost here is specific: I wrote a distinguishability row for the
`usize` site that compared against the floor, the cap and an ordinary
value, wrote one for the `u8` site that compared against the floor
ALONE, and then wrote a PR body claiming both did the first thing. The
sentence was true of one row and false of the other, and it read as
true because the two rows sit in different files.

**So: a claim about a POPULATION of rows is checked against each
member, not against the row you wrote most recently.** The body's
"both rows" is the same defect this register already records as *a
universal in prose owes the sweep rule that produces its population*,
scoped down to two.

**The guard I argued for at greatest length was the one held by
nothing, and my own account of why it exists was wrong.** The
`centre`/`start` check: I wrote that it is earned by the denormal
bulge. It is not — in that producer `radius` is `inf`, so `arc_points`
refuses and `centre` never fires. It is earned by an arc whose radius
is ORDINARY and whose midpoint overflows: two vertices near the top of
the exponent range. Both cases are now rows and deleting the filter
reds exactly one of them. A long argument attached to the wrong
example is harder to catch than a short one, because the length reads
as diligence.

**And driving it through the real door moved a claim of mine.** I had
written that the denormal bulge is a live producer and described it in
a two-vertex loop. Through `preview` that loop never reaches the
flattener at all — the seam reverses onto itself and the driver
refuses it as an undeclared cusp two steps earlier. It takes a third
vertex. The claim was right; the shape I stated it in was not, and the
arithmetic I executed standalone could not tell me, because it was the
flattener's arithmetic and not the door's. **Executing a function's
arithmetic is not the same as reaching the function.**

Signed (VIEW implementer lane `view/clamp-nan`, after review).

## 2026-09-17 — the cut: four successors opened, 86 rows re-homed, VIEW not closed

Ev, in chat: *"you can self merge the cut pr. moving existing items is
low risk."* This entry records what moved and, first, the verification
the whole cut rests on.

### The Order's six units, re-derived against the tree

The exit shape above reads *"every item above has landed or been ruled
out"*, where **above** is the `Order`'s six numbered units and not the
open slate. Each was checked against the tree and the tracker rather
than against this file, because this file's own standing hazard is a
claim going stale rather than a number:

1. **`viewer-session-god-module-split` — DONE**, four PRs on `main`
   (#1801, #1816, #1830, #1832). Its two named residues are rows:
   `session-shims-and-test-imports` (live, now VDOC's) and
   `tool-kind-all-and-ordinal-have-no-production-reader` (closed).
2. **`pick-priority-filter-vocabulary` — DEFERRED**, status `deferred`
   in the file, ratified by `crates/viewer/GUI-DESIGN.md` GQ7. Not
   work, and not dispatchable by the tracker's own vocabulary.
3. **`camera-fold-clears-status-line` — DONE**, #1849. Both residues
   closed (#1957, #2026). The eighteenth status-line writer has its own
   row, `startup-notices-need-holding-to-badge`, which stays here.
4. **`focus-marking-is-per-node-not-per-segment` — HANDED OFF.** Its
   blocker was the authored-step to canonical-segment map, whose
   siting question is `authored-step-to-canonical-segment-map-has-no-home`
   — **now CLOSED on EDIT's slate at PR #2759, 2026-09-16**, with DM8's
   follow-through merged at #2785. So the blocker has fired, the map
   exists, and both halves of the ground are EDIT's. The row goes to
   EDIT with its `refs` intact.
5. **`layer3-recipenodeid-aliases-across-rewinds` — HANDED OFF**, and
   it was never this program's to clear: it is `parked` on
   `next-id-has-no-layer3-door`, a door in `crates/editor-core/src/doc.rs`
   that was DOCM's and is EDIT's since DOCM exited (`docs/DOC-LEDGER.md`
   sweep 14). Both rows go to EDIT together, so the trigger and the
   row it gates land on one slate.
6. **`pick-index-built-on-ui-thread` — DONE**, #1888. 6a ruled by Ev at
   #1843, 6c collapsed into 6b. Seven residues filed as items; the one
   still open, `ui-thread-work-after-the-index-seam`, is VSEAM's.

**So the reading holds: none of the six is open VIEW work.** Three
landed, one is ratified not-now, and two are blocked on doors in
another program's crate — which is a hand-off, not a slate.

### What the slate actually was

**Ninety-four live rows**, not the seventy-eight a triage taken on
2026-09-16 recorded: five of that triage's rows had closed
(`a-supersession-outlives-its-own-frame` and the four `datums.rs`
rows), and **twenty-one new rows had been filed since**, sixteen of
them by the six units that merged in between. The triage's own counts
were stale by construction and are not carried forward; every count in
this entry was re-derived with `scripts/work.py status --program view`
on the branch's merge base.

### Where they went

| destination | rows |
|---|---|
| `vnews` | 14 |
| `vgeom` | 21 |
| `vseam` | 14 |
| `vdoc` | 22 |
| `guard`, `ciw`, `edit` | 3 each |
| `suite`, `meta` | 2 each |
| `dup`, `chrome` | 1 each |
| stayed here | 8 |

Eighty-six moves, each a `git mv` with the body, the id and the history
unchanged. **No row's prose was edited on the way past** — and the item
schema carries no `program:` field at all (`scripts/work.py`'s `SCHEMA`;
ownership is read from the directory and nowhere else), so a re-home is
the move and nothing else. `refs`, `blocked_on` and `rides_with`
resolve by id and are unaffected.

### The four charters, and the test they were held to

Each track's charter is the sentence that is true of its rows and false
of the other three tracks' rows — this program's own rule about splits,
applied to itself. In one line each:

- **`vnews`** — a defect in the vocabulary a fact travels in on its way
  to a reader, never in the fact; the fix changes a type or a door and
  nothing it touches survives its frame.
- **`vgeom`** — a value: a non-finite, out-of-range or under-precise
  number crossing a door whose prose says it refuses such a thing, or a
  control that never reaches the transform it names; the fix changes
  what the viewer SHOWS.
- **`vseam`** — something the viewer holds on behalf of the document
  that outlives the frame that made it, with no named boundary owning
  it; the fix names one.
- **`vdoc`** — a claim the tree makes about itself; apply any fix on
  that slate and nothing a person could observe has changed.

**One row was placed by elimination and its plan says so**:
`adjacent-same-typed-arguments-are-the-same-swap` is on VSEAM's slate
because its two worst instances are that program's authoring doors and
because the other three charters are each false of it, not because the
charter fits.

### Sequencing, and why `vdoc` is last

`vnews`, `vgeom` and `vseam` are file-disjoint except at the shared
files their `keep_out`s name on both sides, and all three dispatch from
their opening day. **`vdoc` does not.** Its spine is
`stale-file-citations-after-the-split` and every unit the other three
land invalidates more of it; the register's own rule is that an
out-of-fence citation table expires the moment another diff touches the
same file. That is written into `work/vdoc/plan.md` §Order as the
program's opening condition, with two named exceptions whose subject
does not move with the code.

### Territory

`work.py lint` went **18 warnings → 24**, all six new ones one-sided
pairs whose other half is in a file this program may not edit: four
against CHROME and two (via `vdoc`'s `crates/viewer/tests/*`) against
S-TCOST and S-TINT. The six pairs *inside* the new family are silent,
because both sides were written in this commit. This program's own
`keep_out` was extended to name the four successors, which is why the
`view` pairs do not appear. The CHROME half is filed as
`work/chrome/the-four-view-successors-are-a-one-sided-double-claim`;
the tests half is the standing case in
`work/meta/double-claim-lint-rule-waits-on-the-tests-seam`.

### The register

The six hundred lines of rule register in `work/view/plan.md` bind
lanes in all four successors and are **inherited by reference, not
copied** — four copies of a register re-derived every wave give four
divergent copies inside a week, which is this program's own
count-fixed-in-one-place defect turned on its own discipline. The cost
is stated rather than hidden: the file dies with this directory. Filed
as `the-lane-register-has-no-home-after-views-directory-goes`, and it
is a **precondition of the exit walk**, not a follow-up to it.

### What this entry does NOT record

This program is not closed. There is no exit walk, no
`docs/DOC-LEDGER.md` entry, and no directory sweep; PR #2762 is still
open and parked on a ruling. Eight rows stay on this slate and the
successors' `plan.md` §Inbound names where each of the six in `review`
goes when its PR merges.
---

## 2026-09-17 — a crashed seam worker panics; the badge vocabulary is deleted

`a-dead-seam-worker-reads-as-an-ordinary-idle-state` closed a second
time, on a different ruling. The first (2026-09-16: badge it,
`Tone::Actionable`, and the fit seam refuses the index build) was made
on a description of the cost that was **wrong**, and Ev replaced it
rather than amending it: *"isn't a worker dying an infra thing that
should show up as a panic?"*, then *"panic on crash is good"*.

**What the review found, and why it inverted the justification.** The
fit refusal does not cost picking. `settled_delta` answers `None`
forever once the fitter is gone, `PickCache::sync` forgets and returns
`CacheStep::Nothing`, and `sync_scene` returns on that step BEFORE the
scene rebuild — and `self.scene` has exactly one writer. On a fresh
open, where the first landing is what fires the fit, the document never
draws. Ev's own justification for refusing was *a frozen window is
worse than a dead one*; the true cost was a dead one.

**The gating fact, established before anything was built.** The
announcement is a panic on the UI thread, raised inside
`<ViewerApp as eframe::App>::ui` — so if anything up the toolkit stack
caught it, the loudest thing this crate does would be a lie. It does
not: `egui`, `eframe`, `egui-winit` and `egui-wgpu` 0.36.1 contain no
`catch_unwind` at all (eframe's only panic machinery is
`web/panic_handler.rs`, a `set_hook` on the wasm build), and `winit`
0.30.13 has none on the linux backends — it catches on macOS and
Windows only, and both re-raise (`macos/event_loop.rs:307` and `:374`,
`windows/event_loop.rs:425`). The layer nearest the panic is
**executed** rather than read:
`a_panic_inside_an_egui_frame_is_not_swallowed` plants one inside
`Context::run_ui` — eframe's own per-frame call, at
`epi_integration.rs:288` — and asserts the unwind escapes with its
payload intact.

**The discriminator is exact, and that is the whole fix.**
`Coalescing::close` takes the request channel and is called from `Drop`
and nowhere else, so on a running application a detection that still
holds the channel is a crash and nothing else. The two endings used to
be answered by the same three lines, which is what made a crashed seam
report exactly what an idle one reports — the item's defect, one layer
below where the item looked for it.

**What could not be tested the way the dispatch asked, and why.** The
three shipped handles cannot be crashed from outside: a worker dies
only by panicking inside its own job, and the job is `build_index`,
`run_fit` or `run_once` behind a private closure. A row through
`ThreadIndexer` would have to make the kernel panic on an input, which
is what D9 says cannot happen. The rows therefore drive `Coalescing`
itself — the shipped machine every handle delegates to, not a mirror of
it — and say so.

**And one of the two arms is not sequence-reachable.** `dispatch`'s
failed send needs `running` false with a dead worker; after a crash
`running` stays true until a `poll` clears it, and that poll takes the
other arm. The redispatch entry needs a buffered answer AND a dead
worker at once, which the worker loop cannot produce: it sends an
answer only when `answer` RETURNED, and a worker that returned is one
that went back to `recv` and can only die on a job it was then handed,
for which no answer is ever sent. The arm stays — the condition means
what it means — and its row clears `running` by hand and claims the
arm's behaviour and nothing about reachability.

**Four mutations, each redding only its own row**, and the first is the
base tree's own behaviour: `poll` forgetting instead of announcing
(the reachable row), `dispatch` forgetting instead of announcing (the
arm row), an orderly close treated as a crash (the discriminator row,
which nothing else covers), and removing the planted panic from the
egui row.

**Deleted, not kept:** `WorkerGone` and its three sentences,
`worker_gone` on the three seam traits and all six implementations,
`evalseam::settled_delta`, `frame::dead_seam_badge` with
`EVALUATION_SEAM`, `DocSession::eval_worker_gone`,
`PickCache::worker_gone`, the disabled Re-evaluate control, and the
README's badge-population and seam clauses. A badge reachable only
through a test fake is a badge family held up by its own tests. The
first ruling's residue row — `the-canceled-label-names-a-cause-a-dead-worker-did-not-have`
— was deleted rather than carried, because its premise was a dead
evaluator reaching `Progress::Canceled`, which the ruling makes
unreachable.

**Surfaced rather than overridden.** This is the workspace's first
non-test `panic!`: every existing `allow(clippy::panic)` in `crates/`
sits on a `#[cfg(test)]` module. D9's family is scoped to INPUT and a
worker thread is reachable from none, so the site takes an `expect`
with that reason written at it — but PIPE's `work/pipe/S14` is open,
`needs_ev: true`, and carries Ev's own 2026-08-18 reframe, *"no panic
on any reachable state, yes panic on things that can only indicate
bugs"*, which S14 records as an **amendment** to D9 rather than a
clarification. This unit lands on that ground while it is unruled.
Read, not edited — S14 is PIPE's.

Residue filed:
`the-quiet-seam-half-of-pickcache-indexing-has-no-shipped-producer`
and `the-dying-seam-fakes-mirror-a-machine-they-do-not-share`.

Signed (VIEW implementer lane `view/dead-seam-badge`, after Ev's
second ruling).

## 2026-09-21 — five rows were merged and never closed

Found by the `vseam/gesture-naming` lane, which read VSEAM's Order item
7 as waiting on `two-hand-written-copies-of-the-g1-gesture-machine` and
then found `crates/viewer/src/g1.rs` sitting on `main`. The row still
read `status: review`.

It was not one row. Five VIEW rows carried `status: review` with a `pr:`
whose work is on `main`:

| row | PR | merged |
|---|---|---|
| `id-query-is-keyed-on-the-generation-not-on-the-picture` | 2622 | 2026-09-14 |
| `a-pick-over-a-stale-picture-answers-about-a-picture-nobody-can-see` | 2662 | 2026-09-15 |
| `the-two-seams-are-hand-maintained-twins` | 2666 | 2026-09-15 |
| `the-picture-key-never-became-a-type` | 2670 | 2026-09-15 |
| `two-hand-written-copies-of-the-g1-gesture-machine` | 2672 | 2026-09-15 |

The orchestrator merged all five and closed none of them. `review` means
*a PR is open over this*, so for six days the board said five units were
awaiting review that had already landed.

**It propagated.** The cut (#2806) deliberately kept the `review` rows on
VIEW rather than re-homing them, on the stated ground that *"their lanes
are editing those exact files on `view/` branches"*. That premise was
false when it was written — every one of those branches had merged — so
five rows stayed on a program that is winding down instead of going to
the successors that own their ground, and VSEAM's plan recorded a wait on
a trigger that had already fired.

**Two instrument errors while establishing this, both already in the
register and both made anyway.** `git log --grep="Merge pull request
#N "` returned nothing for all five, which reads as *not merged*; the
repository uses at least two merge-commit title formats and these five
landed as `VIEW: … (#N)`. The pattern was the wrong population, not the
answer. And the first reading of that nil result was *the clone is
shallow, so the pickaxe proves nothing* — also a register rule, also
reached for, and also wrong here: the history goes back to 2026-07-13.
A nil result has more than one explanation and the rule that names one
of them is not a diagnosis.

Closed with their merge dates. `4889b7c3d2` and its four siblings each
carry two parents, so merge-only held; only the bookkeeping lapsed.

## 2026-09-21 — the orchestrator's own board was stale, and two lanes went out

**Four PRs merged.** #2990 (`vseam/plan-staleness` — four stale claims
in VSEAM's plan, plus the META lint row), #2994 (`vseam/fmt-literal-row`
— AUTH-2's re-minted literal filed on AUTHOR's slate, plus two register
entries in `work/vseam/log.md`), #2995 (this branch's 1,043 lines of
log, three rulings and one register rule), and before them the #2967
sketch-guard unit that `main` already carried.

Two of the four needed a `main` merge resolved by hand on
`work/vseam/log.md` — both lanes had appended to it. That file is now
the most conflict-prone path in the tracker, and the register's
merge-before-push rule is what keeps it cheap.

**The finding of the wave, and it is the orchestrator's own.** Picking
the next unit meant opening `work/vgeom/`. Order item 2 — the two
sketch guards — read `status: open` in both rows over a program log
that ended at its opening state. Both false: #2967 had merged the
fixes and closed both rows six hours earlier. This clone was pinned at
`0c530f67ef` and every `work/` read this session had been taken from
it.

It is #2990's own defect one level up — a status read from somewhere
nothing checks — and it had a second consequence: 1,043 lines of this
log and three rulings had never reached `main`, the same way none of
the register's rules had until #2952. Both are now on `main`, and the
rule is in the register: **a board read is `git show origin/main:<path>`,
never `cat <path>`.**

**Two lanes dispatched, and the constraint that shaped the pair.**
Disk is the parallelism ceiling — ~22 GB free and a viewer target dir
costs 6–13 GB — so only one lane can build:

- `vgeom/refusal-floor` — VGEOM Order item 1, the four-row refusal-floor
  sweep (`props.rs`, `input.rs`, `camera.rs`, `gpu.rs`), dispatched
  against #2644's shape rather than a second one invented for it. One
  target dir.
- `vdoc/readme-counts` — five VDOC rows that are one class, *a number
  stated in prose that does not re-derive under its own rule*.
  README-only, docs tier, **no build and no target dir**, which is the
  whole reason it can run beside the other.

The pair is coupled and the dispatch says so: `recourse text is composed
six ways across five modules` moves if the refusal sweep adds recourse
text. The instruction is not to coordinate but to **re-derive every
count on the merged tree immediately before pushing, and name the commit
beside the rule** — and to report it as the finding if a number moves
between the first derivation and the last.

## 2026-09-21 — the VDOC count unit, and a rule that returned zero

#2998 merged: five P4 rows on `crates/viewer/README.md`, one class —
*a number stated in prose that does not re-derive under its own rule*.

**Reviewed by re-deriving, not by reading.** Eleven driver
declarations against an eleven-row table; 22 (later 24) destructuring
`let` binds; thirteen census rows; `.field(` = 17 + 4 = 21; 41
`Display` impls with none inside a comment, six of them structs, so 35
enums; `PickCache::forget` clears three of four fields and **there is
no field called `attempted`** — the sentence the README had been
carrying named a field that does not exist.

**Two orchestrator premises overturned by the lane, and both stand.**
The dispatch called
`the-citation-receipts-summary-numbers-are-not-re-derivable` "the
substance of this unit" and said every row was README prose. Those
four numbers are not in the README and never were — they live in a
merged PR body, `work/view/log.md` and a closed VIEW row — and #2089
had already repaired all four, so what was owed was a check nobody had
run. And the dispatch asked for a derivation SHA beside each number;
the lane refused, on CLAUDE.md's present-tense-only rule for these
pages and on a SHA being a second number to keep stale. That is the
better answer.

`docs/DESIGN.md:33` settles the sign-off question for this page, and it
is worth having found rather than assumed: *"`crates/viewer/README.md`
beside it is the implementation record, **which the program maintains
itself**"*. The page is not ratified design text, so no `[ev]` PR is
owed for its own prose.

**The blocking finding was the unit's own class, in the rule the fix
wrote.** The census section printed

    rg -U --no-heading -o 'let\s+&?[A-Z]\w*\s*\{[^}]*\}\s*=' ...

and said to read the lines beginning `let`. Without `-n`,
`--no-heading` prefixes each match with `path:`, so that reading
returns **0** against a stated 22; the PR body's receipt carried the
`-n` form. **A rule that yields zero where the prose says twenty-two is
worse than the bare number it replaced, because it reads as a
receipt.**

Sent back and fixed: every printed command now ends in `| wc -l` so
its output IS the answer, and the lane ran the exact printed text
extracted from the file with `sed -n Np` rather than retyped. Correcting
the pattern to admit a qualified path then moved the population 22 → 24
— two `egui::` binds at the chrome boundary that the old `[A-Z]`-only
pattern could not see — which is the rule earning its keep on the day
it was written.

**The shape, now named: a receipt that was never run in the form it was
written down in.** Three instances today — the `\`-continued literal
`cargo fmt` re-minted after its author last saw it (#2994), the
un-mergeable-PR signature whose four symptoms included three that fire
on healthy runs, and this. The tell in all three is **a rule stated as
a description of the output rather than as the command that produces
the answer.** Recorded in `work/vdoc/log.md` by the lane that paid for
it.

**Next dispatch, same file, sibling class.** `vdoc/readme-attributions`
— four rows where the README attributes a fact to the wrong site or
states a population a member has since joined. Zero build, so it runs
beside `vgeom/refusal-floor`, whose target dir is 7.8 GB of the 22 that
were free.

## 2026-09-21 — the attribution unit, and a fence the orchestrator had built wrong

#3002 merged: four P4 rows on `crates/viewer/README.md`, the sibling
class to #2998's — *the page attributes a fact to the wrong site, or
states a population a member has since joined.* All four closed.

**Verified by re-deriving, not by reading.** `features.rs:80-81`
carries the comment *"How LOUD a drawn badge is, is not decided here —
that is `RowStatus::tone()`"* four lines above the call the page
blamed, so the README had been contradicting a comment sitting beside
its own subject. `tree.rs:149-153` is where the rule is stated.
`display.rs:355` holds the link the page cited at `:262`. All four
printed commands run verbatim out of the file: 11, 24, 3, 43 and 15.
`rg -l -e SessionOp -e OpOutcome crates/viewer/src` is 27, against a
cell that said seven.

**The unit produced a live CI finding, not four citation fixes.**
Re-deriving the cross-crate-link census moved it from twelve
hand-written addresses to a command that prints **15**, and the
bullet's *conclusion* went with the addresses: fourteen of the fifteen
target `pncad`, and the fifteenth is `session/refuse.rs:151`'s
`` [`editor_core::edit::UNDECLARED_PARAM_RECOURSE`] ``.
`ci-filter.py:1436` is `VIEWER_TOOLKIT_SEEDS = {"viewer", "pncad",
"bvh"}` — **`editor-core` is not in it**, so the ungated-link hole the
bullet called theoretical is open today. Filed on MIRROR's slate beside
the parent row that asserts the hole is empty.

**And the lane caught a fence this orchestrator had built wrong.**
`work/vdoc/program.md`'s `keep_out` said *"`crates/viewer/README.md` is
ratified design beside the code per CLAUDE.md so … a change to what a
Where-in-the-code decision row DECIDES waits for Ev."* That is false,
and it was written **in the cut itself** (`f8a822e8c1`) — by me, not by
Ev. CLAUDE.md's sign-off exception covers the `crates/<crate>/README.md`
pages **that `docs/DESIGN.md`'s companion table lists**, and
`grep -cE '^\| `crates/viewer/README\.md`' docs/DESIGN.md` is **0**.
The table's `crates/viewer/GUI-DESIGN.md` row says of it: *"beside it is
the implementation record, **which the program maintains itself**"*.

So the fence was inventing a sign-off requirement out of nothing, on a
page four of this program's rows touch. Corrected in `program.md` in
the same wave, with the companion-table reasoning written into it so
the next lane does not have to re-derive it. **The check that found it
is CLAUDE.md's own**: *check that Ev ever agreed, before you wait for
Ev* — `git log --all -S` over the sentence, which returned the cut's
own commit and no ratification behind it.

Two rules for the register fall out, and both are about the
orchestrator rather than the lanes. **A fence written in the same
commit as the program it fences has no independent authority** — the
cut wrote eighty-six rows' new homes and four `keep_out` clauses in one
pass, and nothing re-derived the clauses afterwards. And **a `keep_out`
that asserts a sign-off requirement is a claim about CLAUDE.md, so it
is checkable**: the companion table either lists the page or it does
not.

## 2026-09-21 — the cut's residue: three rows, two closed, and the list had moved twice

`view/cut-residue`, a tracker-only lane. All three rows are one class:
**the cut of 2026-09-17 wrote four programs' territory and status in one
pass and nothing re-derived what it wrote** — the class the register's
newest rule names (*a fence written in the same commit as the program it
fences has no independent authority*), whose fourth instance was the
VDOC README fence corrected this morning at #3002.

**Row 1 — the unclaimed files — closed, and re-derivation moved it
twice.** The row said eleven files under `crates/viewer/src` were
claimed by no successor. The difference, re-run with
`fnmatch.fnmatchcase` as `scripts/work.py:459` matches a territory glob,
gives **ten**: `tree.rs` left the set exactly as the row predicted it
would, claimed by VNEWS in the commit that filed the row, with nothing
announcing that the entry had closed.

The second move is the row's own instrument. Its claim is that an
unclaimed file has *"an owner that will not dispatch it and a
dispatching program that does not claim it"* — a claim about every open
program, tested against four of them. Widening the match to every
`work/*/program.md`, ignoring `view` and `chrome` whose
`crates/viewer/src/*` globs are the ones that die, puts **`drafts.rs` in
AUTHOR's `paths` already**, where CHROME's cut of 2026-09-20 put it.
AUTHOR dispatches. So the gap was **nine**, and a claim of it for VSEAM
was written and then removed on that evidence. This is this register's
proxy rule with the row holding the proxy.

The nine went: `bin/viewer.rs`, `platform.rs`, `prefs.rs` → VNEWS;
`blend.rs`, `matetool.rs`, `revolvetool.rs` → VSEAM; `pane/profile.rs`
and `parts.rs` → VSEAM **and** VNEWS; `theme.rs` → VGEOM **and** VNEWS.
Each split is written on both sides, which is the one-sidedness this
class keeps producing. Nothing went to VDOC, which claims no
`crates/viewer/src` file but `lib.rs` by design.

**And one thing the sort could not place, said rather than papered
over.** `bin/viewer.rs`, `platform.rs` and `prefs.rs` are the viewer's
**process boundary** — what the run commits before the first frame, what
the environment offers, what survives between runs — and no successor
charter has that subject: VNEWS's test is that nothing it touches
survives the frame, and a preference survives the run; VSEAM's is what
the viewer holds *on behalf of the document*, and a `zenity` probe is
not the document's. The three are claimed for the word-shaped rows live
on them today, and `work/vnews/program.md`'s `keep_out` now says that a
row about whether a preference is KEPT, or when a probe is re-read, goes
to this orchestrator as a report rather than into a program whose own
test excludes it.

**Row 2 — the tier rule — closed, with a discriminator that is not a
count.** *Which tier am I on* is now answered by
`python3 scripts/ci-filter.py --base <base>` printing `TIER=`, named as
authoritative because the workflow classifies with the same script; the
docs tier on the run is `gate ok` **green with every code row skipped**,
and `docs-only ok` is green on both tiers so it is never the marker. The
**21** went and so did the **38-39**. Twelve `test (…)` and five
`k-lint (gate, …)` stayed, because each carries an enumeration rule
`ci.yml` states — `{default, interval}` x `{default, 1e-6, 1e-12}` x
`shard: [1, 2]`, and the literal `klint_rows` list — which is the test
for whether a count may be written down at all.

The sweep for the same stale number found it in four other populations,
all of them **receipts rather than rules**: log entries and item bodies
recording what a named run showed on a named day. Those are not
rewritten — rewriting a receipt makes it a receipt for a run nobody
took. Nothing outside `work/view/plan.md` states the count AS a rule.

**Row 3 — the register's home — stays open, with the options and the
archaeology written in.** 1,470 lines; **92** bold-opening paragraphs in
the register span, of which 5 are wave narrative and **87 are rules**;
**twelve** of the 87 are VIEW-bound by the test *would this still be
true if `crates/viewer` did not exist*, so **75 are general**. Six
sevenths of the file has nothing to do with the viewer, which is the
input the decision turns on. Twenty citing lines in fourteen files
outside `work/view/`, ten of them the structural inheritance that
dangles on sweep day. Recommended: **`work/LANE-REGISTER.md`**, a peer
of `work/README.md` belonging to no program, with the split deferred and
`docs/prompts/` put to Ev as its own governance question later — because
87 rules in eighteen days, two of them written this morning within an
hour of the failures that produced them, is a cadence a sign-off gate
would not survive.

**Two instrument errors while measuring, both in the file being
measured.** The rule count came back **97** first, from an `awk` whose
`NR>=165` guard sat on the `exit` rule rather than on the counting rule,
so it counted the whole file. And the first draft of option B said a
file at the top of `work/` is simply unparsed; `scripts/work.py:280`
makes it a lint **ERROR** unless it is in `FREE_FILES`. Both are the
register's *read it at the source* rule, committed against the register.

**Filed elsewhere:**
`work/vgeom/vgeom-plan-has-no-register-section-and-no-charter` —
`work/vgeom/program.md` points every VGEOM lane at a
`work/vgeom/plan.md` §The register that does not exist, and the
priority-seam re-cut of 2026-09-20 took §Charter with it, so VGEOM is
the only one of the four successors with no written charter test. That
is why this lane had to reconstruct VGEOM's test from `program.md`'s
description rather than apply the charter it was told to apply.

**Not filed, because a row already owns it:**
`work/vdoc/the-four-view-successors-are-a-one-sided-double-claim` asks
CHROME to name the four successors in its own `keep_out`; the nine newly
claimed files ride that row unchanged. `python3 scripts/work.py
territory --overlaps` reports **58 lines before and 58 after** with no
successor-to-successor pair in either (a pair recorded on both sides is
not reported), and the `chrome`-side counts grow 11 → 17 (`vnews`),
16 → 17 (`vgeom`), 21 → 27 (`vseam`).

**A premise correction for whoever dispatches next.** The dispatch said
`work.py lint` prints one-sided territory warnings and asked for before
and after counts. It prints **none** — `ok (0 problems, 0 warnings)` on
both trees. The double-claim lint warning was retired deliberately
(`work/README.md`, Territory; `work/meta/double-claim-lint-rule-waits-
on-the-tests-seam`), and the one-sided census is
`work.py territory --overlaps`, a report run on demand. A dispatch that
asks for a warning count nobody can produce teaches a lane to invent
one.

Signed (VIEW implementer lane `view/cut-residue`).

## 2026-09-21 — the refusal-floor unit, the cut's residue, and a posture a cut reopened

**#3000 merged** — VGEOM's four refusal-floor doors, the P0 among them.
`SlotValue::of` now answers `Result` and raises
`DimensionError::NonFiniteLiteral` — *the same refusal, by name*, that
`Expr::literal` raises for the continuous half, which makes
`props.rs`'s standing promise true rather than merely patching it.
`world_per_px` asks `is_finite` before reading its bound as one, and
guards the quotient too. `camera::sphere` asks `finite` of the PRODUCT,
where the overflow is. `gpu.rs`'s two saturating casts became one
`draw_range` door that refuses. Five mutations, one row red each.

Verified the receipt myself rather than reading it: run 35583567062 is
39 jobs, **twelve `test (…)` and five `k-lint (gate, …)` all green**,
`gate ok` green and posted last (09:57:19, after `release-default`'s
09:57:08). Counted from the check list, not read off `docs-only ok`.

**#3003 merged** — the cut's residue. Nine of `crates/viewer/src`'s
fifty-two files had no successor claiming them while VIEW is NOT
DISPATCHING; each now has one whose charter sentence decides it, every
split written on both sides. Re-derived: 52 files, exactly
`drafts.rs` left outside the four, and it is AUTHOR's and AUTHOR
dispatches. The tier rule lost its totals and kept the two counts that
carry an enumeration rule.

**Four dispatch premises wrong, all four caught by lanes.** I wrote a
receipt line (`work.py territory` with no argument) that exits 1; asked
for a lint warning count the tool does not produce; said "the four
§Charter sections" when there are three; and quoted `work/vgeom/plan.md`
as naming #2644 as its shape precedent when that file says no such
thing.

**The fourth is the stale-board rule failing on its second day.** I
re-derived VGEOM's ROW LIST from `origin/main` and read its PLAN from
the working tree, which was pinned at `0c530f67ef`. The rule as written
covers both; I applied it to the artefact I had just been burned on and
not to the one beside it. And a third variant bit an hour later:
`git show origin/main:<path>` is only as fresh as the last `git fetch`,
and mine was two merges behind. **The rule wants both halves: fetch,
then read the ref.**

## The posture a cut reopened

The 2026-09-20 priority-seam cut (`fb899b019c`) rewrote
`work/vgeom/plan.md` and took §Charter and §The register with it —
`work/vgeom/program.md` went on pointing lanes at a §The register that
had not existed for a day — and it overwrote §Review posture with the
template for a **newly opened** program: *"OPEN, for this program's
first dispatch … the first orchestrator answers it here rather than
inheriting an answer."*

VGEOM is a parent of that cut, not a child of it. The roster entry for
the cut itself says which programs the v7 triage question is open for —
EMIT, GATHER and FIT, the three it opened — in the same sentence that
says *"WIRE and VGEOM are NOT closed and keep their bands 3700-3799 and
5300-5399"*. And the 2026-09-17 clause is explicit: *"All four inherit
VIEW's posture verbatim (Ev, in-chat, 2026-09-04, reaffirmed that
evening) … no duals and no row recorded … Each program's `plan.md`
§Review posture states it."* Ev reaffirmed it at this session's
hand-over.

So a template silently reopened a question Ev had answered twice, on a
program that was mid-wave. All three sections restored, quoting the
roster rather than re-asserting anything, and the stale *"Nothing
dispatched"* head line re-pointed at the log.

**This is the morning's `keep_out` finding again with a different
subject**, and the register rule generalises to cover it: a fence, a
posture or a status **written by a cut** has no independent authority,
because nothing re-derives what a cut writes. Two instances in one day,
from two different cuts, one of them mine.
