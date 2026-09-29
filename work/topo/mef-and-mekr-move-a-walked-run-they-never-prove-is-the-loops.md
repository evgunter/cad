---
id: mef-and-mekr-move-a-walked-run-they-never-prove-is-the-loops
kind: issue
title: mef's chord run and mekr's ring walk are re-parented without proving they claim the walked loop: a walk diverted through a third loop moves that loop's anchor out of it through Ok
status: open
opened: 2026-09-29
---


## What

Found by the second pass of `kill-ops-loop-anchor-on-an-unproven-next-step`'s
receipt (PR 3495). That pass looked at the writes that move a loop's
anchor without writing the loop record: a `parent_loop` write to a
half-edge that another loop names as `first`. PR 3495 makes `kef` and
`kemr` prove that the run they move claims the loop they walked
(`KillRun` in `Body::require_kill_anchors`, `crates/topo/src/euler.rs`).
Two make-side writers move a walked run the same way, and prove only
that the walk closes:

- `Body::mef` (`crates/topo/src/euler.rs`, the chord site's run
  `[he1 .. he2)`, re-parented into the new loop);
- `Body::mekr` (`crates/topo/src/euler_ring.rs`, the ring's
  `loop_cycle_live` walk, re-parented into the target loop).

A torn `next` can divert the walk through a third loop and back. The
run then carries that loop's members, its `first` among them, into
the new loop, and the third loop is left anchored in another loop.

## Measured

A throwaway probe at PR 3495's head (release, debug assertions off)
ran `review_d18`'s three `FIXTURES`, seeds 1..=100 of one and two
`NextForeign` tears, and `mef_chord(Chords { he1, he2 })` at every
ordered pair of half-edges: 750,400 calls, 79,736 `Ok`, and 89 of
those leave a loop whose `first` is dead or lies in another loop
(the probe's `loop_off` reading). The first is `ops_strut_cube`,
seed 26, two tears, `he1 = halves[4]`, `he2 = halves[13]`. `mekr` was
not measured.

## The shape to give

Each plan proves that its run claims the loop it walked, and refuses
`LoopCycleBroken` naming that loop. `Body::require_kill_anchors`'s
run check is the primitive: lift it into a shared helper, or call it
from these plans. Pin one counterexample per operator.
