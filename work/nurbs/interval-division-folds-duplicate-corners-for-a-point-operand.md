---
id: interval-division-folds-duplicate-corners-for-a-point-operand
kind: issue
title: The interval backend's division folds four corner quotients where a point operand has two, so every point-operand division pays twice the exactness witnesses it needs
status: open
opened: 2026-10-12
priority: P3
cost: E
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
