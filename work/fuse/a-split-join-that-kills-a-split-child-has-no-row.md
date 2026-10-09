---
id: a-split-join-that-kills-a-split-child-has-no-row
kind: issue
title: No split row's join kills a split child, so SplitNaming::joined_lineage is never read
status: open
opened: 2026-10-08
priority: P3
cost: M
---


## The finding

`SplitNaming::joined_lineage` records, for each edge a side's join
kills that the cut had split off another, the edge it came from
(`crates/topo/src/splitting/mod.rs`, before the join), because a kill
takes its edge's birth record with it. The emitter chases a joined
edge's cover through it (`emit_topo::chase_split_lineage`). Dropping the
record changes no row's outcome (PR 4307's delta review, O2).

Why no row reads it: the join takes `gone` as the edge of the first
half-edge leaving the vertex in arena order
(`crates/topo/src/boolean/edge_join.rs`, `joinable`). A split child is
minted after its parent, so wherever no arena slot is reused the
original-key piece is the one killed, and its own name is in the operand
table, so the chase needs no lineage. A probe of the tilted-rim split
over sixteen azimuths (`a_split_touching_one_rim_at_a_point_keeps_that_rims_name`'s
fixture) found one join at each and an empty lineage at every one.

## What it needs

A split whose join kills a child: either an operand whose arena reuses a
freed half-edge slot for the cut's child, or a cut that splits one
operand edge twice on one side and joins both points (a child then dies
in the second join). Neither is built by a plane against the corpus's
operands today.
