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

**And a real arrival is better evidence than a planted one.** The
dispatch asked for the alarm to be proved with a planted 43rd variant.
A planted variant proves the compiler is exhaustive, which was never in
doubt; what the charter's claim needs is that the alarm is SITED where
arrivals actually land, and only an arrival nobody arranged can show
that. `SetParamUnit` and `SetParamText` are that arrival. Generalises:
when a guard's claim is about where it sits rather than about what it
computes, a planted input tests the computation and says nothing about
the siting.

**A merge of two green branches can carry a finding neither one's CI
saw, and this program has no rule for it.** The AUTH-2 merge arrived
with two clippy findings that CI's own
`cargo clippy -p viewer --features app --all-targets -- -D warnings`
would have failed on — a `type_complexity` on the probe pair's return
and a redundant closure — and both were fixed in the merge commit here.
Neither side could have seen them: `type_complexity` fires on a return
type that only exists once this branch's `free_move_gesture` meets
main's re-shaped call sites, and CI runs on each head separately.
`merge-tree` answers *does this apply*, not *is the result green*, and
nothing in the gate answers the second question for a branch that has
not pushed its merge yet. The instrument that works is the one used
here: merge `main` in, then re-run the receipts on the MERGED tree
before pushing — which the register already says for a different
reason (a stale base) and which turns out to buy this as well. The
whitespace-run hit filed as
`work/author/fmt-re-minted-a-continued-literal-into-the-assertion-message`
arrived the same way and is the same shape, one step less severe: it
was green on both sides because no gate greps for it at all.

## 2026-09-21 — the plan recorded a wait, a count and a parked PR, all stale

Found by the review of #2965, which was asked to check whether VSEAM's
Order still named a wait that had ended. It did, and three more claims
beside it.

| claim in `plan.md` | what was true |
|---|---|
| Order item 7 *"waits on `two-hand-written-copies-of-the-g1-gesture-machine`"* | that row's PR #2672 merged 2026-09-15; `g1.rs` has been on `main` since |
| §Inbound *"**Four** rows in `review`"* | the list under it holds **five** |
| §Inbound *"their lanes are in flight"* | all five PRs had merged before the sentence was written |
| *"#2762 is parked on a ruling from Ev"* | the ruling landed 2026-09-17 and #2762 merged carrying it |

All four corrected. The rows themselves were closed by PR #2976.

**The shape is one shape.** Every one of these is a *status recorded in
prose* that outlived the event it described, and prose has no lint. The
row files have `status:` frontmatter that `work.py lint` reads; a plan's
sentence saying a thing is in flight is checked by nobody, so it stays
true-looking for as long as no one re-derives it.

It compounded in a specific way worth naming: the cut (#2806) declined
to re-home these five rows **because** they read `review`, so a stale
frontmatter field became a stale plan sentence became a program's
schedule waiting on a trigger that had fired. Three artefacts wrong from
one un-updated field.

**What would have caught it.** `work.py lint` can see a row whose
`status:` is `review` while its `pr:` is merged — that is a query against
the tracker plus the forge, not a judgement — and it is the check that
would have fired six days ago. Filed as
`work/meta/a-row-in-review-whose-pr-has-merged-is-lintable.md`; META owns
`scripts/work.py`, so this program reports it rather than writing it.

## 2026-09-21 — two rows in from VIEW, on the charter test

VIEW is winding down and does not dispatch; Ev approved emptying its
slate. Two of its rows are this program's by §Charter, moved by
`git mv` with ids, bodies and history unchanged.

- **`face-selection-carries-a-bare-stable-name` (P1, D).**
  `session/select.rs`'s `FaceSelection` carries `pub name: StableName`
  — a face by its door's rule (the picking door refuses
  `SelectionRefusal::NotAFace` before a selection exists) and not by
  its type. So `matetool.rs`'s `picked_member` calls `FaceName::new`
  on a name the picker already proved is a face, and answers
  `MateToolError::PickIsNotAFace` for a state the door forbids. Filed
  by EDIT's `edit/mate-head-kind`, which made a mate head's entity
  kind a type one layer in. Both files are this program's —
  `matetool.rs` came here in #3003.

- **`startup-notices-need-holding-to-badge` (P1, D).** `ViewerApp::new`
  renders the preferences store's load notices once into the status
  line and drops them, so the first batch the user acts on clears the
  sentence with no way back. **VNEWS's charter excludes it in as many
  words** — *"nothing it touches survives the frame that produced
  it"* — and this row's whole content is that the notices must be
  HELD. Its blocker is a lifetime question: a preferences notice is
  true until the file changes, nothing in the viewer watches the file,
  so a badge would stand for the session with no event able to retire
  it. That is this program's subject, not a wording one.

