# AUTHOR — the log

## 2026-09-20 — opened

Cut out of CHROME, which was carrying 65 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut was made on CHROME's PRIORITY seam per `work/README.md` "Track
size", and the seam was unusually clean: ten of its rows are doors the
viewer does not have, and they are the GUI half of the standing goal
Ev put at P0 — authoring arbitrary geometry through the UI.

Ten rows arrived by `git mv` with their ids, bodies and history
unchanged. Six prose rows of the same cut went to VDOC, whose charter
already covers claims the tree makes about itself; CHROME keeps its
reported defects, its entrenching architecture and its band 1600-1699.
Band 6800-6899 claimed in this commit (`docs/MODEL-AB-LOG.md`).
Nothing dispatched.

## 2026-09-21 — AUTH-1 dispatched; the posture answered

**First dispatch.** `add-profile-placement-on-picked-face-frame` went
out as AUTH-1 (`docs/AUTH-1-SPEC.md`, branch
`author/face-frame-seat`): the sixth `DatumSpec` seat and the
add-datum form arm that fills it, so a person can pick a face and mint
the `Datum::FaceFrame` the whole placement path already admits.

**One unit, not two** — the question `plan.md` left open. The
evidence pushed the two rows apart rather than together. The seat is
`crates/viewer/src/session/author.rs`, `forms.rs`, `drafts.rs` and one
arm of `add_datum_ui`; the sibling
(`add-profile-mints-no-frame`) is the add-profile form's frame picker
and a `commit_action` gesture, and neither is load-bearing for a face
frame existing. Folding them would have put two forms in one lane's
diff for no part of the wall. The sibling runs next, better informed:
its half 2 wants a frame's honest label, and a face frame's honest
label is the face it sits on, which is not writable until a face frame
can be minted.

**The residue is stated and scheduled, not disclosed in prose.** After
AUTH-1 the gesture is two forms — add the datum, then draw on it —
and the one-gesture version is exactly the sibling row, which already
exists on this slate. Nothing new to file.

**Review posture answered** in `plan.md`: CHROME's inherited posture
adopted unchanged, with the second correctness arm made this
program's DEFAULT rather than its exception, because every row here is
a door whose failure mode is a model quietly unlike what the author
drew. No A/B row — the account's Fable budget is spent for the week
(Ev, at hand-over); band 6800-6899 stays claimed and unspent.

**Seam.** AUTH-1's files (`session/author.rs`, `forms.rs`,
`drafts.rs`, `pane/create.rs`) are claimed by AUTHOR and CHROME; no
open PR touches them. PR 2929 (`dup/viewer-shared-doors`) is live on
`crates/viewer/tests/common`, which AUTH-1's tests consume — the brief
carries the re-merge obligation.

## 2026-09-21 — AUTH-2 dispatched, in parallel

`parameter-row-field-has-no-text-door` went out as AUTH-2
(`docs/AUTH-2-SPEC.md`, branch `author/param-notation`), with
`add-parameter-form-authors-canonical-only` riding it.

**Two rows, one unit, because the rows said so.** The carrier's own
`## Its relation to the sibling item` reads *"the design call about
how notation crosses `props` is shared"*. A shared design call settled
twice is how two doors drift apart, which is the class this whole
program keeps finding. Recorded as `rides_with` rather than folded in:
the create door's finding has its own evidence and closing the carrier
does not close it (`work/README.md`, rides-along).

**The parser question, settled at dispatch.** A parameter field that
accepts `50 mm` needs to read a unit-bearing number, and
`editor_core::parse::parse_expr` already does — it is what the slot
field's expression door uses, it derives the dimension from
`UnitSym::measures`, and it refuses an unknown symbol naming the token
and its offset. The spec forbids a second unit table in as many words.
The hazard the reviewers are pointed at is the other side of that:
`parse_expr` already multiplies by the unit factor, so a chrome that
multiplies again ships a parameter scaled twice, invisibly in metres.

**Value and notation are ONE undo.** `50 mm` is a `SetDocParamValue`
and a `SetDocParamUnit`, through `DocSession::commit_action` — the
same door `crates/editor-core/REFERENCES.md`'s DM1 bullet named for
the face-frame gesture. Two ops pushed and hoped over would be two
undo steps for one keystroke.

