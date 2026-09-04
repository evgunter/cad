---
id: gesture-drags-have-no-cancel-door
kind: issue
title: A value gesture has no cancel door in the chrome, so an abandoned slider drag strands the session gesture-in-flight
status: open
opened: 2026-09-04
refs: [two-gestures-can-be-in-flight-together]
---



Found by VIEW-7 while establishing whether the value gesture and the
free-move probe can be in flight together (2026-09-04).

## What happens

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

## Why it matters

The stranded state is not quiet. `perform` fences on it once
(`session.rs:642`), so from then on `Undo`, `Redo`, `Open`,
`NewDocument`, `DeleteNode`, every `Add*` and every other slot edit
refuse `Refusal::GestureInFlight` — and the scratch document
(`DocSession::doc`, `session.rs:322`) keeps a preview on screen that
the history does not have. Nothing in the chrome offers a way back, and
no key does either: there is no Escape binding for it in `input.rs`.

It is also the ONE route by which the two gestures overlap in the real
UI. Both drags are `egui::DragValue`s in the Properties pane through
one mapping (`drag_ops`), so a single pointer cannot hold both at once;
a stranded value gesture is what removes the pointer from the value
half. VIEW-7 established that the overlap is sound either way (the
mechanism is written at
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
