# VSEAM — log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/vseam/plan.md`. A/B band 5400–5499
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

- `adjacent-same-typed-arguments-are-the-same-swap`
- `evalservice-coalescing-rule-is-prose-no-implementor-is-held-to`
- `index-request-and-index-inputs-are-one-concept-twice`
- `new-document-owes-the-reframe-open-gets`
- `no-persistent-setplacement-session-op`
- `patternrulespec-is-a-partial-mirror-with-no-growth-alarm`
- `pick-priority-filter-vocabulary`
- `projection-fault-has-no-sweeper`
- `refused-a5-gate-eats-the-body-the-fit-then-regathers`
- `revolve-tool-unreachable-no-axisinplane-form`
- `session-save-is-two-acts`
- `the-two-drags-name-their-gestures-in-two-shapes`
- `ui-thread-work-after-the-index-seam`
- `viewerapp-document-derived-state-has-no-boundary`

The charter that makes these one program, and the sentence that is true
of them and false of the other three tracks' rows, is `plan.md`
§Charter. The band 5400–5499 is claimed in `docs/MODEL-AB-LOG.md` in the
commit that opens this program, per that entry's own rule.

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`
if the posture ever changes; under the inherited posture it records no
row.

## 2026-09-19 — Ev's Save As row landed (PR 2858)

Orchestrated from a session taking Ev's high-priority GUI rows across
the paused viewer programs. Not an A/B-protocol unit, by Ev's
instruction. The Fable implementer hit the account limit mid-work; Opus
finished from its uncommitted diff, keeping the rule, the helper, the
prefs key and the tests. It dropped a zenity/portal backend split, which
the headless test showed the probe cannot decide. The review was
style-only. Its fix pass took ten findings. The main ones:
- the door also owns remembering
- one home for the empty-parent rule
- the sibling prefs keys rendered through TOML, which fixed an older
  unescaped-preset bug
- a rename sweep over prose and open rows
- a GUARD row for the gate's `current_dir` blind spot
- the chooser-probe row restated to match its evidence

Candidate order kept as document → remembered → launch; flagged to Ev.
The profile-editor row is still in flight.

## 2026-09-19 — Ev's profile-editor row landed (PR 2862)

Orchestrated from the same session as PR 2858. This was not an
A/B-protocol unit, by Ev's instruction. The Fable lane hit the account
limit mid-work, and Opus finished the unit from its uncommitted diff.

It had a full review, correctness and style. The review found:

- **MAJOR:** the editor hid the slot rows, which were the only GUI door
  to driving an argument by an expression, per-slot units, and the
  range probe.
- **MINOR:** the greedy write order refused edits that had a valid
  order (7 of 52 in the reviewer's probe).
- **MINOR:** a disabled `DragValue` clamped a document's split-circle
  count to the authoring cap.

The fix pass fixed all three:

- the slot rows are kept, folded; the per-field version is filed;
- the write order is now an exact memoized search, capped at 12 writes;
- the clamp is off for document values.

It also:

- added the base-program guard in the op;
- merged the doubled preview pipeline into one;
- gave the lowering map one home;
- moved the editor into its own module;
- wired VGEOM's `except` seam.

The delta review held on all six checks. It ran on reading only,
because the build slot never freed in 1h45m. It found one new MINOR
(an outline freezes during a slot-row drag), filed rather than cycled.
Two NOTEs are recorded here:

- `ORDER_SEARCH_CAP`'s "fraction of a second" is unmeasured (worst
  case about 49k `apply` calls);
- the cap path's apply loop is a second copy of `commit_action`'s.

The shape lock waits on Ev (EDIT's whole-program-edit row).

## 2026-09-21 — the gesture concept gets a type

`vseam/gesture-naming` closed two rows on `session/op.rs`:
`the-two-drags-name-their-gestures-in-two-shapes` (P1, D) and
`op-rs-calls-the-panels-admission-test-its-own-copy` (P1, E).

The six driving operations kept their three payload spellings; what
they are spellings OF became `GestureName`, in the op vocabulary they
all belong to, with `ValueGestureName` and `FreeMoveName` as its two
halves. `SessionOp::names_gesture` is the one place an operation
becomes a name and is exhaustive over the enum, so an operation that
joins a drag cannot skip the question. The session's private
`GestureName` is gone; the value drag's two doors compare the public
`ValueGestureName` and keep their two-arm exactness.

The chrome half is where a convention became a type:
`GestureVocabulary`'s four fields are private to `widgets.rs`, and
`value_gesture` / `free_move_gesture` are the only way to build one, so
a panel spells its target ONCE and writes no operation. The three
`pane/properties.rs` sites are each one call now.

Order item 7 said this row waits on
`two-hand-written-copies-of-the-g1-gesture-machine`; that trigger had
fired — `crates/viewer/src/g1.rs` is on `main` at `d0bc79735f` — while
the row's own file still read `status: review`. Read `git log` before
believing a status, which is the register's own standing hazard.

Three rows landed with it, and one gap was filed. The gap is a
mutation that stays green: shifting the instance a free-move
vocabulary names, uniformly, passes the whole viewer suite, because
nothing reaches `ViewerBehavior`'s panel methods from a test. Filed as
`the-chrome-vocabulary-is-not-held-to-the-target-the-panel-draws`. The
hole predates the unit — `properties.rs` spelled `instance: node` four
times inline — so the unit narrowed it rather than opening it.

The prose row was a word: the Properties pane holds no copy of the
admission test, it calls `display::instance_check`, the same function
`drawn_targets` runs. The bullet's real claim — which document each of
the three asks — is untouched.

### The review of #2965, and the two mutations it should have caught

"Merge with corrections", three of them inside closed rows. The
strongest was a defect in the evidence rather than the code: the
census's *read-and-mint are inverse* arm is a **fixed-point** check,
not an identity one — it reads a name off an operation and checks that
minting from THAT name reads back the same — and its containment arm
compares variants, not payloads. So an idempotent wrong answer
satisfies both. A `names_gesture` whose slot arm returned
`RecipeNodeId(0)` for every slot drag passed the whole viewer suite.

The repair is an **anchor**: `a_name_is_the_payload_it_was_read_off`
mints from hand-written names and reads them back, so the round trip
starts somewhere `names_gesture` did not write. The first attempt at
this row asserted injectivity only and skipped the `i == j` case, so it
was green under the very mutation it was written for — caught by
running the mutation rather than by reading the row, which is the rule
about asserted-somewhere and asserted-here landing one level up, on a
row written to close a gap.

The second: a probe's typed arm — the keyboard's one-shot
begin/preview/commit — was still hand-written beside the constructor,
and naming another instance in it passed everything.
`free_move_gesture` mints it now, and
`the_typed_arm_names_the_gesture_the_drag_does` reads the names back
off it.

Two general shapes, for whoever writes the next evidence:

- **A round trip anchored at the function under test is a fixed point.**
  It answers *is this map idempotent*, which every constant map is.
  Anchor it at a value the test wrote.
- **A fix that mints a vocabulary owes the arms beside it.** The unit
  replaced the drag's four operations and left the typed triple
  hand-written one argument over — not a fresh instance of the defect,
  an adjacent one left standing, with the PR's prose claiming
  otherwise.

Also corrected: `properties.rs:399` -> `:392`, the one citation this
diff broke and its own census missed; the filed row's receipt said 15
`widgets::tests` rows where the command prints 6 (15 is `widgets::`,
which includes `field_tests`) and "four times inline" where the base
spelled the probe's target six times. The unreachability argument in
that row is now a proof rather than a citation: `ViewerBehavior` and
`instance_ui` are both `pub(crate)`, so no test in `crates/viewer/tests/`
can name them at all.

Swept while there: `session.rs`'s *"the panel's own admission test"*,
the nearest relative of the sentence the prose row calls false, in a
file this PR already edits. `widgets.rs`'s module doc universal now
carries the rule that produces its exceptions (a function taking no
`ui: &mut egui::Ui`) and their count.

**The alarm fired on a real arrival, in the merge that followed.**
AUTH-2's value-field rework landed on `main` mid-review and added
`SessionOp::SetParamUnit` and `SessionOp::SetParamText`.
`names_gesture` refused to compile (`E0004`) until both were answered,
which is the exhaustiveness property this unit exists to buy, bought by
a diff that had never heard of it. Both are direct edits and drive no
gesture. The same merge moved every gesture-vocabulary site; the five
that exist now — three in `pane/properties.rs` and two new ones in
`widgets.rs`'s `field_tests` — are all minted from a name, and
`GestureVocabulary {` has no literal outside `widgets.rs`.
