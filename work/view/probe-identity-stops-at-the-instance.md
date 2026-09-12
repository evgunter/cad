---
id: probe-identity-stops-at-the-instance
kind: issue
title: A free-move driving op names the instance, so nothing below the instance can be refused
status: open
opened: 2026-09-11
refs: [a-keyboard-bump-lands-and-closes-the-pointers-own-probe, the-two-drags-name-their-gestures-in-two-shapes]
---



Filed by `a-keyboard-bump-lands-and-closes-the-pointers-own-probe`'s
unit (2026-09-11) as the half its fix does not close.

## What it is

`SessionOp::PreviewFreeMove` and `SessionOp::CommitFreeMove` name a
`RecipeNodeId` (`crates/viewer/src/session/op.rs:321-335`), and
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
(`crates/viewer/src/widgets.rs:168-211`). That makes this chrome have
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