**Why in parallel with AUTH-1.** The file sets are disjoint but for
`drafts.rs`, and additively there. The build slot is a machine-wide
mutex (`memories/agent-lane-operations.md`, width 1, measured), so two
lanes serialize their builds rather than starving each other, and
hosted CI runs per branch regardless. Both briefs carry the seam and
the re-merge obligation.

**A plan correction, made where it was found.** `plan.md` said both E
rows were drive-bys "for whoever is next in `forms.rs` and
`props.rs`". `a-negative-extrude-distance-probes-as-valid` is in
neither — it is `crates/viewer/src/bounds.rs` and
`crates/viewer/tests/valid_range.rs` — so it does not ride either live
lane and now says so.

**Stale citations fixed in passing**, not filed: three
`work/chrome/…` paths on rows that moved to `work/author/` at the cut,
and one `work/issues/doc-param-unit-edit-has-no-door.md` that was
claimed by EDIT and has since closed.

## 2026-09-21 — AUTH-1 landed green; two reviewers out

PR https://github.com/evgunter/cad/pull/2955, head `9b06ab941`, CI run
35551509615 green at 39 jobs — twelve `test (…)` across
{default, interval} × {default ε, 1e-6, 1e-12} × two shards, and all
five `k-lint (gate, …)` unifications, so nothing was narrowed. Open,
not merged; the correctness and style lanes are running.

**The deviation worth the program's attention.** The spec's
`Ambiguous`/multi-body hypothesis was WRONG about the tree, and the
lane said so rather than building on it. `face_carrier_kind` cannot
answer `Ambiguous` for multi-bodiness — a name's ENTRY carries its own
output-body index — so the tag read alone would have ADMITTED a face
picked on a split half. What a multi-body `at` actually breaks is
`eval::wire`'s `body_operand`. The lane carried
`FaceFrameFault::NotOneBody` over `combine::denotes_body` and also
gated `at` at `DocSession::add_datum`, so bypassing the form does not
bypass the rule. **That second gate sits on the shared `add_datum`
path**, which is why the correctness reviewer is asked whether it
narrows a datum kind that used to work — a fix that lands on a shared
door is the shape that costs someone else something quietly.

**The dispatcher's hypothesis was the thing that was wrong**, which is
the outcome `reviewer-style-lane.md` names ("the dispatch is a
hypothesis") and implementer-discipline invites. Worth saying plainly
in a log that will be read by whoever writes the next spec: the spec
asked the lane to *decide and say* rather than asserting the answer,
and that phrasing is what made the correction cheap.

**A residue that reaches another row.** With no `AddPart` op, a face
picked on a split half or a pattern instance can never be given a
frame from the GUI at all — so `NotOneBody` is a second ordinary
gesture blocked on the missing door. Evidence went onto the EXISTING
`work/author/viewer-cannot-author-a-part-node.md` rather than a second
row, which raises that row's value when it comes up in the order.

**Disk and lanes.** AUTH-1's build target was reclaimed the moment its
report was in hand (89% → 66%); review lanes are the biggest consumers
and both were told to build only when a claim needs running.

## 2026-09-21 — the scratchpad collision, adjudicated not re-opened

AUTH-1 reported that its CI poller in the session scratchpad was
silently overwritten by AUTH-2's, and that for one round it read
AUTH-2's job counts believing they were its own. It caught it and
re-verified.

`work/meta/lane-scratchpad-is-shared-between-worktrees.md` already
holds this finding, `deferred` on Ev's 2026-09-11 ruling, and states
its own re-opening condition: a lane that LOSES work re-opens it, a
third instance that is CAUGHT changes nothing. This one was caught, so
**the deferral stands and no `[ev]` PR was opened** — the tracker
answered the question without one. Evidence appended to that row
rather than a second row opened, with the two things about it that are
new: the blast radius is larger than the row's two instances (this one
fed a lane's reading of hosted CI, the verification of record, not a
draft), and the row's unverified "per-session or wider" is now
half-settled — both lanes were one session's, so the cross-session
case is still untested.

Shape 2 is in force here as the orchestrator convention it always was:
every AUTHOR lane gets a lane-private scratch path at dispatch. AUTH-2
was told mid-flight and asked to re-verify its own green against its
head SHA rather than against a file it wrote earlier.

## 2026-09-21 — AUTH-1's style lane, adjudicated

Nineteen findings, none rising to a MAJOR on correctness in the
reviewer's reading. The correctness lane is still out; the fix pass
waits for it so it answers both reports at once.

