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

## Three units merged; the machine's first catch was a sibling lane (2026-09-04)

`view/set-param-precheck` (#1846), `view/status-lifetimes` (#1849) and
`view/module-kind-gate` (#1848) are all on main, plus the handover
(#1844) and one cross-program finding (#1852). Style review each;
correctness reviewer added on #1849 only, where the failure mode was a
confident wrong answer. **No A/B rows written, per Ev.**

Board: 20 open, 1 parked, 6 closed, 1 on Ev. The open count went UP by
five, which is the point — five of today's items existed this morning
only as sentences in PR bodies or inside other items' prose, where
`work/README.md` says the close sweep cannot see them.

### The finding of the day: a ratified rule was false when it was ratified

`crates/viewer/README.md`'s vocabulary/driver rule went in through an
`[ev]` PR (#1801) **sold on being mechanically checkable**, and it was
false at five sites the day it landed: `pick.rs:64,2266` and
`parts.rs:40,140,149` take `&DocSession`. Nobody noticed for a day,
because nothing checked it — which is exactly what
`boundary-rule-has-no-mechanical-check` was filed to fix. **The item's
thesis was confirmed by its own fix.**

That is not the only one. `crates/viewer/README.md`'s flat-arm list
named *"this boolean's operands are the same node"* as a fact existing
only at layer 3, and VIEW-4 proved by execution that
`DocEdit::InsertNode` refuses it as `DuplicateInput`. Two ratified
clauses, both false, both found by **building the machine rather than
by reading**. The instruments this program has been relying on — a
reader with the tree open — had already read past both.

VIEW-4's fix is the one to remember: it replaced the false example AND
added the sentence that qualifies an entry (*each names a fact `apply`
has been read for and does not hold*), turning an instance fix into a
class fix. The list now states the reading that puts an arm in it.

### The gate's first catch was VIEW-3

VIEW-5 built the machine; the first thing it caught was this
program's own other lane. VIEW-3's `frame.rs` test module built a
`DocSession`, which the ratified rule forbids in as many words ("can
be read, **and tested**, without a session or a window existing").
**Neither lane could see it — each had half the tree.** I found it by
merging the branches in a scratch worktree and running the gate, which
is now the thing to do before merging two lanes over one crate.

VIEW-3 resolved it better than either remedy I offered: rather than
moving every row to the driver, it split them by what they test —
`frame` keeps policy rows over hand-built values (`Folded`'s fields
are public; `ProductError` variants are values), and the rows needing
a real session moved to `pane/viewport.rs`, which IS the driver. So
`frame` satisfies the README clause for real, not around the gate.

### Reviews earned their keep, and corrected the dispatcher five times

Every review was a style review; only #1849 got a second. The
corrections against me: the status-line census (I said ~15, listed 17,
truth is 20 at the merge base); the fence reason (I claimed CHROME has
items in `pane/create.rs` — **no CHROME file names that path**; they
cite `app.rs` and the split moved the code, so
`stale-file-citations-after-the-split` is now blocking a fence
argument, its third appearance today); "the gate's header is too long"
(third LOWEST comment ratio in `scripts/gates/`); and two counts in a
brief (four copies of the clearing walk, not three; 21 fault enums,
not 20).

**Nine dispatcher corrections across this program now.** The posture
that produces them stays: state the dispatch's claims AS claims, and
say so in the brief.

The reviews also broke things I had assumed. VIEW-3's "strictly
louder" badge measured **lower** luminance contrast in every palette
(4.34:1 dark, under WCAG AA) and `theme.rs` says that colour is
*redundant*, not loud — so the choice stood and the argument was
rewritten. And VIEW-5's exception array was **file**-granular while
three documents said site-granular; a sixth `&DocSession` site in
`pick.rs` passed green. It is `FILE|NEEDLE|COUNT` now, and I verified
the fix by planting the violation myself rather than on the lane's
word.

### Two things that went in because a review asked "can it fire?"

- VIEW-5's gate was sited in `discipline`, which is gated on
  `run_build`. Its own subject is a README, and a README-only change
  classifies `TIER=docs`. So it could not fire on its own inputs —
  against `ci.yml:804-812`, a ratified Ev ruling that had already
  moved two gates for this. It is in `mirror` now AND registered in
  `check-ci-mirror-parity.py`'s `TIER_BLIND`, so putting it back reds.
- VIEW-5's self-test passed with nine of thirteen needles deleted.
  The fixture list is now derived from the same two documents the
  matcher is, and seven weakenings go red.

### Where the program stands

Unit 1 closed, item 3 closed, the two countermeasures closed. **Item 4
(focus map) and item 5 (layer-3 ids) are still blocked on other
programs** and nothing today changed that; both blockers now have
files rather than sentences. **6a is with Ev** (#1843) and has moved
a long way: he proposed a loading indicator over any stale-read
policy, which — if confirmed — **collapses Q1 entirely**, because an
index never read while stale is not a shadow and GQ6 needs no new
sentence. I withdrew my own recommendation there: the id-buffer
fallback renders from the same stale mesh, so it was never a second
path.

`tracker-has-no-status-for-an-unscheduled-trigger` is `needs_ev` with
**no `[ev]` PR carrying it**, which by `work/README.md` means the
question has not actually been asked. That is the next orchestrator
debt.
