---
id: the-value-drags-in-flight-refusal-has-two-spellings
kind: issue
title: "the value drag's in-flight refusal is spelled twice: the mid-gesture table's row and g1::Slot's floor"
status: open
opened: 2026-09-15
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
