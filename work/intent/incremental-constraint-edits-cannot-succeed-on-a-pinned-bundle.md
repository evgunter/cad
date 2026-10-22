---
id: incremental-constraint-edits-cannot-succeed-on-a-pinned-bundle
kind: issue
title: AddConstraint, AddValue, AddValues and RemoveConstraint(s) can no longer succeed: every bundle pins its copy, so an increment over- or under-pins
status: open
opened: 2026-10-22
---


Found by the D′1 lane (`a-placement-names-its-target-space`).

D′1 checks every bundle at the door (`MateFault::Unpinned` for one that
leaves a freedom unwritten, an empty one included), so every stored
bundle pins its copy. An increment then has nowhere to land:

- `DocEdit::AddConstraint` (`crates/editor-core/src/edit.rs:549`) adds a
  mate to a pinned bundle and refuses `Overconstrained`, unless the mate
  fixes nothing the bundle fixes, which a pinned bundle leaves no room
  for;
- `AddValue` and `AddValues` (`:568`, `:586`) refuse the same;
- `RemoveConstraint` and `RemoveConstraints` (`:626`, `:640`) leave a
  freedom unwritten and refuse `Unpinned`.

Every caller that changes a bundle now writes it whole
(`DocEdit::SetConstraints`, `Placed::write`); the viewer's mate tool
and the façades do. The incremental doors are kept, wire and all, and
still answer their refusals typed, and the Python binding still binds
them (`Doc.constrain`, `remove_mate`, `add_value`, `add_values`,
`crates/pncad-py/src/py/doc.rs:1527`, `:1958`, `:2012`, `:2048`;
`DocEdit.add_value`, `add_values`, `remove_constraint`, `:4739`,
`:4757`, `:4777`).

Options: retire the five edits (a wire change: logs holding them stop
loading); or make each a whole-bundle rewrite under its old name (a
removed mate taking its values with it and the identity put back,
say), which is a design question for D′2, where a value charts on the
target space.
