---
id: free-move-drag-dissolved-by-open
kind: issue
title: Open and NewDocument dissolve an in-flight free-move drag with no refusal and no report
refs: [session-clearing-walk-is-hand-maintained-three-times, 1885]
status: closed
opened: 2026-09-05
closed: 2026-09-11
branch: view/gesture-doors
---


Found by the #1885 style review (M3), reading the ratified prose that
PR added against the walk one line below it. Pre-existing; #1885
collapsed the walk into one value and did not change what it does to
this field.

## What happens

`DocSession::clear_for_new_document` (`crates/viewer/src/session.rs:1626`)
calls `self.display.clear()` (`:1628`), and `DisplayState::clear`
sets `*free_move = None` (`crates/viewer/src/display.rs:937`). So an
in-flight FREE-MOVE drag is silently dissolved by `Open` and by
`NewDocument`.

Nothing refuses either door while a free move is open.
`SessionOp::permitted_during_value_gesture`
(`crates/viewer/src/session/op.rs:651`) governs the VALUE gesture only
— its name carries that limit deliberately
(`crates/viewer/README.md`, Gesture safety is data) — and the free-move
arms' own guard is inside `DisplayState`, which refuses
`DisplayFault::FreeMoveInFlight` for free-move OPERATIONS and says
nothing about a document replacement.

## Why it is a defect and not a choice

The two drags are documented as independently open
(`display.rs:609-618`), and the value drag's treatment is a ratified
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
is the existing channel; `frame::Withdrawal::superseded` already
renders it, `app.rs:785`). What it must not stay is silent and unstated.

## Closed — the FREE-MOVE side was the wrong one, and the door refuses

Answered by `view/gesture-doors`. The two drags now get the same
answer at the same door: `SessionOp::permitted_during_free_move`
(`crates/viewer/src/session/op.rs`) is a second exhaustive table,
consulted in `DocSession::perform` right after the value one, and it
refuses `Open` and `NewDocument` with
`Refusal::Display(DisplayFault::FreeMoveInFlight)` — *"finish the
free-move first"*, which names a door the user has (`CancelFreeMove`,
in the toolbar) rather than the other drag's sentence.

**What settled the fork was #2348's `DisplayState::clear` clause, not a
preference between a refusal and a report.** A `Withdrawn` names an
instance and carries a `DisplayFault` about a document, and at a
replacement the only document left to ask is the incoming one, where a
`RecipeNodeId` minted by the outgoing document's counter means another
node or none. So the report half of this item's menu was never
available here; a door that can neither report truthfully nor act
without dissolving a gesture refuses.

**The table's subject did not widen — a second table was added.** The
two drags refuse different sets, and one predicate could serve both
only by refusing the union, which is wrong in both directions: a commit
landing under a probe is pruned and REPORTED (`killed_gesture`, #2348),
which is a better answer than a refusal. What the two tables agree on
is exactly the two doors that REPLACE the document rather than moving
it, because a replacement drops the whole display state and leaves a
prune nothing to report against.

Held by `no_operation_dissolves_an_in_flight_free_move_in_silence`
(`crates/viewer/tests/gesture_table.rs`), which asserts the invariant
over every operation rather than two rows about the two doors: a probe
ends because the user ended it, or because a prune reported killing it,
or the row fails. `the_free_move_table_refuses_exactly_the_replacement_doors`
checks the table against the property (`replaces_the_document`) rather
than against a second copy of its 39 rows.

`save-is-not-gesture-guarded` is NOT in scope and stays closed: it was
answered rather than fixed on 2026-09-04
(`work/view/save-is-not-gesture-guarded.md:5,7`), and `Save` is `true`
in both tables for reasons stated at each.

Residue, with its own file:
`work/view/census-table-in-the-viewer-readme-is-not-its-own-population.md`.
