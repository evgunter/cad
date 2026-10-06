---
id: a-union-keeps-valence-two-vertices-on-the-tubes-seam-rulings
kind: issue
title: A dome rim lying on a tube's wall leaves valence-2 vertices on the tube's seam rulings in the union
status: open
opened: 2026-10-02
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
