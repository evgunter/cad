---
id: a-mate-sides-docs-name-a-retired-select-kind
kind: issue
title: A mate side's docs name a retired evaluation refusal, and head_of's kind check is now a backstop
status: open
opened: 2026-10-17
---


## Finding

Found by WIRE's `wire/selection-kind-at-the-door` unit (rows
`the-entity-doors-key-comes-from-its-caller` and
`the-designation-road-resolves-before-it-asks-the-kind`), which moved a
selection's per-name kind check from evaluation to the edit and load
doors (`Select::fault`'s `SelectionFault::Kind` arm) and deleted
`NodeErrorKind::SelectKind`. The unit was fenced off mate code (D10
stage 3 rebuilds it), so two doc comments there still describe the old
road:

- `crates/editor-core/src/mate/member.rs`, `head_of`'s doc: *"it holds
  a name of another kind. Each is the mate's evaluation's refusal in its
  own words (the read's `UnresolvedRead`, the selection's `SelectKind`)"*.
- `crates/editor-core/src/mate.rs`, `MateFault::SideUnresolved`'s doc:
  *"The mate's evaluation refuses in the read's own words
  (`UnresolvedRead`, `SelectKind`)"*.

Neither variant word is a rustdoc link, so nothing fails; the prose is
wrong. And the behaviour they describe moved: a saved mate side whose
`Face` selection names a body, an edge or a vertex now refuses at load
(`SnapshotError::SelectionShape { fault: SelectionFault::Kind { .. } }`,
re-baselined in `crates/editor-core/tests/edit_one_predicate.rs`
`a_saved_mate_side_that_is_not_a_face_refuses_at_load`), and the edit
door refuses one at the seat (`SelectionFault::Seat`). So `head_of`'s
`name.kind` check (through the one face-name constructor) is a backstop
no stored document reaches, and a side that `head_of` answers `None`
for over kind is no longer a `MateFault::SideUnresolved` a document can
produce.

## What is owed

When stage 3 rebuilds the member walk: drop `SelectKind` from both
docs, and decide whether `head_of`'s kind arm stays as a stated
invariant backstop or goes.
