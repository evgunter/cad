---
id: two-hand-written-copies-of-the-g1-gesture-machine
kind: issue
title: session::Gesture and display::FreeMoveGesture are two hand-written copies of one G1 preview/commit state machine
status: closed
opened: 2026-09-04
closed: 2026-09-15
branch: view/g1-gesture
refs: [two-gestures-can-be-in-flight-together, gesture-drags-have-no-cancel-door, the-value-drags-in-flight-refusal-has-two-spellings]
pr: 2672
priority: P1
cost: D
---



Found by VIEW-7's style review (2026-09-04), while answering whether
the two gestures may be in flight together.

## What happens

`session::Gesture` (`crates/viewer/src/session.rs:192`) and
`display::FreeMoveGesture` (`crates/viewer/src/display.rs:400`) are two
hand-written implementations of ONE state machine — G1's
preview/commit shape — with the same four operations and the same three
rules:

- **begin** refuses if one is already in flight, and validates its
  target first (`session.rs:1478-1504`, `display.rs:741-755`);
- **preview** replaces the last rather than composing, and refuses if
  none is in flight (`session.rs:1516-1559`, `display.rs:623-636`);
- **commit** lands exactly one value, and **a gesture that never
  previewed commits nothing** — the no-move rule, written twice
  (`session.rs:1575-1588`, `display.rs:648-658`);
- **cancel** takes the gesture and restores the prior picture.

They carry two refusal vocabularies for the same three states —
`Refusal::NoGesture` / `Refusal::GestureInFlight` against
`DisplayFault::NoFreeMove` / `DisplayFault::FreeMoveInFlight` — and the
relationship between the copies is reconciled in PROSE, by hand, at
`display.rs:639-640` ("the no-move rule the document gestures follow")
and `session.rs:1126-1128` ("Same rule as a no-move commit").

## Why it is worth a file

**The chrome already unified and the state machines did not follow.**
`crate::widgets::drag_ops` (`widgets.rs:101-123`) is one mapping over
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

Note DI5 (`crates/editor-core/IDENTITY.md`, ratified) moves the free-move
commit onto the document as a `DocEdit::SetPlacement`, which brings the
two machines CLOSER, not further apart: after it, both commits land a
document edit. Sequencing this after
`no-persistent-setplacement-session-op` is probably right.

## Home

VIEW's: `crates/viewer/src/session.rs`, `crates/viewer/src/display.rs`.

## The sequencing question, answered: it does not wait (2026-09-15)

The item says *"sequencing this after `no-persistent-setplacement-session-op`
is probably right"* because DI5 brings the two machines closer. It does
not, and the caveat lands on the shape this item already rules out
rather than on this one.

DI5 changes what the probe's commit LANDS — a `DocEdit::SetPlacement`
on the document instead of an entry in `DisplayState::moves` — and
changes none of the three rules: begin-refuses-when-in-flight,
preview-replaces, and never-previewed-lands-nothing are true whatever
the commit lands in. What DI5 would bring closer is the two machines'
VALUE KINDS and side effects, which is the sharing this item says would
be worse than two clear copies. Holding the rules first is also the
cheaper order: after it, DI5 changes one landing step in one caller
against a machine it cannot break, instead of re-deriving the no-move
rule in a second copy. `g1::Slot::commit` already hands its caller back
the frame it took, which is the shape DI5's session-side edit needs.

## Closed by `view/g1-gesture`

`crates/viewer/src/g1.rs` holds the three rules once, as `Slot<Held,
Value>` with `Refusals<Fault>` handed in per call. The in-flight state
is private to that module, so no caller can observe or move a gesture
except through `begin` / `preview` / `commit` / `cancel` / `discard` —
a rule about the transitions cannot be spelled anywhere else, and one
cannot be fixed in one machine and left broken in the other. The two
vocabularies are NOT merged: `session::gesture_words` and
`display::free_move_words` each declare their three words once.

The prose reconciliations are gone as reconciliations: `commit_free_move`
and the `CancelGesture` arm now say where the rule is held rather than
that the other copy follows it.

**What this did not settle**, filed rather than absorbed:
`the-value-drags-in-flight-refusal-has-two-spellings` — the value
drag's rule-1 refusal is now raised both by `perform`'s table and by
`g1::Slot::begin`, and the free-move rows in `session/op.rs` argue
against exactly that shape.

**One claim in this item is now historical.** *"`CancelGesture` and
`CancelFreeMove` both have zero emitters"* was the evidence that the
divergence was live; `gesture-drags-have-no-cancel-door` closed on
2026-09-11 and both have doors. Re-derived on this tree, the two copies
AGREE on all three rules today, so the defect this unit closes is the
hazard rather than a present divergence — which is what
`widgets::drag_ops`'s own history is evidence for.
