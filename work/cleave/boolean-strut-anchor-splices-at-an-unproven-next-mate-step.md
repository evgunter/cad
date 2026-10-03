---
id: boolean-strut-anchor-splices-at-an-unproven-next-mate-step
kind: issue
title: boolean insert's strut_anchor and mint_run step next(mate(·)) without proving the step starts at the site vertex, and a strut site's mev proves only the walk from the step it is handed
status: open
opened: 2026-10-03
priority: P3
cost: E
refs: [vertex-orbit-reads-no-start-vertex, kill-ops-anchor-emanating-on-an-unproven-next-mate-step]
---


## What

Found by the receipt of `vertex-orbit-reads-no-start-vertex` (topo).

In `crates/topo/src/boolean/insert.rs`, `strut_anchor` and `mint_run`
each step a half-edge one place clockwise with a local `successor`
closure (`body.mate(he)`, then that half's `next`), and neither reads
the start of the half it lands on. `strut_anchor` keeps stepping while
the half it holds is a strut hung earlier at `vertex`. `mint_run`
steps once: from the sector's own half for a dangling strut, or from
the run's last half for `he2`. Either result goes into
`MevSite::Fan { he1, he2 }`.

For a run, `Body::mev_fan_plan` refuses an `he2` that starts somewhere
other than `he1` (`FanStartMismatch`). It also refuses a walk from
`he1` that leaves `he1`'s start, which is the orbit walk's own proof.

For a strut site (`he1 == he2 == he`) no read ties `he` to `vertex`. A
torn `next` puts `he` at another vertex. The plan then proves the walk
from `he` stays at *that* vertex, and the split hangs the strut there
instead of at `vertex`.

These are one-step reads, not closed walks, so the orbit walk's start
proof does not cover them. They are the shape the kill operators had
(`kill-ops-anchor-emanating-on-an-unproven-next-mate-step`), on the
boolean's ground.

## The shape to give

Make `successor` refuse `BooleanError::corrupt_at(operand, vertex)`
when the half it lands on does not start at `vertex`, in both
functions.
