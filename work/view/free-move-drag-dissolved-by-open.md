---
id: free-move-drag-dissolved-by-open
kind: issue
title: Open and NewDocument dissolve an in-flight free-move drag with no refusal and no report
refs: [session-clearing-walk-is-hand-maintained-three-times, 1885]
status: open
opened: 2026-09-05
---


Found by the #1885 style review (M3), reading the ratified prose that
PR added against the walk one line below it. Pre-existing; #1885
collapsed the walk into one value and did not change what it does to
this field.

## What happens

`DocSession::clear_for_new_document` (`crates/viewer/src/session.rs:1318`)
calls `self.display.clear()` (`:1320`), and `DisplayState::clear`
sets `free_move = None` (`crates/viewer/src/display.rs:689`). So an
in-flight FREE-MOVE drag is silently dissolved by `Open` and by
`NewDocument`.

Nothing refuses either door while a free move is open.
`SessionOp::permitted_during_value_gesture`
(`crates/viewer/src/session/op.rs:650`) governs the VALUE gesture only
— its name carries that limit deliberately
(`crates/viewer/README.md`, Gesture safety is data) — and the free-move
arms' own guard is inside `DisplayState`, which refuses
`DisplayFault::FreeMoveInFlight` for free-move OPERATIONS and says
nothing about a document replacement.

## Why it is a defect and not a choice

The two drags are documented as independently open
(`display.rs:441-450`), and the value drag's treatment is a ratified
policy with a stated reason: a gesture dissolved under the pointer is
the half-acted state the refusal exists to prevent. The same walk
applies the opposite rule to the other drag, with no refusal, no
report on `OpOutcome`, and no sentence anywhere saying it is meant to.
One of the two is wrong; which one is the question this item asks.

Note the interaction with `save-is-not-gesture-guarded` and with
`two-gestures-can-be-in-flight-together` (closed): the answer here
probably wants to name what the table is a table OF, not just add a
row.

## What resolving it looks like

Either the door refuses while a free move is open — which means the
mid-gesture table's subject widens past the value gesture, and its
name and its README section widen with it — or the dissolution is
deliberate and gets a report the user can see (`OpOutcome::superseded`
is the existing channel; `frame::supersession_notice` already renders
it, `app.rs:785`). What it must not stay is silent and unstated.
