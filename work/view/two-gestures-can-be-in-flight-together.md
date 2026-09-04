---
id: two-gestures-can-be-in-flight-together
kind: issue
title: session::Gesture and display::FreeMoveGesture share a field name and no guard, so a slider drag and a free-move probe can overlap
status: review
opened: 2026-09-04
refs: [viewer-session-god-module-split, save-is-not-gesture-guarded, gesture-drags-have-no-cancel-door]
branch: view/two-gestures
pr: 1873
---


Found by the whole-file read that opened
`viewer-session-god-module-split` (2026-09-04).

## What happens

Two unrelated types are both reached as `self.gesture`:

- `session::Gesture` (`crates/viewer/src/session.rs:153`) — a slot or
  parameter drag;
- the free-move gesture on `DisplayState`
  (`crates/viewer/src/display.rs`, read at 555, 574, 599, 619, 658,
  672).

Same field name, different owner, different type, no relation. That is
two spellings of one concept across two files, and it is the reason
the second half is easy to miss: the four `*FreeMove` operations are
**not** guarded against the session gesture (they are among the
unguarded set in `save-is-not-gesture-guarded`), so a slider drag and
a free-move probe can be in flight at the same time.

Whether that overlap is reachable through the real UI depends on
whether the panels can accept a drag while the viewport holds a probe;
whether it is *intended* is not stated anywhere. What is certain is
that nothing enforces either answer.

## Why it matters for unit 1

The charter asks whether gesture-safety becomes data. If it does, this
is the case that decides the shape of the datum: one flag is not
enough if there are two gestures, and a
`SessionOp::permitted_during_value_gesture` predicate that silently
meant "safe against the slot gesture only" would be a table
that reads as complete and is not — the exact failure the table exists
to prevent.

## Home

VIEW's: `crates/viewer/src/session.rs` and `display.rs`.


## Citations re-pointed after the 1c split (VIEW orchestrator, 2026-09-04)

This file was written against the pre-split tree. The `file:line`
citations above are corrected in place; this note exists so a reader
who remembers the old ones can tell a correction from a claim change.
Nothing about the finding moved — `stale-file-citations-after-the-split`
is the general case, and this is VIEW's own half of it being paid.


## What VIEW-7 established (2026-09-04)

**The overlap is sound, and the mechanism is now written at the site.**
Both halves of the finding are answered; what stays open is one
residue, filed as `gesture-drags-have-no-cancel-door`.

### The framing above is wrong in one place

"Whether the panels can accept a drag while the viewport holds a probe"
assumes the probe is a viewport gesture. **It is not.** The four
`*FreeMove` ops are pushed from exactly one place in the crate —
`crates/viewer/src/pane/properties.rs:384-397`, the selected instance's
free-move field — through `crate::widgets::drag_ops`, which is the same
one mapping the slot and parameter fields use for the value gesture
(`properties.rs:98-107`, `:552-559`). Both drags are `egui::DragValue`
drags in the Properties pane. The viewport never holds a probe.

### Reachability

**With one pointer and a frame that draws the pane, the two cannot
overlap** — not because anything forbids it, but because egui binds a
drag to the widget the press landed on, and every op that could change
what the pane draws mid-drag (`SessionOp::Select` from the pick path,
the feature rows, the parameter links) is itself click-driven. That is
an accident of the input device, and nothing states or enforces it:
`DocSession::perform` is a public door and accepts the interleaving
directly, which is how the row below drives it.

The one UI route into the overlap is a value gesture stranded in
flight, which is possible because `SessionOp::CancelGesture` is emitted
from **nowhere** in the chrome — filed separately as
`gesture-drags-have-no-cancel-door`, since it is a defect on its own
terms whether or not the overlap matters.

### Soundness — today, with a ratified expiry date