Neither is dispatched. `startup-notices` is a design question before
it is a unit; `app.rs` is shared with CHROME, AUTHOR and VGEOM, so
its seam is announced when it goes out.

## 2026-09-22 — one row in from VIEW, and its stated reason to wait is spent

`opoutcome-committed-doc-says-at-most-one` (P4, E) moved by `git mv`,
id, body and history unchanged. Filed onto VIEW's slate by AUTHOR's
AUTH-3 lane on 2026-09-21; VIEW does not dispatch, and `plan.md` §What
is left of this slate says a row landing on its ground goes to the
successor whose charter covers it.

**Why here and not VDOC**, since the fix is one doc sentence and
changes no viewer behaviour, which is VDOC's charter exactly: VDOC's
`paths` are `crates/viewer/README.md`, `crates/viewer/tests/*` and
`crates/viewer/src/lib.rs`. **`session/op.rs` is not among them and is
in this program's**, so a VDOC lane could not edit the file its own fix
needs. Territory decides where charter cannot.

**The row's reason for deferring is already spent.** It says *"the row
beside it (`opoutcome-superseded-has-no-production-reader`) is already
reading this type's fields … taking both in one sitting is cheaper
than taking this alone."* That sibling is **closed** on `main`. So this
is a lone one-line unit now, not half a pair — worth knowing before
someone schedules it expecting company.

The finding itself is sound and was checked: `OpOutcome::committed`'s
doc says *"at most one per op"*, while `DocSession::delete_node`
commits one `DocEdit::DeleteNode` per doomed node through
`commit_action` and has answered with several for as long as the
cascade has existed. AUTH-3's `ProfilePlane::NewXy` added a second
such door but did not make the sentence false; it was false before.
Nothing reads it as a bound — the row records that the twenty-one test
sites assert a length rather than assuming one.

## A row from EDIT (2026-09-29) — `layer3-recipenodeid-aliases-across-rewinds`, re-homed by `git mv`

EDIT re-homed this row to your slate because VIEW left the tracker and
your program claims the file it lands on. The work left is the layer-3 rewind walk over `history.rs` and the session state, reading EDIT's `Doc::has_minted`.
EDIT's side (the door it reads) is built. Id and priority kept; the
row's last section says what moved.

— EDIT orchestrator


- 2026-09-30 — Seam note from AUTH-9 (`author/declared-union`). The boolean door now evaluates the boolean it is about to record synchronously, outside the seam (`session::probe::evaluate_with`, now `pub(crate)`, with the landed run as the memo), and records it only if it does not refuse an undeclared contact of its own; the seam then evaluates the recorded document as before, so a committed boolean is evaluated twice. Filed as `work/author/the-boolean-door-evaluates-its-boolean-twice.md`; either fix touches the seam's landing contract. `combine.rs`, `tools.rs` (module doc only) and `session/probe.rs` changed. (AUTH-9 implementer)

- 2026-09-30 — Seam note from AUTH-9's fix pass (`author/declared-union`, PR #3543). `evalseam.rs` now holds the one spelling of a session run's options (`options`) and the evaluation taken beside the seam (`evaluate_beside`), which replaces `session::probe::evaluate_with`. `same_resolver` is `pub(crate)`. The session records the resolver of each request and its landed run (`requested_resolver`, `LandedRun::resolver`), and primes the boolean door's judge and the range probe only from a landed run under the same resolver (`DocSession::memo_under`). The probe now resolves through `NoFile::seam()` like the seam does, instead of `None`. (AUTH-9 implementer)

- 2026-09-30 — Seam note from AUTH-10 (`author/held-face-mark`, PR 3556). A pick a form or tool HOLDS has a mark of its own: the selection's colour (`Theme::held`), told from the live selection by shape (stripes on a face, a hollow line on an edge). `matetool.rs`: `MateToolState::picks`, which `line` now reads. `blend.rs`: `BlendTool::mark_segments` is gone; `BlendTool::held_edges` hands the set to `marks::HeldEdges::segments`, which holds the one-pass walk. `pane/viewport.rs`: `frame_marks` (in a sealed `composed` module) gathers every holder — `Drafts::held_face`, the mate tool, the blend tool — and is the only door that mints `Composed`; `drawn_index` is `pub(crate)` for the gate. `pane/create.rs`: the add-datum gate is `session::face_frame_seat_drawn` over the on-screen index, refusing a held face the picture does not draw (`FaceFrameFault::NotDrawn`); the form's face line reads `held_face`; the add-profile form's withholding reasons are renamed `Withheld` (`withheld_for`, `withheld_line`, `bore_withholds`) so "held" means a held pick. `session/refuse.rs`: `FaceFrameFault::NotDrawn` and `face_frame_seat_drawn` (re-exported from `session.rs`). `pane/profile.rs`: one comment says "withheld". `app.rs`: a committed `Open` or `NewDocument` drops the add-datum form's face latch (`Drafts::document_replaced`); `properties_pane_tests` gains `cap_of` and `clicked` (the mate panel row and `painted_with_tool` now use them) and rows for the held face, the consumed-body refusal, the document replacement, the mate picks and the blend lane; `held_reason_said` is `withheld_reason_said`. (AUTH-10 implementer)

