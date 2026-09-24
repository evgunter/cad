---
id: failed-rows-naming-another-node-outside-mate-faults-draw-no-link
kind: issue
title: Failed rows whose NodeErrorKind names another node (outside MateFault) draw no link to it
status: open
opened: 2026-09-23
priority: P4
cost: E
---


## Finding

`chrome/placer-link` gave a `Failed` mate row a link to the node its
`MateFault::PlacerRefused` names (`crates/viewer/src/tree.rs`,
`repaired_at` and `TreeRow::repair_at`; drawn by
`crates/viewer/src/pane/features.rs`, `failure_lines`). `repaired_at`
is exhaustive over `MateFault` only. A `Failed` row whose error is any
OTHER `NodeErrorKind` arm is not asked the question at all: its words
may name another node and the row draws no link to it.

The sweep (the shape: a `NodeErrorKind` arm carrying a
`RecipeNodeId` besides the failing node,
`crates/editor-core/src/eval/mod.rs`, `enum NodeErrorKind`) hits:

- `MissingInput { input }`, `WrongOperand { input }`,
  `EmptyOperand { input }`, `EmptyHalf { input }`,
  `InstanceOutOfRange { input }` — an operand of the failing node.
- `SeedPinnedSection { section }`.
- `AxisInDifferentPlane { axis, axis_plane, profile_plane }`.
- `DeclareSiteNotAnOperand { at }`.
- `DerivedFrameSection { profile, frame }`,
  `FrameDirection { profile, frame }`.

None is decided either way. Ev's ruling on
`blamed-mates-sends-the-eye-past-the-node-the-fault-says-to-fix`
(option c) covered `PlacerRefused` alone, whose kernel doc calls the
named node *"the node an author goes and fixes"*; the test for each arm
above is the same one — does the kernel's own doc for the arm call the
named node the thing to repair — and nobody has applied it.

**Blind spot of the sweep:** it read `NodeErrorKind`'s own fields.
Node ids nested inside a boxed payload (`BooleanError`,
`TransformError`, `ShellError`, …) were not searched beyond
`PlacementRuleFault`, `NamingError` and `EvalError`, which carry none.

## Where to look

`crates/viewer/src/tree.rs` — `repair_of` is where a second fault
family would be read; `crates/editor-core/src/eval/mod.rs` for each
arm's doc (MSOLVE's and the evaluator's ground; read, do not edit).

Signed: (CHROME implementer lane, `chrome/placer-link`)
