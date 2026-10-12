---
id: a-union-keeps-valence-two-vertices-on-the-tubes-seam-rulings
kind: issue
title: A dome rim lying on a tube's wall leaves valence-2 vertices on the tube's seam rulings in the union
status: closed
closed: 2026-10-07
branch: fuse/curved-join
opened: 2026-10-02
priority: P3
cost: M
---

## What

`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`, measured
on branch `tang/abutting-rim` (PR 3823), undeclared:

- `a_dome_sunk_into_the_tube_builds_undeclared` (dz = −1e-3 and −0.3,
  both orders);
- `a_tube_through_the_domes_base_builds_every_op_undeclared`, its two
  unions.

Each union is `(6, 12, 9)` faces, edges, vertices, one shell, at its
closed-form volume and valid at tier 3 and 3′. The minimal body is
`(6, 10, 7)`. The extra two vertices sit on the tube's two seam rulings
(`x = ±1`, `y = 0`), at the heights of rims the union dissolved. At
dz = −0.3 those are `(±1, 0, 1.7)`, where the dome's rim met the wall,
and `(±1, 0, 2.0)`, the tube's own top rim. Each has valence 2: the two
pieces of the ruling, split there by the sweep, and no rim survives to
meet them.

## Why it matters

The body is valid, but not minimal. A later op meets the split rulings
as two edges, and a consumer counting vertices sees two that bound
nothing.

## Direction

The finish stage (or the merge it runs) could fuse a valence-2 vertex
between two edges on one line carrier, the inverse of the sweep's
split, when no record names it. The pinned rows cite this file, so the
fix turns them red at `(6, 10, 7)` knowingly.

## Also on the rim where it crosses a ruling

`a_dome_sunk_across_the_tubes_seam_rulings_builds_every_op_undeclared`
(same file) turns the sunk dome about the axis, so each rim semicircle
crosses a seam ruling mid-arc and the sweep splits it there. The split
vertices stay:

- the intersections are `(4, 8, 6)` on the two-face tube and
  `(4, 10, 8)` on the four-face tube where each semicircle crosses two
  rulings, against `(4, 6, 4)` unturned: valence-2 vertices on the rim
  circle, where no ruling survives to meet them;
- `t ∖ d` is `(7, 16, 12)` and `(9, 26, 20)`, against `(7, 14, 10)`;
- the four-face tube's unions are `(8, 20, 15)` at every turn, the
  rulings split at the rim height and at the top rim.

The same fusion would turn those rows red knowingly.

## Also on a ruling lying on a seam ruling

`crates/sweep/tests/a_ruling_lying_on_a_wall.rs`,
`a_prism_edge_on_the_tubes_wall_builds_every_op_undeclared`: the
prism turned onto a seam ruling (0 or π rad) shares a stretch of it,
and the sweep splits the ruling at the prism edge's ends. Its unions
are `(4, 8, 6)` with the edge inside one wall face, against the tube's
`(4, 6, 4)`, and `(9, 19, 12)` across the rim, against a minimal
`(9, 18, 11)`.

## Closed — the curved join takes them (#fuse/curved-join)

The seam rulings' split vertices are valence-2 vertices between two
pieces of one ruling on one chart's `u = const` family at a regular
point, so FUSE's curved join (`topo::boolean::edge_join`, the `Chart`
arm) joins them. Every row this item named now reads the minimal
`(6, 10, 7)`, and `pi_seam_and_kiss_through_the_boolean`'s `built`
asserts `joinable_vertices` empty on every body it checks.

