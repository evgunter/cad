---
id: a-disjoint-union-volume-differs-by-an-ulp-between-operand-orders
kind: issue
title: A disjoint union with a far box measures 1 ulp apart between its two operand orders on a donut-and-bar union
status: open
priority: P3
cost: M
opened: 2026-11-23
---

Found by the review of LATHE's PR 7532 (2026-11-23), confirmed on the
lane's branch `lathe/wrap-edge-station` and on its base
`lathe/full-revolve-one-band`.

## What

`U = donut ∪ bar`, the mesh suite's tour donut (`R = 2`, `r = 0.5`,
revolved about `y`) and the brick `x ∈ [1, 3]`, `y, z ∈ [−0.1, 0.1]`,
is at rest, and its seam meridians carry `SplitEdge` lineage where the
bar's edges end on them. Its disjoint union with a far brick
`[50, 51]³`, measured by `topo::test_support::pinned_volume`, reads

- `union(far, U)`: `10.909855025067612`
- `union(U, far)`: `10.909855025067614`

one ulp apart. DESIGN `[operations.operators]` holds member order to
deciding only which description a glued face keeps and which fold
step a refusal names. A disjoint union glues nothing, so the two orders
should hold one body, and its volume should agree bit for bit (as the
split-seam donut's did in `mesh10r2_probes`' former union half,
issue 1597).

## Where to look

Whether the two orders' bodies differ (cell order, a carrier restated,
a lineage chased differently through the graft), or the bodies agree
and the measurement reads them in an order-dependent sequence. The
first is this program's; the second belongs to the props owner.
