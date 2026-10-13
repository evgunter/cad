---
id: split-rows-are-chased-with-no-cycle-check
kind: issue
title: The boolean's edge-split and cone-split rows are chased by readers with no bound and no check where they are written, so a cycling row hangs the reader
status: closed
opened: 2026-10-13
priority: P3
cost: E
refs: [null-edge-copy-rows-are-checked-for-a-cycle-by-their-reader]
closed: 2026-10-13
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
type. The guard is that runtime refusal at append (a writer that
recorded a key it did not mint ends the op as `LineageRow` rather than
hanging a reader), not a second measurement of the writers.

Territory: two of the three sites are other programs' ground —
`ops::input_cell` and `BooleanNaming::settle_cones` in `ops.rs`
(cleave/hone, reachhold), and `split_root` in `boolean/mod.rs`
(cleave/hone); the writers `reduce::split_logged` and
`zip::split_cones` sit there too.

## Closed

Measured first: with a probe on both writers, the topo, sweep and
editor-core ci suites (8,515 tests, ε 1e-9) recorded 15,402 reductions
with edge splits and 334 copies per cone, and no row whose minted key
an earlier row named. `reduce::split_logged` records the child
`split_edge` just minted, and `zip::split_side` the vertex `mev_null`
just minted.

- **Edge splits.** The reduction records them through `EdgeSplits`
  (`crates/topo/src/boolean/mod.rs`), which pushes each operand's
  `(child, parent)` onto a `crate::lineage::EdgeDivisions` (the
  `Minted` core over edges) and refuses a non-fresh child as
  `BooleanError::LineageRow`. `EdgeSplits::root` is total, and it is
  the one walk: `mod.rs`'s `split_root` is gone, and `ops::input_cell`
  and `piece_classes` call it.
- **Copies per cone.** `zip::split_cones` records them on a
  `crate::lineage::ConeCopies`, now the type of the public
  `BooleanNaming::cone_splits` (rows `(copy, split)`), refusing a
  non-fresh copy the same way. `Settler::settle_cones` reads
  `ConeCopies::with_copies` (finite, since a vertex's copies form a
  tree), and editor-core's `Fused::origin` reads `ConeCopies::root`
  instead of its own unbounded chase.

The guard is the refusal at append, not the measurement.
