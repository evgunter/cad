---
id: a-pierce-strut-after-half-may-be-moved-by-the-other-run
kind: issue
title: vtxfac's bisector-only pierce run reads its strut corner 'after' off the entry table after the other run's fan may have moved it
status: dispatched
opened: 2026-10-05
priority: P2
cost: M
branch: cleave/pierce-strut-after
---


Found by the sweep in the PR closing
`work/topo/rc-wide-battery-panics-at-the-orbit-step-unreachable`, which
was the same shape in `insert.rs`: a later run at one vertex reading a
sector half that an earlier run's fan had already carried to its copy
vertex. Not reproduced; a question to prove or refute.

## The read

`crates/topo/src/boolean/vtxfac.rs`, the piercing-side loop over
`runs` (up to two Out-runs at the piercing vertex, minted in turn): a
run with no real-edge member hangs a strut at
`after = entries[(run.0 + run.1) % n].he`, read off the entry table
built before either run was minted. `mev_null` then splices the spike
at `after`'s start vertex, whatever that now is; nothing checks it is
still `vertex`. (A run with real-edge members reaches `mev` through
`run_site`, so a moved half there refuses `Euler(FanStartMismatch)`.)

## Why it might be reachable

Entries are pieces of physical sectors; twins share their sector's
`he`. If the first run's last real-edge member is the entry that crosses
into physical sector X (so its fan moves `X.he`), and the second run is
bisector-only inside X with a non-Out twin between them, then the
second run's `after` is `X.he`, already at the first run's copy. The
strut then hangs at the copy, mis-placed silently: the same state that,
in `insert.rs`, the join refused downstream as `JoinDesync` on all 56
battery poses that reached it.

## Done when

Either a proof in `vtxfac.rs` that the two runs never share a physical
sector this way, or the same typed refusal `insert.rs` `mint_directed`
gives (`ClassificationInvariant`, "an earlier run at the vertex carried
a strut's corner to its copy") when `after` no longer starts at
`vertex`, with a witness pose if one exists.
