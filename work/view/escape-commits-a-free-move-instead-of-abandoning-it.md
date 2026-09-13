---
id: escape-commits-a-free-move-instead-of-abandoning-it
kind: issue
title: Escape during a free-move drag commits the probe instead of abandoning it
status: closed
opened: 2026-09-11
closed: 2026-09-11
---



Found by the free-move reachability unit (2026-09-11), driving the
probe field against a headless `egui::Context`.

egui aborts a drag on Escape and on nothing else:
`egui-0.36.1/src/interaction.rs:137-141` clears `dragged` there, which
makes `drag_changed` true, which makes `drag_stopped` name the widget.
So the key that every other control in this chrome spells *abandon*
reaches `drag_gesture_ops`' release arm (`crates/viewer/src/widgets.rs`,
`:95-98` at the SHA this was filed at) as an ordinary release, and the
chrome emits `CommitFreeMove`.

Measured, with the harness now in `crates/viewer/src/widgets.rs`'s test
module: press and move on the probe's x component gives
`["begin", "preview"]`; the next frame carrying only
`egui::Key::Escape` gives `["commit"]`. The probed frame is landed into
`DisplayState::moves`, not discarded.

That is the opposite of what the chrome offers beside it: `cancel_doors`
draws *"Cancel free-move"* precisely so a probe can be abandoned
(`crates/viewer/src/session.rs:660`), and `SessionOp::CancelFreeMove`
exists for it.

The same reading applies to the value gesture, whose triple runs through
the same function — not checked here, and worth checking with the fix.

## The fork, because it is not obviously a bug in this crate

Escape is egui's abort, so one repair is to read it where the triple is
mapped: `drag_gesture_ops` could emit the cancel operation rather than
the commit on a `drag_stopped` frame that carries an Escape press. That
needs a fourth operation parameter and an `Option` for the vocabularies
that have no cancel. The other is to decide that a probe ends by
landing whatever it previewed, whichever key ended it, and to say so —
in which case the row closes as ratified rather than fixed.

## Closed (`view/escape-abandons`): fixed, shape 1, at both drags

The gesture triple became a `GestureVocabulary`
(`crates/viewer/src/widgets.rs:38-49`) with a fourth operation, and
`widgets::drag_gesture_ops` emits that one, not the commit, on a
`drag_stopped` frame carrying an Escape press
(`crates/viewer/src/widgets.rs:125-151`). A struct rather than a fourth
positional parameter because `commit` and `cancel` are the same type
and mean opposite things. The three fields the call sites fill are
`CancelGesture` for the parameter drag, `CancelFreeMove` for the probe
and `CancelGesture` for the slot drag
(`crates/viewer/src/pane/properties.rs:108`, `:400`, `:580`).

**Three things the fork's own text got wrong, all found by re-deriving
it against the tree.**

1. **No `Option` is needed.** All three vocabularies the panel maps
   have a cancel, and that is structural rather than lucky:
   `gesture_table.rs`'s `every_gesture_cancel_has_a_chrome_door`
   matches exhaustively over `SessionOp`, so a gesture joining the enum
   with no cancel reds there.
2. **The cancel doors cannot be reached during a live pointer drag.**
   They are toolbar controls, and pressing one costs the release that
   lands the value — so the chrome's *"Cancel free-move"* is the
   STRANDED drag's exit, and until this change a held drag had no
   abandon at all. That, not the door's mere existence, is the
   argument that settles the fork.
3. **The value gesture has the same defect and a larger stake**, not a
   smaller one: a free-move commit lands a display frame no history
   holds, a value-gesture commit reaches the document and costs an undo
   step. Measured by mutation — with the Escape branch removed, both
   new rows report `["commit"]` where `["cancel"]` belongs.

Held by three rows in `crates/viewer/src/widgets.rs`'s test module,
over both vocabularies:
`escape_abandons_a_free_move_drag_instead_of_landing_it`,
`escape_abandons_a_value_drag_instead_of_committing_it` and
`releasing_the_pointer_still_commits_both_gestures` — the third so the
rule stays *which end happened* rather than *end quietly*.

**Two prose claims this made false, both amended here**:
`crates/viewer/README.md`'s *"There is no key for it"* paragraph and
`input::PRESETS`' *"no key denotes an operation anywhere"*. The crate
now reads exactly one key, and reading is not binding: `egui` ends a
drag on Escape whatever this crate does, so the branch decides which of
two things the toolkit did is reported, not which operation a key
denotes. A keyboard vocabulary still needs every decision `PRESETS`
names.

Read from the key rather than inferred from the absence of a pointer
release: a long touch also ends a drag with no release
(`egui-0.36.1/src/interaction.rs:143-155`) and means something else.
