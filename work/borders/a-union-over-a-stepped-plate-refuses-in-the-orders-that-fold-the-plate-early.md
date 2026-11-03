---
id: a-union-over-a-stepped-plate-refuses-in-the-orders-that-fold-the-plate-early
kind: issue
title: Unions over a stepped plate or a flush engulf refuse 'a piece of a face held as several borders no recorded discard between them' in some member orders and publish in others
status: open
opened: 2026-10-19
priority: P1
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

## Another witness, newly reachable (REFS, PR 6342 review, 2026-11-02)

`Union[Union[plate, boss], c1, c2, bar]` refuses `Emission { what: "a piece of a face held as several borders no recorded discard between them" }` (`names/borders.rs`) in 16 of 24 member orders, and names with one table in the other 8. The parts:

- plate `[0,3]² × [0,1]`; boss of radius 0.3 about (1.5, 1.5), z 0.5 to 1.5, its wall two semicircles;
- c1 = `block_on(plate, (1.0,1.5), (1.0,2.0), 1.3, 0.5)` and c2 = `block_on(plate, (1.5,2.0), (1.0,2.0), 1.2, 0.6)`, two caps meeting face to face at x = 1.5;
- bar = `block_on(plate, (1.4,1.6), (0.5,2.5), 0.8, 1.2)`.

The three-member form `[Union[joined, c2], c1, bar]` refuses the same way in 4 of 6 orders.

Controls:

- the caps without the bar name in all 6 orders;
- the caps 0.1 apart, with the bar, name in all 24 orders;
- with `Links::judged` restricted to cross-member pairs (main before PR 6342), every order refuses as the typed `UnsupportedUnjudgedGlue`.

So PR 6342's same-member link moves this recipe from a uniform typed refusal into this row's order-dependent class. The reviewer's probe source, with these fixtures, is in the PR 6342 review record; the fixtures above are enough to rebuild it.

## Measured under the per-face rule (REFS, PR 6381, 2026-11-03)

PR 6381 names a boolean's faces for exactly the member faces each holds,
read off the kernel's holdings, and takes a face's `Borders` walls over
the lost region of the member faces the finished faces join. A holder
that meets no divider takes no wall, and the emission refuses only where
two faces would then share one name. On the three stepped-plate fixtures
of `tests/emit_union_borders.rs`:

- `a_corner_touching_the_far_half_of_a_joined_seam_is_not_cited`: all
  24 orders publish one table (16 refused before).
- `a_corner_touching_the_near_half_of_a_joined_seam_is_part_of_the_divider`:
  all 24 orders publish; the 8 that fold the diamond after the plate and
  `m` do not cite the diamond, the rest do. That split is
  `borders-records-a-join-twice-and-reads-lineage-three-ways`'s.
- `a_divider_beside_a_joined_seam_does_not_reach_across_it`: unchanged
  in shape; (plate, m, t) and (m, plate, t) still over-link.

The flush engulf and the three-box reproducer were not re-measured.

### The near half's 8 orders publish order-dependent walls (PR 6381 review)

The review asked that the 8 near-half orders refuse again rather than
publish walls that depend on member order. The naming cannot tell them
apart from a right answer: in those orders the diamond's lost region
meets the divider at one corner of the lower top, which is also exactly
what the far half looks like in every order (whose right answer is "not
cited"). A refusal keyed on that shape (an obstacle that divides nothing
sharing a corner with a divider in one holder's boundary) was tried in
`Obstacles::walls`: it refused all 24 orders of both the far and the
near half, (0, 2, 1) of `a_divider_beside_a_joined_seam_does_not_reach_across_it`,
and `split_join` of `an_earlier_seam_split_and_joined_away_in_one_step_names_alike`,
so it was not kept. Which half of a joined edge a corner lies on has to
come from the record (the join's own row), which is this row's and
`borders-records-a-join-twice-and-reads-lineage-three-ways`'s work; until
then the 8 orders publish a table that differs from the other 16, a
known order dependence and a P2 here.

## Re-banded P1 (REFS orchestrator, 2026-11-03, at PR 6381's merge)

Since PR 6381 (per-face naming), `emit_union_borders::a_corner_touching_the_near_half_of_a_joined_seam_is_part_of_the_divider` publishes, in 8 of 24 member orders, a table that drops the diamond's walls from the right piece. Main refused those orders. That is a silently wrong name that depends on member order, against N2's "every member order names alike". A refusal keyed on what the naming can see over-refused: all 24 orders of both halves, plus two other rows. So the fix has to come from the join's record of which half of a joined edge a corner lies on (see `borders-records-a-join-twice-and-reads-lineage-three-ways`). Band P1, per the PR 6381 review.
