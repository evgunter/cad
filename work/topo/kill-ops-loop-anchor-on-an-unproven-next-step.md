---
id: kill-ops-loop-anchor-on-an-unproven-next-step
kind: issue
title: kef, kemr and kev re-anchor a surviving loop's first on a next step whose loop no plan proves: a torn next carries a loop anchored in another loop through Ok
status: open
opened: 2026-09-29
refs: [kill-ops-anchor-emanating-on-an-unproven-next-mate-step]
priority: P2
cost: E
pr: 3495
branch: topo/kill-loop-anchor-proof
---

## What

Found by the receipt of
`kill-ops-anchor-emanating-on-an-unproven-next-mate-step`, which
made `kef`, `kemr` and `kev` prove the half-edge each re-anchors a
vertex's `emanating` at. The same three plans re-anchor a surviving
loop's `Cycle::first` on a `next` step too, and prove only that the
step resolves, not that it lies in the loop it anchors:

- `Body::kef` (`crates/topo/src/euler_kill.rs`, the splice's
  `(None, false)` and `(Some(b), false)` arms): the surviving loop
  `l2` re-anchors at `d = next(m)`. The remnant's `b` is re-parented
  into `l2` by the kill, so it holds; `d` is not.
- `Body::kev` (`euler_kill.rs`, `kev_execute`'s unsplice): `l1` at
  `b = next(he)` or `d = next(m)`, and `l2` at `d`.
- `Body::kemr` (`crates/topo/src/euler_ring.rs`): the old loop at
  `next(he2)`, the old side's first member, which the kill does not
  re-parent. The ring's first is re-parented with its side, so it
  holds.

A torn `next` into another loop makes the kill write a `first` whose
`parent_loop` is another loop, and return `Ok`.

The same plans also write a loop `Empty`, which says it has no member
left and names the lone vertex it holds, where a `next` equality says
so:

- `Body::kev` (`kev_execute`'s segment kill, `next(he) == m` and
  `next(m) == he`): `l1` becomes `Empty` at the survivor.
- `Body::kef` (the splice's `(None, true)` arm, the `Lone` inverse:
  `next(he) == he` and `next(m) == m`): `l2` becomes `Empty` at `w`.
- `Body::kemr` (an empty side, `next(he1) == he2` or
  `next(he2) == he1`): the ring `Empty` at `w`, the old loop `Empty`
  at `u`.

The emanating proof (`Body::require_kill_anchors`, `euler.rs`)
covers part of this. `kef`'s and `kemr`'s `Empty` writes happen only
where the vertex they hold is anchored at `None`, which the plan now
proves leaves that vertex no half-edge but the killed two. Nothing
proves that an emptied surviving loop (`kef`'s `l2`, `kemr`'s old
loop, `kev`'s `l1`) has no member left outside the walk. `kev`'s
segment test is not its `None` arm's test (an empty merged fan and
`next(m) == he`): where the mate's own edge is another, the two
disagree, and `l1` can become `Empty` at a survivor anchored at the
fan's first member.

## Measured

`review_d18::kill_anchors_on_torn_bodies` is an `#[ignore]` row: the
`review_d18` fixtures plus the genus-2 body and the holed box, seeds
1..=2,000 of one and two `NextForeign` or `EdgeBijection` tears,
every kill at every half-edge, release, debug assertions off. Among
the `Ok` results it counts those with a loop anchor off: a `first`
that is dead or lies in another loop, or an `Empty` whose vertex is
dead, has a half-edge starting at it, or shares the loop with a
member. Neither tear moves a `first`, a start or a `parent_loop`, so
each is the kill's write. The `EdgeBijection` rows are 0 for all
three operators.

| op | `NextForeign` calls | `Ok`, loop anchor off (`d9465eb31e`, the emanating proof) | after the `None` proof |
| --- | --- | --- | --- |
| `kef` | 856,000 | 788 | 788 |
| `kemr` | 856,000 | 73 | 45 |
| `kev` | 856,000 | 25,734 | 25,725 |

On the earlier population (the three `review_d18` fixtures,
`NextForeign` only, `first` writes only) the column read, at merge
base `477a696178` and after the emanating proof: `kef` 10,481 and
652, `kemr` 22 and 6, `kev` 14,990 and 14,836.

## The shape to give

Each plan proves that the half-edge it re-anchors a loop at lies in
that loop after the kill (its `parent_loop`, or its membership in
the moved run), and that a loop it empties keeps no member, and
refuses typed otherwise, naming the loop (`LoopCycleBroken`, or a
variant the owner picks). Pin a counterexample per operator from the
probe above, and make the probe assert its loop column is 0.
