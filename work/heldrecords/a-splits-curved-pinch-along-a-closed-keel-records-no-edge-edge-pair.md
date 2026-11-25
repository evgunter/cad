---
id: a-splits-curved-pinch-along-a-closed-keel-records-no-edge-edge-pair
kind: issue
title: A split whose two pieces touch along a closed curved keel mints two coincident circles and records no edge-edge pair, which tier 3 prime refuses
status: open
opened: 2026-11-23
priority: P2
cost: M
---

Found by `heldrecords/split-on` (plan step 3).

## What

`crates/sweep/tests/a_curved_keel_in_the_plane.rs` `keeled_round`: the
full annulus less the full V, split by `y = 1`. Read from the V's side
(the plane's normal `−y` below, `+y` above), that side is two pieces
touching along the keel's circle, each holding its own closed circle
edge there. The split records neither pair: its slit records come from
a tree's slit (`crates/topo/src/splitting/join.rs` `Tree::slit`, which a
closed edge is not part of) and its pinch records from line edges
sharing both end point keys (`finish.rs` `Sides::minted_contacts`,
`on_lines`). Measured on `heldrecords/split-on` at the default ε, the
tip at 0 and `±0.6 ε`: `validate_pseudomanifold` on that side against its
records refuses `UnrecordedContact { EdgeEdgeOverlap }` on the two
circles, and `CensusUndecidable` beside it. The side passes tier 3 and
the split's gate, which confirms records and searches for none
(`the-gate-raises-no-unbacked-meeting-until-the-census-reads-what-doors-build`).

## Direction

The split records two closed curved edges over one operand edge ON the
plane as it does a slit's arcs, citing that edge's `SplitOn` row: read
which side edges lie over the operand's closed edge (the join's `closed`
halves on each loop), and record each pair one side holds. The
`CensusUndecidable` beside it is its own reading, to be measured once
the pair is recorded.