**The three the fix pass takes first, and why they are one argument.**
All three are the same defect wearing different clothes — *the unit
computed a thing twice and used one copy*:

- `face_frame_seat` returns `Ok((at, face))` and `Drafts::datum_spec`
  independently re-derives the identical pair, so **the button is
  gated by one computation and commits the other**; in `src/` the `Ok`
  half is dead and only tests consume it.
- `FaceFrameFault::NoFace`'s `Display` and
  `DatumKindChoice::unmet_seat`'s `FaceFrame` arm are the same string
  literal byte for byte in two modules, and the form deliberately
  suppresses the first — so the `refuse.rs` copy is **prose the only
  renderer never draws**, which is the drift shape with nothing that
  can catch it.
- a bare `format!("feature {} body {}", …)` re-mints
  `Display for BlendTarget`, whose doc argues at length for one home —
  in a file that **already imports `BlendTarget`**.

**This is the fresh-instance trap, and it landed exactly where the
brief says it lands.** AUTH-1's subject was replacing a hardcoded
per-kind sentence with a roster; the same diff added a hardcoded
per-kind sentence twelve lines away and two hand-maintained
derivations. `docs/prompts/reviewer-style-lane.md`: *"Naming the trap
in the PR body does not prevent it — only a reader who did not write
the fix has ever caught it."* The PR did not name it; the reviewer
caught it anyway. The lesson for this program's next dispatch is to
put the trap in the SPEC, not to hope.

**Two prose falsifications land with this change, not later**
(CLAUDE.md: a clause re-worded because an approved code change moved
what it describes is not a second decision): `crates/viewer/src/datums.rs`'s
*"The two differ by exactly `AxisInPlane`"* — now also `FaceFrame` —
and `add_datum_ui`'s own header, *"Every kind but one is numbers
alone"*, when two kinds now need picks.

**The sweep receipt is not accurate as written**, and the fix pass
re-does it. Its consumer row missed `datums.rs`'s `DatumKindChoice`
mention, which is one of the two sentences above; and its own
blind-spot paragraph names `ui.label` as unmatched and then nobody
sweeps it, which is where the third per-kind sentence was hiding. A
disclosed blind spot never swept is the sweep claiming less than it
needs to.

**Held for the correctness lane**: the latch's doc asserts *"nothing
else clears it"* and *"the arm writes it whenever the live selection
is a face"*, both true only while that arm is being RENDERED — inside
a collapsible section. That is item 3 of what I asked the correctness
lane to push on, and whether it is taste or a defect is that lane's to
say.

