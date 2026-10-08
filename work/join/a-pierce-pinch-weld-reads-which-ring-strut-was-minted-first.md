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
(`boolean/vtxfac.rs`, now `crate::null::ring_tree`).

## What

A pierce ring's struts round the ring vertex are a cyclic order: the
same corners whichever strut is minted first. The ring was first read
clockwise from the sort's first pair, which at k = 2 can be run 1's.
Minting run 1's strut first (the `MevSite::Lone`), the second spliced
before it, gives the same cyclic ring, yet one row went from `OK SOUND`
to `JoinDesync { what: "a pinch face runs through a pierce vertex twice" }`
(`finish::pinch_site`):
`sweep::all join_pierce_runs_sweep::a_pinchs_cones_share_one_point_key`,
pose `Ltop asym seed=2296 fib21`, tag `xy U`.

The ring now starts its walk at the root's corner before the lowest
run, which is main's mint order, and the row builds again. So the pinch weld, or the
join before it, reads something that only the mint order fixes: which
strut holds the ring loop's anchor, or which copy is minted first.

## Owed

Find what `pinch_site` (or the zips before it) reads that a rotation of
the same ring moves, and make it read the ring's cyclic order only. The
pin: mint the ring's struts from each rotation in turn at that pose, and
every rotation builds `OK SOUND` alike.

## Every root of the ring (2026-10-08)

The ring is now a tree of struts (`crate::null::ring_tree`), rooted at
the region bordering the most runs, and
`topo::test_support::with_ring_root` mints it from any region. At the
k = 2 poses of `a_pinchs_cones_share_one_point_key` the ring has three
regions: the hub, bordering both runs, and a leaf inside each run's
chord. Measured over its six poses, both operand orders
(`sweep::all join_pierce_runs_sweep::a_pinchs_unions_from_every_root_of_the_ring`):

- the hub (the default) and the leaf inside run 0's chord build
  `OK SOUND` everywhere; from that leaf, run 0's strut is at the ring
  vertex and run 1's hangs off its far end;
- the leaf inside run 1's chord refuses `JoinDesync` "a pinch face runs
  through a pierce vertex twice" on all four `Ltop asym` poses, both
  orders, and builds on the two `asym asym` poses. From it, run 1's strut
  is minted first and run 0's hangs off its far end.

So the root does not decide it: the refusal follows the strut minted
first, run 1's, from the hub as from the leaf. The row asserts that
refusal on those eight lines, and is this item's flip-back row: they
must build once it is fixed.

`finish::pinch_site` lies on CLEAVE/HONE ground by `work.py territory`
(`crates/topo/src/boolean/finish.rs`), though JOIN has owned the pinch
welds historically.

At k ≥ 4 the default root's walk does not always mint run 0 first:
measured over every closed meander (PR 4300's second review), 6 of 42
at k = 4 up to 5,090 of 13,820 at k = 7 mint another run first. No row
measures a pinch weld meeting such a ring.