**A probe committed or discarded mid-drag cannot make the slider's
preview or its commit wrong.** A value gesture owns a `Doc` snapshot
(`Gesture::base`) and writes a scratch `Doc`; the free-move gesture's
previews and committed frames enter no `Doc`, so nothing it writes is
reachable from `apply(&gesture.base, …)` or from what `commit_gesture`
records.

**That last clause has an expiry date.** DI5
(`docs/DOCM-IDENTITY-DESIGN.md`, ratified 2026-09-04) rules that
releasing a free-move emits one `DocEdit::SetPlacement` and empties
`DisplayState::moves`. When CHROME's
`no-persistent-setplacement-session-op` lands, `CommitFreeMove` becomes
the ONLY permitted row that commits to history while a value gesture is
open — and a commit applies against `history.doc()` while the gesture
previews against its own snapshot. **That row has to be re-decided
then**; the other three `*FreeMove` rows are unaffected. The doc
comment says this at the site, so the argument names the ruling that
ends it.

The two drags meet in **three** places, not one — all three ask a
display predicate about a document, and not all about the same one:

- the op admits against the COMMITTED document (`session.rs:704`);
- `display_view` resolves against the PREVIEWED one (`session.rs:467`),
  **and so does the Properties pane's own copy of the admission test**
  (`pane/properties.rs:333`, `:345`), which decides whether to DRAW the
  control the op then decides whether to ACCEPT;
- every committed edit prunes the display state (`display.rs:651-685`),
  discarding committed probes and killing an in-flight free-move
  without reporting the kill.

Those agree because of one identity: a value gesture's edits are
`SetParam` and `SetStructuralParam`, which replace an expression on a
node that already exists, and `SetDocParamValue`, which writes
`doc.params` and touches no node at all; none mints or removes one
(`crates/editor-core/src/edit.rs:1518-1537`, `minted: None`). Every
display predicate is a function of the node graph and never of a slot's
expression or a parameter's value. VIEW-7's correctness review
strengthened this: `set_slot` takes `&mut Expr` and assigns through it,
so changing a `RecipeNodeId` is type-level impossible, and neither slot
arm sets `reconcile`, so `mate::solve::reconcile` — the one path that
can insert nodes and re-key placements — never runs.

**Whether the overlap was ever DECIDED is not established.** The four
rows were carried forward from a tree where the `*FreeMove` ops were
merely unguarded. What is established is that it is sound today.

### What landed

- The mechanism is stated at the table's four `*FreeMove` rows, under
  "Why the two drags may overlap"
  (`SessionOp::permitted_during_value_gesture`), replacing the
  paragraph that said the question was open.
- `DisplayState::gesture` is renamed `free_move` (11 private sites, one
  file), so `self.gesture` names one thing crate-wide; the field doc
  says why and points at the mechanism. `DocSession::gesture` keeps its
  name — it is what `Refusal::GestureInFlight` speaks about.
- `crates/viewer/README.md`'s gesture-safety clause, which asserted the
  pre-rename spelling ("both fields are spelled `self.gesture`").
- A guard where prose was doing the work: `preview_gesture` asserts
  `applied.record.minted.is_none()`. It holds the half a check can hold;
  the other half is that `GestureTarget::edit` can produce nothing but
  the three edits named above.
- `crates/viewer/tests/gesture_table.rs` gains
  `a_value_gesture_and_a_free_move_probe_do_not_disturb_each_other`,
  which drags a pattern's `Spacing` (`SetParam`) and `Count`
  (`SetStructuralParam`) — the two value-gesture doors that write into
  `doc.nodes` — over both a committed and an in-flight probe. An
  earlier draft dragged a document parameter, whose edit touches no
  node, which made the identity assertions incapable of failing.

### Residue

- `gesture-drags-have-no-cancel-door` — widened to the class: neither
  `CancelGesture` nor `CancelFreeMove` has an emitter.
- `two-hand-written-copies-of-the-g1-gesture-machine` — the two
  gestures implement one state machine twice.
