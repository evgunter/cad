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
