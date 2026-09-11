---
id: free-move-in-flight-refusal-has-no-reachable-producer
kind: issue
title: DisplayFault::FreeMoveInFlight cannot be shown to a user: every route to it needs the free-move strand nothing has traced
status: open
opened: 2026-09-11
refs: [gesture-drags-have-no-cancel-door]
---


Found by the cancel-door unit (2026-09-11), which was dispatched
against `gesture-drags-have-no-cancel-door`'s sharpest claim and found
that claim's subject one door over from where the item puts it.

## What the item said and what the tree says

`gesture-drags-have-no-cancel-door` hangs its honesty argument on
`DisplayFault::FreeMoveInFlight`, which renders as **"finish the
free-move first"** (`crates/viewer/src/display.rs:215`) — *"an
instruction the user cannot follow, because the gesture it names has no
pointer behind it and no door to close it."* The shape is right. The
sentence is the wrong one, because **no route in today's chrome shows
it to anybody.**

`DisplayState::begin_free_move` raises it only when a free-move is
already in flight (`display.rs:708-710`). The probe's field is three
`DragValue`s on the selected instance (`pane/properties.rs:377-402`),
each pushing `BeginFreeMove` on `drag_started`; one pointer cannot hold
two of them, and the typed arm emits begin/preview/commit in one batch.
Selecting another instance needs a click, which releases the drag and
commits it first. So reaching a second `BeginFreeMove` under an open
one needs the probe to be STRANDED — and the strand traced by the
cancel-door unit is the VALUE drag's, off `slot_rows` emptying for a
dead standing. The probe's field is drawn off the shown document
(`display::is_instance`, `display::free_move_check`), not off the landed
evaluation, and a document change while a probe is in flight is pruned
rather than stranded (`display.rs:821-827`), so that trace does not
carry over.

The honesty inversion the item describes is real and it lands on
`Refusal::GestureInFlight` — **"finish the drag first"**
(`crates/viewer/src/session/refuse.rs:427`) — which the same unit
traced to a state with no pointer behind the drag. That half is closed
by the cancel doors.

## What is open

1. **Is the free-move strand reachable at all?** Candidates nobody has
   traced: a second pointer (egui multi-touch), and an OS- or
   browser-driven relayout that removes the field without a click. If
   one of them is real, `FreeMoveInFlight` becomes reachable and this
   row closes as answered.
2. **If it is not**, the crate carries a `DisplayFault` arm no chrome
   can produce, which is the shape
   `opoutcome-superseded-has-no-production-reader` and
   `generation-get-has-no-reader` were filed as — with the difference
   that this one is a REFUSAL, so deleting it means deciding that
   `begin_free_move` cannot be called twice rather than that it must
   not be.

Not the cancel-door unit's to settle: the door is owed either way (the
operation had no emitter), and which way this goes changes the door not
at all.

## Home

VIEW's: `crates/viewer/src/display.rs`,
`crates/viewer/src/pane/properties.rs`.
