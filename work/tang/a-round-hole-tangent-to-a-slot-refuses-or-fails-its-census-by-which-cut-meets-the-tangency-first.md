---
id: a-round-hole-tangent-to-a-slot-refuses-or-fails-its-census-by-which-cut-meets-the-tangency-first
kind: issue
title: A plate cut by a round hole tangent to a slot refuses or fails its census by which cut meets the tangency first
status: open
opened: 2026-11-23
priority: P3
cost: M
---

Found by the delta review of PR 7426 (formwords: a subtract's tool is a
`Bodies` seat), and pre-existing: a chain of pair subtracts does the
same, so it is the pair boolean's, not the n-ary fold's.

## The repro

`crates/editor-core/tests/review_subtract_delta.rs`, scenario "round
hole tangent to a slot" in `scenarios()`:

- a plate;
- a round through-hole of radius 0.25 at (1.0, 0.5) (`disc`);
- a through slot spanning x 1.25 to 1.75, y 0.2 to 0.8, whose wall at
  x = 1.25 is tangent to the hole along a ruling;
- a block spanning x 0.6 to 1.1, y 0.45 to 0.55, crossing the hole.

The plate is cut by the three in every order. `nary_and_chain` runs
each order both as the n-ary subtract and as the chain
`((plate − t0) − t1) − t2`.

## What happens

Whether the cut refuses or fails its census
depends on which tool meets the tangency first. The chain of pair cuts
does the same as the n-ary subtract, order for order. So the outcome is
decided by which body carries the tangent ruling when the second
tangent face arrives: the plain hole wall, or a wall already cut by the
block. It is not decided by the fold. `every_order_of_the_tools_builds_alike`
leaves this scenario out for that reason (its doc, and the "the chain
itself depends on the order" line it prints).

## What would close it

The tangency between the cylinder wall and the slot's plane wall is
settled the same way whether the wall it meets is whole or already cut.
Then every order of the three cuts builds one body, or refuses
alike, and the scenario rejoins `every_order_of_the_tools_builds_alike`.
