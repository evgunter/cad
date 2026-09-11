---
id: gesture-drags-have-no-cancel-door
kind: issue
title: Neither gesture has a cancel door in the chrome: CancelGesture and CancelFreeMove both have zero emitters
status: closed
opened: 2026-09-04
refs: [two-gestures-can-be-in-flight-together, preview-and-commit-carry-no-gesture-identity, free-move-in-flight-refusal-has-no-reachable-producer]
closed: 2026-09-11
branch: view/cancel-doors
---



Found by VIEW-7 while establishing whether the value gesture and the
free-move probe can be in flight together (2026-09-04).

## What happens

**Both cancels, not one.** `SessionOp::CancelGesture` and
`SessionOp::CancelFreeMove` each exist in the vocabulary, each has an
arm in `DocSession::perform`, and **neither is pushed from anywhere in
the crate.** Filed first against the value gesture; the free-move half
is the same defect in the same enum and is stated below.

`SessionOp::CancelGesture` exists, `DocSession::perform` handles it
(`crates/viewer/src/session.rs:1065-1079`), and **nothing in the chrome
ever emits it.** The whole crate pushes it from zero sites: the two
`CommitGesture` pushes are `crates/viewer/src/pane/properties.rs:100`
and `:557`, both from `drag_stopped()` on the widget that pushed the
matching `BeginGesture` / `BeginParamGesture`, and there is no other
door.

So a value gesture has exactly one way out through the UI: the
`egui::Response` that opened it must still be there on the frame the
pointer is released, so that `drag_gesture_ops`
(`crates/viewer/src/widgets.rs:81-98`) sees `drag_stopped()`. If that
widget is not drawn on the release frame, no op is emitted, egui clears
its own drag state, and `DocSession::gesture` stays `Some` with no
pointer behind it.

### The free-move half, and why it is worse

`SessionOp::CancelFreeMove` has zero emitters (`crates/viewer/src/`,
outside `session/op.rs` and `session.rs`). The free-move gesture's only
exit through the UI is the same one: `drag_stopped()` on the
`egui::DragValue` that opened it (`pane/properties.rs:384-397`), which
pushes `CommitFreeMove`.

A stranded free-move gesture is MORE visible than a stranded value
gesture, and its wording is worse. `begin_free_move` refuses a re-open
with `DisplayFault::FreeMoveInFlight`, which renders as **"finish the
free-move first"** (`display.rs:157`) — an instruction the user cannot
follow, because the gesture it names has no pointer behind it and no
door to close it. That is the honesty rule inverted: a refusal that
names a remedy that does not exist.

## Why it matters

The stranded state is not quiet. `perform` fences on it once
(`session.rs:1041`), so from then on `Undo`, `Redo`, `Open`,
`NewDocument`, `DeleteNode`, every `Add*` and every other slot edit
refuse `Refusal::GestureInFlight` — and the scratch document
(`DocSession::doc`, `session.rs:563`) keeps a preview on screen that
the history does not have. Nothing in the chrome offers a way back, and
no key does either: there is no Escape binding for it in `input.rs`.

A stranded value gesture is also the ONE route by which the two
gestures overlap in the real UI. Both drags are `egui::DragValue`s in
the Properties pane through one mapping (`drag_ops`), so a single
pointer cannot hold both at once; stranding is what removes the pointer
from one half. VIEW-7 established that the overlap is sound today (the
mechanism, and DI5's expiry date on it, are written at
`SessionOp::permitted_during_value_gesture`), so this item is about the
stranding, not about the overlap.

## What is NOT established

**Whether the widget can actually vanish mid-drag.** Every op that
changes what the Properties pane draws — `SessionOp::Select` from the
pick path, the feature rows and the parameter links — is click-driven,
so with one pointer held on a `DragValue` none of them can fire. A
landed evaluation, a pane layout change or a second pointer are the
candidates and none was traced. So the reachability of the STRANDING is
open; the absence of the cancel door is not — that is read directly off
the emitter count.

The cheap half is worth stating separately: even if nothing can strand
a drag today, a gesture whose only exit is one widget's release event
is a door with no lock and no key, and `CancelGesture` is a vocabulary
arm the GUI never uses.

