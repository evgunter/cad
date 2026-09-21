---
id: the-two-drags-name-their-gestures-in-two-shapes
kind: issue
title: The gesture identity is spelled three ways across two drags, and drag_ops is generic over the difference
status: closed
opened: 2026-09-11
refs: [preview-and-commit-carry-no-gesture-identity, two-hand-written-copies-of-the-g1-gesture-machine]
priority: P1
cost: D
closed: 2026-09-21
branch: vseam/gesture-naming
pr: 2965
---



Found by `preview-and-commit-carry-no-gesture-identity`'s unit
(2026-09-11) while giving the driving operations a target.

## What it is

Six operations now name the gesture they drive, in **three** spellings:

- `SessionOp::PreviewGesture { node, slot, value }` /
  `SessionOp::CommitGesture { node, slot }`
  (`crates/viewer/src/session/op.rs:188-206`)
- `SessionOp::PreviewParamGesture { name, value }` /
  `SessionOp::CommitParamGesture { name }`
  (`crates/viewer/src/session/op.rs:216-227`)
- `SessionOp::PreviewFreeMove { instance, frame }` /
  `SessionOp::CommitFreeMove { instance }`
  (`crates/viewer/src/session/op.rs:331-345`)

Each is right for its own door — the first two are the two doors
`SessionOp::BeginParamGesture` argues for, and the third is a different
drag on a different state. What has no home is the CONCEPT: the session
holds `GestureName` privately (`crates/viewer/src/session.rs:172-190`)
for the value drag's two, and `DisplayState` compares a bare
`RecipeNodeId` — now as the naming predicate it hands `g1::Slot`
(`preview_free_move`, `crates/viewer/src/display.rs:781`, and
`commit_free_move`, `:817`) — and nothing says these are the same kind
of fact.

`crates/viewer/src/widgets.rs:164-237` is where it shows: `drag_ops` is
generic over the gesture vocabulary precisely so one mapping serves
both drags, and the `GestureVocabulary` it takes (`:46-57`) is four
operations built by the caller with nothing holding their targets to
each other. The caller does hold them to each other:
`pane/properties.rs:562-582` builds the three that name a target from
one `node` and one `row.slot`, and the cancel names none. But that is a
convention, not a type.

## Why it is not this unit's

Collapsing it means either one public target type used by all six ops
(a second spelling of the two begins' payloads, unless the begins
collapse too — which re-opens `BeginParamGesture`'s stated decision) or
teaching `drag_ops` to build the triple from a target it is given,
which is the same question as
`two-hand-written-copies-of-the-g1-gesture-machine`. Both are
vocabulary decisions the defect did not force.

## Home

VIEW's: `crates/viewer/src/session/op.rs`,
`crates/viewer/src/widgets.rs`, `crates/viewer/src/session.rs`.

## Closed

2026-09-21, on `vseam/gesture-naming`.

The six operations keep their three payload spellings — each is right
for its own door, and that half of the item was never the defect. What
they are spellings OF now has a type, in the op vocabulary they all
belong to (`crates/viewer/src/session/op.rs`):

- `GestureName` — which gesture an operation names, for every drag in
  the crate; `ValueGestureName` and `FreeMoveName` are its two halves,
  split because their previews carry different value kinds (a number, a
  rigid frame) and each mints its own three driving operations from one
  target.
- `SessionOp::names_gesture` — the one place an operation becomes a
  name, exhaustive over the enum, so an operation that joins a drag
  does not compile until someone writes down which gesture it names.
- `GestureName::cancel` — which of the two cancels a drag's control
  emits, decided by the drag rather than written beside the field.

The session's private `GestureName` is gone; `GestureTarget::name`
returns the public `ValueGestureName` and the two doors compare it, so
the value drag keeps the two-arm exactness it had.

`widgets::GestureVocabulary`'s four fields are **private to
`widgets.rs`**, and `widgets::value_gesture` / `widgets::free_move_gesture`
are the only way to build one. A panel therefore spells its target once
and writes no operation at all: the convention this item is about is a
type now, and `pane/properties.rs`'s three sites are the proof, each
one call.

**What the rows pin.** `widgets::tests::every_operation_one_drag_emits_names_the_same_gesture`
drives a real pointer drag through both vocabularies and reads the
names back off the ops the widget emitted.
`gesture_table::every_driving_operation_names_one_gesture` ranges over
`every_op`'s per-variant samples and asserts three things: that naming
agrees with a second hand-written copy (`drives_a_gesture`), that
reading a name off an operation and minting the operations back from it
are inverse, and that every minted operation reads back as its own
name. `gesture_table::a_names_cancel_is_its_own_drags` pins the cancel
pairing, which nothing pinned before — `widgets::tests`' two escape
rows read `kind`, which calls both cancels "cancel".

**What they do not pin**, found by a mutation that stays green:
`the-chrome-vocabulary-is-not-held-to-the-target-the-panel-draws`.

**Not done, and deliberately.** The begins' payloads are unchanged, so
`SessionOp::BeginParamGesture`'s stated decision is untouched: the
item's other collapse — one public target type used by all six
operations — re-opens it and is a design question rather than this
unit's.
