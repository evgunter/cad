---
id: a-union-over-a-stepped-plate-refuses-in-the-orders-that-fold-the-plate-early
kind: issue
title: Unions over a stepped plate or a flush engulf refuse 'a piece of a face held as several borders no recorded discard between them' in some member orders and publish in others
status: open
opened: 2026-10-19
priority: P2
cost: M
refs: [a-held-edge-wholly-inside-a-dropped-face-is-recorded-nowhere, coincident-faces-are-not-imprinted-on-each-other-before-the-lump]
---


## What

Two union shapes refuse `Naming(Emission)` "a piece of a face held as
several borders no recorded discard between them" in some member
orders and publish one table in the others. So the member order
decides whether there is an answer, against DM4's "in every member
order". Found by the delta review of PR 5356. The same refusal and the
same order sets were measured on origin/main 33fc2321b3 and on
`emit/borders-closure`, so PR 5356 did not cause it.

## Measured (2026-10-19, `emit/borders-closure`)

**The stepped plate.** The helper is `stepped_plate_and` in `editor-core`
`tests/emit_union_borders.rs`: `[0,3] × [0,2] × [0,2]` less an L
pocket above `z = 1`. Its lower top is `[0,2] × [0,1] ∪ [0,1] × [1,1.5]`,
and a riser stands at `y = 1` for `x` in `[1,2]`.
- `m` = `[-0.5,1.3] × [1,1.6] × [0.5,2.7]` fills the pocket's
  extension. Its `y = 1` wall is coplanar with the riser, same-oriented,
  and merges with it.
- With `t` (a bar across `y`) and an optional diamond prism:
  - **The three fixtures** `a_corner_touching_the_far_half_of_a_joined_seam_is_not_cited`,
    `a_divider_beside_a_joined_seam_does_not_reach_across_it` and
    `a_corner_touching_the_near_half_of_a_joined_seam_is_part_of_the_divider`
    use this plate.
  - With 4 members, 16 of 24 orders refuse: exactly the orders that
    fold the plate before `m` or before `t`. The fixtures pin that set.
  - With 3 members (plate, m, t at `x` in `[1.5,1.7]`), all 6 orders
    publish.
  - With 5 members, splitting `m` into `[-0.5,0.7]` and `[0.6,1.3]` and
    moving `t` to `x` in `[0.2,0.4]`, 80 of 120 orders refuse. A
    scratch probe found that an order publishes exactly when `t` and
    one of the two `m` halves both fold before the plate.

**The flush engulf** (the review's `rev2_p1_flush_engulf`):
- the plate `[0,3] × [0,2] × [0,1]`;
- the slab `[1.4,1.6] × [-1,3] × [0.5,2]`;
- `big`, a side profile at `y = 3.5` extruded `5` toward `−y`:
  `(-0.5,-0.5) (3.5,-0.5) (3.5,1) (1.7,1) (1.7,2) (1.3,2) (1.3,1) (-0.5,1)`.
  Its low tops are flush with the plate's top at `z = 1`, and its rise
  covers the slab.

The orders (slab, big, plate), (big, plate, slab) and (big, slab,
plate) refuse, which are the orders that fold `big` before the plate.
The other three publish `plate:z=1.00+big:z=1.00+big:z=1.00` as
{`big:x=1.30`} / {`big:x=1.70`}.

## Next

Both shapes hold a same-oriented coincident pair: the riser with `m`'s
wall, and the plate's top with `big`'s low tops. This makes FLUSH's
`coincident-faces-are-not-imprinted-on-each-other-before-the-lump` the
first suspect. It has not been shown to be the cause; the discard rows
were not dumped. Once that row lands, re-measure both shapes. If they
still refuse, dump the refusing step's `DiscardRow`s against the
piece that borders none. The three-box reproducer below is the nearest
known relative.

## The three-box reproducer (moved from `a-held-edge-wholly-inside-a-dropped-face-is-recorded-nowhere`, 2026-10-20)

`A = [0,1]³`, `C = [1,2]×[0,1]²` touching `A` at `x = 1`, and
`M = [0.5,1.5]×[−0.5,1.5]²` covering that contact. A `Union` over the six
member orders, measured on PR 5368's head (the fate-change cut landed):
`[a, m, c]`, `[m, a, c]`, `[m, c, a]` and `[c, m, a]` refuse "a piece of
a face held as several borders no recorded discard between them", and
`[a, c, m]` and `[c, a, m]` build, as before the cut.

Read in `[a, m, c]`, the parent is `a`'s top merged with `c`'s. The fold
loses its region to two discards, `a`'s top over x 0.5..1 (when `m`
folds in) and `c`'s top over x 1..1.5 (when `c` folds in). The two meet
along x = 1, each dying with its own member's rim there, two edges no
step ever glued, because `m` covered the contact before `c` arrived. No
binary step meets a coincident pair, so the cut does not reach it; linking
the two discards needs a reading of coincidence the fold never performs
(the `(a, c)` judgement's glue of those rims, or geometry).
