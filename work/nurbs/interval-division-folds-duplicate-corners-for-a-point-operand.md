---
id: interval-division-folds-duplicate-corners-for-a-point-operand
kind: issue
title: The interval backend's division folds four corner quotients where a point operand has two, so every point-operand division pays twice the exactness witnesses it needs
status: closed
opened: 2026-10-12
priority: P3
cost: E
closed: 2026-10-12
branch: nurbs/div-point-paths
pr: 4851
---

`interval-transcendentals/src/arith.rs`'s `Div for DInterval`, in its
branch whose divisor does not touch zero, folds the four corner
quotients `div_lo` and `div_hi` over `[self.lo, self.hi] × [rhs.lo,
rhs.hi]`. When either operand is a point, two of those corners repeat,
so half the quotients and their exactness witnesses are computed
twice.

The fold's extremes over the de-duplicated corners are the same bits
(`min` and `max` are idempotent, and the `lower_of`/`upper_of`
signed-zero rule only asks whether a zero occurs). This is the same
argument as `Mul`'s point paths, which `routed-de-boor-costs-the-sweep-digest-row-1-7x`
(PR 4841) added, pinned by `arith.rs`'s
`a_point_product_is_the_four_corner_fold`.

Where it costs: a knot-ratio division per de Boor entry
(`spline::blossom`'s `slice_ratio`, `props::quad`'s `de_boor_on`).
Its divisor, a difference of two knots, is usually a point. It was not
on that row's cost, which was six multiplies per `between` against one
division, so it was left out of that PR.

Owed: the same point paths in `Div`'s non-touching branch, with a row
pinning them to the four-corner fold over an edge table (the `Mul`
row's `EDGES`, minus divisors that touch zero). The point paths should
reuse `arith.rs`'s `corner_bounds`, generalized over the corner
function, rather than mint a third fold.

## Closed (NURBS, branch `nurbs/div-point-paths`)

`Div`'s branch whose divisor keeps clear of zero now takes the point
paths: a point operand folds its 2 distinct corner quotients, and
point ÷ point takes its 1. The output is bit-identical.

`corner_bounds` takes its directed corner functions, so `Mul` and `Div`
share one fold and no third copy exists.

**Rows** (`interval-transcendentals/src/arith.rs`):
- `a_point_product_is_the_four_corner_fold` and
  `a_point_quotient_is_the_four_corner_fold` sweep every interval over
  `EDGES` at two decorations. For the quotient, divisors are restricted
  to those clear of zero.
- Their oracle is the literal four-corner fold, written out in the test
  rather than read through `corner_bounds`, so a fault in the shared
  fold cannot hide in its own reference.
- Mutants, each red:
  - a dropped divisor corner (quotient row);
  - point ÷ point padded downward on both ends (quotient row);
  - the shared fold taking `max` for its least (both rows).
