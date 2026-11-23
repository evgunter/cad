---
id: the-tree-view-nests-a-nodes-inputs-under-its-consumer
kind: issue
title: the flat feature tree dropped the 'tools one level in' grouping; a later view decides how inputs nest
status: open
opened: 2026-11-01
priority: P2
cost: M
design: true
---

## Finding

The feature tree draws every node row in one column, in
`Evaluation::order` (`crates/viewer/src/tree.rs`, module doc "Order and
columns"; `crates/viewer/src/pane/features.rs`, `feature_row_ui`). It
replaced a per-row branch count (a node at its primary input's depth,
one deeper than every other input), which, drawn in topological order,
made a row read as the child of whatever evaluated before it
(`the-tree-nests-rows-under-unrelated-placements`).

The flat rows claim nothing false, and they drop the grouping the
branch count was for: a boolean's tool, or a placement's mated body,
no longer reads as one level in. A chain of twenty cuts reads as one
column, its tools indistinguishable from the body being cut.

## The option a later view weighs

**B: a node's non-primary inputs nest under the node that consumes
them.** The primary line stays in one column in evaluation order. Each
consumer row is followed by an indented block of the rows only it reads
as a non-primary input, in their own evaluation order:

```text
Profile 1 / Extrude 1
Union
  Profile 2 / Extrude 2 / Place (the boss, stood on Extrude 1)
Subtract 1
  Profile 3 / Extrude 3 / Place (the cylinder)
Place (the bracket, in the world)
```

The order is then no longer evaluation order: a block's rows evaluate
before the row they hang under. Two sub-rules B leaves open, each a
decision of its own:

1. **A node read by several consumers.** In the replay above
   `Extrude 1` is read by the boss's profile (its face), the boss's
   placement (a mate to its face) and the `Union`. It stays on the
   primary line, nests under its first consumer, or is drawn once per
   consumer with a cross-reference.
2. **Whether a placement's constraint reads pull a body into its
   block.** A `Place`'s primary input is its `source`; its constraints
   read faces of other bodies (`Doc::upstream` lists them). Those are
   inputs in the DAG and not in the sense a person reads "made from".

Ev plans the tree view this belongs to; this row records what the
flat rows gave up and what that view has to settle.
