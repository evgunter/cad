---
id: the-two-drags-name-their-gestures-in-two-shapes
kind: issue
title: The gesture identity is spelled three ways across two drags, and drag_ops is generic over the difference
status: open
opened: 2026-09-11
refs: [preview-and-commit-carry-no-gesture-identity, two-hand-written-copies-of-the-g1-gesture-machine]
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
holds `GestureName` privately (`crates/viewer/src/session.rs:171-188`)
for the value drag's two, and `DisplayState` compares a bare
`RecipeNodeId` (`crates/viewer/src/display.rs:778-780`, `:806-812`), and nothing
says these are the same kind of fact.

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
