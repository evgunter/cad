---
id: stacked-plates-with-mismatched-fillet-radii-refuse-in-both-orders
kind: issue
title: Two stacked plates whose corner fillets differ in radius refuse their union in both operand orders
status: open
opened: 2026-10-01
priority: P3
cost: M
---

Found by the review of PR 3657, measured on `d2d5b09076`.

## Repro

Two 6 × 4 × 1 plates stacked on one footprint, corner fillets of
r = 0.5 on one and r = 0.3 on the other, every finding declared: the
union refuses in both operand orders. The fillets of the two plates
are different cylinders (different axes and radii), so they are no
continuation, and the larger fillet's tangent points fall in the middle
of the smaller-radius plate's flat wall edges.

Probably the same family as
`a-stack-across-a-mid-edge-tangency-builds-in-one-operand-order-only`,
except that here neither order splits the edge first. The refusal's
kind was not recorded by the review; record it when the row is picked
up.
