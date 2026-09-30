---
id: loop-cycle-broken-display-names-one-of-its-causes
kind: issue
title: LoopCycleBroken's Display says the walk never reaches the second half-edge, which is one of the three faults the variant now names
status: dispatched
opened: 2026-09-29
priority: P3
cost: E
---


## What

Found by PR 3495 (`kill-ops-loop-anchor-on-an-unproven-next-step`),
which gives `EulerOpError::LoopCycleBroken` (`crates/topo/src/euler.rs`)
two more causes. Its doc comment now names three: a cycle walk fails
to close, the walk closes without the half-edge it had to reach, or a
kill's `next` step disagrees with the loop's members (the anchor lies
in another loop, an emptied loop keeps a member, a moved run claims
another loop). Its `Display` arm still reads "loop {loop}'s cycle walk
never reaches the second half-edge (malformed body)". That describes
only the second cause. Before PR 3495 it already misdescribed the
first: `kef`'s and `movefac`'s walks that fail to close, and
`mekr`'s ring walk.

PR 3495 left the arm alone because another TOPO lane
(`topo/route-refusal-subjects`) was rewriting error `Display` arms at
the same time.

## The shape to give

A `Display` arm that names what the variant names, a malformed cycle
or membership of the loop, without claiming one cause, or a split of
the variant if a caller needs to tell the causes apart.
