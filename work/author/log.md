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

## AUTH-2 fix pass — the guard is over TEXT (2026-09-21)

The correctness review returned NOT-MERGEABLE with two MAJORs, both
settled by running a production-faithful egui harness. Both were about
the same decision, taken the wrong way round.

**The guard compares TEXT, not numbers.** The implementation judged a
typed number against the number the field displays, by the renderer's
own tolerance — which is relative and unbounded in absolute terms, so
a field reading `1000` in millimetres discarded a typed `1000.4`
silently: no edit, no refusal, the field reverting. The echo is
identifiable exactly as text: `egui::DragValue` seeds its keyboard
edit with the text its formatter returned, so the field keeps that
text and the parser compares against it (`props::echoed`). No
tolerance, one rule, both fields, and a row showing SOURCE rather than
a number is answered by the same comparison.

**Two rules, not three spellings of one.** What a field's guard
decides ("did this text come out of the field?") and what a document
door decides ("would this edit move anything?") are different
questions with different answers, and the implementation's doc claimed
they were the same rule. They are now two functions with one home
each: `props::echoed` at the field, `DocSession::writes_nothing` at
the door. The second is what makes one typed number one undo step,
because `egui` hands a buffered text over on two consecutive frames —
filed as `work/vgeom/a-typed-field-hands-its-text-over-on-two-frames`.

**The panel's op emission has a row now.** The crate said it carried
no headless egui harness; it does, in `widgets.rs`, over a real
`egui::Context` and a real `DocSession`. Both MAJORs were reproduced
there before they were fixed — the click-in/click-away emitted
`SetParam(0.04)` over a field holding 0.040000019, and the typed
`1000.4` emitted nothing.
