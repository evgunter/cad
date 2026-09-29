---
id: kill-ops-loop-anchor-on-an-unproven-next-step
kind: issue
title: kef, kemr and kev re-anchor a surviving loop's first on a next step whose loop no plan proves: a torn next carries a loop anchored in another loop through Ok
status: open
opened: 2026-09-29
refs: [kill-ops-anchor-emanating-on-an-unproven-next-mate-step]
priority: P2
cost: E
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

## Measured

`review_d18::kill_anchors_on_torn_bodies` (an `#[ignore]` row: the
`review_d18` fixtures, seeds 1..=2,000 of one and two `NextForeign`
tears, every kill at every half-edge; release, debug assertions off)
counts, among the `Ok` results, those with a loop whose `first` is
dead or lies in another loop. A `NextForeign` tear moves no `first`
and no `parent_loop`, so each is the kill's write.

| op | calls | `Ok`, loop `first` off (merge base `477a696178`) | after the emanating proof |
| --- | --- | --- | --- |
| `kef` | 400,000 | 10,481 | 652 |
| `kemr` | 400,000 | 22 | 6 |
| `kev` | 400,000 | 14,990 | 14,836 |

## The shape to give

Each plan proves that the half-edge it re-anchors a loop at lies in
that loop after the kill (its `parent_loop`, or its membership in
the moved run), and refuses typed otherwise, naming the loop
(`LoopCycleBroken`, or a variant the owner picks). Pin a
counterexample per operator from the probe above, and extend the
probe's loop column to assert 0.