## Home

VIEW's: `crates/viewer/src/pane/properties.rs`,
`crates/viewer/src/widgets.rs`, `crates/viewer/src/input.rs`.

The two halves are one item because they are one defect with one
shape — a gesture whose only exit is one widget's release event — and
splitting them would file the instance twice rather than the class.

## Closed (2026-09-11)

Both operations have a door. `DocSession::cancel_doors` composes one
[`CancelDoor`] per gesture and `app.rs`'s toolbar draws them beside Undo
and Redo; `SessionOp::CancelGesture` and `SessionOp::CancelFreeMove`
each have an emitter now. The argument is `crates/viewer/README.md`'s
**Every gesture has a cancel door** and the value's own docs
(`session/op.rs`).

### The reachability question: TRACED for the value drag

The item left it open and named three candidates — a landed evaluation,
a pane layout change, a second pointer — and said none was traced. It is
the first, and it does not need a second pointer or anything the user
does: **the drag's own preview strands it.**

`DocSession::slot_rows` returns nothing when `standing().live()` is
false, and a face whose name did not resolve is not live. Every frame a
drag moves, `PreviewGesture` submits its scratch document; a preview
that takes an extrude's distance to zero lands an evaluation in which
the picked face does not resolve. So the panel is handed no rows, the
field whose `drag_stopped()` is the gesture's only exit is not drawn,
the release reports nothing, and the gesture stays open. The item's
reason for not finding this is in its own text: it looked for an
operation that could change what the panel draws and found all of them
click-driven. The drag is not another operation — it is the drag.

Held by `gesture_table.rs`'s
`a_drags_own_preview_can_strand_it_and_the_door_closes_it`, which runs
the whole session half. **What it does not run** is the last link — that
a group absent from `slot_groups` is not drawn and so reports no release
— which is `properties_ui`'s `for group in &groups` and is read rather
than executed, because this crate has no headless egui harness.

**NOT traced for the free-move**, and the door is owed anyway. The
probe's field is drawn off the shown document rather than the landed
evaluation, and a document change under an in-flight probe is pruned,
not stranded, so the trace above does not carry over. A second pointer
or an OS-driven relayout are still untraced candidates.

### What this item got wrong

**Its sharpest claim names the wrong sentence.** The honesty inversion
is real, but `DisplayFault::FreeMoveInFlight` — *"finish the free-move
first"* — cannot be shown to a user at all today: every route to it
needs a free-move already in flight, which needs the untraced free-move
strand. The reachable inverted refusal is `Refusal::GestureInFlight` —
*"finish the drag first"* — which the trace above reaches with no
pointer behind the drag. Filed as
`free-move-in-flight-refusal-has-no-reachable-producer`.

**`input.rs` is not where a cancel key would live**, so "there is no
Escape binding for it in `input.rs`" points at a module that could not
hold one. `input.rs` maps what the POINTER did inside the VIEWPORT
(`ViewportEvent`), and a cancel key has to work with the cursor over the
Properties pane. `input::PRESETS`' own docs say the rest: this crate
binds no key to any operation anywhere, and the first one needs a
keyboard vocabulary decided first (which key means which op, how modal
state is held, how a chord is represented). The door is chrome, and the
key is a separate decision nobody has taken.

### Mutation evidence

Six, each reverted: the drag's door removed from `cancel_doors` (census
+ strand rows red); `perform`'s `CancelGesture` arm no longer taking the
gesture (strand row); `blocked` inverted (three rows); the toolbar loop
deleted (`the_cancel_doors_have_a_reader_in_the_chrome`);
`CancelFreeMove` marked as not a gesture cancel (census); and
`slot_rows`' dead-standing guard removed, so the reachability half is
not vacuous (strand row).

### Residue

- `preview-and-commit-carry-no-gesture-identity` — while stranded, a
  drag on any other field previews and COMMITS into the stranded slot.
  Traced and recorded there; the door is a way out of the state and does
  not repair this.
- `free-move-in-flight-refusal-has-no-reachable-producer` — above.
