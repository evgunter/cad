---
id: a-split-half-loses-the-lineage-of-a-twice-crossed-edge
kind: issue
title: A split half's split_root stops at a key the other half holds, so two pieces of one twice-crossed edge in one half read as two roots
status: open
opened: 2026-09-30
priority: P3
cost: M
refs: [a-cylinder-split-refuses-missing-upstream-once-its-pieces-rank]
---


## What

`topo::split` carves both halves from one scratch arena
(`splitting/finish.rs`, `SplitNaming`'s docs), and each half keeps
only the provenance records of the entities it holds. Take an edge the
plane crosses twice. `split_edge` keeps the parent key for the first
child, so the second crossing splits that child. The chain on the far
side is then `outer piece → middle piece → parent`, and the middle
piece lives in the other half. Read inside the far half,
`Body::split_root` stops at the middle piece's key. So the two outer
pieces, both in that half, report two different roots. That breaks
`split_root`'s own promise that "every piece of one original edge
reaches one root".

The naming lane now chases across both halves
(`emit_topo::chase_split_edge_to_table`). Two other readers still read
one body at a time:

- `topo::props`' loop flattening mints `carrier_id` from it (the torus
  fold asks whether two arcs are pieces of one original edge);
- `mesh::memo` folds the same root into the curved lane's entry
  identity.

## Evidence

Measured in the naming lane. The cylinder split by the plane through
`(0, 0.2, 0)` with normal `(0, 1, -1)` puts a Below rim piece whose
record names an edge only the Above half holds; see the unit test
`emit_topo::split_edge_lineage`. The effect on the props fold and the
mesh memo has not been measured. The fold declines to fold edges with
different ids, so the likely symptom is a refusal or a missed fold, not
a wrong answer.

## Options

- The carve keeps the dead chain's records in each half, as a
  boolean's `dead_split` rows do for `borders.rs`.
- Or it re-points a record past the keys the half does not hold.