- 2026-09-30 — Seam note from AUTH-11 (`author/binder-prefix`, PR 3563). An unfinished chain whose tip is unclosable (no `line_to` leaves it, so the provisional close is ill-typed) now draws the prefix `sketch::prefix_loop` walks back to, and the form says that tip's end-of-program refusal, advisory. `sketch::LoopEnd` is now `Closed | Unfinished(Option<Cut>) | Refused(Cut)`, where `Cut { refusal, closes }` is shared, and `LoopEnd::unclosable()` reads an unfinished chain's cut; `PreviewHold::Refused` is renamed `PreviewHold::Refusal` and also carries an unclosable tip's refusal. `pane/profile.rs`: a new pane row for the sentence; `preview_verdict` is unchanged. `pane/viewport.rs`: `push_preview`'s tip-mark match is re-spelled over `Cut`, and two new rows paint an unclosable tip's prefix. (AUTH-11 implementer)

- 2026-09-30 — Seam note from AUTH-12 (`author/tool-census`, PR 3573). The nine tool activation buttons in `pane/create.rs` take their words from `ToolKind::button()` (`tools.rs`: `label()` capitalised, with the ellipsis) instead of literals, and a whole-app row (`app::properties_pane_tests::every_tool_opens_from_its_activation_button`) clicks each kind open by those words. A new tool panel places its kind in that row's section match, and a panel call dropped from `create_ui` or `properties_ui` reddens it. `pane::create::EXTRUDE` and `ADD_PART` are the extrude form's and the part chooser's button words, held by `the_extrude_form_and_the_part_chooser_are_reachable`. Painted text is unchanged.

- 2026-09-30 — Seam note from AUTH-12's fix pass (`author/tool-census`, PR 3573). `ToolKind::label()` (`tools.rs`) now returns the BARE noun ("mate", …, "projection"). `says` adds " tool: ", `button` capitalises and adds " tool…", and the new `ToolKind::commit()` gives "Commit <noun>". A new reader of `label()` gets the noun, not "<noun> tool". `tool_commit_row` lost its `label` parameter and reads `kind.commit()`; the mate and blend commit buttons read `ToolKind::{Mate,Blend}.commit()`. Painted text is unchanged. `pane/create.rs` gains consts for the three section headings (`ADD_FEATURE`, `COMBINE_BODIES`, `BLEND_EDGES`) and the form buttons (`ADD_DATUM`, `ADD_PROFILE`). `pane.rs`'s `headless` doc now sends a row that must drive a pane METHOD to `app::properties_pane_tests`' whole-app harness.

- 2026-09-30 — Seam note from AUTH-13 (`author/geometry-close`, PR 3579). An unfinished chain whose provisional close is refused now draws the legs written: it walks back through the one `sketch::prefix_loop` call the other two arms take and ends `LoopEnd::Unfinished(Some(Cut))`, carrying the close's own refusal (`PreviewError::Geometry`, which now carries a typed `kind: PathErrorKind`), or the end-of-program refusal where the close is ill-typed or the last leg itself closes the loop. `PreviewHold::Unfinished` is the new advisory hold for such a cut, and `LoopEnd::unclosable()` is renamed `LoopEnd::unfinished_refusal()`. The loop's start is read off the entry (`sketch::loop_start`), and the provisional close is `line_to Start`, re-spelled where the lattice refuses it as tangent (`sketch::replay_provisionally_closed`). `pane/profile.rs` and `pane/viewport.rs`: new test rows only; no production change on those two files. (AUTH-13 implementer)

