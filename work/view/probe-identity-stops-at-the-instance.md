---
id: probe-identity-stops-at-the-instance
kind: issue
title: A free-move driving op names the instance, so nothing below the instance can be refused
status: closed
opened: 2026-09-11
closed: 2026-09-12
refs: [a-keyboard-bump-lands-and-closes-the-pointers-own-probe, the-two-drags-name-their-gestures-in-two-shapes]
---



Filed by `a-keyboard-bump-lands-and-closes-the-pointers-own-probe`'s
unit (2026-09-11) as the half its fix does not close.

## What it is

`SessionOp::PreviewFreeMove` and `SessionOp::CommitFreeMove` name a
`RecipeNodeId` (`crates/viewer/src/session/op.rs:331-345`), and
`DisplayState` answers by comparing that against the open gesture's
instance (`crates/viewer/src/display.rs:778-780`, `:807`). The value
drag's rule is stated at `SessionOp::PreviewGesture`
(`crates/viewer/src/session/op.rs:180-187`): *"a drag that could not
open cannot steer the one that did"* — a second drag's begin is
refused `GestureInFlight`, and the driving pair's payload is what
refuses its preview and its commit with it.

The probe cannot deliver that, because its payload's finest grain is
the instance and an instance has one probe. Two drivers naming the
same instance are indistinguishable at the door: the second's begin is
refused `FreeMoveInFlight`, its preview overwrites the first's frame,
and its commit lands it and closes the probe the first is still
holding.

`gesture_table.rs`'s `a_drag_on_another_field_cannot_steer_the_open_one`
is the value drag's row for this. There is no free-move counterpart,
and the reason is that there is nothing for one to assert.

## Why it is not closed by the unit that filed it

That unit fixed the reachable instance — the chrome had three boxes
each spelling the whole triple, so the keyboard could be the second
driver — by mapping the probe row ONCE
(`crates/viewer/src/widgets.rs:239-282`). That makes this chrome have
exactly one driver per probe, so no route reaches the hole today. It
does not close the hole: a second driver on one instance (a viewport
gizmo, a second panel, a scripted batch) would walk straight into it,
and *unreachable from today's chrome* is the claim
`free-move-in-flight-refusal-has-no-reachable-producer` was filed on
and that turned out to be false twice over.

## The fork

Naming the COMPONENT is dead: the op takes any rigid `Frame` and the
chrome's three translation boxes are one chrome's decomposition, not
the door's.

Naming the GESTURE — a token the begin mints — cannot be spelled from
the chrome as it stands: `widgets::drag_ops` builds the whole
vocabulary as VALUES before any of them is performed, and the typed arm
literally builds `vec![Begin, Preview, Commit]`, so no payload in that
batch can carry something the begin returned. A client-minted id works,
but then the chrome is the thing deciding which drivers are one
gesture — which is where the fix already is.

So the live question is whether the door should refuse a second driver
at all, or whether *one probe per instance, driven by whoever names it*
is the ratified rule and the prose should say so.
`crates/viewer/README.md`'s *"A driving operation names its own
gesture"* section states the asymmetry as it stands. It rides with
`the-two-drags-name-their-gestures-in-two-shapes`, which asks where the
gesture-identity CONCEPT lives.

## Home

VIEW's: `crates/viewer/src/session/op.rs`,
`crates/viewer/src/display.rs`, `crates/viewer/README.md`.

## Closed: the door is right, and the rule is now stated

Answered rather than fixed. The fork's premise was an asymmetry with
the value drag, and the asymmetry is not there.

**Two drivers on one subject are not refused for the VALUE drag
either, and that is the argued design.**
`crates/viewer/tests/gesture_table.rs`'s
`the_open_drags_own_field_dragged_again_lands_its_number` asserts
exactly the three behaviours this item calls a hole: the second
batch's `SessionOp::BeginGesture` is refused `Refusal::GestureInFlight`,
its preview steers the open gesture, and its commit lands it and ends
the drag the first was holding. `crates/viewer/README.md`'s *A driving
operation names its own gesture* argues it as the whole difference
between a target and a token — *"A token minted per begin would refuse
them and strand the reader a second time."* So *one gesture per
subject, driven by whoever names it* was already the ratified rule of
the value drag; the probe obeys it at its own subject, and the payload
refusing nothing below the instance is that rule and not a gap in it.

**The item compared the wrong pair.**
`a_drag_on_another_field_cannot_steer_the_open_one` is the value
drag's row for a second SUBJECT, not a second DRIVER, and the probe's
counterpart for it exists —
`a_probe_on_another_instance_cannot_steer_the_open_one`, added by
#2361. This item's *"There is no free-move counterpart, and the reason
is that there is nothing for one to assert"* is false at both
readings. What was genuinely missing was the probe's counterpart for
the second-DRIVER row, which had nothing to do with the door and
everything to do with the suite. It is
`the_open_probes_own_instance_driven_again_lands_its_frame`.

**The probe's second driver is not hypothetical, and it is not a
gizmo.** No gizmo, second panel or scripted chrome is on any roadmap —
the word `gizmo` occurs in this tree only in this file. The reachable
second batch on one instance is the one a reader makes recovering a
STRANDED probe, and this item's own sibling
`free-move-in-flight-refusal-has-no-reachable-producer` had already
traced the strand and left it: *"The hole left is the SELECTION, which
no prune covers: `instance_ui` is drawn only for `selection().node()`,
so a `Select` performed under an open probe would take the field away
with the drag still live."* `SessionOp::Select` is permitted mid-probe
(`SessionOp::permitted_during_free_move`), and the hand that reaches it
under a held drag is the hand
`a-keyboard-bump-lands-and-closes-the-pointers-own-probe` established:
the feature tree's row is a `selectable_label(…).clicked()`
(`crates/viewer/src/pane/features.rs:59-60`), and egui answers
`clicked()` for a focused widget's Space/Enter and for an AccessKit
`Action::Click` with no pointer anywhere. Re-select the instance, drag
a box, and the batch lands the frame and ends the probe — which is the
right answer, and the one a per-begin token would break.

So the guard this item asked for is not a guard that cannot fire
today; it is a guard that would fire on the one route that exists, and
refuse it wrongly.

**What landed** (#PR): the rule stated where the asymmetry was stated
— `crates/viewer/README.md`'s *A driving operation names its own
gesture* and `SessionOp::PreviewFreeMove`'s doc — and
`the_open_probes_own_instance_driven_again_lands_its_frame` holding
it. Three mutations red it: permitting a second `begin_free_move`,
refusing `Select` mid-probe, and inverting `commit_free_move`'s name
check.

No residue. `the-two-drags-name-their-gestures-in-two-shapes` still
owns where the gesture-identity CONCEPT lives; nothing here moves it.
