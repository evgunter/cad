---
id: split-rows-are-chased-with-no-cycle-check
kind: issue
title: The boolean's edge-split and cone-split rows are chased by readers with no bound and no check where they are written, so a cycling row hangs the reader
status: open
opened: 2026-10-13
priority: P3
cost: E
refs: [null-edge-copy-rows-are-checked-for-a-cycle-by-their-reader]
---

Found by the sweep of `fuse/join-refusals-hold-at-their-type`, which
moved the null-edge copies onto a list that refuses a non-fresh copy
where it is recorded (`crate::lineage::VertexCopies`).

Two more of the reduction's lineage lists are rows `(minted, minted
from)` whose minted key is fresh by construction, and whose readers
chase them with neither a bound nor a check where they are written:

- **Edge splits** (`EdgeSplit`, `crates/topo/src/boolean/mod.rs`):
  `reduce::split_logged` pushes `(parent, vertex, child)` with `child`
  the edge the split minted. `mod.rs`'s `split_root` and the `root`
  closure in `ops::input_cell` walk child → parent in a bare
  `while let`; `ops::split_pieces` walks parent → children forward.
- **Cone splits** (`ConesSplit::splits`,
  `crates/topo/src/boolean/zip.rs`): `split_cones` pushes `(split,
  made.vertex)` with `made.vertex` the vertex `mev_null` minted.
  `BooleanNaming::settle_cones` (`crates/topo/src/boolean/ops.rs`)
  extends its worklist through them with no visited set.

A cycling row (an arena that reused a bare key, a writer that recorded
a key it did not mint) would make each of these walks run forever
rather than refuse. What is owed: the `VertexCopies` shape —
`crate::lineage`'s `Minted` list over `EdgeKey` and `VertexKey`,
pushed where the split is made and refusing a non-fresh row as
`BooleanError::LineageRow` — so each chase is total by the list's
type. Measure first that every writer records the key it minted.
