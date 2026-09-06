---
id: gesture-drags-have-no-cancel-door
kind: issue
title: Neither gesture has a cancel door in the chrome: CancelGesture and CancelFreeMove both have zero emitters
status: open
opened: 2026-09-04
refs: [two-gestures-can-be-in-flight-together]
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
(`crates/viewer/src/session.rs:667-681`), and **nothing in the chrome
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
(`session.rs:642`), so from then on `Undo`, `Redo`, `Open`,
`NewDocument`, `DeleteNode`, every `Add*` and every other slot edit
refuse `Refusal::GestureInFlight` — and the scratch document
(`DocSession::doc`, `session.rs:322`) keeps a preview on screen that
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
