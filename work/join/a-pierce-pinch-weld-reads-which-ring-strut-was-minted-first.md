---
id: a-pierce-pinch-weld-reads-which-ring-strut-was-minted-first
kind: issue
title: The pinch weld refuses JoinDesync when a k = 2 pierce ring mints its struts in the other cyclic rotation
status: open
opened: 2026-10-08
priority: P2
cost: M
---


Found by TANG (2026-10-08) while landing the ring's corner order
(`boolean/vtxfac.rs` `ring_corners`).

## What

A pierce ring's struts round the ring vertex are a cyclic order: the
same corners whichever strut is minted first. `ring_corners` read them
clockwise from the sort's first pair, which at k = 2 can be run 1's.
Minting run 1's strut first (the `MevSite::Lone`), the second spliced
before it, gives the same cyclic ring, yet one row went from `OK SOUND`
to `JoinDesync { what: "a pinch face runs through a pierce vertex twice" }`
(`finish::pinch_site`):
`sweep::all join_pierce_runs_sweep::a_pinchs_cones_share_one_point_key`,
pose `Ltop asym seed=2296 fib21`, tag `xy U`.

`ring_corners` now rotates its corners to start at run 0, which is
main's mint order, and the row builds again. So the pinch weld, or the
join before it, reads something that only the mint order fixes: which
strut holds the ring loop's anchor, or which copy is minted first.

## Owed

Find what `pinch_site` (or the zips before it) reads that a rotation of
the same ring moves, and make it read the ring's cyclic order only. The
pin: mint the ring's struts from each rotation in turn at that pose, and
every rotation builds `OK SOUND` alike.
