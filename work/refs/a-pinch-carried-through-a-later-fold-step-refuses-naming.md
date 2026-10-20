---
id: a-pinch-carried-through-a-later-fold-step-refuses-naming
kind: issue
title: A union whose pinch an earlier fold step made and a later step carries refuses naming: a copy of a corner held as several meets no recorded discard between them
status: open
opened: 2026-10-18
priority: P2
cost: M
refs: [a-union-fold-publishes-no-pinch-row]
---


Filed by WIRE's `wire-pinch` lane (branch `wire/union-fold-pinch-row`)
while measuring `work/wire/a-union-fold-publishes-no-pinch-row.md`.

## What

The L-prism and tilted cube of
`crates/topo/tests/records_cite_their_decision.rs::a_pinch_records_the_vertex_identity_that_made_it`
pinch at the prism's reflex corner (1, 1, 1), the result holding one
vertex per cone there. Built as an editor document (the L as an
extrude of its profile on the xy frame, z 0 to 1; the cube as a square
of half-width 2 on the frame through (1, 1, 1) whose normal is
(1, 1.3, 0.7) normalised, extruded 4 along it) and folded with a
third member, measured on main at `4489bedb04`:

- `Union [far, prism, cube]` (far: the block x 6..7, y 6..7, z 0..1)
  builds, two vertices at the corner: the pinch is the last step's.
- `Union [prism, cube, far]` refuses
  `Naming(Emission { what: "a copy of a corner held as several meets no recorded discard between them" })`:
  step 1 makes the pinch and step 2 carries it untouched.
- `Intersect [prism, cube, big]` (big: the block −5..5 in x and y, z
  −5..10, holding the corner inside) refuses the same way.

The refusal is `names/borders.rs` `Obstacles::split_corner`, reached
from the union's end pass over a corner the accumulation already held
as several copies when the next member joined. A pinch an earlier step
made is the operand's own structure at the later step (topo's
`pinch_rows` reads it as inherited), so the end pass meets a corner
held as several with no step's discard between the copies.

## What it needs

The corner's copies named where an earlier step made the pinch as
where the last step makes it, so every member order of the union
builds the body `[far, prism, cube]` builds and publishes one table. A
pin over every member order of the three members closes it.

## Re-measured (2026-10-19, WIRE `wire-citerows`, PR 5352)

PR 5352 deletes topo's `pinch_rows`: a boolean now records every
decision it takes with a margin, or holds it unspelled
(`BooleanNaming::unspelled`), whatever the result made of the vertex.
The refusal does not depend on it. On the PR's head,
`editor-core` `union_member_orders_decide_alike::a_vertex_row_is_the_unions_once_in_every_member_order`
("the pinch with a far member") still asserts that every order folding
the far block last after the pinch refuses with "a copy of a corner held
as several meets no recorded discard between them", and it passes. The
far block's step decides nothing at the corner, so no row, emitted or
held, reaches the end pass there. The cause is the end pass reading a
corner the accumulation already holds as several
(`names/borders.rs` `Obstacles::split_corner`), not a missing topo
row. `Intersect [prism, cube, big]` was not re-run.

## More witnesses (WIRE lane `wire-nameinv`, 2026-10-20)

`crates/editor-core/tests/name_invariance.rs` reaches this refusal in
three of its cases, and admits each one against this row:
- `pinch and far / union` (`a_member_order_moves_no_name`): the orders
  `[1, 0, 2]` and `[0, 1, 2]` of `[prism, cube, far]` refuse, and the
  others build 97 names.
- `pinch and cap / intersect`: `Intersect [prism, cube, cap]` in those
  same two orders refuses, and the others build 46 names. So the
  intersect reaches it without a covering block.
- `prism and cube / a far member` (`a_far_feature_moves_no_other_name`):
  `Union [prism, cube]` builds 71 names, and appending a far block
  (x 9..10, y 9..10) as a last member refuses.
