---
id: two-hand-written-copies-of-the-g1-gesture-machine
kind: issue
title: session::Gesture and display::FreeMoveGesture are two hand-written copies of one G1 preview/commit state machine
status: open
opened: 2026-09-04
refs: [two-gestures-can-be-in-flight-together, gesture-drags-have-no-cancel-door]
---



Found by VIEW-7's style review (2026-09-04), while answering whether
the two gestures may be in flight together.

## What happens

`session::Gesture` (`crates/viewer/src/session.rs:155`) and
`display::FreeMoveGesture` (`crates/viewer/src/display.rs:390`) are two
hand-written implementations of ONE state machine — G1's
preview/commit shape — with the same four operations and the same three
rules:

- **begin** refuses if one is already in flight, and validates its
  target first (`session.rs:969-995`, `display.rs:562-570`);
- **preview** replaces the last rather than composing, and refuses if
  none is in flight (`session.rs:997-1037`, `display.rs:585-598`);
- **commit** lands exactly one value, and **a gesture that never
  previewed commits nothing** — the no-move rule, written twice
  (`session.rs:1045-1048`, `display.rs:610-620`);
- **cancel** takes the gesture and restores the prior picture.

They carry two refusal vocabularies for the same three states —
`Refusal::NoGesture` / `Refusal::GestureInFlight` against
`DisplayFault::NoFreeMove` / `DisplayFault::FreeMoveInFlight` — and the
relationship between the copies is reconciled in PROSE, by hand, at
`display.rs:601-602` ("the no-move rule the document gestures follow")
and `session.rs:669-671` ("Same rule as a no-move commit").

## Why it is worth a file

**The chrome already unified and the state machines did not follow.**
`crate::widgets::drag_ops` (`widgets.rs:30-52`) is one mapping over
both vocabularies, and its own doc says why: *"the same file once had
two copies of it and one of them was wrong"* — the typed-input arm was
silently dropped by a hand-written copy. That is the same failure
shape, one layer down, and one layer down it is unguarded: nothing
makes the two machines agree, and a rule fixed in one is fixed in one.

The evidence that the divergence is live rather than hypothetical:
`CancelGesture` and `CancelFreeMove` both have zero emitters
(`gesture-drags-have-no-cancel-door`) — the same hole, arrived at
twice, independently.

## What this item is NOT

It is not a claim that the two should share a type. They own different
value kinds (a `SlotValue` against a `Frame`), different validation
(a slot's dimension against a rigid-motion check) and different
side effects (a scratch `Doc` against a display revision), and a
premature generic over both would be worse than two clear copies. The
question is whether the three SHARED rules — begin refuses when in
flight, preview replaces, a gesture that never previewed commits
nothing — can be held once, the way `drag_ops` holds the widget mapping
once, with the vocabularies as parameters.

Note DI5 (`docs/DOCM-IDENTITY-DESIGN.md`, ratified) moves the free-move
commit onto the document as a `DocEdit::SetPlacement`, which brings the
two machines CLOSER, not further apart: after it, both commits land a
document edit. Sequencing this after
`no-persistent-setplacement-session-op` is probably right.

## Home

VIEW's: `crates/viewer/src/session.rs`, `crates/viewer/src/display.rs`.
