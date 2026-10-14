---
id: a-pose-defined-split-tool-names-no-tool-cell
kind: issue
title: A split whose tool is a pose definition has no tool node, so a tool-cell coincidence row cannot be named
status: open
opened: 2026-10-10
---

## What

A split's tool cell in a coincidence row is named by the node the tool
reads (`NamedCell::Tool { input }`, built in
`crates/editor-core/src/coincide.rs:473` from `RowInputs::tool`). Since
INTENT stage 3 A a split's tool may be a pose definition, which has no
operation: `wire_split` passes `doc.operation_of(tool)`
(`crates/editor-core/src/eval/wire.rs:348`), `None` for a pose
definition, so a row the kernel emits with a `RowCell::Tool` cell (an
operand vertex decided ON the plane whose neighbourhood leaves two runs
on a side, `crates/topo/src/splitting/mod.rs:717`) would fail naming with
"a coincidence row names a cell no input of its node names" instead of
building.

Not measured end to end: the one configuration that reaches the row — a
plane touching a notch's apex — refuses earlier for a datum tool too
(`work/issues/a-split-touching-at-a-notch-apex-mints-a-vertex-twice.md`).

## Fix shape

FORK-DM4 unit 1 (PR 4527) re-keys `NamedCell::Tool` by the read rather
than a node; a pose-definition tool is then a read like any other. Check
this case when that lands, with a row once the apex refusal is fixed.