- 2026-09-30 — Seam note from AUTH-14 (`author/edge-name-fault`). The index's loud edge-name arms now reach the author. `pickindex.rs`: `PickIndex::edge_names_in(node, body)` walks a body's drawn edges and answers `EdgeNames { named, refused }`, where `EdgeNamesRefused { first: EdgeNameFault, refused, drawn }` renders through the fault's own `Display`; every refusal from that walk is loud (the ids come from the index's own window), and a body the index does not draw answers nothing and refuses nothing. `EdgeNameFault` gains `Eq`. Test doors: `PickIndex::unname_edge` (cfg(test)) plants the naming layer's refusal; `PickCache::index_mut` (cfg(test, app)). `blend.rs`: `load_all_edges` refuses the whole load with `BlendEvent::EdgesUnnamed { target, refused }` when any drawn edge refuses (was `NoEdgesOnTarget` when all did, a partial set when some did). `marks.rs`: `HeldEdges::segments` is `HeldEdges::mark`, answering the segments and the refusal; `EdgeOverlay::held_refused` carries it. `pane/viewport.rs` writes it per frame into `ViewerBehavior::held_edges_refused`; `app.rs` zeroes/assigns `ViewerApp::held_edges_refused` like `profiles_undrawn` and draws `frame::held_edges_badge` (Advisory, the pick-index seam's subject; `impl SeamSubject for EdgeNamesRefused`). `frame.rs`: `tool_notice` answers `EdgesUnnamed` `Retold::Again`. `gpu.rs`: one test literal gains the field. `test_support.rs`: `plate_indexed(tol)` (the fixture `marks.rs`'s tests held privately). README: the badge population is eleven. Tests: `frame_policy.rs` (badge count 11, the new retold row), `error_display.rs` (`edge_names_refused_forwards_its_first_refusal`), `blend_authoring.rs` (the held-set row reads `mark`). (AUTH-14 implementer)

- 2026-09-30 — Seam note from AUTH-14 (`author/edge-name-fault`), review fixes; supersedes the shapes in the note above. `pickindex.rs`: `EdgeNamesRefused` is `{ node, body, first: UnnamedEntity, named, refused }`, rendered as "the index names N of the M edges it draws on body B of node K; the first it cannot: <EdgeNameFault::Unnamed>"; `edge_names_in` reads the window through `PartWindows::named_in`, so only the unnamed arm can refuse; `EdgeNameFault` does not gain `Eq` after all. `blend.rs`: `BlendEvent::EdgesUnnamed { refused }` (no separate `target`). `frame.rs`: `SeamSubject for EdgeNamesRefused` is `Subject::Document` (the line's `tool_notice` subject too), not the pick-index seam. `test_support.rs`: `unnamed_edge(node, body)`. Filed `work/chrome/the-per-frame-badge-reads-are-three-hand-copied-fields`. (AUTH-14 implementer)

- 2026-09-30 — Seam note from AUTH-14 (`author/edge-name-fault`), third pass. `pickindex.rs`: `PartWindows::in_target` and `named_in` both cut their run through the new `PartWindows::laid_out`, which panics (`unreachable!`, naming the window and both list lengths) when a window runs past the entities or names laid out with it, instead of answering an empty run that reads as a body with no edges; `a_window_past_its_names_is_not_an_empty_body` is its `#[should_panic]` row. (AUTH-14 implementer)

- 2026-09-30 — Seam note from AUTH-15 (`author/accept-part-version`, PR #3591). A new `SessionOp::AcceptPartVersion { id }` commits `pncad::workspace::update_to_store`'s edits (one `DocEdit::UpdateReference` per site whose pin moves) as one action. The store is read through a new `DocSession::read_store`, which `add_instance` and `part_catalogue` now share; `parts::catalogue` takes the scanned `&Workspace` and is infallible. `frame::version_offer(kind, files)` sits beside `declare_offer` and reads a `session::VersionOffer` off an instance's own `PartFault::Unresolved { fault: PinMismatch }`. `tree::TreeRow` gains `version_offer: Option<VersionOffer>` (every `TreeRow` literal needs the field). `DocSession::tree_rows` withholds every offer while `busy()`. `pane::features::feature_row_ui` draws `Refusal::version_question` and a `VersionOffer::LABEL` button under the row (`RowClicks::accept`). `DeclareOffer::ACCEPT_LABEL` and `DECLINE_LABEL` replace `pane::create`'s "Declare"/"Decline" literals. Every exhaustive `SessionOp` table gains the arm (`tools.rs`, `frame::acts`, the three in `session/op.rs`, and `tests/gesture_table.rs`, whose `OP_COUNT` is 47). `test_support::{PART_FILE, part_refused}` are new fixtures. Rows: `tests/instance_authoring.rs` (accept, one undo, the store's refusals, the committed-document read), `frame`'s `PartFault` census, the label-versus-recourse row in `session::refuse`, and a `pane::features` unit test.
