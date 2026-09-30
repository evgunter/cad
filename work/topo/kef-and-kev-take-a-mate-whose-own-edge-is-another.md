---
id: kef-and-kev-take-a-mate-whose-own-edge-is-another
kind: issue
title: kef and kev take the mate from the killed edge's slots and never check the mate's own edge: a torn edge bijection kills an edge that other half-edges still name
status: review
pr: 3570
branch: topo/kill-proves-removals
opened: 2026-09-29
refs: [kill-ops-anchor-emanating-on-an-unproven-next-mate-step]
priority: P3
cost: E
---

## What

Found by the fix pass of
`kill-ops-anchor-emanating-on-an-unproven-next-mate-step`, whose
review planted `EdgeBijection` tears (an edge's slots set to `he` and
a foreign half) under the kill operators.

`Body::kef` (`crates/topo/src/euler_kill.rs`, the preconditions at
the top of `kef`) and `Body::kev` (`kev_plan`, shared by
`kev_describing` and `kev_merged_members`) read the mate `m` out of
`he`'s edge's two slots and check that the edge claims `he`
(`UnclaimedHalfEdge`). Neither checks that `m`'s own `edge` field is
that edge, or that the half the edge used to claim is gone. `kemr`
does (`NotSameEdge`: "one edge which claims them both", both `edge`
fields compared).

Under a torn bijection where `he`'s edge claims a foreign `m`, the
kill removes `he`, `m` and `he`'s edge. `m`'s own edge still claims
the dead `m`, and the half-edge the edge used to claim still names
the dead edge, so the result carries `DanglingTopology` through `Ok`
(debug assertions off), or panics at the tier-1 postcondition (dev).
The orbit walk from `m` also steps through `m`'s own edge's mate
rather than `he`, so the merged fan is not `end(he)`'s fan.

## Measured

The review's `anchorr_c3_bijection` probe (the five fixtures of
`review_d18::kill_anchors_on_torn_bodies`, seeds 1..=1,000 of one and
two `EdgeBijection` tears, `kev` through either door at each half-edge
whose merged fan reads empty exactly where `next(he)` is not the mate,
or the other way round; release, debug assertions off) sees `m`'s
edge differ from `he`'s in every one of its 489 calls. At the fix
pass's head 483 refuse (`OrbitBroken`) and 6 return `Ok`, each of
which leaves `m`'s own edge naming a dead half-edge. The probe selects
on the fan, so it does not size the whole class; a row in the style
of `kill_anchors_on_torn_bodies` counting `DanglingTopology` among
`kef`'s and `kev`'s `Ok` results would.

PR 3495's review adds a constructed case (its S11): the strut cube
with a second strut planted two steps along the top loop, the first
strut's edge torn to claim the second strut's tip half `m` as the
mate of the first's tip half `he`, and `next(m)` torn onto `he`. The
orbit walk from `m` steps through `m`'s own edge and closes at once,
so the fan reads empty; `next(m) == he` with `next(he) != m` is the
mirror arm with the survivor anchored `None`. At the review head
`kev(he)` returns `Ok` through both the ungated kill and `kev`, and
`validate` reports `DanglingTopology`, `HalfEdgeUnclaimed`,
`OrphanEntity` and `UnreachableHalfEdge`. At the fix-pass head it
refuses `OrbitBroken` naming `he`: the anchor proof now requires a
`None` anchor to be held by a loop the kill empties, and the mirror
arm empties none. The refusal comes from the anchor proof, not from a
check of the mate's own edge, so this row's subject stands.

## The shape to give

`kef` and `kev` prove that the mate's own edge is `he`'s (so the
edge claims exactly `{he, m}` and both name it), refusing typed where
it is not, the way `kemr` refuses `NotSameEdge`. Pin one torn
bijection per operator.
