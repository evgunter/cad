---
id: the-value-drags-in-flight-refusal-has-two-spellings
kind: issue
title: the value drag's in-flight refusal is spelled twice: the mid-gesture table's row and g1::Slot's floor
status: closed
opened: 2026-09-15
closed: 2026-09-16
---


Found by the `two-hand-written-copies-of-the-g1-gesture-machine` lane
while holding the three shared G1 rules once.

## What happens

`g1::Slot::begin` refuses a begin that would overwrite a gesture
already in flight, with the vocabulary its caller hands in — that is
rule 1, and it is where both gestures now get it. For the FREE-MOVE
probe it is the only spelling: `SessionOp::BeginFreeMove` is `true` in
`SessionOp::permitted_during_free_move`, and `session/op.rs` says why
in as many words — *"it is already answered one layer down … A second
`false` row would be a second spelling of one answer."*

For the VALUE drag there are two. `SessionOp::BeginGesture` and
`SessionOp::BeginParamGesture` are `false` in
`SessionOp::permitted_during_value_gesture`, so `DocSession::perform`
refuses `Refusal::GestureInFlight` before `begin_gesture` runs at all;
`DocSession::start` then hands `g1::Slot::begin` the same refusal for a
state `perform` has already excluded. Same refusal, two places, and
the door's arm is unreachable through `perform` — which is the exact
shape the free-move rows are argued against.

## Why it was not settled here

Moving the answer down — the two table rows to `true`, the slot's
refusal reachable — is a change to the mid-gesture policy, which has
its own ratified section (`crates/viewer/README.md`, **Gesture safety
is data**), its own hand-written second copy of the answers
(`crates/viewer/tests/gesture_table.rs`'s `expected`), and one ordering
consequence to weigh: today a `BeginGesture` on an expression-driven
slot under an open drag refuses `GestureInFlight`, and with the table
row gone it would refuse whatever `guard_driven` says unless the
validation moves inside the slot's `begin` closure — which needs
`DocSession::driver_of` and `guard_driven` to stop taking `&self`, so
that the closure can borrow `self.history` while `self.gesture` is
borrowed mutably.

Keeping the floor is the other answer, and it is not obviously worse:
the table's row is then a POLICY (a begin is refused mid-drag, like
every other document-moving operation) and the slot's guard is the
SAFETY (a begin never overwrites an open gesture), which is what makes
the row's answer a choice rather than a requirement. What the tree
cannot have is both spellings with neither sentence saying which is
which — which is what this row is for.

## Home

VIEW's: `crates/viewer/src/session.rs`,
`crates/viewer/src/session/op.rs`, `crates/viewer/src/g1.rs`.

## Answered: the answer moves down (2026-09-16)

The two rows are `true` now, and `g1::Slot::begin` is the door a second
`BeginGesture` meets. The two doors' own target checks — the driven-slot
guard and the parameter lookup — moved inside `DocSession::start`'s
closure, so rule 1 still answers before either of them and every
user-visible refusal is the one it was.

**Which site a user reached before**: the table's. `perform` consults
`permitted_during_value_gesture` before dispatch, so `begin_gesture`
never ran under an open drag and `g1::Slot::begin`'s arm was
unreachable through the only door that calls it.

**Why not the floor.** The alternative asked for a sentence saying which
row is POLICY and which is SAFETY, and there is no input on which the
two differ: the table's row fires on `self.gesture.held().is_some()`,
the slot's arm on the same state, and both raise
`Refusal::GestureInFlight`. A policy extensionally identical to the
safety floor is not a second decision the table records. The table's own
certifying sentence did not cover the rows either — *"everything else
moves the document, the history or the file the drag is previewing
against"* is false of both begins, which move none of the three.

`session/op.rs`'s `BeginFreeMove` argument survives and is now general:
it is one rule about rule 1 rather than one table's exception.