**Filed outside the fence, four rows** (none AUTHOR's to take):
three on CHROME — `a-fifth-spelling-of-this-seat-is-empty` (P1),
`four-pick-state-vocabularies-in-one-create-module` (P1),
`a-per-kind-sentence-lives-in-the-widget-not-on-the-choice` (P4) —
and one on VDOC,
`viewer-readme-recourse-count-does-not-say-what-it-counts` (P4).
Seams announced on both programs' logs.

**One row DISCHARGED outside the fence.** VDOC's
`add-profile-ui-doc-comment-states-a-premise-the-tree-falsified` asked
for exactly the rewrite AUTH-1 made, down to its last sentence about
what the deferral note should point at. Set to `review` on PR 2955 and
announced on VDOC's log; it closes when 2955 merges. Worth noting that
neither the spec nor the lane found that row — the style review did,
by reading the comment rather than the diff.

## 2026-09-21 — AUTHOR's territory was wrong, and is fixed

The opening `paths` named `forms.rs`, `props.rs` and `sketch.rs`, and
**not one of them is where either of the first two units did its
work**. Every row on this slate is a creation or property FORM, which
live in `pane/create.rs` and `pane/properties.rs`. So
`work.py territory --base main` could not have warned either lane
about the ground it was on — the one thing that list exists for.

Widened (not swapped) to add `drafts.rs`, `pane/create.rs`,
`pane/properties.rs`, `session/author.rs`, `session/refuse.rs`. All
five are shared with CHROME and, per file, with VIEW, VNEWS, VSEAM or
VGEOM; shared ground is legitimate and what is owed is awareness while
a lane is live (`work/README.md`, 2026-09-20). Seam announced on
CHROME's log with both live AUTHOR branches named. No `keep_out`
written, because nothing about the overlap needs explaining.

A file-list correction with no design implication is not an `[ev]`
question (Ev, PR 1916: *"you don't need to ask me about moving things
around"*); it is logged instead.

## 2026-09-21 — AUTH-1's correctness lane, adjudicated; fix pass out

**MERGEABLE-WITH-FIXES.** C1–C7 all survived, four of them RUN rather
than read, and the reviewer broke one thing by running it. Twelve
fixes dispatched on the branch (F1–F12, covering both reviews); I
merge when it is green.

**The one that matters, and it is the spec's own clause failing.**
`combine::denotes_body` is a NODE-KIND predicate; the evaluator's
`body_operand` is a VALUE predicate. A `Node::Transform` over a
`Node::Pattern` is a `denotes_body` node whose value is `Instances`,
so the gate answers `Ok`, the session door raises nothing, the
`Datum::FaceFrame` LANDS, and the evaluator refuses
`WrongOperand { expected: "body", found: "instances" }`. That is
exactly *"mint a node that refuses later"*, which the spec told the
unit not to do — reachable through `SessionOp::Open`. Not a MAJOR
(the kernel still refuses loudly, no wrong geometry), but it fails the
unit's own contract, so it gates. The fix is to test the landed VALUE
in a function that already holds the evaluation.

**The deviation was right and not exact, which is a distinction worth
keeping.** The lane's correction of my `Ambiguous` premise was
verified from kernel source by the reviewer, independently. What the
lane then built on it — `denotes_body` as "the viewer's existing
statement of that same operand door" — is an equality that is not
one, and `denotes_body`'s own doc names two directions it differs
while this is a third. **A lane that correctly refutes a bad premise
can still inherit a bad premise one level down**, and only a second
reader found it.

**Three findings are one argument with the style lane's three**, which
is what running two lanes buys: the dead `Ok` half of `face_frame_seat`
(correctness NOTE-7) and the double derivation (style S4) are the same
defect, and fixing it also fixes the test that could not discriminate
`face.node` from `face.feature()` (correctness MINOR-4). Neither lane
alone would have ordered the fix that way.

**A refusal sentence that lies, found by reading the reachable
states**: `doc.node(at).is_some_and(..)` conflates "the node is gone"
with "the node is several bodies", so undoing the feature a latched
face belongs to tells the author to "project the one you mean first".
Reachable by exactly the case the PR leans on to argue the latch is
safe.

**Filed outside the fence, two more rows on CHROME**:
`denotes-body-enumerates-its-gaps-against-the-operand-door-and-misses-one`
(P1 — an enumerated list that is short is a false claim, and AUTH-1 is
the first caller to make the predicate load-bearing as a gate) and
`a-creation-forms-held-pick-survives-a-document-swap` (P1 — `Drafts`
is app state nothing resets, `StableName` carries no document
identity, and the pre-existing `datum_frame` hold has the same hazard,
so AUTH-1 corrects its doc and leaves the behaviour to the class).

**Recorded on the next unit's row**: the held face pick renders as
`"feature N body M"` — the same text for all six faces of a box — so
`add-profile-mints-no-frame`'s half 2 now has a third site, and its
taker closes three rather than discovering the third afterwards. Two
stale "unowned residue" lines on that row fixed in passing.

**What I am NOT having the fix pass do**, said here because a scope
kept is only visible if it is written down: clearing drafts on a
document swap (the class row above), the two `create.rs` vocabulary
classes, moving AUTH-1's own per-kind sentence onto the choice (the
PR says it is a knowing instance of the CHROME row instead), and the
honest face label (the next unit's). The N5 ladder's rung 1 having no
counterpart in `interrogate::entity_of` stays a NOTE — the reviewer
could not reach it.

## 2026-09-21 — AUTH-2 landed green; two reviewers out

PR https://github.com/evgunter/cad/pull/2957, head `a44ca680b`, CI run
35552454126 green at 39 jobs — twelve `test (…)` and five
`k-lint (gate, …)`, nothing narrowed. Open, not merged.

**The lane verified its own green the way it was asked to**, after the
scratchpad warning: it re-derived the head SHA fresh and confirmed the
run's head equalled it, rather than reading a job count back out of a
file it had written. That is the difference between a green earned and
a green indistinguishable from one.

**The decision I am putting the hardest review on: the no-op guard was
WIDENED, not just shared.** The lane gave both fields one guard
(`props::typed_edit` over a new `readout::reads_as`) which judges what
the field would SHOW after the edit against what it shows now — and
the pre-existing SLOT field's guard was an exact value comparison. The
lane discloses the cost in the PR: the chrome can no longer tell an
echoed text from a re-typed one, so a user typing a number within the
render's own accuracy of what the field displays gets no edit.

That is a behaviour change to a door this unit was not asked to touch,
traded for a different door's guard, and it deserves the question the
reviewer is asked: **an echo is a value the field itself produced; a
re-type is a value the user produced.** A guard that cannot tell them
apart may be solving the wrong problem, and if it discards a genuine
slot edit it is a MAJOR. Q1's pull toward one home is real and this
project's standing defect is duplication — but "one home" is not a
reason to make an exact test lossy, and adjudicating that is mine, not
the lane's.

**The spec was wrong on a line again, and the lane said so.** `base_r
* 2` never reaches the new door: `2` is a count and the expression
vocabulary refuses count × length without an explicit promotion, so
the user reads the parser's sentence about the multiply.
`base_r * 2.0` and bare `base_r` are the texts this door answers for,
and the test asserts all three. Second dispatcher premise falsified in
two units; the pattern is that my specs assert what the tree does
where they should ask.

**The seam narrowed to nothing.** `drafts.rs` was not touched at all —
the form reuses the crate's single form-notation drafts rather than
minting new fields — so `origin/main` merged clean against AUTH-1.
What remains is `gesture_table.rs`'s hand-maintained op-index census:
AUTH-2 takes 42 and 43 and raises `OP_COUNT` to 44, and AUTH-1 adds no
`SessionOp` at all, so the two cannot collide. The reviewer is asked
whether that census can still go red.

**Filed outside the fence by the lane**: a new CHROME row
(`bounds-reading-respells-the-panels-one-divide` — `BoundsReading::wording`
hand-writes what `props::shown_in` exists to prevent, and that
function's own doc names "a hand-written `map_or` at each" as the
thing it prevents), plus evidence onto two VGEOM rows, one of which
asks for exactly this unit's guard generalised.

## 2026-09-21 — AUTH-2's style lane, adjudicated

Twenty-one findings, one of them handed straight to the correctness
lane as a probable MAJOR and forwarded there while it was still
running.

**S2, the forwarded one: the parameter field may emit TWO ops for one
typed number.** The `Selection::Param` arm still calls
`widgets::drag_ops`, whose typed arm pushes `SessionOp::SetParam`
unconditionally, and the unit ALSO added a `match typed.into_inner()`
dispatch that pushes `SetParam` again through the new guard.
`slot_value_ui` uses `drag_gesture_ops` — the drag half only —
precisely so its own dispatch is the sole emitter; the parameter arm
kept `drag_ops`. If egui reports `changed()` on a parser-produced
value, C5's "no op at all" is FALSE at the panel and a typed bare
number costs two undo steps, with `typed_edit` still correct as a pure
function. **Nothing in the suite would go red either way**:
`panel_edits` drives ops directly and `typed_edit` is tested pure, so
the panel's op emission is untested ground.

**S3 falsifies the PR's headline.** The no-op rule has THREE
spellings now, not one: `props::typed_edit` (tolerance, on the
render), `slot_value_ui`'s expression arm (exact, on the source text),
and `DocSession::set_param_text` (**exact f64 equality**, on the
canonical value) — whose doc calls its guard *"the same rule …
(`props::typed_edit`)"* when it is the exact comparison `typed_edit`
was written to replace. A Q2 reconciliation comment that is false is
the strongest evidence the rule has no home.

**The fresh-instance trap again, in its literal form.** S1:
`param_unit_ui` is a third hand-rolled copy of `widgets::pick_unit` —
same options call, same early return, same salt shape, same
`.width(72.0)` — in a file that already imports from `widgets` in this
very diff. Its sibling `slot_unit_ui` has a real difference (the
`mixed` label, the vector fan-out); this one has none. So both units
this sitting closed a duplication and minted one. That is now a
pattern in this program and not an accident, and the next spec says so
in the brief.

**I was wrong about one thing and the reviewer corrected me.** My
brief said the disclosed echo/re-type cost carried no schedule. It
does: the lane put it on
`work/vgeom/a-fields-text-commits-within-the-renders-own-tolerance.md`,
an open item file, which is what Q6 asks for. The caveat the reviewer
kept is the real one — it sits on VGEOM's slate under a different
question, so AUTHOR's trade is scheduled somewhere AUTHOR does not
read.

**Held for the correctness lane**: S15, that the guard's rows cannot
go red when the guard DEGRADES (pinned inside the band and at
`showing * 2.0`, nothing at the boundary — widen `REL_TOLERANCE` by
three orders and every row still passes). Asked to be verified by
widening the constant and re-running, not by reading.

## 2026-09-21 — [ev] PR 2974: sweep the blind spot you just named

**The one thing this sitting found that binds future work.** Both
units wrote §5 receipts of unusual quality, and in BOTH the stated
blind spot is exactly where a real finding was — AUTH-1's `ui.label`
gap held three per-kind sentences, one of them its own diff's;
AUTH-2's "a `match` on `Dimension`" gap held six ladders, three of
them its own. Two lanes, two independent reviewers, no contact. Each
receipt's counts were re-derived and HELD, so the sweeps were not
sloppy — the rule got what it asked for, which is a statement and not
a second pass.

`docs/prompts/` binds every lane by path, so it waits for Ev
(CLAUDE.md). PR https://github.com/evgunter/cad/pull/2974, item
`work/meta/a-stated-sweep-blind-spot-is-never-swept.md` with
`needs_ev: true`, subscribed for comments. The PR states the case
against as well as for, and says plainly that the base is narrow: two
instances, one sitting, one program, both units mine. The precedent
cited is the scratchpad row, where Ev declined a rule for not being
specific to this project — this one is about a rule the repo already
has, which is the distinction, and it is Ev's to weigh.

Filed on META's slate because `docs/prompts/*` is META's ground; seam
announced on META's log. Neither
`an-items-stated-sweep-pattern-may-not-match-its-own-instance` nor
VDOC's `sweep-blind-spots-the-precheck-sweep-could-not-see` covers it
— the first is a pattern that misses its OWN instance, the second
preserves one sweep's gaps as a record rather than changing the rule.

## 2026-09-21 — AUTH-2's correctness lane: NOT-MERGEABLE, two MAJORs

Both settled by RUNNING, in a production-faithful egui harness the
reviewer built — real `DocSession`, real `egui::Context`, ops
performed per frame as `app` does. Fix pass dispatched.

**M1: the new guard is DEAD CODE on the parameter row.** The
`Selection::Param` arm still calls `widgets::drag_ops`, whose typed arm
pushes `SessionOp::SetParam` unconditionally and BEFORE the new
dispatch runs. egui marks `changed()` on an exact `f64` comparison, so
it fires on the echo case and on every real edit alike. Measured: a
click-in/click-away on a parameter rendering `40` but holding
40.000019 mm still commits and still costs an undo step — **the exact
defect this unit was dispatched to fix** — and typing `1002` over
`1000` costs three history states, one frame emitting two identical
`SetParam`s. C5 is true of `typed_edit` as a pure function and false
of the panel, which is the gap between a unit-tested helper and a
wired one.

**M2: the widened guard discards real slot edits.** The band is
relative to the value and unbounded in absolute terms — ±0.5 mm on a
1 m slot, ±50 mm on a 100 m one — and on the SLOT row `typed_edit` is
the sole emitter, so a field reading `1000` typed as `1000.4` commits
nothing at all. The guard it replaced committed it. A behaviour
regression on a pre-existing door, taken to fix a different one.

**I asked whether widening was the right direction and the answer is
no**, with the alternative in hand: the echo is identifiable AS TEXT.
`number_field` renders through `widgets::number_text` and the
`custom_parser` receives that exact `&str`, so `text != last_rendered`
separates an echo from a re-type **exactly**, needs no tolerance, and
is still one rule in one home. The PR's *"the chrome cannot tell an
echoed text from a re-typed one"* was never forced — it was a
consequence of comparing numbers instead of the thing the field
actually produced. The fix pass is told to re-shape the guard to text
and, if that fails, to stop and report rather than fall back.

**This is the adjudication the posture exists for.** Q1's pull toward
one home is right and duplication is this project's standing defect —
but "one home" is not a reason to make an exact test lossy, and the
lane took the trade in good faith and disclosed it honestly. Deciding
against it is the orchestrator's call, not the lane's, and it needed a
reviewer who would RUN the band to make the cost visible.

**Both units this sitting closed a duplication and minted one.**
AUTH-1 added a hardcoded per-kind sentence twelve lines from the one
it replaced; AUTH-2's `param_unit_ui` is a third hand-rolled copy of
`widgets::pick_unit` — same options call, same early return, same salt
shape, same `.width(72.0)` — in a file importing `widgets` in its own
diff. That is a PATTERN in this program now, and the next spec carries
the trap in its own text rather than leaving it to the reviewer.

**The sweep blind spot again — the SECOND instance, in the second unit.** AUTH-2's
declared gap — *"a unit vocabulary reached through a `match` on
`Dimension`"* — holds six ladders, three of them added by that unit.
Filed as `work/chrome/dimension-to-unit-ladders-have-six-homes-in-the-viewer.md`
(P1). The `[ev]` question's item and PR 2975 already carry this
instance; the count is TWO, one per unit, and an earlier draft of this
entry said "third" by miscounting the three ladders AUTH-2 itself added
as three instances. The question on 2975 rests on that count, so it is
worth being exact about: two units, two reviewers, two gaps.

## 2026-09-21 — the [ev] PR was rebuilt; my mistake

PR 2974 carried **22 files and 1560 lines** of this session's tracker
state behind a one-paragraph design question, because I branched it
off the orchestrator branch rather than off `main`. `work/README.md`
gives a design conversation its own PR precisely so the question is
not buried, and the concrete cost is worse than reading: a "no" would
have stranded the tracker state behind a declined question.

Closed 2974, opened **PR 2975** off `main` with the one file. The
item lands with ordinary tracker state and does not wait on the
answer — the item existing records a finding; it does not ratify a
rule. Subscription moved.

**Main moved a long way underneath this session** and the merge
brought in two things that bind what I write: a program `status`
vocabulary (`ready` / `active` / `blocked` — AUTHOR reads `active`,
correctly, an orchestrator is on it) and Ev's 2026-09-21 rule that
**no status scaffolding goes in the diff** — "proposed", "pending
sign-off", "awaits ratification" only have to be taken out again
before merging; the `[ev]` title, the PR body and `needs_ev:` carry
that status. The META item's "The proposed amendment (for Ev)"
heading was exactly that shape and is now "The amendment, and the
argument either way". Worth noting the rule reached me through a merge
rather than a message, which is the case `work/README.md` says the
board exists to prevent.

## 2026-09-21 — [ev] PR 2975 green; the board as this sitting stands

PR 2975 (the §5 amendment) is green — `tier=docs`, `gate ok`'s "every
job in this run concluded, and concluded green", work-tracker lint
included — mergeable, and waiting on Ev and nothing else. Nothing
further is owed on it until he answers; the check-in is armed and
re-arms silently.

**Open:** AUTH-1 (PR 2955, fix pass on twelve items from two reviews)
and AUTH-2 (PR 2957, fix pass on two MAJORs). Both rows on each unit
read `review`.

**Rows filed outside this program's fence this sitting**, so a
successor can find them without re-reading the narrative: CHROME gains
`a-fifth-spelling-of-this-seat-is-empty`,
`four-pick-state-vocabularies-in-one-create-module`,
`a-per-kind-sentence-lives-in-the-widget-not-on-the-choice`,
`denotes-body-enumerates-its-gaps-against-the-operand-door-and-misses-one`,
`a-creation-forms-held-pick-survives-a-document-swap`,
`dimension-to-unit-ladders-have-six-homes-in-the-viewer`, and (from
AUTH-2's lane) `bounds-reading-respells-the-panels-one-divide`. VDOC
gains `viewer-readme-recourse-count-does-not-say-what-it-counts` and
has `add-profile-ui-doc-comment-states-a-premise-the-tree-falsified`
at `review` on 2955. META gains
`a-stated-sweep-blind-spot-is-never-swept`. VGEOM has evidence on two
existing rows from AUTH-2's lane.

**What a successor should NOT re-derive.** Three things this sitting
learned that are not obvious from the diffs:

1. **A spec should ask, not assert.** Two dispatcher premises were
   falsified by the lanes (the `Ambiguous` multi-body hypothesis; the
   `base_r * 2` text). Both corrections were cheap because the spec
   said *decide and say* around them. Where a spec asserted, the lane
   inherited the error.
2. **Closing a duplication mints one.** Both units did it — AUTH-1 a
   per-kind sentence twelve lines from the one it replaced, AUTH-2 a
   third copy of `widgets::pick_unit`. The next spec carries the trap
   in its own text rather than leaving it to the reviewer, which is
   what `docs/prompts/reviewer-style-lane.md` says never works from
   the PR body alone.
3. **A unit-tested helper is not a wired one.** AUTH-2's guard was
   correct as a pure function and dead code at the panel, because a
   second emitter above it ran first. No row covered the panel's op
   emission; the reviewer had to build an egui harness to see it.

## 2026-09-21 04:09 — the box hit 100% disk, and I was late

**76K free on `/`.** Caught by a check-in, not by a monitor (none are
armed — `local-scripts/monitors/README.md`'s temporary note still
stands) and not by a lane failing. The first symptom was a harness
error: the session's task directory could not be written.

Freed to 11 GB: a finished review lane's 5.6 GB target that I should
have reclaimed when its report arrived, three finished review clones,
and **3.3 GB of in-tree build artifacts in AUTH-1's clone**.

**The reclaim was mine and I did it late.**
`memories/agent-lane-operations.md` is explicit — *"Reclaiming a
finished lane is the ORCHESTRATOR's job… Do it WHEN A REVIEW RETURNS,
not when a lane runs out of disk — a review lane's `target/` is pure
waste the moment its report is in hand, and review lanes are the
biggest consumers."* I reclaimed the two implementer targets on
report and left the review ones. That is the rule working exactly as
written and me not following it.

**The genuinely new fact, for whoever hits this next.** The 3.3 GB was
**in-tree**, under `demos/tour/target`, `demos/wild/target`,
`benches/target` and `tools/*/target` — the cargo roots `Cargo.toml`
EXCLUDES from the workspace. Each is its own cargo root, so a
`CARGO_TARGET_DIR` that does not reach the subshell running them
(`(cd demos/tour && cargo clippy …)`, the shape the discipline itself
suggests) builds into the worktree. The memory's reclaim advice globs
`/home/user/*-target` and names `/root/<lane>-target` as the wandering
case; this is a third location it does not name, and it is inside the
clone rather than beside it.

**Nothing was committed** — working tree clean, zero untracked, and
all four paths are covered by `.gitignore` / `demos/.gitignore`, so
the CERT-M2 hazard (a lane pushing its build directory) did not
recur. Checked rather than assumed, because that is the one failure
here that would have been unfixable under merge-only rules.

Not filed as a row and not proposed as a memory line: one instance,
and the existing rule already covers the part I got wrong. If a second
lane does it, it has earned a sentence in
`memories/agent-lane-operations.md` beside the wandering-target
paragraph.

## 2026-09-21 — AUTH-1's fix pass is green

Run 35558347640 on head `7a5b41609`: 39 jobs, **twelve `test (…)`**,
**five `k-lint (gate, …)`**, zero failures, nothing narrowed. Eleven
of the twelve items landed.

**F12 was MINE and was wrong about the tree.** I adjudicated that the
`partial_mirror!` macro expands to a fn returning `()`; that is the
`@exhaustive` helper arm. The `DatumSpec, onto DatumKindChoice`
invocation takes the `onto` arm, which returns
`Option<DatumKindChoice>` exactly as the original PR body said. The
lane answered rather than implemented, with the macro source quoted.
**Third dispatcher premise falsified in this program** — the first two
were in specs, this one in an adjudication, which is worse: a spec is
read by a lane that will check it, and an adjudication arrives as a
list of things to do.

**F4 produced a finding rather than the fix I asked for**, and the
lane was right to report it. I asked for the boss volume to be made
discriminating by unioning it into the block. It cannot be authored:
a boss drawn on a face frame is FLUSH with the block by construction,
and the kernel refuses an undeclared coincident contact
(`UndeclaredContact`, `FlushFinding`), while the declaration is a
`Declare` node that `SessionOp::AddBoolean` has no seat for. So the
sum-of-volumes assertion is unreachable through this op vocabulary.
The lane asserted the frame's landed POSE whole instead — origin,
normal and sketch +x — which discriminates strictly more than the
previous `z`-only assertion, and verified it by mutation
(`CapEnd::Start` now reds). **That gap is worth knowing about beyond
this unit**: the viewer cannot author a union of two bodies it made
flush, which is an ordinary CAD gesture.
