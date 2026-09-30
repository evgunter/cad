---
id: kef-and-mekr-kill-a-loop-whose-members-they-read-by-the-walk
kind: issue
title: kef and mekr kill a loop whose members they take from its cycle walk: a member the walk skips keeps a parent_loop naming the dead loop through Ok
status: dispatched
opened: 2026-09-29
priority: P2
cost: E
---


## What

Found by the receipt of `kill-ops-loop-anchor-on-an-unproven-next-step`
(PR 3495), whose second pass looked at the loop writes no anchor
pattern matches: the loops a kill removes rather than re-anchors.

`Body::kef` (`crates/topo/src/euler_kill.rs`, the dying loop `l1`)
takes the dying loop's members from its cycle walk (`loop_cycle_live`
from `he`), moves the remnant into the surviving loop, and removes
`l1`. `Body::mekr` (`crates/topo/src/euler_ring.rs`, the ring loop it
kills) does the same with the ring's walk. Neither proves that the
walk visited every half-edge whose `parent_loop` is the dying loop. A
torn `next` that skips a member closes the walk without it, and that
member keeps a `parent_loop` naming the removed loop.

This is the dual of the `Empty` proof PR 3495 adds (`Body::require_kill_anchors`: a loop
a kill empties keeps no member but the killed halves), for a loop the
kill removes. PR 3495 also proves that the run a kill moves claims
the walked loop, which covers the opposite direction: a walk that
wanders into another loop's members.

## Measured

On the declined cube at PR 3495's head (release, debug assertions
off), one `NextForeign` tear per kill that skips the dying loop's
third member (`next(next(he)) := next(next(next(he)))`), then
`kef(he)`: all 24 kills return `Ok`, and `validate` reports
`DanglingTopology { from: HalfEdge(skipped), to: Loop(l1) }` on each,
beside the `NextPrevMismatch` the tear planted. For `mekr`, the
review of PR 3495 measured 80 of 8,764 one-tear `mekr_chord(Cycles)`
calls returning `Ok` with a `parent_loop` that no longer resolves
(`mef-and-mekr-move-a-walked-run-they-never-prove-is-the-loops` has
the construction).

## The shape to give

`kef`, and `mekr` for the ring it kills, prove that the dying loop's
members are exactly its walk (or the killed halves), and refuse
`LoopCycleBroken` naming the dying loop otherwise, through
`Body::require_kill_anchors` or beside it. Pin the one-tear row.
