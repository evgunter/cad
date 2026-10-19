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

## Evidence from `fuse/split-hop-lineage-at-append` (2026-10-14)

The table is now written: the cone graze in
`split_tangent_edge_curved` (`a_cone_graze_records_each_rejoined_piece_with_the_edge_it_was_split_from`)
records one or two rows at eight of ten azimuths, each the killed edge
with the edge its join kept, and some carry a record older than the cut
(`8v3 ← 10v1` is not an `edge_pieces` row). A probe over the topo,
sweep and editor-core ci suites (8,557 tests) found 112 rows over 74
splits, and dropping them left every test green, so what this row asks
for still holds: no row's naming outcome reads the table. No split in
those suites killed a child and then its parent on one side (the order
a mint-freshness check refuses); the profile segments are lines and
arcs, so a plane touches one edge at most once per cut, and a second
joinable vertex along one lineage would have to be one the operand
already held.
